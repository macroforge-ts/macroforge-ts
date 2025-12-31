use super::*;

// =========================================================================
// Interpolation
// =========================================================================

impl Parser {
    pub(super) fn parse_interpolation(&mut self) -> Option<IrNode> {
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
            TokenStream::from_str(&format!("{{ {} }}", rust_expr_str)).unwrap_or_default()
        });

        Some(IrNode::Placeholder { kind, expr })
    }
}
