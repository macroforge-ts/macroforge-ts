use super::super::expr::errors::{ParseError, ParseResult};
use super::super::*;

impl Parser {
    pub(in super::super) fn parse_ts_ident_or_placeholder(&mut self) -> Option<IrNode> {
        if self.at(SyntaxKind::At) {
            // It's a placeholder - parse it
            let first_placeholder = self.parse_interpolation().ok()?;
            let mut parts = vec![first_placeholder];

            // Check for more placeholders or identifier suffix (e.g., @{prefix}@{suffix}Name)
            loop {
                if self.at(SyntaxKind::At) {
                    // Another placeholder follows
                    let placeholder = self.parse_interpolation().ok()?;
                    parts.push(placeholder);
                } else if let Some(token) = self.current() {
                    if token.kind == SyntaxKind::Ident {
                        // Identifier suffix follows
                        let suffix_token = self.consume().unwrap();
                        parts.push(IrNode::ident(&suffix_token));
                        break;
                    } else {
                        // End of identifier parts
                        break;
                    }
                } else {
                    break;
                }
            }

            // Return single placeholder or IdentBlock for multiple parts
            if parts.len() == 1 {
                Some(parts.into_iter().next().unwrap())
            } else {
                Some(IrNode::IdentBlock {
                    span: IrSpan::empty(),
                    parts,
                })
            }
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

    /// Parses a type annotation until one of the terminator tokens is reached.
    /// Uses the structured parse_type function instead of collecting raw tokens.
    /// Also handles control flow blocks ({#for}, {#if}, etc.) that may appear in type contexts.
    pub(in super::super) fn parse_type_until(&mut self, terminators: &[SyntaxKind]) -> ParseResult<Option<IrNode>> {
        self.skip_whitespace();

        // Check if we're already at a terminator or EOF
        if self.at_eof() {
            return Ok(None);
        }

        if let Some(k) = self.current_kind() {
            if terminators.contains(&k) {
                return Ok(None);
            }

            // Handle control flow blocks in type contexts
            if k.is_brace_hash_open() {
                let control = self.parse_type_control_block(k)?;
                return Ok(Some(control));
            }
        }

        // Use the structured parse_type function which returns proper IR nodes
        let ty = self.parse_type()?;
        Ok(Some(ty))
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
