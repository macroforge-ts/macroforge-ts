use super::*;

impl Parser {
    pub(super) fn update_context(&mut self, kind: SyntaxKind, text: &str) {
        match kind {
            // Question mark in expression context starts ternary
            SyntaxKind::Question => {
                if self.is_expression_context() && !self.is_ternary() {
                    self.push_context(Context::Expression(ExpressionKind::Ternary));
                }
            }

            // Colon: type annotation, ternary separator, or object property
            SyntaxKind::Colon => {
                // Pop identifier context first
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }

                // Pop type contexts before checking for ternary
                // This handles cases like `x ? y as T : z` where TypeAssertion might be on top
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context(),
                        Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                    )
                {
                    self.pop_context();
                }

                if self.is_ternary() {
                    // Ternary separator - pop ternary context, stay in expression
                    self.pop_context();
                } else if !self.is_object_literal() {
                    // Type annotation
                    self.push_context(Context::TypeAnnotation);
                }
                // In object literal, `:` is property separator - no context change
            }

            // Keywords that start type context
            SyntaxKind::AsKw | SyntaxKind::SatisfiesKw => {
                self.push_context(Context::TypeAssertion);
            }
            SyntaxKind::KeyofKw | SyntaxKind::TypeofKw | SyntaxKind::InferKw => {
                self.push_context(Context::TypeAnnotation);
            }
            SyntaxKind::ExtendsKw | SyntaxKind::ImplementsKw => {
                self.push_context(Context::TypeAnnotation);
            }

            // Keywords that start identifier context
            SyntaxKind::FunctionKw
            | SyntaxKind::ClassKw
            | SyntaxKind::InterfaceKw
            | SyntaxKind::TypeKw
            | SyntaxKind::ConstKw
            | SyntaxKind::LetKw
            | SyntaxKind::VarKw => {
                self.push_context(Context::Identifier);
            }

            // Keywords that start expression context
            SyntaxKind::ReturnKw
            | SyntaxKind::ThrowKw
            | SyntaxKind::YieldKw
            | SyntaxKind::AwaitKw
            | SyntaxKind::NewKw => {
                self.push_context(Context::Expression(ExpressionKind::Normal));
            }

            // Dot starts identifier context (member access)
            SyntaxKind::Dot => {
                self.push_context(Context::Identifier);
            }

            // Regular identifier consumes identifier context
            SyntaxKind::Ident => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // Opening paren ends identifier context
            SyntaxKind::LParen => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // Less-than might end identifier context (generics)
            SyntaxKind::Lt => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                // Could push GenericParams context here if needed
            }

            // Equals ends type annotation and identifier, starts expression
            SyntaxKind::Eq => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                if self.current_context() == Context::TypeAnnotation {
                    self.pop_context();
                }
                self.push_context(Context::Expression(ExpressionKind::Normal));
            }

            // Semicolon ends expression and type contexts (but keep base context)
            SyntaxKind::Semicolon => {
                while self.context_stack.len() > 1
                    && matches!(
                        self.current_context(),
                        Context::Expression(_)
                            | Context::TypeAnnotation
                            | Context::TypeAssertion
                            | Context::GenericParams
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
                        self.current_context(),
                        Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                    )
                {
                    self.pop_context();
                }
            }

            // Comma might end type context
            SyntaxKind::Comma => {
                if matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion
                ) {
                    self.pop_context();
                }
            }

            // Opening brace in expression context starts object literal
            SyntaxKind::LBrace => {
                if self.is_expression_context() {
                    self.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
                }
            }

            _ => {}
        }
    }

    // =========================================================================
    // Context management
    // =========================================================================

    pub(super) fn current_context(&self) -> Context {
        *self.context_stack.last().unwrap_or(&Context::Statement)
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
        matches!(self.current_context(), Context::Expression(_))
    }

    pub(super) fn is_ternary(&self) -> bool {
        matches!(
            self.current_context(),
            Context::Expression(ExpressionKind::Ternary)
        )
    }

    pub(super) fn is_object_literal(&self) -> bool {
        matches!(
            self.current_context(),
            Context::Expression(ExpressionKind::ObjectLiteral)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Context Enum Tests ====================

    #[test]
    fn test_context_equality() {
        assert_eq!(Context::Statement, Context::Statement);
        assert_eq!(
            Context::Expression(ExpressionKind::Normal),
            Context::Expression(ExpressionKind::Normal)
        );
        assert_ne!(Context::Statement, Context::Identifier);
    }

    #[test]
    fn test_context_clone() {
        let ctx = Context::TypeAnnotation;
        let cloned = ctx;
        assert_eq!(ctx, cloned);
    }

    #[test]
    fn test_context_debug() {
        let ctx = Context::Statement;
        let debug_str = format!("{:?}", ctx);
        assert!(debug_str.contains("Statement"));
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
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_push_pop_context() {
        let mut parser = Parser::new("");
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );

        parser.push_context(Context::TypeAnnotation);
        assert_eq!(parser.current_context(), Context::TypeAnnotation);

        parser.pop_context();
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_push_multiple_contexts() {
        let mut parser = Parser::new("");

        parser.push_context(Context::TypeAnnotation);
        parser.push_context(Context::GenericParams);
        parser.push_context(Context::Identifier);

        assert_eq!(parser.current_context(), Context::Identifier);

        parser.pop_context();
        assert_eq!(parser.current_context(), Context::GenericParams);

        parser.pop_context();
        assert_eq!(parser.current_context(), Context::TypeAnnotation);

        parser.pop_context();
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_parser_pop_preserves_base() {
        let mut parser = Parser::new("");

        // Pop on initial state should preserve base context
        parser.pop_context();
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );

        // Multiple pops should still preserve base
        parser.pop_context();
        parser.pop_context();
        parser.pop_context();
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
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
        parser.push_context(Context::Expression(ExpressionKind::Ternary));
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_is_expression_context_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_is_expression_context_false() {
        let mut parser = Parser::new("");
        parser.push_context(Context::TypeAnnotation);
        assert!(!parser.is_expression_context());

        parser.pop_context();
        parser.push_context(Context::Statement);
        assert!(!parser.is_expression_context());

        parser.pop_context();
        parser.push_context(Context::Identifier);
        assert!(!parser.is_expression_context());
    }

    #[test]
    fn test_is_ternary() {
        let mut parser = Parser::new("");
        assert!(!parser.is_ternary());

        parser.push_context(Context::Expression(ExpressionKind::Ternary));
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

        parser.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
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
        parser.push_context(Context::Expression(ExpressionKind::Ternary));
        let stack_len_before = parser.context_stack.len();

        // Already in ternary, should not push another
        parser.update_context(SyntaxKind::Question, "?");
        assert_eq!(parser.context_stack.len(), stack_len_before);
    }

    #[test]
    fn test_update_context_colon_in_ternary() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Expression(ExpressionKind::Ternary));
        assert!(parser.is_ternary());

        // : in ternary should pop ternary context
        parser.update_context(SyntaxKind::Colon, ":");
        assert!(!parser.is_ternary());
    }

    #[test]
    fn test_update_context_colon_type_annotation() {
        let mut parser = Parser::new("");
        // Push identifier context to simulate `const x`
        parser.push_context(Context::Identifier);

        // : should pop identifier and push type annotation
        parser.update_context(SyntaxKind::Colon, ":");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_colon_in_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
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
        assert_eq!(parser.current_context(), Context::TypeAssertion);
    }

    #[test]
    fn test_update_context_satisfies_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::SatisfiesKw, "satisfies");
        assert_eq!(parser.current_context(), Context::TypeAssertion);
    }

    #[test]
    fn test_update_context_keyof_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::KeyofKw, "keyof");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_typeof_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::TypeofKw, "typeof");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_function_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::FunctionKw, "function");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_class_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ClassKw, "class");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_const_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ConstKw, "const");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_let_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::LetKw, "let");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_var_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::VarKw, "var");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_return_keyword() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Statement);
        parser.update_context(SyntaxKind::ReturnKw, "return");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_throw_keyword() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Statement);
        parser.update_context(SyntaxKind::ThrowKw, "throw");
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_dot_starts_identifier() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::Dot, ".");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_ident_consumes_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Identifier);
        assert_eq!(parser.current_context(), Context::Identifier);

        parser.update_context(SyntaxKind::Ident, "foo");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_lparen_ends_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Identifier);
        parser.update_context(SyntaxKind::LParen, "(");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_lt_ends_identifier() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Identifier);
        parser.update_context(SyntaxKind::Lt, "<");
        // Should pop back to expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_eq_pops_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Identifier);
        parser.push_context(Context::TypeAnnotation);

        parser.update_context(SyntaxKind::Eq, "=");

        // Should have popped identifier and type, pushed expression
        assert!(parser.is_expression_context());
    }

    #[test]
    fn test_update_context_semicolon_ends_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Expression(ExpressionKind::Normal));
        parser.push_context(Context::TypeAnnotation);
        parser.push_context(Context::TypeAssertion);

        parser.update_context(SyntaxKind::Semicolon, ";");

        // Should pop all expression/type contexts but preserve base
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_update_context_rbrace_pops_object_literal() {
        let mut parser = Parser::new("");
        parser.push_context(Context::Expression(ExpressionKind::ObjectLiteral));
        assert!(parser.is_object_literal());

        parser.update_context(SyntaxKind::RBrace, "}");
        assert!(!parser.is_object_literal());
    }

    #[test]
    fn test_update_context_rbrace_pops_type_contexts() {
        let mut parser = Parser::new("");
        parser.push_context(Context::TypeAnnotation);
        parser.push_context(Context::GenericParams);

        parser.update_context(SyntaxKind::RBrace, "}");

        // Should pop type contexts
        assert_eq!(
            parser.current_context(),
            Context::Expression(ExpressionKind::Normal)
        );
    }

    #[test]
    fn test_update_context_comma_pops_type() {
        let mut parser = Parser::new("");
        parser.push_context(Context::TypeAnnotation);

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
        parser.push_context(Context::TypeAnnotation);

        parser.update_context(SyntaxKind::LBrace, "{");

        // Should not push object literal in type context
        assert!(!parser.is_object_literal());
    }

    #[test]
    fn test_update_context_extends_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ExtendsKw, "extends");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_implements_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::ImplementsKw, "implements");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_infer_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::InferKw, "infer");
        assert_eq!(parser.current_context(), Context::TypeAnnotation);
    }

    #[test]
    fn test_update_context_interface_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::InterfaceKw, "interface");
        assert_eq!(parser.current_context(), Context::Identifier);
    }

    #[test]
    fn test_update_context_type_keyword() {
        let mut parser = Parser::new("");
        parser.update_context(SyntaxKind::TypeKw, "type");
        assert_eq!(parser.current_context(), Context::Identifier);
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
        assert_eq!(parser.current_context(), Context::Identifier);

        parser.update_context(SyntaxKind::Ident, "x"); // -> pops Identifier
        assert!(parser.is_expression_context());

        parser.update_context(SyntaxKind::Colon, ":"); // -> TypeAnnotation
        assert_eq!(parser.current_context(), Context::TypeAnnotation);

        parser.update_context(SyntaxKind::Ident, "T"); // -> stays in TypeAnnotation
        assert_eq!(parser.current_context(), Context::TypeAnnotation);

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
        assert_eq!(parser.current_context(), Context::TypeAssertion);

        // When we hit :, it should pop TypeAssertion and then pop Ternary
        parser.update_context(SyntaxKind::Colon, ":");
        assert!(!parser.is_ternary());
        assert!(parser.is_expression_context());
    }
}
