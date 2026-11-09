//! How a field or alias gets its default value.

use std::cell::RefCell;

use crate::builtin::derive::common::function_prefix;

use super::literal::decode_literal_default;
use crate::builtin::derive::common::{
    Declared, DefaultFieldOptions, TypeNames, entry_derives, extract_named_string,
    fields_from_definition, get_type_default_with_registry, is_primitive_type, js_string, mentions,
    rendered, standalone_fn_name,
};
use crate::builtin::derive::common::{array_element_type, tuple_body, undefaultable};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::{
    FileImportEntry, TypeBody, TypeDefinitionIR, TypeMemberKind, TypeRegistry, TypeRegistryEntry,
    resolve_generic_aliases,
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

impl<'a> DefaultSources<'a> {
    /// Where the names in the type being defaulted resolve.
    pub(super) fn declared(&self) -> Declared<'a> {
        Declared {
            registry: self.registry,
            file_path: self.caller_file_path,
            file_imports: self.file_imports,
        }
    }
}

/// How the default of a field typed `ts_type` comes to need a default of
/// its own type `owner` again, when it does: nothing for a field that holds
/// an `owner` itself, and the fields of the other types it goes through
/// otherwise. `None` for a field whose default ends.
fn endless_default(owner: &str, ts_type: &str, sources: &DefaultSources) -> Option<String> {
    let mut through = Vec::new();
    defaults_back_to(
        (sources.caller_file_path, owner),
        ts_type,
        sources.declared(),
        &mut through,
        &mut Vec::new(),
    )
    .then(|| match through.as_slice() {
        [] => String::new(),
        fields => format!(", through {}", fields.join(" and ")),
    })
}

/// Whether the default of a value typed `ts_type`, read where `declared`
/// resolves its names, calls the default of the type `owner` names by its
/// file and name. `through` gathers the fields it goes by, and `seen` the
/// types already followed.
fn defaults_back_to(
    owner: (&str, &str),
    ts_type: &str,
    declared: Declared<'_>,
    through: &mut Vec<String>,
    seen: &mut Vec<(String, String)>,
) -> bool {
    for (name, entry) in defaulted_types(ts_type, declared) {
        // Expanded on its own, a file's types are not in the registry: the
        // owner is still known by its name.
        let Some(entry) = entry else {
            if name == owner.1 {
                return true;
            }
            continue;
        };
        if (entry.file_path.as_str(), entry.name.as_str()) == owner {
            return true;
        }
        let key = (entry.file_path.clone(), entry.name.clone());
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        let within = Declared {
            registry: declared.registry,
            file_path: &entry.file_path,
            file_imports: &entry.file_imports,
        };
        for (field, field_type) in defaulting_fields(entry, declared.registry) {
            through.push(format!("`{}.{field}`", entry.name));
            if defaults_back_to(owner, &field_type, within, through, seen) {
                return true;
            }
            through.pop();
        }
    }
    false
}

/// The types whose own defaults the default of a value typed `ts_type`
/// calls, read off the default that is written for it: each by the name the
/// type gives it, with its declaration where the registry holds one.
fn defaulted_types<'t, 'r>(
    ts_type: &'t str,
    declared: Declared<'r>,
) -> Vec<(&'t str, Option<&'r TypeRegistryEntry>)> {
    let written = get_type_default_with_registry(
        ts_type,
        declared.registry,
        declared.file_path,
        declared.file_imports,
    );
    ts_type
        .split(|letter: char| !(letter.is_alphanumeric() || matches!(letter, '_' | '$')))
        .filter(|name| !name.is_empty())
        .filter(|name| mentions(&written, &standalone_fn_name(name, "DefaultValue")))
        .map(|name| {
            let entry =
                declared
                    .registry
                    .resolve_in_file(name, declared.file_path, declared.file_imports);
            (name, entry)
        })
        .collect()
}

/// The fields of the type `entry` declares that its default fills with the
/// default of their own type, as their names and types: the ones it has to
/// have, without a `@default` or an initializer of their own. None for a
/// type that derives no `Default`.
fn defaulting_fields(entry: &TypeRegistryEntry, registry: &TypeRegistry) -> Vec<(String, String)> {
    if !entry_derives(entry, "Default") {
        return Vec::new();
    }
    fn undeclared(decorators: &[crate::ts_syn::abi::DecoratorIR]) -> bool {
        !DefaultFieldOptions::from_decorators(decorators).has_default
    }
    match &entry.definition {
        TypeDefinitionIR::Class(class) => class
            .fields
            .iter()
            .filter(|field| {
                !field.optional && field.initializer.is_none() && undeclared(&field.decorators)
            })
            .map(|field| (field.name.clone(), field.ts_type.clone()))
            .collect(),
        TypeDefinitionIR::TypeAlias(alias) if alias.body.as_alias().is_some() => alias
            .body
            .as_alias()
            .map(|aliased| (alias.name.clone(), aliased.to_string()))
            .into_iter()
            .collect(),
        TypeDefinitionIR::Interface(_) | TypeDefinitionIR::TypeAlias(_) => {
            fields_from_definition(entry, registry)
                .unwrap_or_default()
                .into_iter()
                .filter(|field| !field.optional && undeclared(&field.decorators))
                .map(|field| (field.name, field.ts_type))
                .collect()
        }
        TypeDefinitionIR::Enum(_) => Vec::new(),
    }
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

/// Whether `expression` is several expressions with commas between them,
/// which read as one value nowhere a default is written out.
fn writes_several(expression: &str) -> bool {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::Expression;

    let allocator = Allocator::default();
    let parser = oxc::parser::Parser::new(&allocator, expression, oxc::span::SourceType::ts());
    // An unparseable default is reported where it is parsed.
    parser
        .parse_expression()
        .is_ok_and(|parsed| matches!(parsed, Expression::SequenceExpression(_)))
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
        if writes_several(&v) {
            sources.rejected.borrow_mut().push(format!(
                "@default({v}) writes several values, and a default is one; write an object or an array in its brackets"
            ));
            return "undefined".to_string();
        }
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
        .find_map(|d| (d.name == "endec").then(|| extract_named_string(&d.args_src, "tag")))
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
                let camel = function_prefix(p.split('<').next().unwrap_or(&p).trim());
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

/// An error at each required field with no default to fall back on, as
/// Rust's derive requires a default for every field: a field typed by one of
/// the type's `params`, which would need a `T: Default` bound, or by a type
/// the registry knows but which does not derive `Default`. `fields` holds
/// each field's name, type and span.
pub(super) fn missing_default_derives<'f>(
    fields: impl Iterator<Item = (&'f str, &'f str, SpanIR)>,
    names: &TypeNames,
    sources: &DefaultSources,
    diagnostics: &mut DiagnosticCollector,
) {
    let parent_name = &names.type_name;
    let registry = sources.registry;
    for (field_name, written, span) in fields {
        // A generic alias defaults as what it stands for, which is what the
        // default written for the field resolves it to.
        let resolved = resolve_generic_aliases(
            written.trim(),
            registry,
            sources.caller_file_path,
            sources.file_imports,
        );
        let ts_type = resolved.trim();
        if names.params.iter().any(|param| param == ts_type) {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' has type parameter '{ts_type}', which has no default; add @default(value)"
                ),
            );
            continue;
        }
        if let Some(through) = endless_default(parent_name, written, sources) {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' needs a default of '{parent_name}' itself{through}, so a default of it would never end; make the field optional or nullable, or add @default(value)"
                ),
            );
            continue;
        }
        let declared = sources.declared();
        if let Some(reason) = undefaultable(ts_type, declared) {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' {reason}; add @default(value)"
                ),
            );
            continue;
        }
        if is_primitive_type(ts_type)
            || array_element_type(ts_type).is_some()
            || tuple_body(ts_type).is_some()
            || ts_type.starts_with("Map<")
            || ts_type.starts_with("Set<")
            || ts_type == "Date"
            || ts_type.contains('|')
        {
            continue;
        }
        // A generic type is known, and derives, by its name.
        let type_name = ts_type.split_once('<').map_or(ts_type, |(base, _)| base);
        let derives_default = registry
            .resolve_in_file(type_name, sources.caller_file_path, sources.file_imports)
            .map(|entry| entry_derives(entry, "Default"));
        if derives_default == Some(false) {
            diagnostics.error(
                span,
                format!(
                    "@derive(Default) on '{parent_name}': field '{field_name}' has type '{}', which does not derive Default; derive it or add @default(value)",
                    written.trim()
                ),
            );
        }
    }
}
