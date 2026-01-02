//! Expression parser module.
//!
//! This module implements a Pratt parser (top-down operator precedence parser)
//! for TypeScript/JavaScript expressions. It properly handles:
//!
//! - Operator precedence and associativity
//! - Prefix operators (-, +, !, ~, typeof, void, delete, ++, --)
//! - Postfix operators (++, --, !)
//! - Binary operators (arithmetic, logical, bitwise, comparison)
//! - Assignment operators (=, +=, -=, etc.)
//! - Ternary conditional (?:)
//! - Member access (.property, [computed])
//! - Optional chaining (?.)
//! - Function calls and type instantiation
//! - TypeScript type assertions (as, satisfies)
//! - Template literals and tagged templates
//! - Sequence expressions (comma operator)
//!
//! # Architecture
//!
//! The parser follows the Pratt parser pattern:
//!
//! 1. **`parse_expression`** - Main entry point, calls `parse_expression_with_precedence(0)`
//! 2. **`parse_expression_with_precedence`** - Core Pratt loop
//!    - Parses a prefix expression (primary.rs)
//!    - Loops to parse infix/postfix operators (postfix.rs)
//! 3. **`parse_primary_expr`** - Handles "atoms" (literals, identifiers, etc.)
//! 4. **`parse_postfix_and_infix`** - Handles operators after the initial atom

pub mod control_expr;
pub mod errors;
pub mod operators;
pub mod postfix;
pub mod precedence;
pub mod primary;

use crate::compiler::ir::{IntoIrNode, IrNode, IrSpan};
use crate::compiler::parser::Parser;
use crate::compiler::syntax::SyntaxKind;
use errors::{ParseError, ParseResult};
use precedence::prec;

impl Parser {
    /// Parses an expression.
    ///
    /// This is the main entry point for expression parsing.
    /// Returns a fully-parsed expression IR node.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let expr = parser.parse_expression()?;
    /// // expr might be: BinExpr { op: Add, left: NumLit("1"), right: NumLit("2") }
    /// ```
    pub fn parse_expression(&mut self) -> ParseResult<IrNode> {
        self.parse_expression_with_precedence(0)
    }

    /// Parses an expression with a minimum binding power.
    ///
    /// This is the core of the Pratt parser. It:
    /// 1. Parses a prefix expression (the "left" side)
    /// 2. Loops to parse infix operators while their precedence is high enough
    ///
    /// The `min_bp` parameter controls which operators can be parsed:
    /// - 0: Parse any expression (including comma)
    /// - prec::ASSIGN.right: Parse assignment-level and higher
    /// - prec::CONDITIONAL.right: Parse ternary-level and higher
    /// - etc.
    pub fn parse_expression_with_precedence(&mut self, min_bp: u8) -> ParseResult<IrNode> {
        self.skip_whitespace();

        // First, parse the prefix/primary expression
        let left = self.parse_primary_expr()?;

        // Then, loop to parse infix/postfix operators
        self.parse_postfix_and_infix(left, min_bp)
    }

    /// Parses an assignment expression.
    ///
    /// This is equivalent to `parse_expression_with_precedence(prec::COMMA.right)`,
    /// which excludes the comma operator. Use this when parsing individual items
    /// in comma-separated lists like function arguments or array elements.
    pub fn parse_assignment_expression(&mut self) -> ParseResult<IrNode> {
        self.parse_expression_with_precedence(prec::COMMA.right)
    }

    /// Parses an expression, stopping at any of the given terminators.
    ///
    /// This uses the Pratt parser with context-based termination. The terminators
    /// are pushed onto the context stack and checked in `at_terminator()` during
    /// the infix parsing loop.
    pub fn parse_expression_until(&mut self, terminators: &[SyntaxKind]) -> ParseResult<IrNode> {
        use super::{Context, ExpressionKind};
        self.with_context(
            Context::expression_from_slice(ExpressionKind::Normal, terminators),
            |parser| parser.parse_expression_with_precedence(0),
        )
    }

    /// Parses an expression until terminators, with explicit object literal context handling.
    ///
    /// The `in_object_literal` parameter explicitly indicates whether to parse in object literal
    /// context. This is used for control expressions like `{#for}` inside object literals where
    /// the context may have been modified during control block header parsing.
    ///
    /// When in object literal context, this parses a key-value property pattern (key: value)
    /// instead of treating `:` as an unexpected token.
    pub fn parse_expression_until_in_context(
        &mut self,
        terminators: &[SyntaxKind],
        in_object_literal: bool,
    ) -> ParseResult<IrNode> {
        use super::{Context, ExpressionKind};

        if in_object_literal {
            // Parse with ObjectLiteral context to properly handle `:` as property separator
            self.with_context(
                Context::expression_from_slice(ExpressionKind::ObjectLiteral, terminators),
                |parser| parser.parse_object_property_in_control_context(),
            )
        } else {
            // Normal expression parsing
            self.with_context(
                Context::expression_from_slice(ExpressionKind::Normal, terminators),
                |parser| parser.parse_expression_with_precedence(0),
            )
        }
    }

    /// Parses an object property inside a control block context.
    ///
    /// This handles patterns like `@{name}: @{value}` inside `{#for}` blocks within object literals.
    fn parse_object_property_in_control_context(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Parse the key (first part before potential `:`)
        let key = self.parse_expression_with_precedence(0)?;

        self.skip_whitespace();

        // Check if this is a key-value property (has `:`)
        if self.at(SyntaxKind::Colon) {
            self.consume(); // :
            self.skip_whitespace();

            // Parse the value - use assignment precedence to properly handle the value expression
            let value = self.parse_expression_with_precedence(prec::ASSIGN.right)?;

            self.skip_whitespace();

            // Consume trailing comma if present (common in for loops generating properties)
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }

            Ok(IrNode::KeyValueProp {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                key: Box::new(key),
                value: Box::new(value),
            })
        } else {
            // No colon - treat as shorthand property or regular expression
            // Consume trailing comma if present
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }

            // Check if it looks like a shorthand property (identifier without value)
            match &key {
                IrNode::Ident { .. } | IrNode::Placeholder { .. } => {
                    Ok(IrNode::ShorthandProp {
                        span: key.span(),
                        key: Box::new(key),
                    })
                }
                _ => Ok(key),
            }
        }
    }

    /// Parses arrow function parameters.
    ///
    /// Handles: `(a, b, c)` or `a` (single identifier without parens).
    pub(super) fn parse_arrow_params(&mut self) -> ParseResult<Vec<IrNode>> {
        self.skip_whitespace();

        if self.at(SyntaxKind::LParen) {
            self.parse_function_params()
        } else if self.at(SyntaxKind::Ident) {
            // Single identifier: x => x + 1
            let token = self.consume().expect("guarded by at() check");
            let span = token.ir_span();
            Ok(vec![IrNode::Param {
                span,
                decorators: Vec::new(),
                pat: Box::new(IrNode::BindingIdent {
                    span,
                    name: Box::new(IrNode::ident(&token)),
                    type_ann: None,
                    optional: false,
                }),
            }])
        } else {
            Err(ParseError::new(
                errors::ParseErrorKind::InvalidArrowParams,
                self.current_byte_offset(),
            ))
        }
    }

    /// Parses function parameters: (param1, param2, ...)
    pub(super) fn parse_function_params(&mut self) -> ParseResult<Vec<IrNode>> {
        let start_pos = self.pos;

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["("])
        })?;

        let mut params = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RParen) {
                break;
            }

            let param = self.parse_single_param()?;
            params.push(param);

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::RParen) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingParen,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume(); // )

        Ok(params)
    }

    /// Parses a single function parameter.
    pub(super) fn parse_single_param(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Handle decorators
        let decorators = self.parse_decorators()?;

        self.skip_whitespace();

        // Handle rest parameter: ...arg
        if self.at(SyntaxKind::DotDotDot) {
            self.consume();
            self.skip_whitespace();
            let pat = self.parse_binding_pattern()?;
            return Ok(IrNode::RestPat {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                arg: Box::new(pat),
                type_ann: None, // TODO: parse type annotation
            });
        }

        // Handle placeholder
        if self.at(SyntaxKind::At) {
            let placeholder = self.parse_interpolation()?;
            return Ok(IrNode::Param {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                decorators,
                pat: Box::new(placeholder),
            });
        }

        // Parse the pattern (identifier, destructuring, etc.)
        let pat = self.parse_binding_pattern()?;

        Ok(IrNode::Param {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            decorators,
            pat: Box::new(pat),
        })
    }

    /// Parses a binding pattern (identifier with optional type annotation and initializer).
    fn parse_binding_pattern(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "binding pattern"));
        };

        match token.kind {
            SyntaxKind::Ident => {
                let ident_token = self.consume().expect("guarded by match arm");
                let span = ident_token.ir_span();
                self.skip_whitespace();

                // Check for optional marker: ?
                let optional = if self.at(SyntaxKind::Question) {
                    self.consume();
                    self.skip_whitespace();
                    true
                } else {
                    false
                };

                // Check for type annotation: : Type
                let type_ann = if self.at(SyntaxKind::Colon) {
                    self.consume();
                    self.skip_whitespace();
                    Some(Box::new(self.parse_type()?))
                } else {
                    None
                };

                self.skip_whitespace();

                // Check for initializer: = default
                if self.at_text("=") {
                    self.consume();
                    self.skip_whitespace();
                    let init = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
                    return Ok(IrNode::AssignPat {
                        span,
                        left: Box::new(IrNode::BindingIdent {
                            span,
                            name: Box::new(IrNode::ident(&ident_token)),
                            type_ann,
                            optional,
                        }),
                        right: Box::new(init),
                    });
                }

                Ok(IrNode::BindingIdent {
                    span,
                    name: Box::new(IrNode::ident(&ident_token)),
                    type_ann,
                    optional,
                })
            }
            SyntaxKind::LBracket => self.parse_array_pattern(),
            SyntaxKind::LBrace => self.parse_object_pattern(),
            _ => Err(ParseError::new(
                errors::ParseErrorKind::InvalidFunctionParameter,
                self.current_byte_offset(),
            )
            .with_found(&token.text)),
        }
    }

    /// Parses an array destructuring pattern: [a, b, ...rest]
    fn parse_array_pattern(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBracket);

        let mut elems = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            // Handle holes: [,, x]
            if self.at(SyntaxKind::Comma) {
                elems.push(None); // Hole in array pattern
                self.consume();
                continue;
            }

            // Handle rest: [...x]
            if self.at(SyntaxKind::DotDotDot) {
                let rest_start = self.current_byte_offset();
                self.consume();
                self.skip_whitespace();
                let arg = self.parse_binding_pattern()?;
                elems.push(Some(IrNode::RestPat {
                    span: IrSpan::new(rest_start, self.current_byte_offset()),
                    arg: Box::new(arg),
                    type_ann: None,
                }));
            } else {
                let elem = self.parse_binding_pattern()?;
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
                errors::ParseErrorKind::MissingClosingBracket,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        self.skip_whitespace();

        // Optional type annotation
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(IrNode::ArrayPat {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            elems,
            type_ann,
            optional: false,
        })
    }

    /// Parses an object destructuring pattern: { a, b: c, ...rest }
    fn parse_object_pattern(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

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
                let arg = self.parse_binding_pattern()?;
                props.push(IrNode::RestPat {
                    span: IrSpan::new(rest_start, self.current_byte_offset()),
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                let prop = self.parse_binding_object_prop()?;
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
                errors::ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        self.skip_whitespace();

        // Optional type annotation
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(IrNode::ObjectPat {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            props,
            type_ann,
            optional: false,
        })
    }

    /// Parses a single property in an object pattern.
    fn parse_binding_object_prop(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "object pattern property"));
        };

        // Get the key
        let key = match token.kind {
            SyntaxKind::Ident => {
                let t = self.consume().expect("guarded by match arm");
                IrNode::ident(&t)
            }
            SyntaxKind::DoubleQuote | SyntaxKind::SingleQuote => {
                self.parse_string_literal()?
            }
            SyntaxKind::LBracket => {
                // Computed property name
                self.consume();
                self.skip_whitespace();
                let expr = self.parse_expression_with_precedence(0)?;
                self.skip_whitespace();
                self.expect(SyntaxKind::RBracket);
                IrNode::ComputedPropName {
                    span: IrSpan::empty(),
                    expr: Box::new(expr),
                }
            }
            _ if token.kind.is_ts_keyword() => {
                let t = self.consume().expect("guarded by match arm");
                IrNode::ident(&t)
            }
            _ => {
                return Err(ParseError::new(
                    errors::ParseErrorKind::InvalidPropertyName,
                    self.current_byte_offset(),
                )
                .with_found(&token.text))
            }
        };

        self.skip_whitespace();

        let key_span = key.span();

        // Check for : to rename
        if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            let value = self.parse_binding_pattern()?;
            return Ok(IrNode::ObjectPatProp {
                span: key_span,
                key: Box::new(key),
                value: Some(Box::new(value)),
            });
        }

        // Shorthand: { a } or { a = default }
        self.skip_whitespace();

        if self.at_text("=") {
            self.consume();
            self.skip_whitespace();
            let init = self.parse_expression_with_precedence(prec::ASSIGN.right)?;
            // For default values in destructuring, wrap in AssignPat
            return Ok(IrNode::ObjectPatProp {
                span: key_span,
                key: Box::new(key.clone()),
                value: Some(Box::new(IrNode::AssignPat {
                    span: key_span,
                    left: Box::new(key),
                    right: Box::new(init),
                })),
            });
        }

        // Shorthand property: { a }
        Ok(IrNode::ObjectPatProp {
            span: key_span,
            key: Box::new(key),
            value: None,
        })
    }

    /// Parses decorators: @decorator @decorator2
    fn parse_decorators(&mut self) -> ParseResult<Vec<IrNode>> {
        let mut decorators = Vec::new();

        // Handle both At (placeholder @{...}) and DecoratorAt (@name) tokens
        while self.at(SyntaxKind::At) || self.at(SyntaxKind::DecoratorAt) {
            // Check if this is a placeholder @{} or a decorator @name
            // The lexer combines "@{" into a single At token, so check the text
            if let Some(text) = self.current_text() {
                if text.starts_with("@{") {
                    // It's a placeholder, stop parsing decorators
                    break;
                }
            }

            let dec_start = self.current_byte_offset();
            self.consume(); // @ or DecoratorAt
            self.skip_whitespace();

            let expr = self.parse_expression_with_precedence(prec::CALL.left)?;
            decorators.push(IrNode::Decorator {
                span: IrSpan::new(dec_start, self.current_byte_offset()),
                expr: Box::new(expr),
            });

            self.skip_whitespace();
        }

        Ok(decorators)
    }

    /// Parses optional type parameters: <T, U>
    pub(super) fn parse_optional_type_params(&mut self) -> Option<Box<IrNode>> {
        self.skip_whitespace();

        if !self.at(SyntaxKind::Lt) {
            return None;
        }

        // Try to parse type params
        match self.parse_type_params() {
            Ok(params) => Some(Box::new(params)),
            Err(_) => None,
        }
    }

    /// Parses type parameters: <T, U extends V>
    fn parse_type_params(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::Lt).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["<"])
        })?;

        let mut params = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::Gt) {
                break;
            }

            let param = self.parse_type_param()?;
            params.push(param);

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::Gt) {
            return Err(ParseError::new(
                errors::ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_expected(&[">"]));
        }
        self.consume();

        Ok(IrNode::TypeParams {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            params,
        })
    }

    /// Parses a single type parameter: T or T extends U = Default
    fn parse_type_param(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Get the name
        let name = if self.at(SyntaxKind::Ident) {
            self.consume().expect("guarded by at() check").text
        } else if self.at(SyntaxKind::At) {
            // Placeholder as type param name
            let placeholder = self.parse_interpolation()?;
            return Ok(placeholder);
        } else {
            return Err(ParseError::new(errors::ParseErrorKind::ExpectedIdentifier, self.current_byte_offset()));
        };

        self.skip_whitespace();

        // Optional constraint: extends Type
        let constraint = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        self.skip_whitespace();

        // Optional default: = Type
        let default = if self.at_text("=") {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(IrNode::TypeParam {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            name,
            constraint,
            default,
        })
    }

    /// Parses optional type arguments: <T, U>
    pub(super) fn parse_optional_type_args(&mut self) -> ParseResult<Option<IrNode>> {
        self.skip_whitespace();

        if !self.at(SyntaxKind::Lt) {
            return Ok(None);
        }

        let type_args = self.parse_type_args()?;
        Ok(Some(type_args))
    }

    /// Parses type arguments: <T, U>
    fn parse_type_args(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        self.expect(SyntaxKind::Lt).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["<"])
        })?;

        let mut params = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::Gt) {
                break;
            }

            let ty = self.parse_type()?;
            params.push(ty);

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::Gt) {
            return Err(ParseError::new(
                errors::ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_expected(&[">"]));
        }
        self.consume();

        Ok(IrNode::TypeArgs {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            args: params,
        })
    }

    /// Parses an optional return type annotation: : Type
    pub(super) fn parse_optional_return_type(&mut self) -> ParseResult<Option<Box<IrNode>>> {
        self.skip_whitespace();

        if !self.at(SyntaxKind::Colon) {
            return Ok(None);
        }

        self.consume(); // :
        self.skip_whitespace();

        let ty = self.parse_type()?;
        Ok(Some(Box::new(ty)))
    }

    /// Parses a type annotation.
    ///
    /// This is a simplified type parser - full TypeScript type parsing is complex.
    /// For now, we handle common cases and fall back to collecting tokens for complex types.
    pub(super) fn parse_type(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        // Handle placeholder
        if self.at(SyntaxKind::At) {
            return self.parse_interpolation();
        }

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "type annotation"));
        };

        // Handle keyword types
        let base_type = match token.kind {
            SyntaxKind::TypeKw
                if token.text == "string"
                    || token.text == "number"
                    || token.text == "boolean" =>
            {
                let t = self.consume().ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "expected type keyword"))?;
                IrNode::keyword_type(&t, self.text_to_ts_keyword(&t.text)?)
            }
            SyntaxKind::Ident => {
                let t = self.consume().expect("guarded by match arm");
                let span = t.ir_span();

                // Check for common type keywords that come as identifiers
                match t.text.as_str() {
                    "string" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::String),
                    "number" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Number),
                    "boolean" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Boolean),
                    "any" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Any),
                    "unknown" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Unknown),
                    "never" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Never),
                    "object" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Object),
                    "symbol" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Symbol),
                    "bigint" => IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::BigInt),
                    _ => {
                        // Check for qualified name: NS.SubModule.Type
                        let mut current: IrNode = IrNode::ident(&t);
                        let mut current_span = span;

                        while self.at(SyntaxKind::Dot) {
                            self.consume(); // consume '.'
                            self.skip_whitespace();

                            if !self.at(SyntaxKind::Ident) {
                                return Err(ParseError::new(
                                    errors::ParseErrorKind::ExpectedTypeAnnotation,
                                    self.current_byte_offset(),
                                ).with_expected(&["identifier"]));
                            }

                            let right_token = self.consume().expect("guarded by at() check");
                            let right_span = right_token.ir_span();

                            current = IrNode::QualifiedName {
                                span: IrSpan::new(current_span.start, right_span.end),
                                left: Box::new(current),
                                right: Box::new(IrNode::ident(&right_token)),
                            };
                            current_span = IrSpan::new(current_span.start, right_span.end);
                        }

                        // Check for generic type arguments like <string, unknown>
                        let type_params = self.parse_optional_type_args()?.map(Box::new);
                        IrNode::TypeRef {
                            span: current_span,
                            name: Box::new(current),
                            type_params,
                        }
                    }
                }
            }
            SyntaxKind::VoidKw => {
                let t = self.consume().unwrap();
                IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Void)
            }
            SyntaxKind::NullKw => {
                let t = self.consume().unwrap();
                IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Null)
            }
            SyntaxKind::UndefinedKw => {
                let t = self.consume().unwrap();
                IrNode::keyword_type(&t, crate::compiler::ir::TsKeyword::Undefined)
            }
            // Function type: () => T  OR  Parenthesized type: (T)
            SyntaxKind::LParen => self.parse_paren_or_function_type()?,
            // Array type or tuple: [T, U] or T[]
            SyntaxKind::LBracket => self.parse_tuple_type()?,
            // Object type: { prop: T }
            SyntaxKind::LBrace => self.parse_object_type()?,
            // typeof type
            SyntaxKind::TypeofKw => {
                let t = self.consume().unwrap();
                self.skip_whitespace();
                let expr = self.parse_primary_expr()?;
                IrNode::TypeofType {
                    span: t.ir_span(),
                    expr: Box::new(expr),
                }
            }
            // keyof type
            SyntaxKind::KeyofKw => {
                let t = self.consume().unwrap();
                self.skip_whitespace();
                let ty = self.parse_type()?;
                IrNode::KeyofType {
                    span: t.ir_span(),
                    type_ann: Box::new(ty),
                }
            }
            // String literal type: "foo" or 'bar'
            SyntaxKind::DoubleQuote | SyntaxKind::SingleQuote => {
                let lit = self.parse_string_literal()?;
                IrNode::LiteralType {
                    span: lit.span(),
                    lit: Box::new(lit),
                }
            }
            // Number literal type: 42, 3.14
            SyntaxKind::Text if token.text.chars().next().map(|c| c.is_ascii_digit() || c == '-').unwrap_or(false) => {
                let lit = self.parse_numeric_literal()?;
                IrNode::LiteralType {
                    span: lit.span(),
                    lit: Box::new(lit),
                }
            }
            // Boolean literal type: true, false
            SyntaxKind::TrueKw | SyntaxKind::FalseKw => {
                let t = self.consume().unwrap();
                let lit = IrNode::bool_lit(&t);
                IrNode::LiteralType {
                    span: t.ir_span(),
                    lit: Box::new(lit),
                }
            }
            // Constructor type: new (params) => Type
            SyntaxKind::NewKw => {
                let start = self.consume().unwrap(); // consume 'new'
                self.skip_whitespace();

                // Parse optional type parameters
                let type_params = self.parse_optional_type_params();
                self.skip_whitespace();

                // Parse parameters
                let params = self.parse_function_params()?;
                self.skip_whitespace();

                // Expect =>
                if !self.at_text("=>") {
                    return Err(ParseError::new(
                        errors::ParseErrorKind::MissingArrowBody,
                        self.current_byte_offset(),
                    ).with_expected(&["=>"]));
                }
                self.consume();
                self.skip_whitespace();

                let return_type = self.parse_type()?;

                IrNode::ConstructorType {
                    span: IrSpan::new(start.ir_span().start, self.current_byte_offset()),
                    type_params,
                    params,
                    return_type: Box::new(return_type),
                }
            }
            // this type
            SyntaxKind::ThisKw => {
                let t = self.consume().unwrap();
                IrNode::this_type(&t)
            }
            // infer type: infer T
            SyntaxKind::InferKw => {
                let start = self.consume().unwrap(); // consume 'infer'
                self.skip_whitespace();

                // Parse the type parameter name
                let type_param = self.parse_type_param()?;

                IrNode::InferType {
                    span: IrSpan::new(start.ir_span().start, self.current_byte_offset()),
                    type_param: Box::new(type_param),
                }
            }
            // import type: import("module")
            SyntaxKind::ImportKw => {
                let start = self.consume().unwrap(); // consume 'import'
                self.skip_whitespace();

                // Expect (
                if !self.at(SyntaxKind::LParen) {
                    return Err(ParseError::new(
                        errors::ParseErrorKind::ExpectedTypeAnnotation,
                        self.current_byte_offset(),
                    ).with_expected(&["("]));
                }
                self.consume(); // consume '('
                self.skip_whitespace();

                // Parse the module string argument
                let arg = self.parse_type()?;
                self.skip_whitespace();

                // Expect )
                if !self.at(SyntaxKind::RParen) {
                    return Err(ParseError::new(
                        errors::ParseErrorKind::MissingClosingParen,
                        self.current_byte_offset(),
                    ).with_expected(&[")"]));
                }
                self.consume(); // consume ')'
                self.skip_whitespace();

                // Optional qualifier: import("module").Foo
                let qualifier = if self.at(SyntaxKind::Dot) {
                    self.consume(); // consume '.'
                    self.skip_whitespace();
                    let q = self.parse_type()?;
                    Some(Box::new(q))
                } else {
                    None
                };

                // Optional type args: import("module")<T>
                let type_args = self.parse_optional_type_args()?.map(Box::new);

                IrNode::ImportType {
                    span: IrSpan::new(start.ir_span().start, self.current_byte_offset()),
                    arg: Box::new(arg),
                    qualifier,
                    type_args,
                }
            }
            _ => {
                return Err(
                    ParseError::new(errors::ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                        .with_found(&token.text),
                )
            }
        };

        self.skip_whitespace();

        // Handle type modifiers: []  for array, | for union, & for intersection
        self.parse_type_modifiers(base_type)
    }

    /// Parses type modifiers after a base type ([], |, &, extends, [K])
    fn parse_type_modifiers(&mut self, mut ty: IrNode) -> ParseResult<IrNode> {
        loop {
            self.skip_whitespace();

            // Array type: T[] or Indexed access type: T[K]
            if self.at(SyntaxKind::LBracket) {
                self.consume(); // [
                self.skip_whitespace();

                if self.at(SyntaxKind::RBracket) {
                    // Array type: T[]
                    self.consume(); // ]
                    ty = IrNode::ArrayType {
                        span: ty.span(),
                        elem: Box::new(ty),
                    };
                    continue;
                } else {
                    // Indexed access type: T[K]
                    let index_type = self.parse_type()?;
                    self.skip_whitespace();

                    if !self.at(SyntaxKind::RBracket) {
                        return Err(ParseError::new(
                            errors::ParseErrorKind::MissingClosingBracket,
                            self.current_byte_offset(),
                        ));
                    }
                    self.consume(); // ]

                    ty = IrNode::IndexedAccessType {
                        span: ty.span().extend(IrSpan::at(self.current_byte_offset())),
                        obj: Box::new(ty),
                        index: Box::new(index_type),
                    };
                    continue;
                }
            }

            // Conditional type: T extends U ? X : Y
            if self.at(SyntaxKind::ExtendsKw) {
                self.consume(); // extends
                self.skip_whitespace();

                let extends_type = self.parse_type()?;
                self.skip_whitespace();

                // Check for ? (conditional type)
                if self.at(SyntaxKind::Question) {
                    self.consume(); // ?
                    self.skip_whitespace();

                    let true_type = self.parse_type()?;
                    self.skip_whitespace();

                    if !self.at(SyntaxKind::Colon) {
                        return Err(ParseError::new(
                            errors::ParseErrorKind::UnexpectedToken,
                            self.current_byte_offset(),
                        ).with_expected(&[":"]));
                    }
                    self.consume(); // :
                    self.skip_whitespace();

                    let false_type = self.parse_type()?;

                    ty = IrNode::ConditionalType {
                        span: ty.span().extend(false_type.span()),
                        check: Box::new(ty),
                        extends: Box::new(extends_type),
                        true_type: Box::new(true_type),
                        false_type: Box::new(false_type),
                    };
                    continue;
                }
                // If no ?, this might be a constraint - for now, error
                return Err(ParseError::new(
                    errors::ParseErrorKind::UnexpectedToken,
                    self.current_byte_offset(),
                ).with_expected(&["?"]));
            }

            // Union type: T | U
            if self.at_text("|") {
                self.consume();
                self.skip_whitespace();
                let right = self.parse_type()?;
                ty = IrNode::UnionType {
                    span: ty.span().extend(right.span()),
                    types: vec![ty, right],
                };
                continue;
            }

            // Intersection type: T & U
            if self.at(SyntaxKind::Ampersand) && self.current_text() == Some("&") {
                self.consume();
                self.skip_whitespace();
                let right = self.parse_type()?;
                ty = IrNode::IntersectionType {
                    span: ty.span().extend(right.span()),
                    types: vec![ty, right],
                };
                continue;
            }

            break;
        }

        Ok(ty)
    }

    /// Parses either a parenthesized type `(T)` or a function type `(params) => R`
    fn parse_paren_or_function_type(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LParen).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["("])
        })?;

        self.skip_whitespace();

        // Empty parens `()` must be function type
        if self.at(SyntaxKind::RParen) {
            self.consume(); // consume )
            self.skip_whitespace();

            // Expect =>
            if !self.at_text("=>") {
                return Err(ParseError::new(
                    errors::ParseErrorKind::MissingArrowBody,
                    self.current_byte_offset(),
                ).with_expected(&["=>"]));
            }
            self.consume();
            self.skip_whitespace();

            let return_type = self.parse_type()?;
            return Ok(IrNode::FnType {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                type_params: None,
                params: vec![],
                return_type: Box::new(return_type),
            });
        }

        // Check if this looks like a function parameter (has `:` or is `...`)
        // Peek ahead: if we see Ident followed by `:` or `,`, or `...`, it's function params
        let is_function_params = self.at(SyntaxKind::DotDotDot)
            || self.at(SyntaxKind::At) // placeholder as param
            || (self.at(SyntaxKind::Ident) && {
                // Look ahead for : or , after the identifier
                let mut lookahead = 1;
                while self.peek_kind(lookahead) == Some(SyntaxKind::Whitespace) {
                    lookahead += 1;
                }
                matches!(
                    self.peek_kind(lookahead),
                    Some(SyntaxKind::Colon) | Some(SyntaxKind::Comma) | Some(SyntaxKind::Question)
                )
            });

        if is_function_params {
            // Parse as function type - need to re-parse from (
            // We already consumed the (, so parse params from here
            let mut params = Vec::new();

            loop {
                self.skip_whitespace();
                if self.at(SyntaxKind::RParen) {
                    break;
                }

                let param = self.parse_single_param()?;
                params.push(param);

                self.skip_whitespace();
                if self.at(SyntaxKind::Comma) {
                    self.consume();
                } else {
                    break;
                }
            }

            if !self.at(SyntaxKind::RParen) {
                return Err(ParseError::new(
                    errors::ParseErrorKind::MissingClosingParen,
                    self.current_byte_offset(),
                ));
            }
            self.consume(); // )

            self.skip_whitespace();

            // Expect =>
            if !self.at_text("=>") {
                return Err(ParseError::new(
                    errors::ParseErrorKind::MissingArrowBody,
                    self.current_byte_offset(),
                ).with_expected(&["=>"]));
            }
            self.consume();
            self.skip_whitespace();

            let return_type = self.parse_type()?;
            return Ok(IrNode::FnType {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                type_params: None,
                params,
                return_type: Box::new(return_type),
            });
        }

        // Otherwise, try to parse as parenthesized type (T)
        let inner_type = self.parse_type()?;

        self.skip_whitespace();

        if !self.at(SyntaxKind::RParen) {
            return Err(ParseError::new(
                errors::ParseErrorKind::MissingClosingParen,
                self.current_byte_offset(),
            ));
        }
        self.consume(); // )

        self.skip_whitespace();

        // Check if followed by => (would be function type with single unnamed param)
        if self.at_text("=>") {
            // This is `(Type) => R` which in TypeScript is a function type
            // where "Type" is actually the parameter name
            self.consume();
            self.skip_whitespace();

            let return_type = self.parse_type()?;

            // Treat the inner_type as a parameter name (convert to BindingIdent)
            let param = IrNode::Param {
                span: inner_type.span(),
                decorators: vec![],
                pat: Box::new(IrNode::BindingIdent {
                    span: inner_type.span(),
                    name: Box::new(inner_type),
                    type_ann: None,
                    optional: false,
                }),
            };

            return Ok(IrNode::FnType {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                type_params: None,
                params: vec![param],
                return_type: Box::new(return_type),
            });
        }

        // It's a parenthesized type
        Ok(IrNode::ParenType {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            type_ann: Box::new(inner_type),
        })
    }

    /// Parses a function type: (a: A, b: B) => R
    fn parse_function_type(&mut self) -> ParseResult<IrNode> {
        let start_byte = self.current_byte_offset();
        let params = self.parse_function_params()?;

        self.skip_whitespace();

        // Expect =>
        if !self.at_text("=>") {
            return Err(ParseError::new(
                errors::ParseErrorKind::MissingArrowBody,
                self.current_byte_offset(),
            )
            .with_expected(&["=>"]));
        }
        self.consume();

        self.skip_whitespace();

        let return_type = self.parse_type()?;

        Ok(IrNode::FnType {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            type_params: None,
            params,
            return_type: Box::new(return_type),
        })
    }

    /// Parses a tuple type: [A, B?, ...C]
    fn parse_tuple_type(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBracket);

        let mut elems = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            let elem_start = self.current_byte_offset();

            // Check for rest element: ...T
            let is_rest = self.at(SyntaxKind::DotDotDot);
            if is_rest {
                self.consume(); // consume ...
                self.skip_whitespace();
            }

            let ty = self.parse_type()?;

            self.skip_whitespace();

            // Check for optional element: T?
            // Only valid if not a rest element
            let is_optional = !is_rest && self.at(SyntaxKind::Question);
            if is_optional {
                self.consume(); // consume ?
            }

            let elem = if is_rest {
                IrNode::RestType {
                    span: IrSpan::new(elem_start, self.current_byte_offset()),
                    type_ann: Box::new(ty),
                }
            } else if is_optional {
                IrNode::OptionalType {
                    span: IrSpan::new(elem_start, self.current_byte_offset()),
                    type_ann: Box::new(ty),
                }
            } else {
                ty
            };

            elems.push(elem);

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        if !self.at(SyntaxKind::RBracket) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingBracket,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::TupleType {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            elems,
        })
    }

    /// Parses an object type: { prop: T, method(): R } or mapped type: { [K in T]: V }
    fn parse_object_type(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBrace);
        self.skip_whitespace();

        // Check for mapped type: { [readonly] [K in T]: V }
        // Look ahead to see if this is a mapped type
        let is_mapped_type = self.is_mapped_type_start();

        if is_mapped_type {
            return self.parse_mapped_type_body(start_pos, start_byte);
        }

        let mut members = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            let member = self.parse_type_member()?;
            members.push(member);

            self.skip_whitespace();

            // Allow ; or , as separator, or neither
            if self.at(SyntaxKind::Semicolon) || self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        if !self.at(SyntaxKind::RBrace) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::ObjectType {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            members,
        })
    }

    /// Check if we're at the start of a mapped type: [readonly] [+/-readonly] [K in ...]
    fn is_mapped_type_start(&self) -> bool {
        let mut lookahead_pos = self.pos;

        // Helper to skip whitespace
        macro_rules! skip_ws {
            () => {
                while lookahead_pos < self.tokens.len()
                    && self.tokens[lookahead_pos].kind == SyntaxKind::Whitespace
                {
                    lookahead_pos += 1;
                }
            };
        }

        // Helper to get current token
        macro_rules! current {
            () => {
                if lookahead_pos < self.tokens.len() {
                    Some(&self.tokens[lookahead_pos])
                } else {
                    None
                }
            };
        }

        skip_ws!();

        // Skip optional readonly/+readonly/-readonly modifier
        if let Some(t) = current!() {
            if t.kind == SyntaxKind::ReadonlyKw || t.text == "+" || t.text == "-" {
                lookahead_pos += 1;
                skip_ws!();
                // Skip readonly after +/-
                if let Some(t2) = current!() {
                    if t2.kind == SyntaxKind::ReadonlyKw {
                        lookahead_pos += 1;
                        skip_ws!();
                    }
                }
            }
        }

        // Expect [
        if let Some(t) = current!() {
            if t.kind != SyntaxKind::LBracket {
                return false;
            }
            lookahead_pos += 1;
            skip_ws!();
        } else {
            return false;
        }

        // Skip identifier
        if let Some(t) = current!() {
            if t.kind != SyntaxKind::Ident {
                return false;
            }
            lookahead_pos += 1;
            skip_ws!();
        } else {
            return false;
        }

        // Check for "in" keyword
        if let Some(t) = current!() {
            t.kind == SyntaxKind::InKw
        } else {
            false
        }
    }

    /// Parse the body of a mapped type after the opening brace
    fn parse_mapped_type_body(&mut self, start_pos: usize, start_byte: usize) -> ParseResult<IrNode> {
        // Parse optional readonly modifier: readonly, +readonly, -readonly
        let readonly = if self.at_text("+") || self.at_text("-") {
            let sign = self.consume().unwrap();
            let is_plus = sign.text == "+";
            self.skip_whitespace();
            if self.at(SyntaxKind::ReadonlyKw) {
                self.consume();
                self.skip_whitespace();
            }
            Some(is_plus)
        } else if self.at(SyntaxKind::ReadonlyKw) {
            self.consume();
            self.skip_whitespace();
            Some(true)
        } else {
            None
        };

        // Expect [
        self.expect(SyntaxKind::LBracket);
        self.skip_whitespace();

        // Parse type parameter name: K
        let (type_param_name, name_span) = if self.at(SyntaxKind::Ident) {
            let t = self.consume().unwrap();
            let name = t.text.clone();
            let span = IrSpan::new(t.start, t.start + t.text.len());
            (name, span)
        } else {
            return Err(ParseError::new(
                errors::ParseErrorKind::ExpectedIdentifier,
                self.current_byte_offset(),
            ).with_context("mapped type parameter"));
        };
        self.skip_whitespace();

        // Expect "in"
        if !self.at(SyntaxKind::InKw) {
            return Err(ParseError::new(
                errors::ParseErrorKind::ExpectedTypeAnnotation,
                self.current_byte_offset(),
            ).with_expected(&["in"]));
        }
        self.consume();
        self.skip_whitespace();

        // Parse constraint type
        let constraint_type = self.parse_type()?;
        self.skip_whitespace();

        // Build type parameter
        let type_param = IrNode::TypeParam {
            span: name_span,
            name: type_param_name,
            constraint: Some(Box::new(constraint_type)),
            default: None,
        };

        // Parse optional "as" clause for key remapping
        let name_type = if self.at(SyntaxKind::AsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };
        self.skip_whitespace();

        // Expect ]
        self.expect(SyntaxKind::RBracket);
        self.skip_whitespace();

        // Parse optional modifier: ?, +?, -?
        let optional = if self.at_text("+") || self.at_text("-") {
            let sign = self.consume().unwrap();
            let is_plus = sign.text == "+";
            self.skip_whitespace();
            if self.at(SyntaxKind::Question) {
                self.consume();
                self.skip_whitespace();
            }
            Some(is_plus)
        } else if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            Some(true)
        } else {
            None
        };

        // Expect :
        self.expect(SyntaxKind::Colon);
        self.skip_whitespace();

        // Parse value type
        let type_ann = self.parse_type()?;
        self.skip_whitespace();

        // Allow optional semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
            self.skip_whitespace();
        }

        // Expect }
        if !self.at(SyntaxKind::RBrace) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::MappedType {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            readonly,
            type_param: Box::new(type_param),
            name_type,
            optional,
            type_ann: Some(Box::new(type_ann)),
        })
    }

    /// Parses a single type member in an object type.
    fn parse_type_member(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Handle readonly modifier
        let readonly = if self.at(SyntaxKind::ReadonlyKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        let Some(token) = self.current() else {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "type member"));
        };

        // Get the key
        let key = match token.kind {
            SyntaxKind::Ident => {
                let t = self.consume().expect("guarded by match arm");
                IrNode::ident(&t)
            }
            SyntaxKind::LBracket => {
                // Index signature: [key: string]: T
                self.consume();
                self.skip_whitespace();

                let param_token = if self.at(SyntaxKind::Ident) {
                    self.consume().expect("guarded by at() check")
                } else {
                    return Err(
                        ParseError::new(errors::ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                            .with_context("index signature"),
                    );
                };
                let param_span = param_token.ir_span();

                self.skip_whitespace();
                self.expect(SyntaxKind::Colon);
                self.skip_whitespace();

                let key_type = self.parse_type()?;

                self.skip_whitespace();
                self.expect(SyntaxKind::RBracket);

                self.skip_whitespace();
                self.expect(SyntaxKind::Colon);
                self.skip_whitespace();

                let value_type = self.parse_type()?;

                return Ok(IrNode::IndexSignature {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    readonly,
                    params: vec![IrNode::BindingIdent {
                        span: param_span,
                        name: Box::new(IrNode::ident(&param_token)),
                        type_ann: Some(Box::new(key_type)),
                        optional: false,
                    }],
                    type_ann: Box::new(value_type),
                });
            }
            _ if token.kind.is_ts_keyword() => {
                let t = self.consume().expect("guarded by match arm");
                IrNode::ident(&t)
            }
            _ => {
                return Err(
                    ParseError::new(errors::ParseErrorKind::InvalidPropertyName, self.current_byte_offset())
                        .with_found(&token.text),
                )
            }
        };

        self.skip_whitespace();

        // Check for optional: ?
        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check for method signature: ()
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            let type_params = self.parse_optional_type_params();
            let params = self.parse_function_params()?;
            self.skip_whitespace();
            let return_type = self.parse_optional_return_type()?;

            return Ok(IrNode::MethodSignature {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                name: Box::new(key),
                optional,
                type_params,
                params,
                return_type,
            });
        }

        // Property signature: : T
        self.expect(SyntaxKind::Colon);
        self.skip_whitespace();

        let type_ann = self.parse_type()?;

        Ok(IrNode::PropSignature {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            readonly,
            name: Box::new(key),
            optional,
            type_ann: Some(Box::new(type_ann)),
        })
    }

    /// Converts a string to a TypeScript keyword type.
    fn text_to_ts_keyword(&self, text: &str) -> ParseResult<crate::compiler::ir::TsKeyword> {
        match text {
            "string" => Ok(crate::compiler::ir::TsKeyword::String),
            "number" => Ok(crate::compiler::ir::TsKeyword::Number),
            "boolean" => Ok(crate::compiler::ir::TsKeyword::Boolean),
            "any" => Ok(crate::compiler::ir::TsKeyword::Any),
            "unknown" => Ok(crate::compiler::ir::TsKeyword::Unknown),
            "never" => Ok(crate::compiler::ir::TsKeyword::Never),
            "void" => Ok(crate::compiler::ir::TsKeyword::Void),
            "null" => Ok(crate::compiler::ir::TsKeyword::Null),
            "undefined" => Ok(crate::compiler::ir::TsKeyword::Undefined),
            "object" => Ok(crate::compiler::ir::TsKeyword::Object),
            "symbol" => Ok(crate::compiler::ir::TsKeyword::Symbol),
            "bigint" => Ok(crate::compiler::ir::TsKeyword::BigInt),
            _ => Err(ParseError::new(
                errors::ParseErrorKind::UnexpectedToken,
                self.current_byte_offset(),
            )
            .with_found(text)
            .with_expected(&["string", "number", "boolean", "any", "unknown", "never", "void", "null", "undefined", "object", "symbol", "bigint"])),
        }
    }

    /// Parses implements list: implements A, B, C
    pub(super) fn parse_implements_list(&mut self) -> ParseResult<Vec<IrNode>> {
        let mut implements = Vec::new();

        loop {
            self.skip_whitespace();

            let ty = self.parse_type()?;
            implements.push(ty);

            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                self.consume();
            } else {
                break;
            }
        }

        Ok(implements)
    }

    /// Parses a block statement: { stmt; stmt; }
    pub(super) fn parse_block_stmt(&mut self) -> ParseResult<IrNode> {
        let start_pos = self.pos;
        let start_byte = self.current_byte_offset();

        self.expect(SyntaxKind::LBrace).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{"])
        })?;

        // Parse statements using the proper statement list parser
        let stmts = self.parse_block_stmt_list()?;

        if !self.at(SyntaxKind::RBrace) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(IrNode::BlockStmt {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            stmts,
        })
    }

    /// Parses a list of statements inside a block until `}`
    fn parse_block_stmt_list(&mut self) -> ParseResult<Vec<IrNode>> {
        let mut stmts = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow - any {#... opening token
            if self.at_brace_hash_open() {
                let kind = self.current_kind().unwrap();
                stmts.push(self.parse_control_block(kind)?);
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
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    // For expression statements that fail to parse, we might have raw text
                    // But propagate the actual error for debugging
                    return Err(e.with_context("parsing statement in block"));
                }
            }
        }

        Ok(Self::merge_adjacent_text(stmts))
    }

    /// Parses a class body: { members }
    pub(super) fn parse_class_body(&mut self) -> ParseResult<Vec<IrNode>> {
        let start_pos = self.pos;

        self.expect(SyntaxKind::LBrace).ok_or_else(|| {
            ParseError::new(errors::ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{"])
        })?;

        let mut body = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            if self.at_eof() {
                return Err(ParseError::missing_closing(
                    errors::ParseErrorKind::MissingClosingBrace,
                    self.current_byte_offset(),
                    start_pos,
                ));
            }

            // Parse class member
            let member = self.parse_class_member()?;
            body.push(member);

            self.skip_whitespace();

            // Optional semicolon
            if self.at(SyntaxKind::Semicolon) {
                self.consume();
            }
        }

        if !self.at(SyntaxKind::RBrace) {
            return Err(ParseError::missing_closing(
                errors::ParseErrorKind::MissingClosingBrace,
                self.current_byte_offset(),
                start_pos,
            ));
        }
        self.consume();

        Ok(body)
    }

    /// Parses a single class member.
    fn parse_class_member(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();
        let start_byte = self.current_byte_offset();

        // Parse decorators (parsed but not stored in IR Method/ClassProp)
        let _decorators = self.parse_decorators()?;

        self.skip_whitespace();

        // Parse modifiers
        let mut is_static = false;
        let mut accessibility = None;
        #[allow(unused_variables)]
        let mut is_abstract = false;
        let mut is_readonly = false;
        #[allow(unused_variables)]
        let mut is_override = false;

        loop {
            self.skip_whitespace();

            if self.at(SyntaxKind::StaticKw) {
                self.consume();
                is_static = true;
            } else if self.at(SyntaxKind::PublicKw) {
                self.consume();
                accessibility = Some(crate::compiler::ir::Accessibility::Public);
            } else if self.at(SyntaxKind::PrivateKw) {
                self.consume();
                accessibility = Some(crate::compiler::ir::Accessibility::Private);
            } else if self.at(SyntaxKind::ProtectedKw) {
                self.consume();
                accessibility = Some(crate::compiler::ir::Accessibility::Protected);
            } else if self.at(SyntaxKind::AbstractKw) {
                self.consume();
                is_abstract = true;
            } else if self.at(SyntaxKind::ReadonlyKw) {
                self.consume();
                is_readonly = true;
            } else if self.at_text("override") {
                self.consume();
                is_override = true;
            } else {
                break;
            }
        }

        self.skip_whitespace();

        // Check for getter/setter
        if self.at(SyntaxKind::GetKw) || self.at(SyntaxKind::SetKw) {
            let is_getter = self.at(SyntaxKind::GetKw);
            self.consume();
            self.skip_whitespace();

            let key = self.parse_property_name()?;
            let type_params = self.parse_optional_type_params();

            if is_getter {
                self.expect(SyntaxKind::LParen);
                self.skip_whitespace();
                self.expect(SyntaxKind::RParen);
                self.skip_whitespace();

                let type_ann = self.parse_optional_return_type()?;
                self.skip_whitespace();

                let body = if self.at(SyntaxKind::LBrace) {
                    Some(Box::new(self.parse_block_stmt()?))
                } else {
                    None
                };

                return Ok(IrNode::Method {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    static_: is_static,
                    accessibility,
                    readonly: is_readonly,
                    async_: false,
                    generator: false,
                    kind: crate::compiler::ir::MethodKind::Getter,
                    name: Box::new(key),
                    optional: false,
                    type_params,
                    params: Vec::new(),
                    return_type: type_ann,
                    body,
                });
            } else {
                let params = self.parse_function_params()?;
                self.skip_whitespace();

                let body = if self.at(SyntaxKind::LBrace) {
                    Some(Box::new(self.parse_block_stmt()?))
                } else {
                    None
                };

                return Ok(IrNode::Method {
                    span: IrSpan::new(start_byte, self.current_byte_offset()),
                    static_: is_static,
                    accessibility,
                    readonly: is_readonly,
                    async_: false,
                    generator: false,
                    kind: crate::compiler::ir::MethodKind::Setter,
                    name: Box::new(key),
                    optional: false,
                    type_params,
                    params,
                    return_type: None,
                    body,
                });
            }
        }

        // Check for constructor
        if self.at_text("constructor") {
            self.consume();
            self.skip_whitespace();

            let params = self.parse_function_params()?;
            self.skip_whitespace();

            let body = if self.at(SyntaxKind::LBrace) {
                Some(Box::new(self.parse_block_stmt()?))
            } else {
                None
            };

            return Ok(IrNode::Constructor {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                accessibility,
                params,
                body,
            });
        }

        // Parse key (property/method name)
        let key = self.parse_property_name()?;

        self.skip_whitespace();

        // Check for optional: ?
        let is_optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check for method: ( or <
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
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

            return Ok(IrNode::Method {
                span: IrSpan::new(start_byte, self.current_byte_offset()),
                static_: is_static,
                accessibility,
                readonly: is_readonly,
                async_: false,
                generator: false,
                kind: crate::compiler::ir::MethodKind::Method,
                name: Box::new(key),
                optional: is_optional,
                type_params,
                params,
                return_type,
                body,
            });
        }

        // Property
        self.skip_whitespace();

        // Type annotation
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        self.skip_whitespace();

        // Initializer
        let value = if self.at_text("=") {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_expression_with_precedence(prec::ASSIGN.right)?))
        } else {
            None
        };

        Ok(IrNode::ClassProp {
            span: IrSpan::new(start_byte, self.current_byte_offset()),
            static_: is_static,
            accessibility,
            readonly: is_readonly,
            declare: false,
            optional: is_optional,
            definite: false,
            name: Box::new(key),
            type_ann,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added with the integration
}
