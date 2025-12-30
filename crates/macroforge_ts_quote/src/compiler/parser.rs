//! Parser for the template language.
//!
//! This parser produces an AST directly from tokens, with inline placeholder
//! classification based on syntactic context.

use super::ir::{Accessibility, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind, VarDeclarator, VarKind};
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

    /// Merges adjacent Raw nodes.
    fn merge_adjacent_text(nodes: Vec<IrNode>) -> Vec<IrNode> {
        let mut result = Vec::with_capacity(nodes.len());
        let mut pending_text = String::new();

        for node in nodes {
            match node {
                IrNode::Raw(text) => {
                    pending_text.push_str(&text);
                }
                other => {
                    if !pending_text.is_empty() {
                        result.push(IrNode::Raw(std::mem::take(&mut pending_text)));
                    }
                    result.push(other);
                }
            }
        }

        if !pending_text.is_empty() {
            result.push(IrNode::Raw(pending_text));
        }

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
        if self.at(kind) {
            self.consume()
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.at(SyntaxKind::Whitespace) {
            self.advance();
        }
    }

    // =========================================================================
    // Context management
    // =========================================================================

    fn current_context(&self) -> Context {
        *self.context_stack.last().unwrap_or(&Context::Statement)
    }

    fn push_context(&mut self, ctx: Context) {
        self.context_stack.push(ctx);
    }

    fn pop_context(&mut self) {
        if self.context_stack.len() > 1 {
            self.context_stack.pop();
        }
    }

    fn is_expression_context(&self) -> bool {
        matches!(self.current_context(), Context::Expression(_))
    }

    fn is_ternary(&self) -> bool {
        matches!(
            self.current_context(),
            Context::Expression(ExpressionKind::Ternary)
        )
    }

    fn is_object_literal(&self) -> bool {
        matches!(
            self.current_context(),
            Context::Expression(ExpressionKind::ObjectLiteral)
        )
    }

    fn placeholder_kind(&self) -> PlaceholderKind {
        match self.current_context() {
            Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams => {
                PlaceholderKind::Type
            }
            Context::Identifier => PlaceholderKind::Ident,
            Context::Statement => PlaceholderKind::Stmt,
            Context::Expression(_) | Context::Parameters => PlaceholderKind::Expr,
        }
    }

    fn update_context(&mut self, kind: SyntaxKind, text: &str) {
        match kind {
            // Question mark in expression context starts ternary
            SyntaxKind::Question => {
                if self.is_expression_context() && !self.is_ternary() {
                    self.push_context(Context::Expression(ExpressionKind::Ternary));
                }
            }

            // Colon: type annotation, ternary separator, or object property
            SyntaxKind::Colon => {
                // Pop identifier context first
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }

                if self.is_ternary() {
                    // Ternary separator - pop ternary context, stay in expression
                    self.pop_context();
                } else if !self.is_object_literal() {
                    // Type annotation
                    self.push_context(Context::TypeAnnotation);
                }
                // In object literal, `:` is property separator - no context change
            }

            // Keywords that start type context
            SyntaxKind::AsKw | SyntaxKind::SatisfiesKw => {
                self.push_context(Context::TypeAssertion);
            }
            SyntaxKind::KeyofKw | SyntaxKind::TypeofKw | SyntaxKind::InferKw => {
                self.push_context(Context::TypeAnnotation);
            }
            SyntaxKind::ExtendsKw | SyntaxKind::ImplementsKw => {
                self.push_context(Context::TypeAnnotation);
            }

            // Keywords that start identifier context
            SyntaxKind::FunctionKw
            | SyntaxKind::ClassKw
            | SyntaxKind::InterfaceKw
            | SyntaxKind::TypeKw
            | SyntaxKind::ConstKw
            | SyntaxKind::LetKw
            | SyntaxKind::VarKw => {
                self.push_context(Context::Identifier);
            }

            // Keywords that start expression context
            SyntaxKind::ReturnKw
            | SyntaxKind::ThrowKw
            | SyntaxKind::YieldKw
            | SyntaxKind::AwaitKw
            | SyntaxKind::NewKw => {
                self.push_context(Context::Expression(ExpressionKind::Normal));
            }

            // Dot starts identifier context (member access)
            SyntaxKind::Dot => {
                self.push_context(Context::Identifier);
            }

            // Regular identifier consumes identifier context
            SyntaxKind::Ident => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // Opening paren ends identifier context
            SyntaxKind::LParen => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // Less-than might end identifier context (generics)
            SyntaxKind::Lt => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                // Could push GenericParams context here if needed
            }

            // Equals ends type annotation and identifier, starts expression
            SyntaxKind::Eq => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                if self.current_context() == Context::TypeAnnotation {
                    self.pop_context();
                }
                self.push_context(Context::Expression(ExpressionKind::Normal));
            }

            // Semicolon ends expression and type contexts (but keep base context)
            SyntaxKind::Semicolon => {
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context(),
                        Context::Expression(_)
                            | Context::TypeAnnotation
                            | Context::TypeAssertion
                            | Context::GenericParams
                    )
                {
                    self.pop_context();
                }
            }

            // Closing brace ends type contexts
            // BUT only for actual closing braces, not interpolation content (which has text like "expr}")
            SyntaxKind::RBrace if text == "}" || text == "}}" => {
                // Pop object literal context if we're in one
                if self.is_object_literal() {
                    self.pop_context();
                }
                // Pop any remaining type contexts (but keep base context)
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context(),
                        Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                    )
                {
                    self.pop_context();
                }
            }

            // Comma might end type context
            SyntaxKind::Comma => {
                if matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion
                ) {
                    self.pop_context();
                }
            }

            // Opening brace in expression context starts object literal
            SyntaxKind::LBrace => {
                if self.is_expression_context() {
                    self.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
                }
            }

            _ => {}
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
            SyntaxKind::At => self.parse_interpolation(),
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
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => self.parse_var_decl(false),
            SyntaxKind::AsyncKw => self.parse_async_decl(false),
            // Interface/type members (can appear in for-loop bodies inside interfaces)
            SyntaxKind::ReadonlyKw => self.parse_maybe_interface_member(),
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

    // =========================================================================
    // Interpolation
    // =========================================================================

    fn parse_interpolation(&mut self) -> Option<IrNode> {
        // IMPORTANT: Capture placeholder kind BEFORE consuming any tokens.
        // The RBrace token will trigger update_context which pops TypeAnnotation,
        // so we must determine the kind while the context is still intact.
        let kind = self.placeholder_kind();

        // Consume @{ (the At token includes both @ and {)
        let at_token = self.consume()?;

        // The lexer puts all content until } in one RBrace token
        // So we just need to get the RBrace token and extract the content
        let rbrace_token = self.expect(SyntaxKind::RBrace)?;

        // Combine and extract the Rust expression
        let full_text = format!("{}{}", at_token.text, rbrace_token.text);
        let rust_expr_str = full_text
            .strip_prefix("@{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| full_text);

        // Parse as TokenStream
        let expr = TokenStream::from_str(&rust_expr_str).unwrap_or_else(|_| {
            // Fallback: wrap in an identifier
            TokenStream::from_str(&format!("{{ {} }}", rust_expr_str))
                .unwrap_or_default()
        });

        Some(IrNode::Placeholder { kind, expr })
    }

    // =========================================================================
    // Control blocks
    // =========================================================================

    fn parse_control_block(&mut self) -> Option<IrNode> {
        // Consume {#
        self.consume()?;
        self.skip_whitespace();

        match self.current_kind() {
            Some(SyntaxKind::IfKw) => self.parse_if_block(),
            Some(SyntaxKind::ForKw) => self.parse_for_block(),
            Some(SyntaxKind::WhileKw) => self.parse_while_block(),
            Some(SyntaxKind::MatchKw) => self.parse_match_block(),
            _ => {
                // Unknown control block - consume until }
                self.consume_until_rbrace();
                None
            }
        }
    }

    fn parse_if_block(&mut self) -> Option<IrNode> {
        // Consume "if"
        self.consume()?;
        self.skip_whitespace();

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream(&condition_str);
        self.expect(SyntaxKind::RBrace);

        // Parse body until {:else}, {:else if}, or {/if}
        let then_body = self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);

        // Check for else-if and else clauses
        let mut else_if_branches = Vec::new();
        let mut else_body = None;

        while self.at(SyntaxKind::ColonOpen) {
            self.consume(); // {:
            self.skip_whitespace();

            if self.at(SyntaxKind::ElseKw) {
                self.consume(); // else
                self.skip_whitespace();

                if self.at(SyntaxKind::IfKw) {
                    // {:else if condition}
                    self.consume(); // if
                    self.skip_whitespace();
                    let cond_str = self.collect_rust_until(SyntaxKind::RBrace);
                    let cond = Self::str_to_token_stream(&cond_str);
                    self.expect(SyntaxKind::RBrace);
                    let body =
                        self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);
                    else_if_branches.push((cond, body));
                } else {
                    // {:else}
                    self.expect(SyntaxKind::RBrace);
                    else_body = Some(self.parse_block_body(&[SyntaxKind::SlashOpen]));
                    break;
                }
            } else {
                // Unknown clause - consume until }
                self.consume_until_rbrace();
                break;
            }
        }

        // Consume {/if}
        if self.at(SyntaxKind::SlashOpen) {
            self.consume(); // {/
            self.skip_whitespace();
            self.expect(SyntaxKind::IfKw);
            self.expect(SyntaxKind::RBrace);
        }

        Some(IrNode::If {
            condition,
            then_body: Self::merge_adjacent_text(then_body),
            else_if_branches: else_if_branches
                .into_iter()
                .map(|(c, b)| (c, Self::merge_adjacent_text(b)))
                .collect(),
            else_body: else_body.map(Self::merge_adjacent_text),
        })
    }

    fn parse_for_block(&mut self) -> Option<IrNode> {
        // Consume "for"
        self.consume()?;
        self.skip_whitespace();

        // Parse "pattern in iterator"
        let mut pattern_str = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::InKw) && !self.at(SyntaxKind::RBrace) {
            if let Some(token) = self.consume() {
                pattern_str.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::InKw);
        self.skip_whitespace();

        let iterator_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        // Parse body
        let body = self.parse_block_body(&[SyntaxKind::SlashOpen]);

        // Consume {/for}
        if self.at(SyntaxKind::SlashOpen) {
            self.consume();
            self.skip_whitespace();
            self.expect(SyntaxKind::ForKw);
            self.expect(SyntaxKind::RBrace);
        }

        Some(IrNode::For {
            pattern: Self::str_to_token_stream(pattern_str.trim()),
            iterator: Self::str_to_token_stream(&iterator_str),
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_while_block(&mut self) -> Option<IrNode> {
        // Consume "while"
        self.consume()?;
        self.skip_whitespace();

        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        let body = self.parse_block_body(&[SyntaxKind::SlashOpen]);

        // Consume {/while}
        if self.at(SyntaxKind::SlashOpen) {
            self.consume();
            self.skip_whitespace();
            self.expect(SyntaxKind::WhileKw);
            self.expect(SyntaxKind::RBrace);
        }

        Some(IrNode::While {
            condition: Self::str_to_token_stream(&condition_str),
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_match_block(&mut self) -> Option<IrNode> {
        // Consume "match"
        self.consume()?;
        self.skip_whitespace();

        let expr_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        let mut arms = Vec::new();

        // Parse cases
        while self.at(SyntaxKind::ColonOpen) {
            self.consume(); // {:
            self.skip_whitespace();

            if self.at(SyntaxKind::CaseKw) {
                self.consume(); // case
                self.skip_whitespace();

                let pattern_str = self.collect_rust_until(SyntaxKind::RBrace);
                self.expect(SyntaxKind::RBrace);

                let body =
                    self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);

                arms.push(MatchArm {
                    pattern: Self::str_to_token_stream(&pattern_str),
                    guard: None,
                    body: Self::merge_adjacent_text(body),
                });
            } else {
                self.consume_until_rbrace();
                break;
            }
        }

        // Consume {/match}
        if self.at(SyntaxKind::SlashOpen) {
            self.consume();
            self.skip_whitespace();
            self.expect(SyntaxKind::MatchKw);
            self.expect(SyntaxKind::RBrace);
        }

        Some(IrNode::Match {
            expr: Self::str_to_token_stream(&expr_str),
            arms,
        })
    }

    fn parse_block_body(&mut self, terminators: &[SyntaxKind]) -> Vec<IrNode> {
        let mut nodes = Vec::new();

        while !self.at_eof() {
            if let Some(kind) = self.current_kind() {
                if terminators.contains(&kind) {
                    break;
                }
            }

            if let Some(node) = self.parse_node() {
                nodes.push(node);
            }
        }

        nodes
    }

    fn collect_rust_until(&mut self, terminator: SyntaxKind) -> String {
        let mut result = String::new();

        while !self.at_eof() && !self.at(terminator) {
            if let Some(token) = self.consume() {
                result.push_str(&token.text);
            }
        }

        result.trim().to_string()
    }

    /// Convert a string to a TokenStream, with error handling.
    fn str_to_token_stream(s: &str) -> TokenStream {
        TokenStream::from_str(s).unwrap_or_else(|_| {
            // If parsing fails, try wrapping in braces
            TokenStream::from_str(&format!("{{ {} }}", s)).unwrap_or_default()
        })
    }

    // =========================================================================
    // Directives
    // =========================================================================

    fn parse_directive(&mut self) -> Option<IrNode> {
        // Consume {$
        self.consume()?;
        self.skip_whitespace();

        match self.current_kind() {
            Some(SyntaxKind::LetKw) => self.parse_let_directive(),
            Some(SyntaxKind::DoKw) => self.parse_do_directive(),
            Some(SyntaxKind::TypeScriptKw) => self.parse_typescript_directive(),
            _ => {
                self.consume_until_rbrace();
                None
            }
        }
    }

    fn parse_let_directive(&mut self) -> Option<IrNode> {
        // Consume "let"
        self.consume()?;
        self.skip_whitespace();

        // Check for "mut"
        let mutable = if self.at(SyntaxKind::MutKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Collect everything until }
        let content = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        // Parse "name: Type = value" or "name = value"
        if let Some(eq_pos) = content.find('=') {
            let name_part = content[..eq_pos].trim();
            let value_str = content[eq_pos + 1..].trim();

            let (pattern_str, type_hint) = if let Some(colon_pos) = name_part.find(':') {
                (
                    name_part[..colon_pos].trim(),
                    Some(Self::str_to_token_stream(name_part[colon_pos + 1..].trim())),
                )
            } else {
                (name_part, None)
            };

            Some(IrNode::Let {
                pattern: Self::str_to_token_stream(pattern_str),
                mutable,
                type_hint,
                value: Self::str_to_token_stream(value_str),
            })
        } else {
            None
        }
    }

    fn parse_do_directive(&mut self) -> Option<IrNode> {
        // Consume "do"
        self.consume()?;
        self.skip_whitespace();

        let code_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::Do {
            code: Self::str_to_token_stream(&code_str),
        })
    }

    fn parse_typescript_directive(&mut self) -> Option<IrNode> {
        // Consume "typescript"
        self.consume()?;
        self.skip_whitespace();

        let stream_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::TypeScript {
            stream: Self::str_to_token_stream(&stream_str),
        })
    }

    // =========================================================================
    // Ident blocks and strings
    // =========================================================================

    fn parse_ident_block(&mut self) -> Option<IrNode> {
        // Consume {|
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::PipeClose) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::PipeClose);

        Some(IrNode::IdentBlock {
            parts: Self::merge_adjacent_text(parts),
        })
    }

    fn parse_string_literal(&mut self) -> Option<IrNode> {
        // Consume opening "
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::DoubleQuote) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::DoubleQuote);

        Some(IrNode::StringInterp {
            quote: '"',
            parts: Self::merge_adjacent_text(parts),
        })
    }

    fn parse_template_literal(&mut self) -> Option<IrNode> {
        // Consume opening `
        self.consume()?;

        let mut parts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::Backtick) {
            if self.at(SyntaxKind::At) {
                if let Some(node) = self.parse_interpolation() {
                    parts.push(node);
                }
            } else if let Some(token) = self.consume() {
                parts.push(IrNode::Raw(token.text));
            }
        }

        self.expect(SyntaxKind::Backtick);

        Some(IrNode::StringInterp {
            quote: '`',
            parts: Self::merge_adjacent_text(parts),
        })
    }

    // =========================================================================
    // Comments
    // =========================================================================

    fn parse_line_comment(&mut self) -> Option<IrNode> {
        // Consume {>
        self.consume()?;

        let mut text = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::CommentLineClose) {
            if let Some(token) = self.consume() {
                text.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::CommentLineClose);

        Some(IrNode::LineComment {
            text: text.trim().to_string(),
        })
    }

    fn parse_block_comment(&mut self) -> Option<IrNode> {
        // Consume {>>
        self.consume()?;

        let mut text = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::CommentBlockClose) {
            if let Some(token) = self.consume() {
                text.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::CommentBlockClose);

        Some(IrNode::BlockComment {
            text: text.trim().to_string(),
        })
    }

    fn parse_rust_doc_attr(&mut self) -> Option<IrNode> {
        // Token text is already just the doc content (extracted by lexer)
        let token = self.consume()?;
        Some(IrNode::DocComment { text: token.text })
    }

    fn parse_doc_comment(&mut self) -> Option<IrNode> {
        let mut text = String::new();

        match self.current_kind() {
            Some(SyntaxKind::DocCommentPrefix) => {
                // /// style
                self.consume();
                // Collect until end of line
                while !self.at_eof() {
                    if let Some(token) = self.current() {
                        if token.text.contains('\n') {
                            break;
                        }
                    }
                    if let Some(token) = self.consume() {
                        text.push_str(&token.text);
                    }
                }
            }
            Some(SyntaxKind::JsDocOpen) => {
                // /** style
                self.consume();
                while !self.at_eof() && !self.at(SyntaxKind::JsDocClose) {
                    if let Some(token) = self.consume() {
                        text.push_str(&token.text);
                    }
                }
                self.expect(SyntaxKind::JsDocClose);
            }
            _ => return None,
        }

        Some(IrNode::DocComment {
            text: text.trim().to_string(),
        })
    }

    // =========================================================================
    // TypeScript Declaration Parsing
    // =========================================================================

    fn parse_export_decl(&mut self) -> Option<IrNode> {
        // Consume "export"
        self.consume()?;
        self.skip_whitespace();

        // Check what follows
        match self.current_kind() {
            Some(SyntaxKind::ClassKw) => self.parse_class_decl(true),
            Some(SyntaxKind::FunctionKw) => self.parse_function_decl(true, false),
            Some(SyntaxKind::InterfaceKw) => self.parse_interface_decl(true),
            Some(SyntaxKind::ConstKw) | Some(SyntaxKind::LetKw) | Some(SyntaxKind::VarKw) => {
                self.parse_var_decl(true)
            }
            Some(SyntaxKind::AsyncKw) => self.parse_async_decl(true),
            Some(SyntaxKind::TypeKw) => self.parse_type_alias_decl(true),
            Some(SyntaxKind::DefaultKw) => {
                // export default - just emit raw for now
                Some(IrNode::Raw("export ".to_string()))
            }
            _ => Some(IrNode::Raw("export ".to_string())),
        }
    }

    fn parse_async_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "async"
        self.consume()?;
        self.skip_whitespace();

        if self.at(SyntaxKind::FunctionKw) {
            self.parse_function_decl(exported, true)
        } else {
            // Not a function, just return raw
            let prefix = if exported { "export async " } else { "async " };
            Some(IrNode::Raw(prefix.to_string()))
        }
    }

    fn parse_class_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "class"
        self.consume()?;
        self.skip_whitespace();

        // Parse class name (may be placeholder)
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse optional extends
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::ImplementsKw, SyntaxKind::LBrace])?))
        } else {
            None
        };

        // Parse optional implements
        let implements = if self.at(SyntaxKind::ImplementsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
        } else {
            vec![]
        };

        // Parse class body
        if !self.at(SyntaxKind::LBrace) {
            // No body, return raw
            return Some(IrNode::Raw("class ".to_string()));
        }
        self.consume(); // consume {
        self.skip_whitespace();

        let body = self.parse_class_body();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::ClassDecl {
            exported,
            declare: false,
            abstract_: false,
            name: Box::new(name),
            type_params,
            extends,
            implements,
            body,
        })
    }

    fn parse_class_body(&mut self) -> Vec<IrNode> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
                    members.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            // Parse class member
            if let Some(member) = self.parse_class_member() {
                members.push(member);
            } else {
                // Consume unknown token
                self.advance();
            }
        }

        members
    }

    fn parse_class_member(&mut self) -> Option<IrNode> {
        self.skip_whitespace();

        // Check for doc comment
        if self.at(SyntaxKind::DocCommentPrefix) || self.at(SyntaxKind::JsDocOpen) {
            if let Some(IrNode::DocComment { text }) = self.parse_doc_comment() {
                self.pending_doc = Some(text);
            }
            return self.parse_class_member();
        }

        // Parse modifiers
        let mut static_ = false;
        let mut readonly = false;
        let mut accessibility = None;
        let mut async_ = false;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::StaticKw) => {
                    static_ = true;
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ReadonlyKw) => {
                    readonly = true;
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PublicKw) => {
                    accessibility = Some(Accessibility::Public);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PrivateKw) => {
                    accessibility = Some(Accessibility::Private);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ProtectedKw) => {
                    accessibility = Some(Accessibility::Protected);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::AsyncKw) => {
                    async_ = true;
                    self.consume();
                    self.skip_whitespace();
                }
                _ => break,
            }
        }

        // Check for constructor
        if self.current_text() == Some("constructor") {
            return self.parse_constructor(accessibility);
        }

        // Parse member name
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Check for optional marker
        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check if this is a method (has parentheses or type params) or property
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            // Method
            let type_params = self.parse_optional_type_params();
            self.skip_whitespace();

            let params = self.parse_param_list();
            self.skip_whitespace();

            // Return type
            let return_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_type_until(&[SyntaxKind::LBrace, SyntaxKind::Semicolon])?))
            } else {
                None
            };

            // Body
            let body = if self.at(SyntaxKind::LBrace) {
                Some(Box::new(self.parse_block_stmt()?))
            } else {
                self.expect(SyntaxKind::Semicolon);
                None
            };

            let node = IrNode::Method {
                static_,
                accessibility,
                readonly,
                async_,
                generator: false,
                kind: MethodKind::Method,
                name: Box::new(name),
                optional,
                type_params,
                params,
                return_type,
                body,
            };

            return self.wrap_with_doc(node);
        }

        // Property
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Semicolon, SyntaxKind::RBrace])?))
        } else {
            None
        };

        let value = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Semicolon, SyntaxKind::RBrace])?))
        } else {
            None
        };

        // Consume optional semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let node = IrNode::ClassProp {
            static_,
            accessibility,
            readonly,
            declare: false,
            optional,
            definite: false,
            name: Box::new(name),
            type_ann,
            value,
        };

        self.wrap_with_doc(node)
    }

    fn parse_constructor(&mut self, accessibility: Option<Accessibility>) -> Option<IrNode> {
        // Consume "constructor"
        self.consume()?;
        self.skip_whitespace();

        let params = self.parse_param_list();
        self.skip_whitespace();

        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };

        let node = IrNode::Constructor {
            accessibility,
            params,
            body,
        };

        self.wrap_with_doc(node)
    }

    fn parse_function_decl(&mut self, exported: bool, async_: bool) -> Option<IrNode> {
        // Consume "function"
        self.consume()?;
        self.skip_whitespace();

        // Check for generator (by checking for `*` text)
        let generator = if self.current_text() == Some("*") {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Parse function name
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse params
        let params = self.parse_param_list();
        self.skip_whitespace();

        // Parse return type
        let return_type = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[SyntaxKind::LBrace, SyntaxKind::Semicolon])?))
        } else {
            None
        };

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };

        let node = IrNode::FnDecl {
            exported,
            declare: false,
            async_,
            generator,
            name: Box::new(name),
            type_params,
            params,
            return_type,
            body,
        };

        self.wrap_with_doc(node)
    }

    fn parse_interface_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "interface"
        self.consume()?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse extends
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
        } else {
            vec![]
        };

        // Parse body
        if !self.at(SyntaxKind::LBrace) {
            return Some(IrNode::Raw("interface ".to_string()));
        }
        self.consume();
        self.skip_whitespace();

        let body = self.parse_interface_body();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        let node = IrNode::InterfaceDecl {
            exported,
            declare: false,
            name: Box::new(name),
            type_params,
            extends,
            body,
        };

        self.wrap_with_doc(node)
    }

    fn parse_interface_body(&mut self) -> Vec<IrNode> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
                    members.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            if let Some(member) = self.parse_interface_member() {
                members.push(member);
            } else {
                self.advance();
            }
        }

        members
    }

    /// Try to parse an interface member (property/method signature) when we see `readonly`.
    /// Falls back to Raw text if it doesn't look like an interface member pattern.
    fn parse_maybe_interface_member(&mut self) -> Option<IrNode> {
        // We're at `readonly` - consume it
        let readonly_token = self.consume()?;
        self.skip_whitespace();

        // Check if next is ident/placeholder (looks like interface member)
        if self.at(SyntaxKind::At) || self.at(SyntaxKind::Ident) || self.current_kind().map_or(false, |k| k.is_ts_keyword()) {
            // Looks like interface member - parse the name
            let name = match self.parse_ts_ident_or_placeholder() {
                Some(n) => n,
                None => return Some(IrNode::Raw(readonly_token.text)),
            };
            self.skip_whitespace();

            let optional = if self.at(SyntaxKind::Question) {
                self.consume();
                self.skip_whitespace();
                true
            } else {
                false
            };

            // Need colon for type annotation
            if !self.at(SyntaxKind::Colon) {
                // Not a valid member pattern - return what we consumed as raw
                return Some(IrNode::Raw(format!("{} ", readonly_token.text)));
            }

            self.consume(); // colon
            self.skip_whitespace();

            let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon, SyntaxKind::Comma, SyntaxKind::RBrace, SyntaxKind::SlashOpen]);

            // Consume optional separator
            if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
                self.consume();
            }

            Some(IrNode::PropSignature {
                readonly: true,
                name: Box::new(name),
                optional,
                type_ann: type_ann.map(Box::new),
            })
        } else {
            // Doesn't look like interface member - return readonly as raw text
            Some(IrNode::Raw(readonly_token.text))
        }
    }

    fn parse_interface_member(&mut self) -> Option<IrNode> {
        self.skip_whitespace();

        let readonly = if self.at(SyntaxKind::ReadonlyKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check if method signature or property
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            let type_params = self.parse_optional_type_params();
            let params = self.parse_param_list();
            self.skip_whitespace();

            let return_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_type_until(&[SyntaxKind::Semicolon, SyntaxKind::Comma, SyntaxKind::RBrace])?))
            } else {
                None
            };

            // Consume optional separator
            if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
                self.consume();
            }

            return Some(IrNode::MethodSignature {
                name: Box::new(name),
                optional,
                type_params,
                params,
                return_type,
            });
        }

        // Property signature
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[SyntaxKind::Semicolon, SyntaxKind::Comma, SyntaxKind::RBrace])?))
        } else {
            None
        };

        // Consume optional separator
        if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
            self.consume();
        }

        Some(IrNode::PropSignature {
            readonly,
            name: Box::new(name),
            optional,
            type_ann,
        })
    }

    fn parse_var_decl(&mut self, exported: bool) -> Option<IrNode> {
        let kind = match self.current_kind() {
            Some(SyntaxKind::ConstKw) => VarKind::Const,
            Some(SyntaxKind::LetKw) => VarKind::Let,
            Some(SyntaxKind::VarKw) => VarKind::Var,
            _ => return None,
        };
        self.consume();
        self.skip_whitespace();

        let mut decls = Vec::new();

        loop {
            let name = self.parse_ts_ident_or_placeholder()?;
            self.skip_whitespace();

            let type_ann = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?))
            } else {
                None
            };

            let init = if self.at(SyntaxKind::Eq) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Comma, SyntaxKind::Semicolon])?))
            } else {
                None
            };

            decls.push(VarDeclarator {
                name: Box::new(name),
                type_ann,
                init,
                definite: false,
            });

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
                self.skip_whitespace();
            } else {
                break;
            }
        }

        // Consume semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::VarDecl {
            exported,
            declare: false,
            kind,
            decls,
        })
    }

    fn parse_type_alias_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "type"
        self.consume()?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        if !self.at(SyntaxKind::Eq) {
            return Some(IrNode::Raw("type ".to_string()));
        }
        self.consume();
        self.skip_whitespace();

        let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon])?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let node = IrNode::TypeAliasDecl {
            exported,
            declare: false,
            name: Box::new(name),
            type_params,
            type_ann: Box::new(type_ann),
        };

        self.wrap_with_doc(node)
    }

    fn parse_block_stmt(&mut self) -> Option<IrNode> {
        // Consume {
        self.expect(SyntaxKind::LBrace)?;
        self.skip_whitespace();

        let stmts = self.parse_stmt_list();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::BlockStmt { stmts })
    }

    fn parse_stmt_list(&mut self) -> Vec<IrNode> {
        let mut stmts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
                    stmts.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    stmts.push(node);
                }
                continue;
            }

            // Parse statement
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Unknown - consume one token as raw
                if let Some(token) = self.consume() {
                    stmts.push(IrNode::Raw(token.text));
                }
            }
        }

        Self::merge_adjacent_text(stmts)
    }

    fn parse_stmt(&mut self) -> Option<IrNode> {
        match self.current_kind()? {
            SyntaxKind::ReturnKw => self.parse_return_stmt(),
            SyntaxKind::ThrowKw => self.parse_throw_stmt(),
            SyntaxKind::IfKw => self.parse_ts_if_stmt(),
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => self.parse_var_decl(false),
            SyntaxKind::At => self.parse_interpolation(), // Statement placeholder
            _ => {
                // Expression statement - collect until semicolon or special tokens
                let expr = self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?;

                if self.at(SyntaxKind::Semicolon) {
                    self.consume();
                }

                Some(IrNode::ExprStmt {
                    expr: Box::new(expr),
                })
            }
        }
    }

    fn parse_return_stmt(&mut self) -> Option<IrNode> {
        self.consume()?; // return
        self.skip_whitespace();

        if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::RBrace) {
            if self.at(SyntaxKind::Semicolon) {
                self.consume();
            }
            return Some(IrNode::ReturnStmt { arg: None });
        }

        let arg = self.parse_ts_expr_until(&[SyntaxKind::Semicolon]);

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ReturnStmt {
            arg: arg.map(Box::new),
        })
    }

    fn parse_throw_stmt(&mut self) -> Option<IrNode> {
        self.consume()?; // throw
        self.skip_whitespace();

        let arg = self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ThrowStmt {
            arg: Box::new(arg),
        })
    }

    fn parse_ts_if_stmt(&mut self) -> Option<IrNode> {
        self.consume()?; // if
        self.skip_whitespace();

        // Parse condition in parens
        self.expect(SyntaxKind::LParen);
        let test = self.parse_ts_expr_until(&[SyntaxKind::RParen])?;
        self.expect(SyntaxKind::RParen);
        self.skip_whitespace();

        // Parse consequent
        let cons = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()?
        } else {
            // Single statement
            self.parse_stmt()?
        };

        self.skip_whitespace();

        // Parse optional else
        let alt = if self.at(SyntaxKind::ElseKw) {
            self.consume();
            self.skip_whitespace();
            if self.at(SyntaxKind::LBrace) {
                Some(Box::new(self.parse_block_stmt()?))
            } else if self.at(SyntaxKind::IfKw) {
                Some(Box::new(self.parse_ts_if_stmt()?))
            } else {
                Some(Box::new(self.parse_stmt()?))
            }
        } else {
            None
        };

        Some(IrNode::TsIfStmt {
            test: Box::new(test),
            cons: Box::new(cons),
            alt,
        })
    }

    // =========================================================================
    // Helper Parsing Functions
    // =========================================================================

    fn current_text(&self) -> Option<&str> {
        self.current().map(|t| t.text.as_str())
    }

    fn wrap_with_doc(&mut self, node: IrNode) -> Option<IrNode> {
        if let Some(doc) = self.pending_doc.take() {
            Some(IrNode::Documented {
                doc,
                inner: Box::new(node),
            })
        } else {
            Some(node)
        }
    }

    fn parse_ts_ident_or_placeholder(&mut self) -> Option<IrNode> {
        if self.at(SyntaxKind::At) {
            // It's a placeholder
            self.parse_interpolation()
        } else if let Some(token) = self.current() {
            if token.kind == SyntaxKind::Ident || token.kind.is_ts_keyword() {
                let name = token.text.clone();
                self.consume();
                Some(IrNode::Ident(name))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_optional_type_params(&mut self) -> Option<Box<IrNode>> {
        if !self.at(SyntaxKind::Lt) {
            return None;
        }

        self.consume(); // <
        let mut parts = Vec::new();

        let mut angle_depth = 1;
        while !self.at_eof() && angle_depth > 0 {
            let text = self.current().map(|t| t.text.as_str()).unwrap_or("");
            match self.current_kind() {
                Some(SyntaxKind::Lt) => {
                    angle_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::Gt) => {
                    // Handle >> as two >
                    if text == ">>" {
                        angle_depth -= 2;
                    } else {
                        angle_depth -= 1;
                    }
                    if angle_depth >= 0 {
                        if let Some(t) = self.consume() {
                            if angle_depth > 0 {
                                parts.push(IrNode::Raw(t.text));
                            }
                        }
                    }
                }
                Some(SyntaxKind::At) => {
                    if let Some(node) = self.parse_interpolation() {
                        parts.push(node);
                    }
                }
                _ => {
                    // Check for >> as text (in case it's not tokenized as Gt)
                    if text == ">>" {
                        angle_depth -= 2;
                        if angle_depth >= 0 {
                            if let Some(t) = self.consume() {
                                if angle_depth > 0 {
                                    parts.push(IrNode::Raw(t.text));
                                }
                            }
                        }
                    } else {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                }
            }
        }

        Some(Box::new(IrNode::TypeParams {
            params: Self::merge_adjacent_text(parts),
        }))
    }

    fn parse_param_list(&mut self) -> Vec<IrNode> {
        if !self.at(SyntaxKind::LParen) {
            return vec![];
        }

        self.consume(); // (
        let mut params = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RParen) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RParen) {
                break;
            }

            if let Some(param) = self.parse_param() {
                params.push(param);
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RParen);
        params
    }

    fn parse_param(&mut self) -> Option<IrNode> {
        self.skip_whitespace();

        // Parse optional modifiers (for constructor parameter properties)
        // These are stored but currently we use simplified Param structure
        let mut _accessibility = None;
        let mut _readonly = false;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::PublicKw) => {
                    _accessibility = Some(Accessibility::Public);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PrivateKw) => {
                    _accessibility = Some(Accessibility::Private);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ProtectedKw) => {
                    _accessibility = Some(Accessibility::Protected);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ReadonlyKw) => {
                    _readonly = true;
                    self.consume();
                    self.skip_whitespace();
                }
                _ => break,
            }
        }

        // Rest parameter? (check for `...` text)
        let rest = if self.current_text() == Some("...") {
            self.consume();
            true
        } else {
            false
        };

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[SyntaxKind::Comma, SyntaxKind::RParen, SyntaxKind::Eq])?))
        } else {
            None
        };

        let default_value = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Comma, SyntaxKind::RParen])?))
        } else {
            None
        };

        // Build the pattern
        let binding = IrNode::BindingIdent {
            name: Box::new(name),
            type_ann,
            optional,
        };

        // Wrap with rest or assign pattern if needed
        let pat = if rest {
            Box::new(IrNode::RestPat {
                arg: Box::new(binding),
                type_ann: None, // type_ann is already on BindingIdent
            })
        } else if let Some(right) = default_value {
            Box::new(IrNode::AssignPat {
                left: Box::new(binding),
                right,
            })
        } else {
            Box::new(binding)
        };

        Some(IrNode::Param {
            decorators: vec![],
            pat,
        })
    }

    fn parse_type_until(&mut self, terminators: &[SyntaxKind]) -> Option<IrNode> {
        let mut parts = Vec::new();
        let mut depth = 0; // Track nested brackets/parens

        while !self.at_eof() {
            let kind = self.current_kind()?;
            let text = self.current().map(|t| t.text.as_str()).unwrap_or("");

            // Only terminate at depth 0
            if depth == 0 && terminators.contains(&kind) {
                break;
            }

            match kind {
                SyntaxKind::Lt | SyntaxKind::LParen | SyntaxKind::LBrace | SyntaxKind::LBracket => {
                    depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::Gt | SyntaxKind::RParen | SyntaxKind::RBrace | SyntaxKind::RBracket => {
                    // Handle >> as two >
                    if text == ">>" {
                        depth -= 2;
                    } else {
                        depth -= 1;
                    }
                    if depth < 0 {
                        break;
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::At => {
                    if let Some(node) = self.parse_interpolation() {
                        parts.push(node);
                    }
                }
                _ => {
                    // Handle >> as text (in case it's not tokenized as Gt)
                    if text == ">>" {
                        depth -= 2;
                        if depth < 0 {
                            break;
                        }
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
            }
        }

        if parts.is_empty() {
            None
        } else {
            let merged = Self::merge_adjacent_text(parts);
            if merged.len() == 1 {
                Some(merged.into_iter().next().unwrap())
            } else {
                // Wrap multiple parts in TypeAnnotation for complex types with placeholders
                Some(IrNode::TypeAnnotation {
                    type_ann: Box::new(IrNode::IdentBlock { parts: merged }),
                })
            }
        }
    }

    fn parse_type_list_until(&mut self, terminator: SyntaxKind) -> Vec<IrNode> {
        let mut types = Vec::new();

        while !self.at_eof() && !self.at(terminator) {
            self.skip_whitespace();
            if let Some(ty) = self.parse_type_until(&[SyntaxKind::Comma, terminator]) {
                types.push(ty);
            }
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        types
    }

    fn parse_ts_expr_until(&mut self, terminators: &[SyntaxKind]) -> Option<IrNode> {
        let mut parts = Vec::new();
        let mut depth = 0;

        while !self.at_eof() {
            let kind = self.current_kind()?;

            // Only terminate at depth 0
            if depth == 0 && terminators.contains(&kind) {
                break;
            }

            // Don't consume control flow markers
            if depth == 0 && (kind == SyntaxKind::HashOpen || kind == SyntaxKind::SlashOpen || kind == SyntaxKind::ColonOpen) {
                break;
            }

            match kind {
                SyntaxKind::LParen | SyntaxKind::LBrace | SyntaxKind::LBracket => {
                    depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::RParen | SyntaxKind::RBrace | SyntaxKind::RBracket => {
                    depth -= 1;
                    if depth < 0 {
                        break;
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::At => {
                    if let Some(node) = self.parse_interpolation() {
                        parts.push(node);
                    }
                }
                SyntaxKind::Backtick => {
                    if let Some(node) = self.parse_template_literal() {
                        parts.push(node);
                    }
                }
                SyntaxKind::DoubleQuote => {
                    if let Some(node) = self.parse_string_literal() {
                        parts.push(node);
                    }
                }
                _ => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
            }
        }

        if parts.is_empty() {
            None
        } else {
            let merged = Self::merge_adjacent_text(parts);
            if merged.len() == 1 {
                Some(merged.into_iter().next().unwrap())
            } else {
                // Wrap multiple parts in IdentBlock for complex expressions with placeholders
                Some(IrNode::IdentBlock { parts: merged })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Ir {
        Parser::new(input).parse()
    }

    /// Helper to find all placeholders in the IR (recursively traverses all node types)
    fn find_placeholders(ir: &Ir) -> Vec<(PlaceholderKind, String)> {
        fn collect_node(node: &IrNode, result: &mut Vec<(PlaceholderKind, String)>) {
            match node {
                IrNode::Placeholder { kind, expr } => {
                    result.push((*kind, expr.to_string()));
                }
                IrNode::If { then_body, else_if_branches, else_body, .. } => {
                    collect_nodes(then_body, result);
                    for (_, body) in else_if_branches {
                        collect_nodes(body, result);
                    }
                    if let Some(body) = else_body {
                        collect_nodes(body, result);
                    }
                }
                IrNode::For { body, .. } | IrNode::While { body, .. } => {
                    collect_nodes(body, result);
                }
                IrNode::Match { arms, .. } => {
                    for arm in arms {
                        collect_nodes(&arm.body, result);
                    }
                }
                IrNode::IdentBlock { parts } | IrNode::StringInterp { parts, .. } => {
                    collect_nodes(parts, result);
                }
                // Declarations
                IrNode::FnDecl { name, type_params, params, return_type, body, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_nodes(params, result);
                    if let Some(rt) = return_type {
                        collect_node(rt, result);
                    }
                    if let Some(b) = body {
                        collect_node(b, result);
                    }
                }
                IrNode::ClassDecl { name, type_params, extends, implements, body, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    if let Some(ext) = extends {
                        collect_node(ext, result);
                    }
                    collect_nodes(implements, result);
                    collect_nodes(body, result);
                }
                IrNode::InterfaceDecl { name, type_params, extends, body, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_nodes(extends, result);
                    collect_nodes(body, result);
                }
                IrNode::TypeAliasDecl { name, type_params, type_ann, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_node(type_ann, result);
                }
                IrNode::VarDecl { decls, .. } => {
                    for decl in decls {
                        collect_node(&decl.name, result);
                        if let Some(ta) = &decl.type_ann {
                            collect_node(ta, result);
                        }
                        if let Some(init) = &decl.init {
                            collect_node(init, result);
                        }
                    }
                }
                // Class members
                IrNode::Constructor { params, body, .. } => {
                    collect_nodes(params, result);
                    if let Some(b) = body {
                        collect_node(b, result);
                    }
                }
                IrNode::Method { name, type_params, params, return_type, body, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_nodes(params, result);
                    if let Some(rt) = return_type {
                        collect_node(rt, result);
                    }
                    if let Some(b) = body {
                        collect_node(b, result);
                    }
                }
                IrNode::ClassProp { name, type_ann, value, .. } => {
                    collect_node(name, result);
                    if let Some(ta) = type_ann {
                        collect_node(ta, result);
                    }
                    if let Some(v) = value {
                        collect_node(v, result);
                    }
                }
                // Statements
                IrNode::BlockStmt { stmts } => {
                    collect_nodes(stmts, result);
                }
                IrNode::ExprStmt { expr } => {
                    collect_node(expr, result);
                }
                IrNode::ReturnStmt { arg } => {
                    if let Some(a) = arg {
                        collect_node(a, result);
                    }
                }
                IrNode::ThrowStmt { arg } => {
                    collect_node(arg, result);
                }
                IrNode::TsIfStmt { test, cons, alt } => {
                    collect_node(test, result);
                    collect_node(cons, result);
                    if let Some(a) = alt {
                        collect_node(a, result);
                    }
                }
                // Parameters
                IrNode::Param { decorators, pat } => {
                    collect_nodes(decorators, result);
                    collect_node(pat, result);
                }
                IrNode::BindingIdent { name, type_ann, .. } => {
                    collect_node(name, result);
                    if let Some(ta) = type_ann {
                        collect_node(ta, result);
                    }
                }
                IrNode::RestPat { arg, type_ann } => {
                    collect_node(arg, result);
                    if let Some(ta) = type_ann {
                        collect_node(ta, result);
                    }
                }
                IrNode::AssignPat { left, right } => {
                    collect_node(left, result);
                    collect_node(right, result);
                }
                // Types
                IrNode::TypeAnnotation { type_ann } => {
                    collect_node(type_ann, result);
                }
                IrNode::TypeParams { params } => {
                    collect_nodes(params, result);
                }
                // Interface members
                IrNode::PropSignature { name, type_ann, .. } => {
                    collect_node(name, result);
                    if let Some(ta) = type_ann {
                        collect_node(ta, result);
                    }
                }
                IrNode::MethodSignature { name, type_params, params, return_type, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_nodes(params, result);
                    if let Some(rt) = return_type {
                        collect_node(rt, result);
                    }
                }
                // Wrapper
                IrNode::Documented { inner, .. } => {
                    collect_node(inner, result);
                }
                _ => {}
            }
        }
        fn collect_nodes(nodes: &[IrNode], result: &mut Vec<(PlaceholderKind, String)>) {
            for node in nodes {
                collect_node(node, result);
            }
        }
        let mut result = Vec::new();
        collect_nodes(&ir.nodes, &mut result);
        result
    }

    // =========================================================================
    // Basic parsing tests
    // =========================================================================

    #[test]
    fn test_simple_text() {
        let ir = parse("hello world");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::Raw(text) => assert!(text.contains("hello")),
            _ => panic!("Expected Raw"),
        }
    }

    #[test]
    fn test_empty_input() {
        let ir = parse("");
        assert!(ir.nodes.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let ir = parse("   \n\t  ");
        // Should produce text node(s) with whitespace
        assert!(!ir.nodes.is_empty());
    }

    // =========================================================================
    // Placeholder classification tests
    // =========================================================================

    #[test]
    fn test_expr_placeholder_default() {
        let ir = parse("@{expr}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0], (PlaceholderKind::Expr, "expr".to_string()));
    }

    #[test]
    fn test_expr_placeholder_in_assignment() {
        let ir = parse("const x = @{value}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Expr);
    }

    #[test]
    fn test_type_annotation_const() {
        let ir = parse("const x: @{T} = 1");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0], (PlaceholderKind::Type, "T".to_string()));
    }

    #[test]
    fn test_type_annotation_let() {
        let ir = parse("let x: @{MyType} = value");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_type_annotation_function_param() {
        let ir = parse("function foo(x: @{T}) {}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_type_annotation_function_return() {
        let ir = parse("function foo(): @{ReturnType} {}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_type_assertion_as() {
        let ir = parse("x as @{T}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_type_assertion_satisfies() {
        let ir = parse("x satisfies @{T}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_ternary_colon_not_type() {
        // In ternary, the : is not a type annotation
        let ir = parse("cond ? @{a} : @{b}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 2);
        // Both should be Expr, not Type
        assert_eq!(placeholders[0].0, PlaceholderKind::Expr);
        assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
    }

    #[test]
    fn test_ternary_with_type_before() {
        // Type annotation followed by ternary
        let ir = parse("const x: @{T} = cond ? @{a} : @{b}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 3);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type); // T
        assert_eq!(placeholders[1].0, PlaceholderKind::Expr); // a
        assert_eq!(placeholders[2].0, PlaceholderKind::Expr); // b
    }

    #[test]
    fn test_multiple_placeholders_mixed() {
        let ir = parse("const x: @{T} = @{v}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 2);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
        assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
    }

    // =========================================================================
    // Control flow tests
    // =========================================================================

    #[test]
    fn test_if_simple() {
        let ir = parse("{#if cond}body{/if}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::If { condition, then_body, else_if_branches, else_body } => {
                assert_eq!(condition.to_string(), "cond");
                assert!(!then_body.is_empty());
                assert!(else_if_branches.is_empty());
                assert!(else_body.is_none());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_if_else() {
        let ir = parse("{#if cond}yes{:else}no{/if}");
        match &ir.nodes[0] {
            IrNode::If { condition, then_body, else_body, .. } => {
                assert_eq!(condition.to_string(), "cond");
                assert!(!then_body.is_empty());
                assert!(else_body.is_some());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_if_else_if_else() {
        let ir = parse("{#if a}1{:else if b}2{:else if c}3{:else}4{/if}");
        match &ir.nodes[0] {
            IrNode::If { condition, else_if_branches, else_body, .. } => {
                assert_eq!(condition.to_string(), "a");
                assert_eq!(else_if_branches.len(), 2);
                assert_eq!(else_if_branches[0].0.to_string(), "b");
                assert_eq!(else_if_branches[1].0.to_string(), "c");
                assert!(else_body.is_some());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_for_loop() {
        let ir = parse("{#for item in items}@{item}{/for}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::For { pattern, iterator, body } => {
                assert_eq!(pattern.to_string(), "item");
                assert_eq!(iterator.to_string(), "items");
                assert!(!body.is_empty());
            }
            _ => panic!("Expected For"),
        }
    }

    #[test]
    fn test_for_with_tuple_pattern() {
        let ir = parse("{#for (key, value) in map}@{key}: @{value}{/for}");
        match &ir.nodes[0] {
            IrNode::For { pattern, iterator, .. } => {
                let pat_str = pattern.to_string();
                assert!(pat_str.contains("key"));
                assert!(pat_str.contains("value"));
                assert_eq!(iterator.to_string(), "map");
            }
            _ => panic!("Expected For"),
        }
    }

    #[test]
    fn test_while_loop() {
        let ir = parse("{#while cond}body{/while}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::While { condition, body } => {
                assert_eq!(condition.to_string(), "cond");
                assert!(!body.is_empty());
            }
            _ => panic!("Expected While"),
        }
    }

    #[test]
    fn test_match_block() {
        let ir = parse("{#match expr}{:case Some(x)}found{:case None}empty{/match}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::Match { expr, arms } => {
                assert_eq!(expr.to_string(), "expr");
                assert_eq!(arms.len(), 2);
                assert!(arms[0].pattern.to_string().contains("Some"));
                assert!(arms[1].pattern.to_string().contains("None"));
            }
            _ => panic!("Expected Match"),
        }
    }

    #[test]
    fn test_nested_control_flow() {
        let ir = parse("{#if outer}{#for x in xs}@{x}{/for}{/if}");
        match &ir.nodes[0] {
            IrNode::If { then_body, .. } => {
                assert_eq!(then_body.len(), 1);
                assert!(matches!(&then_body[0], IrNode::For { .. }));
            }
            _ => panic!("Expected If"),
        }
    }

    // =========================================================================
    // Directive tests
    // =========================================================================

    #[test]
    fn test_let_directive() {
        let ir = parse("{$let x = 1}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::Let { pattern, mutable, value, .. } => {
                assert_eq!(pattern.to_string(), "x");
                assert!(!mutable);
                assert_eq!(value.to_string(), "1");
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_let_mut_directive() {
        let ir = parse("{$let mut count = 0}");
        match &ir.nodes[0] {
            IrNode::Let { pattern, mutable, .. } => {
                assert_eq!(pattern.to_string(), "count");
                assert!(*mutable);
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_do_directive() {
        let ir = parse("{$do println!(\"test\")}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::Do { code } => {
                assert!(code.to_string().contains("println"));
            }
            _ => panic!("Expected Do"),
        }
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_adjacent_placeholders() {
        let ir = parse("@{a}@{b}@{c}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 3);
    }

    #[test]
    fn test_placeholder_in_string() {
        // String interpolation
        let ir = parse("`hello @{name}`");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::StringInterp { quote, parts } => {
                assert_eq!(*quote, '`');
                assert!(!parts.is_empty());
            }
            _ => panic!("Expected StringInterp, got {:?}", ir.nodes[0]),
        }
    }

    #[test]
    fn test_complex_rust_expr() {
        let ir = parse("@{vec![1, 2, 3].iter().map(|x| x * 2).collect::<Vec<_>>()}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        // TokenStream.to_string() may format differently, check for key parts
        let expr_str = &placeholders[0].1;
        assert!(expr_str.contains("iter"), "Expected 'iter' in: {}", expr_str);
        assert!(expr_str.contains("map"), "Expected 'map' in: {}", expr_str);
        assert!(expr_str.contains("collect"), "Expected 'collect' in: {}", expr_str);
    }

    #[test]
    fn test_placeholder_with_generics() {
        let ir = parse("const x: @{HashMap<String, i32>} = map");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_semicolon_lexer() {
        // First verify lexer handles semicolons correctly
        use crate::compiler::lexer::Lexer;
        let tokens = Lexer::new("x; y").tokenize();
        let has_semi = tokens.iter().any(|t| t.kind == SyntaxKind::Semicolon);
        assert!(has_semi, "Expected Semicolon token, got: {:?}", tokens.iter().map(|t| (t.kind, &t.text)).collect::<Vec<_>>());
    }

    #[test]
    fn test_semicolon_simple() {
        // Simple semicolon test - should not hang
        let ir = parse("x; y");
        assert!(!ir.nodes.is_empty());
    }

    #[test]
    fn test_semicolon_ends_type_context() {
        // After semicolon, we should be back in expression context
        let ir = parse("const x: @{T} = 1; @{expr}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 2);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
        assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
    }

    // =========================================================================
    // Lexer integration tests
    // =========================================================================

    #[test]
    fn test_lexer_produces_colon() {
        use crate::compiler::lexer::Lexer;
        let tokens = Lexer::new("const x: T").tokenize();
        let has_colon = tokens.iter().any(|t| t.kind == SyntaxKind::Colon);
        assert!(has_colon, "Expected Colon token");
    }

    #[test]
    fn test_lexer_produces_keywords() {
        use crate::compiler::lexer::Lexer;
        let tokens = Lexer::new("const let function class interface type").tokenize();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
        assert!(kinds.contains(&SyntaxKind::ConstKw));
        assert!(kinds.contains(&SyntaxKind::LetKw));
        assert!(kinds.contains(&SyntaxKind::FunctionKw));
        assert!(kinds.contains(&SyntaxKind::ClassKw));
        assert!(kinds.contains(&SyntaxKind::InterfaceKw));
        assert!(kinds.contains(&SyntaxKind::TypeKw));
    }

    #[test]
    fn test_debug_parser_output() {
        // Debug test to see what the parser produces for a template with control flow
        let input = r#"export function @{fn_name}() {
            {#for field in fields}
            console.log(@{field});
            {/for}
        }"#;
        let ir = parse(input);

        eprintln!("=== IR Nodes ===");
        for (i, node) in ir.nodes.iter().enumerate() {
            eprintln!("Node {}: {:?}", i, node);
        }

        // Helper to recursively find For nodes
        fn has_for_node(node: &IrNode) -> bool {
            match node {
                IrNode::For { .. } => true,
                IrNode::FnDecl { body, .. } => body.as_ref().map(|b| has_for_node(b)).unwrap_or(false),
                IrNode::BlockStmt { stmts } => stmts.iter().any(has_for_node),
                _ => false,
            }
        }

        // Verify for loop is parsed correctly (now inside FnDecl.body)
        let has_for = ir.nodes.iter().any(|n| has_for_node(n));
        assert!(has_for, "Expected For node in IR (may be nested inside FnDecl)");

        // Verify raw nodes don't contain control flow markers
        fn check_no_control_flow_in_raw(node: &IrNode) {
            match node {
                IrNode::Raw(text) => {
                    assert!(!text.contains("{#for"), "Raw node should not contain {{#for: {}", text);
                    assert!(!text.contains("{/for"), "Raw node should not contain {{/for: {}", text);
                }
                IrNode::FnDecl { body, .. } => {
                    if let Some(b) = body {
                        check_no_control_flow_in_raw(b);
                    }
                }
                IrNode::BlockStmt { stmts } => {
                    for stmt in stmts {
                        check_no_control_flow_in_raw(stmt);
                    }
                }
                _ => {}
            }
        }
        for node in &ir.nodes {
            check_no_control_flow_in_raw(node);
        }
    }
}
