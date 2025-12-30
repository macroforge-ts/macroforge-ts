//! Parser for the template language.
//!
//! This parser produces an AST directly from tokens, with inline placeholder
//! classification based on syntactic context.

use super::ir::{Ir, IrNode, PlaceholderKind};
use super::lexer::{Lexer, Token};
use super::syntax::SyntaxKind;

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
        }
    }

    /// Parses the input and returns an AST.
    pub fn parse(mut self) -> Ir {
        let nodes = self.parse_nodes();
        Ir::with_nodes(Self::merge_adjacent_text(nodes))
    }

    /// Merges adjacent Text nodes.
    fn merge_adjacent_text(nodes: Vec<IrNode>) -> Vec<IrNode> {
        let mut result = Vec::with_capacity(nodes.len());
        let mut pending_text = String::new();

        for node in nodes {
            match node {
                IrNode::Text(text) => {
                    pending_text.push_str(&text);
                }
                other => {
                    if !pending_text.is_empty() {
                        result.push(IrNode::Text(std::mem::take(&mut pending_text)));
                    }
                    result.push(other);
                }
            }
        }

        if !pending_text.is_empty() {
            result.push(IrNode::Text(pending_text));
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

    fn update_context(&mut self, kind: SyntaxKind, _text: &str) {
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

            // Semicolon ends expression and type contexts
            SyntaxKind::Semicolon => {
                while matches!(
                    self.current_context(),
                    Context::Expression(_)
                        | Context::TypeAnnotation
                        | Context::TypeAssertion
                        | Context::GenericParams
                ) {
                    self.pop_context();
                }
            }

            // Closing brace ends type contexts
            SyntaxKind::RBrace => {
                // Pop object literal context if we're in one
                if self.is_object_literal() {
                    self.pop_context();
                }
                // Pop any remaining type contexts
                while matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                ) {
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
        match self.current_kind()? {
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
            SyntaxKind::DocCommentPrefix | SyntaxKind::JsDocOpen => self.parse_doc_comment(),
            SyntaxKind::DoubleQuote => self.parse_string_literal(),
            SyntaxKind::Backtick => self.parse_template_literal(),
            _ => self.parse_text_token(),
        }
    }

    fn parse_text_token(&mut self) -> Option<IrNode> {
        let token = self.consume()?;
        Some(IrNode::Text(token.text))
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
        let rust_expr = full_text
            .strip_prefix("@{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| full_text);

        Some(IrNode::Placeholder { kind, rust_expr })
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
        let condition = self.collect_rust_until(SyntaxKind::RBrace);
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
                    let cond = self.collect_rust_until(SyntaxKind::RBrace);
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
        let mut pattern = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::InKw) && !self.at(SyntaxKind::RBrace) {
            if let Some(token) = self.consume() {
                pattern.push_str(&token.text);
            }
        }

        self.expect(SyntaxKind::InKw);
        self.skip_whitespace();

        let iterator = self.collect_rust_until(SyntaxKind::RBrace);
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
            pattern: pattern.trim().to_string(),
            iterator,
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_while_block(&mut self) -> Option<IrNode> {
        // Consume "while"
        self.consume()?;
        self.skip_whitespace();

        let condition = self.collect_rust_until(SyntaxKind::RBrace);
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
            condition,
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_match_block(&mut self) -> Option<IrNode> {
        // Consume "match"
        self.consume()?;
        self.skip_whitespace();

        let expr = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        let mut arms = Vec::new();

        // Parse cases
        while self.at(SyntaxKind::ColonOpen) {
            self.consume(); // {:
            self.skip_whitespace();

            if self.at(SyntaxKind::CaseKw) {
                self.consume(); // case
                self.skip_whitespace();

                let pattern = self.collect_rust_until(SyntaxKind::RBrace);
                self.expect(SyntaxKind::RBrace);

                let body =
                    self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);

                arms.push((pattern, None, Self::merge_adjacent_text(body)));
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

        Some(IrNode::Match { expr, arms })
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
            let value = content[eq_pos + 1..].trim().to_string();

            let (name, type_hint) = if let Some(colon_pos) = name_part.find(':') {
                (
                    name_part[..colon_pos].trim().to_string(),
                    Some(name_part[colon_pos + 1..].trim().to_string()),
                )
            } else {
                (name_part.to_string(), None)
            };

            Some(IrNode::Let {
                name,
                mutable,
                type_hint,
                value,
            })
        } else {
            None
        }
    }

    fn parse_do_directive(&mut self) -> Option<IrNode> {
        // Consume "do"
        self.consume()?;
        self.skip_whitespace();

        let code = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::Do { code })
    }

    fn parse_typescript_directive(&mut self) -> Option<IrNode> {
        // Consume "typescript"
        self.consume()?;
        self.skip_whitespace();

        let stream = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::TypeScript { stream })
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
                parts.push(IrNode::Text(token.text));
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
                parts.push(IrNode::Text(token.text));
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
                parts.push(IrNode::Text(token.text));
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

        Some(IrNode::Comment {
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

        Some(IrNode::Comment {
            text: text.trim().to_string(),
        })
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

        Some(IrNode::Comment {
            text: text.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Ir {
        Parser::new(input).parse()
    }

    #[test]
    fn test_simple_text() {
        let ast = parse("hello world");
        assert_eq!(ast.nodes.len(), 1);
        match &ast.nodes[0] {
            IrNode::Text(text) => assert!(text.contains("hello")),
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn test_interpolation() {
        let ast = parse("@{expr}");
        assert_eq!(ast.nodes.len(), 1);
        match &ast.nodes[0] {
            IrNode::Placeholder { kind, rust_expr } => {
                assert_eq!(*kind, PlaceholderKind::Expr);
                assert_eq!(rust_expr, "expr");
            }
            _ => panic!("Expected Placeholder"),
        }
    }

    #[test]
    fn test_type_annotation() {
        let ast = parse("const x: @{T} = 1");
        // Find the placeholder
        let placeholder = ast.nodes.iter().find_map(|n| match n {
            IrNode::Placeholder { kind, rust_expr } => Some((kind, rust_expr)),
            _ => None,
        });
        assert!(placeholder.is_some());
        let (kind, expr) = placeholder.unwrap();
        assert_eq!(*kind, PlaceholderKind::Type);
        assert_eq!(expr, "T");
    }

    #[test]
    fn test_for_loop() {
        let ast = parse("{#for item in items}@{item}{/for}");
        assert_eq!(ast.nodes.len(), 1);
        match &ast.nodes[0] {
            IrNode::For {
                pattern,
                iterator,
                body,
            } => {
                assert_eq!(pattern, "item");
                assert_eq!(iterator, "items");
                assert!(!body.is_empty());
            }
            _ => panic!("Expected For"),
        }
    }

    #[test]
    fn test_if_else() {
        let ast = parse("{#if cond}yes{:else}no{/if}");
        assert_eq!(ast.nodes.len(), 1);
        match &ast.nodes[0] {
            IrNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                assert_eq!(condition, "cond");
                assert!(!then_body.is_empty());
                assert!(else_body.is_some());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_debug_tokens() {
        use crate::compiler::lexer::Lexer;

        let input = "const x: @{T} = 1";
        let tokens = Lexer::new(input).tokenize();

        // Print tokens for debugging
        for (i, t) in tokens.iter().enumerate() {
            eprintln!("Token {}: {:?} {:?}", i, t.kind, t.text);
        }

        // Verify we get a Colon token
        let has_colon = tokens.iter().any(|t| t.kind == SyntaxKind::Colon);
        assert!(has_colon, "Expected Colon token in: {:?}", tokens.iter().map(|t| (t.kind, &t.text)).collect::<Vec<_>>());
    }
}
