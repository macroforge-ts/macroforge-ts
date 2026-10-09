use std::collections::HashMap;

use crate::ts_syn::abi::{
    ClassIR, Diagnostic, DiagnosticLevel, EnumIR, FunctionIR, InterfaceIR, MethodSigIR, SpanIR,
    TypeAliasIR,
};
use crate::ts_syn::jsdoc::adjacent_jsdoc;

use super::DERIVE_MODULE_PATH;

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct SpanKey(u32, u32);

impl From<SpanIR> for SpanKey {
    fn from(span: SpanIR) -> Self {
        SpanKey(span.start, span.end)
    }
}

/// The IR for a derive target - class, interface, enum, or type alias
#[derive(Clone)]
pub(crate) enum DeriveTargetIR {
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
}

#[derive(Clone)]
pub(crate) struct DeriveTarget {
    pub macro_names: Vec<(String, String)>,
    pub decorator_span: SpanIR,
    pub target_ir: DeriveTargetIR,
}

pub(crate) fn collect_derive_targets(
    class_map: &HashMap<SpanKey, ClassIR>,
    interface_map: &HashMap<SpanKey, InterfaceIR>,
    enum_map: &HashMap<SpanKey, EnumIR>,
    type_alias_map: &HashMap<SpanKey, TypeAliasIR>,
    source: &str,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();

    // Read import sources from the registry (already built during lowering)
    let import_sources = crate::host::import_registry::with_registry(|r| r.source_modules());

    for class_ir in class_map.values() {
        collect_from_class(class_ir, source, &import_sources, &mut targets);
    }

    for interface_ir in interface_map.values() {
        collect_from_interface(interface_ir, source, &import_sources, &mut targets);
    }

    for enum_ir in enum_map.values() {
        collect_from_enum(enum_ir, source, &import_sources, &mut targets);
    }

    for type_alias_ir in type_alias_map.values() {
        collect_from_type_alias(type_alias_ir, source, &import_sources, &mut targets);
    }

    // The maps iterate in hash order; expanding in source order keeps the
    // output, including the order of generated imports, the same every run.
    targets.sort_by_key(|target| target.decorator_span.start);
    targets
}

fn collect_from_class(
    class_ir: &ClassIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    for decorator in &class_ir.decorators {
        // Only process @derive decorators, skip other decorators like @enumFieldsetController
        if !decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: decorator.span,
                target_ir: DeriveTargetIR::Class(class_ir.clone()),
            });
            return;
        }
    }

    if let Some((span, args_src)) = find_leading_derive_comment(source, class_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::Class(class_ir.clone()),
        });
    }
}

fn collect_from_interface(
    interface_ir: &InterfaceIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    for decorator in &interface_ir.decorators {
        // Only process @derive decorators, skip other decorators like @enumFieldsetController
        if !decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: decorator.span,
                target_ir: DeriveTargetIR::Interface(interface_ir.clone()),
            });
            return;
        }
    }

    if let Some((span, args_src)) = find_leading_derive_comment(source, interface_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::Interface(interface_ir.clone()),
        });
    }
}

fn collect_from_enum(
    enum_ir: &EnumIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    for decorator in &enum_ir.decorators {
        // Only process @derive decorators, skip other decorators like @enumFieldsetController
        if !decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: decorator.span,
                target_ir: DeriveTargetIR::Enum(enum_ir.clone()),
            });
            return;
        }
    }

    if let Some((span, args_src)) = find_leading_derive_comment(source, enum_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::Enum(enum_ir.clone()),
        });
    }
}

fn collect_from_type_alias(
    type_alias_ir: &TypeAliasIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    // Compute combined span for ALL adjacent decorators (to remove all JSDoc comments)
    let combined_span = if !type_alias_ir.decorators.is_empty() {
        let min_start = type_alias_ir
            .decorators
            .iter()
            .map(|d| d.span.start)
            .min()
            .unwrap_or(0);
        let max_end = type_alias_ir
            .decorators
            .iter()
            .map(|d| d.span.end)
            .max()
            .unwrap_or(0);
        Some(SpanIR::new(min_start, max_end))
    } else {
        None
    };

    for decorator in &type_alias_ir.decorators {
        // Only process @derive decorators, skip other decorators like @enumFieldsetController
        if !decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            // Use combined span to remove ALL adjacent JSDoc comments
            let decorator_span = combined_span.unwrap_or(decorator.span);

            out.push(DeriveTarget {
                macro_names,
                decorator_span,
                target_ir: DeriveTargetIR::TypeAlias(type_alias_ir.clone()),
            });
            return;
        }
    }

    if let Some((span, args_src)) = find_leading_derive_comment(source, type_alias_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::TypeAlias(type_alias_ir.clone()),
        });
    }
}

/// The 1-based span of `macro_name` among the arguments of the decorator at
/// `decorator_span`, matched as a whole identifier.
pub(crate) fn find_macro_name_span(
    source: &str,
    decorator_span: SpanIR,
    macro_name: &str,
) -> Option<SpanIR> {
    let range = decorator_span.source_range();
    let decorator_source = source.get(range.clone())?;
    let args_start = range.start + decorator_source.find('(')? + 1;
    let args = source.get(args_start..range.end)?;

    let is_ident = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    let (offset, _) = args.match_indices(macro_name).find(|(offset, _)| {
        let before = args
            .get(..*offset)
            .and_then(|text| text.chars().next_back());
        let after = args
            .get(offset + macro_name.len()..)
            .and_then(|text| text.chars().next());
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })?;

    let macro_start = u32::try_from(args_start + offset).ok()?;
    let macro_len = u32::try_from(macro_name.len()).ok()?;
    Some(SpanIR::new(macro_start + 1, macro_start + macro_len + 1))
}

/// `span` narrowed to the `@derive(...)` directive when it covers the JSDoc
/// holding one. Both spans are 1-based.
pub(crate) fn diagnostic_span_for_derive(span: SpanIR, source: &str) -> SpanIR {
    let start = span.start.saturating_sub(1) as usize;
    let end = span.end.saturating_sub(1) as usize;
    let Some(comment) = source.get(start..end.min(source.len())) else {
        return span;
    };
    if !comment.starts_with("/**") {
        return span;
    }
    comment
        .find("@derive")
        .or_else(|| comment.find("@Derive"))
        .and_then(|at| {
            comment[at..].find(')').map(|close| {
                let derive_start = (start + at) as u32 + 1;
                SpanIR::new(derive_start, derive_start + close as u32 + 1)
            })
        })
        .unwrap_or(span)
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

/// The `@derive(...)` in the JSDoc directly above a declaration starting at
/// `target_start` (1-based). A derive comment on an earlier declaration is not
/// this one's, and taking it would expand that derive twice.
fn find_leading_derive_comment(source: &str, target_start: u32) -> Option<(SpanIR, String)> {
    let block = adjacent_jsdoc(source, target_start.saturating_sub(1) as usize)?;
    let comment = &source[block.start..block.end];

    let at_derive_rel = comment
        .find("@derive")
        .or_else(|| comment.find("@Derive"))?;
    let derive_start_idx = block.start + at_derive_rel;

    let derive_close_rel = comment[at_derive_rel..].find(')')?;
    let derive_end_idx = derive_start_idx + derive_close_rel + 1;

    let content = block.body(source).trim().trim_start_matches('*').trim();
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
    Some((
        SpanIR::new(derive_start_idx as u32 + 1, derive_end_idx as u32 + 1),
        args_src,
    ))
}

// ============================================================================
// Attribute macro targets
// ============================================================================

/// The IR for an attribute target: any declaration type including functions.
#[derive(Clone)]
pub(crate) enum AttributeTargetIR {
    Function(FunctionIR),
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
}

/// A single attribute macro invocation discovered on a declaration.
#[derive(Clone)]
pub(crate) struct AttributeTarget {
    pub macro_name: String,
    pub module_path: String,
    pub decorator_span: SpanIR,
    pub target_ir: AttributeTargetIR,
}

impl AttributeTarget {
    /// Get the span of the target declaration.
    pub fn target_ir_span(&self) -> SpanIR {
        match &self.target_ir {
            AttributeTargetIR::Function(f) => f.span,
            AttributeTargetIR::Class(c) => c.span,
            AttributeTargetIR::Interface(i) => i.span,
            AttributeTargetIR::Enum(e) => e.span,
            AttributeTargetIR::TypeAlias(t) => t.span,
        }
    }
}

/// Discover attribute macro targets across all lowered items.
///
/// Checks each item's decorators against `import_sources` (populated from
/// `/** import macro { X } from "..." */` comments) and the built-in registry.
/// Any decorator whose name matches a known macro (and is not `@derive`) is
/// treated as an attribute macro invocation.
///
/// Returns the discovered targets plus any diagnostics (e.g., `@derive` on a function).
pub(crate) fn collect_attribute_targets(
    function_map: &HashMap<SpanKey, FunctionIR>,
    class_map: &HashMap<SpanKey, ClassIR>,
    interface_map: &HashMap<SpanKey, InterfaceIR>,
    enum_map: &HashMap<SpanKey, EnumIR>,
    type_alias_map: &HashMap<SpanKey, TypeAliasIR>,
    source: &str,
    import_sources: &HashMap<String, String>,
) -> (Vec<AttributeTarget>, Vec<Diagnostic>) {
    let mut targets = Vec::new();
    let mut diagnostics = Vec::new();

    // --- Functions ---
    for func_ir in function_map.values() {
        for decorator in &func_ir.decorators {
            if decorator.name.eq_ignore_ascii_case("derive") {
                diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "@derive cannot be used on function '{}'. Use @MacroName directly.",
                        func_ir.name,
                    ),
                    span: Some(decorator.span),
                    notes: vec![],
                    help: Some(
                        "Attribute macros on functions use @MacroName syntax, not @derive(MacroName)"
                            .to_string(),
                    ),
                });
                continue;
            }

            if let Some(module_path) = resolve_macro_module(&decorator.name, import_sources) {
                targets.push(AttributeTarget {
                    macro_name: decorator.name.clone(),
                    module_path,
                    decorator_span: decorator.span,
                    target_ir: AttributeTargetIR::Function(func_ir.clone()),
                });
            }
        }
    }

    // --- Class methods ---
    for class_ir in class_map.values() {
        for method in &class_ir.methods {
            collect_attribute_from_method(method, source, import_sources, &mut targets);
        }
    }

    // --- Classes (non-derive decorators) ---
    for class_ir in class_map.values() {
        collect_attribute_from_decorators(
            &class_ir.decorators,
            || AttributeTargetIR::Class(class_ir.clone()),
            import_sources,
            &mut targets,
        );
    }

    // --- Interfaces ---
    for iface_ir in interface_map.values() {
        collect_attribute_from_decorators(
            &iface_ir.decorators,
            || AttributeTargetIR::Interface(iface_ir.clone()),
            import_sources,
            &mut targets,
        );
    }

    // --- Enums ---
    for enum_ir in enum_map.values() {
        collect_attribute_from_decorators(
            &enum_ir.decorators,
            || AttributeTargetIR::Enum(enum_ir.clone()),
            import_sources,
            &mut targets,
        );
    }

    // --- Type aliases ---
    for ta_ir in type_alias_map.values() {
        collect_attribute_from_decorators(
            &ta_ir.decorators,
            || AttributeTargetIR::TypeAlias(ta_ir.clone()),
            import_sources,
            &mut targets,
        );
    }

    // The maps iterate in hash order; source order keeps expansion and its
    // diagnostics the same every run.
    targets.sort_by_key(|target| target.decorator_span.start);
    diagnostics.sort_by_key(|diagnostic| diagnostic.span.map(|span| span.start));
    (targets, diagnostics)
}

/// Check non-derive decorators on a type-like declaration for attribute macros.
/// `target_ir` builds the declaration's IR, only for a decorator that names one.
fn collect_attribute_from_decorators(
    decorators: &[crate::ts_syn::abi::DecoratorIR],
    target_ir: impl Fn() -> AttributeTargetIR,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<AttributeTarget>,
) {
    for decorator in decorators {
        if decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(module_path) = resolve_macro_module(&decorator.name, import_sources) {
            out.push(AttributeTarget {
                macro_name: decorator.name.clone(),
                module_path,
                decorator_span: decorator.span,
                target_ir: target_ir(),
            });
        }
    }
}

/// Collect attribute macro targets from a class method.
///
/// Methods with attribute decorators are presented to the macro as a
/// `FunctionIR`, matching Rust's behavior where `#[attr]` on a method
/// gives the attribute macro a function-like token stream.
fn collect_attribute_from_method(
    method: &MethodSigIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<AttributeTarget>,
) {
    for decorator in &method.decorators {
        if decorator.name.eq_ignore_ascii_case("derive") {
            continue;
        }
        if let Some(module_path) = resolve_macro_module(&decorator.name, import_sources) {
            // Build a FunctionIR from the method so the macro sees it as a function.
            let func_ir = method_to_function_ir(method, source);
            out.push(AttributeTarget {
                macro_name: decorator.name.clone(),
                module_path,
                decorator_span: decorator.span,
                target_ir: AttributeTargetIR::Function(func_ir),
            });
        }
    }
}

/// Convert a MethodSigIR into a FunctionIR for attribute macro dispatch.
fn method_to_function_ir(method: &MethodSigIR, source: &str) -> FunctionIR {
    use crate::ts_syn::abi::FunctionParamIR;

    let body_span = method.body_span.unwrap_or(method.span);
    let body_src = method.body_src.clone().unwrap_or_default();

    // Signature span: everything before the body.
    let sig_end = body_span.start;
    let signature_span = SpanIR::new(method.span.start, sig_end);

    // Parse params from params_src as a single "source string" param.
    // We don't have structured param info from MethodSigIR, so provide
    // the raw params_src as the name of a single "virtual" param.
    let params: Vec<FunctionParamIR> = if method.params_src.is_empty() {
        vec![]
    } else {
        method
            .params_src
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|p| {
                let trimmed = p.trim();
                let (name, type_src) = if let Some(colon_pos) = trimmed.find(':') {
                    (
                        trimmed[..colon_pos].trim().to_string(),
                        trimmed[colon_pos + 1..].trim().to_string(),
                    )
                } else {
                    (trimmed.to_string(), String::new())
                };
                FunctionParamIR {
                    name,
                    span: method.span,
                    type_src,
                    default_src: None,
                    is_optional: false,
                    is_rest: trimmed.starts_with("..."),
                    decorators: vec![],
                }
            })
            .collect()
    };

    // Extract the target source from the span.
    let _target_src = source
        .get(
            method.span.start.saturating_sub(1) as usize
                ..method.span.end.saturating_sub(1) as usize,
        )
        .unwrap_or("");

    FunctionIR {
        name: method.name.clone(),
        span: method.span,
        body_span,
        signature_span,
        is_async: method.is_async,
        is_generator: false,
        is_exported: false,
        is_default_export: false,
        type_params: method.type_params.clone(),
        params,
        return_type_src: method.return_type_src.clone(),
        body_src,
        decorators: method.decorators.clone(),
    }
}

/// Resolve a decorator name to its module path.
///
/// Checks the import registry first, then falls back to the built-in
/// macro registry for attribute macros.
fn resolve_macro_module(name: &str, import_sources: &HashMap<String, String>) -> Option<String> {
    // 1. Check import sources (from `/** import macro { X } from "..." */`)
    if let Some(module) = import_sources.get(name) {
        return Some(module.clone());
    }

    // 2. Check built-in macros registered with kind == Attribute
    if let Some(descriptor) = crate::host::derived::lookup_by_name(name)
        && descriptor.kind == crate::ts_syn::abi::MacroKind::Attribute
    {
        return Some(DERIVE_MODULE_PATH.to_string());
    }

    None
}
