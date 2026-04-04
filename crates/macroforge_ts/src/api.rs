#[cfg(feature = "swc")]
use swc_core::common::{GLOBALS, Globals};

use crate::api_types::{
    ExpandOptions, ExpandResult, ImportSourceResult, LoadConfigResult, MacroDiagnostic,
    ScanOptions, ScanResult, SyntaxCheckResult, TransformResult,
};
use crate::expand_core::{expand_inner, transform_inner};

pub trait MacroforgeApi {
    type Error;

    fn check_syntax(code: String, filepath: String) -> Result<SyntaxCheckResult, Self::Error>;
    fn parse_import_sources(
        code: String,
        filepath: String,
    ) -> Result<Vec<ImportSourceResult>, Self::Error>;
    fn load_config(content: String, filepath: String) -> Result<LoadConfigResult, Self::Error>;
    fn clear_config_cache();
    fn transform_sync(code: String, filepath: String) -> Result<TransformResult, Self::Error>;
    fn expand_sync(
        code: String,
        filepath: String,
        options: Option<ExpandOptions>,
    ) -> Result<ExpandResult, Self::Error>;
    fn scan_project_sync(
        root_dir: String,
        options: Option<ScanOptions>,
    ) -> Result<ScanResult, Self::Error>;
}

pub struct CoreEngine;

impl CoreEngine {
    pub fn check_syntax(code: &str, filepath: &str) -> Result<SyntaxCheckResult, String> {
        #[cfg(feature = "swc")]
        {
            match crate::expand_core::parse_program(code, filepath) {
                Ok(_) => Ok(SyntaxCheckResult {
                    ok: true,
                    error: None,
                }),
                Err(err) => Ok(SyntaxCheckResult {
                    ok: false,
                    error: Some(err.to_string()),
                }),
            }
        }

        #[cfg(all(not(feature = "swc"), feature = "oxc"))]
        {
            use oxc_allocator::Allocator;
            use oxc_parser::Parser;
            use oxc_span::SourceType;

            let allocator = Allocator::default();
            let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));
            let parsed = Parser::new(&allocator, code, source_type).parse();

            if parsed.errors.is_empty() {
                Ok(SyntaxCheckResult {
                    ok: true,
                    error: None,
                })
            } else {
                Ok(SyntaxCheckResult {
                    ok: false,
                    error: Some(
                        parsed
                            .errors
                            .into_iter()
                            .map(|diagnostic| diagnostic.to_string())
                            .collect::<Vec<_>>()
                            .join("; "),
                    ),
                })
            }
        }
    }

    pub fn parse_import_sources(
        code: &str,
        filepath: &str,
    ) -> Result<Vec<ImportSourceResult>, String> {
        #[cfg(feature = "swc")]
        {
            use swc_core::ecma::ast::Program;

            let (program, _cm) =
                crate::expand_core::parse_program(code, filepath).map_err(|e| e.to_string())?;
            let module = match program {
                Program::Module(module) => module,
                Program::Script(_) => return Ok(vec![]),
            };

            let import_result = crate::host::collect_import_sources(&module, code);
            let mut imports = Vec::with_capacity(import_result.sources.len());
            for (local, module) in import_result.sources {
                imports.push(ImportSourceResult { local, module });
            }
            Ok(imports)
        }

        #[cfg(all(not(feature = "swc"), feature = "oxc"))]
        {
            use oxc_allocator::Allocator;
            use oxc_parser::Parser;
            use oxc_span::SourceType;

            let allocator = Allocator::default();
            let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));
            let parsed = Parser::new(&allocator, code, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(parsed
                    .errors
                    .into_iter()
                    .map(|diagnostic| diagnostic.to_string())
                    .collect::<Vec<_>>()
                    .join("; "));
            }

            let registry = crate::ts_syn::ImportRegistry::from_oxc_program(&parsed.program, code);
            Ok(registry
                .source_modules()
                .into_iter()
                .map(|(local, module)| ImportSourceResult { local, module })
                .collect())
        }
    }

    pub fn load_config(content: &str, filepath: &str) -> Result<LoadConfigResult, String> {
        use crate::host::MacroforgeConfigLoader;
        eprintln!("[macroforge:api] load_config called for {}", filepath);

        let config = MacroforgeConfigLoader::load_and_cache(content, filepath).map_err(|e| {
            eprintln!("[macroforge:api] load_config failed: {}", e);
            format!("Failed to parse config: {}", e)
        })?;

        let has_foreign_types = !config.foreign_types.is_empty();
        let foreign_type_count = config.foreign_types.len() as u32;

        eprintln!(
            "[macroforge:api] load_config success: keep_decorators={}, foreign_types={}",
            config.keep_decorators, foreign_type_count
        );

        Ok(LoadConfigResult {
            keep_decorators: config.keep_decorators,
            generate_convenience_const: config.generate_convenience_const,
            has_foreign_types,
            foreign_type_count,
        })
    }

    pub fn clear_config_cache() {
        eprintln!("[macroforge:api] clear_config_cache called");
        crate::host::clear_config_cache();
    }

    pub fn transform_sync(code: String, filepath: String) -> Result<TransformResult, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
            let handle = builder
                .spawn(
                    move || -> std::thread::Result<anyhow::Result<TransformResult>> {
                        #[cfg(feature = "swc")]
                        {
                            let globals = Globals::default();
                            return GLOBALS.set(&globals, || {
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    transform_inner(&code, &filepath)
                                }))
                            });
                        }

                        #[cfg(not(feature = "swc"))]
                        {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                transform_inner(&code, &filepath)
                            }))
                        }
                    },
                )
                .map_err(|e| format!("Failed to spawn transform thread: {}", e))?;

            handle
                .join()
                .map_err(|_| "Transform worker crashed".to_string())?
                .map_err(|_| "Transform panicked".to_string())?
                .map_err(|e| e.to_string())
        }

        #[cfg(target_arch = "wasm32")]
        {
            #[cfg(feature = "swc")]
            {
                let globals = Globals::default();
                GLOBALS.set(&globals, || {
                    transform_inner(&code, &filepath).map_err(|e| e.to_string())
                })
            }

            #[cfg(not(feature = "swc"))]
            {
                transform_inner(&code, &filepath).map_err(|e| e.to_string())
            }
        }
    }

    pub fn expand_sync(
        code: String,
        filepath: String,
        options: Option<ExpandOptions>,
    ) -> Result<ExpandResult, String> {
        eprintln!("[macroforge:api] expand_sync called for {}", filepath);
        if let Some(ref opts) = options {
            eprintln!(
                "[macroforge:api] options: config_path={:?}, external_decorator_modules={:?}, has_type_registry={}",
                opts.config_path,
                opts.external_decorator_modules,
                opts.type_registry_json.is_some()
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
            let handle = builder
                .spawn(
                    move || -> std::thread::Result<anyhow::Result<ExpandResult>> {
                        #[cfg(feature = "swc")]
                        {
                            let globals = Globals::default();
                            return GLOBALS.set(&globals, || {
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    expand_inner(&code, &filepath, options)
                                }))
                            });
                        }

                        #[cfg(not(feature = "swc"))]
                        {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                expand_inner(&code, &filepath, options)
                            }))
                        }
                    },
                )
                .map_err(|e| format!("Failed to spawn expand thread: {}", e))?;

            handle
                .join()
                .map_err(|_| "Expand worker crashed".to_string())?
                .map_err(|_| "Expand panicked".to_string())?
                .map_err(|e| e.to_string())
        }

        #[cfg(target_arch = "wasm32")]
        {
            #[cfg(feature = "swc")]
            {
                let globals = Globals::default();
                GLOBALS.set(&globals, || {
                    expand_inner(&code, &filepath, options).map_err(|e| e.to_string())
                })
            }

            #[cfg(not(feature = "swc"))]
            {
                expand_inner(&code, &filepath, options).map_err(|e| e.to_string())
            }
        }
    }

    pub fn scan_project_sync(
        root_dir: String,
        options: Option<ScanOptions>,
    ) -> Result<ScanResult, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
            let handle = builder
                .spawn(move || Self::scan_project_inner(&root_dir, options))
                .map_err(|e| format!("Failed to spawn scan thread: {}", e))?;

            handle
                .join()
                .map_err(|_| "Scan worker panicked".to_string())?
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self::scan_project_inner(&root_dir, options)
        }
    }

    fn scan_project_inner(
        root_dir: &str,
        options: Option<ScanOptions>,
    ) -> Result<ScanResult, String> {
        use crate::host::scanner::{ProjectScanner, ScanConfig};

        let mut config = ScanConfig {
            root_dir: std::path::PathBuf::from(root_dir),
            ..ScanConfig::default()
        };

        if let Some(opts) = options {
            if let Some(exts) = opts.extensions {
                config.extensions = exts;
            }
            if let Some(exported) = opts.exported_only {
                config.exported_only = exported;
            }
        }

        let scanner = ProjectScanner::new(config);
        let output = scanner
            .scan()
            .map_err(|e| format!("Project scan failed: {}", e))?;

        let types_found = output.registry.len() as u32;
        let registry_json = serde_json::to_string(&output.registry)
            .map_err(|e| format!("Failed to serialize registry: {}", e))?;

        let diagnostics = output
            .warnings
            .into_iter()
            .map(|msg| MacroDiagnostic {
                level: "warning".to_string(),
                message: msg,
                start: None,
                end: None,
            })
            .collect();

        Ok(ScanResult {
            registry_json,
            files_scanned: output.files_scanned,
            types_found,
            diagnostics,
        })
    }
}
