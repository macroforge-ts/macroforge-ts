use super::super::*;

impl Parser {
    pub(in super::super) fn parse_ts_ident_or_placeholder(&mut self) -> Option<IrNode> {
        if self.at(SyntaxKind::At) {
            // It's a placeholder - parse it
            let placeholder = self.parse_interpolation()?;

            // Check if there's an identifier suffix immediately after (e.g., @{name}Obj)
            if let Some(token) = self.current() {
                if token.kind == SyntaxKind::Ident {
                    let suffix = token.text.clone();
                    self.consume();

                    // Return as IdentBlock combining placeholder + suffix
                    return Some(IrNode::IdentBlock {
                        parts: vec![placeholder, IrNode::Raw(suffix)],
                    });
                }
            }

            Some(placeholder)
        } else if let Some(token) = self.current() {
            if token.kind == SyntaxKind::Ident || token.kind.is_ts_keyword() {
                let name = token.text.clone();
                self.consume();
                Some(IrNode::Ident(name))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(in super::super) fn parse_optional_type_params(&mut self) -> Option<Box<IrNode>> {
        if !self.at(SyntaxKind::Lt) {
            return None;
        }

        self.consume(); // <
        let mut parts = Vec::new();

        let mut angle_depth = 1;
        while !self.at_eof() && angle_depth > 0 {
            let text = self.current().map(|t| t.text.as_str()).unwrap_or("");
            match self.current_kind() {
                Some(SyntaxKind::Lt) => {
                    angle_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::Gt) => {
                    // Handle >> as two >
                    if text == ">>" {
                        angle_depth -= 2;
                    } else {
                        angle_depth -= 1;
                    }
                    if angle_depth >= 0 {
                        if let Some(t) = self.consume() {
                            if angle_depth > 0 {
                                parts.push(IrNode::Raw(t.text));
                            }
                        }
                    }
                }
                Some(SyntaxKind::At) => {
                    if let Some(node) = self.parse_interpolation() {
                        parts.push(node);
                    }
                }
                _ => {
                    // Check for >> as text (in case it's not tokenized as Gt)
                    if text == ">>" {
                        angle_depth -= 2;
                        if angle_depth >= 0 {
                            if let Some(t) = self.consume() {
                                if angle_depth > 0 {
                                    parts.push(IrNode::Raw(t.text));
                                }
                            }
                        }
                    } else {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                }
            }
        }

        Some(Box::new(IrNode::TypeParams {
            params: Self::merge_adjacent_text(parts),
        }))
    }

    pub(in super::super) fn parse_param_list(&mut self) -> Vec<IrNode> {
        if !self.at(SyntaxKind::LParen) {
            return vec![];
        }

        self.consume(); // (
        let mut params = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RParen) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RParen) {
                break;
            }

            if let Some(param) = self.parse_param() {
                params.push(param);
            } else {
                // If parse_param returns None, advance to prevent infinite loop
                // This can happen with unexpected tokens like string literals in param position
                if !self.at(SyntaxKind::RParen) && !self.at(SyntaxKind::Comma) {
                    self.advance();
                }
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RParen);
        params
    }

    fn parse_param(&mut self) -> Option<IrNode> {
        self.skip_whitespace();

        // Parse optional modifiers (for constructor parameter properties)
        // These are stored but currently we use simplified Param structure
        let mut _accessibility = None;
        let mut _readonly = false;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::PublicKw) => {
                    _accessibility = Some(Accessibility::Public);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PrivateKw) => {
                    _accessibility = Some(Accessibility::Private);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ProtectedKw) => {
                    _accessibility = Some(Accessibility::Protected);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ReadonlyKw) => {
                    _readonly = true;
                    self.consume();
                    self.skip_whitespace();
                }
                _ => break,
            }
        }

        // Rest parameter? (check for `...` text)
        let rest = if self.current_text() == Some("...") {
            self.consume();
            true
        } else {
            false
        };

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            self.push_context(Context::TypeAnnotation);
            let ty = self.parse_type_until(&[SyntaxKind::Comma, SyntaxKind::RParen, SyntaxKind::Eq])?;
            self.pop_context();
            Some(Box::new(ty))
        } else {
            None
        };

        let default_value = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[
                SyntaxKind::Comma,
                SyntaxKind::RParen,
            ])?))
        } else {
            None
        };

        // Build the pattern
        let binding = IrNode::BindingIdent {
            name: Box::new(name),
            type_ann,
            optional,
        };

        // Wrap with rest or assign pattern if needed
        let pat = if rest {
            Box::new(IrNode::RestPat {
                arg: Box::new(binding),
                type_ann: None, // type_ann is already on BindingIdent
            })
        } else if let Some(right) = default_value {
            Box::new(IrNode::AssignPat {
                left: Box::new(binding),
                right,
            })
        } else {
            Box::new(binding)
        };

        Some(IrNode::Param {
            decorators: vec![],
            pat,
        })
    }

    pub(in super::super) fn parse_type_until(&mut self, terminators: &[SyntaxKind]) -> Option<IrNode> {
        let mut parts = Vec::new();
        let mut depth = 0; // Track nested brackets/parens

        while !self.at_eof() {
            let kind = self.current_kind()?;
            let text = self.current().map(|t| t.text.as_str()).unwrap_or("");

            // Only terminate at depth 0
            if depth == 0 && terminators.contains(&kind) {
                break;
            }

            match kind {
                SyntaxKind::Lt | SyntaxKind::LParen | SyntaxKind::LBrace | SyntaxKind::LBracket => {
                    depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::Gt | SyntaxKind::RParen | SyntaxKind::RBrace | SyntaxKind::RBracket => {
                    // Handle >> as two >
                    if text == ">>" {
                        depth -= 2;
                    } else {
                        depth -= 1;
                    }
                    if depth < 0 {
                        break;
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::At => {
                    if let Some(placeholder) = self.parse_interpolation() {
                        // Check if there's an identifier suffix immediately after
                        if let Some(token) = self.current() {
                            if token.kind == SyntaxKind::Ident {
                                let suffix = token.text.clone();
                                self.consume();
                                // Force placeholder to be Ident kind for identifier concatenation
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
                // Control flow blocks within types (e.g., {#for ...} inside object types)
                SyntaxKind::HashOpen => {
                    #[cfg(debug_assertions)]
                    if std::env::var("MF_DEBUG_PARSER").is_ok() {
                        eprintln!(
                            "[MF_DEBUG_PARSER] parse_type_until: found HashOpen, calling parse_type_control_block"
                        );
                    }
                    if let Some(control) = self.parse_type_control_block() {
                        #[cfg(debug_assertions)]
                        if std::env::var("MF_DEBUG_PARSER").is_ok() {
                            eprintln!(
                                "[MF_DEBUG_PARSER] parse_type_until: got control node: {:?}",
                                control
                            );
                        }
                        parts.push(control);
                    }
                }
                _ => {
                    // Handle >> as text (in case it's not tokenized as Gt)
                    if text == ">>" {
                        depth -= 2;
                        if depth < 0 {
                            break;
                        }
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
            }
        }

        if parts.is_empty() {
            None
        } else {
            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_PARSER").is_ok() {
                eprintln!(
                    "[MF_DEBUG_PARSER] parse_type_until: parts before merge: {:?}",
                    parts
                );
            }
            let merged = Self::merge_adjacent_text(parts);
            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_PARSER").is_ok() {
                eprintln!(
                    "[MF_DEBUG_PARSER] parse_type_until: merged parts: {:?}",
                    merged
                );
            }
            if merged.len() == 1 {
                Some(merged.into_iter().next().unwrap())
            } else {
                // Wrap multiple parts in TypeAnnotation for complex types with placeholders
                Some(IrNode::TypeAnnotation {
                    type_ann: Box::new(IrNode::IdentBlock { parts: merged }),
                })
            }
        }
    }

    pub(in super::super) fn parse_type_list_until(&mut self, terminator: SyntaxKind) -> Vec<IrNode> {
        let mut types = Vec::new();

        while !self.at_eof() && !self.at(terminator) {
            self.skip_whitespace();
            if let Some(ty) = self.parse_type_until(&[SyntaxKind::Comma, terminator]) {
                types.push(ty);
            }
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        types
    }

    pub(in super::super) fn parse_ts_expr_until(&mut self, terminators: &[SyntaxKind]) -> Option<IrNode> {
        let mut parts = Vec::new();
        let mut depth = 0;

        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        while !self.at_eof() {
            let kind = self.current_kind()?;

            #[cfg(debug_assertions)]
            if debug_parser {
                if let Some(token) = self.current() {
                    eprintln!(
                        "[MF_DEBUG_PARSER] parse_ts_expr_until: kind={:?}, text={:?}, depth={}",
                        kind, token.text, depth
                    );
                }
            }

            // Only terminate at depth 0
            if depth == 0 && terminators.contains(&kind) {
                break;
            }

            // Don't consume control flow markers
            if depth == 0
                && (kind == SyntaxKind::HashOpen
                    || kind == SyntaxKind::SlashOpen
                    || kind == SyntaxKind::ColonOpen)
            {
                break;
            }

            match kind {
                SyntaxKind::LParen | SyntaxKind::LBrace | SyntaxKind::LBracket => {
                    depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::RParen | SyntaxKind::RBrace | SyntaxKind::RBracket => {
                    depth -= 1;
                    if depth < 0 {
                        break;
                    }
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::At => {
                    if let Some(placeholder) = self.parse_interpolation() {
                        // Check if there's an identifier suffix immediately after (e.g., @{name}Obj)
                        if let Some(token) = self.current() {
                            if token.kind == SyntaxKind::Ident {
                                let suffix = token.text.clone();
                                self.consume();
                                // Force placeholder to be Ident kind for identifier concatenation
                                let ident_placeholder = match placeholder {
                                    IrNode::Placeholder { expr, .. } => IrNode::Placeholder {
                                        kind: PlaceholderKind::Ident,
                                        expr,
                                    },
                                    other => other,
                                };
                                // Create an IdentBlock combining placeholder + suffix
                                parts.push(IrNode::IdentBlock {
                                    parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                                });
                                continue;
                            }
                        }
                        parts.push(placeholder);
                    }
                }
                SyntaxKind::Backtick => {
                    if let Some(node) = self.parse_template_literal() {
                        parts.push(node);
                    }
                }
                SyntaxKind::DoubleQuote => {
                    if let Some(node) = self.parse_string_literal() {
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

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG_PARSER] parse_ts_expr_until: collected {} parts",
                parts.len()
            );
            for (i, p) in parts.iter().enumerate() {
                eprintln!("[MF_DEBUG_PARSER]   part {}: {:?}", i, p);
            }
        }

        if parts.is_empty() {
            None
        } else {
            let merged = Self::merge_adjacent_text(parts);
            if merged.len() == 1 {
                Some(merged.into_iter().next().unwrap())
            } else {
                // Wrap multiple parts in IdentBlock for complex expressions with placeholders
                Some(IrNode::IdentBlock { parts: merged })
            }
        }
    }
}
