use std::collections::HashSet;

use crate::ts_syn::abi::ir::type_registry::FileImportEntry;

/// Extract import declarations from a parsed SWC module.
pub fn collect_file_imports(module: &swc_core::ecma::ast::Module) -> Vec<FileImportEntry> {
    use swc_core::ecma::ast::*;

    let mut imports = Vec::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
            let module_specifier =
                String::from_utf8_lossy(import_decl.src.value.as_bytes()).to_string();
            let is_type_only = import_decl.type_only;

            for specifier in &import_decl.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local = named.local.sym.to_string();
                        let original = named.imported.as_ref().map(|i| match i {
                            ModuleExportName::Ident(id) => id.sym.to_string(),
                            ModuleExportName::Str(s) => {
                                String::from_utf8_lossy(s.value.as_bytes()).to_string()
                            }
                        });
                        imports.push(FileImportEntry {
                            local_name: local,
                            module_specifier: module_specifier.clone(),
                            original_name: original,
                            is_type_only: is_type_only || named.is_type_only,
                        });
                    }
                    ImportSpecifier::Default(default_spec) => {
                        imports.push(FileImportEntry {
                            local_name: default_spec.local.sym.to_string(),
                            module_specifier: module_specifier.clone(),
                            original_name: Some("default".to_string()),
                            is_type_only,
                        });
                    }
                    ImportSpecifier::Namespace(ns) => {
                        imports.push(FileImportEntry {
                            local_name: ns.local.sym.to_string(),
                            module_specifier: module_specifier.clone(),
                            original_name: Some("*".to_string()),
                            is_type_only,
                        });
                    }
                }
            }
        }
    }

    imports
}

/// Collect names of exported declarations from a parsed SWC module.
pub fn collect_exported_names(module: &swc_core::ecma::ast::Module) -> HashSet<String> {
    use swc_core::ecma::ast::*;

    let mut names = HashSet::new();

    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) => match &export.decl {
                Decl::Class(c) => {
                    names.insert(c.ident.sym.to_string());
                }
                Decl::TsInterface(i) => {
                    names.insert(i.id.sym.to_string());
                }
                Decl::TsEnum(e) => {
                    names.insert(e.id.sym.to_string());
                }
                Decl::TsTypeAlias(t) => {
                    names.insert(t.id.sym.to_string());
                }
                _ => {}
            },
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named)) => {
                // Handle `export { Foo, Bar }` and `export { Foo as Bar }`
                if named.src.is_none() {
                    // Only local re-exports (not `export { X } from './mod'`)
                    for spec in &named.specifiers {
                        if let ExportSpecifier::Named(n) = spec {
                            let name = match &n.orig {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(s) => {
                                    String::from_utf8_lossy(s.value.as_bytes()).to_string()
                                }
                            };
                            names.insert(name);
                        }
                    }
                }
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(default_export)) => {
                // `export default class Foo { ... }`
                if let DefaultDecl::Class(c) = &default_export.decl
                    && let Some(ident) = &c.ident
                {
                    names.insert(ident.sym.to_string());
                }
            }
            _ => {}
        }
    }

    names
}
