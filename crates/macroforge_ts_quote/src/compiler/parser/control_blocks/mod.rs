use super::*;

// =========================================================================
// Control blocks
// =========================================================================

impl Parser {
    pub(super) fn parse_control_block(&mut self) -> Option<IrNode> {
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

    /// Parse control flow inside type context - body content is collected as raw text with placeholders
    pub(super) fn parse_type_control_block(&mut self) -> Option<IrNode> {
        // Consume {#
        self.consume()?;
        self.skip_whitespace();

        match self.current_kind() {
            Some(SyntaxKind::IfKw) => self.parse_type_if_block(),
            Some(SyntaxKind::ForKw) => self.parse_type_for_block(),
            Some(SyntaxKind::WhileKw) => self.parse_type_while_block(),
            _ => {
                self.consume_until_rbrace();
                None
            }
        }
    }

    fn parse_type_for_block(&mut self) -> Option<IrNode> {
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

        // Parse body as type content (raw text with placeholders) until {/for}
        let body = self.parse_type_block_body(&[SyntaxKind::SlashOpen]);

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

    fn parse_type_if_block(&mut self) -> Option<IrNode> {
        // Consume "if"
        self.consume()?;
        self.skip_whitespace();

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream(&condition_str);
        self.expect(SyntaxKind::RBrace);

        // Parse body as type content until {:else}, {:else if}, or {/if}
        let then_body = self.parse_type_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);

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
                        self.parse_type_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);
                    else_if_branches.push((cond, body));
                } else {
                    // {:else}
                    self.expect(SyntaxKind::RBrace);
                    else_body = Some(self.parse_type_block_body(&[SyntaxKind::SlashOpen]));
                    break;
                }
            } else {
                self.consume_until_rbrace();
                break;
            }
        }

        // Consume {/if}
        if self.at(SyntaxKind::SlashOpen) {
            self.consume();
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

    fn parse_type_while_block(&mut self) -> Option<IrNode> {
        // Consume "while"
        self.consume()?;
        self.skip_whitespace();

        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        let condition = Self::str_to_token_stream(&condition_str);
        self.expect(SyntaxKind::RBrace);

        let body = self.parse_type_block_body(&[SyntaxKind::SlashOpen]);

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
                        if let Some(placeholder) = self.parse_interpolation() {
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
                    SyntaxKind::HashOpen => {
                        // Nested control flow
                        if let Some(control) = self.parse_type_control_block() {
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
                    let body = self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);
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

        // Skip whitespace after the match expression before looking for cases
        self.skip_whitespace();

        // Parse cases
        while self.at(SyntaxKind::ColonOpen) {
            self.consume(); // {:
            self.skip_whitespace();

            if self.at(SyntaxKind::CaseKw) {
                self.consume(); // case
                self.skip_whitespace();

                let pattern_str = self.collect_rust_until(SyntaxKind::RBrace);
                self.expect(SyntaxKind::RBrace);

                let body = self.parse_block_body(&[SyntaxKind::ColonOpen, SyntaxKind::SlashOpen]);

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

    /// Convert a string to a TokenStream, with error handling.
    pub(super) fn str_to_token_stream(s: &str) -> TokenStream {
        TokenStream::from_str(s).unwrap_or_else(|_| {
            // If parsing fails, try wrapping in braces
            TokenStream::from_str(&format!("{{ {} }}", s)).unwrap_or_default()
        })
    }
}
