use super::super::*;

impl Parser {
    /// Parse a TypeScript for/while loop as raw text with placeholders.
    /// This is needed because loops are not expressions and need special handling.
    pub(in super::super) fn parse_ts_loop_stmt(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.consume()?.text; // for/while
        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_loop_stmt: keyword={:?}", keyword);
        }
        let mut parts = vec![IrNode::Raw(keyword)];

        // Helper to add whitespace
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                parts.push(IrNode::Raw(self.consume()?.text));
            } else {
                break;
            }
        }

        // Parse the loop header in parens
        if !self.at(SyntaxKind::LParen) {
            // Try to recover - just return what we have
            return Some(IrNode::IdentBlock { parts });
        }

        let mut paren_depth = 0;
        loop {
            if self.at_eof() {
                break;
            }

            let kind = match self.current_kind() {
                Some(k) => k,
                None => break,
            };

            match kind {
                SyntaxKind::LParen => {
                    paren_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::RParen => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                }
                SyntaxKind::At => {
                    if let Some(placeholder) = self.parse_interpolation() {
                        // Check for identifier suffix
                        if let Some(token) = self.current() {
                            if token.kind == SyntaxKind::Ident {
                                let suffix = token.text.clone();
                                self.consume();
                                let ident_placeholder = match placeholder {
                                    IrNode::Placeholder { expr, .. } => {
                                        IrNode::Placeholder { kind: PlaceholderKind::Ident, expr }
                                    }
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
                SyntaxKind::DoubleQuote => {
                    if let Some(node) = self.parse_string_literal() {
                        parts.push(node);
                    }
                }
                SyntaxKind::Backtick => {
                    if let Some(node) = self.parse_template_literal() {
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

        // Skip whitespace before body
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                parts.push(IrNode::Raw(self.consume()?.text));
            } else {
                break;
            }
        }

        // Parse the body block
        if self.at(SyntaxKind::LBrace) {
            let mut brace_depth = 0;
            loop {
                if self.at_eof() {
                    break;
                }

                let kind = match self.current_kind() {
                    Some(k) => k,
                    None => break,
                };

                match kind {
                    SyntaxKind::LBrace => {
                        brace_depth += 1;
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                    SyntaxKind::RBrace => {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            break;
                        }
                    }
                    SyntaxKind::At => {
                        if let Some(placeholder) = self.parse_interpolation() {
                            // Check for identifier suffix
                            if let Some(token) = self.current() {
                                if token.kind == SyntaxKind::Ident {
                                    let suffix = token.text.clone();
                                    self.consume();
                                    let ident_placeholder = match placeholder {
                                        IrNode::Placeholder { expr, .. } => {
                                            IrNode::Placeholder { kind: PlaceholderKind::Ident, expr }
                                        }
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
                    SyntaxKind::DoubleQuote => {
                        if let Some(node) = self.parse_string_literal() {
                            parts.push(node);
                        }
                    }
                    SyntaxKind::Backtick => {
                        if let Some(node) = self.parse_template_literal() {
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

        // Merge adjacent Raw nodes
        let merged = Self::merge_adjacent_text(parts);

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_loop_stmt: collected {} parts", merged.len());
            for (i, part) in merged.iter().enumerate() {
                match part {
                    IrNode::Raw(text) => eprintln!("  part[{}]: Raw({:?})", i, &text[..text.len().min(50)]),
                    IrNode::Placeholder { kind, .. } => eprintln!("  part[{}]: Placeholder({:?})", i, kind),
                    IrNode::IdentBlock { parts } => eprintln!("  part[{}]: IdentBlock({} parts)", i, parts.len()),
                    IrNode::StringInterp { .. } => eprintln!("  part[{}]: StringInterp", i),
                    other => eprintln!("  part[{}]: {:?}", i, std::mem::discriminant(other)),
                }
            }
        }

        // Return as TsLoopStmt which will be handled as a structured statement
        Some(IrNode::TsLoopStmt { parts: merged })
    }
}
