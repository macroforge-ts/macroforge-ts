use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use swc_core::common::{GLOBALS, Globals};

use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::{lower_classes, lower_enums, lower_interfaces, lower_type_aliases};

use super::collectors::{collect_exported_names, collect_file_imports};
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
        let classes = lower_classes(&module, &source, None).unwrap_or_default();
        let interfaces = lower_interfaces(&module, &source, None).unwrap_or_default();
        let enums = lower_enums(&module, &source, None).unwrap_or_default();
        let type_aliases = lower_type_aliases(&module, &source, None).unwrap_or_default();

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
