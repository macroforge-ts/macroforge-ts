use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
#[cfg(feature = "swc")]
use swc_core::common::{GLOBALS, Globals};

#[cfg(all(feature = "swc", not(feature = "oxc")))]
use super::collectors::{collect_exported_names, collect_file_imports};
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
#[cfg(all(feature = "swc", not(feature = "oxc")))]
use crate::ts_syn::{lower_classes, lower_enums, lower_interfaces, lower_type_aliases};

#[cfg(feature = "oxc")]
use crate::ts_syn::{
    collect_exported_names_oxc, collect_file_imports_oxc, lower_classes_oxc, lower_enums_oxc,
    lower_interfaces_oxc, lower_type_aliases_oxc,
};

use super::config::ScanConfig;

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

        #[cfg(feature = "swc")]
        let globals = Globals::default();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            if self.is_in_skip_dir(path) {
                continue;
            }

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

            files_scanned += 1;
            if files_scanned > self.config.max_files as u32 {
                warnings.push(format!(
                    "Reached max file limit ({}). Some types may be missing.",
                    self.config.max_files
                ));
                break;
            }

            #[cfg(feature = "swc")]
            {
                GLOBALS.set(&globals, || {
                    if let Err(e) = self.scan_file(path, &mut registry, &root_str) {
                        warnings.push(format!("Failed to scan {:?}: {}", path, e));
                    }
                });
            }
            #[cfg(all(not(feature = "swc"), feature = "oxc"))]
            {
                if let Err(e) = self.scan_file(path, &mut registry, &root_str) {
                    warnings.push(format!("Failed to scan {:?}: {}", path, e));
                }
            }
        }

        Ok(ScanOutput {
            registry,
            files_scanned,
            warnings,
        })
    }

    pub(crate) fn is_in_skip_dir(&self, path: &Path) -> bool {
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

    fn scan_file(
        &self,
        path: &Path,
        registry: &mut TypeRegistry,
        project_root: &str,
    ) -> anyhow::Result<()> {
        let source = std::fs::read_to_string(path)?;
        let file_name = path.to_string_lossy().to_string();

        #[cfg(feature = "oxc")]
        {
            use oxc_allocator::Allocator;
            use oxc_parser::Parser;
            use oxc_span::SourceType;

            let allocator = Allocator::default();
            let source_type = SourceType::ts().with_jsx(file_name.ends_with(".tsx"));
            let ret = Parser::new(&allocator, &source, source_type).parse();

            if !ret.errors.is_empty() {
                return Err(anyhow::anyhow!("Oxc parse errors: {:?}", ret.errors));
            }

            let classes = lower_classes_oxc(&ret.program, &source, None).unwrap_or_default();
            let interfaces = lower_interfaces_oxc(&ret.program, &source, None).unwrap_or_default();
            let enums = lower_enums_oxc(&ret.program, &source, None).unwrap_or_default();
            let type_aliases =
                lower_type_aliases_oxc(&ret.program, &source, None).unwrap_or_default();

            if classes.is_empty()
                && interfaces.is_empty()
                && enums.is_empty()
                && type_aliases.is_empty()
            {
                return Ok(());
            }

            let file_imports = collect_file_imports_oxc(&ret.program);
            let exported_names = collect_exported_names_oxc(&ret.program);

            self.register_items(
                registry,
                project_root,
                &file_name,
                classes,
                interfaces,
                enums,
                type_aliases,
                file_imports,
                exported_names,
            );
            Ok(())
        }

        #[cfg(all(feature = "swc", not(feature = "oxc")))]
        {
            let module = crate::ts_syn::parse::parse_ts_module(&source, &file_name)
                .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

            let classes = lower_classes(&module, &source, None).unwrap_or_default();
            let interfaces = lower_interfaces(&module, &source, None).unwrap_or_default();
            let enums = lower_enums(&module, &source, None).unwrap_or_default();
            let type_aliases = lower_type_aliases(&module, &source, None).unwrap_or_default();

            if classes.is_empty()
                && interfaces.is_empty()
                && enums.is_empty()
                && type_aliases.is_empty()
            {
                return Ok(());
            }

            let file_imports = collect_file_imports(&module);
            let exported_names = collect_exported_names(&module);

            self.register_items(
                registry,
                project_root,
                &file_name,
                classes,
                interfaces,
                enums,
                type_aliases,
                file_imports,
                exported_names,
            );
            Ok(())
        }
        #[cfg(all(not(feature = "swc"), not(feature = "oxc")))]
        {
            Err(anyhow::anyhow!("No compiler backend enabled"))
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn register_items(
        &self,
        registry: &mut TypeRegistry,
        project_root: &str,
        file_name: &str,
        classes: Vec<crate::ts_syn::abi::ir::ClassIR>,
        interfaces: Vec<crate::ts_syn::abi::ir::InterfaceIR>,
        enums: Vec<crate::ts_syn::abi::ir::EnumIR>,
        type_aliases: Vec<crate::ts_syn::abi::ir::TypeAliasIR>,
        file_imports: Vec<crate::ts_syn::abi::ir::type_registry::FileImportEntry>,
        exported_names: std::collections::HashSet<String>,
    ) {
        for class in classes {
            let is_exported = exported_names.contains(&class.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            registry.insert(
                TypeRegistryEntry {
                    name: class.name.clone(),
                    file_path: file_name.to_string(),
                    is_exported,
                    definition: TypeDefinitionIR::Class(class),
                    file_imports: file_imports.clone(),
                },
                project_root,
            );
        }

        for iface in interfaces {
            let is_exported = exported_names.contains(&iface.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            registry.insert(
                TypeRegistryEntry {
                    name: iface.name.clone(),
                    file_path: file_name.to_string(),
                    is_exported,
                    definition: TypeDefinitionIR::Interface(iface),
                    file_imports: file_imports.clone(),
                },
                project_root,
            );
        }

        for enum_ir in enums {
            let is_exported = exported_names.contains(&enum_ir.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            registry.insert(
                TypeRegistryEntry {
                    name: enum_ir.name.clone(),
                    file_path: file_name.to_string(),
                    is_exported,
                    definition: TypeDefinitionIR::Enum(enum_ir),
                    file_imports: file_imports.clone(),
                },
                project_root,
            );
        }

        for alias in type_aliases {
            let is_exported = exported_names.contains(&alias.name);
            if self.config.exported_only && !is_exported {
                continue;
            }
            registry.insert(
                TypeRegistryEntry {
                    name: alias.name.clone(),
                    file_path: file_name.to_string(),
                    is_exported,
                    definition: TypeDefinitionIR::TypeAlias(alias),
                    file_imports: file_imports.clone(),
                },
                project_root,
            );
        }
    }
}
