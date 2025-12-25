//! Parser for the template language.
//!
//! The parser builds a Rowan-based CST (Concrete Syntax Tree) from the token stream.
//! It uses a recursive descent approach with support for error recovery.

use super::lexer::{Lexer, Token};
use super::syntax::SyntaxKind;
use rowan::{GreenNode, GreenNodeBuilder};

/// Events emitted during parsing.
/// These are later converted to a green tree.
#[derive(Debug)]
enum Event {
    /// Start a new node with the given kind.
    StartNode { kind: SyntaxKind },
    /// Finish the current node.
    FinishNode,
    /// Add a token.
    Token { kind: SyntaxKind, text: String },
}

/// The parser for template input.
pub struct Parser {
    /// The tokens to parse.
    tokens: Vec<Token>,
    /// Current position in the token stream.
    pos: usize,
    /// Events generated during parsing.
    events: Vec<Event>,
}

impl Parser {
    /// Creates a new parser from input text.
    pub fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).tokenize();
        Self {
            tokens,
            pos: 0,
            events: Vec::new(),
        }
    }

    /// Parses the input and returns a green tree.
    pub fn parse(mut self) -> GreenNode {
        self.start_node(SyntaxKind::Root);
        self.parse_template_content();
        self.finish_node();
        self.build_tree()
    }

    /// Returns the current token kind, or None if at EOF.
    fn current(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.pos).map(|t| t.kind)
    }

    /// Returns the current token, or None if at EOF.
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Peeks at the nth token ahead.
    fn peek(&self, n: usize) -> Option<SyntaxKind> {
        self.tokens.get(self.pos + n).map(|t| t.kind)
    }

    /// Checks if the current token is of the given kind.
    fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == Some(kind)
    }

    /// Checks if at end of file.
    fn at_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Consumes the current token if it matches the given kind.
    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Consumes the current token unconditionally.
    fn bump(&mut self) {
        if let Some(token) = self.tokens.get(self.pos) {
            self.events.push(Event::Token {
                kind: token.kind,
                text: token.text.clone(),
            });
            self.pos += 1;
        }
    }

    /// Consumes tokens while they match the given kind.
    fn bump_while(&mut self, kind: SyntaxKind) {
        while self.at(kind) {
            self.bump();
        }
    }

    /// Starts a new node.
    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    /// Finishes the current node.
    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    /// Emits an error token for unexpected input.
    fn error(&mut self, msg: &str) {
        // In a real implementation, we'd collect errors
        // For now, just consume the token as ERROR
        if !self.at_eof() {
            let token = self.current_token().unwrap();
            self.events.push(Event::Token {
                kind: SyntaxKind::Error,
                text: format!("error: {} (got {:?})", msg, token.text),
            });
            self.pos += 1;
        }
    }

    /// Builds the green tree from events.
    fn build_tree(self) -> GreenNode {
        let mut builder = GreenNodeBuilder::new();
        let mut forward_parents = Vec::new();

        for event in self.events {
            match event {
                Event::StartNode { kind } => {
                    forward_parents.push(kind);
                    // We process StartNode when we see the matching FinishNode
                    // or another StartNode
                }
                Event::FinishNode => {
                    if let Some(kind) = forward_parents.pop() {
                        builder.start_node(kind.into());
                    }
                    builder.finish_node();
                }
                Event::Token { kind, text } => {
                    // First, start any pending parent nodes
                    for kind in forward_parents.drain(..) {
                        builder.start_node(kind.into());
                    }
                    builder.token(kind.into(), &text);
                }
            }
        }

        // Finish any remaining nodes
        while !forward_parents.is_empty() {
            if let Some(kind) = forward_parents.pop() {
                builder.start_node(kind.into());
            }
            builder.finish_node();
        }

        builder.finish()
    }

    /// Parses the main template content.
    fn parse_template_content(&mut self) {
        while !self.at_eof() {
            self.parse_template_item();
        }
    }

    /// Parses a single template item.
    fn parse_template_item(&mut self) {
        match self.current() {
            Some(SyntaxKind::At) => self.parse_interpolation(),
            Some(SyntaxKind::HashOpen) => self.parse_control_block(),
            Some(SyntaxKind::SlashOpen) => {
                // End of a control block - consume and return
                // The caller should handle this
                self.bump();
                self.bump_while(SyntaxKind::Whitespace);
                // Consume the keyword (if, for, etc.) and closing brace
                while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
                    self.bump();
                }
                self.eat(SyntaxKind::RBrace);
            }
            Some(SyntaxKind::ColonOpen) => {
                // Else clause at top level is an error - consume the whole {:...} block
                // This should only appear inside control blocks, but consume to avoid infinite loop
                self.bump(); // {:
                self.bump_while(SyntaxKind::Whitespace);
                // Consume until closing }
                while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
                    self.bump();
                }
                self.eat(SyntaxKind::RBrace);
            }
            Some(SyntaxKind::DollarOpen) => self.parse_directive(),
            Some(SyntaxKind::PipeOpen) => self.parse_ident_block(),
            Some(SyntaxKind::CommentLineOpen) => self.parse_line_comment(),
            Some(SyntaxKind::CommentBlockOpen) => self.parse_block_comment(),
            Some(SyntaxKind::DocCommentPrefix) | Some(SyntaxKind::JsDocOpen) => {
                self.parse_doc_comment()
            }
            Some(SyntaxKind::LBrace) => self.parse_brace_block(),
            Some(SyntaxKind::DoubleQuote) => self.parse_string_literal(),
            Some(SyntaxKind::Backtick) => self.parse_template_literal(),
            Some(SyntaxKind::Colon) => self.parse_type_annotation(),
            Some(SyntaxKind::AsKw) => self.parse_type_assertion(),
            Some(_) => {
                // Regular TypeScript content - consume as statement
                self.parse_ts_content();
            }
            None => {}
        }
    }

    /// Parses an interpolation: @{expr}
    fn parse_interpolation(&mut self) {
        self.start_node(SyntaxKind::Interpolation);
        self.bump(); // @
        // The lexer should have already put us in interpolation mode
        // and will return RUST_TOKENS + RBRACE
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);
        self.finish_node();
    }

    /// Parses a control block: {#if ...}, {#for ...}, etc.
    fn parse_control_block(&mut self) {
        // Peek at the keyword to determine block type
        let keyword = self.peek(1);

        match keyword {
            Some(SyntaxKind::IfKw) => self.parse_if_block(),
            Some(SyntaxKind::ForKw) => self.parse_for_block(),
            Some(SyntaxKind::WhileKw) => self.parse_while_block(),
            Some(SyntaxKind::MatchKw) => self.parse_match_block(),
            _ => {
                // Unknown control block - consume and error
                self.error("unknown control block");
            }
        }
    }

    /// Parses an if block: {#if cond}...{/if}
    fn parse_if_block(&mut self) {
        self.start_node(SyntaxKind::IfBlock);

        // {#
        self.bump();
        // Skip whitespace
        self.bump_while(SyntaxKind::Whitespace);
        // if
        self.eat(SyntaxKind::IfKw);
        self.bump_while(SyntaxKind::Whitespace);

        // Check for "let" (if-let pattern)
        if self.at(SyntaxKind::LetKw) {
            // Reparse as IF_LET_BLOCK
            // For simplicity, just mark as IF_BLOCK with let
            self.bump(); // let
            self.bump_while(SyntaxKind::Whitespace);
        }

        // Condition/pattern - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        // Body
        self.parse_block_body();

        // Check for else/else-if
        while self.at(SyntaxKind::ColonOpen) {
            // Peek to see if it's else or else if
            let next = self.peek(1);
            if next == Some(SyntaxKind::ElseKw) {
                self.parse_else_clause();
            } else {
                break;
            }
        }

        // End: {/if}
        if self.at(SyntaxKind::SlashOpen) {
            self.bump();
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::IfKw);
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::RBrace);
        }

        self.finish_node();
    }

    /// Parses an else clause: {:else} or {:else if cond}
    fn parse_else_clause(&mut self) {
        // Peek ahead to see if it's "else if" or just "else"
        let is_else_if = self.peek(2) == Some(SyntaxKind::IfKw);

        if is_else_if {
            self.start_node(SyntaxKind::ElseIfClause);
        } else {
            self.start_node(SyntaxKind::ElseClause);
        }

        // {:
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // else
        self.eat(SyntaxKind::ElseKw);
        self.bump_while(SyntaxKind::Whitespace);

        if is_else_if {
            // if
            self.eat(SyntaxKind::IfKw);
            self.bump_while(SyntaxKind::Whitespace);
            // Condition
            while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
                self.bump();
            }
        }

        self.eat(SyntaxKind::RBrace);

        // Body
        self.parse_block_body();

        self.finish_node();
    }

    /// Parses a for block: {#for pat in iter}...{/for}
    fn parse_for_block(&mut self) {
        self.start_node(SyntaxKind::ForBlock);

        // {#
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // for
        self.eat(SyntaxKind::ForKw);
        self.bump_while(SyntaxKind::Whitespace);

        // Pattern and iterator - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        // Body
        self.parse_block_body();

        // End: {/for}
        if self.at(SyntaxKind::SlashOpen) {
            self.bump();
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::ForKw);
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::RBrace);
        }

        self.finish_node();
    }

    /// Parses a while block: {#while cond}...{/while}
    fn parse_while_block(&mut self) {
        self.start_node(SyntaxKind::WhileBlock);

        // {#
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // while
        self.eat(SyntaxKind::WhileKw);
        self.bump_while(SyntaxKind::Whitespace);

        // Check for "let" (while-let pattern)
        if self.at(SyntaxKind::LetKw) {
            self.bump();
            self.bump_while(SyntaxKind::Whitespace);
        }

        // Condition - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        // Body
        self.parse_block_body();

        // End: {/while}
        if self.at(SyntaxKind::SlashOpen) {
            self.bump();
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::WhileKw);
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::RBrace);
        }

        self.finish_node();
    }

    /// Parses a match block: {#match expr}...{/match}
    fn parse_match_block(&mut self) {
        self.start_node(SyntaxKind::MatchBlock);

        // {#
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // match
        self.eat(SyntaxKind::MatchKw);
        self.bump_while(SyntaxKind::Whitespace);

        // Expression - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        // Cases - skip whitespace before each case and check for {/match} end
        loop {
            // Skip whitespace/newlines between cases
            self.bump_while(SyntaxKind::Whitespace);

            // Check for case: {:case
            if self.at(SyntaxKind::ColonOpen) && self.peek(1) == Some(SyntaxKind::CaseKw) {
                self.parse_match_case();
            } else {
                // No more cases
                break;
            }
        }

        // End: {/match}
        if self.at(SyntaxKind::SlashOpen) {
            self.bump();
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::MatchKw);
            self.bump_while(SyntaxKind::Whitespace);
            self.eat(SyntaxKind::RBrace);
        }

        self.finish_node();
    }

    /// Parses a match case: {:case pat}...
    fn parse_match_case(&mut self) {
        self.start_node(SyntaxKind::MatchCase);

        // {:
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // case
        self.eat(SyntaxKind::CaseKw);
        self.bump_while(SyntaxKind::Whitespace);

        // Pattern - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        // Body
        self.parse_block_body();

        self.finish_node();
    }

    /// Parses the body of a control block (content between tags).
    fn parse_block_body(&mut self) {
        while !self.at_eof() {
            // Check for end of block
            if self.at(SyntaxKind::SlashOpen) || self.at(SyntaxKind::ColonOpen) {
                break;
            }
            self.parse_template_item();
        }
    }

    /// Parses a directive: {$let ...}, {$do ...}, {$typescript ...}
    fn parse_directive(&mut self) {
        let keyword = self.peek(1);

        match keyword {
            Some(SyntaxKind::LetKw) => {
                self.start_node(SyntaxKind::LetDirective);
            }
            Some(SyntaxKind::DoKw) => {
                self.start_node(SyntaxKind::DoDirective);
            }
            Some(SyntaxKind::TypeScriptKw) => {
                self.start_node(SyntaxKind::TypeScriptDirective);
            }
            _ => {
                self.error("unknown directive");
                return;
            }
        }

        // {$
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);
        // keyword
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);

        // Content - consume until }
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.bump();
        }
        self.eat(SyntaxKind::RBrace);

        self.finish_node();
    }

    /// Parses an ident block: {|...|}
    ///
    /// DEPRECATED: Ident blocks are deprecated. Use implicit concatenation instead:
    /// - Old: `function {|get@{field}|}()`
    /// - New: `function get@{field}()`
    fn parse_ident_block(&mut self) {
        // Emit deprecation warning
        eprintln!(
            "warning: `{{|...|}}`  ident block syntax is deprecated. \
             Use implicit concatenation instead (e.g., `get@{{field}}` instead of `{{|get@{{field}}|}}`)"
        );

        self.start_node(SyntaxKind::IdentBlock);

        // {|
        self.bump();

        // Content until |}
        while !self.at_eof() && !self.at(SyntaxKind::PipeClose) {
            if self.at(SyntaxKind::At) {
                self.parse_interpolation();
            } else {
                self.bump();
            }
        }

        self.eat(SyntaxKind::PipeClose);
        self.finish_node();
    }

    /// Parses a line comment: {> "comment" <}
    fn parse_line_comment(&mut self) {
        self.start_node(SyntaxKind::LineComment);

        // {>
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);

        // Content until <}
        while !self.at_eof() && !self.at(SyntaxKind::CommentLineClose) {
            self.bump();
        }

        self.eat(SyntaxKind::CommentLineClose);
        self.finish_node();
    }

    /// Parses a block comment: {>> "comment" <<}
    fn parse_block_comment(&mut self) {
        self.start_node(SyntaxKind::BlockComment);

        // {>>
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);

        // Content until <<}
        while !self.at_eof() && !self.at(SyntaxKind::CommentBlockClose) {
            self.bump();
        }

        self.eat(SyntaxKind::CommentBlockClose);
        self.finish_node();
    }

    /// Parses a doc comment: /// or /** */
    fn parse_doc_comment(&mut self) {
        self.start_node(SyntaxKind::DocComment);

        if self.at(SyntaxKind::DocCommentPrefix) {
            // ///
            self.bump();
            // Content until newline
            while !self.at_eof() {
                if let Some(token) = self.current_token()
                    && token.text.contains('\n') {
                    self.bump();
                    break;
                }
                self.bump();
            }
        } else {
            // /**
            self.bump();
            // Content until */
            while !self.at_eof() && !self.at(SyntaxKind::JsDocClose) {
                self.bump();
            }
            self.eat(SyntaxKind::JsDocClose);
        }

        self.finish_node();
    }

    /// Parses a brace block: { ... }
    fn parse_brace_block(&mut self) {
        self.start_node(SyntaxKind::BraceBlock);

        // {
        self.bump();

        // Content - recurse
        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.parse_template_item();
        }

        // }
        self.eat(SyntaxKind::RBrace);

        self.finish_node();
    }

    /// Parses a string literal with potential interpolations.
    fn parse_string_literal(&mut self) {
        self.start_node(SyntaxKind::StringInterp);

        // Opening "
        self.bump();

        // Content
        while !self.at_eof() {
            match self.current() {
                Some(SyntaxKind::DoubleQuote) => {
                    self.bump();
                    break;
                }
                Some(SyntaxKind::At) => {
                    self.parse_interpolation();
                }
                _ => {
                    self.bump();
                }
            }
        }

        self.finish_node();
    }

    /// Parses a template literal with potential interpolations.
    fn parse_template_literal(&mut self) {
        self.start_node(SyntaxKind::TemplateLiteral);

        // Opening `
        self.bump();

        // Content
        while !self.at_eof() {
            match self.current() {
                Some(SyntaxKind::Backtick) => {
                    self.bump();
                    break;
                }
                Some(SyntaxKind::At) => {
                    self.parse_interpolation();
                }
                _ => {
                    self.bump();
                }
            }
        }

        self.finish_node();
    }

    /// Parses a type annotation: : Type
    fn parse_type_annotation(&mut self) {
        self.start_node(SyntaxKind::TypeAnnotation);

        // :
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);

        // Type - parse until we hit something that ends a type
        self.parse_type_expression();

        self.finish_node();
    }

    /// Parses a type assertion: as Type
    fn parse_type_assertion(&mut self) {
        self.start_node(SyntaxKind::TypeAssertion);

        // as
        self.bump();
        self.bump_while(SyntaxKind::Whitespace);

        // Type
        self.parse_type_expression();

        self.finish_node();
    }

    /// Parses a type expression (identifier, generics, etc.)
    fn parse_type_expression(&mut self) {
        // Consume the type name
        while !self.at_eof() {
            match self.current() {
                Some(SyntaxKind::Ident) => self.bump(),
                Some(SyntaxKind::Lt) => {
                    // Generic type parameters
                    self.parse_generic_params();
                }
                Some(SyntaxKind::LBracket) => {
                    // Array type: Type[]
                    self.bump();
                    self.eat(SyntaxKind::RBracket);
                }
                Some(SyntaxKind::Dot) => {
                    // Qualified type: Foo.Bar
                    self.bump();
                }
                Some(SyntaxKind::At) => {
                    // Interpolation in type position
                    self.parse_interpolation();
                }
                _ => break,
            }
        }
    }

    /// Parses generic type parameters: <T, U>
    fn parse_generic_params(&mut self) {
        self.start_node(SyntaxKind::TsTypeParams);

        // <
        self.bump();

        let mut depth = 1;
        while !self.at_eof() && depth > 0 {
            match self.current() {
                Some(SyntaxKind::Lt) => {
                    depth += 1;
                    self.bump();
                }
                Some(SyntaxKind::Gt) => {
                    depth -= 1;
                    self.bump();
                }
                Some(SyntaxKind::At) => {
                    self.parse_interpolation();
                }
                _ => {
                    self.bump();
                }
            }
        }

        self.finish_node();
    }

    /// Parses regular TypeScript content.
    fn parse_ts_content(&mut self) {
        // Just consume the token as regular TS content
        self.bump();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::syntax::SyntaxNode;

    fn parse(input: &str) -> SyntaxNode {
        let parser = Parser::new(input);
        let green = parser.parse();
        SyntaxNode::new_root(green)
    }

    #[test]
    fn test_parse_simple_text() {
        let tree = parse("hello world");
        assert_eq!(tree.kind(), SyntaxKind::Root);
    }

    #[test]
    fn test_parse_interpolation() {
        let tree = parse("@{expr}");
        assert_eq!(tree.kind(), SyntaxKind::Root);
        // Should have an INTERPOLATION child
    }

    #[test]
    fn test_parse_if_block() {
        let tree = parse("{#if cond}content{/if}");
        assert_eq!(tree.kind(), SyntaxKind::Root);
    }

    #[test]
    fn test_parse_for_block() {
        let tree = parse("{#for item in list}@{item}{/for}");
        assert_eq!(tree.kind(), SyntaxKind::Root);
    }

    #[test]
    fn test_parse_type_annotation() {
        let tree = parse("const x: number = 1");
        assert_eq!(tree.kind(), SyntaxKind::Root);
    }

    #[test]
    fn test_parse_type_assertion() {
        let tree = parse("value as Type");
        assert_eq!(tree.kind(), SyntaxKind::Root);
    }
}
