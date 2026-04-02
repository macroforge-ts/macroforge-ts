use std::collections::HashMap;

use crate::ts_syn::abi::{ClassIR, EnumIR, InterfaceIR, SpanIR, TypeAliasIR};
use swc_core::common::Span;

use super::DERIVE_MODULE_PATH;

#[derive(Hash, PartialEq, Eq)]
pub(super) struct SpanKey(u32, u32);

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

/// The IR for a derive target - class, interface, enum, or type alias
#[derive(Clone)]
pub(super) enum DeriveTargetIR {
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
}

#[derive(Clone)]
pub(super) struct DeriveTarget {
    pub macro_names: Vec<(String, String)>,
    pub decorator_span: SpanIR,
    pub target_ir: DeriveTargetIR,
}

pub(super) fn collect_derive_targets(
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
                decorator_span: span_ir_with_at(decorator.span, source),
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
                decorator_span: span_ir_with_at(decorator.span, source),
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
                decorator_span: span_ir_with_at(decorator.span, source),
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
            let decorator_span = combined_span
                .map(|s| span_ir_with_at(s, source))
                .unwrap_or_else(|| span_ir_with_at(decorator.span, source));

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

pub(super) fn span_ir_with_at(span: SpanIR, source: &str) -> SpanIR {
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

pub(super) fn find_macro_name_span(
    source: &str,
    decorator_span: SpanIR,
    macro_name: &str,
) -> Option<SpanIR> {
    let start = decorator_span.start.saturating_sub(1) as usize;
    let end = decorator_span.end.saturating_sub(1) as usize;

    if start >= source.len() || end > source.len() {
        return None;
    }

    let decorator_source = &source[start..end];

    let paren_start = decorator_source.find('(')?;
    let args_slice = &decorator_source[paren_start + 1..];

    let mut search_start = 0;
    while let Some(pos) = args_slice[search_start..].find(macro_name) {
        let abs_pos = search_start + pos;

        let before_ok = abs_pos == 0
            || !args_slice
                .chars()
                .nth(abs_pos - 1)
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
        let after_ok = abs_pos + macro_name.len() >= args_slice.len()
            || !args_slice
                .chars()
                .nth(abs_pos + macro_name.len())
                .is_some_and(|c| c.is_alphanumeric() || c == '_');

        if before_ok && after_ok {
            let macro_start = start + paren_start + 1 + abs_pos;
            let macro_end = macro_start + macro_name.len();
            return Some(SpanIR::new(macro_start as u32 + 1, macro_end as u32 + 1));
        }

        search_start = abs_pos + 1;
    }

    None
}

pub(super) fn diagnostic_span_for_derive(span: SpanIR, source: &str) -> SpanIR {
    let start = span.start.saturating_sub(1) as usize;
    let end = span.end.saturating_sub(1) as usize;

    if start >= source.len() {
        return SpanIR::new(span.start.saturating_sub(1), span.end.saturating_sub(1));
    }

    if source[start..].starts_with("/**") {
        let comment_slice = &source[start..end.min(source.len())];
        if let Some(at_pos) = comment_slice
            .find("@derive")
            .or_else(|| comment_slice.find("@Derive"))
            && let Some(close_pos) = comment_slice[at_pos..].find(')')
        {
            let derive_start = start + at_pos;
            let derive_end = derive_start + close_pos + 1;
            return SpanIR::new(derive_start as u32, derive_end as u32);
        }
    }

    SpanIR::new(span.start.saturating_sub(1), span.end.saturating_sub(1))
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
    let start = class_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }

    let search_area = &source[..start];
    let comment_start_idx = search_area.rfind("/**")?;
    let rest = &search_area[comment_start_idx..];
    let end_rel = rest.find("*/")?;
    let comment_end_idx = comment_start_idx + end_rel + 2;

    let at_derive_rel = rest.find("@derive").or_else(|| rest.find("@Derive"))?;
    let derive_start_idx = comment_start_idx + at_derive_rel;

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
    Some((
        SpanIR::new(derive_start_idx as u32 + 1, derive_end_idx as u32 + 1),
        args_src,
    ))
}
