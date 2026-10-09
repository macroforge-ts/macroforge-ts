use crate::ast::Expr;
use crate::ts_syn::TsSynError;
use crate::ts_syn::config::ForeignHandler;

use convert_case::{Case, Casing};

use super::super::value_kind::is_ts_literal;
use super::super::{EndecFieldOptions, TypeCategory, ValidatorSpec, get_foreign_types};
use super::types::DecodeField;
use super::validation::Missing;
use crate::builtin::derive::common::{js_string, rendered};
use crate::host::ForeignTypeConfig;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_alias::{TypeBody, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeDefinitionIR, TypeRegistry};
use crate::ts_syn::abi::ir::{
    is_primitive_keyword, split_top_level_intersection, split_top_level_union, typeof_primitive,
};

/// Determines whether a TypeScript type can accept a raw `string` value,
/// using the type registry and foreign type configs to resolve types.
/// `type_name` is resolved as the file at `caller_file_path` sees it.
///
/// When true, the generated `Decode` wrapper must NOT call `JSON.parse`
/// on string inputs -- the string IS the value, not a JSON-encoded payload.
pub(super) fn type_accepts_string(
    type_name: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    foreign_types: &[ForeignTypeConfig],
) -> bool {
    accepts_string_visiting(
        type_name,
        registry,
        caller_file_path,
        file_imports,
        foreign_types,
        &mut Vec::new(),
    )
}

/// `type_accepts_string` over the aliases already being followed, keyed by
/// declaring file and name. An alias reached again is a cycle, which the
/// expander reports at the derive; here it just stops the recursion.
fn accepts_string_visiting(
    type_name: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    foreign_types: &[ForeignTypeConfig],
    visiting: &mut Vec<String>,
) -> bool {
    // Primitive string keyword
    if type_name == "string" {
        return true;
    }

    // Check foreign types -- their hasShape expr tells us what values they accept.
    // e.g. Utc has `hasShape: (v) => typeof v === "string"` -> accepts strings.
    for ft in foreign_types {
        if ft.get_type_name() == type_name
            || ft.name == type_name
            || ft.aliases.iter().any(|a| a.name == type_name)
        {
            if let Some(has_shape) = &ft.has_shape_expr
                && has_shape.contains("typeof")
                && has_shape.contains("\"string\"")
            {
                return true;
            }
            // No hasShape or it doesn't check for string -> not a string type
            return false;
        }
    }

    let Some(entry) = registry.resolve_in_file(type_name, caller_file_path, file_imports) else {
        return false;
    };
    // Names inside the definition resolve from the file that declares it.
    let declaring_file = entry.file_path.as_str();
    let declaring_imports = entry.file_imports.as_slice();
    let key = format!("{declaring_file}::{}", entry.name);
    if visiting.contains(&key) {
        return false;
    }
    visiting.push(key);

    let accepts = match &entry.definition {
        TypeDefinitionIR::TypeAlias(alias) => match &alias.body {
            body if body.primitive_base().is_some() => body.primitive_base() == Some("string"),
            // Union: check if any member is string, a string literal, or a foreign string type
            TypeBody::Union(members) => members.iter().any(|member| match &member.kind {
                TypeMemberKind::TypeRef(referenced) => accepts_string_visiting(
                    referenced,
                    registry,
                    declaring_file,
                    declaring_imports,
                    foreign_types,
                    visiting,
                ),
                TypeMemberKind::Literal(lit) => lit.starts_with('"') || lit.starts_with('\''),
                TypeMemberKind::Object { .. }
                | TypeMemberKind::Intersection(_)
                | TypeMemberKind::Brand(_) => false,
            }),
            // Simple alias: recurse
            TypeBody::Alias(target) => accepts_string_visiting(
                target,
                registry,
                declaring_file,
                declaring_imports,
                foreign_types,
                visiting,
            ),
            _ => false,
        },
        // Enums with string members accept strings
        TypeDefinitionIR::Enum(enum_ir) => enum_ir
            .variants
            .iter()
            .any(|variant| variant.value.is_string()),
        // Classes and interfaces are always objects
        TypeDefinitionIR::Class(_) | TypeDefinitionIR::Interface(_) => false,
    };
    visiting.pop();
    accepts
}

/// Reads the validators declared on the primitive arm of a generic record-link
/// alias (`Alias<T> = primitive | T`). The alias's primitive arm can carry
/// `@endec` validators (e.g. `nonEmpty`) that don't appear on a field
/// referencing the alias, so resolve the alias by name and read them directly.
///
/// `ts_type` is the field's original (pre-resolution) type reference, resolved
/// as the file at `caller_file_path` imports it, and `primitive` is the
/// primitive keyword detected for the union (e.g. `"string"`). Two files can
/// declare same-named aliases with different validators, so resolving by name
/// alone would pick one arbitrarily. Returns an empty list when the name
/// doesn't resolve to a union alias or the primitive arm carries no validators.
pub(super) fn alias_primitive_arm_validators(
    ts_type: &str,
    primitive: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> Vec<ValidatorSpec> {
    let base = extract_base_type(ts_type);
    let Some(entry) = registry.resolve_in_file(&base, caller_file_path, file_imports) else {
        return Vec::new();
    };
    let TypeDefinitionIR::TypeAlias(alias) = &entry.definition else {
        return Vec::new();
    };
    let TypeBody::Union(members) = &alias.body else {
        return Vec::new();
    };
    for member in members {
        if let TypeMemberKind::TypeRef(t) = &member.kind
            && t == primitive
        {
            return EndecFieldOptions::from_decorators(&member.decorators, "")
                .options
                .validators;
        }
    }
    Vec::new()
}

/// How a raw value is checked against a primitive-like field type: the
/// condition under which it does not match, and the error to report.
pub(super) struct PrimitiveCheck {
    pub(super) mismatch: Expr,
    pub(super) message: Expr,
}

/// The check for a field whose type is primitive-like: a primitive keyword,
/// a literal, `null`, `undefined`, or a union of those. `None` for any other
/// type, which has no single runtime test. A missing value where the type
/// requires one is reported as required rather than as the wrong type.
pub(super) fn primitive_check(
    field: &DecodeField,
    raw: &str,
    context_name: &str,
) -> Option<PrimitiveCheck> {
    let ts_type = field.ts_type.trim();
    let parts = split_top_level_union(ts_type).unwrap_or_else(|| vec![ts_type]);
    let accepts = parts
        .iter()
        .map(|part| {
            let part = branded_base(part.trim());
            if typeof_primitive(part).is_some() || part == "symbol" {
                let kind = js_string(part);
                Some(rendered(ts_template! { typeof @{raw} === @{kind} }))
            } else if is_primitive_keyword(part) || is_ts_literal(part) {
                Some(rendered(ts_template! { @{raw} === @{part} }))
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()?;
    let wrong_type = js_string(&format!("expected {ts_type}"));
    let message = match field.missing() {
        Missing::Required => {
            let required = js_string(&format!("{context_name}.{} is required", field.json_key));
            rendered(ts_template! { @{raw} == null ? @{required} : @{wrong_type} })
        }
        Missing::Allowed | Missing::Excluded => wrong_type,
    };
    let accepts = accepts.join(" || ");
    Some(PrimitiveCheck {
        mismatch: Expr::parse(&rendered(ts_template! { !(@{accepts}) }))
            .expect("primitive check should parse"),
        message: Expr::parse(&message).expect("primitive check message should parse"),
    })
}

/// The primitive under a brand (`number & { readonly [B]: true }` or
/// `string & { __brand: "Id" }`), which is all a primitive value carries at
/// runtime; any other type is returned unchanged.
fn branded_base(ts_type: &str) -> &str {
    let Some(parts) = split_top_level_intersection(ts_type) else {
        return ts_type;
    };
    let mut bases = parts.iter().filter(|part| !part.starts_with('{'));
    match (bases.next(), bases.next()) {
        (Some(base), None) if typeof_primitive(base).is_some() => base,
        _ => ts_type,
    }
}

pub(super) fn parse_default_expr(expr_src: &str) -> Result<Expr, TsSynError> {
    let expr = Expr::parse(expr_src)?;
    if matches!(expr, Expr::Ident(_)) {
        let literal_src = format!("{expr_src:?}");
        return Expr::parse(&literal_src);
    }
    Ok(expr)
}

/// Tries to generate a composite decode expression for types where a foreign
/// type is nested inside nullable or array wrappers (e.g., `Utc[] | null`, `Utc | null`).
///
/// Splits the type by `|`, strips `null`/`undefined`, detects `[]` arrays,
/// and matches the element type against configured foreign types.
pub(super) fn try_composite_foreign_decode(ts_type: &str) -> Option<String> {
    let foreign_types = get_foreign_types();

    // Split by | and classify parts
    let parts: Vec<&str> = ts_type
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let is_nullable = parts.iter().any(|s| *s == "null" || *s == "undefined");
    let non_null: Vec<&str> = parts
        .iter()
        .filter(|s| **s != "null" && **s != "undefined")
        .copied()
        .collect();

    // Only handle single non-null type for now
    if non_null.len() != 1 {
        return None;
    }

    let core = non_null[0];

    // Check for container types
    let (container, elem_type) = if let Some(inner) = core.strip_suffix("[]") {
        ("array", inner.trim())
    } else if let Some(rest) = core.strip_prefix("Array<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("array", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Set<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("set", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Map<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("map", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Record<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("record", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some((_, value)) = crate::ts_syn::type_normalize::parse_index_signature(core) {
        // Inline TS index-signature `{ [k: K]: V }` is the structural
        // equivalent of `Record<K, V>`; recognise it so the per-value
        // foreign-type decoder runs the same way for both spellings.
        ("record", value)
    } else {
        ("none", core)
    };

    // Try matching the element type against foreign types
    let ft_match = TypeCategory::match_foreign_type(elem_type, &foreign_types);
    let rewritten = ft_match
        .config
        .and_then(|ft| ft.handler_callee(ForeignHandler::Decode))?;

    let decoded = match container {
        "array" => ts_template! { (raw as any[]).map(item => @{&rewritten}(item)) },
        "set" => ts_template! { new Set((raw as any[]).map(item => @{&rewritten}(item))) },
        "map" => ts_template! {
            new Map(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, @{&rewritten}(v)]))
        },
        "record" => ts_template! {
            Object.fromEntries(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, @{&rewritten}(v)])) as any
        },
        // A bare foreign type decodes directly, unless it is nullable.
        "none" if is_nullable => ts_template! { @{&rewritten}(raw) },
        _ => return None,
    };
    Some(rendered(if is_nullable {
        ts_template! { (raw) => raw === null ? null : @{decoded} }
    } else {
        ts_template! { (raw) => @{decoded} }
    }))
}

/// Extracts the base type name from a potentially generic type.
/// For example: "User<T>" -> "User", "Map<string, number>" -> "Map"
pub(super) fn extract_base_type(ts_type: &str) -> String {
    if let Some(idx) = ts_type.find('<') {
        ts_type[..idx].to_string()
    } else {
        ts_type.to_string()
    }
}

/// Generates the DecodeWithContext function name for a nested decodable type.
/// For example: "User" -> "userDecodeWithContext"
pub(super) fn nested_decode_fn_name(type_name: &str) -> String {
    // Strip generic parameters before camelCase conversion (see nested_encode_fn_name)
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}DecodeWithContext", base.to_case(Case::Camel))
}

/// Generates the public Decode function name (result-returning) for a nested decodable type.
/// For example: "User" -> "userDecode"
pub(super) fn nested_decode_result_fn_name(type_name: &str) -> String {
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}Decode", base.to_case(Case::Camel))
}

/// Generates the HasShape function name for a type.
/// For example: "DailyRecurrenceRule" -> "dailyRecurrenceRuleHasShape"
pub(super) fn nested_has_shape_fn_name(type_name: &str) -> String {
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}HasShape", base.to_case(Case::Camel))
}
