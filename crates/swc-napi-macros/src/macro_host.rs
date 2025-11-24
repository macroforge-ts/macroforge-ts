use anyhow::{Context, Result};
use libloading::Library;
use playground_macros::register_playground_macros;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use swc_core::{
    common::Span,
    ecma::ast::{
        Class, Decl, Decorator, ExportDecl, Module, ModuleDecl, ModuleItem, Program, Stmt,
    },
};
use ts_macro_abi::{ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, Patch, SpanIR};
use ts_macro_host::{
    MacroConfig, MacroDispatcher, MacroManifest, MacroRegistry, PatchCollector,
    builtin::register_builtin_macros,
};
use ts_syn::lower_classes;
use walkdir::WalkDir;

const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Connects the SWC parser to the macro host.
pub struct MacroHostIntegration {
    dispatcher: MacroDispatcher,
    config: MacroConfig,
    _dynamic_libs: Vec<libloading::Library>,
}

/// Result of attempting to expand macros in a source file.
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
    pub type_output: Option<String>,
}

impl MacroHostIntegration {
    /// Build a macro host with the built-in macro registry populated.
    pub fn new() -> Result<Self> {
        let (config, root_dir) = MacroConfig::find_with_root()
            .context("failed to discover macro configuration")?
            .unwrap_or_else(|| {
                (
                    MacroConfig::default(),
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                )
            });
        Self::with_config(config, root_dir)
    }

    pub fn with_config(config: MacroConfig, root_dir: std::path::PathBuf) -> Result<Self> {
        let registry = MacroRegistry::new();
        let dynamic_libs = register_packages(&registry, &config, &root_dir)?;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
            _dynamic_libs: dynamic_libs,
        })
    }

    /// Expand all macros found in the parsed program and return the updated source code.
    pub fn expand(
        &self,
        source: &str,
        program: &Program,
        file_name: &str,
    ) -> Result<MacroExpansion> {
        if !source.contains("@Derive") {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
            });
        }

        let module = match program {
            Program::Module(module) => module,
            Program::Script(_) => {
                return Ok(MacroExpansion {
                    code: source.to_string(),
                    diagnostics: Vec::new(),
                    changed: false,
                    type_output: None,
                });
            }
        };

        let classes = lower_classes(module, source)
            .context("failed to lower classes for macro processing")?;

        if classes.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
            });
        }

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let derive_targets = collect_derive_targets(module, &class_map);

        if derive_targets.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
            });
        }

        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();

        for target in derive_targets {
            let decorator_removal = Patch::Delete {
                span: target.decorator_span,
            };
            collector.add_runtime_patches(vec![decorator_removal.clone()]);
            collector.add_type_patches(vec![decorator_removal]);

            for macro_name in target.macro_names {
                let ctx = MacroContextIR::new_derive_class(
                    macro_name.clone(),
                    DERIVE_MODULE_PATH.to_string(),
                    target.decorator_span,
                    target.class_ir.span,
                    file_name.to_string(),
                    target.class_ir.clone(),
                );

                let result = self.dispatcher.dispatch(ctx);

                if !result.diagnostics.is_empty() {
                    diagnostics.extend(result.diagnostics.clone());
                }

                collector.add_runtime_patches(result.runtime_patches);
                collector.add_type_patches(result.type_patches);
            }
        }

        let updated_code = collector
            .apply_runtime_patches(source)
            .context("failed to apply macro-generated patches")?;

        let type_output = if collector.has_type_patches() {
            Some(
                collector
                    .apply_type_patches(source)
                    .context("failed to apply macro-generated type patches")?,
            )
        } else {
            None
        };

        let mut expansion = MacroExpansion {
            code: updated_code,
            diagnostics,
            changed: true,
            type_output,
        };

        self.enforce_diagnostic_limit(&mut expansion.diagnostics);

        Ok(expansion)
    }

    fn enforce_diagnostic_limit(&self, diagnostics: &mut Vec<Diagnostic>) {
        let max = self.config.limits.max_diagnostics;
        if max == 0 {
            diagnostics.clear();
            return;
        }

        if diagnostics.len() > max {
            diagnostics.truncate(max.saturating_sub(1));
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!(
                    "Diagnostic output truncated to {} entries per macro host configuration",
                    max
                ),
                span: None,
                notes: vec![],
                help: Some(
                    "Adjust `limits.maxDiagnostics` in ts-macros.json to see all diagnostics"
                        .to_string(),
                ),
            });
        }
    }
}

/// Internal span key for matching lowered IR to SWC nodes.
#[derive(Hash, PartialEq, Eq)]
struct SpanKey(u32, u32);

impl From<SpanIR> for SpanKey {
    fn from(span: SpanIR) -> Self {
        SpanKey(span.start, span.end)
    }
}

impl From<Span> for SpanKey {
    fn from(span: Span) -> Self {
        SpanKey(span.lo.0, span.hi.0)
    }
}

#[derive(Clone)]
struct DeriveTarget {
    macro_names: Vec<String>,
    decorator_span: SpanIR,
    class_ir: ClassIR,
}

fn collect_derive_targets(
    module: &Module,
    class_map: &HashMap<SpanKey, ClassIR>,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();

    for item in &module.body {
        match item {
            ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => {
                collect_from_class(&class_decl.class, class_map, &mut targets);
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Class(class_decl),
                ..
            })) => {
                collect_from_class(&class_decl.class, class_map, &mut targets);
            }
            _ => {}
        }
    }

    targets
}

fn collect_from_class(
    class: &Class,
    class_map: &HashMap<SpanKey, ClassIR>,
    out: &mut Vec<DeriveTarget>,
) {
    if class.decorators.is_empty() {
        return;
    }

    let key = SpanKey::from(class.span);
    let Some(class_ir) = class_map.get(&key) else {
        return;
    };

    for decorator in &class.decorators {
        if let Some(macro_names) = parse_derive_decorator(decorator) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: span_to_ir(decorator.span),
                class_ir: class_ir.clone(),
            });
        }
    }
}

fn parse_derive_decorator(decorator: &Decorator) -> Option<Vec<String>> {
    let call = match decorator.expr.as_ref() {
        swc_core::ecma::ast::Expr::Call(call) => call,
        _ => return None,
    };

    let callee = match &call.callee {
        swc_core::ecma::ast::Callee::Expr(expr) => match expr.as_ref() {
            swc_core::ecma::ast::Expr::Ident(ident) => ident.sym.to_string(),
            _ => return None,
        },
        _ => return None,
    };

    if callee != "Derive" {
        return None;
    }

    let mut macros = Vec::new();
    for arg in &call.args {
        if arg.spread.is_some() {
            continue;
        }

        if let Some(name) = derive_name_from_expr(arg.expr.as_ref()) {
            macros.push(name);
        }
    }

    Some(macros)
}

fn derive_name_from_expr(expr: &swc_core::ecma::ast::Expr) -> Option<String> {
    use swc_core::ecma::ast::{Expr, Lit};

    match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn span_to_ir(span: Span) -> SpanIR {
    SpanIR::new(span.lo.0, span.hi.0)
}

type PackageRegistrar = fn(&MacroRegistry) -> ts_macro_host::Result<()>;

fn available_package_registrars() -> Vec<(&'static str, PackageRegistrar)> {
    vec![
        ("@macro/derive", register_builtin_macros as PackageRegistrar),
        (
            "@playground/macro",
            register_playground_macros as PackageRegistrar,
        ),
    ]
}

fn register_packages(
    registry: &MacroRegistry,
    config: &MacroConfig,
    config_root: &Path,
) -> Result<Vec<Library>> {
    let mut embedded_map: HashMap<&'static str, PackageRegistrar> =
        available_package_registrars().into_iter().collect();
    for pkg in ts_macro_host::package_registry::registrars() {
        embedded_map.entry(pkg.module).or_insert(pkg.registrar);
    }

    let mut loaded_libs = Vec::new();
    let requested = if config.macro_packages.is_empty() {
        embedded_map.keys().cloned().collect::<Vec<_>>()
    } else {
        config
            .macro_packages
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    };

    let search_roots = default_search_roots(config_root);
    let discovered_manifests = discover_manifests(&search_roots);

    for module in requested {
        if let Some(registrar) = embedded_map.get(module) {
            registrar(registry)
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register macro package {module}"))?;
            continue;
        }

        if let Some(entry) = discovered_manifests.get(module) {
            load_dynamic_package(module, entry, registry)
                .with_context(|| format!("failed to load macro package {module}"))?
                .into_iter()
                .for_each(|lib| loaded_libs.push(lib));
        } else {
            let roots = search_roots
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            eprintln!(
                "[ts-macros] warning: macro package '{}' not found (searched under {}). \
                 Ensure its macro.toml is present or adjust ts-macros.json.",
                module, roots
            );
        }
    }

    Ok(loaded_libs)
}

fn default_search_roots(config_root: &Path) -> Vec<PathBuf> {
    let mut set: HashSet<PathBuf> = HashSet::new();
    let mut roots = Vec::new();

    let mut push = |path: PathBuf| {
        if path.exists() && set.insert(path.clone()) {
            roots.push(path);
        }
    };

    push(config_root.to_path_buf());
    if let Ok(cwd) = std::env::current_dir() {
        push(cwd);
    }

    for rel in ["crates", "playground", "packages", "node_modules"] {
        push(config_root.join(rel));
    }

    roots
}

struct DiscoveredManifest {
    manifest: MacroManifest,
    manifest_path: PathBuf,
}

fn discover_manifests(roots: &[PathBuf]) -> HashMap<String, DiscoveredManifest> {
    let mut map = HashMap::new();

    for root in roots {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.file_name() != "macro.toml" {
                continue;
            }

            if let Ok(manifest) = MacroManifest::from_toml_file(entry.path())
                && let Some(module) = manifest.module.clone()
            {
                map.entry(module).or_insert_with(|| DiscoveredManifest {
                    manifest,
                    manifest_path: entry.path().to_path_buf(),
                });
            }
        }
    }

    map
}

fn load_dynamic_package(
    module: &str,
    discovered: &DiscoveredManifest,
    registry: &MacroRegistry,
) -> Result<Vec<Library>> {
    registry.register_manifest(module, discovered.manifest.clone())?;

    let mut libs = Vec::new();

    let Some(native) = &discovered.manifest.native else {
        return Ok(libs);
    };

    let lib_path = resolve_library_path(&discovered.manifest_path, &native.path)?;
    let library = unsafe { Library::new(&lib_path) }
        .with_context(|| format!("failed to load {}", lib_path.display()))?;

    unsafe {
        let registrar: libloading::Symbol<unsafe extern "C" fn(*const MacroRegistry) -> bool> =
            library.get(native.symbol.as_bytes()).with_context(|| {
                format!(
                    "failed to locate symbol '{}' in {}",
                    native.symbol,
                    lib_path.display()
                )
            })?;
        if !registrar(registry as *const MacroRegistry) {
            return Err(anyhow::anyhow!(
                "macro registrar '{}' in {} reported failure",
                native.symbol,
                lib_path.display()
            ));
        }
    }

    libs.push(library);
    Ok(libs)
}

fn resolve_library_path(manifest_path: &Path, configured: &str) -> Result<PathBuf> {
    let base_dir = manifest_path.parent().ok_or_else(|| {
        anyhow::anyhow!("manifest path {} has no parent", manifest_path.display())
    })?;
    let configured_path = base_dir.join(configured);

    if configured_path.extension().is_some() {
        return Ok(configured_path);
    }

    let stem_os = configured_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("invalid native library path '{}'", configured))?;
    let stem = stem_os.to_str().ok_or_else(|| {
        anyhow::anyhow!(
            "native library path '{}' contains invalid unicode",
            configured
        )
    })?;
    let dir = configured_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| base_dir.to_path_buf());

    let prefix = if cfg!(target_os = "windows") {
        ""
    } else {
        "lib"
    };
    #[cfg(target_os = "macos")]
    const SUFFIX: &str = ".dylib";
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    const SUFFIX: &str = ".so";
    #[cfg(target_os = "windows")]
    const SUFFIX: &str = ".dll";

    let filename = format!("{}{}{}", prefix, stem, SUFFIX);
    Ok(dir.join(filename))
}
