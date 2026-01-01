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
pub mod expr;
mod helpers;
mod ident;
mod interpolation;
mod stmt;

// Re-export error types for use throughout the parser
pub use expr::errors::{ParseError, ParseErrorKind, ParseResult};

use super::ir::{
    Accessibility, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind, VarDeclarator, VarKind,
};
use super::lexer::{Lexer, Token};
use super::syntax::SyntaxKind;
use proc_macro2::TokenStream;
use smallvec::SmallVec;
use std::str::FromStr;

/// Control flow markers that ALWAYS terminate expression parsing.
/// These are template-language constructs, not JS/TS syntax.
pub(super) const CONTROL_FLOW_TERMINATORS: &[SyntaxKind] = &[
    // Opening constructs: {#if}, {#for}, {#while}, {#match}
    SyntaxKind::BraceHashIf,
    SyntaxKind::BraceHashFor,
    SyntaxKind::BraceHashWhile,
    SyntaxKind::BraceHashMatch,
    // Closing constructs: {/if}, {/for}, {/while}, {/match}
    SyntaxKind::BraceSlashIf,
    SyntaxKind::BraceSlashFor,
    SyntaxKind::BraceSlashWhile,
    SyntaxKind::BraceSlashMatch,
    // Continuation constructs: {:else}, {:else if}, {:case}
    SyntaxKind::BraceColonElse,
    SyntaxKind::BraceColonElseIf,
    SyntaxKind::BraceColonCase,
];

/// Analysis context for placeholder classification.
/// Combines the context kind (for placeholder classification) with
/// terminators (for expression parsing termination).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Context {
    kind: ContextKind,
    terminators: SmallVec<[SyntaxKind; 4]>,
}

impl Context {
    /// Creates an expression context with the given kind and terminators (array form).
    fn expression<const N: usize>(kind: ExpressionKind, terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::Expression(kind),
            terminators: SmallVec::from_iter(terminators),
        }
    }

    /// Creates an expression context with the given kind and terminators (slice form).
    fn expression_from_slice(kind: ExpressionKind, terminators: &[SyntaxKind]) -> Self {
        Self {
            kind: ContextKind::Expression(kind),
            terminators: SmallVec::from_iter(terminators.iter().copied()),
        }
    }

    /// Creates a type annotation context with the given terminators.
    fn type_annotation<const N: usize>(terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::TypeAnnotation,
            terminators: SmallVec::from_iter(terminators),
        }
    }

    /// Creates a type assertion context with the given terminators.
    fn type_assertion<const N: usize>(terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::TypeAssertion,
            terminators: SmallVec::from_iter(terminators),
        }
    }

    /// Creates an identifier context with the given terminators.
    fn identifier<const N: usize>(terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::Identifier,
            terminators: SmallVec::from_iter(terminators),
        }
    }

    /// Creates a generic params context with the given terminators.
    fn generic_params<const N: usize>(terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::GenericParams,
            terminators: SmallVec::from_iter(terminators),
        }
    }

    /// Creates a statement context (no terminators needed).
    fn statement() -> Self {
        Self {
            kind: ContextKind::Statement,
            terminators: SmallVec::new(),
        }
    }

    /// Creates a parameters context with the given terminators.
    fn parameters<const N: usize>(terminators: [SyntaxKind; N]) -> Self {
        Self {
            kind: ContextKind::Parameters,
            terminators: SmallVec::from_iter(terminators),
        }
    }
}

/// The kind of context (for placeholder classification).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextKind {
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
    /// Pending decorators to attach to next class/function.
    pending_decorators: Vec<IrNode>,
}

impl Parser {
    /// Creates a new parser from input text.
    /// Panics if the input cannot be tokenized (lexer error).
    pub fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).tokenize().unwrap_or_else(|e| {
            panic!("Lexer error: {}", e);
        });
        Self {
            tokens,
            pos: 0,
            context_stack: vec![Context::expression(ExpressionKind::Normal, [])],
            source: input.to_string(),
            pending_doc: None,
            pending_decorators: Vec::new(),
        }
    }

    /// Creates a new parser from input text, returning an error if lexing fails.
    pub fn try_new(input: &str) -> Result<Self, crate::compiler::lexer::LexError> {
        let tokens = Lexer::new(input).tokenize()?;
        Ok(Self {
            tokens,
            pos: 0,
            context_stack: vec![Context::expression(ExpressionKind::Normal, [])],
            source: input.to_string(),
            pending_doc: None,
            pending_decorators: Vec::new(),
        })
    }

    /// Parses the input and returns an AST.
    pub fn parse(mut self) -> Ir {
        let nodes = self.parse_nodes();
        Ir::with_nodes(Self::merge_adjacent_text(nodes))
    }

    /// Execute a function with a temporary context pushed.
    /// Context is popped when function returns.
    pub(super) fn with_context<T>(&mut self, ctx: Context, f: impl FnOnce(&mut Self) -> T) -> T {
        self.context_stack.push(ctx);
        let result = f(self);
        self.context_stack.pop();
        result
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

    /// Returns true if the current token is any `{#...` opening token (BraceHashIf, BraceHashFor, etc.)
    fn at_brace_hash_open(&self) -> bool {
        self.current_kind().map_or(false, |k| k.is_brace_hash_open())
    }

    /// Returns true if the current token is any `{/...}` closing token (BraceSlashIf, BraceSlashFor, etc.)
    fn at_brace_slash_close(&self) -> bool {
        self.current_kind().map_or(false, |k| k.is_brace_slash_close())
    }

    /// Returns true if the current token is any `{:...` continuation token (BraceColonElse, BraceColonElseIf, etc.)
    fn at_brace_colon(&self) -> bool {
        self.current_kind().map_or(false, |k| k.is_brace_colon())
    }

    fn at_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.at_eof() {
            // Clone token data to avoid borrow issues
            // SAFETY: We just verified !at_eof(), so current() must return Some
            let token = self
                .current()
                .expect("advance() called when not at EOF but current() returned None");
            let (kind, text) = (token.kind, token.text.clone());

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
        let mut iterations = 0usize;
        const MAX_ITERATIONS: usize = 100_000;

        while !self.at_eof() {
            iterations += 1;
            let pos_before = self.pos;

            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_PARSER").is_ok() && iterations % 1000 == 0 {
                eprintln!(
                    "[MF_DEBUG_PARSER] parse_nodes iteration {}, pos={}/{}, current={:?}",
                    iterations,
                    self.pos,
                    self.tokens.len(),
                    self.current().map(|t| (&t.kind, &t.text))
                );
            }

            if iterations > MAX_ITERATIONS {
                #[cfg(debug_assertions)]
                {
                    eprintln!(
                        "[MF_DEBUG_PARSER] ERROR: parse_nodes exceeded {} iterations!",
                        MAX_ITERATIONS
                    );
                    eprintln!(
                        "[MF_DEBUG_PARSER] pos={}/{}, current={:?}",
                        self.pos,
                        self.tokens.len(),
                        self.current()
                    );
                    eprintln!("[MF_DEBUG_PARSER] source snippet: {:?}", &self.source[..self.source.len().min(500)]);
                }
                panic!("Parser infinite loop detected: exceeded {} iterations", MAX_ITERATIONS);
            }

            if let Some(node) = self.parse_node() {
                nodes.push(node);
            }

            // Safety check: ensure we made progress
            if self.pos == pos_before && !self.at_eof() {
                #[cfg(debug_assertions)]
                {
                    eprintln!(
                        "[MF_DEBUG_PARSER] ERROR: No progress at pos={}, token={:?}",
                        self.pos,
                        self.current()
                    );
                }
                // Force progress to avoid infinite loop
                self.advance();
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Option<IrNode> {
        self.parse_node_inner(0)
    }

    fn parse_node_inner(&mut self, depth: usize) -> Option<IrNode> {
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            #[cfg(debug_assertions)]
            eprintln!(
                "[MF_DEBUG_PARSER] ERROR: parse_node recursion depth {} exceeded at pos={}, token={:?}",
                depth,
                self.pos,
                self.current()
            );
            panic!("Parser infinite recursion detected in parse_node");
        }

        // Check for doc comments first - store them for the next node
        match self.current_kind()? {
            SyntaxKind::RustDocAttr => {
                let pos_before = self.pos;
                if let Some(IrNode::DocComment { text }) = self.parse_rust_doc_attr() {
                    self.pending_doc = Some(text);
                }
                // Ensure we made progress before recursing
                if self.pos == pos_before {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[MF_DEBUG_PARSER] WARNING: parse_rust_doc_attr didn't advance at pos={}",
                        self.pos
                    );
                    self.advance(); // Force progress
                }
                return self.parse_node_inner(depth + 1);
            }
            SyntaxKind::DocCommentPrefix | SyntaxKind::JsDocOpen => {
                let pos_before = self.pos;
                if let Some(IrNode::DocComment { text }) = self.parse_doc_comment() {
                    self.pending_doc = Some(text);
                }
                // Ensure we made progress before recursing
                if self.pos == pos_before {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "[MF_DEBUG_PARSER] WARNING: parse_doc_comment didn't advance at pos={}",
                        self.pos
                    );
                    self.advance(); // Force progress
                }
                return self.parse_node_inner(depth + 1);
            }
            _ => {}
        }

        let node = match self.current_kind()? {
            SyntaxKind::At => {
                if let Ok(placeholder) = self.parse_interpolation() {
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
            // Template control flow opening constructs
            SyntaxKind::BraceHashIf => self.parse_if_block_from_token(),
            SyntaxKind::BraceHashFor => self.parse_for_block_from_token(),
            SyntaxKind::BraceHashWhile => self.parse_while_block_from_token(),
            SyntaxKind::BraceHashMatch => self.parse_match_block_from_token(),
            // Template control flow closing constructs - consume and return None
            SyntaxKind::BraceSlashIf
            | SyntaxKind::BraceSlashFor
            | SyntaxKind::BraceSlashWhile
            | SyntaxKind::BraceSlashMatch => {
                // End of control block - consume the closing token
                self.consume();
                None
            }
            // Template control flow continuation constructs - error at top level
            SyntaxKind::BraceColonElse | SyntaxKind::BraceColonElseIf | SyntaxKind::BraceColonCase => {
                // Else/case clause at top level - error, consume
                self.consume();
                None
            }
            SyntaxKind::DollarOpen => self.parse_directive(),
            SyntaxKind::PipeOpen => self.parse_ident_block(),
            SyntaxKind::CommentLineOpen => self.parse_line_comment(),
            SyntaxKind::CommentBlockOpen => self.parse_block_comment(),
            SyntaxKind::DoubleQuote => self.parse_string_literal().ok(),
            SyntaxKind::Backtick => self.parse_template_literal().ok(),
            // TypeScript decorator - collect and attach to next class/function
            SyntaxKind::DecoratorAt => {
                if let Some(decorator) = self.parse_decorator_raw() {
                    self.pending_decorators.push(decorator);
                }
                // Continue to parse the next node (the decorated class/function)
                return self.parse_node_inner(depth + 1);
            }
            // TypeScript declarations
            SyntaxKind::ExportKw => self.parse_export_decl(),
            SyntaxKind::ImportKw => self.parse_import_decl(),
            SyntaxKind::EnumKw => self.parse_enum_decl(false, false).ok(),
            SyntaxKind::ClassKw => self.parse_class_decl(false).ok(),
            SyntaxKind::FunctionKw => self.parse_function_decl(false, false),
            SyntaxKind::InterfaceKw => self.parse_interface_decl(false),
            SyntaxKind::ConstKw => {
                // Check for `const enum`
                if self.peek_is_enum() {
                    self.parse_enum_decl(false, true).ok()
                } else {
                    self.parse_var_decl(false).ok()
                }
            }
            SyntaxKind::LetKw | SyntaxKind::VarKw => {
                self.parse_var_decl(false).ok()
            }
            SyntaxKind::AsyncKw => self.parse_async_decl(false),
            // Interface/type members (can appear in for-loop bodies inside interfaces)
            SyntaxKind::ReadonlyKw => self.parse_maybe_interface_member(),
            // Block statement at module level - use lookahead to distinguish from object literal
            SyntaxKind::LBrace if self.looks_like_block_stmt() => self.parse_block_stmt().ok(),
            // TypeScript statements that can appear at module level
            SyntaxKind::IfKw => self.parse_ts_if_stmt().ok(),
            SyntaxKind::ForKw | SyntaxKind::WhileKw => self.parse_ts_loop_stmt().ok(),
            SyntaxKind::TryKw => self.parse_ts_try_stmt(),
            SyntaxKind::ReturnKw => self.parse_return_stmt().ok(),
            SyntaxKind::ThrowKw => self.parse_throw_stmt().ok(),
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

    /// Parse a TypeScript decorator and pass through as raw text.
    /// Handles: `@decorator`, `@decorator(args)`, `@decorator.member`, etc.
    fn parse_decorator_raw(&mut self) -> Option<IrNode> {
        // The DecoratorAt token is just "@", we need to collect the full decorator
        self.consume()?; // consume @
        let mut parts = vec![IrNode::Raw("@".to_string())];

        // Consume the decorator identifier/expression
        // This could be: identifier, member access (a.b.c), or call (fn(args))
        let mut paren_depth = 0;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::Ident) => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::At) => {
                    // Interpolation inside decorator
                    if let Ok(placeholder) = self.parse_interpolation() {
                        parts.push(placeholder);
                    }
                }
                Some(SyntaxKind::LParen) => {
                    paren_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::RParen) => {
                    paren_depth -= 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                    if paren_depth <= 0 {
                        break;
                    }
                }
                Some(SyntaxKind::Dot) | Some(SyntaxKind::Colon) | Some(SyntaxKind::Comma)
                | Some(SyntaxKind::LBrace) | Some(SyntaxKind::RBrace)
                | Some(SyntaxKind::LBracket) | Some(SyntaxKind::RBracket)
                | Some(SyntaxKind::Lt) | Some(SyntaxKind::Gt)
                | Some(SyntaxKind::DoubleQuote) | Some(SyntaxKind::SingleQuote)
                | Some(SyntaxKind::Text) | Some(SyntaxKind::Eq) => {
                    if paren_depth > 0 {
                        // Inside parens, consume everything
                        if self.at(SyntaxKind::DoubleQuote) {
                            if let Ok(s) = self.parse_string_literal() {
                                parts.push(s);
                            }
                        } else if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    } else {
                        // Outside parens, we're done with the decorator
                        break;
                    }
                }
                Some(SyntaxKind::Whitespace) => {
                    if paren_depth > 0 {
                        // Consume whitespace inside parens
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    } else {
                        // Whitespace after decorator, we're done
                        break;
                    }
                }
                _ => {
                    if paren_depth > 0 {
                        // Inside parens, consume anything
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // Merge and return
        let merged = Self::merge_adjacent_text(parts);
        if merged.len() == 1 {
            Some(merged.into_iter().next().unwrap())
        } else {
            Some(IrNode::IdentBlock { parts: merged })
        }
    }

    // =========================================================================
    // Control block handlers (called from parse_node for new token types)
    // =========================================================================

    fn parse_if_block_from_token(&mut self) -> Option<IrNode> {
        self.parse_control_block(SyntaxKind::BraceHashIf)
    }

    fn parse_for_block_from_token(&mut self) -> Option<IrNode> {
        self.parse_control_block(SyntaxKind::BraceHashFor)
    }

    fn parse_while_block_from_token(&mut self) -> Option<IrNode> {
        self.parse_control_block(SyntaxKind::BraceHashWhile)
    }

    fn parse_match_block_from_token(&mut self) -> Option<IrNode> {
        self.parse_control_block(SyntaxKind::BraceHashMatch)
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
