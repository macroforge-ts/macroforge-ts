mod enum_decl;
mod function;
mod import_export;
mod interface;

use super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::*;

// =========================================================================
// TypeScript Declaration Parsing
// =========================================================================

impl Parser {
    pub(super) fn parse_export_decl(&mut self) -> ParseResult<IrNode> {
        let _start_byte = self.current_byte_offset();
        // Consume "export"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "export keyword"))?;
        self.skip_whitespace();

        // Check what follows
        match self.current_kind() {
            // Declaration exports
            Some(SyntaxKind::ClassKw) => self.parse_class_decl(true),
            Some(SyntaxKind::FunctionKw) => self.parse_function_decl(true, false),
            Some(SyntaxKind::InterfaceKw) => self.parse_interface_decl(true),
            Some(SyntaxKind::ConstKw) | Some(SyntaxKind::LetKw) | Some(SyntaxKind::VarKw) => {
                self.parse_var_decl(true)
            }
            Some(SyntaxKind::AsyncKw) => self.parse_async_decl(true),
            Some(SyntaxKind::EnumKw) => self.parse_enum_decl(true, false),

            // Named export, export all, export default, export type
            Some(SyntaxKind::LBrace)
            | Some(SyntaxKind::Star)
            | Some(SyntaxKind::DefaultKw)
            | Some(SyntaxKind::TypeKw) => self.parse_export_decl_full(),

            // Fallback - unexpected token after export
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected declaration after 'export'")),
        }
    }

    pub(super) fn parse_async_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        let _start_byte = self.current_byte_offset();
        // Consume "async"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "async keyword"))?;
        self.skip_whitespace();

        if self.at(SyntaxKind::FunctionKw) {
            self.parse_function_decl(exported, true)
        } else {
            // async must be followed by function
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected 'function' after 'async'"))
        }
    }

    pub(super) fn parse_class_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "class"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "class keyword"))?;
        self.skip_whitespace();

        // Parse class name (may be placeholder)
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("class declaration name"))?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse optional extends
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[
                SyntaxKind::ImplementsKw,
                SyntaxKind::LBrace,
            ]).map_err(|e| e.with_context("class extends clause"))?))
        } else {
            None
        };

        // Parse optional implements
        let implements = if self.at(SyntaxKind::ImplementsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
                .map_err(|e| e.with_context("class implements clause"))?
        } else {
            vec![]
        };

        // Parse class body - use the new parse_class_body from expr/mod.rs
        if !self.at(SyntaxKind::LBrace) {
            // Class must have a body
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected '{' for class body"));
        }

        let body = self.parse_class_body()
            .map_err(|e| e.with_context("class body"))?;

        // Take pending decorators
        let decorators = std::mem::take(&mut self.pending_decorators);

        Ok(IrNode::ClassDecl {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            exported,
            declare: false,
            abstract_: false,
            decorators,
            name: Box::new(name),
            type_params,
            extends,
            implements,
            body,
        })
    }

    // parse_class_body and parse_class_member are now in expr/mod.rs with proper error handling

    fn parse_constructor(&mut self, accessibility: Option<Accessibility>) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "constructor"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "constructor keyword"))?;
        self.skip_whitespace();

        let params = self.parse_param_list()
            .map_err(|e| e.with_context("constructor parameters"))?;
        self.skip_whitespace();

        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()
                .map_err(|e| e.with_context("constructor body"))?))
        } else {
            None
        };

        let node = IrNode::Constructor {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            accessibility,
            params,
            body,
        };

        self.wrap_with_doc(node)
    }

    pub(super) fn parse_function_decl(&mut self, exported: bool, async_: bool) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "function"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "function keyword"))?;
        self.skip_whitespace();

        // Check for generator (by checking for `*` text)
        let generator = if self.current_text() == Some("*") {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Parse function name
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("function declaration name"))?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse params
        let params = self.parse_param_list()
            .map_err(|e| e.with_context("function parameters"))?;
        self.skip_whitespace();

        // Parse return type
        let return_type = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[
                SyntaxKind::LBrace,
                SyntaxKind::Semicolon,
            ]).map_err(|e| e.with_context("function return type"))?
                .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                    .with_context("function return type"))?))
        } else {
            None
        };

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()
                .map_err(|e| e.with_context("function body"))?))
        } else {
            None
        };

        let node = IrNode::FnDecl {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            exported,
            declare: false,
            async_,
            generator,
            name: Box::new(name),
            type_params,
            params,
            return_type,
            body,
        };

        self.wrap_with_doc(node)
    }

    pub(super) fn parse_var_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let kind = match self.current_kind() {
            Some(SyntaxKind::ConstKw) => VarKind::Const,
            Some(SyntaxKind::LetKw) => VarKind::Let,
            Some(SyntaxKind::VarKw) => VarKind::Var,
            _ => return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("variable declaration")
                .with_expected(&["const", "let", "var"])),
        };
        self.consume();
        self.skip_whitespace();

        let mut decls = Vec::new();

        loop {
            let decl_start = self.current_byte_offset();

            // Check for destructuring pattern (object or array)
            let name_with_type = if self.at(SyntaxKind::LBrace) || self.at(SyntaxKind::LBracket) {
                // Parse destructuring pattern using existing pattern parser
                self.parse_destructuring_pattern()?
            } else {
                // Push Identifier context for variable name
                self.push_context(Context::identifier([
                    SyntaxKind::Colon,
                    SyntaxKind::Eq,
                    SyntaxKind::Semicolon,
                    SyntaxKind::LParen,
                ]));
                let name = self.parse_ts_ident_or_placeholder()
                    .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                        .with_context("variable declarator name"))?;
                self.pop_context();
                self.skip_whitespace();

                // If there's a type annotation, wrap the name in a BindingIdent
                if self.at(SyntaxKind::Colon) {
                    let binding_span = name.span();
                    self.consume();
                    self.skip_whitespace();
                    let type_ann = self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?
                        .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                            .with_context("variable type annotation"))?;
                    IrNode::BindingIdent {
                        span: IrSpan::new(binding_span.start, self.current_byte_offset()),
                        name: Box::new(name),
                        type_ann: Some(Box::new(type_ann)),
                        optional: false,
                    }
                } else {
                    name
                }
            };

            let init = if self.at(SyntaxKind::Eq) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_ts_expr_until(&[
                    SyntaxKind::Comma,
                    SyntaxKind::Semicolon,
                ]).map_err(|e| e.with_context("variable initializer"))?))
            } else {
                None
            };

            decls.push(VarDeclarator {
                span: IrSpan::new(decl_start, self.current_byte_offset()),
                name: Box::new(name_with_type),
                type_ann: None, // Type annotation is now in the BindingIdent
                init,
                definite: false,
            });

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
                self.skip_whitespace();
            } else {
                break;
            }
        }

        // Consume semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Ok(IrNode::VarDecl {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            exported,
            declare: false,
            kind,
            decls,
        })
    }

    /// Parse a destructuring pattern for variable declarations.
    /// Handles both array patterns `[a, b]` and object patterns `{ a, b }`.
    fn parse_destructuring_pattern(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();

        let pattern = if self.at(SyntaxKind::LBracket) {
            self.parse_var_decl_array_pattern()?
        } else if self.at(SyntaxKind::LBrace) {
            self.parse_var_decl_object_pattern()?
        } else {
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("expected destructuring pattern"));
        };

        self.skip_whitespace();

        // Check for optional type annotation after the pattern
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(
                self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?
                    .ok_or_else(|| {
                        ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                            .with_context("destructuring pattern type annotation")
                    })?,
            ))
        } else {
            None
        };

        // Update pattern with type annotation if present
        match pattern {
            IrNode::ArrayPat { elems, optional, .. } => Ok(IrNode::ArrayPat {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                elems,
                type_ann,
                optional,
            }),
            IrNode::ObjectPat { props, optional, .. } => Ok(IrNode::ObjectPat {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                props,
                type_ann,
                optional,
            }),
            _ => Ok(pattern),
        }
    }

    /// Parse array destructuring pattern for variable declarations: [a, b, ...rest]
    fn parse_var_decl_array_pattern(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let start_pos = self.pos;

        self.expect(SyntaxKind::LBracket);
        let mut elems = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            // Handle holes: [,, x]
            if self.at(SyntaxKind::Comma) {
                elems.push(None);
                self.consume();
                continue;
            }

            // Handle rest: [...x]
            if self.at(SyntaxKind::DotDotDot) {
                let rest_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let arg = self.parse_var_decl_pattern_element()?;
                elems.push(Some(IrNode::RestPat {
                    span: IrSpan::new(rest_start, self.current_byte_offset()),
                    arg: Box::new(arg),
                    type_ann: None,
                }));
            } else {
                let elem = self.parse_var_decl_pattern_element()?;
                elems.push(Some(elem));
            }

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::RBracket) {
            return Err(ParseError::missing_closing(
                ParseErrorKind::MissingClosingBracket,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::ArrayPat {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            elems,
            type_ann: None,
            optional: false,
        })
    }

    /// Parse object destructuring pattern for variable declarations: { a, b: c, ...rest }
    fn parse_var_decl_object_pattern(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let start_pos = self.pos;

        self.expect(SyntaxKind::LBrace);
        let mut props = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Handle rest: {...x}
            if self.at(SyntaxKind::DotDotDot) {
                let rest_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let arg = self.parse_var_decl_pattern_element()?;
                props.push(IrNode::RestPat {
                    span: IrSpan::new(rest_start, self.current_byte_offset()),
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                let prop = self.parse_var_decl_object_prop()?;
                props.push(prop);
            }

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::RBrace) {
            return Err(ParseError::missing_closing(
                ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::ObjectPat {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            props,
            type_ann: None,
            optional: false,
        })
    }

    /// Parse a single element in a destructuring pattern (identifier, nested pattern, or placeholder)
    fn parse_var_decl_pattern_element(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        // Check for nested patterns
        if self.at(SyntaxKind::LBracket) {
            return self.parse_var_decl_array_pattern();
        }
        if self.at(SyntaxKind::LBrace) {
            return self.parse_var_decl_object_pattern();
        }

        // Check for placeholder
        if self.at(SyntaxKind::At) {
            let placeholder = self.parse_interpolation()
                .map_err(|e| e.with_context("pattern element placeholder"))?;
            self.skip_whitespace();

            // Check for default value
            if self.at(SyntaxKind::Eq) {
                let span = placeholder.span();
                self.consume();
                self.skip_whitespace();
                let default_val = self.parse_ts_expr_until(&[
                    SyntaxKind::Comma,
                    SyntaxKind::RBracket,
                    SyntaxKind::RBrace,
                ])?;
                return Ok(IrNode::AssignPat {
                    span: IrSpan::new(span.start, self.current_byte_offset()),
                    left: Box::new(placeholder),
                    right: Box::new(default_val),
                });
            }

            return Ok(placeholder);
        }

        // Parse identifier
        let ident = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("pattern element"))?;
        self.skip_whitespace();

        // Check for default value
        if self.at(SyntaxKind::Eq) {
            let span = ident.span();
            self.consume();
            self.skip_whitespace();
            let default_val = self.parse_ts_expr_until(&[
                SyntaxKind::Comma,
                SyntaxKind::RBracket,
                SyntaxKind::RBrace,
            ])?;
            return Ok(IrNode::AssignPat {
                span: IrSpan::new(span.start, self.current_byte_offset()),
                left: Box::new(ident),
                right: Box::new(default_val),
            });
        }

        Ok(ident)
    }

    /// Parse a property in an object destructuring pattern: a, a: b, a = default, a: b = default
    fn parse_var_decl_object_prop(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();

        // Parse the key (could be identifier or placeholder)
        let key = if self.at(SyntaxKind::At) {
            self.parse_interpolation()
                .map_err(|e| e.with_context("object pattern key placeholder"))?
        } else {
            self.parse_ts_ident_or_placeholder()
                .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                    .with_context("object pattern property key"))?
        };
        self.skip_whitespace();

        // Check for : to indicate renamed property
        if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();

            // Parse the value (could be nested pattern, identifier, or placeholder)
            let value = self.parse_var_decl_pattern_element()?;

            Ok(IrNode::ObjectPatProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key),
                value: Some(Box::new(value)),
            })
        } else if self.at(SyntaxKind::Eq) {
            // Shorthand with default value: { a = default }
            self.consume();
            self.skip_whitespace();
            let default_val = self.parse_ts_expr_until(&[
                SyntaxKind::Comma,
                SyntaxKind::RBrace,
            ])?;

            Ok(IrNode::ObjectPatProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key.clone()),
                value: Some(Box::new(IrNode::AssignPat {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    left: Box::new(key),
                    right: Box::new(default_val),
                })),
            })
        } else {
            // Shorthand: { a }
            Ok(IrNode::ObjectPatProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key),
                value: None,
            })
        }
    }

    pub(super) fn parse_type_alias_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        // Consume "type"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "type keyword"))?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("type alias name"))?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        if !self.at(SyntaxKind::Eq) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected '=' in type alias"));
        }
        self.consume();
        self.skip_whitespace();

        let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon])
            .map_err(|e| e.with_context("type alias definition"))?
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                .with_context("type alias definition"))?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let node = IrNode::TypeAliasDecl {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            exported,
            declare: false,
            name: Box::new(name),
            type_params,
            type_ann: Box::new(type_ann),
        };

        self.wrap_with_doc(node)
    }
}
