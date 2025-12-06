//! Reusable macro expansion logic
//!
//! This module provides the core expansion functionality that can be used by any
//! macro package to implement its own `expandSync` NAPI function.

use std::collections::HashMap;

use swc_core::{common::Span, ecma::ast::Module};
use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode,
    SourceMapping, SpanIR, TargetIR,
};
use ts_syn::{lower_classes, parse_ts_module};

use crate::{MacroConfig, MacroDispatcher, MacroRegistry, PatchCollector, derived};

/// Default module path for built-in derive macros
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Result of macro expansion
#[derive(Debug, Clone)]
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
    pub type_output: Option<String>,
    pub classes: Vec<ClassIR>,
    pub source_mapping: Option<SourceMapping>,
}

/// Core macro expansion engine
///
/// This struct provides the expansion logic that can be reused by any macro package.
/// Each macro package creates its own instance, which will use that package's
/// local inventory of macros.
pub struct MacroExpander {
    pub dispatcher: MacroDispatcher,
    config: MacroConfig,
    /// Whether to keep decorators in emitted output (used only by host integrations that need mapping)
    keep_decorators: bool,
}

impl MacroExpander {
    /// Create a new expander with the local registry populated from inventory
    pub fn new() -> crate::Result<Self> {
        let (config, root_dir) = MacroConfig::find_with_root()
            .map_err(|e| crate::MacroError::InvalidConfig(format!("{:?}", e)))?
            .unwrap_or_else(|| {
                (
                    MacroConfig::default(),
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                )
            });
        Self::with_config(config, root_dir)
    }

    /// Create an expander with a specific config
    pub fn with_config(config: MacroConfig, root_dir: std::path::PathBuf) -> crate::Result<Self> {
        let registry = MacroRegistry::new();
        register_packages(&registry, &config, &root_dir)?;

        let keep_decorators = config.keep_decorators;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
            keep_decorators,
        })
    }

    /// Control whether decorators are preserved in the expanded output.
    pub fn set_keep_decorators(&mut self, keep: bool) {
        self.keep_decorators = keep;
    }

    /// Expand all macros in the source code
    pub fn expand(&self, source: &str, file_name: &str) -> crate::Result<MacroExpansion> {
        // Parse the module
        let module = parse_ts_module(source)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Parse error: {:?}", e)))?;

        // Lower classes to IR
        let classes = lower_classes(&module, source)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        if classes.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
                classes: Vec::new(),
                source_mapping: None,
            });
        }

        let classes_clone = classes.clone();

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, classes, file_name, source);

        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, classes_clone)
    }

    fn collect_macro_patches(
        &self,
        module: &Module,
        classes: Vec<ClassIR>,
        file_name: &str,
        source: &str,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();

        // Add patches to remove method bodies from type output
        for class_ir in classes.iter() {
            for method in &class_ir.methods {
                let return_type = method
                    .return_type_src
                    .trim_start()
                    .trim_start_matches(':')
                    .trim_start();
                let method_signature = if method.name == "constructor" {
                    let visibility = match method.visibility {
                        ts_macro_abi::Visibility::Private => "private ",
                        ts_macro_abi::Visibility::Protected => "protected ",
                        ts_macro_abi::Visibility::Public => "",
                    };
                    format!(
                        "{visibility}constructor({params_src});",
                        visibility = visibility,
                        params_src = method.params_src
                    )
                } else {
                    let visibility = match method.visibility {
                        ts_macro_abi::Visibility::Private => "private ",
                        ts_macro_abi::Visibility::Protected => "protected ",
                        ts_macro_abi::Visibility::Public => "",
                    };
                    let static_kw = if method.is_static { "static " } else { "" };
                    let async_kw = if method.is_async { "async " } else { "" };

                    format!(
                        "{visibility}{static_kw}{async_kw}{method_name}{type_params}({params_src}): {return_type};",
                        visibility = visibility,
                        static_kw = static_kw,
                        async_kw = async_kw,
                        method_name = method.name,
                        type_params = method.type_params_src,
                        params_src = method.params_src,
                        return_type = return_type
                    )
                };

                collector.add_type_patches(vec![Patch::Replace {
                    span: method.span,
                    code: method_signature.into(),
                    source_macro: None, // Method body stripping is internal, not macro-generated
                }]);
            }
        }

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let derive_targets = collect_derive_targets(module, &class_map, source);

        if derive_targets.is_empty() {
            return (collector, diagnostics);
        }

        for target in derive_targets {
            if !self.keep_decorators {
                let decorator_removal = Patch::Delete {
                    span: target.decorator_span,
                };
                collector.add_runtime_patches(vec![decorator_removal.clone()]);
                collector.add_type_patches(vec![decorator_removal]);
            }

            // Remove all field decorators when not keeping decorators
            if !self.keep_decorators {
                for field in &target.class_ir.fields {
                    for decorator in &field.decorators {
                        let field_dec_removal = Patch::Delete {
                            span: span_ir_with_at(decorator.span, source),
                        };
                        collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                        collector.add_type_patches(vec![field_dec_removal]);
                    }

                    // find_macro_comment_span now only returns comments directly adjacent
                    // to the field, so this won't accidentally find class-level decorators
                    if let Some(span) = find_macro_comment_span(source, field.span.start) {
                        let removal = Patch::Delete { span };
                        collector.add_runtime_patches(vec![removal.clone()]);
                        collector.add_type_patches(vec![removal]);
                    }
                }
            }

            let target_source = source
                .get(target.class_ir.span.start as usize..target.class_ir.span.end as usize)
                .unwrap_or("")
                .to_string();

            for (macro_name, module_path) in target.macro_names {
                let ctx = MacroContextIR::new_derive_class(
                    macro_name.clone(),
                    module_path.clone(),
                    target.decorator_span,
                    target.class_ir.span,
                    file_name.to_string(),
                    target.class_ir.clone(),
                    target_source.clone(),
                );

                let mut result = self.dispatcher.dispatch(ctx.clone());

                // Process potential token stream result
                if let Ok((runtime, type_def)) =
                    self.process_macro_output(&mut result, &ctx, source)
                {
                    result.runtime_patches.extend(runtime);
                    result.type_patches.extend(type_def);
                }

                if !result.diagnostics.is_empty() {
                    diagnostics.extend(result.diagnostics.clone());
                }

                collector.add_runtime_patches(result.runtime_patches);
                collector.add_type_patches(result.type_patches);
            }
        }
        (collector, diagnostics)
    }

    fn process_macro_output(
        &self,
        result: &mut MacroResult,
        ctx: &MacroContextIR,
        source: &str,
    ) -> crate::Result<(Vec<Patch>, Vec<Patch>)> {
        use swc_core::ecma::ast::ClassMember;

        let mut runtime_patches = Vec::new();
        let mut type_patches = Vec::new();

        if let Some(tokens) = &result.tokens
            && ctx.macro_kind == ts_macro_abi::MacroKind::Derive
            && let TargetIR::Class(class_ir) = &ctx.target
        {
            let chunks = split_by_markers(tokens);

            let macro_name = Some(ctx.macro_name.clone());

            for (location, code) in chunks {
                match location {
                    "above" => {
                        let patch = Patch::Insert {
                            at: SpanIR {
                                start: class_ir.span.start,
                                end: class_ir.span.start,
                            },
                            code: PatchCode::Text(code.clone()),
                            source_macro: macro_name.clone(),
                        };
                        runtime_patches.push(patch.clone());
                        type_patches.push(patch);
                    }
                    "below" => {
                        let patch = Patch::Insert {
                            at: SpanIR {
                                start: class_ir.span.end,
                                end: class_ir.span.end,
                            },
                            code: PatchCode::Text(code.clone()),
                            source_macro: macro_name.clone(),
                        };
                        runtime_patches.push(patch.clone());
                        type_patches.push(patch);
                    }
                    "signature" => {
                        let patch = Patch::Insert {
                            at: SpanIR {
                                start: class_ir.body_span.start,
                                end: class_ir.body_span.start,
                            },
                            code: PatchCode::Text(code.clone()),
                            source_macro: macro_name.clone(),
                        };
                        runtime_patches.push(patch.clone());
                        type_patches.push(patch);
                    }
                    "body" => {
                        let insert_pos = derive_insert_pos(class_ir, source);
                        match parse_members_from_tokens(&code) {
                            Ok(members) => {
                                for member in members {
                                    runtime_patches.push(Patch::Insert {
                                        at: SpanIR {
                                            start: insert_pos,
                                            end: insert_pos,
                                        },
                                        code: PatchCode::ClassMember(member.clone()),
                                        source_macro: macro_name.clone(),
                                    });

                                    let mut signature_member = member.clone();
                                    match &mut signature_member {
                                        ClassMember::Method(m) => m.function.body = None,
                                        ClassMember::Constructor(c) => c.body = None,
                                        ClassMember::PrivateMethod(m) => m.function.body = None,
                                        _ => {}
                                    }

                                    type_patches.push(Patch::Insert {
                                        at: SpanIR {
                                            start: insert_pos,
                                            end: insert_pos,
                                        },
                                        code: PatchCode::ClassMember(signature_member),
                                        source_macro: macro_name.clone(),
                                    });
                                }
                            }
                            Err(err) => {
                                return Err(crate::MacroError::InvalidConfig(format!(
                                    "Failed to parse macro output: {:?}",
                                    err
                                )));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok((runtime_patches, type_patches))
    }

    fn apply_and_finalize_expansion(
        &self,
        source: &str,
        collector: &mut PatchCollector,
        diagnostics: &mut Vec<Diagnostic>,
        classes: Vec<ClassIR>,
    ) -> crate::Result<MacroExpansion> {
        let runtime_result = collector
            .apply_runtime_patches_with_mapping(source, None)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Patch error: {:?}", e)))?;

        let type_output = if collector.has_type_patches() {
            Some(collector.apply_type_patches(source).map_err(|e| {
                crate::MacroError::InvalidConfig(format!("Type patch error: {:?}", e))
            })?)
        } else {
            None
        };

        let source_mapping = if runtime_result.mapping.is_empty() {
            None
        } else {
            Some(runtime_result.mapping)
        };

        let mut code = runtime_result.code;
        if !self.keep_decorators {
            code = strip_decorators(&code);
        }

        let mut expansion = MacroExpansion {
            code,
            diagnostics: std::mem::take(diagnostics),
            changed: true,
            type_output,
            classes,
            source_mapping,
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
                    "Adjust `limits.maxDiagnostics` in macroforge.json to see all diagnostics"
                        .to_string(),
                ),
            });
        }
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new().expect("Failed to create default MacroExpander")
    }
}

fn strip_decorators(code: &str) -> String {
    code.lines()
        .filter(|line| !line.trim_start().starts_with('@'))
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================================================
// Helper types and functions
// ============================================================================

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
    macro_names: Vec<(String, String)>,
    decorator_span: SpanIR,
    class_ir: ClassIR,
}

fn collect_import_sources(module: &Module, source: &str) -> HashMap<String, String> {
    use swc_core::ecma::ast::{ImportDecl, ImportSpecifier, ModuleDecl, ModuleItem};

    let mut import_map = HashMap::new();

    import_map.extend(collect_macro_import_comments(source));

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            specifiers, src, ..
        })) = item
        {
            let module_source = src.value.to_string_lossy().to_string();

            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local_name = named.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Default(default) => {
                        let local_name = default.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Namespace(ns) => {
                        let local_name = ns.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                }
            }
        }
    }

    import_map
}

fn collect_macro_import_comments(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut search_start = 0usize;

    while let Some(idx) = source[search_start..].find("/** import macro") {
        let abs_idx = search_start + idx;
        let remaining = &source[abs_idx..];
        let Some(end_idx) = remaining.find("*/") else {
            break;
        };
        let block = &remaining[..end_idx];

        if let Some(open_brace) = block.find('{')
            && let Some(close_brace) = block.find('}')
            && close_brace > open_brace
            && let Some(from_idx) = block[close_brace..].find("from")
        {
            let names_src = &block[open_brace + 1..close_brace];
            let from_section = &block[close_brace + from_idx + "from".len()..];
            let module_src = from_section
                .split(['"', '\''])
                .nth(1)
                .map(str::trim)
                .unwrap_or("");

            if !module_src.is_empty() {
                for name in names_src.split(',') {
                    let trimmed = name.trim();
                    if !trimmed.is_empty() {
                        out.insert(trimmed.to_string(), module_src.to_string());
                    }
                }
            }
        }

        search_start = abs_idx + end_idx + 2;
    }

    out
}

fn collect_derive_targets(
    module: &Module,
    class_map: &HashMap<SpanKey, ClassIR>,
    source: &str,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();
    let import_sources = collect_import_sources(module, source);

    for class_ir in class_map.values() {
        collect_from_class(class_ir, source, &import_sources, &mut targets);
    }

    targets
}

fn collect_from_class(
    class_ir: &ClassIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    // Prefer decorators (legacy and comment-lowered)
    for decorator in &class_ir.decorators {
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: span_ir_with_at(decorator.span, source),
                class_ir: class_ir.clone(),
            });
            return;
        }
    }

    // Fallback: detect leading /** @derive(...) */ comment directly
    if let Some((span, args_src)) = find_leading_derive_comment(source, class_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            class_ir: class_ir.clone(),
        });
    }
}

fn span_ir_with_at(span: SpanIR, source: &str) -> SpanIR {
    let mut ir = span;
    let start = ir.start as usize;
    if start > 0 && start <= source.len() {
        let bytes = source.as_bytes();
        if bytes[start - 1] == b'@' {
            ir.start -= 1;
        }
    }
    ir
}

fn parse_derive_decorator(
    args_src: &str,
    import_sources: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let args = args_src.split(',');
    let mut macros = Vec::new();
    for arg in args {
        let name = arg.trim();
        if name.is_empty() {
            continue;
        }
        let name = name.trim_matches(|c| c == '"' || c == '\'').to_string();
        if name.is_empty() {
            continue;
        }
        let module_path = import_sources
            .get(&name)
            .cloned()
            .unwrap_or_else(|| DERIVE_MODULE_PATH.to_string());
        macros.push((name, module_path));
    }

    Some(macros)
}

fn find_leading_derive_comment(source: &str, class_start: u32) -> Option<(SpanIR, String)> {
    // SWC spans are 1-based, so class_start of N means byte index N-1 (0-based)
    let start = class_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }

    let search_area = &source[..start];
    let comment_start_idx = search_area.rfind("/**")?;
    let rest = &search_area[comment_start_idx..];
    let end_rel = rest.find("*/")?;
    let comment_end_idx = comment_start_idx + end_rel + 2;

    // Find @derive within the comment for diagnostic span
    let at_derive_rel = rest.find("@derive").or_else(|| rest.find("@Derive"))?;
    let derive_start_idx = comment_start_idx + at_derive_rel;

    // Find the closing paren for the derive span end
    let derive_close_rel = rest[at_derive_rel..].find(')')?;
    let derive_end_idx = derive_start_idx + derive_close_rel + 1;

    let comment_body = &search_area[comment_start_idx + 3..comment_end_idx - 2];

    let content = comment_body.trim().trim_start_matches('*').trim();
    let content = content.strip_prefix('@')?;

    let open = content.find('(')?;
    let close = content.rfind(')')?;
    if close <= open {
        return None;
    }

    let name = content[..open].trim();
    if !name.eq_ignore_ascii_case("derive") {
        return None;
    }

    let args_src = content[open + 1..close].trim().to_string();
    // Return 1-based span pointing to @derive(...) for better diagnostics
    Some((
        SpanIR::new(derive_start_idx as u32 + 1, derive_end_idx as u32 + 1),
        args_src,
    ))
}

fn derive_insert_pos(class_ir: &ClassIR, source: &str) -> u32 {
    // class_ir.span.end is 1-based, so we can use it directly for exclusive slice end
    let end = class_ir.span.end as usize;
    let search = &source[..end.min(source.len())];
    // rfind returns 0-based index, add 1 to convert to 1-based for SpanIR
    search
        .rfind('}')
        .map(|idx| idx as u32 + 1)
        .unwrap_or_else(|| {
            // Fallback: use body_span.end (already 1-based)
            class_ir.body_span.end.max(class_ir.span.start)
        })
}

fn find_macro_comment_span(source: &str, target_start: u32) -> Option<SpanIR> {
    // SWC spans are 1-based, so target_start of 17 means byte index 16 (0-based)
    // We want to search in the source up to but NOT including the target
    let start = target_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }
    let search_area = &source[..start];
    let start_idx = search_area.rfind("/**")?;
    let rest = &search_area[start_idx..];
    let end_rel = rest.find("*/")?;
    let end_idx = start_idx + end_rel + 2;

    // Only return the comment if it's directly adjacent to the target
    // (only whitespace between comment end and target start)
    let between = &search_area[end_idx..];
    if !between.trim().is_empty() {
        // There's non-whitespace content between the comment and target
        return None;
    }

    // Return 1-based span to match SWC conventions
    Some(SpanIR::new(start_idx as u32 + 1, end_idx as u32 + 1))
}

fn split_by_markers(source: &str) -> Vec<(&str, String)> {
    let markers = [
        ("above", "/* @macroforge:above */"),
        ("below", "/* @macroforge:below */"),
        ("body", "/* @macroforge:body */"),
        ("signature", "/* @macroforge:signature */"),
    ];

    // Find all occurrences
    let mut occurrences = Vec::new();
    for (name, pattern) in markers {
        for (idx, _) in source.match_indices(pattern) {
            occurrences.push((idx, pattern.len(), name));
        }
    }
    occurrences.sort_by_key(|k| k.0);

    if occurrences.is_empty() {
        return vec![("below", source.to_string())];
    }

    let mut chunks = Vec::new();

    // Handle text before first marker (default to below)
    if occurrences[0].0 > 0 {
        let text = &source[0..occurrences[0].0];
        if !text.trim().is_empty() {
            chunks.push(("below", text.to_string()));
        }
    }

    for i in 0..occurrences.len() {
        let (start, len, name) = occurrences[i];
        let content_start = start + len;
        let content_end = if i + 1 < occurrences.len() {
            occurrences[i + 1].0
        } else {
            source.len()
        };

        let content = &source[content_start..content_end];
        chunks.push((name, content.to_string()));
    }

    chunks
}

fn parse_members_from_tokens(tokens: &str) -> crate::Result<Vec<swc_core::ecma::ast::ClassMember>> {
    let wrapped_stmt = format!("class __Temp {{ {} }}", tokens);
    if let Ok(swc_core::ecma::ast::Stmt::Decl(swc_core::ecma::ast::Decl::Class(class_decl))) =
        ts_syn::parse_ts_stmt(&wrapped_stmt)
    {
        return Ok(class_decl.class.body);
    }

    // Fallback: parse as module and grab first class
    if let Ok(module) = ts_syn::parse_ts_module(&wrapped_stmt) {
        for item in module.body {
            if let swc_core::ecma::ast::ModuleItem::Stmt(swc_core::ecma::ast::Stmt::Decl(
                swc_core::ecma::ast::Decl::Class(class_decl),
            )) = item
            {
                return Ok(class_decl.class.body);
            }
        }
    }

    Err(crate::MacroError::InvalidConfig(
        "Failed to parse macro output into class members".into(),
    ))
}

// ============================================================================
// Package registration
// ============================================================================

/// Special marker for dynamic module resolution
const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

fn register_packages(
    registry: &MacroRegistry,
    config: &MacroConfig,
    _config_root: &std::path::Path,
) -> crate::Result<()> {
    use crate::package_registry;

    let mut embedded_map: HashMap<&'static str, fn(&MacroRegistry) -> crate::Result<()>> =
        HashMap::new();

    for pkg in package_registry::registrars() {
        embedded_map.entry(pkg.module).or_insert(pkg.registrar);
    }

    let derived_modules = derived::modules();
    let derived_set: std::collections::HashSet<&'static str> =
        derived_modules.iter().copied().collect();

    let mut requested = if config.macro_packages.is_empty() {
        embedded_map.keys().cloned().collect::<Vec<_>>()
    } else {
        config
            .macro_packages
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    };

    if config.macro_packages.is_empty() {
        requested.extend(derived_modules.iter().copied());
    }

    requested.sort();
    requested.dedup();

    for module in requested {
        let mut found = false;

        if let Some(registrar) = embedded_map.get(module) {
            registrar(registry)?;
            found = true;
        }

        if derived_set.contains(module) {
            derived::register_module(module, registry)?;
            found = true;
        }

        if !found {
            // Module not found - this is a warning, not an error
        }
    }

    // Register dynamic-module macros under the default derive path
    if derived_set.contains(DYNAMIC_MODULE_MARKER) {
        let _ = derived::register_module(DYNAMIC_MODULE_MARKER, registry);
    }

    for entry in inventory::iter::<derived::DerivedMacroRegistration> {
        let descriptor = entry.descriptor;
        if descriptor.module == DYNAMIC_MODULE_MARKER {
            let _ = registry.register(
                DERIVE_MODULE_PATH,
                descriptor.name,
                (descriptor.constructor)(),
            );
        }
    }

    Ok(())
}
