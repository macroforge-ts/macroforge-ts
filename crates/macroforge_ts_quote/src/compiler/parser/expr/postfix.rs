//! Postfix and infix expression parsing.
//!
//! This module handles the "continuation" part of expression parsing:
//! - Postfix operators (++, --, !)
//! - Member access (.property, [computed], ?.)
//! - Function calls and type instantiation
//! - Binary operators
//! - Ternary conditional
//! - Assignment operators
//! - Sequence expressions (comma operator)
//! - TypeScript type assertions (as, satisfies)
//! - Tagged template literals

use super::errors::{ParseError, ParseErrorKind, ParseResult};
use super::operators::{keyword_to_binary_op, text_to_assign_op, text_to_binary_op, to_update_op};
use super::precedence::{infix_binding_power, is_assignment_operator, is_postfix_operator, prec};
use crate::compiler::ir::{IrNode, IrSpan};
use crate::compiler::parser::Parser;
use crate::compiler::syntax::SyntaxKind;

impl Parser {
    /// Parses postfix and infix operations on a left-hand expression.
    ///
    /// This is the main infix parsing loop of the Pratt parser. It handles:
    /// - Postfix update (x++, x--)
    /// - TypeScript non-null assertion (x!)
    /// - Member access (x.y, x[y])
    /// - Optional chaining (x?.y, x?.[y], x?.())
    /// - Function calls (x(args))
    /// - Type instantiation (x<T>)
    /// - Type assertions (x as T, x satisfies T)
    /// - Binary operators (x + y)
    /// - Ternary conditional (x ? y : z)
    /// - Assignment (x = y)
    /// - Sequence (x, y)
    /// - Tagged templates (x`...`)
    pub(super) fn parse_postfix_and_infix(
        &mut self,
        mut left: IrNode,
        min_bp: u8,
    ) -> ParseResult<IrNode> {
        loop {
            self.skip_whitespace();

            // Check if we've hit a terminator (from context stack)
            if self.at_terminator() {
                break;
            }

            let Some(token) = self.current() else {
                break;
            };

            let kind = token.kind;
            let text = token.text.clone();

            // =========================================================================
            // Postfix operators (highest precedence, no binding power comparison)
            // =========================================================================

            // Postfix update: x++, x--
            if is_postfix_operator(kind) {
                // Check if this could be a postfix operator (no newline between)
                if prec::POSTFIX >= min_bp {
                    let op = to_update_op(kind).ok_or_else(|| {
                        ParseError::new(ParseErrorKind::InvalidPostfixOperator, self.current_byte_offset())
                    })?;
                    let start = left.span().start;
                    self.consume();
                    left = IrNode::UpdateExpr {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        op,
                        prefix: false,
                        arg: Box::new(left),
                    };
                    continue;
                }
            }

            // TypeScript non-null assertion: x!
            // Must not be followed by = (which would be x !== ...)
            if kind == SyntaxKind::Exclaim && prec::TS_NON_NULL >= min_bp {
                // Peek ahead to make sure it's not !== or !=
                let next_text = self.peek_text(1);
                if next_text != Some("=") && next_text != Some("==") {
                    let start = left.span().start;
                    self.consume();
                    left = IrNode::TsNonNullExpr {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        expr: Box::new(left),
                    };
                    continue;
                }
            }

            // =========================================================================
            // Member access and calls (very high precedence)
            // =========================================================================

            // Member access: x.y
            if kind == SyntaxKind::Dot && prec::CALL.left >= min_bp {
                let start = left.span().start;
                self.consume();
                self.skip_whitespace();

                let prop = self.parse_member_property()?;
                left = IrNode::MemberExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    obj: Box::new(left),
                    prop: Box::new(prop),
                    computed: false,
                };
                continue;
            }

            // Computed member access: x[y]
            if kind == SyntaxKind::LBracket && prec::CALL.left >= min_bp {
                let start_pos = self.pos;
                let start = left.span().start;
                self.consume();
                self.skip_whitespace();

                let prop = self.parse_expression_with_precedence(0)?;

                self.skip_whitespace();
                if !self.at(SyntaxKind::RBracket) {
                    return Err(ParseError::missing_closing(
                        ParseErrorKind::MissingClosingBracket,
                        self.current_byte_offset(),
                        start_pos,
                    ));
                }
                self.consume();

                left = IrNode::MemberExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    obj: Box::new(left),
                    prop: Box::new(prop),
                    computed: true,
                };
                continue;
            }

            // Optional chaining: x?.y, x?.[y], x?.()
            if kind == SyntaxKind::Question && prec::CALL.left >= min_bp {
                if let Some(next) = self.peek_kind(1) {
                    if next == SyntaxKind::Dot || text == "?." {
                        left = self.parse_optional_chain(left)?;
                        continue;
                    }
                }
            }

            // Check for ?. as a single token
            if text == "?." && prec::CALL.left >= min_bp {
                left = self.parse_optional_chain(left)?;
                continue;
            }

            // Function call: x(args)
            if kind == SyntaxKind::LParen && prec::CALL.left >= min_bp {
                let start = left.span().start;
                let args = self.parse_call_arguments()?;
                left = IrNode::CallExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    callee: Box::new(left),
                    type_args: None,
                    args,
                };
                continue;
            }

            // Type instantiation: x<T> - this is tricky as < could be comparison
            if kind == SyntaxKind::Lt && prec::CALL.left >= min_bp {
                let start = left.span().start;
                // Try to parse as type arguments
                if let Some(type_args) = self.try_parse_type_args_for_call()? {
                    self.skip_whitespace();

                    // If followed by ( it's a call with type args
                    if self.at(SyntaxKind::LParen) {
                        let args = self.parse_call_arguments()?;
                        left = IrNode::CallExpr {
                            span: IrSpan::new(start, self.current_byte_offset()),
                            callee: Box::new(left),
                            type_args: Some(Box::new(type_args)),
                            args,
                        };
                        continue;
                    }

                    // Otherwise it's a type instantiation (x<T> without call)
                    left = IrNode::TsInstantiation {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        expr: Box::new(left),
                        type_args: Box::new(type_args),
                    };
                    continue;
                }
                // Not type args, fall through to comparison operator handling
            }

            // Tagged template literal: x`...`
            if kind == SyntaxKind::Backtick && prec::CALL.left >= min_bp {
                let start = left.span().start;
                let tpl = self.parse_template_literal()?;
                left = IrNode::TaggedTpl {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    tag: Box::new(left),
                    type_args: None,
                    tpl: Box::new(tpl),
                };
                continue;
            }

            // =========================================================================
            // TypeScript type assertions
            // =========================================================================

            // Type assertion: x as T
            if kind == SyntaxKind::AsKw && prec::TS_AS.left >= min_bp {
                let start = left.span().start;
                self.consume();
                self.skip_whitespace();

                // Check for 'as const'
                if self.at(SyntaxKind::ConstKw) {
                    self.consume();
                    left = IrNode::TsConstAssertion {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        expr: Box::new(left),
                    };
                    continue;
                }

                let type_ann = self.parse_type()?;
                left = IrNode::TsAsExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    expr: Box::new(left),
                    type_ann: Box::new(type_ann),
                };
                continue;
            }

            // Satisfies expression: x satisfies T
            if kind == SyntaxKind::SatisfiesKw && prec::TS_AS.left >= min_bp {
                let start = left.span().start;
                self.consume();
                self.skip_whitespace();

                let type_ann = self.parse_type()?;
                left = IrNode::TsSatisfiesExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    expr: Box::new(left),
                    type_ann: Box::new(type_ann),
                };
                continue;
            }

            // =========================================================================
            // Ternary conditional operator
            // =========================================================================

            if kind == SyntaxKind::Question && prec::CONDITIONAL.left >= min_bp {
                // Make sure it's not optional chaining
                let next = self.peek_kind(1);
                if next != Some(SyntaxKind::Dot) && text != "?." {
                    let start = left.span().start;
                    self.consume(); // ?
                    self.skip_whitespace();

                    // Parse consequent with lowest precedence (can contain commas inside)
                    let consequent = self.parse_expression_with_precedence(0)?;

                    self.skip_whitespace();

                    // Expect colon
                    if !self.at(SyntaxKind::Colon) {
                        return Err(ParseError::new(
                            ParseErrorKind::MissingConditionalColon,
                            self.current_byte_offset(),
                        )
                        .with_expected(&[":"]));
                    }
                    self.consume(); // :

                    self.skip_whitespace();

                    // Parse alternate with conditional precedence (right associative)
                    let alternate = self.parse_expression_with_precedence(prec::CONDITIONAL.right)?;

                    left = IrNode::CondExpr {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        test: Box::new(left),
                        consequent: Box::new(consequent),
                        alternate: Box::new(alternate),
                    };
                    continue;
                }
            }

            // =========================================================================
            // Assignment operators
            // =========================================================================

            if is_assignment_operator(&text) && prec::ASSIGN.left >= min_bp {
                let start = left.span().start;
                let op = text_to_assign_op(&text).ok_or_else(|| {
                    ParseError::new(ParseErrorKind::InvalidOperator, self.current_byte_offset())
                        .with_found(&text)
                })?;

                self.consume();
                self.skip_whitespace();

                // Right-associative: parse with lower precedence
                let right = self.parse_expression_with_precedence(prec::ASSIGN.right)?;

                left = IrNode::AssignExpr {
                    span: IrSpan::new(start, self.current_byte_offset()),
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                continue;
            }

            // =========================================================================
            // Binary operators (including comma/sequence)
            // =========================================================================

            // Check for binary operator by SyntaxKind first
            if let Some(bp) = infix_binding_power(kind, &text) {
                if bp.left >= min_bp {
                    let start = left.span().start;
                    // Get the operator
                    let op = if let Some(op) = text_to_binary_op(&text) {
                        op
                    } else if let Some(op) = keyword_to_binary_op(kind) {
                        op
                    } else {
                        // Should not happen if infix_binding_power returned Some
                        return Err(ParseError::new(ParseErrorKind::InvalidBinaryOperator, self.current_byte_offset())
                            .with_found(&text));
                    };

                    self.consume();
                    self.skip_whitespace();

                    let right = self.parse_expression_with_precedence(bp.right)?;

                    left = IrNode::BinExpr {
                        span: IrSpan::new(start, self.current_byte_offset()),
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    continue;
                }
            }

            // Check for comma (sequence expression)
            if kind == SyntaxKind::Comma && prec::COMMA.left >= min_bp {
                left = self.parse_sequence_expr(left)?;
                continue;
            }

            // =========================================================================
            // No more infix operations at this precedence level
            // =========================================================================

            break;
        }

        Ok(left)
    }

    /// Parses an optional chain expression: x?.y, x?.[y], x?.()
    fn parse_optional_chain(&mut self, obj: IrNode) -> ParseResult<IrNode> {
        let start = obj.span().start;
        // Consume ?. or ? followed by .
        if self.at_text("?.") {
            self.consume();
        } else {
            self.expect(SyntaxKind::Question);
            if self.at(SyntaxKind::Dot) {
                self.consume();
            }
        }

        self.skip_whitespace();

        // Determine what follows the ?.
        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "optional chain"));
        };

        match token.kind {
            // x?.() - optional call
            SyntaxKind::LParen => {
                let args = self.parse_call_arguments()?;
                let end = self.current_byte_offset();
                Ok(IrNode::OptChainExpr {
                    span: IrSpan::new(start, end),
                    base: Box::new(obj),
                    expr: Box::new(IrNode::CallExpr {
                        span: IrSpan::new(start, end),
                        callee: Box::new(IrNode::Placeholder {
                            span: IrSpan::empty(),
                            kind: crate::compiler::ir::PlaceholderKind::Expr,
                            expr: proc_macro2::TokenStream::new(), // sentinel placeholder, actual callee is the base
                        }),
                        type_args: None,
                        args,
                    }),
                })
            }
            // x?.[y] - optional computed access
            SyntaxKind::LBracket => {
                let start_pos = self.pos;
                self.consume();
                self.skip_whitespace();

                let prop = self.parse_expression_with_precedence(0)?;

                self.skip_whitespace();
                if !self.at(SyntaxKind::RBracket) {
                    return Err(ParseError::missing_closing(
                        ParseErrorKind::MissingClosingBracket,
                        self.current_byte_offset(),
                        start_pos,
                    ));
                }
                self.consume();
                let end = self.current_byte_offset();

                Ok(IrNode::OptChainExpr {
                    span: IrSpan::new(start, end),
                    base: Box::new(obj),
                    expr: Box::new(IrNode::MemberExpr {
                        span: IrSpan::new(start, end),
                        obj: Box::new(IrNode::Placeholder {
                            span: IrSpan::empty(),
                            kind: crate::compiler::ir::PlaceholderKind::Expr,
                            expr: proc_macro2::TokenStream::new(), // sentinel placeholder
                        }),
                        prop: Box::new(prop),
                        computed: true,
                    }),
                })
            }
            // x?.y - optional property access
            _ => {
                let prop = self.parse_member_property()?;
                let end = self.current_byte_offset();
                Ok(IrNode::OptChainExpr {
                    span: IrSpan::new(start, end),
                    base: Box::new(obj),
                    expr: Box::new(IrNode::MemberExpr {
                        span: IrSpan::new(start, end),
                        obj: Box::new(IrNode::Placeholder {
                            span: IrSpan::empty(),
                            kind: crate::compiler::ir::PlaceholderKind::Expr,
                            expr: proc_macro2::TokenStream::new(), // sentinel placeholder
                        }),
                        prop: Box::new(prop),
                        computed: false,
                    }),
                })
            }
        }
    }

    /// Parses a property name after a dot.
    fn parse_member_property(&mut self) -> ParseResult<IrNode> {
        let Some(token) = self.current() else {
            return Err(ParseError::new(ParseErrorKind::MissingPropertyName, self.current_byte_offset()));
        };

        match token.kind {
            SyntaxKind::Ident => {
                let t = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "identifier"))?;
                Ok(IrNode::ident(&t))
            }
            // Private identifier: #name
            SyntaxKind::Hash => self.parse_private_name(),
            // Keywords can be used as property names
            _ if token.kind.is_ts_keyword() => {
                let t = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "keyword as property name"))?;
                Ok(IrNode::ident(&t))
            }
            // Placeholder in property position
            SyntaxKind::At => self.parse_interpolation(),
            _ => Err(ParseError::new(ParseErrorKind::MissingPropertyName, self.current_byte_offset())
                .with_found(&token.text)),
        }
    }

    /// Tries to parse type arguments for a call expression.
    ///
    /// Returns `Some(type_args)` if successful, `None` if this doesn't look like type args.
    /// This uses lookahead to distinguish `x<T>()` from `x < T`.
    fn try_parse_type_args_for_call(&mut self) -> ParseResult<Option<IrNode>> {
        // Save position for backtracking
        let saved_pos = self.pos;

        if !self.at(SyntaxKind::Lt) {
            return Ok(None);
        }
        self.consume(); // <

        // Try to parse type arguments
        let mut types = Vec::new();
        let mut depth = 1;

        loop {
            self.skip_whitespace();

            if self.at_eof() {
                // Hit EOF, backtrack
                self.restore_pos(saved_pos);
                return Ok(None);
            }

            if self.at(SyntaxKind::Gt) {
                depth -= 1;
                self.consume();
                if depth == 0 {
                    break;
                }
            } else if self.at(SyntaxKind::Lt) {
                depth += 1;
                self.consume();
            } else if self.at(SyntaxKind::Comma) && depth == 1 {
                self.consume();
            } else {
                // Try to parse a type
                match self.parse_type() {
                    Ok(ty) => types.push(ty),
                    Err(_) => {
                        // Doesn't look like type args, backtrack
                        self.restore_pos(saved_pos);
                        return Ok(None);
                    }
                }
            }
        }

        self.skip_whitespace();

        // After >, should see ( or ` or . or another valid continuation
        // If we see something that doesn't make sense, backtrack
        if let Some(token) = self.current() {
            let valid_after = matches!(
                token.kind,
                SyntaxKind::LParen
                    | SyntaxKind::Backtick
                    | SyntaxKind::Dot
                    | SyntaxKind::Semicolon
                    | SyntaxKind::RParen
                    | SyntaxKind::RBracket
                    | SyntaxKind::RBrace
                    | SyntaxKind::Comma
            ) || token.text == "=>"
                || token.text == "?"
                || is_assignment_operator(&token.text);

            if !valid_after {
                // Doesn't look like type args, probably comparison
                self.restore_pos(saved_pos);
                return Ok(None);
            }
        }

        Ok(Some(IrNode::TypeArgs {
            span: IrSpan::new(saved_pos, self.current_byte_offset()),
            args: types,
        }))
    }

    /// Parses a sequence expression: x, y, z
    fn parse_sequence_expr(&mut self, first: IrNode) -> ParseResult<IrNode> {
        let start = first.span().start;
        let mut exprs = vec![first];

        while self.at(SyntaxKind::Comma) {
            self.consume();
            self.skip_whitespace();

            // Parse next expression with assignment precedence
            // (each element in sequence can be an assignment)
            let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
            exprs.push(expr);

            self.skip_whitespace();
        }

        if exprs.len() == 1 {
            Ok(exprs.remove(0))
        } else {
            Ok(IrNode::SeqExpr {
                span: IrSpan::new(start, self.current_byte_offset()),
                exprs,
            })
        }
    }

    /// Parses call arguments: (arg1, arg2, ...)
    pub(super) fn parse_call_arguments(&mut self) -> ParseResult<Vec<IrNode>> {
        let start_pos = self.pos;

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["("])
        })?;

        let mut args = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RParen) {
                break;
            }

            // Check for spread argument: ...expr
            if self.at(SyntaxKind::DotDotDot) {
                let spread_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                args.push(IrNode::SpreadElement {
                    span: IrSpan::new(spread_start, self.current_byte_offset()),
                    expr: Box::new(expr),
                });
            } else {
                // Regular argument
                let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                args.push(expr);
            }

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::RParen) {
            return Err(ParseError::missing_closing(
                ParseErrorKind::MissingClosingParen,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume(); // )

        Ok(args)
    }

    /// Peeks at the text of a token at the given offset.
    fn peek_text(&self, offset: usize) -> Option<&str> {
        self.tokens.get(self.pos + offset).map(|t| t.text.as_str())
    }

    /// Peeks at the kind of a token at the given offset.
    pub(in crate::compiler::parser) fn peek_kind(&self, offset: usize) -> Option<SyntaxKind> {
        self.tokens.get(self.pos + offset).map(|t| t.kind)
    }

    /// Restores the parser position (for backtracking).
    fn restore_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added with the integration
}
