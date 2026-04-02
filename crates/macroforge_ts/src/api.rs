use swc_core::{
    common::{GLOBALS, Globals},
    ecma::ast::Program,
};

use crate::api_types::{
    ExpandOptions, ExpandResult, ImportSourceResult, LoadConfigResult, MacroDiagnostic,
    ScanOptions, ScanResult, SyntaxCheckResult, TransformResult,
};
use crate::expand_core::{expand_inner, parse_program, transform_inner};

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
        match parse_program(code, filepath) {
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

    pub fn parse_import_sources(
        code: &str,
        filepath: &str,
    ) -> Result<Vec<ImportSourceResult>, String> {
        let (program, _cm) = parse_program(code, filepath).map_err(|e| e.to_string())?;
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

    pub fn load_config(content: &str, filepath: &str) -> Result<LoadConfigResult, String> {
        use crate::host::MacroforgeConfigLoader;

        let config = MacroforgeConfigLoader::load_and_cache(content, filepath)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(LoadConfigResult {
            keep_decorators: config.keep_decorators,
            generate_convenience_const: config.generate_convenience_const,
            has_foreign_types: !config.foreign_types.is_empty(),
            foreign_type_count: config.foreign_types.len() as u32,
        })
    }

    pub fn clear_config_cache() {
        crate::host::clear_config_cache();
    }

    pub fn transform_sync(code: String, filepath: String) -> Result<TransformResult, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
            let handle = builder
                .spawn(
                    move || -> std::thread::Result<anyhow::Result<TransformResult>> {
                        let globals = Globals::default();
                        GLOBALS.set(&globals, || {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                transform_inner(&code, &filepath)
                            }))
                        })
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
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                transform_inner(&code, &filepath).map_err(|e| e.to_string())
            })
        }
    }

    pub fn expand_sync(
        code: String,
        filepath: String,
        options: Option<ExpandOptions>,
    ) -> Result<ExpandResult, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
            let handle = builder
                .spawn(
                    move || -> std::thread::Result<anyhow::Result<ExpandResult>> {
                        let globals = Globals::default();
                        GLOBALS.set(&globals, || {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                expand_inner(&code, &filepath, options)
                            }))
                        })
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
            let globals = Globals::default();
            GLOBALS.set(&globals, || {
                expand_inner(&code, &filepath, options).map_err(|e| e.to_string())
            })
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
