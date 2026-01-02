//! Import and Export declaration parsing
//!
//! Handles:
//! - `import { a, b as c } from "module"`
//! - `import foo from "module"`
//! - `import * as ns from "module"`
//! - `import type { T } from "module"`
//! - `export { a, b as c }`
//! - `export { a } from "module"`
//! - `export * from "module"`
//! - `export default expr`

use super::super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::*;

impl Parser {
    /// Parse import declaration
    /// Handles: import default, import named, import namespace, import type
    pub(crate) fn parse_import_decl(&mut self) -> Option<IrNode> {
        // Consume "import"
        self.consume()?;
        self.skip_whitespace();

        // Check for type-only import: `import type { ... }`
        let type_only = if self.at(SyntaxKind::TypeKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        // Parse the import specifiers
        let mut specifiers = Vec::new();

        // Check what kind of import this is
        match self.current_kind() {
            // Namespace import: `import * as ns from "module"`
            Some(SyntaxKind::Star) => {
                self.consume(); // consume *
                self.skip_whitespace();

                if self.at(SyntaxKind::AsKw) {
                    self.consume(); // consume "as"
                    self.skip_whitespace();

                    let local = self.parse_ts_ident_or_placeholder()?;
                    specifiers.push(IrNode::NamespaceImport {
                        local: Box::new(local),
                    });
                }
            }

            // Named import: `import { a, b } from "module"`
            Some(SyntaxKind::LBrace) => {
                self.consume(); // consume {
                self.skip_whitespace();

                while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
                    // Check for type-only specifier: `import { type Foo }`
                    let specifier_type_only = if self.at(SyntaxKind::TypeKw) {
                        self.consume();
                        self.skip_whitespace();
                        true
                    } else {
                        false
                    };

                    let imported = self.parse_ts_ident_or_placeholder()?;
                    self.skip_whitespace();

                    // Check for alias: `a as b`
                    let (local, imported_name) = if self.at(SyntaxKind::AsKw) {
                        self.consume(); // consume "as"
                        self.skip_whitespace();
                        let local = self.parse_ts_ident_or_placeholder()?;
                        (local, Some(Box::new(imported)))
                    } else {
                        (imported, None)
                    };

                    // If type-only specifier, wrap in a special node or mark it
                    // For now, we treat it as a regular named import
                    let _ = specifier_type_only; // TODO: handle per-specifier type-only

                    specifiers.push(IrNode::NamedImport {
                        local: Box::new(local),
                        imported: imported_name,
                    });

                    self.skip_whitespace();
                    if self.at(SyntaxKind::Comma) {
                        self.consume();
                        self.skip_whitespace();
                    } else {
                        break;
                    }
                }

                self.expect(SyntaxKind::RBrace);
            }

            // Default import or default + named: `import foo from "module"` or `import foo, { bar } from "module"`
            Some(SyntaxKind::Ident) | Some(SyntaxKind::At) => {
                let default_local = self.parse_ts_ident_or_placeholder()?;
                specifiers.push(IrNode::DefaultImport {
                    local: Box::new(default_local),
                });
                self.skip_whitespace();

                // Check for comma followed by named imports: `import foo, { bar } from "module"`
                if self.at(SyntaxKind::Comma) {
                    self.consume();
                    self.skip_whitespace();

                    if self.at(SyntaxKind::LBrace) {
                        self.consume(); // consume {
                        self.skip_whitespace();

                        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
                            let imported = self.parse_ts_ident_or_placeholder()?;
                            self.skip_whitespace();

                            let (local, imported_name) = if self.at(SyntaxKind::AsKw) {
                                self.consume();
                                self.skip_whitespace();
                                let local = self.parse_ts_ident_or_placeholder()?;
                                (local, Some(Box::new(imported)))
                            } else {
                                (imported, None)
                            };

                            specifiers.push(IrNode::NamedImport {
                                local: Box::new(local),
                                imported: imported_name,
                            });

                            self.skip_whitespace();
                            if self.at(SyntaxKind::Comma) {
                                self.consume();
                                self.skip_whitespace();
                            } else {
                                break;
                            }
                        }

                        self.expect(SyntaxKind::RBrace);
                    } else if self.at(SyntaxKind::Star) {
                        // `import foo, * as ns from "module"`
                        self.consume(); // consume *
                        self.skip_whitespace();
                        if self.at(SyntaxKind::AsKw) {
                            self.consume();
                            self.skip_whitespace();
                            let local = self.parse_ts_ident_or_placeholder()?;
                            specifiers.push(IrNode::NamespaceImport {
                                local: Box::new(local),
                            });
                        }
                    }
                }
            }

            // String literal only (side-effect import): `import "module"`
            Some(SyntaxKind::DoubleQuote) | Some(SyntaxKind::SingleQuote) => {
                // No specifiers, just the source
            }

            _ => {
                // Unknown pattern, fall back to raw
                return Some(IrNode::Raw("import ".to_string()));
            }
        }

        self.skip_whitespace();

        // Parse "from" and source
        if self.at(SyntaxKind::FromKw) {
            self.consume();
            self.skip_whitespace();
        }

        // Parse source string
        let src = self.parse_string_content()?;

        // Consume semicolon if present
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ImportDecl {
            type_only,
            specifiers,
            src,
        })
    }

    /// Parse export declaration (enhanced version)
    /// Handles: export named, export from, export all, export default
    pub(crate) fn parse_export_decl_full(&mut self) -> ParseResult<IrNode> {
        // Note: "export" is already consumed by parse_export_decl
        // This is called from parse_export_decl for complex cases

        match self.current_kind() {
            // Named export: `export { a, b as c }`
            Some(SyntaxKind::LBrace) => self.parse_named_export().ok_or_else(|| {
                ParseError::new(ParseErrorKind::ExpectedExpression, self.current_byte_offset())
                    .with_context("parsing named export")
            }),

            // Export all: `export * from "module"`
            Some(SyntaxKind::Star) => self.parse_export_all().ok_or_else(|| {
                ParseError::new(ParseErrorKind::ExpectedExpression, self.current_byte_offset())
                    .with_context("parsing export all")
            }),

            // Export default: `export default expr`
            Some(SyntaxKind::DefaultKw) => self.parse_export_default(),

            // Type export: `export type { T }` or `export type Foo = ...`
            Some(SyntaxKind::TypeKw) => {
                // Peek ahead to check if it's a named export or type alias
                // Save position and check what follows "type"
                self.consume(); // consume "type"
                self.skip_whitespace();

                if self.at(SyntaxKind::LBrace) {
                    // `export type { T }` - named type export
                    self.parse_named_export_inner(true).ok_or_else(|| {
                        ParseError::new(ParseErrorKind::ExpectedExpression, self.current_byte_offset())
                            .with_context("parsing named type export")
                    })
                } else {
                    // `export type Foo = ...` - type alias
                    // Don't consume again - parse_type_alias_decl expects "type" to be current
                    // But we already consumed it, so we need to parse the name directly here
                    let name = self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                        ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                            .with_context("type alias name")
                    })?;
                    self.skip_whitespace();

                    let type_params = self.parse_optional_type_params();
                    self.skip_whitespace();

                    if !self.at(SyntaxKind::Eq) {
                        return Ok(IrNode::Raw("export type ".to_string()));
                    }
                    self.consume(); // consume =
                    self.skip_whitespace();

                    let type_ann = self.parse_type_until(&[SyntaxKind::Semicolon])?
                        .ok_or_else(|| {
                            ParseError::new(ParseErrorKind::ExpectedTypeAnnotation, self.current_byte_offset())
                                .with_context("type alias definition")
                        })?;

                    if self.at(SyntaxKind::Semicolon) {
                        self.consume();
                    }

                    Ok(IrNode::TypeAliasDecl {
                        exported: true,
                        declare: false,
                        name: Box::new(name),
                        type_params,
                        type_ann: Box::new(type_ann),
                    })
                }
            }

            // Declaration exports handled by parse_export_decl
            _ => Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_context("parsing export declaration")
                .with_expected(&["{", "*", "default", "type"])),
        }
    }

    /// Parse named export: `export { a, b as c }` or `export { a } from "module"`
    fn parse_named_export(&mut self) -> Option<IrNode> {
        self.parse_named_export_inner(false)
    }

    fn parse_named_export_inner(&mut self, type_only: bool) -> Option<IrNode> {
        self.expect(SyntaxKind::LBrace)?;
        self.skip_whitespace();

        let mut specifiers = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            // Check for type-only specifier
            let specifier_type_only = if self.at(SyntaxKind::TypeKw) {
                self.consume();
                self.skip_whitespace();
                true
            } else {
                false
            };
            let _ = specifier_type_only; // TODO: handle per-specifier type-only

            let local = self.parse_ts_ident_or_placeholder()?;
            self.skip_whitespace();

            // Check for alias: `a as b`
            let exported = if self.at(SyntaxKind::AsKw) {
                self.consume();
                self.skip_whitespace();
                Some(Box::new(self.parse_ts_ident_or_placeholder()?))
            } else {
                None
            };

            specifiers.push(IrNode::ExportSpecifier {
                local: Box::new(local),
                exported,
            });

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
                self.skip_whitespace();
            } else {
                break;
            }
        }

        self.expect(SyntaxKind::RBrace);
        self.skip_whitespace();

        // Check for re-export: `export { a } from "module"`
        let src = if self.at(SyntaxKind::FromKw) {
            self.consume();
            self.skip_whitespace();
            Some(self.parse_string_content()?)
        } else {
            None
        };

        // Consume semicolon if present
        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::NamedExport {
            specifiers,
            src,
            type_only,
        })
    }

    /// Parse export all: `export * from "module"` or `export * as ns from "module"`
    fn parse_export_all(&mut self) -> Option<IrNode> {
        self.consume(); // consume *
        self.skip_whitespace();

        // Check for `export * as ns from "module"` (namespace re-export)
        if self.at(SyntaxKind::AsKw) {
            self.consume();
            self.skip_whitespace();
            let ns_name = self.parse_ts_ident_or_placeholder()?;
            self.skip_whitespace();

            // For now, treat `export * as ns from "module"` as a named export
            // with a single namespace specifier
            if self.at(SyntaxKind::FromKw) {
                self.consume();
                self.skip_whitespace();
            }
            let src = self.parse_string_content()?;

            if self.at(SyntaxKind::Semicolon) {
                self.consume();
            }

            return Some(IrNode::NamedExport {
                specifiers: vec![IrNode::ExportSpecifier {
                    local: Box::new(IrNode::Raw("*".to_string())),
                    exported: Some(Box::new(ns_name)),
                }],
                src: Some(src),
                type_only: false,
            });
        }

        // Regular export all
        if self.at(SyntaxKind::FromKw) {
            self.consume();
            self.skip_whitespace();
        }

        let src = self.parse_string_content()?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Some(IrNode::ExportAll {
            src,
            type_only: false,
        })
    }

    /// Parse export default: `export default expr`
    fn parse_export_default(&mut self) -> ParseResult<IrNode> {
        self.consume(); // consume "default"
        self.skip_whitespace();

        // Check if it's a class or function declaration
        match self.current_kind() {
            Some(SyntaxKind::ClassKw) => {
                // export default class Foo { }
                let class_decl = self
                    .parse_class_decl(true)
                    .map_err(|e| e.with_context("parsing export default class declaration"))?;
                return Ok(IrNode::ExportDefaultExpr {
                    expr: Box::new(class_decl),
                });
            }
            Some(SyntaxKind::FunctionKw) => {
                // export default function foo() { }
                let fn_decl = self.parse_function_decl(true, false)
                    .map_err(|e| e.with_context("parsing export default function declaration"))?;
                return Ok(IrNode::ExportDefaultExpr {
                    expr: Box::new(fn_decl),
                });
            }
            Some(SyntaxKind::AsyncKw) => {
                // export default async function foo() { }
                let fn_decl = self.parse_async_decl(true)
                    .map_err(|e| e.with_context("parsing export default async function declaration"))?;
                return Ok(IrNode::ExportDefaultExpr {
                    expr: Box::new(fn_decl),
                });
            }
            _ => {}
        }

        // Parse as expression
        let expr = self
            .parse_ts_expr_until(&[SyntaxKind::Semicolon])
            .map_err(|e| e.with_context("parsing export default expression"))?;

        if self.at(SyntaxKind::Semicolon) {
            self.consume();
        }

        Ok(IrNode::ExportDefaultExpr {
            expr: Box::new(expr),
        })
    }

    /// Parse string content (handles both single and double quotes)
    fn parse_string_content(&mut self) -> Option<String> {
        match self.current_kind() {
            Some(SyntaxKind::DoubleQuote) => {
                self.consume(); // consume opening quote
                let mut content = String::new();

                while !self.at_eof() && !self.at(SyntaxKind::DoubleQuote) {
                    if let Some(token) = self.consume() {
                        content.push_str(&token.text);
                    }
                }

                self.expect(SyntaxKind::DoubleQuote); // consume closing quote
                Some(content)
            }
            Some(SyntaxKind::SingleQuote) => {
                self.consume(); // consume opening quote
                let mut content = String::new();

                while !self.at_eof() && !self.at(SyntaxKind::SingleQuote) {
                    if let Some(token) = self.consume() {
                        content.push_str(&token.text);
                    }
                }

                self.expect(SyntaxKind::SingleQuote); // consume closing quote
                Some(content)
            }
            // Handle raw string tokens that might already include quotes
            Some(SyntaxKind::Ident) | Some(SyntaxKind::Text) => {
                let text = self.consume()?.text;
                // Remove surrounding quotes if present
                if (text.starts_with('"') && text.ends_with('"'))
                    || (text.starts_with('\'') && text.ends_with('\''))
                {
                    Some(text[1..text.len() - 1].to_string())
                } else {
                    Some(text)
                }
            }
            _ => None,
        }
    }
}
