//! Project-wide TypeScript scanner for building the type registry.
//!
//! This module walks the project directory, parses all `.ts`/`.tsx` files with SWC,
//! lowers declarations to IR, and collects them into a [`TypeRegistry`].
//!
//! The scanner runs BEFORE macro expansion as a pre-pass to give macros
//! project-wide type awareness (inspired by Zig's compile-time type introspection).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig};
//!
//! let config = ScanConfig {
//!     root_dir: std::path::PathBuf::from("/path/to/project"),
//!     ..Default::default()
//! };
//! let scanner = ProjectScanner::new(config);
//! let output = scanner.scan().expect("scan failed");
//! println!("Found {} types", output.registry.len());
//! ```

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use swc_core::common::{GLOBALS, Globals};

use crate::ts_syn::abi::ir::type_registry::{
    FileImportEntry, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};
use crate::ts_syn::{lower_classes, lower_enums, lower_interfaces, lower_type_aliases};

/// Configuration for the project scanner.
pub struct ScanConfig {
    /// Project root directory.
    pub root_dir: PathBuf,

    /// File extensions to scan (default: `[".ts", ".tsx"]`).
    pub extensions: Vec<String>,

    /// Directories to skip (default: common non-source dirs).
    pub skip_dirs: HashSet<String>,

    /// Whether to only collect exported types (default: `false` - collect all).
    pub exported_only: bool,

    /// Maximum number of files to scan (safety limit, default: 10,000).
    pub max_files: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            extensions: vec![".ts".into(), ".tsx".into()],
            skip_dirs: [
                "node_modules",
                ".git",
                "dist",
                "build",
                ".macroforge",
                "coverage",
                ".next",
                ".nuxt",
                ".svelte-kit",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            exported_only: false,
            max_files: 10_000,
        }
    }
}

/// Result of scanning a project.
pub struct ScanOutput {
    /// The populated type registry.
    pub registry: TypeRegistry,
    /// Number of files scanned.
    pub files_scanned: u32,
    /// Warnings from files that failed to parse.
    pub warnings: Vec<String>,
}

/// Scans a TypeScript project and builds a [`TypeRegistry`].
pub struct ProjectScanner {
    config: ScanConfig,
}

impl ProjectScanner {
    /// Create a new scanner with the given configuration.
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    /// Create a scanner with defaults, rooted at the given directory.
    pub fn with_root(root_dir: PathBuf) -> Self {
        Self {
            config: ScanConfig {
                root_dir,
                ..Default::default()
            },
        }
    }

    /// Perform the full project scan and return a populated [`TypeRegistry`].
    pub fn scan(&self) -> anyhow::Result<ScanOutput> {
        let mut registry = TypeRegistry::new();
        let root_str = self.config.root_dir.to_string_lossy().to_string();
        let mut files_scanned: u32 = 0;
        let mut warnings = Vec::new();

        let walker = WalkBuilder::new(&self.config.root_dir)
            .hidden(true) // Skip hidden files by default
            .git_ignore(true) // Respect .gitignore
            .git_global(false)
            .git_exclude(false)
            .build();

        let globals = Globals::default();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();

            // Skip directories in skip list
            if path.is_dir() {
                // Note: `ignore` crate handles .gitignore; we still skip known dirs
                // in case they aren't gitignored.
                continue;
            }

            // Check if any ancestor directory is in skip list
            if self.is_in_skip_dir(path) {
                continue;
            }

            // Check file extension
            let has_matching_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|ext| {
                    let dotted = format!(".{}", ext);
                    self.config.extensions.contains(&dotted)
                })
                .unwrap_or(false);

            if !has_matching_ext {
                continue;
            }

            // Safety limit
            files_scanned += 1;
            if files_scanned > self.config.max_files as u32 {
                warnings.push(format!(
                    "Reached max file limit ({}). Some types may be missing.",
                    self.config.max_files
                ));
                break;
            }

            // Scan this file inside SWC globals context
            GLOBALS.set(&globals, || {
                if let Err(e) = self.scan_file(path, &mut registry, &root_str) {
                    warnings.push(format!("Failed to scan {:?}: {}", path, e));
                }
            });
        }

        Ok(ScanOutput {
            registry,
            files_scanned,
            warnings,
        })
    }

    /// Check if a path has any ancestor directory in the skip list.
    fn is_in_skip_dir(&self, path: &Path) -> bool {
        for component in path.components() {
            if let std::path::Component::Normal(name) = component
                && let Some(name_str) = name.to_str()
                && self.config.skip_dirs.contains(name_str)
            {
                return true;
            }
        }
        false
    }

    /// Scan a single TypeScript file and add its types to the registry.
    fn scan_file(
        &self,
        path: &Path,
        registry: &mut TypeRegistry,
        project_root: &str,
    ) -> anyhow::Result<()> {
        let source = std::fs::read_to_string(path)?;
        let file_name = path.to_string_lossy().to_string();

        // Parse the file with SWC
        let module = crate::ts_syn::parse::parse_ts_module(&source, &file_name)
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

        // Lower all declaration types
        let classes = lower_classes(&module, &source).unwrap_or_default();
        let interfaces = lower_interfaces(&module, &source).unwrap_or_default();
        let enums = lower_enums(&module, &source).unwrap_or_default();
        let type_aliases = lower_type_aliases(&module, &source).unwrap_or_default();

        // Skip files with no type declarations
        if classes.is_empty()
            && interfaces.is_empty()
            && enums.is_empty()
            && type_aliases.is_empty()
        {
            return Ok(());
        }

        // Collect import information from this file
        let file_imports = collect_file_imports(&module);

        // Determine which declarations are exported
        let exported_names = collect_exported_names(&module);

        // Register classes
        for class in classes {
            let is_exported = exported_names.contains(&class.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            let entry = TypeRegistryEntry {
                name: class.name.clone(),
                file_path: file_name.clone(),
                is_exported,
                definition: TypeDefinitionIR::Class(class),
                file_imports: file_imports.clone(),
            };
            registry.insert(entry, project_root);
        }

        // Register interfaces
        for iface in interfaces {
            let is_exported = exported_names.contains(&iface.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            let entry = TypeRegistryEntry {
                name: iface.name.clone(),
                file_path: file_name.clone(),
                is_exported,
                definition: TypeDefinitionIR::Interface(iface),
                file_imports: file_imports.clone(),
            };
            registry.insert(entry, project_root);
        }

        // Register enums
        for enum_ir in enums {
            let is_exported = exported_names.contains(&enum_ir.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            let entry = TypeRegistryEntry {
                name: enum_ir.name.clone(),
                file_path: file_name.clone(),
                is_exported,
                definition: TypeDefinitionIR::Enum(enum_ir),
                file_imports: file_imports.clone(),
            };
            registry.insert(entry, project_root);
        }

        // Register type aliases
        for alias in type_aliases {
            let is_exported = exported_names.contains(&alias.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            let entry = TypeRegistryEntry {
                name: alias.name.clone(),
                file_path: file_name.clone(),
                is_exported,
                definition: TypeDefinitionIR::TypeAlias(alias),
                file_imports: file_imports.clone(),
            };
            registry.insert(entry, project_root);
        }

        Ok(())
    }
}

/// Extract import declarations from a parsed SWC module.
fn collect_file_imports(module: &swc_core::ecma::ast::Module) -> Vec<FileImportEntry> {
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
fn collect_exported_names(module: &swc_core::ecma::ast::Module) -> HashSet<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_config_defaults() {
        let config = ScanConfig::default();
        assert_eq!(config.extensions, vec![".ts", ".tsx"]);
        assert!(config.skip_dirs.contains("node_modules"));
        assert!(config.skip_dirs.contains("dist"));
        assert!(!config.exported_only);
        assert_eq!(config.max_files, 10_000);
    }

    #[test]
    fn test_is_in_skip_dir() {
        let scanner = ProjectScanner::new(ScanConfig::default());
        assert!(scanner.is_in_skip_dir(Path::new("/project/node_modules/foo/bar.ts")));
        assert!(scanner.is_in_skip_dir(Path::new("/project/dist/main.ts")));
        assert!(!scanner.is_in_skip_dir(Path::new("/project/src/models/user.ts")));
    }

    #[test]
    fn test_collect_exported_names() {
        use swc_core::common::{GLOBALS, Globals};

        let source = r#"
            export class User { name: string = ""; }
            export interface Config { key: string; }
            class Internal {}
            export enum Status { Active, Inactive }
            export type ID = string;
        "#;

        let globals = Globals::default();
        GLOBALS.set(&globals, || {
            let module =
                crate::ts_syn::parse::parse_ts_module(source, "test.ts").expect("parse failed");
            let names = collect_exported_names(&module);
            assert!(names.contains("User"));
            assert!(names.contains("Config"));
            assert!(names.contains("Status"));
            assert!(names.contains("ID"));
            assert!(!names.contains("Internal"));
        });
    }

    #[test]
    fn test_collect_file_imports() {
        use swc_core::common::{GLOBALS, Globals};

        let source = r#"
            import { User, type Config } from "./models";
            import type { Status } from "./status";
            import DefaultExport from "./default";
        "#;

        let globals = Globals::default();
        GLOBALS.set(&globals, || {
            let module =
                crate::ts_syn::parse::parse_ts_module(source, "test.ts").expect("parse failed");
            let imports = collect_file_imports(&module);

            assert_eq!(imports.len(), 4);

            let user_import = imports.iter().find(|i| i.local_name == "User").unwrap();
            assert_eq!(user_import.module_specifier, "./models");
            assert!(!user_import.is_type_only);

            let config_import = imports.iter().find(|i| i.local_name == "Config").unwrap();
            assert!(config_import.is_type_only);

            let status_import = imports.iter().find(|i| i.local_name == "Status").unwrap();
            assert!(status_import.is_type_only);

            let default_import = imports
                .iter()
                .find(|i| i.local_name == "DefaultExport")
                .unwrap();
            assert_eq!(default_import.original_name.as_deref(), Some("default"));
        });
    }
}
