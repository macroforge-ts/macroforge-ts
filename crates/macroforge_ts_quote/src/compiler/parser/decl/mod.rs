mod enum_decl;
mod function;
mod import_export;
mod interface;

use super::*;

// =========================================================================
// TypeScript Declaration Parsing
// =========================================================================

impl Parser {
    pub(super) fn parse_export_decl(&mut self) -> Option<IrNode> {
        // Consume "export"
        self.consume()?;
        self.skip_whitespace();

        // Check what follows
        match self.current_kind() {
            // Declaration exports
            Some(SyntaxKind::ClassKw) => self.parse_class_decl(true),
            Some(SyntaxKind::FunctionKw) => self.parse_function_decl(true, false),
            Some(SyntaxKind::InterfaceKw) => self.parse_interface_decl(true),
            Some(SyntaxKind::ConstKw) | Some(SyntaxKind::LetKw) | Some(SyntaxKind::VarKw) => {
                self.parse_var_decl(true)
            }
            Some(SyntaxKind::AsyncKw) => self.parse_async_decl(true),
            Some(SyntaxKind::EnumKw) => self.parse_enum_decl(true, false),

            // Named export, export all, export default, export type
            Some(SyntaxKind::LBrace)
            | Some(SyntaxKind::Star)
            | Some(SyntaxKind::DefaultKw)
            | Some(SyntaxKind::TypeKw) => self.parse_export_decl_full(),

            // Fallback
            _ => Some(IrNode::Raw("export ".to_string())),
        }
    }

    pub(super) fn parse_async_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "async"
        self.consume()?;
        self.skip_whitespace();

        if self.at(SyntaxKind::FunctionKw) {
            self.parse_function_decl(exported, true)
        } else {
            // Not a function, just return raw
            let prefix = if exported { "export async " } else { "async " };
            Some(IrNode::Raw(prefix.to_string()))
        }
    }

    pub(super) fn parse_class_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "class"
        self.consume()?;
        self.skip_whitespace();

        // Parse class name (may be placeholder)
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse optional extends
        let extends = if self.at(SyntaxKind::ExtendsKw) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[
                SyntaxKind::ImplementsKw,
                SyntaxKind::LBrace,
            ])?))
        } else {
            None
        };

        // Parse optional implements
        let implements = if self.at(SyntaxKind::ImplementsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
        } else {
            vec![]
        };

        // Parse class body
        if !self.at(SyntaxKind::LBrace) {
            // No body, return raw
            return Some(IrNode::Raw("class ".to_string()));
        }
        self.consume(); // consume {
        self.skip_whitespace();

        let body = self.parse_class_body();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        // Take pending decorators
        let decorators = std::mem::take(&mut self.pending_decorators);

        Some(IrNode::ClassDecl {
            exported,
            declare: false,
            abstract_: false,
            decorators,
            name: Box::new(name),
            type_params,
            extends,
            implements,
            body,
        })
    }

    fn parse_class_body(&mut self) -> Vec<IrNode> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
                    members.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            // Check for decorators (on class members)
            if self.at(SyntaxKind::DecoratorAt) {
                if let Some(node) = self.parse_decorator_raw() {
                    members.push(node);
                }
                continue;
            }

            // Parse class member
            if let Some(member) = self.parse_class_member() {
                members.push(member);
            } else {
                // Consume unknown token
                self.advance();
            }
        }

        members
    }

    fn parse_class_member(&mut self) -> Option<IrNode> {
        self.skip_whitespace();

        // Check for doc comment
        if self.at(SyntaxKind::DocCommentPrefix) || self.at(SyntaxKind::JsDocOpen) {
            if let Some(IrNode::DocComment { text }) = self.parse_doc_comment() {
                self.pending_doc = Some(text);
            }
            return self.parse_class_member();
        }

        // Parse modifiers
        let mut static_ = false;
        let mut readonly = false;
        let mut accessibility = None;
        let mut async_ = false;

        loop {
            match self.current_kind() {
                Some(SyntaxKind::StaticKw) => {
                    static_ = true;
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ReadonlyKw) => {
                    readonly = true;
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PublicKw) => {
                    accessibility = Some(Accessibility::Public);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::PrivateKw) => {
                    accessibility = Some(Accessibility::Private);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::ProtectedKw) => {
                    accessibility = Some(Accessibility::Protected);
                    self.consume();
                    self.skip_whitespace();
                }
                Some(SyntaxKind::AsyncKw) => {
                    async_ = true;
                    self.consume();
                    self.skip_whitespace();
                }
                _ => break,
            }
        }

        // Check for constructor
        if self.current_text() == Some("constructor") {
            return self.parse_constructor(accessibility);
        }

        // Parse member name
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Check for optional marker
        let optional = if self.at(SyntaxKind::Question) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Check if this is a method (has parentheses or type params) or property
        if self.at(SyntaxKind::LParen) || self.at(SyntaxKind::Lt) {
            // Method
            let type_params = self.parse_optional_type_params();
            self.skip_whitespace();

            let params = self.parse_param_list();
            self.skip_whitespace();

            // Return type
            let return_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                self.push_context(Context::TypeAnnotation);
                let ty = self.parse_type_until(&[SyntaxKind::LBrace, SyntaxKind::Semicolon])?;
                self.pop_context();
                Some(Box::new(ty))
            } else {
                None
            };

            // Body
            let body = if self.at(SyntaxKind::LBrace) {
                Some(Box::new(self.parse_block_stmt()?))
            } else {
                self.expect(SyntaxKind::Semicolon);
                None
            };

            let node = IrNode::Method {
                static_,
                accessibility,
                readonly,
                async_,
                generator: false,
                kind: MethodKind::Method,
                name: Box::new(name),
                optional,
                type_params,
                params,
                return_type,
                body,
            };

            return self.wrap_with_doc(node);
        }

        // Property
        let type_ann = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[
                SyntaxKind::Eq,
                SyntaxKind::Semicolon,
                SyntaxKind::RBrace,
            ])?))
        } else {
            None
        };

        let value = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_ts_expr_until(&[
                SyntaxKind::Semicolon,
                SyntaxKind::RBrace,
            ])?))
        } else {
            None
        };

        // Consume optional semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let node = IrNode::ClassProp {
            static_,
            accessibility,
            readonly,
            declare: false,
            optional,
            definite: false,
            name: Box::new(name),
            type_ann,
            value,
        };

        self.wrap_with_doc(node)
    }

    fn parse_constructor(&mut self, accessibility: Option<Accessibility>) -> Option<IrNode> {
        // Consume "constructor"
        self.consume()?;
        self.skip_whitespace();

        let params = self.parse_param_list();
        self.skip_whitespace();

        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };

        let node = IrNode::Constructor {
            accessibility,
            params,
            body,
        };

        self.wrap_with_doc(node)
    }

    pub(super) fn parse_function_decl(&mut self, exported: bool, async_: bool) -> Option<IrNode> {
        // Consume "function"
        self.consume()?;
        self.skip_whitespace();

        // Check for generator (by checking for `*` text)
        let generator = if self.current_text() == Some("*") {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Parse function name
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse params
        let params = self.parse_param_list();
        self.skip_whitespace();

        // Parse return type
        let return_type = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[
                SyntaxKind::LBrace,
                SyntaxKind::Semicolon,
            ])?))
        } else {
            None
        };

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()?))
        } else {
            None
        };

        let node = IrNode::FnDecl {
            exported,
            declare: false,
            async_,
            generator,
            name: Box::new(name),
            type_params,
            params,
            return_type,
            body,
        };

        self.wrap_with_doc(node)
    }

    pub(super) fn parse_var_decl(&mut self, exported: bool) -> Option<IrNode> {
        let kind = match self.current_kind() {
            Some(SyntaxKind::ConstKw) => VarKind::Const,
            Some(SyntaxKind::LetKw) => VarKind::Let,
            Some(SyntaxKind::VarKw) => VarKind::Var,
            _ => return None,
        };
        self.consume();
        self.skip_whitespace();

        // Check for destructuring pattern (object or array)
        if self.at(SyntaxKind::LBrace) || self.at(SyntaxKind::LBracket) {
            // For destructuring patterns, collect as raw text until semicolon
            return self.parse_var_decl_as_raw(exported, &kind);
        }

        let mut decls = Vec::new();

        loop {
            // Push Identifier context for variable name
            self.push_context(Context::Identifier);
            let name = self.parse_ts_ident_or_placeholder()?;
            self.pop_context();
            self.skip_whitespace();

            // If there's a type annotation, wrap the name in a BindingIdent
            let name_with_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                let type_ann =
                    self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?;
                IrNode::BindingIdent {
                    name: Box::new(name),
                    type_ann: Some(Box::new(type_ann)),
                    optional: false,
                }
            } else {
                name
            };

            let init = if self.at(SyntaxKind::Eq) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_ts_expr_until(&[
                    SyntaxKind::Comma,
                    SyntaxKind::Semicolon,
                ])?))
            } else {
                None
            };

            decls.push(VarDeclarator {
                name: Box::new(name_with_type),
                type_ann: None, // Type annotation is now in the BindingIdent
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

        // Consume semicolon
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::VarDecl {
            exported,
            declare: false,
            kind,
            decls,
        })
    }

    /// Parse a variable declaration with destructuring pattern as raw text.
    /// This handles patterns like `const { a, b } = obj;` or `const [x, y] = arr;`
    fn parse_var_decl_as_raw(&mut self, exported: bool, kind: &VarKind) -> Option<IrNode> {
        let kind_str = match kind {
            VarKind::Const => "const ",
            VarKind::Let => "let ",
            VarKind::Var => "var ",
        };
        let export_str = if exported { "export " } else { "" };

        let mut parts = Vec::new();
        parts.push(IrNode::Raw(format!("{}{}", export_str, kind_str)));

        // Collect everything until semicolon, handling nested braces/brackets and placeholders
        let mut brace_depth = 0;
        let mut bracket_depth = 0;

        loop {
            if self.at_eof() {
                break;
            }

            match self.current_kind() {
                Some(SyntaxKind::LBrace) => {
                    brace_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::RBrace) => {
                    brace_depth -= 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::LBracket) => {
                    bracket_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::RBracket) => {
                    bracket_depth -= 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                Some(SyntaxKind::At) => {
                    if let Some(placeholder) = self.parse_interpolation() {
                        parts.push(placeholder);
                    }
                }
                Some(SyntaxKind::Semicolon) => {
                    if brace_depth == 0 && bracket_depth == 0 {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                        break;
                    } else {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                }
                _ => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
            }
        }

        // Merge adjacent raw nodes and return as TsLoopStmt (which handles raw statements)
        let merged = Self::merge_adjacent_text(parts);
        Some(IrNode::TsLoopStmt { parts: merged })
    }

    pub(super) fn parse_type_alias_decl(&mut self, exported: bool) -> Option<IrNode> {
        // Consume "type"
        self.consume()?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        if !self.at(SyntaxKind::Eq) {
            return Some(IrNode::Raw("type ".to_string()));
        }
        self.consume();
        self.skip_whitespace();

        let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon])?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        let node = IrNode::TypeAliasDecl {
            exported,
            declare: false,
            name: Box::new(name),
            type_params,
            type_ann: Box::new(type_ann),
        };

        self.wrap_with_doc(node)
    }
}
