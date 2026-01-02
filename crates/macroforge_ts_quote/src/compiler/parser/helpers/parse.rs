use super::super::expr::errors::{ParseError, ParseResult};
use super::super::*;

impl Parser {
    pub(in super::super) fn parse_ts_ident_or_placeholder(&mut self) -> Option<IrNode> {
        if self.at(SyntaxKind::At) {
            // It's a placeholder - parse it
            let placeholder = self.parse_interpolation().ok()?;

            // Check if there's an identifier suffix immediately after (e.g., @{name}Obj)
            if let Some(token) = self.current() {
                if token.kind == SyntaxKind::Ident {
                    let suffix_token = self.consume().unwrap();

                    // Return as IdentBlock combining placeholder + suffix
                    return Some(IrNode::IdentBlock {
                        span: IrSpan::empty(),
                        parts: vec![placeholder, IrNode::raw(&suffix_token)],
                    });
                }
            }

            Some(placeholder)
        } else if let Some(token) = self.current() {
            if token.kind == SyntaxKind::Ident || token.kind.is_ts_keyword() {
                let t = self.consume().unwrap();
                Some(IrNode::ident(&t))
            } else {
                None
            }
        } else {
            None
        }
    }

    // parse_optional_type_params is now in expr/mod.rs with proper error handling

    pub(in super::super) fn parse_param_list(&mut self) -> ParseResult<Vec<IrNode>> {
        if !self.at(SyntaxKind::LParen) {
            return Ok(vec![]);
        }

        self.consume(); // (
        let mut params = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RParen) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RParen) {
                break;
            }

            match self.parse_param() {
                Ok(param) => params.push(param),
                Err(e) => {
                    // If parse_param returns an error, check if we should recover
                    // This can happen with unexpected tokens like string literals in param position
                    if !self.at(SyntaxKind::RParen) && !self.at(SyntaxKind::Comma) {
                        // Return the error with context for better debugging
                        return Err(e.with_context("parsing function parameter list"));
                    }
                    // Otherwise we can recover by continuing to the next parameter
                }
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RParen);
        Ok(params)
    }

    fn parse_param(&mut self) -> ParseResult<IrNode> {
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

        let name = self
            .parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "parameter name"))?;
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
            self.push_context(Context::type_annotation([
                SyntaxKind::Comma,
                SyntaxKind::RParen,
                SyntaxKind::Eq,
            ]));
            let ty = self
                .parse_type_until(&[SyntaxKind::Comma, SyntaxKind::RParen, SyntaxKind::Eq])?
                .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "parameter type annotation"))?;
            self.pop_context();
            Some(Box::new(ty))
        } else {
            None
        };

        let default_value = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(
                self.parse_ts_expr_until(&[SyntaxKind::Comma, SyntaxKind::RParen])
                    .map_err(|e| e.with_context("parsing parameter default value"))?,
            ))
        } else {
            None
        };

        let name_span = name.span();

        // Build the pattern
        let binding = IrNode::BindingIdent {
            span: name_span,
            name: Box::new(name),
            type_ann,
            optional,
        };

        // Wrap with rest or assign pattern if needed
        let pat = if rest {
            Box::new(IrNode::RestPat {
                span: name_span,
                arg: Box::new(binding),
                type_ann: None, // type_ann is already on BindingIdent
            })
        } else if let Some(right) = default_value {
            Box::new(IrNode::AssignPat {
                span: name_span,
                left: Box::new(binding),
                right,
            })
        } else {
            Box::new(binding)
        };

        Ok(IrNode::Param {
            span: name_span,
            decorators: vec![],
            pat,
        })
    }

    pub(in super::super) fn parse_type_until(&mut self, terminators: &[SyntaxKind]) -> ParseResult<Option<IrNode>> {
        let mut parts = Vec::new();
        let mut depth = 0; // Track nested brackets/parens

        while !self.at_eof() {
            let kind = match self.current_kind() {
                Some(k) => k,
                None => break,
            };
            let text = self.current().map(|t| t.text.as_str()).unwrap_or("");

            // Only terminate at depth 0
            if depth == 0 && terminators.contains(&kind) {
                break;
            }

            match kind {
                SyntaxKind::Lt | SyntaxKind::LParen | SyntaxKind::LBrace | SyntaxKind::LBracket => {
                    depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::raw(&t));
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
                        parts.push(IrNode::raw(&t));
                    }
                }
                SyntaxKind::At => {
                    let placeholder = self.parse_interpolation()
                        .map_err(|e| e.with_context("parsing interpolation in type"))?;
                    // Check if there's an identifier suffix immediately after
                    if let Some(token) = self.current() {
                        if token.kind == SyntaxKind::Ident {
                            let suffix_token = self.consume().unwrap();
                            // Force placeholder to be Ident kind for identifier concatenation
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
                                parts: vec![ident_placeholder, IrNode::raw(&suffix_token)],
                            });
                            continue;
                        }
                    }
                    parts.push(placeholder);
                }
                // Control flow blocks within types (e.g., {#for ...} inside object types)
                // Handle any {#... opening token
                k if k.is_brace_hash_open() => {
                    #[cfg(debug_assertions)]
                    if std::env::var("MF_DEBUG_PARSER").is_ok() {
                        eprintln!(
                            "[MF_DEBUG_PARSER] parse_type_until: found {:?}, calling parse_type_control_block",
                            k
                        );
                    }
                    let control = self.parse_type_control_block(k)?;
                    #[cfg(debug_assertions)]
                    if std::env::var("MF_DEBUG_PARSER").is_ok() {
                        eprintln!(
                            "[MF_DEBUG_PARSER] parse_type_until: got control node: {:?}",
                            control
                        );
                    }
                    parts.push(control);
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
                        parts.push(IrNode::raw(&t));
                    }
                }
            }
        }

        if parts.is_empty() {
            Ok(None)
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
                Ok(Some(merged.into_iter().next().unwrap()))
            } else {
                // Wrap multiple parts in TypeAnnotation for complex types with placeholders
                Ok(Some(IrNode::TypeAnnotation {
                    span: IrSpan::empty(),
                    type_ann: Box::new(IrNode::IdentBlock { span: IrSpan::empty(), parts: merged }),
                }))
            }
        }
    }

    pub(in super::super) fn parse_type_list_until(&mut self, terminator: SyntaxKind) -> ParseResult<Vec<IrNode>> {
        let mut types = Vec::new();

        while !self.at_eof() && !self.at(terminator) {
            self.skip_whitespace();
            if let Some(ty) = self.parse_type_until(&[SyntaxKind::Comma, terminator])? {
                types.push(ty);
            }
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        Ok(types)
    }

    pub(in super::super) fn parse_ts_expr_until(
        &mut self,
        terminators: &[SyntaxKind],
    ) -> super::expr::errors::ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG_PARSER] parse_ts_expr_until: terminators={:?}",
                terminators
            );
        }

        // Use the structured Pratt parser with termination-aware context
        let result = self.parse_expression_until(terminators);

        #[cfg(debug_assertions)]
        if debug_parser {
            match &result {
                Ok(node) => {
                    eprintln!("[MF_DEBUG_PARSER] parse_ts_expr_until: success, node={:?}", node);
                }
                Err(err) => {
                    eprintln!("[MF_DEBUG_PARSER] parse_ts_expr_until: error={:?}", err);
                }
            }
        }

        result
    }
}
