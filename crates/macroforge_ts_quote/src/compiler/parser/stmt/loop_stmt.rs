use super::super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::super::*;

impl Parser {
    /// Parse a TypeScript for/while/do-while loop.
    /// All loop types are now parsed structurally:
    /// - for-in/for-of → ForInStmt/ForOfStmt
    /// - C-style for → TsForStmt
    /// - while → TsWhileStmt
    /// - do-while → TsDoWhileStmt
    pub(in super::super) fn parse_ts_loop_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.current()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "loop statement"))?
            .text.clone();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_loop_stmt: keyword={:?}", keyword);
        }

        match keyword.as_str() {
            "for" => {
                // Try to parse as for-in or for-of first
                if let Some(structured) = self.try_parse_for_in_of()? {
                    return Ok(structured);
                }
                // Parse as C-style for loop
                self.parse_ts_for_stmt()
            }
            "while" => self.parse_ts_while_stmt(),
            "do" => self.parse_ts_do_while_stmt(),
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected 'for', 'while', or 'do'")),
        }
    }

    /// Parse a C-style for statement: `for (init; test; update) body`
    fn parse_ts_for_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let start_byte = self.current_byte_offset();
        self.consume(); // for
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_for_stmt: parsing C-style for loop");
        }

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("expected '(' after 'for'")
        })?;
        self.skip_whitespace();

        // Parse init (optional: VarDecl or expression)
        let init = if self.at(SyntaxKind::Semicolon) {
            None
        } else if self.at(SyntaxKind::ConstKw) || self.at(SyntaxKind::LetKw) || self.at(SyntaxKind::VarKw) {
            Some(Box::new(self.parse_for_init_var_decl()?))
        } else {
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?))
        };
        self.skip_whitespace();
        self.expect(SyntaxKind::Semicolon).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("expected ';' after for loop init")
        })?;
        self.skip_whitespace();

        // Parse test (optional expression)
        let test = if self.at(SyntaxKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?))
        };
        self.skip_whitespace();
        self.expect(SyntaxKind::Semicolon).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("expected ';' after for loop test")
        })?;
        self.skip_whitespace();

        // Parse update (optional expression)
        let update = if self.at(SyntaxKind::RParen) {
            None
        } else {
            Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::RParen])?))
        };
        self.skip_whitespace();
        self.expect(SyntaxKind::RParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("closing for loop header")
        })?;
        self.skip_whitespace();

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        } else {
            self.parse_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        };

        let end_byte = self.current_byte_offset();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_for_stmt: completed, init={}, test={}, update={}",
                init.is_some(), test.is_some(), update.is_some());
        }

        Ok(IrNode::TsForStmt {
            span: IrSpan::new(start_byte, end_byte),
            init,
            test,
            update,
            body: Box::new(body),
        })
    }

    /// Parse a while statement: `while (test) body`
    fn parse_ts_while_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let start_byte = self.current_byte_offset();
        self.consume(); // while
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_while_stmt: parsing while loop");
        }

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("expected '(' after 'while'")
        })?;
        self.skip_whitespace();

        // Parse test expression
        let test = self.parse_ts_expr_until(&[SyntaxKind::RParen])
            .map_err(|e| e.with_context("parsing while loop condition"))?;
        self.skip_whitespace();

        self.expect(SyntaxKind::RParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("closing while loop condition")
        })?;
        self.skip_whitespace();

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing while loop body"))?
        } else {
            self.parse_stmt()
                .map_err(|e| e.with_context("parsing while loop body"))?
        };

        let end_byte = self.current_byte_offset();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_while_stmt: completed");
        }

        Ok(IrNode::TsWhileStmt {
            span: IrSpan::new(start_byte, end_byte),
            test: Box::new(test),
            body: Box::new(body),
        })
    }

    /// Parse a do-while statement: `do body while (test)`
    fn parse_ts_do_while_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let start_byte = self.current_byte_offset();
        self.consume(); // do
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_do_while_stmt: parsing do-while loop");
        }

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing do-while loop body"))?
        } else {
            self.parse_stmt()
                .map_err(|e| e.with_context("parsing do-while loop body"))?
        };
        self.skip_whitespace();

        // Expect 'while' keyword
        if !self.at(SyntaxKind::WhileKw) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected 'while' after do-while body"));
        }
        self.consume(); // while
        self.skip_whitespace();

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("expected '(' after 'while' in do-while")
        })?;
        self.skip_whitespace();

        // Parse test expression
        let test = self.parse_ts_expr_until(&[SyntaxKind::RParen])
            .map_err(|e| e.with_context("parsing do-while loop condition"))?;
        self.skip_whitespace();

        self.expect(SyntaxKind::RParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingParen, self.current_byte_offset())
                .with_context("closing do-while loop condition")
        })?;
        self.skip_whitespace();

        // Optional semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let end_byte = self.current_byte_offset();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_do_while_stmt: completed");
        }

        Ok(IrNode::TsDoWhileStmt {
            span: IrSpan::new(start_byte, end_byte),
            body: Box::new(body),
            test: Box::new(test),
        })
    }

    /// Parse variable declaration in for loop init: `const i = 0` or `let i = 0`
    fn parse_for_init_var_decl(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let kind = match self.current_kind() {
            Some(SyntaxKind::ConstKw) => VarKind::Const,
            Some(SyntaxKind::LetKw) => VarKind::Let,
            Some(SyntaxKind::VarKw) => VarKind::Var,
            _ => return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            ).with_context("expected 'const', 'let', or 'var'")),
        };
        self.consume(); // const/let/var
        self.skip_whitespace();

        // Parse declarators (could be multiple: let i = 0, j = 1)
        let mut decls = Vec::new();
        loop {
            let decl_start = self.current_byte_offset();

            // Parse binding name
            let name = self.parse_for_loop_binding()
                .map_err(|e| e.with_context("parsing for loop variable binding"))?;
            self.skip_whitespace();

            // Optional type annotation
            let type_ann = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?
                    .map(Box::new)
            } else {
                None
            };
            self.skip_whitespace();

            // Optional initializer
            let init = if self.at(SyntaxKind::Eq) {
                self.consume();
                self.skip_whitespace();
                // Parse until semicolon or comma (for multiple declarators)
                Some(Box::new(self.parse_ts_expr_until(&[SyntaxKind::Semicolon, SyntaxKind::Comma])?))
            } else {
                None
            };

            let decl_end = self.current_byte_offset();
            decls.push(VarDeclarator {
                span: IrSpan::new(decl_start, decl_end),
                name: Box::new(name),
                type_ann,
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

        let end_byte = self.current_byte_offset();
        Ok(IrNode::VarDecl {
            span: IrSpan::new(start_byte, end_byte),
            exported: false,
            declare: false,
            kind,
            decls,
        })
    }

    /// Try to parse a for-in or for-of loop.
    /// Returns Ok(None) if it's a C-style for loop (with semicolons).
    fn try_parse_for_in_of(&mut self) -> ParseResult<Option<IrNode>> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        // Save position for backtracking
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.consume(); // for
        self.skip_whitespace();

        // Check for await keyword (for-await-of)
        let has_await = if self.at(SyntaxKind::AwaitKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        if !self.at(SyntaxKind::LParen) {
            // Restore position and let raw parsing handle it
            self.pos = start_pos;
            return Ok(None);
        }
        self.consume(); // (
        self.skip_whitespace();

        // Parse the left-hand side (variable declaration or expression)
        let left = match self.parse_for_loop_left() {
            Ok(node) => node,
            Err(_) => {
                self.pos = start_pos;
                return Ok(None);
            }
        };

        self.skip_whitespace();

        // Check for 'in' or 'of' keyword
        let is_for_in = self.at(SyntaxKind::InKw);
        let is_for_of = self.at(SyntaxKind::OfKw);

        if !is_for_in && !is_for_of {
            // This is a C-style for loop - restore position
            self.pos = start_pos;
            return Ok(None);
        }

        self.consume(); // in/of
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG] parse_for_in_of: is_for_in={}, is_for_of={}, has_await={}",
                is_for_in, is_for_of, has_await
            );
        }

        // Parse the right-hand side (expression)
        let right = self
            .parse_ts_expr_until(&[SyntaxKind::RParen])
            .map_err(|e| e.with_context("parsing for loop iterable"))?;

        self.skip_whitespace();
        self.expect(SyntaxKind::RParen);
        self.skip_whitespace();

        // Parse the body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        } else {
            self.parse_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        };

        let end_byte = self.current_byte_offset();
        if is_for_in {
            Ok(Some(IrNode::ForInStmt {
                span: IrSpan::new(start_byte, end_byte),
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            }))
        } else {
            Ok(Some(IrNode::ForOfStmt {
                span: IrSpan::new(start_byte, end_byte),
                await_: has_await,
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            }))
        }
    }

    /// Parse the left-hand side of a for-in/for-of loop.
    /// This can be a variable declaration (const/let/var x) or an expression.
    fn parse_for_loop_left(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let kind = self
            .current_kind()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "for loop left-hand side"))?;

        match kind {
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => {
                // Variable declaration
                let var_kind = match kind {
                    SyntaxKind::ConstKw => VarKind::Const,
                    SyntaxKind::LetKw => VarKind::Let,
                    SyntaxKind::VarKw => VarKind::Var,
                    _ => unreachable!(),
                };
                self.consume(); // const/let/var
                self.skip_whitespace();

                // Parse the binding pattern or identifier using expr/ implementation
                let name = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing for loop variable binding"))?;

                let end_byte = self.current_byte_offset();
                Ok(IrNode::VarDecl {
                    span: IrSpan::new(start_byte, end_byte),
                    exported: false,
                    declare: false,
                    kind: var_kind,
                    decls: vec![VarDeclarator {
                        span: IrSpan::new(start_byte, end_byte),
                        name: Box::new(name),
                        type_ann: None,
                        init: None,
                        definite: false,
                    }],
                })
            }
            SyntaxKind::At => {
                // Placeholder - could be expression or identifier
                self.parse_interpolation()
            }
            SyntaxKind::LBracket => {
                // Array destructuring pattern - use simple collection
                self.parse_for_loop_array_pattern()
            }
            SyntaxKind::LBrace => {
                // Object destructuring pattern - use simple collection
                self.parse_for_loop_object_pattern()
            }
            _ => {
                // Expression (for reassignment like: for (x in obj))
                self.parse_ts_expr_until(&[SyntaxKind::InKw, SyntaxKind::OfKw])
                    .map_err(|e| e.with_context("parsing for loop left-hand side expression"))
            }
        }
    }

    /// Parse a simple binding for for-loop left-hand side
    fn parse_for_loop_binding(&mut self) -> ParseResult<IrNode> {
        let kind = self
            .current_kind()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "for loop binding"))?;

        match kind {
            SyntaxKind::LBracket => self
                .parse_for_loop_array_pattern()
                .map_err(|e| e.with_context("parsing for loop array pattern")),
            SyntaxKind::LBrace => self
                .parse_for_loop_object_pattern()
                .map_err(|e| e.with_context("parsing for loop object pattern")),
            SyntaxKind::At => self.parse_interpolation(),
            _ => self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                    .with_context("parsing for loop binding identifier")
            }),
        }
    }

    /// Parse array pattern for for-loop: [a, b, ...rest]
    fn parse_for_loop_array_pattern(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.consume(); // [
        self.skip_whitespace();

        let mut elems: Vec<IrNode> = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBracket) {
            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
                continue;
            }

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            // Check for rest pattern
            if self.at(SyntaxKind::DotDotDot) {
                let rest_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let arg = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing rest element in array pattern"))?;
                let rest_end = self.current_byte_offset();
                elems.push(IrNode::RestPat {
                    span: IrSpan::new(rest_start, rest_end),
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                let elem = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing element in array pattern"))?;
                elems.push(elem);
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RBracket).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingBracket, self.current_byte_offset())
                .with_context("parsing array pattern")
        })?;

        let end_byte = self.current_byte_offset();
        Ok(IrNode::ArrayPat {
            span: IrSpan::new(start_byte, end_byte),
            elems: elems.into_iter().map(Some).collect(),
            type_ann: None,
            optional: false,
        })
    }

    /// Parse object pattern for for-loop: { a, b: c, ...rest }
    fn parse_for_loop_object_pattern(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.consume(); // {
        self.skip_whitespace();

        let mut props: Vec<IrNode> = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for rest pattern
            if self.at(SyntaxKind::DotDotDot) {
                let rest_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let arg = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing rest element in object pattern"))?;
                let rest_end = self.current_byte_offset();
                props.push(IrNode::RestPat {
                    span: IrSpan::new(rest_start, rest_end),
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                // Regular property
                let prop_start = self.current_byte_offset();
                let key = self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                    ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                        .with_context("parsing object pattern property key")
                })?;
                self.skip_whitespace();

                if self.at(SyntaxKind::Colon) {
                    // Renamed binding: { a: b }
                    self.consume();
                    self.skip_whitespace();
                    let value = self
                        .parse_for_loop_binding()
                        .map_err(|e| e.with_context("parsing object pattern property value"))?;
                    let prop_end = self.current_byte_offset();
                    props.push(IrNode::ObjectPatProp {
                        span: IrSpan::new(prop_start, prop_end),
                        key: Box::new(key),
                        value: Some(Box::new(value)),
                    });
                } else {
                    // Shorthand: { a }
                    let prop_end = self.current_byte_offset();
                    props.push(IrNode::ObjectPatProp {
                        span: IrSpan::new(prop_start, prop_end),
                        key: Box::new(key),
                        value: None,
                    });
                }
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RBrace).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("parsing object pattern")
        })?;

        let end_byte = self.current_byte_offset();
        Ok(IrNode::ObjectPat {
            span: IrSpan::new(start_byte, end_byte),
            props,
            type_ann: None,
            optional: false,
        })
    }

}
