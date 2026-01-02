use super::super::*;
use super::ParseResult;

impl Parser {
    /// Parse a TypeScript try-catch-finally statement as raw text with placeholders.
    pub(in super::super) fn parse_ts_try_stmt(&mut self) -> ParseResult<IrNode> {
        let mut parts = vec![];

        // Consume "try"
        if let Some(t) = self.consume() {
            parts.push(IrNode::Raw(t.text));
        }

        // Collect whitespace
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                if let Some(ws) = self.consume() {
                    parts.push(IrNode::Raw(ws.text));
                }
            } else {
                break;
            }
        }

        // Parse the try block
        if self.at(SyntaxKind::LBrace) {
            self.collect_block_with_placeholders(&mut parts)?;
        }

        // Skip whitespace
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                if let Some(ws) = self.consume() {
                    parts.push(IrNode::Raw(ws.text));
                }
            } else {
                break;
            }
        }

        // Parse catch clause(s)
        while self.at(SyntaxKind::CatchKw) {
            // Consume "catch"
            if let Some(t) = self.consume() {
                parts.push(IrNode::Raw(t.text));
            }

            // Skip whitespace
            while let Some(t) = self.current() {
                if t.kind == SyntaxKind::Whitespace {
                    if let Some(ws) = self.consume() {
                        parts.push(IrNode::Raw(ws.text));
                    }
                } else {
                    break;
                }
            }

            // Parse catch parameter: (e) or (e: Error) - collect until )
            if self.at(SyntaxKind::LParen) {
                let mut paren_depth = 0;
                loop {
                    if self.at_eof() {
                        break;
                    }
                    match self.current_kind() {
                        Some(SyntaxKind::LParen) => {
                            paren_depth += 1;
                            if let Some(t) = self.consume() {
                                parts.push(IrNode::Raw(t.text));
                            }
                        }
                        Some(SyntaxKind::RParen) => {
                            if let Some(t) = self.consume() {
                                parts.push(IrNode::Raw(t.text));
                            }
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            }
                        }
                        Some(SyntaxKind::At) => {
                            parts.push(self.parse_interpolation()?);
                        }
                        _ => {
                            if let Some(t) = self.consume() {
                                parts.push(IrNode::Raw(t.text));
                            }
                        }
                    }
                }
            }

            // Skip whitespace
            while let Some(t) = self.current() {
                if t.kind == SyntaxKind::Whitespace {
                    if let Some(ws) = self.consume() {
                        parts.push(IrNode::Raw(ws.text));
                    }
                } else {
                    break;
                }
            }

            // Parse catch body
            if self.at(SyntaxKind::LBrace) {
                self.collect_block_with_placeholders(&mut parts)?;
            }

            // Skip whitespace
            while let Some(t) = self.current() {
                if t.kind == SyntaxKind::Whitespace {
                    if let Some(ws) = self.consume() {
                        parts.push(IrNode::Raw(ws.text));
                    }
                } else {
                    break;
                }
            }
        }

        // Parse optional finally clause
        if self.at(SyntaxKind::FinallyKw) {
            // Consume "finally"
            if let Some(t) = self.consume() {
                parts.push(IrNode::Raw(t.text));
            }

            // Skip whitespace
            while let Some(t) = self.current() {
                if t.kind == SyntaxKind::Whitespace {
                    if let Some(ws) = self.consume() {
                        parts.push(IrNode::Raw(ws.text));
                    }
                } else {
                    break;
                }
            }

            // Parse finally body
            if self.at(SyntaxKind::LBrace) {
                self.collect_block_with_placeholders(&mut parts)?;
            }
        }

        // Merge adjacent Raw nodes
        let merged = Self::merge_adjacent_text(parts);

        // Reuse TsLoopStmt since it has the same structure
        Ok(IrNode::TsLoopStmt { parts: merged })
    }
}
