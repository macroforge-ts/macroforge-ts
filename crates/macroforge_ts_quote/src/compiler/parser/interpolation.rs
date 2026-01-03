use super::*;

// =========================================================================
// Interpolation
// =========================================================================

impl Parser {
    pub(super) fn parse_interpolation(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        // IMPORTANT: Capture placeholder kind BEFORE consuming any tokens.
        // The RBrace token will trigger update_context which pops TypeAnnotation,
        // so we must determine the kind while the context is still intact.
        let kind = self.placeholder_kind();

        // Consume @{ (the At token includes both @ and {)
        let at_token = self.consume().ok_or_else(|| {
            ParseError::unexpected_eof(self.current_byte_offset(), "interpolation")
        })?;

        // The lexer puts all content until } in one RBrace token
        // So we just need to get the RBrace token and extract the content
        let rbrace_token = self.expect(SyntaxKind::RBrace).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("interpolation")
                .with_help("Interpolations must be closed with '}'")
        })?;

        // Combine and extract the Rust expression
        let full_text = format!("{}{}", at_token.text, rbrace_token.text);
        let rust_expr_str = full_text
            .strip_prefix("@{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.trim().to_string())
            .ok_or_else(|| {
                ParseError::new(ParseErrorKind::InvalidInterpolation, self.current_byte_offset())
                    .with_context("interpolation")
                    .with_found(&full_text)
                    .with_help("Interpolation must have format @{...}")
            })?;

        // Parse as TokenStream
        let expr = TokenStream::from_str(&rust_expr_str).map_err(|e| {
            ParseError::new(ParseErrorKind::InvalidRustExpression, self.current_byte_offset())
                .with_context("interpolation")
                .with_found(&rust_expr_str)
                .with_help(&format!("Failed to parse as Rust expression: {}", e))
        })?;

        Ok(IrNode::Placeholder {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            kind,
            expr,
        })
    }

    /// Parse an interpolated identifier - a sequence of identifiers and interpolations
    /// that form a single composite identifier (no whitespace between parts).
    ///
    /// Examples:
    /// - `@{name}` -> single placeholder (keeps original kind: Expr, Type, etc.)
    /// - `@{name}Suffix` -> IdentBlock [placeholder(Ident), "Suffix"]
    /// - `@{a}@{b}` -> IdentBlock [placeholder_a(Ident), placeholder_b(Ident)]
    /// - `Pre@{mid}Post` -> would need to start with ident (handled elsewhere)
    /// - `@{prefix}Middle@{suffix}End` -> IdentBlock [placeholder(Ident), "Middle", placeholder(Ident), "End"]
    pub(super) fn parse_interpolated_ident(&mut self) -> ParseResult<IrNode> {
        let first = self.parse_interpolation()?;

        // Collect additional parts (identifiers or more interpolations) with no whitespace
        let mut additional_parts: Vec<IrNode> = Vec::new();

        loop {
            match self.current_kind() {
                // Another interpolation immediately following
                Some(SyntaxKind::At) => {
                    let placeholder = self.parse_interpolation()?;
                    additional_parts.push(placeholder);
                }
                // An identifier immediately following
                Some(SyntaxKind::Ident) => {
                    if let Some(token) = self.consume() {
                        additional_parts.push(IrNode::ident(&token));
                    } else {
                        break;
                    }
                }
                // Any other token or whitespace - stop collecting
                _ => break,
            }
        }

        // If no additional parts, return the first placeholder as-is (keeps original kind)
        if additional_parts.is_empty() {
            return Ok(first);
        }

        // Capture the span start before moving first
        let first_span_start = first.span().start;

        // Multiple parts form a composite identifier - convert all placeholders to Ident kind
        let mut parts = vec![Self::to_ident_placeholder(first)];
        for part in additional_parts {
            parts.push(Self::to_ident_placeholder(part));
        }

        Ok(IrNode::IdentBlock {
            span: IrSpan::new(first_span_start, self.current_byte_offset()),
            parts,
        })
    }

    /// Convert a placeholder to Ident kind for identifier concatenation.
    /// Raw nodes pass through unchanged.
    fn to_ident_placeholder(node: IrNode) -> IrNode {
        match node {
            IrNode::Placeholder { span, expr, .. } => IrNode::Placeholder {
                span,
                kind: PlaceholderKind::Ident,
                expr,
            },
            other => other,
        }
    }
}
