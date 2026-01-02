use super::*;

// =========================================================================
// Directives
// =========================================================================

impl Parser {
    pub(super) fn parse_directive(&mut self) -> Option<IrNode> {
        // Save the context stack - directive contents are Rust code, not TypeScript
        // so we shouldn't let them affect the TypeScript parsing context
        let saved_context = self.context_stack.clone();

        // Consume {$
        self.consume()?;
        self.skip_whitespace();

        let result = match self.current_kind() {
            Some(SyntaxKind::LetKw) => self.parse_let_directive(),
            Some(SyntaxKind::DoKw) => self.parse_do_directive(),
            Some(SyntaxKind::TypeScriptKw) => self.parse_typescript_directive(),
            _ => {
                self.consume_until_rbrace();
                None
            }
        };

        // Restore the context stack to avoid pollution from Rust keywords in directives
        self.context_stack = saved_context;

        result
    }

    fn parse_let_directive(&mut self) -> Option<IrNode> {
        let start_byte = self.current_byte_offset();
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
                    Some(Self::str_to_token_stream_or_panic(name_part[colon_pos + 1..].trim(), "let directive type hint")),
                )
            } else {
                (name_part, None)
            };

            Some(IrNode::Let {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                pattern: Self::str_to_token_stream_or_panic(pattern_str, "let directive pattern"),
                mutable,
                type_hint,
                value: Self::str_to_token_stream_or_panic(value_str, "let directive value"),
            })
        } else {
            None
        }
    }

    fn parse_do_directive(&mut self) -> Option<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "do"
        self.consume()?;
        self.skip_whitespace();

        let code_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::Do {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            code: Self::str_to_token_stream_or_panic(&code_str, "do directive code"),
        })
    }

    fn parse_typescript_directive(&mut self) -> Option<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "typescript"
        self.consume()?;
        self.skip_whitespace();

        let stream_str = self.collect_rust_until(SyntaxKind::RBrace);
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::TypeScript {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            stream: Self::str_to_token_stream_or_panic(&stream_str, "typescript directive stream"),
        })
    }
}
