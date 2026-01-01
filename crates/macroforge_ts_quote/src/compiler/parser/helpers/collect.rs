use super::super::*;

impl Parser {
    /// Helper to collect a brace-delimited block with placeholder support.
    pub(in super::super) fn collect_block_with_placeholders(&mut self, parts: &mut Vec<IrNode>) {
        if !self.at(SyntaxKind::LBrace) {
            return;
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
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::RBrace) => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        break;
                    }
                }
                Some(SyntaxKind::At) => {
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
                                parts.push(IrNode::IdentBlock {
                                    parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                                });
                                continue;
                            }
                        }
                        parts.push(placeholder);
                    }
                }
                // Handle any {#... opening token for control blocks
                Some(k) if k.is_brace_hash_open() => {
                    if let Some(control) = self.parse_control_block(k) {
                        parts.push(control);
                    }
                }
                Some(SyntaxKind::DollarOpen) => {
                    if let Some(directive) = self.parse_directive() {
                        parts.push(directive);
                    }
                }
                Some(SyntaxKind::DoubleQuote) => {
                    if let Ok(node) = self.parse_string_literal() {
                        parts.push(node);
                    }
                }
                Some(SyntaxKind::Backtick) => {
                    if let Ok(node) = self.parse_template_literal() {
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
    }
}
