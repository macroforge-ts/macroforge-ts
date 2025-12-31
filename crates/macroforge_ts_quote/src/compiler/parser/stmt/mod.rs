mod loop_stmt;
mod ts_try_stmt;

use super::*;

impl Parser {
    pub(super) fn parse_stmt(&mut self) -> Option<IrNode> {
        match self.current_kind()? {
            SyntaxKind::ReturnKw => self.parse_return_stmt(),
            SyntaxKind::ThrowKw => self.parse_throw_stmt(),
            SyntaxKind::IfKw => self.parse_ts_if_stmt(),
            SyntaxKind::ForKw | SyntaxKind::WhileKw => self.parse_ts_loop_stmt(),
            SyntaxKind::TryKw => self.parse_ts_try_stmt(),
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => self.parse_var_decl(false),
            SyntaxKind::At => self.parse_interpolation(), // Statement placeholder
            _ => {
                // Expression statement - collect until semicolon or special tokens
                let expr = self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?;

                if self.at(SyntaxKind::Semicolon) {
                    self.consume();
                }

                Some(IrNode::ExprStmt {
                    expr: Box::new(expr),
                })
            }
        }
    }

    pub(super) fn parse_return_stmt(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        self.consume()?; // return
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
            return Some(IrNode::ReturnStmt { arg: None });
        }

        let arg = self.parse_ts_expr_until(&[SyntaxKind::Semicolon]);

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG_PARSER] parse_return_stmt: arg = {:?}", arg);
        }

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ReturnStmt {
            arg: arg.map(Box::new),
        })
    }

    pub(super) fn parse_throw_stmt(&mut self) -> Option<IrNode> {
        self.consume()?; // throw
        self.skip_whitespace();

        let arg = self.parse_ts_expr_until(&[SyntaxKind::Semicolon])?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ThrowStmt { arg: Box::new(arg) })
    }

    pub(super) fn parse_ts_if_stmt(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        self.consume()?; // if
        self.skip_whitespace();

        // Parse condition in parens
        self.expect(SyntaxKind::LParen);
        let test = self.parse_ts_expr_until(&[SyntaxKind::RParen])?;

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG_PARSER] parse_ts_if_stmt: test = {:?}", test);
        }

        self.expect(SyntaxKind::RParen);
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
            self.parse_block_stmt()?
        } else {
            // Single statement
            self.parse_stmt()?
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
                Some(Box::new(self.parse_block_stmt()?))
            } else if self.at(SyntaxKind::IfKw) {
                Some(Box::new(self.parse_ts_if_stmt()?))
            } else {
                Some(Box::new(self.parse_stmt()?))
            }
        } else {
            None
        };

        Some(IrNode::TsIfStmt {
            test: Box::new(test),
            cons: Box::new(cons),
            alt,
        })
    }

    pub(super) fn parse_block_stmt(&mut self) -> Option<IrNode> {
        // Consume {
        self.expect(SyntaxKind::LBrace)?;
        self.skip_whitespace();

        let stmts = self.parse_stmt_list();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::BlockStmt { stmts })
    }

    fn parse_stmt_list(&mut self) -> Vec<IrNode> {
        let mut stmts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
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
