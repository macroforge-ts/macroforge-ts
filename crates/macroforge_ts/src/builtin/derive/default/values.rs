//! How a field or alias gets its default value.

use std::cell::RefCell;

use convert_case::{Case, Casing};

use super::literal::decode_literal_default;
use crate::builtin::derive::common::{
    TypeNames, get_type_default_with_registry, is_primitive_type, js_string, rendered,
    type_has_derive,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::{
    FileImportEntry, TypeBody, TypeDefinitionIR, TypeMemberKind, TypeRegistry,
};
use crate::ts_syn::abi::{DiagnosticCollector, SpanIR};
use crate::ts_syn::ts_ident;

/// Where defaults are looked up: the project's types, as the file being
/// expanded sees them.
pub(super) struct DefaultSources<'a> {
    pub(super) registry: &'a TypeRegistry,
    pub(super) caller_file_path: &'a str,
    pub(super) file_imports: &'a [FileImportEntry],
    /// Why a written default cannot be used, found while resolving it.
    pub(super) rejected: RefCell<Vec<String>>,
}

/// Whether `expression` reads the `this` of the code around it. A function
/// or class inside it has a `this` of its own; an arrow function does not.
pub(super) fn reads_this(expression: &str) -> bool {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::{Class, Function, ThisExpression};
    use oxc::ast_visit::{Visit, walk};
    use oxc::semantic::ScopeFlags;

    struct ThisFinder {
        own_this_depth: u32,
        found: bool,
    }
    impl<'a> Visit<'a> for ThisFinder {
        fn visit_this_expression(&mut self, this: &ThisExpression) {
            self.found |= self.own_this_depth == 0;
            walk::walk_this_expression(self, this);
        }
        fn visit_function(&mut self, function: &Function<'a>, flags: ScopeFlags) {
            self.own_this_depth += 1;
            walk::walk_function(self, function, flags);
            self.own_this_depth -= 1;
        }
        fn visit_class(&mut self, class: &Class<'a>) {
            self.own_this_depth += 1;
            walk::walk_class(self, class);
            self.own_this_depth -= 1;
        }
    }

    let allocator = Allocator::default();
    let parser = oxc::parser::Parser::new(&allocator, expression, oxc::span::SourceType::ts());
    // An unparseable initializer is reported where the default is parsed.
    parser.parse_expression().is_ok_and(|parsed| {
        let mut finder = ThisFinder {
            own_this_depth: 0,
            found: false,
        };
        finder.visit_expression(&parsed);
        finder.found
    })
}

/// Resolve the default expression for a field, wrapping a user-provided
/// string into `{ [tag]: "value" }` when the field type is a registered
/// internally-tagged union (e.g. `@default("Fixed")` on a field typed
/// `PricingMode = { variant: "Fixed" } | ({ variant: "PerWeight" } & ...)`).
/// Without the wrap the bare string fails type-checking.
pub(super) fn resolve_default_value(
    opts_value: Option<String>,
    ts_type: &str,
    sources: &DefaultSources,
) -> String {
    if let Some(v) = opts_value {
        if let Some(wrapped) = wrap_string_for_tagged_union(&v, ts_type, sources) {
            return wrapped;
        }
        return decode_literal_default(v, ts_type, sources);
    }
    get_type_default_with_registry(
        ts_type,
        sources.registry,
        sources.caller_file_path,
        sources.file_imports,
    )
}

/// Returns `Some(wrapped)` when `value` is a string literal and `ts_type`
/// is a type alias whose body is an internally-tagged union containing a
/// variant with that discriminant value. The wrap takes the form
/// `({ [tag]: "value" })` for unit variants, or
/// `({ [tag]: "value", ...payloadDefaultValue() })` for variants whose
/// member is `{ [tag]: "value" } & PayloadType`.
fn wrap_string_for_tagged_union(
    value: &str,
    ts_type: &str,
    sources: &DefaultSources,
) -> Option<String> {
    let trimmed = value.trim();
    let is_string_literal = (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''));
    if !is_string_literal || trimmed.len() < 2 {
        return None;
    }
    let variant_name = &trimmed[1..trimmed.len() - 1];

    let alias_name = ts_type.trim().split('<').next()?.trim();
    let entry = sources.registry.resolve_in_file(
        alias_name,
        sources.caller_file_path,
        sources.file_imports,
    )?;
    let alias = match &entry.definition {
        TypeDefinitionIR::TypeAlias(a) => a,
        _ => return None,
    };
    let members = match &alias.body {
        TypeBody::Union(m) => m,
        _ => return None,
    };

    let tag = alias
        .decorators
        .iter()
        .find_map(|d| (d.name == "endec").then(|| extract_tag_from_endec_args(&d.args_src)))
        .flatten()?;

    for member in members {
        let (tag_fields, payload_type): (
            Option<&[crate::ts_syn::InterfaceFieldIR]>,
            Option<String>,
        ) = match &member.kind {
            TypeMemberKind::Object { fields } => (Some(fields.as_slice()), None),
            TypeMemberKind::Intersection(parts) => {
                let mut tf: Option<&[crate::ts_syn::InterfaceFieldIR]> = None;
                let mut pt: Option<String> = None;
                for p in parts {
                    if let Some(fields) = p.as_object() {
                        tf = Some(fields);
                    }
                    if let Some(tr) = p.as_type_ref() {
                        pt = Some(tr.trim().to_string());
                    }
                }
                (tf, pt)
            }
            _ => continue,
        };
        let fields = tag_fields?;
        let tag_field = fields.iter().find(|f| f.name == tag)?;
        let lit = tag_field
            .ts_type
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if lit == variant_name {
            return Some(if let Some(p) = payload_type {
                let camel = p
                    .split('<')
                    .next()
                    .unwrap_or(&p)
                    .trim()
                    .to_case(Case::Camel);
                let (tag, variant) = (js_string(&tag), js_string(variant_name));
                let payload_default = ts_ident!("{}DefaultValue", camel);
                rendered(ts_template! { ({ @{tag}: @{variant}, ...@{payload_default}() }) })
            } else {
                let (tag, variant) = (js_string(&tag), js_string(variant_name));
                rendered(ts_template! { ({ @{tag}: @{variant} }) })
            });
        }
    }
    None
}

/// True when `s` contains a `|` at top level (depth 0 with respect to
/// matching parens / brackets / braces / angle brackets). Used to tell a
/// parenthesized union (`(string | T)`): which we reject: from a
/// parenthesized intersection (`({ tag } & T)`): which is fine.
pub(super) fn contains_top_level_pipe(s: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            '|' if depth <= 1 => return true,
            _ => {}
        }
    }
    false
}

/// Extract `"value"` out of `tag: "value"` or `tag = "value"` inside the
/// raw arguments of `@endec(...)`. Returns the unquoted variant tag name.
fn extract_tag_from_endec_args(args: &str) -> Option<String> {
    let s = args.trim();
    let idx = s.find("tag")?;
    let after = &s[idx + 3..];
    let after = after.trim_start();
    let after = after
        .strip_prefix(':')
        .or_else(|| after.strip_prefix('='))?
        .trim_start();
    let quote = after.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let rest = &after[1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

/// An error at each required field with no default to fall back on, as
/// Rust's derive requires a default for every field: a field typed by one of
/// the type's `params`, which would need a `T: Default` bound, or by a type
/// the registry knows but which does not derive `Default`. `fields` holds
/// each field's name, type and span.
pub(super) fn missing_default_derives<'f>(
    fields: impl Iterator<Item = (&'f str, &'f str, SpanIR)>,
    names: &TypeNames,
    registry: &TypeRegistry,
    diagnostics: &mut DiagnosticCollector,
) {
    let parent_name = &names.type_name;
    for (field_name, ts_type, span) in fields {
        let ts_type = ts_type.trim();
        if names.params.iter().any(|param| param == ts_type) {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' has type parameter '{ts_type}', which has no default; add @default(value)"
                ),
            );
            continue;
        }
        if is_primitive_type(ts_type)
            || ts_type.ends_with("[]")
            || ts_type.starts_with("Array<")
            || ts_type.starts_with("Map<")
            || ts_type.starts_with("Set<")
            || ts_type == "Date"
            || ts_type.contains('|')
        {
            continue;
        }
        let type_known =
            registry.get(ts_type).is_some() || registry.get_all(ts_type).next().is_some();
        if type_known && !type_has_derive(registry, ts_type, "Default") {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' has type '{ts_type}', which does not derive Default; derive it or add @default(value)"
                ),
            );
        }
    }
}
