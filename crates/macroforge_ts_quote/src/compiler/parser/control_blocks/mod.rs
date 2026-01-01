use super::*;

// =========================================================================
// Control blocks
// =========================================================================

impl Parser {
    /// Dispatch to the appropriate control block parser based on the opening token.
    /// Called when we see BraceHashIf, BraceHashFor, BraceHashWhile, or BraceHashMatch.
    pub(super) fn parse_control_block(&mut self, kind: SyntaxKind) -> Option<IrNode> {
        // Consume the opening token (e.g., {#if, {#for, etc.)
        self.consume()?;
        self.skip_whitespace();

        match kind {
            SyntaxKind::BraceHashIf => self.parse_if_block(),
            SyntaxKind::BraceHashFor => self.parse_for_block(),
            SyntaxKind::BraceHashWhile => self.parse_while_block(),
            SyntaxKind::BraceHashMatch => self.parse_match_block(),
            _ => {
                // Unknown control block - consume until }
                self.consume_until_rbrace();
                None
            }
        }
    }

    /// Parse control flow inside type context - body content is collected as raw text with placeholders.
    /// Called when we see BraceHashIf, BraceHashFor, or BraceHashWhile in type position.
    pub(super) fn parse_type_control_block(&mut self, kind: SyntaxKind) -> Option<IrNode> {
        // Consume the opening token (e.g., {#if, {#for, etc.)
        self.consume()?;
        self.skip_whitespace();

        match kind {
            SyntaxKind::BraceHashIf => self.parse_type_if_block(),
            SyntaxKind::BraceHashFor => self.parse_type_for_block(),
            SyntaxKind::BraceHashWhile => self.parse_type_while_block(),
            _ => {
                self.consume_until_rbrace();
                None
            }
        }
    }

    fn parse_type_for_block(&mut self) -> Option<IrNode> {
        // Note: {#for has already been consumed, we start at the pattern

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

        // Parse body as type content (raw text with placeholders) until {/for}
        let body = self.parse_type_block_body(&[SyntaxKind::BraceSlashFor]);

        // Consume {/for} - it's a complete token including the closing brace
        if self.at(SyntaxKind::BraceSlashFor) {
            self.consume();
        }

        Some(IrNode::For {
            pattern: Self::str_to_token_stream_or_panic(pattern_str.trim(), "type for-loop pattern"),
            iterator: Self::str_to_token_stream_or_panic(&iterator_str, "type for-loop iterator"),
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_type_if_block(&mut self) -> Option<IrNode> {
        // Note: {#if has already been consumed, we start at the condition

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream_or_panic(&condition_str, "type if-block condition");
        self.expect(SyntaxKind::RBrace);

        // Parse body as type content until {:else}, {:else if}, or {/if}
        let then_body = self.parse_type_block_body(&[
            SyntaxKind::BraceColonElse,
            SyntaxKind::BraceColonElseIf,
            SyntaxKind::BraceSlashIf,
        ]);

        // Check for else-if and else clauses
        let mut else_if_branches = Vec::new();
        let mut else_body = None;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::BraceColonElseIf) => {
                    // {:else if condition} - consume the token, then parse condition
                    self.consume();
                    self.skip_whitespace();
                    let cond_str = self.collect_rust_until(SyntaxKind::RBrace);
                    let cond = Self::str_to_token_stream_or_panic(&cond_str, "type else-if condition");
                    self.expect(SyntaxKind::RBrace);
                    let body = self.parse_type_block_body(&[
                        SyntaxKind::BraceColonElse,
                        SyntaxKind::BraceColonElseIf,
                        SyntaxKind::BraceSlashIf,
                    ]);
                    else_if_branches.push((cond, body));
                }
                Some(SyntaxKind::BraceColonElse) => {
                    // {:else} - complete token including closing brace
                    self.consume();
                    else_body = Some(self.parse_type_block_body(&[SyntaxKind::BraceSlashIf]));
                    break;
                }
                _ => break,
            }
        }

        // Consume {/if} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashIf) {
            self.consume();
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

    fn parse_type_while_block(&mut self) -> Option<IrNode> {
        // Note: {#while has already been consumed, we start at the condition

        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream_or_panic(&condition_str, "type while-block condition");
        self.expect(SyntaxKind::RBrace);

        let body = self.parse_type_block_body(&[SyntaxKind::BraceSlashWhile]);

        // Consume {/while} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashWhile) {
            self.consume();
        }

        Some(IrNode::While {
            condition,
            body: Self::merge_adjacent_text(body),
        })
    }

    /// Parse block body in type context - collects raw text with placeholders and nested control flow
    fn parse_type_block_body(&mut self, terminators: &[SyntaxKind]) -> Vec<IrNode> {
        let mut nodes = Vec::new();

        while !self.at_eof() {
            if let Some(kind) = self.current_kind() {
                if terminators.contains(&kind) {
                    break;
                }

                match kind {
                    SyntaxKind::At => {
                        // Placeholder
                        if let Ok(placeholder) = self.parse_interpolation() {
                            // Check for identifier suffix
                            if let Some(token) = self.current() {
                                if token.kind == SyntaxKind::Ident {
                                    let suffix = token.text.clone();
                                    self.consume();
                                    let ident_placeholder = match placeholder {
                                        IrNode::Placeholder { expr, .. } => IrNode::Placeholder {
                                            kind: PlaceholderKind::Ident,
                                            expr,
                                        },
                                        other => other,
                                    };
                                    nodes.push(IrNode::IdentBlock {
                                        parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                                    });
                                    continue;
                                }
                            }
                            nodes.push(placeholder);
                        }
                    }
                    // Nested control flow - handle specific opening tokens
                    SyntaxKind::BraceHashIf
                    | SyntaxKind::BraceHashFor
                    | SyntaxKind::BraceHashWhile => {
                        if let Some(control) = self.parse_type_control_block(kind) {
                            nodes.push(control);
                        }
                    }
                    _ => {
                        // Raw text
                        if let Some(token) = self.consume() {
                            nodes.push(IrNode::Raw(token.text));
                        }
                    }
                }
            } else {
                break;
            }
        }

        nodes
    }

    fn parse_if_block(&mut self) -> Option<IrNode> {
        // Note: {#if has already been consumed, we start at the condition

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream_or_panic(&condition_str, "if-block condition");
        self.expect(SyntaxKind::RBrace);

        // Parse body until {:else}, {:else if}, or {/if}
        let then_body = self.parse_block_body(&[
            SyntaxKind::BraceColonElse,
            SyntaxKind::BraceColonElseIf,
            SyntaxKind::BraceSlashIf,
        ]);

        // Check for else-if and else clauses
        let mut else_if_branches = Vec::new();
        let mut else_body = None;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::BraceColonElseIf) => {
                    // {:else if condition} - consume the token, then parse condition
                    self.consume();
                    self.skip_whitespace();
                    let cond_str = self.collect_rust_until(SyntaxKind::RBrace);
                    let cond = Self::str_to_token_stream_or_panic(&cond_str, "else-if condition");
                    self.expect(SyntaxKind::RBrace);
                    let body = self.parse_block_body(&[
                        SyntaxKind::BraceColonElse,
                        SyntaxKind::BraceColonElseIf,
                        SyntaxKind::BraceSlashIf,
                    ]);
                    else_if_branches.push((cond, body));
                }
                Some(SyntaxKind::BraceColonElse) => {
                    // {:else} - complete token including closing brace
                    self.consume();
                    else_body = Some(self.parse_block_body(&[SyntaxKind::BraceSlashIf]));
                    break;
                }
                _ => break,
            }
        }

        // Consume {/if} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashIf) {
            self.consume();
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
        // Note: {#for has already been consumed, we start at the pattern

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
        let body = self.parse_block_body(&[SyntaxKind::BraceSlashFor]);

        // Consume {/for} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashFor) {
            self.consume();
        }

        Some(IrNode::For {
            pattern: Self::str_to_token_stream_or_panic(pattern_str.trim(), "for-loop pattern"),
            iterator: Self::str_to_token_stream_or_panic(&iterator_str, "for-loop iterator"),
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_while_block(&mut self) -> Option<IrNode> {
        // Note: {#while has already been consumed, we start at the condition

        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        let body = self.parse_block_body(&[SyntaxKind::BraceSlashWhile]);

        // Consume {/while} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashWhile) {
            self.consume();
        }

        Some(IrNode::While {
            condition: Self::str_to_token_stream_or_panic(&condition_str, "while-block condition"),
            body: Self::merge_adjacent_text(body),
        })
    }

    fn parse_match_block(&mut self) -> Option<IrNode> {
        // Note: {#match has already been consumed, we start at the expression

        let expr_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        let mut arms = Vec::new();

        // Skip whitespace after the match expression before looking for cases
        self.skip_whitespace();

        // Parse cases
        while self.at(SyntaxKind::BraceColonCase) {
            // Consume {:case - the token contains "{:case" but not the pattern or closing brace
            self.consume();
            self.skip_whitespace();

            let pattern_str = self.collect_rust_until(SyntaxKind::RBrace);
            self.expect(SyntaxKind::RBrace);

            let body = self.parse_block_body(&[SyntaxKind::BraceColonCase, SyntaxKind::BraceSlashMatch]);

            arms.push(MatchArm {
                pattern: Self::str_to_token_stream_or_panic(&pattern_str, "match case pattern"),
                guard: None,
                body: Self::merge_adjacent_text(body),
            });
        }

        // Consume {/match} - complete token including closing brace
        if self.at(SyntaxKind::BraceSlashMatch) {
            self.consume();
        }

        Some(IrNode::Match {
            expr: Self::str_to_token_stream_or_panic(&expr_str, "match expression"),
            arms,
        })
    }

    pub(super) fn parse_block_body(&mut self, terminators: &[SyntaxKind]) -> Vec<IrNode> {
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

    pub(super) fn collect_rust_until(&mut self, terminator: SyntaxKind) -> String {
        let mut result = String::new();

        while !self.at_eof() && !self.at(terminator) {
            if let Some(token) = self.consume() {
                result.push_str(&token.text);
            }
        }

        result.trim().to_string()
    }

    /// Convert a string to a TokenStream, returning an error if parsing fails.
    pub(super) fn str_to_token_stream(s: &str) -> ParseResult<TokenStream> {
        TokenStream::from_str(s).map_err(|e| {
            ParseError::new(ParseErrorKind::InvalidRustExpression, 0)
                .with_context("Rust expression")
                .with_found(s)
                .with_help(&format!(
                    "Failed to parse as Rust token stream: {}. \
                     Ensure the expression is valid Rust syntax.",
                    e
                ))
        })
    }

    /// Convert a string to a TokenStream, panicking with a detailed error if parsing fails.
    /// Use this in contexts where returning an error is not possible (e.g., inside control blocks).
    pub(super) fn str_to_token_stream_or_panic(s: &str, context: &str) -> TokenStream {
        Self::str_to_token_stream(s).unwrap_or_else(|e| {
            panic!(
                "Failed to parse Rust expression in {}: '{}'\n{}",
                context,
                s,
                e.to_message()
            )
        })
    }
}
