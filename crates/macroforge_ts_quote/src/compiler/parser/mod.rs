//! Parser for the template language.
//!
//! This parser produces an AST directly from tokens, with inline placeholder
//! classification based on syntactic context.

#[cfg(test)]
mod tests;

// Submodules containing impl Parser blocks
mod comments;
mod context;
mod control_blocks;
mod decl;
mod directives;
mod helpers;
mod ident;
mod interpolation;
mod stmt;

use super::ir::{
    Accessibility, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind, VarDeclarator, VarKind,
};
use super::lexer::{Lexer, Token};
use super::syntax::SyntaxKind;
use proc_macro2::TokenStream;
use std::str::FromStr;

/// Analysis context for placeholder classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    /// Top-level or statement context.
    Statement,
    /// Expression context with optional sub-kind.
    Expression(ExpressionKind),
    /// Type annotation context (after `:`)
    TypeAnnotation,
    /// Type assertion context (after `as`, `satisfies`, etc.)
    TypeAssertion,
    /// Generic type parameters (inside `<...>`)
    GenericParams,
    /// Function parameter list.
    Parameters,
    /// Identifier context (for function/class/variable names).
    Identifier,
}

/// Sub-kinds of expression context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ExpressionKind {
    /// Normal expression context.
    #[default]
    Normal,
    /// Inside ternary conditional (after `?`, waiting for `:`)
    Ternary,
    /// Object literal expression (`:` is property separator, not type annotation)
    ObjectLiteral,
}

/// The parser for template input.
pub struct Parser {
    /// The tokens to parse.
    tokens: Vec<Token>,
    /// Current position in the token stream.
    pos: usize,
    /// Context stack for placeholder classification.
    context_stack: Vec<Context>,
    /// Full source text for debugging.
    #[allow(dead_code)]
    source: String,
    /// Pending doc comment to attach to next node.
    pending_doc: Option<String>,
}

impl Parser {
    /// Creates a new parser from input text.
    pub fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).tokenize();
        Self {
            tokens,
            pos: 0,
            context_stack: vec![Context::Expression(ExpressionKind::Normal)],
            source: input.to_string(),
            pending_doc: None,
        }
    }

    /// Parses the input and returns an AST.
    pub fn parse(mut self) -> Ir {
        let nodes = self.parse_nodes();
        Ir::with_nodes(Self::merge_adjacent_text(nodes))
    }

    // =========================================================================
    // Token navigation
    // =========================================================================

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn current_kind(&self) -> Option<SyntaxKind> {
        self.current().map(|t| t.kind)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.current_kind() == Some(kind)
    }

    fn at_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.at_eof() {
            // Clone token data to avoid borrow issues
            let (kind, text) = self
                .current()
                .map(|t| (t.kind, t.text.clone()))
                .unwrap_or((SyntaxKind::Error, String::new()));

            // Update context based on the token we're consuming
            self.update_context(kind, &text);
            self.pos += 1;
        }
    }

    fn consume(&mut self) -> Option<Token> {
        if self.at_eof() {
            return None;
        }
        let token = self.tokens[self.pos].clone();
        self.advance();
        Some(token)
    }

    fn expect(&mut self, kind: SyntaxKind) -> Option<Token> {
        if self.at(kind) { self.consume() } else { None }
    }

    fn skip_whitespace(&mut self) {
        while self.at(SyntaxKind::Whitespace) {
            self.advance();
        }
    }

    // =========================================================================
    // Parsing
    // =========================================================================

    fn parse_nodes(&mut self) -> Vec<IrNode> {
        let mut nodes = Vec::new();

        while !self.at_eof() {
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Option<IrNode> {
        // Check for doc comments first - store them for the next node
        match self.current_kind()? {
            SyntaxKind::RustDocAttr => {
                if let Some(IrNode::DocComment { text }) = self.parse_rust_doc_attr() {
                    self.pending_doc = Some(text);
                }
                return self.parse_node(); // Parse the next node
            }
            SyntaxKind::DocCommentPrefix | SyntaxKind::JsDocOpen => {
                if let Some(IrNode::DocComment { text }) = self.parse_doc_comment() {
                    self.pending_doc = Some(text);
                }
                return self.parse_node(); // Parse the next node
            }
            _ => {}
        }

        let node = match self.current_kind()? {
            SyntaxKind::At => {
                if let Some(placeholder) = self.parse_interpolation() {
                    // Check if there's an identifier suffix immediately after (e.g., @{name}Obj)
                    if let Some(token) = self.current() {
                        if token.kind == SyntaxKind::Ident {
                            let suffix = token.text.clone();
                            self.consume();
                            // Force placeholder to be Ident kind for identifier concatenation
                            let ident_placeholder = match placeholder {
                                IrNode::Placeholder { expr, .. } => IrNode::Placeholder {
                                    kind: PlaceholderKind::Ident,
                                    expr,
                                },
                                other => other,
                            };
                            // Create an IdentBlock combining placeholder + suffix
                            return Some(IrNode::IdentBlock {
                                parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                            });
                        }
                    }
                    Some(placeholder)
                } else {
                    None
                }
            }
            SyntaxKind::HashOpen => self.parse_control_block(),
            SyntaxKind::SlashOpen => {
                // End of control block - consume and return None
                self.consume_until_rbrace();
                None
            }
            SyntaxKind::ColonOpen => {
                // Else clause at top level - error, consume
                self.consume_until_rbrace();
                None
            }
            SyntaxKind::DollarOpen => self.parse_directive(),
            SyntaxKind::PipeOpen => self.parse_ident_block(),
            SyntaxKind::CommentLineOpen => self.parse_line_comment(),
            SyntaxKind::CommentBlockOpen => self.parse_block_comment(),
            SyntaxKind::DoubleQuote => self.parse_string_literal(),
            SyntaxKind::Backtick => self.parse_template_literal(),
            // TypeScript declarations
            SyntaxKind::ExportKw => self.parse_export_decl(),
            SyntaxKind::ClassKw => self.parse_class_decl(false),
            SyntaxKind::FunctionKw => self.parse_function_decl(false, false),
            SyntaxKind::InterfaceKw => self.parse_interface_decl(false),
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => {
                self.parse_var_decl(false)
            }
            SyntaxKind::AsyncKw => self.parse_async_decl(false),
            // Interface/type members (can appear in for-loop bodies inside interfaces)
            SyntaxKind::ReadonlyKw => self.parse_maybe_interface_member(),
            // Block statement at module level - use lookahead to distinguish from object literal
            SyntaxKind::LBrace if self.looks_like_block_stmt() => self.parse_block_stmt(),
            // TypeScript statements that can appear at module level
            SyntaxKind::IfKw => self.parse_ts_if_stmt(),
            SyntaxKind::ForKw | SyntaxKind::WhileKw => self.parse_ts_loop_stmt(),
            SyntaxKind::TryKw => self.parse_ts_try_stmt(),
            SyntaxKind::ReturnKw => self.parse_return_stmt(),
            SyntaxKind::ThrowKw => self.parse_throw_stmt(),
            _ => self.parse_text_token(),
        };

        // Wrap with pending doc comment if present
        if let Some(doc) = self.pending_doc.take() {
            node.map(|n| IrNode::Documented {
                doc,
                inner: Box::new(n),
            })
        } else {
            node
        }
    }

    fn parse_text_token(&mut self) -> Option<IrNode> {
        let token = self.consume()?;
        Some(IrNode::Raw(token.text))
    }

    fn consume_until_rbrace(&mut self) {
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.advance();
        }
        self.expect(SyntaxKind::RBrace);
    }
}
