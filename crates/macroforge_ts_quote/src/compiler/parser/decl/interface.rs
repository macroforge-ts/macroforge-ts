use super::super::*;
use super::ParseResult;

impl Parser {
    pub(in super::super) fn parse_interface_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        // Consume "interface"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "interface keyword"))?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("interface name"))?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse extends
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
                .map_err(|e| e.with_context("interface extends clause"))?
        } else {
            vec![]
        };

        // Parse body
        if !self.at(SyntaxKind::LBrace) {
            return Ok(IrNode::Raw("interface ".to_string()));
        }
        self.consume();
        self.skip_whitespace();

        let body = self.parse_interface_body()?;

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        let node = IrNode::InterfaceDecl {
            exported,
            declare: false,
            name: Box::new(name),
            type_params,
            extends,
            body,
        };

        self.wrap_with_doc(node)
    }

    fn parse_interface_body(&mut self) -> ParseResult<Vec<IrNode>> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if let Some(kind) = self.current_kind() {
                match kind {
                    SyntaxKind::BraceHashIf
                    | SyntaxKind::BraceHashFor
                    | SyntaxKind::BraceHashWhile
                    | SyntaxKind::BraceHashMatch => {
                        members.push(self.parse_control_block(kind)?);
                        continue;
                    }
                    _ => {}
                }
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            if let Some(member) = self.parse_interface_member()? {
                members.push(member);
            } else {
                self.advance();
            }
        }

        Ok(members)
    }

    /// Try to parse an interface member (property/method signature) when we see `readonly`.
    /// Falls back to Raw text if it doesn't look like an interface member pattern.
    pub(in super::super) fn parse_maybe_interface_member(&mut self) -> ParseResult<Option<IrNode>> {
        // We're at `readonly` - consume it
        let Some(readonly_token) = self.consume() else {
            return Ok(None);
        };
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_PARSER").is_ok() {
            eprintln!(
                "[MF_DEBUG_PARSER] parse_maybe_interface_member: after readonly, current = {:?}, context_stack = {:?}",
                self.current(),
                self.context_stack
            );
        }

        // Check if next is ident/placeholder (looks like interface member)
        if self.at(SyntaxKind::At)
            || self.at(SyntaxKind::Ident)
            || self.current_kind().map_or(false, |k| k.is_ts_keyword())
        {
            // Looks like interface member - parse the name
            let name = match self.parse_ts_ident_or_placeholder() {
                Some(n) => n,
                None => return Ok(Some(IrNode::Raw(readonly_token.text))),
            };
            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_PARSER").is_ok() {
                eprintln!(
                    "[MF_DEBUG_PARSER] parse_maybe_interface_member: parsed name = {:?}",
                    name
                );
            }
            self.skip_whitespace();

            let optional = if self.at(SyntaxKind::Question) {
                self.consume();
                self.skip_whitespace();
                true
            } else {
                false
            };

            // Need colon for type annotation
            if !self.at(SyntaxKind::Colon) {
                // Not a valid member pattern - return what we consumed as raw
                return Ok(Some(IrNode::Raw(format!("{} ", readonly_token.text))));
            }

            self.consume(); // colon
            self.skip_whitespace();

            let type_ann = self.parse_type_until(&[
                SyntaxKind::Semicolon,
                SyntaxKind::Comma,
                SyntaxKind::RBrace,
                SyntaxKind::BraceSlashIfBrace,
                SyntaxKind::BraceSlashForBrace,
                SyntaxKind::BraceSlashWhileBrace,
                SyntaxKind::BraceSlashMatchBrace,
            ])?;

            // Consume optional separator
            if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
                self.consume();
            }

            Ok(Some(IrNode::PropSignature {
                readonly: true,
                name: Box::new(name),
                optional,
                type_ann: type_ann.map(Box::new),
            }))
        } else {
            // Doesn't look like interface member - return readonly as raw text
            Ok(Some(IrNode::Raw(readonly_token.text)))
        }
    }

    fn parse_interface_member(&mut self) -> ParseResult<Option<IrNode>> {
        self.skip_whitespace();

        let readonly = if self.at(SyntaxKind::ReadonlyKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check for index signature: [key: Type]: Type
        if self.at(SyntaxKind::LBracket) {
            return self.parse_index_signature(readonly);
        }

        let name = match self.parse_ts_ident_or_placeholder() {
            Some(n) => n,
            None => return Ok(None),
        };
        self.skip_whitespace();

        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check if method signature or property
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            let type_params = self.parse_optional_type_params();
            let params = self.parse_param_list()
                .map_err(|e| e.with_context("interface method signature"))?;
            self.skip_whitespace();

            let return_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                self.parse_type_until(&[
                    SyntaxKind::Semicolon,
                    SyntaxKind::Comma,
                    SyntaxKind::RBrace,
                ])?.map(Box::new)
            } else {
                None
            };

            // Consume optional separator
            if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
                self.consume();
            }

            return Ok(Some(IrNode::MethodSignature {
                name: Box::new(name),
                optional,
                type_params,
                params,
                return_type,
            }));
        }

        // Property signature
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_until(&[
                SyntaxKind::Semicolon,
                SyntaxKind::Comma,
                SyntaxKind::RBrace,
            ])?.map(Box::new)
        } else {
            None
        };

        // Consume optional separator
        if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
            self.consume();
        }

        Ok(Some(IrNode::PropSignature {
            readonly,
            name: Box::new(name),
            optional,
            type_ann,
        }))
    }

    /// Parse an index signature: [key: Type]: Type
    fn parse_index_signature(&mut self, readonly: bool) -> ParseResult<Option<IrNode>> {
        let Some(_) = self.consume() else {
            return Ok(None);
        }; // [
        self.skip_whitespace();

        // Parse parameter(s) - typically just one: key: Type
        let mut params = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBracket) {
            let Some(param_name) = self.parse_ts_ident_or_placeholder() else {
                return Ok(None);
            };
            self.skip_whitespace();

            // Expect colon and type
            if !self.at(SyntaxKind::Colon) {
                break;
            }
            self.consume(); // :
            self.skip_whitespace();

            let Some(param_type) = self.parse_type_until(&[SyntaxKind::RBracket, SyntaxKind::Comma])? else {
                return Ok(None);
            };

            // Create a Param node with the binding
            params.push(IrNode::Param {
                decorators: vec![],
                pat: Box::new(IrNode::BindingIdent {
                    name: Box::new(param_name),
                    type_ann: Some(Box::new(param_type)),
                    optional: false,
                }),
            });

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
                self.skip_whitespace();
            }
        }

        self.expect(SyntaxKind::RBracket);
        self.skip_whitespace();

        // Expect colon and return type
        if !self.at(SyntaxKind::Colon) {
            return Ok(None);
        }
        self.consume(); // :
        self.skip_whitespace();

        let Some(type_ann) = self.parse_type_until(&[
            SyntaxKind::Semicolon,
            SyntaxKind::Comma,
            SyntaxKind::RBrace,
        ])? else {
            return Ok(None);
        };

        // Consume optional separator
        if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
            self.consume();
        }

        Ok(Some(IrNode::IndexSignature {
            readonly,
            params,
            type_ann: Box::new(type_ann),
        }))
    }
}
