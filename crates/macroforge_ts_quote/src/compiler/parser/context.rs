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
