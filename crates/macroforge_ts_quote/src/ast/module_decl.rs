use swc_core::ecma::ast::*;

// Module is the top-level container for a TypeScript/JavaScript file
impl_struct!(Module, [span, body, shebang]);

impl_enum!(
    ModuleDecl,
    [
        Import,
        ExportDecl,
        ExportNamed,
        ExportDefaultDecl,
        ExportDefaultExpr,
        ExportAll,
        TsImportEquals,
        TsExportAssignment,
        TsNamespaceExport
    ]
);

impl_struct!(ImportDecl, [span, specifiers, src, type_only, with, phase]);
impl_struct!(ExportDecl, [span, decl]);
impl_struct!(ExportDefaultDecl, [span, decl]);
impl_struct!(ExportDefaultExpr, [span, expr]);
impl_struct!(ExportAll, [span, type_only, src, with]);
impl_struct!(NamedExport, [span, specifiers, src, type_only, with]);

impl_enum!(ImportSpecifier, [Named, Default, Namespace]);

impl_struct!(ImportNamedSpecifier, [span, local, imported, is_type_only]);
impl_struct!(ImportDefaultSpecifier, [span, local]);
impl_struct!(ImportStarAsSpecifier, [span, local]);

impl_enum!(ExportSpecifier, [Named, Default, Namespace]);

impl_enum!(DefaultDecl, [Class, Fn, TsInterfaceDecl]);

impl_enum!(ModuleExportName, [Ident, Str]);

impl_struct!(ExportNamedSpecifier, [span, orig, exported, is_type_only]);
impl_struct!(ExportDefaultSpecifier, [exported]);
impl_struct!(ExportNamespaceSpecifier, [span, name]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ToCode;
    use crate::ctxt::Ctx;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;
    use swc_core::atoms::Atom;
    use swc_core::common::{Span, SyntaxContext};

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== Module Tests ====================

    #[test]
    fn test_module_empty() {
        let cx = empty_ctx();
        let module = Module {
            span: Span::default(),
            body: vec![],
            shebang: None,
        };
        let code = module.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Module"));
        assert!(code_str.contains("body"));
        assert!(code_str.contains("shebang"));
    }

    #[test]
    fn test_module_with_shebang() {
        let cx = empty_ctx();
        let module = Module {
            span: Span::default(),
            body: vec![],
            shebang: Some(Atom::from("#!/usr/bin/env node")),
        };
        let code = module.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Module"));
    }

    // ==================== ModuleDecl Enum Tests ====================

    #[test]
    fn test_module_decl_import_to_code() {
        let cx = empty_ctx();
        let decl = ModuleDecl::Import(ImportDecl {
            span: Span::default(),
            specifiers: vec![],
            src: Box::new(Str {
                span: Span::default(),
                value: Atom::from("./module").into(),
                raw: None,
            }),
            type_only: false,
            with: None,
            phase: ImportPhase::Evaluation,
        });
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ModuleDecl"));
        assert!(code_str.contains("Import"));
    }

    #[test]
    fn test_module_decl_export_decl_to_code() {
        let cx = empty_ctx();
        let decl = ModuleDecl::ExportDecl(ExportDecl {
            span: Span::default(),
            decl: Decl::Var(Box::new(VarDecl {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![],
            })),
        });
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ModuleDecl"));
        assert!(code_str.contains("ExportDecl"));
    }

    // ==================== ImportDecl Tests ====================

    #[test]
    fn test_import_decl_basic() {
        let cx = empty_ctx();
        let decl = ImportDecl {
            span: Span::default(),
            specifiers: vec![],
            src: Box::new(Str {
                span: Span::default(),
                value: Atom::from("lodash").into(),
                raw: None,
            }),
            type_only: false,
            with: None,
            phase: ImportPhase::Evaluation,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportDecl"));
        assert!(code_str.contains("specifiers"));
        assert!(code_str.contains("src"));
    }

    #[test]
    fn test_import_decl_type_only() {
        let cx = empty_ctx();
        let decl = ImportDecl {
            span: Span::default(),
            specifiers: vec![],
            src: Box::new(Str {
                span: Span::default(),
                value: Atom::from("./types").into(),
                raw: None,
            }),
            type_only: true,
            with: None,
            phase: ImportPhase::Evaluation,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportDecl"));
        assert!(code_str.contains("type_only"));
    }

    // ==================== ImportSpecifier Tests ====================

    #[test]
    fn test_import_specifier_named_to_code() {
        let cx = empty_ctx();
        let spec = ImportSpecifier::Named(ImportNamedSpecifier {
            span: Span::default(),
            local: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("foo"),
                optional: false,
            },
            imported: None,
            is_type_only: false,
        });
        let code = spec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportSpecifier"));
        assert!(code_str.contains("Named"));
    }

    #[test]
    fn test_import_specifier_default_to_code() {
        let cx = empty_ctx();
        let spec = ImportSpecifier::Default(ImportDefaultSpecifier {
            span: Span::default(),
            local: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyModule"),
                optional: false,
            },
        });
        let code = spec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportSpecifier"));
        assert!(code_str.contains("Default"));
    }

    #[test]
    fn test_import_specifier_namespace_to_code() {
        let cx = empty_ctx();
        let spec = ImportSpecifier::Namespace(ImportStarAsSpecifier {
            span: Span::default(),
            local: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("ns"),
                optional: false,
            },
        });
        let code = spec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportSpecifier"));
        assert!(code_str.contains("Namespace"));
    }

    // ==================== ExportDecl Tests ====================

    #[test]
    fn test_export_decl_to_code() {
        let cx = empty_ctx();
        let decl = ExportDecl {
            span: Span::default(),
            decl: Decl::Var(Box::new(VarDecl {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![],
            })),
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExportDecl"));
        assert!(code_str.contains("decl"));
    }

    // ==================== ExportDefaultExpr Tests ====================

    #[test]
    fn test_export_default_expr_to_code() {
        let cx = empty_ctx();
        let decl = ExportDefaultExpr {
            span: Span::default(),
            expr: Box::new(Expr::Lit(Lit::Num(Number {
                span: Span::default(),
                value: 42.0,
                raw: None,
            }))),
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExportDefaultExpr"));
        assert!(code_str.contains("expr"));
    }

    // ==================== ExportAll Tests ====================

    #[test]
    fn test_export_all_to_code() {
        let cx = empty_ctx();
        let decl = ExportAll {
            span: Span::default(),
            type_only: false,
            src: Box::new(Str {
                span: Span::default(),
                value: Atom::from("./other").into(),
                raw: None,
            }),
            with: None,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExportAll"));
        assert!(code_str.contains("src"));
    }

    // ==================== NamedExport Tests ====================

    #[test]
    fn test_named_export_to_code() {
        let cx = empty_ctx();
        let decl = NamedExport {
            span: Span::default(),
            specifiers: vec![],
            src: None,
            type_only: false,
            with: None,
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("NamedExport"));
        assert!(code_str.contains("specifiers"));
    }

    // ==================== ExportSpecifier Tests ====================

    #[test]
    fn test_export_specifier_named_to_code() {
        let cx = empty_ctx();
        let spec = ExportSpecifier::Named(ExportNamedSpecifier {
            span: Span::default(),
            orig: ModuleExportName::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("foo"),
                optional: false,
            }),
            exported: None,
            is_type_only: false,
        });
        let code = spec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ExportSpecifier"));
        assert!(code_str.contains("Named"));
    }

    // ==================== ModuleExportName Tests ====================

    #[test]
    fn test_module_export_name_ident_to_code() {
        let cx = empty_ctx();
        let name = ModuleExportName::Ident(Ident {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            sym: Atom::from("myExport"),
            optional: false,
        });
        let code = name.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ModuleExportName"));
        assert!(code_str.contains("Ident"));
    }

    #[test]
    fn test_module_export_name_str_to_code() {
        let cx = empty_ctx();
        let name = ModuleExportName::Str(Str {
            span: Span::default(),
            value: Atom::from("string-export").into(),
            raw: None,
        });
        let code = name.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ModuleExportName"));
        assert!(code_str.contains("Str"));
    }

    // ==================== DefaultDecl Tests ====================

    #[test]
    fn test_default_decl_fn_to_code() {
        let cx = empty_ctx();
        let decl = DefaultDecl::Fn(FnExpr {
            ident: None,
            function: Box::new(Function {
                ctxt: SyntaxContext::empty(),
                params: vec![],
                decorators: vec![],
                span: Span::default(),
                body: None,
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        });
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("DefaultDecl"));
        assert!(code_str.contains("Fn"));
    }
}
