use super::*;

impl Parser {
    pub(super) fn update_context(&mut self, kind: SyntaxKind, text: &str) {
        match kind {
            // Question mark in expression context starts ternary
            SyntaxKind::Question => {
                if self.is_expression_context() && !self.is_ternary() {
                    self.push_context(Context::expression(
                        ExpressionKind::Ternary,
                        [SyntaxKind::Colon],
                    ));
                }
            }

            // Colon: type annotation, ternary separator, or object property
            SyntaxKind::Colon => {
                // Pop identifier context first
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }

                // Pop type contexts before checking for ternary
                // This handles cases like `x ? y as T : z` where TypeAssertion might be on top
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context_kind(),
                        ContextKind::TypeAnnotation
                            | ContextKind::TypeAssertion
                            | ContextKind::GenericParams
                    )
                {
                    self.pop_context();
                }

                if self.is_ternary() {
                    // Ternary separator - pop ternary context, stay in expression
                    self.pop_context();
                } else if !self.is_object_literal() {
                    // Type annotation
                    self.push_context(Context::type_annotation([
                        SyntaxKind::Eq,
                        SyntaxKind::Comma,
                        SyntaxKind::Semicolon,
                        SyntaxKind::RParen,
                    ]));
                }
                // In object literal, `:` is property separator - no context change
            }

            // Keywords that start type context
            SyntaxKind::AsKw | SyntaxKind::SatisfiesKw => {
                self.push_context(Context::type_assertion([
                    SyntaxKind::RParen,
                    SyntaxKind::Comma,
                    SyntaxKind::Semicolon,
                    SyntaxKind::RBrace,
                ]));
            }
            SyntaxKind::KeyofKw | SyntaxKind::TypeofKw | SyntaxKind::InferKw => {
                self.push_context(Context::type_annotation([
                    SyntaxKind::Comma,
                    SyntaxKind::RParen,
                    SyntaxKind::RBrace,
                    SyntaxKind::Gt,
                ]));
            }
            SyntaxKind::ExtendsKw | SyntaxKind::ImplementsKw => {
                self.push_context(Context::type_annotation([
                    SyntaxKind::LBrace,
                    SyntaxKind::Comma,
                    SyntaxKind::ImplementsKw,
                ]));
            }

            // Keywords that start identifier context
            SyntaxKind::FunctionKw
            | SyntaxKind::ClassKw
            | SyntaxKind::InterfaceKw
            | SyntaxKind::TypeKw
            | SyntaxKind::ConstKw
            | SyntaxKind::LetKw
            | SyntaxKind::VarKw => {
                self.push_context(Context::identifier([
                    SyntaxKind::LParen,
                    SyntaxKind::Lt,
                    SyntaxKind::Colon,
                    SyntaxKind::Eq,
                ]));
            }

            // Keywords that start expression context
            SyntaxKind::ReturnKw
            | SyntaxKind::ThrowKw
            | SyntaxKind::YieldKw
            | SyntaxKind::AwaitKw
            | SyntaxKind::NewKw => {
                self.push_context(Context::expression(
                    ExpressionKind::Normal,
                    [SyntaxKind::Semicolon],
                ));
            }

            // Dot starts identifier context (member access)
            SyntaxKind::Dot => {
                self.push_context(Context::identifier([
                    SyntaxKind::LParen,
                    SyntaxKind::LBracket,
                    SyntaxKind::Dot,
                    SyntaxKind::Semicolon,
                ]));
            }

            // Regular identifier consumes identifier context
            SyntaxKind::Ident => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
            }

            // Keywords used as property names also consume identifier context
            // This handles cases like `obj.is(...)` where `is` is a keyword
            _ if kind.is_ts_keyword() => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
            }

            // Placeholder (@{...}) also consumes identifier context (acts like an identifier)
            SyntaxKind::At => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
            }

            // Opening paren ends identifier context
            SyntaxKind::LParen => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
            }

            // Less-than might end identifier context (generics)
            SyntaxKind::Lt => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
                // Could push GenericParams context here if needed
            }

            // Equals ends type annotation and identifier, starts expression
            SyntaxKind::Eq => {
                if self.current_context_kind() == ContextKind::Identifier {
                    self.pop_context();
                }
                if self.current_context_kind() == ContextKind::TypeAnnotation {
                    self.pop_context();
                }
                self.push_context(Context::expression(
                    ExpressionKind::Normal,
                    [SyntaxKind::Semicolon, SyntaxKind::Comma],
                ));
            }

            // Semicolon ends expression and type contexts (but keep base context)
            SyntaxKind::Semicolon => {
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context_kind(),
                        ContextKind::Expression(_)
                            | ContextKind::TypeAnnotation
                            | ContextKind::TypeAssertion
                            | ContextKind::GenericParams
                    )
                {
                    self.pop_context();
                }
            }

            // Closing brace ends type contexts
            // BUT only for actual closing braces, not interpolation content (which has text like "expr}")
            SyntaxKind::RBrace if text == "}" || text == "}}" => {
                // Pop object literal context if we're in one
                if self.is_object_literal() {
                    self.pop_context();
                }
                // Pop any remaining type contexts (but keep base context)
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context_kind(),
                        ContextKind::TypeAnnotation
                            | ContextKind::TypeAssertion
                            | ContextKind::GenericParams
                    )
                {
                    self.pop_context();
                }
            }

            // Comma might end type context
            SyntaxKind::Comma => {
                if matches!(
                    self.current_context_kind(),
                    ContextKind::TypeAnnotation | ContextKind::TypeAssertion
                ) {
                    self.pop_context();
                }
            }

            // Opening brace in expression context starts object literal
            SyntaxKind::LBrace => {
                if self.is_expression_context() {
                    self.push_context(Context::expression(
                        ExpressionKind::ObjectLiteral,
                        [SyntaxKind::RBrace],
                    ));
                }
            }

            _ => {}
        }
    }

    // =========================================================================
    // Context management
    // =========================================================================

    /// Returns the kind of the current context (for pattern matching/comparison).
    ///
    /// # Panics
    ///
    /// Panics if the context stack is empty. This should never happen in normal
    /// operation since the parser is always initialized with a base context and
    /// `pop_context` prevents popping the last element.
    pub(super) fn current_context_kind(&self) -> ContextKind {
        self.context_stack
            .last()
            .map(|c| c.kind)
            .expect("context stack is empty - this is a parser bug: the stack should always have at least one context")
    }

    pub(super) fn push_context(&mut self, ctx: Context) {
        self.context_stack.push(ctx);
    }

    pub(super) fn pop_context(&mut self) {
        if self.context_stack.len() > 1 {
            self.context_stack.pop();
        }
    }

    pub(super) fn is_expression_context(&self) -> bool {
        matches!(self.current_context_kind(), ContextKind::Expression(_))
    }

    pub(super) fn is_ternary(&self) -> bool {
        matches!(
            self.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Ternary)
        )
    }

    /// Returns true if the current (top) context is an object literal.
    /// Used by `update_context` to know when to pop on `}`.
    pub(super) fn is_object_literal(&self) -> bool {
        matches!(
            self.current_context_kind(),
            ContextKind::Expression(ExpressionKind::ObjectLiteral)
        )
    }

    /// Returns true if we're anywhere inside an object literal context.
    /// Searches the entire stack, so nested contexts (like template control blocks)
    /// can detect they're inside an object literal.
    pub(super) fn is_inside_object_literal(&self) -> bool {
        self.context_stack.iter().any(|ctx| {
            matches!(ctx.kind, ContextKind::Expression(ExpressionKind::ObjectLiteral))
        })
    }

    /// Returns true if we're anywhere inside an interface member context.
    /// Searches the entire stack, so nested control blocks can detect they're inside an interface.
    pub(super) fn is_inside_interface(&self) -> bool {
        self.context_stack
            .iter()
            .any(|ctx| matches!(ctx.kind, ContextKind::InterfaceMember))
    }

    /// Check if the current token is a terminator for expression parsing.
    /// Checks control flow markers first, then context-specific terminators.
    pub(super) fn at_terminator(&self) -> bool {
        let Some(token) = self.current() else {
            return false;
        };

        // Control flow markers always terminate (template-language constructs)
        if CONTROL_FLOW_TERMINATORS.contains(&token.kind) {
            return true;
        }

        // Check context-specific terminators
        if let Some(ctx) = self.context_stack.last() {
            return ctx.terminators.contains(&token.kind);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ContextKind Enum Tests ====================

    #[test]
    fn test_context_kind_equality() {
        assert_eq!(ContextKind::Statement, ContextKind::Statement);
        assert_eq!(
            ContextKind::Expression(ExpressionKind::Normal),
            ContextKind::Expression(ExpressionKind::Normal)
        );
        assert_ne!(ContextKind::Statement, ContextKind::Identifier);
    }

    #[test]
    fn test_context_kind_clone() {
        let ctx = ContextKind::TypeAnnotation;
        let cloned = ctx;
        assert_eq!(ctx, cloned);
    }

    #[test]
    fn test_context_kind_debug() {
        let ctx = ContextKind::Statement;
        let debug_str = format!("{:?}", ctx);
        assert!(debug_str.contains("Statement"));
    }

    // ==================== Context Struct Tests ====================

    #[test]
    fn test_context_constructors() {
        let expr = Context::expression(ExpressionKind::Normal, [SyntaxKind::Semicolon]);
        assert_eq!(expr.kind, ContextKind::Expression(ExpressionKind::Normal));
        assert!(expr.terminators.contains(&SyntaxKind::Semicolon));

        let type_ann = Context::type_annotation([SyntaxKind::Eq, SyntaxKind::Comma]);
        assert_eq!(type_ann.kind, ContextKind::TypeAnnotation);
        assert_eq!(type_ann.terminators.len(), 2);

        let ident = Context::identifier([]);
        assert_eq!(ident.kind, ContextKind::Identifier);
        assert!(ident.terminators.is_empty());
    }

    // ==================== ExpressionKind Tests ====================

    #[test]
    fn test_expression_kind_default() {
        let kind: ExpressionKind = Default::default();
        assert_eq!(kind, ExpressionKind::Normal);
    }

    #[test]
    fn test_expression_kind_variants() {
        assert_eq!(ExpressionKind::Normal, ExpressionKind::Normal);
        assert_ne!(ExpressionKind::Normal, ExpressionKind::Ternary);
        assert_ne!(ExpressionKind::Ternary, ExpressionKind::ObjectLiteral);
    }

    // ==================== Parser Context Management Tests ====================

    #[test]
    fn test_parser_initial_context() {
        let parser = Parser::new("");
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_push_pop_context() {
        let mut parser = Parser::new("");
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );

        parser.push_context(Context::type_annotation([]));
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);

        parser.pop_context();
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_push_multiple_contexts() {
        let mut parser = Parser::new("");

        parser.push_context(Context::type_annotation([]));
        parser.push_context(Context::generic_params([]));
        parser.push_context(Context::identifier([]));

        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);

        parser.pop_context();
        assert_eq!(parser.current_context_kind(), ContextKind::GenericParams);

        parser.pop_context();
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);

        parser.pop_context();
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_pop_preserves_base() {
        let mut parser = Parser::new("");

        // Pop on initial state should preserve base context
        parser.pop_context();
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );

        // Multiple pops should still preserve base
        parser.pop_context();
        parser.pop_context();
        parser.pop_context();
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    // ==================== Context Query Tests ====================

    #[test]
    fn test_is_expression_context_normal() {
        let parser = Parser::new("");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_is_expression_context_ternary() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::Ternary, []));
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_is_expression_context_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::ObjectLiteral, []));
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_is_expression_context_false() {
        let mut parser = Parser::new("");
        parser.push_context(Context::type_annotation([]));
        assert!(!parser.is_expression_context());

        parser.pop_context();
        parser.push_context(Context::statement());
        assert!(!parser.is_expression_context());

        parser.pop_context();
        parser.push_context(Context::identifier([]));
        assert!(!parser.is_expression_context());
    }

    #[test]
    fn test_is_ternary() {
        let mut parser = Parser::new("");
        assert!(!parser.is_ternary());

        parser.push_context(Context::expression(ExpressionKind::Ternary, []));
        assert!(parser.is_ternary());

        parser.pop_context();
        assert!(!parser.is_ternary());
    }

    #[test]
    fn test_is_ternary_not_normal() {
        let parser = Parser::new("");
        assert!(!parser.is_ternary());
    }

    #[test]
    fn test_is_object_literal() {
        let mut parser = Parser::new("");
        assert!(!parser.is_object_literal());

        parser.push_context(Context::expression(ExpressionKind::ObjectLiteral, []));
        assert!(parser.is_object_literal());

        parser.pop_context();
        assert!(!parser.is_object_literal());
    }

    // ==================== update_context Tests ====================

    #[test]
    fn test_update_context_question_starts_ternary() {
        let mut parser = Parser::new("");
        // In expression context, ? should start ternary
        parser.update_context(SyntaxKind::Question, "?");
        assert!(parser.is_ternary());
    }

    #[test]
    fn test_update_context_question_no_double_ternary() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::Ternary, []));
        let stack_len_before = parser.context_stack.len();

        // Already in ternary, should not push another
        parser.update_context(SyntaxKind::Question, "?");
        assert_eq!(parser.context_stack.len(), stack_len_before);
    }

    #[test]
    fn test_update_context_colon_in_ternary() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::Ternary, []));
        assert!(parser.is_ternary());

        // : in ternary should pop ternary context
        parser.update_context(SyntaxKind::Colon, ":");
        assert!(!parser.is_ternary());
    }

    #[test]
    fn test_update_context_colon_type_annotation() {
        let mut parser = Parser::new("");
        // Push identifier context to simulate `const x`
        parser.push_context(Context::identifier([]));

        // : should pop identifier and push type annotation
        parser.update_context(SyntaxKind::Colon, ":");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_colon_in_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::ObjectLiteral, []));
        let stack_len_before = parser.context_stack.len();

        // : in object literal should not push type annotation
        parser.update_context(SyntaxKind::Colon, ":");
        // Stack should remain the same (no type annotation pushed)
        assert_eq!(parser.context_stack.len(), stack_len_before);
    }

    #[test]
    fn test_update_context_as_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::AsKw, "as");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAssertion);
    }

    #[test]
    fn test_update_context_satisfies_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::SatisfiesKw, "satisfies");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAssertion);
    }

    #[test]
    fn test_update_context_keyof_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::KeyofKw, "keyof");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_typeof_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::TypeofKw, "typeof");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_function_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::FunctionKw, "function");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_class_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ClassKw, "class");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_const_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ConstKw, "const");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_let_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::LetKw, "let");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_var_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::VarKw, "var");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_return_keyword() {
        let mut parser = Parser::new("");
        parser.push_context(Context::statement());
        parser.update_context(SyntaxKind::ReturnKw, "return");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_throw_keyword() {
        let mut parser = Parser::new("");
        parser.push_context(Context::statement());
        parser.update_context(SyntaxKind::ThrowKw, "throw");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_dot_starts_identifier() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::Dot, ".");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_ident_consumes_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::identifier([]));
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);

        parser.update_context(SyntaxKind::Ident, "foo");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_lparen_ends_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::identifier([]));
        parser.update_context(SyntaxKind::LParen, "(");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_lt_ends_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::identifier([]));
        parser.update_context(SyntaxKind::Lt, "<");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_eq_pops_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::identifier([]));
        parser.push_context(Context::type_annotation([]));

        parser.update_context(SyntaxKind::Eq, "=");

        // Should have popped identifier and type, pushed expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_semicolon_ends_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::Normal, []));
        parser.push_context(Context::type_annotation([]));
        parser.push_context(Context::type_assertion([]));

        parser.update_context(SyntaxKind::Semicolon, ";");

        // Should pop all expression/type contexts but preserve base
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_update_context_rbrace_pops_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::expression(ExpressionKind::ObjectLiteral, []));
        assert!(parser.is_object_literal());

        parser.update_context(SyntaxKind::RBrace, "}");
        assert!(!parser.is_object_literal());
    }

    #[test]
    fn test_update_context_rbrace_pops_type_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::type_annotation([]));
        parser.push_context(Context::generic_params([]));

        parser.update_context(SyntaxKind::RBrace, "}");

        // Should pop type contexts
        assert_eq!(
            parser.current_context_kind(),
            ContextKind::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_update_context_comma_pops_type() {
        let mut parser = Parser::new("");
        parser.push_context(Context::type_annotation([]));

        parser.update_context(SyntaxKind::Comma, ",");

        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_lbrace_starts_object_literal() {
        let mut parser = Parser::new("");
        // In expression context
        assert!(parser.is_expression_context());

        parser.update_context(SyntaxKind::LBrace, "{");

        assert!(parser.is_object_literal());
    }

    #[test]
    fn test_update_context_lbrace_no_object_in_type() {
        let mut parser = Parser::new("");
        parser.push_context(Context::type_annotation([]));

        parser.update_context(SyntaxKind::LBrace, "{");

        // Should not push object literal in type context
        assert!(!parser.is_object_literal());
    }

    #[test]
    fn test_update_context_extends_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ExtendsKw, "extends");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_implements_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ImplementsKw, "implements");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_infer_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::InferKw, "infer");
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);
    }

    #[test]
    fn test_update_context_interface_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::InterfaceKw, "interface");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_type_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::TypeKw, "type");
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);
    }

    #[test]
    fn test_update_context_yield_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::YieldKw, "yield");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_await_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::AwaitKw, "await");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_new_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::NewKw, "new");
        assert!(parser.is_expression_context());
    }

    // ==================== Complex Scenario Tests ====================

    #[test]
    fn test_complex_type_annotation_flow() {
        let mut parser = Parser::new("");

        // Simulate: const x: T = value
        parser.update_context(SyntaxKind::ConstKw, "const"); // -> Identifier
        assert_eq!(parser.current_context_kind(), ContextKind::Identifier);

        parser.update_context(SyntaxKind::Ident, "x"); // -> pops Identifier
        assert!(parser.is_expression_context());

        parser.update_context(SyntaxKind::Colon, ":"); // -> TypeAnnotation
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);

        parser.update_context(SyntaxKind::Ident, "T"); // -> stays in TypeAnnotation
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAnnotation);

        parser.update_context(SyntaxKind::Eq, "="); // -> pops TypeAnnotation, pushes Expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_complex_ternary_flow() {
        let mut parser = Parser::new("");

        // Simulate: cond ? a : b
        parser.update_context(SyntaxKind::Question, "?"); // -> Ternary
        assert!(parser.is_ternary());

        parser.update_context(SyntaxKind::Colon, ":"); // -> pops Ternary
        assert!(!parser.is_ternary());
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_complex_type_assertion_in_ternary() {
        let mut parser = Parser::new("");

        // Simulate: cond ? x as T : y
        parser.update_context(SyntaxKind::Question, "?"); // -> Ternary
        assert!(parser.is_ternary());

        parser.update_context(SyntaxKind::AsKw, "as"); // -> TypeAssertion on top of Ternary
        assert_eq!(parser.current_context_kind(), ContextKind::TypeAssertion);

        // When we hit :, it should pop TypeAssertion and then pop Ternary
        parser.update_context(SyntaxKind::Colon, ":");
        assert!(!parser.is_ternary());
        assert!(parser.is_expression_context());
    }
}
