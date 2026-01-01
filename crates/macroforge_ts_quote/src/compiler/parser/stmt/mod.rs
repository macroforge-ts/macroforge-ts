mod loop_stmt;
mod ts_try_stmt;

use super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::*;

impl Parser {
    pub(super) fn parse_stmt(&mut self) -> Option<IrNode> {
        match self.current_kind()? {
            SyntaxKind::ReturnKw => self.parse_return_stmt().ok(),
            SyntaxKind::ThrowKw => self.parse_throw_stmt().ok(),
            SyntaxKind::IfKw => self.parse_ts_if_stmt().ok(),
            SyntaxKind::ForKw | SyntaxKind::WhileKw => self.parse_ts_loop_stmt().ok(),
            SyntaxKind::TryKw => self.parse_ts_try_stmt(),
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => self.parse_var_decl(false).ok(),
            SyntaxKind::At => self.parse_interpolation().ok(), // Statement placeholder
            _ => {
                // Expression statement - collect until semicolon or special tokens
                let expr = self.parse_ts_expr_until(&[SyntaxKind::Semicolon]).ok()?;

                if self.at(SyntaxKind::Semicolon) {
                    self.consume();
                }

                Some(IrNode::ExprStmt {
                    expr: Box::new(expr),
                })
            }
        }
    }

    pub(super) fn parse_return_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.pos, "return keyword"))?; // return
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG_PARSER] parse_return_stmt: current token = {:?}",
                self.current()
            );
        }

        if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::RBrace) {
            #[cfg(debug_assertions)]
            if debug_parser {
                eprintln!(
                    "[MF_DEBUG_PARSER] parse_return_stmt: at semicolon or rbrace, returning None arg"
                );
            }
            if self.at(SyntaxKind::Semicolon) {
                self.consume();
            }
            return Ok(IrNode::ReturnStmt { arg: None });
        }

        let arg = self
            .parse_ts_expr_until(&[SyntaxKind::Semicolon])
            .map_err(|e| e.with_context("parsing return statement"))?;

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG_PARSER] parse_return_stmt: arg = {:?}", arg);
        }

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Ok(IrNode::ReturnStmt {
            arg: Some(Box::new(arg)),
        })
    }

    pub(super) fn parse_throw_stmt(&mut self) -> ParseResult<IrNode> {
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.pos, "throw keyword"))?; // throw
        self.skip_whitespace();

        let arg = self
            .parse_ts_expr_until(&[SyntaxKind::Semicolon])
            .map_err(|e| e.with_context("parsing throw statement"))?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Ok(IrNode::ThrowStmt { arg: Box::new(arg) })
    }

    pub(super) fn parse_ts_if_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.pos, "if keyword"))?; // if
        self.skip_whitespace();

        // Parse condition in parens
        self.expect(SyntaxKind::LParen)
            .ok_or_else(|| {
                ParseError::new(ParseErrorKind::UnexpectedToken, self.pos)
                    .with_context("parsing if statement condition")
                    .with_expected(&["("])
            })?;
        let test = self
            .parse_ts_expr_until(&[SyntaxKind::RParen])
            .map_err(|e| e.with_context("parsing if statement condition"))?;

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG_PARSER] parse_ts_if_stmt: test = {:?}", test);
        }

        self.expect(SyntaxKind::RParen)
            .ok_or_else(|| {
                ParseError::new(ParseErrorKind::MissingClosingParen, self.pos)
                    .with_context("parsing if statement condition")
            })?;
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG_PARSER] parse_ts_if_stmt: after ), current = {:?}",
                self.current()
            );
        }

        // Parse consequent
        let cons = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing if statement consequent block"))?
        } else {
            // Single statement
            self.parse_stmt()
                .ok_or_else(|| {
                    ParseError::unexpected_eof(self.pos, "if statement consequent")
                })?
        };

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG_PARSER] parse_ts_if_stmt: cons = {:?}", cons);
        }

        self.skip_whitespace();

        // Parse optional else
        let alt = if self.at(SyntaxKind::ElseKw) {
            self.consume();
            self.skip_whitespace();
            if self.at(SyntaxKind::LBrace) {
                Some(Box::new(
                    self.parse_block_stmt()
                        .map_err(|e| e.with_context("parsing else block"))?,
                ))
            } else if self.at(SyntaxKind::IfKw) {
                Some(Box::new(
                    self.parse_ts_if_stmt()
                        .map_err(|e| e.with_context("parsing else-if statement"))?,
                ))
            } else {
                Some(Box::new(
                    self.parse_stmt()
                        .ok_or_else(|| {
                            ParseError::unexpected_eof(self.pos, "else statement body")
                        })?,
                ))
            }
        } else {
            None
        };

        Ok(IrNode::TsIfStmt {
            test: Box::new(test),
            cons: Box::new(cons),
            alt,
        })
    }

    // parse_block_stmt is now in expr/mod.rs with proper error handling (ParseResult)

    fn parse_stmt_list(&mut self) -> Vec<IrNode> {
        let mut stmts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow - any {#... opening token
            if self.at_brace_hash_open() {
                let kind = self.current_kind().unwrap();
                if let Some(node) = self.parse_control_block(kind) {
                    stmts.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    stmts.push(node);
                }
                continue;
            }

            // Parse statement
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Unknown - consume one token as raw
                if let Some(token) = self.consume() {
                    stmts.push(IrNode::Raw(token.text));
                }
            }
        }

        Self::merge_adjacent_text(stmts)
    }
}
