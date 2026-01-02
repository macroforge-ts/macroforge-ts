//! Primary expression parsing.
//!
//! This module handles parsing of primary expressions - the "atoms" of expressions:
//! - Literals (string, number, boolean, null, bigint)
//! - Identifiers and private names
//! - `this` and `super`
//! - Placeholders (@{expr})
//! - Grouped expressions (parenthesized)
//! - Array and object literals
//! - Function and class expressions
//! - Template literals

use super::errors::{ParseError, ParseErrorKind, ParseResult};
use super::operators::{to_unary_op, to_update_op};
use super::precedence::prec;
use crate::compiler::ir::{IntoIrNode, IrNode, IrSpan, PlaceholderKind, UnaryOp};
use crate::compiler::parser::Parser;
use crate::compiler::syntax::SyntaxKind;

impl Parser {
    /// Parses a primary expression.
    ///
    /// Primary expressions are the "atoms" that make up more complex expressions.
    pub(super) fn parse_primary_expr(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "expression"));
        };

        let kind = token.kind;
        let text = token.text.clone();

        match kind {
            // Placeholder interpolation: @{expr} or composite like @{a}Middle@{b}
            SyntaxKind::At => self.parse_interpolated_ident(),

            // Update operators as prefix: ++x, --x
            SyntaxKind::PlusPlus | SyntaxKind::MinusMinus => self.parse_prefix_update(),

            // Unary keyword operators: typeof, void, delete
            SyntaxKind::TypeofKw | SyntaxKind::VoidKw | SyntaxKind::DeleteKw => {
                self.parse_unary_keyword_expr()
            }

            // Exclamation mark: could be unary ! operator
            SyntaxKind::Exclaim => self.parse_unary_expr(),

            // new expression: new Foo()
            SyntaxKind::NewKw => self.parse_new_expr(),

            // await expression: await promise
            SyntaxKind::AwaitKw => self.parse_await_expr(),

            // yield expression: yield value
            SyntaxKind::YieldKw => self.parse_yield_expr(),

            // function expression: function() {}
            SyntaxKind::FunctionKw => self.parse_function_expr(),

            // async: could be async function or async arrow
            SyntaxKind::AsyncKw => self.parse_async_expr(),

            // class expression: class {}
            SyntaxKind::ClassKw => self.parse_class_expr(),

            // Parenthesized expression or arrow function: (x) or (x) => x
            SyntaxKind::LParen => self.parse_paren_or_arrow(),

            // Array literal: [1, 2, 3]
            SyntaxKind::LBracket => self.parse_array_literal(),

            // Object literal: { a: 1, b: 2 }
            SyntaxKind::LBrace => self.parse_object_literal(),

            // Template literal: `hello ${name}`
            SyntaxKind::Backtick => self.parse_template_literal(),

            // String literals: "hello" or 'hello'
            SyntaxKind::DoubleQuote | SyntaxKind::SingleQuote => self.parse_string_literal(),

            // Keywords that are expressions
            SyntaxKind::ThisKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::this_expr(&token))
            }
            SyntaxKind::SuperKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::super_expr(&token))
            }
            SyntaxKind::NullKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::null_lit(&token))
            }
            SyntaxKind::TrueKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::bool_lit(&token))
            }
            SyntaxKind::FalseKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::bool_lit(&token))
            }
            SyntaxKind::UndefinedKw => {
                let token = self.consume().unwrap();
                Ok(IrNode::ident_with(&token, "undefined"))
            }

            // Private name: #field
            SyntaxKind::Hash => self.parse_private_name(),

            // Identifier
            SyntaxKind::Ident => self.parse_identifier_expr(),

            // Text that might be an identifier or unary operator
            SyntaxKind::Text => {
                // Check if it's a unary operator
                if matches!(text.as_str(), "-" | "+" | "!" | "~") {
                    self.parse_unary_expr()
                } else if text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    self.parse_numeric_literal()
                } else {
                    // Treat as identifier
                    self.parse_identifier_expr()
                }
            }

            // Other keywords that can be used as identifiers in certain contexts
            _ if kind.is_ts_keyword() => self.parse_identifier_expr(),

            // Template control blocks in expression context
            SyntaxKind::BraceHashIf => self.parse_if_expr(),
            SyntaxKind::BraceHashFor => self.parse_for_expr(),
            SyntaxKind::BraceHashWhile => self.parse_while_expr(),
            SyntaxKind::BraceHashMatch => self.parse_match_expr(),

            _ => Err(ParseError::expected_expression_found(self.current_byte_offset(), kind)),
        }
    }

    /// Parses a prefix update expression: ++x or --x
    fn parse_prefix_update(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let token = self.consume().ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedEof, self.current_byte_offset())
                .with_context("prefix update operator")
        })?;

        let op = to_update_op(token.kind).ok_or_else(|| {
            ParseError::new(ParseErrorKind::InvalidPrefixOperator, self.current_byte_offset())
                .with_found(&token.text)
        })?;

        self.skip_whitespace();
        let arg = self.parse_unary_operand()?;

        Ok(IrNode::UpdateExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            op,
            prefix: true,
            arg: Box::new(arg),
        })
    }

    /// Parses a unary keyword expression: typeof x, void x, delete x
    fn parse_unary_keyword_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let token = self.consume().ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedEof, self.current_byte_offset())
                .with_context("unary keyword operator")
        })?;

        let op = to_unary_op(token.kind, &token.text).ok_or_else(|| {
            ParseError::new(ParseErrorKind::InvalidPrefixOperator, self.current_byte_offset())
                .with_found(&token.text)
        })?;

        self.skip_whitespace();
        let arg = self.parse_unary_operand()?;

        Ok(IrNode::UnaryExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            op,
            arg: Box::new(arg),
        })
    }

    /// Parses a unary expression: -x, +x, !x, ~x
    fn parse_unary_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let token = self.consume().ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedEof, self.current_byte_offset())
                .with_context("unary operator")
        })?;

        let op = to_unary_op(token.kind, &token.text).ok_or_else(|| {
            ParseError::new(ParseErrorKind::InvalidPrefixOperator, self.current_byte_offset())
                .with_found(&token.text)
        })?;

        self.skip_whitespace();
        let arg = self.parse_unary_operand()?;

        Ok(IrNode::UnaryExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            op,
            arg: Box::new(arg),
        })
    }

    /// Parses the operand of a unary expression.
    /// This handles the right-to-left associativity of unary operators.
    fn parse_unary_operand(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        let Some(token) = self.current() else {
            return Err(ParseError::new(ParseErrorKind::MissingOperand, self.current_byte_offset())
                .with_context("unary expression"));
        };

        // Check for chained unary operators
        match token.kind {
            SyntaxKind::PlusPlus | SyntaxKind::MinusMinus => self.parse_prefix_update(),
            SyntaxKind::TypeofKw | SyntaxKind::VoidKw | SyntaxKind::DeleteKw => {
                self.parse_unary_keyword_expr()
            }
            SyntaxKind::Exclaim => self.parse_unary_expr(),
            SyntaxKind::Text if matches!(token.text.as_str(), "-" | "+" | "~") => {
                self.parse_unary_expr()
            }
            SyntaxKind::AwaitKw => self.parse_await_expr(),
            _ => self.parse_primary_expr(),
        }
    }

    /// Parses a new expression: new Foo() or new Foo
    fn parse_new_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::NewKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["new"])
        })?;

        self.skip_whitespace();

        // Parse the callee (could be another new expression)
        let callee = if self.at(SyntaxKind::NewKw) {
            self.parse_new_expr()?
        } else {
            self.parse_primary_expr()?
        };

        self.skip_whitespace();

        // Parse optional type arguments
        let type_args = self.parse_optional_type_args()?;

        // Parse optional arguments
        let args = if self.at(SyntaxKind::LParen) {
            self.parse_call_arguments()?
        } else {
            Vec::new()
        };

        Ok(IrNode::NewExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            callee: Box::new(callee),
            type_args: type_args.map(Box::new),
            args,
        })
    }

    /// Parses an await expression: await promise
    fn parse_await_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::AwaitKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["await"])
        })?;

        self.skip_whitespace();
        let arg = self.parse_unary_operand()?;

        Ok(IrNode::AwaitExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            arg: Box::new(arg),
        })
    }

    /// Parses a yield expression: yield value or yield* value
    fn parse_yield_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::YieldKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["yield"])
        })?;

        self.skip_whitespace();

        // Check for yield*
        let delegate = if self.at(SyntaxKind::Star) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // yield can have no argument in some contexts
        let arg = if self.is_at_expression_start() {
            Some(Box::new(self.parse_unary_operand()?))
        } else {
            None
        };

        Ok(IrNode::YieldExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            delegate,
            arg,
        })
    }

    /// Parses a function expression: function name?() {}
    fn parse_function_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::FunctionKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["function"])
        })?;

        self.skip_whitespace();

        // Check for generator: function*
        let generator = if self.at(SyntaxKind::Star) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Optional name
        let name = if self.at(SyntaxKind::Ident) {
            let token = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "function name"))?;
            Some(Box::new(IrNode::ident(&token)))
        } else {
            None
        };

        self.skip_whitespace();

        // Parse type parameters
        let type_params = self.parse_optional_type_params();

        // Parse parameters
        let params = self.parse_function_params()?;

        self.skip_whitespace();

        // Parse return type
        let return_type = self.parse_optional_return_type()?;

        self.skip_whitespace();

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };

        Ok(IrNode::FnExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            async_: false,
            generator,
            name,
            type_params,
            params,
            return_type,
            body,
        })
    }

    /// Parses an async expression: async function or async arrow
    fn parse_async_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::AsyncKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["async"])
        })?;

        self.skip_whitespace();

        if self.at(SyntaxKind::FunctionKw) {
            // async function
            self.consume();
            self.skip_whitespace();

            // Check for generator: async function*
            let generator = if self.at(SyntaxKind::Star) {
                self.consume();
                self.skip_whitespace();
                true
            } else {
                false
            };

            // Optional name
            let name = if self.at(SyntaxKind::Ident) {
                let token = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "async function name"))?;
                Some(Box::new(IrNode::ident(&token)))
            } else {
                None
            };

            self.skip_whitespace();

            let type_params = self.parse_optional_type_params();
            let params = self.parse_function_params()?;
            self.skip_whitespace();
            let return_type = self.parse_optional_return_type()?;
            self.skip_whitespace();

            let body = if self.at(SyntaxKind::LBrace) {
                Some(Box::new(self.parse_block_stmt()?))
            } else {
                None
            };

            Ok(IrNode::FnExpr {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                async_: true,
                generator,
                name,
                type_params,
                params,
                return_type,
                body,
            })
        } else {
            // async arrow function: async (x) => x or async x => x
            self.parse_async_arrow_function_from(start_byte)
        }
    }

    /// Parses an async arrow function: async (x) => x or async x => x
    fn parse_async_arrow_function_from(&mut self, start_byte: usize) -> ParseResult<IrNode> {
        // async keyword already consumed
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        let params = self.parse_arrow_params()?;

        self.skip_whitespace();
        let return_type = self.parse_optional_return_type()?;

        self.skip_whitespace();

        // Expect =>
        if !self.at_text("=>") {
            return Err(ParseError::new(ParseErrorKind::MissingArrowBody, self.current_byte_offset())
                .with_expected(&["=>"]));
        }
        self.consume(); // =>

        self.skip_whitespace();

        // Parse body (expression or block)
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()?
        } else {
            // Expression body - need to parse with assignment precedence
            self.parse_expression_with_precedence(prec::ASSIGN.right)?
        };

        Ok(IrNode::ArrowExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            async_: true,
            type_params,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    /// Parses a class expression: class Name? extends Base? {}
    fn parse_class_expr(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::ClassKw).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["class"])
        })?;

        self.skip_whitespace();

        // Optional name
        let name = if self.at(SyntaxKind::Ident) {
            let token = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "class name"))?;
            Some(Box::new(IrNode::ident(&token)))
        } else {
            None
        };

        self.skip_whitespace();

        // Type parameters
        let type_params = self.parse_optional_type_params();

        self.skip_whitespace();

        // extends clause
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_primary_expr()?))
        } else {
            None
        };

        self.skip_whitespace();

        // implements clause
        let implements = if self.at(SyntaxKind::ImplementsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_implements_list()?
        } else {
            Vec::new()
        };

        self.skip_whitespace();

        // Parse class body
        let body = self.parse_class_body()?;

        Ok(IrNode::ClassExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            name,
            type_params,
            extends,
            implements,
            body,
        })
    }

    /// Parses a parenthesized expression or arrow function.
    fn parse_paren_or_arrow(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["("])
        })?;

        self.skip_whitespace();

        // Empty parens - either () => or ()
        if self.at(SyntaxKind::RParen) {
            self.consume();
            self.skip_whitespace();

            // Check for arrow
            if self.at_text("=>") {
                return self.parse_arrow_after_params_from(Vec::new(), start_pos);
            }

            // Empty parentheses without arrow - error or undefined behavior
            return Err(ParseError::new(ParseErrorKind::ExpectedExpression, self.current_byte_offset())
                .with_context("parenthesized expression"));
        }

        // Try to determine if this is arrow function params or expression
        // Heuristic: if we see `:` for type annotation, it's likely arrow params
        // Otherwise, parse as expression first

        // Parse the first element
        let first = self.parse_expression_with_precedence(0)?;

        self.skip_whitespace();

        // If we see a comma, could be sequence or arrow params
        if self.at(SyntaxKind::Comma) {
            // Parse remaining as potential arrow params or sequence
            let mut items = vec![first];

            while self.at(SyntaxKind::Comma) {
                self.consume();
                self.skip_whitespace();

                if self.at(SyntaxKind::RParen) {
                    break; // Trailing comma
                }

                items.push(self.parse_expression_with_precedence(0)?);
                self.skip_whitespace();
            }

            if !self.at(SyntaxKind::RParen) {
                return Err(ParseError::missing_closing(
                    ParseErrorKind::MissingClosingParen,
                    self.current_byte_offset(),
                    start_pos,
                ));
            }
            self.consume(); // )

            self.skip_whitespace();

            // Check for arrow
            if self.at_text("=>") {
                // Convert expressions to params
                let params = self.exprs_to_params(items)?;
                return self.parse_arrow_after_params_from(params, start_byte);
            }

            // It's a sequence expression wrapped in parens
            let end_byte = self.current_byte_offset();
            if items.len() == 1 {
                return Ok(IrNode::ParenExpr {
                    span: IrSpan::new(start_byte, end_byte),
                    expr: Box::new(items.remove(0)),
                });
            } else {
                return Ok(IrNode::ParenExpr {
                    span: IrSpan::new(start_byte, end_byte),
                    expr: Box::new(IrNode::SeqExpr {
                        span: IrSpan::new(start_byte, end_byte),
                        exprs: items,
                    }),
                });
            }
        }

        // Single expression
        if !self.at(SyntaxKind::RParen) {
            return Err(ParseError::missing_closing(
                ParseErrorKind::MissingClosingParen,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume(); // )

        self.skip_whitespace();

        // Check for arrow
        if self.at_text("=>") {
            let params = self.exprs_to_params(vec![first])?;
            return self.parse_arrow_after_params_from(params, start_byte);
        }

        // Just a parenthesized expression
        Ok(IrNode::ParenExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            expr: Box::new(first),
        })
    }

    /// Parses the rest of an arrow function after parameters are known.
    fn parse_arrow_after_params_from(
        &mut self,
        params: Vec<IrNode>,
        start_byte: usize,
    ) -> ParseResult<IrNode> {
        self.skip_whitespace();

        // Consume =>
        if !self.at_text("=>") {
            return Err(ParseError::new(ParseErrorKind::MissingArrowBody, self.current_byte_offset())
                .with_expected(&["=>"]));
        }
        self.consume();

        self.skip_whitespace();

        // Parse optional return type (for typed arrow functions like (x): number => x)
        let return_type = self.parse_optional_return_type()?;

        self.skip_whitespace();

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()?
        } else {
            self.parse_expression_with_precedence(prec::ASSIGN.right)?
        };

        Ok(IrNode::ArrowExpr {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            async_: false,
            type_params: None,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    /// Converts a list of expressions to arrow function parameters.
    fn exprs_to_params(&self, exprs: Vec<IrNode>) -> ParseResult<Vec<IrNode>> {
        let mut params = Vec::new();

        for expr in exprs {
            let param = match expr {
                IrNode::Ident { span, value } => IrNode::Param {
                    span,
                    decorators: Vec::new(),
                    pat: Box::new(IrNode::BindingIdent {
                        span,
                        name: Box::new(IrNode::Ident { span, value }),
                        type_ann: None,
                        optional: false,
                    }),
                },
                IrNode::Placeholder { span, kind, expr } => IrNode::Param {
                    span,
                    decorators: Vec::new(),
                    pat: Box::new(IrNode::Placeholder { span, kind, expr }),
                },
                // Could add more patterns here (destructuring, etc.)
                other => IrNode::Param {
                    span: other.span(),
                    decorators: Vec::new(),
                    pat: Box::new(other),
                },
            };
            params.push(param);
        }

        Ok(params)
    }

    /// Parses an array literal: [elem1, elem2, ...]
    fn parse_array_literal(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBracket).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["["])
        })?;

        let mut elems = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            // Check for spread element
            if self.at(SyntaxKind::DotDotDot) {
                let spread_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                elems.push(IrNode::SpreadElement {
                    span: IrSpan::new(spread_start, self.current_byte_offset()),
                    expr: Box::new(expr),
                });
            } else if self.at(SyntaxKind::Comma) {
                // Hole in array: [1, , 3]
                // We represent this by not pushing anything between commas
                // For now, just skip
            } else {
                let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                elems.push(expr);
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
        self.consume(); // ]

        Ok(IrNode::ArrayLit {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            elems,
        })
    }

    /// Parses an object literal: { prop1: val1, prop2, ...spread }
    fn parse_object_literal(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBrace).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{"])
        })?;

        let mut props = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            let prop = self.parse_object_property()?;
            props.push(prop);

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
        self.consume(); // }

        Ok(IrNode::ObjectLit {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            props,
        })
    }

    /// Parses a single object property.
    fn parse_object_property(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Control blocks in object literal context: {#if}, {#for}, {#while}, {#match}
        // These can generate multiple properties dynamically
        match self.current_kind() {
            Some(SyntaxKind::BraceHashIf) => return self.parse_if_expr(),
            Some(SyntaxKind::BraceHashFor) => return self.parse_for_expr(),
            Some(SyntaxKind::BraceHashWhile) => return self.parse_while_expr(),
            Some(SyntaxKind::BraceHashMatch) => return self.parse_match_expr(),
            _ => {}
        }

        // Spread property: ...expr
        if self.at(SyntaxKind::DotDotDot) {
            self.consume();
            self.skip_whitespace();
            let expr = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
            return Ok(IrNode::SpreadElement {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                expr: Box::new(expr),
            });
        }

        // Placeholder as property: @{prop}, @{key}: value, or @{name}() {}
        if self.at(SyntaxKind::At) {
            let placeholder = self.parse_interpolation()?;

            self.skip_whitespace();

            // Method shorthand: @{name}() {} or @{name}<T>() {}
            if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
                return self.parse_method_prop_from(placeholder, false, false, start_byte);
            }

            if self.at(SyntaxKind::Colon) {
                // @{key}: value
                self.consume();
                self.skip_whitespace();
                let value = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                return Ok(IrNode::KeyValueProp {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    key: Box::new(placeholder),
                    value: Box::new(value),
                });
            } else {
                // Just @{prop} - shorthand
                return Ok(IrNode::ShorthandProp {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    key: Box::new(placeholder),
                });
            }
        }

        // Check for getter/setter
        if self.at(SyntaxKind::GetKw) || self.at(SyntaxKind::SetKw) {
            return self.parse_getter_setter_prop_from(start_byte);
        }

        // Check for async method
        if self.at(SyntaxKind::AsyncKw) {
            return self.parse_async_method_prop_from(start_byte);
        }

        // Check for generator method: *name()
        if self.at(SyntaxKind::Star) {
            return self.parse_generator_method_prop_from(start_byte);
        }

        // Computed property: [expr]: value
        if self.at(SyntaxKind::LBracket) {
            return self.parse_computed_property_from(start_byte);
        }

        // Regular property: key: value or shorthand
        let key = self.parse_property_name()?;

        self.skip_whitespace();

        // Method shorthand: name() {}
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            return self.parse_method_prop_from(key, false, false, start_byte);
        }

        // Key: value
        if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            let value = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
            return Ok(IrNode::KeyValueProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key),
                value: Box::new(value),
            });
        }

        // Shorthand: name (same as name: name)
        Ok(IrNode::ShorthandProp {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            key: Box::new(key),
        })
    }

    /// Parses a property name (identifier, string, number, computed, or placeholder).
    pub(in crate::compiler::parser) fn parse_property_name(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "property name"));
        };

        match token.kind {
            SyntaxKind::Ident => {
                let t = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "property name identifier"))?;
                Ok(IrNode::ident(&t))
            }
            SyntaxKind::DoubleQuote | SyntaxKind::SingleQuote => self.parse_string_literal(),
            SyntaxKind::Text if token.text.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                self.parse_numeric_literal()
            }
            SyntaxKind::At => {
                // Placeholder as property name: @{name}
                self.parse_interpolation()
            }
            SyntaxKind::LBracket => {
                // Computed property name: [expr]
                self.consume();
                self.skip_whitespace();
                let expr = self.parse_expression_with_precedence(0)?;
                self.skip_whitespace();
                self.expect(SyntaxKind::RBracket).ok_or_else(|| {
                    ParseError::new(ParseErrorKind::MissingClosingBracket, self.current_byte_offset())
                        .with_context("computed property name")
                })?;
                Ok(IrNode::ComputedPropName {
                    span: IrSpan::empty(),
                    expr: Box::new(expr),
                })
            }
            _ if token.kind.is_ts_keyword() => {
                // Keywords can be property names
                let t = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "property name keyword"))?;
                Ok(IrNode::ident(&t))
            }
            _ => Err(ParseError::new(ParseErrorKind::InvalidPropertyName, self.current_byte_offset())
                .with_found(&token.text)),
        }
    }

    /// Parses a getter or setter property.
    fn parse_getter_setter_prop_from(&mut self, start_byte: usize) -> ParseResult<IrNode> {
        let is_getter = self.at(SyntaxKind::GetKw);
        self.consume(); // get/set

        self.skip_whitespace();

        let name = self.parse_property_name()?;

        self.skip_whitespace();

        if is_getter {
            // get name() { body }
            self.expect(SyntaxKind::LParen);
            self.skip_whitespace();
            self.expect(SyntaxKind::RParen);
            self.skip_whitespace();

            let type_ann = self.parse_optional_return_type()?;
            self.skip_whitespace();

            let body = self.parse_block_stmt()?;

            Ok(IrNode::GetterProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                name: Box::new(name),
                type_ann,
                body: Box::new(body),
            })
        } else {
            // set name(param) { body }
            self.expect(SyntaxKind::LParen);
            self.skip_whitespace();

            let param = self.parse_single_param()?;

            self.skip_whitespace();
            self.expect(SyntaxKind::RParen);
            self.skip_whitespace();

            let body = self.parse_block_stmt()?;

            Ok(IrNode::SetterProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                name: Box::new(name),
                param: Box::new(param),
                body: Box::new(body),
            })
        }
    }

    /// Parses an async method property.
    fn parse_async_method_prop_from(&mut self, start_byte: usize) -> ParseResult<IrNode> {
        self.consume(); // async
        self.skip_whitespace();

        let generator = if self.at(SyntaxKind::Star) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        let name = self.parse_property_name()?;
        self.parse_method_prop_from(name, true, generator, start_byte)
    }

    /// Parses a generator method property.
    fn parse_generator_method_prop_from(&mut self, start_byte: usize) -> ParseResult<IrNode> {
        self.consume(); // *
        self.skip_whitespace();

        let name = self.parse_property_name()?;
        self.parse_method_prop_from(name, false, true, start_byte)
    }

    /// Parses a computed property: [expr]: value
    fn parse_computed_property_from(&mut self, start_byte: usize) -> ParseResult<IrNode> {
        let start_pos = self.pos;

        self.expect(SyntaxKind::LBracket);
        self.skip_whitespace();

        let expr = self.parse_expression_with_precedence(0)?;

        self.skip_whitespace();

        if !self.at(SyntaxKind::RBracket) {
            return Err(ParseError::missing_closing(
                ParseErrorKind::MissingClosingBracket,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume(); // ]

        self.skip_whitespace();

        let key = IrNode::ComputedPropName {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            expr: Box::new(expr),
        };

        // Check if it's a method
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            return self.parse_method_prop_from(key, false, false, start_byte);
        }

        // Key: value
        if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            let value = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
            return Ok(IrNode::KeyValueProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key),
                value: Box::new(value),
            });
        }

        Err(ParseError::new(ParseErrorKind::InvalidComputedProperty, self.current_byte_offset()))
    }

    /// Parses a method property: name() {}
    fn parse_method_prop_from(
        &mut self,
        name: IrNode,
        async_: bool,
        generator: bool,
        start_byte: usize,
    ) -> ParseResult<IrNode> {
        let type_params = self.parse_optional_type_params();

        let params = self.parse_function_params()?;

        self.skip_whitespace();

        let return_type = self.parse_optional_return_type()?;

        self.skip_whitespace();

        let body = self.parse_block_stmt()?;

        Ok(IrNode::MethodProp {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            async_,
            generator,
            name: Box::new(name),
            type_params,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    /// Parses a template literal: `text ${expr} more`
    pub(in crate::compiler::parser) fn parse_template_literal(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_offset = self.current_byte_offset();

        self.expect(SyntaxKind::Backtick).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["`"])
        })?;

        let mut quasis = Vec::new();
        let mut exprs = Vec::new();
        let mut current_text = String::new();

        loop {
            if self.at_eof() {
                return Err(ParseError::new(
                    ParseErrorKind::UnterminatedTemplateLiteral,
                    start_pos,
                ));
            }

            // Check for closing backtick
            if self.at(SyntaxKind::Backtick) {
                quasis.push(current_text);
                self.consume();
                break;
            }

            // Check for JS template interpolation: ${
            if self.at(SyntaxKind::DollarBrace) {
                quasis.push(std::mem::take(&mut current_text));
                self.consume(); // ${

                self.skip_whitespace();
                let expr = self.parse_expression_with_precedence(0)?;
                exprs.push(expr);

                self.skip_whitespace();

                // Expect closing }
                if !self.at(SyntaxKind::RBrace) {
                    return Err(ParseError::missing_closing(
                        ParseErrorKind::MissingClosingBrace,
                        self.current_byte_offset(),
                        start_pos,
                    ));
                }
                self.consume(); // }
            } else if self.at(SyntaxKind::At) {
                // @{} placeholder inside template
                quasis.push(std::mem::take(&mut current_text));
                let placeholder = self.parse_interpolation()?;
                exprs.push(placeholder);
            } else {
                // Regular text
                if let Some(token) = self.consume() {
                    current_text.push_str(&token.text);
                }
            }
        }

        let end_offset = self.current_byte_offset();
        Ok(IrNode::TplLit {
            span: IrSpan::new(start_offset, end_offset),
            quasis,
            exprs,
        })
    }

    /// Parses a string literal.
    pub(in crate::compiler::parser) fn parse_string_literal(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_offset = self.current_byte_offset();

        let quote_kind = self.current_kind().ok_or_else(|| {
            ParseError::unexpected_eof(self.current_byte_offset(), "string literal")
        })?;

        let quote_char = if quote_kind == SyntaxKind::DoubleQuote {
            '"'
        } else {
            '\''
        };

        self.consume(); // opening quote

        let mut content = String::new();
        let mut parts: Vec<IrNode> = Vec::new();
        let mut has_interpolation = false;

        loop {
            if self.at_eof() {
                return Err(ParseError::new(
                    ParseErrorKind::UnterminatedStringLiteral,
                    start_pos,
                ));
            }

            // Check for closing quote
            if self.at(quote_kind) {
                if has_interpolation {
                    if !content.is_empty() {
                        parts.push(IrNode::StrLit {
                            span: IrSpan::empty(),
                            value: std::mem::take(&mut content),
                        });
                    }
                }
                self.consume();
                break;
            }

            // Check for @{} interpolation
            if self.at(SyntaxKind::At) {
                has_interpolation = true;
                if !content.is_empty() {
                    parts.push(IrNode::StrLit {
                        span: IrSpan::empty(),
                        value: std::mem::take(&mut content),
                    });
                }
                let placeholder = self.parse_interpolation()?;
                parts.push(placeholder);
            } else {
                // Regular text
                if let Some(token) = self.consume() {
                    content.push_str(&token.text);
                }
            }
        }

        let end_offset = self.current_byte_offset();
        let span = IrSpan::new(start_offset, end_offset);

        if has_interpolation {
            if !content.is_empty() {
                parts.push(IrNode::StrLit {
                    span: IrSpan::empty(),
                    value: content,
                });
            }
            Ok(IrNode::StringInterp {
                span,
                quote: quote_char,
                parts,
            })
        } else {
            Ok(IrNode::StrLit {
                span,
                value: content,
            })
        }
    }

    /// Parses a numeric literal.
    fn parse_numeric_literal(&mut self) -> ParseResult<IrNode> {
        let token = self.consume().ok_or_else(|| {
            ParseError::unexpected_eof(self.current_byte_offset(), "numeric literal")
        })?;

        let text = &token.text;

        // Check for BigInt
        if text.ends_with('n') {
            Ok(IrNode::BigIntLit {
                span: token.ir_span(),
                value: text[..text.len() - 1].to_string(),
            })
        } else {
            Ok(IrNode::num_lit(&token))
        }
    }

    /// Parses a private name: #field
    pub(in crate::compiler::parser) fn parse_private_name(&mut self) -> ParseResult<IrNode> {
        self.expect(SyntaxKind::Hash).ok_or_else(|| {
            ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["#"])
        })?;

        let name_token = self.expect(SyntaxKind::Ident).ok_or_else(|| {
            ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("private name")
        })?;

        Ok(IrNode::private_name(&name_token))
    }

    /// Parses an identifier expression.
    fn parse_identifier_expr(&mut self) -> ParseResult<IrNode> {
        let token = self.consume().ok_or_else(|| {
            ParseError::unexpected_eof(self.current_byte_offset(), "identifier")
        })?;

        Ok(IrNode::ident(&token))
    }

    // =========================================================================
    // Helper methods
    // =========================================================================

    /// Checks if the current position could start an expression.
    fn is_at_expression_start(&self) -> bool {
        let Some(token) = self.current() else {
            return false;
        };

        matches!(
            token.kind,
            SyntaxKind::Ident
                | SyntaxKind::Text
                | SyntaxKind::At
                | SyntaxKind::LParen
                | SyntaxKind::LBracket
                | SyntaxKind::LBrace
                | SyntaxKind::Backtick
                | SyntaxKind::DoubleQuote
                | SyntaxKind::SingleQuote
                | SyntaxKind::PlusPlus
                | SyntaxKind::MinusMinus
                | SyntaxKind::Exclaim
                | SyntaxKind::Hash
                | SyntaxKind::NewKw
                | SyntaxKind::FunctionKw
                | SyntaxKind::ClassKw
                | SyntaxKind::AsyncKw
                | SyntaxKind::AwaitKw
                | SyntaxKind::YieldKw
                | SyntaxKind::TypeofKw
                | SyntaxKind::VoidKw
                | SyntaxKind::DeleteKw
                | SyntaxKind::ThisKw
                | SyntaxKind::SuperKw
                | SyntaxKind::NullKw
                | SyntaxKind::TrueKw
                | SyntaxKind::FalseKw
                // Template control blocks in expression context
                | SyntaxKind::BraceHashIf
                | SyntaxKind::BraceHashFor
                | SyntaxKind::BraceHashWhile
                | SyntaxKind::BraceHashMatch
        ) || matches!(token.text.as_str(), "-" | "+" | "~")
    }

    /// Checks if current token text matches.
    pub(in crate::compiler::parser) fn at_text(&self, text: &str) -> bool {
        self.current().map(|t| t.text.as_str() == text).unwrap_or(false)
    }

    /// Gets current token text.
    pub(in crate::compiler::parser) fn current_text(&self) -> Option<&str> {
        self.current().map(|t| t.text.as_str())
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added with the integration
}
