mod enum_decl;
mod function;
mod import_export;
mod interface;

use super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::*;

// =========================================================================
// TypeScript Declaration Parsing
// =========================================================================

impl Parser {
    pub(super) fn parse_export_decl(&mut self) -> ParseResult<IrNode> {
        // Consume "export"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "export keyword"))?;
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
            _ => Ok(IrNode::Raw("export ".to_string())),
        }
    }

    pub(super) fn parse_async_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        // Consume "async"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "async keyword"))?;
        self.skip_whitespace();

        if self.at(SyntaxKind::FunctionKw) {
            self.parse_function_decl(exported, true)
        } else {
            // Not a function, just return raw
            let prefix = if exported { "export async " } else { "async " };
            Ok(IrNode::Raw(prefix.to_string()))
        }
    }

    pub(super) fn parse_class_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        // Consume "class"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "class keyword"))?;
        self.skip_whitespace();

        // Parse class name (may be placeholder)
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("class declaration name"))?;
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
            ]).map_err(|e| e.with_context("class extends clause"))?))
        } else {
            None
        };

        // Parse optional implements
        let implements = if self.at(SyntaxKind::ImplementsKw) {
            self.consume();
            self.skip_whitespace();
            self.parse_type_list_until(SyntaxKind::LBrace)
                .map_err(|e| e.with_context("class implements clause"))?
        } else {
            vec![]
        };

        // Parse class body - use the new parse_class_body from expr/mod.rs
        if !self.at(SyntaxKind::LBrace) {
            // No body, return raw
            return Ok(IrNode::Raw("class ".to_string()));
        }

        let body = self.parse_class_body()
            .map_err(|e| e.with_context("class body"))?;

        // Take pending decorators
        let decorators = std::mem::take(&mut self.pending_decorators);

        Ok(IrNode::ClassDecl {
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

    // parse_class_body and parse_class_member are now in expr/mod.rs with proper error handling

    fn parse_constructor(&mut self, accessibility: Option<Accessibility>) -> ParseResult<IrNode> {
        // Consume "constructor"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "constructor keyword"))?;
        self.skip_whitespace();

        let params = self.parse_param_list()
            .map_err(|e| e.with_context("constructor parameters"))?;
        self.skip_whitespace();

        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()
                .map_err(|e| e.with_context("constructor body"))?))
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

    pub(super) fn parse_function_decl(&mut self, exported: bool, async_: bool) -> ParseResult<IrNode> {
        // Consume "function"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "function keyword"))?;
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
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("function declaration name"))?;
        self.skip_whitespace();

        // Parse optional type params
        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        // Parse params
        let params = self.parse_param_list()
            .map_err(|e| e.with_context("function parameters"))?;
        self.skip_whitespace();

        // Parse return type
        let return_type = if self.at(SyntaxKind::Colon) {
            self.consume();
            self.skip_whitespace();
            Some(Box::new(self.parse_type_until(&[
                SyntaxKind::LBrace,
                SyntaxKind::Semicolon,
            ]).map_err(|e| e.with_context("function return type"))?
                .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                    .with_context("function return type"))?))
        } else {
            None
        };

        // Parse body
        let body = if self.at(SyntaxKind::LBrace) {
            Some(Box::new(self.parse_block_stmt()
                .map_err(|e| e.with_context("function body"))?))
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

    pub(super) fn parse_var_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        let kind = match self.current_kind() {
            Some(SyntaxKind::ConstKw) => VarKind::Const,
            Some(SyntaxKind::LetKw) => VarKind::Let,
            Some(SyntaxKind::VarKw) => VarKind::Var,
            _ => return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("variable declaration")
                .with_expected(&["const", "let", "var"])),
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
            self.push_context(Context::identifier([
                SyntaxKind::Colon,
                SyntaxKind::Eq,
                SyntaxKind::Semicolon,
                SyntaxKind::LParen,
            ]));
            let name = self.parse_ts_ident_or_placeholder()
                .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                    .with_context("variable declarator name"))?;
            self.pop_context();
            self.skip_whitespace();

            // If there's a type annotation, wrap the name in a BindingIdent
            let name_with_type = if self.at(SyntaxKind::Colon) {
                self.consume();
                self.skip_whitespace();
                let type_ann = self.parse_type_until(&[SyntaxKind::Eq, SyntaxKind::Comma, SyntaxKind::Semicolon])?
                    .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                        .with_context("variable type annotation"))?;
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
                ]).map_err(|e| e.with_context("variable initializer"))?))
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

        Ok(IrNode::VarDecl {
            exported,
            declare: false,
            kind,
            decls,
        })
    }

    /// Parse a variable declaration with destructuring pattern as raw text.
    /// This handles patterns like `const { a, b } = obj;` or `const [x, y] = arr;`
    fn parse_var_decl_as_raw(&mut self, exported: bool, kind: &VarKind) -> ParseResult<IrNode> {
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
                    let placeholder = self.parse_interpolation()
                        .map_err(|e| e.with_context("destructuring pattern interpolation"))?;
                    parts.push(placeholder);
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
        Ok(IrNode::TsLoopStmt { parts: merged })
    }

    pub(super) fn parse_type_alias_decl(&mut self, exported: bool) -> ParseResult<IrNode> {
        // Consume "type"
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "type keyword"))?;
        self.skip_whitespace();

        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                .with_context("type alias name"))?;
        self.skip_whitespace();

        let type_params = self.parse_optional_type_params();
        self.skip_whitespace();

        if !self.at(SyntaxKind::Eq) {
            return Ok(IrNode::Raw("type ".to_string()));
        }
        self.consume();
        self.skip_whitespace();

        let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon])
            .map_err(|e| e.with_context("type alias definition"))?
            .ok_or_else(|| ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                .with_context("type alias definition"))?;

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
