use super::super::*;
use super::ParseResult;
use crate::compiler::ir::TsCatchClause;

impl Parser {
    /// Parse a TypeScript try-catch-finally statement with structured IR.
    ///
    /// Syntax:
    /// - `try { } catch (e) { }`
    /// - `try { } catch { }` (parameter-less catch)
    /// - `try { } finally { }`
    /// - `try { } catch (e) { } finally { }`
    /// - `try { } catch (e: Error) { }` (typed catch parameter)
    pub(in super::super) fn parse_ts_try_stmt(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();

        // Consume "try"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "try keyword"))?;
        self.skip_whitespace();

        // Parse the try block
        if !self.at(SyntaxKind::LBrace) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_context("expected '{' after 'try'"));
        }
        let block = Box::new(
            self.parse_block_stmt()
                .map_err(|e| e.with_context("try block"))?,
        );
        self.skip_whitespace();

        // Parse optional catch clause
        let handler = if self.at(SyntaxKind::CatchKw) {
            Some(self.parse_catch_clause()?)
        } else {
            None
        };
        self.skip_whitespace();

        // Parse optional finally clause
        let finalizer = if self.at(SyntaxKind::FinallyKw) {
            self.consume(); // consume "finally"
            self.skip_whitespace();

            if !self.at(SyntaxKind::LBrace) {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    self.current_byte_offset(),
                )
                .with_context("expected '{' after 'finally'"));
            }
            Some(Box::new(
                self.parse_block_stmt()
                    .map_err(|e| e.with_context("finally block"))?,
            ))
        } else {
            None
        };

        // Must have either catch or finally
        if handler.is_none() && finalizer.is_none() {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_context("try statement requires catch or finally clause"));
        }

        Ok(IrNode::TsTryStmt {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            block,
            handler,
            finalizer,
        })
    }

    /// Parse a catch clause: `catch (e) { }` or `catch { }`
    fn parse_catch_clause(&mut self) -> ParseResult<TsCatchClause> {
        let start_byte = self.current_byte_offset();

        // Consume "catch"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "catch keyword"))?;
        self.skip_whitespace();

        // Parse optional catch parameter
        let param = if self.at(SyntaxKind::LParen) {
            self.consume(); // consume '('
            self.skip_whitespace();

            // Parse the catch parameter (identifier with optional type annotation)
            let param_start = self.current_byte_offset();

            // Parse the parameter name (could be a placeholder or identifier)
            let name = self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                    .with_context("catch parameter name")
            })?;
            self.skip_whitespace();

            // Check for optional type annotation
            let param_node = if self.at(SyntaxKind::Colon) {
                self.consume(); // consume ':'
                self.skip_whitespace();

                let type_ann = self
                    .parse_type_until(&[SyntaxKind::RParen])?
                    .ok_or_else(|| {
                        ParseError::new(
                            ParseErrorKind::ExpectedTypeAnnotation,
                            self.current_byte_offset(),
                        )
                        .with_context("catch parameter type")
                    })?;

                IrNode::BindingIdent {
                    span: IrSpan::new(param_start, self.current_byte_offset()),
                    name: Box::new(name),
                    type_ann: Some(Box::new(type_ann)),
                    optional: false,
                }
            } else {
                name
            };

            self.skip_whitespace();

            // Expect ')'
            if !self.at(SyntaxKind::RParen) {
                return Err(ParseError::new(
                    ParseErrorKind::MissingClosingParen,
                    self.current_byte_offset(),
                )
                .with_context("catch parameter"));
            }
            self.consume(); // consume ')'
            self.skip_whitespace();

            Some(Box::new(param_node))
        } else {
            None
        };

        // Parse the catch body
        if !self.at(SyntaxKind::LBrace) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_context("expected '{' after catch"));
        }
        let body = Box::new(
            self.parse_block_stmt()
                .map_err(|e| e.with_context("catch body"))?,
        );

        Ok(TsCatchClause {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            param,
            body,
        })
    }
}
