use super::super::*;
use super::ParseResult;

impl Parser {
    /// Helper to collect a brace-delimited block with placeholder support.
    pub(in super::super) fn collect_block_with_placeholders(&mut self, parts: &mut Vec<IrNode>) -> ParseResult<()> {
        if !self.at(SyntaxKind::LBrace) {
            return Ok(());
        }

        let mut brace_depth = 0;
        loop {
            if self.at_eof() {
                break;
            }

            match self.current_kind() {
                Some(SyntaxKind::LBrace) => {
                    brace_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::ident(&t));
                    }
                }
                Some(SyntaxKind::RBrace) => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::ident(&t));
                    }
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        break;
                    }
                }
                Some(SyntaxKind::At) => {
                    let placeholder = self.parse_interpolation()?;
                    // Check for identifier suffix
                    if let Some(token) = self.current() {
                        if token.kind == SyntaxKind::Ident {
                            let suffix_token = self.consume().unwrap();
                            let ident_placeholder = match placeholder {
                                IrNode::Placeholder { span, expr, .. } => IrNode::Placeholder {
                                    span,
                                    kind: PlaceholderKind::Ident,
                                    expr,
                                },
                                other => other,
                            };
                            parts.push(IrNode::IdentBlock {
                                span: IrSpan::empty(),
                                parts: vec![ident_placeholder, IrNode::ident(&suffix_token)],
                            });
                            continue;
                        }
                    }
                    parts.push(placeholder);
                }
                // Handle any {#... opening token for control blocks
                Some(k) if k.is_brace_hash_open() => {
                    parts.push(self.parse_control_block(k)?);
                }
                Some(SyntaxKind::DollarOpen) => {
                    if let Some(directive) = self.parse_directive() {
                        parts.push(directive);
                    }
                }
                Some(SyntaxKind::DoubleQuote) => {
                    parts.push(self.parse_string_literal()?);
                }
                Some(SyntaxKind::Backtick) => {
                    parts.push(self.parse_template_literal()?);
                }
                _ => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::ident(&t));
                    }
                }
            }
        }
        Ok(())
    }
}
