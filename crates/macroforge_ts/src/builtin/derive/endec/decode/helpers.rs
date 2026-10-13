use crate::ast::Expr;
use crate::builtin::derive::endec::TypeCategory;
use crate::ts_syn::TsSynError;

use convert_case::{Case, Casing};

use super::super::uncodable::{TemplateHole, template_text};
use super::super::value_kind::is_ts_literal;
use super::super::{EndecFieldOptions, ValidatorSpec};
use super::types::DecodeField;
use super::validation::Missing;
use crate::builtin::derive::common::{js_string, primitive_keyword_test, rendered};
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
    // A string is a `string`'s value, a `Date`'s as its ISO text, and a
    // template literal type's.
    if matches!(type_name, "string" | "Date") || type_name.trim_start().starts_with('`') {
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
                TypeMemberKind::Literal(lit) => lit.starts_with(['"', '\'', '`']),
                TypeMemberKind::Object { .. }
                | TypeMemberKind::Intersection(_)
                | TypeMemberKind::Brand(_) => false,
            }),
            // A template literal type holds text.
            TypeBody::Other(written) => written.trim_start().starts_with('`'),
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
    let accepts = primitive_accepts(&field.ts_type, raw)?;
    Some(PrimitiveCheck {
        mismatch: Expr::parse(&rendered(ts_template! { !(@{accepts}) }))
            .expect("primitive check should parse"),
        message: wrong_type_message(field, raw, context_name),
    })
}

/// The condition under which `raw` is a value of `ts_type`, when that is
/// primitive-like: a primitive keyword, a literal, `null`, `undefined`, or a
/// union of those. `None` for any other type, which has no single runtime
/// test.
pub(crate) fn primitive_accepts(ts_type: &str, raw: &str) -> Option<String> {
    let ts_type = ts_type.trim();
    let parts = split_top_level_union(ts_type).unwrap_or_else(|| vec![ts_type]);
    let accepts = parts
        .iter()
        .map(|part| {
            let part = branded_base(part.trim());
            if typeof_primitive(part).is_some() || part == "symbol" {
                Some(decoded_keyword_test(part, raw))
            } else if part == "object" {
                Some(rendered(
                    ts_template! { (typeof @{raw} === "object" && @{raw} !== null) },
                ))
            } else if let Some(spells) = template_test(part, raw) {
                Some(spells)
            } else if is_primitive_keyword(part) || is_ts_literal(part) {
                Some(rendered(ts_template! { @{raw} === @{part} }))
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()?;
    Some(accepts.join(" || "))
}

/// The condition under which `raw` is a value of `ts_type`, when every
/// value of the type is its own encoding and can be told by looking at it: a
/// primitive, a literal, `Date`, an array or a `Record` of such, or a union
/// of them. `None` for any other type. `depth` numbers the locals of nested
/// callbacks apart.
pub(crate) fn structural_accepts(ts_type: &str, raw: &str, depth: usize) -> Option<String> {
    let ts_type = ts_type.trim();
    if let Some(parts) = split_top_level_union(ts_type) {
        let accepts = parts
            .iter()
            .map(|part| {
                structural_accepts(part, raw, depth)
                    .map(|accepts| rendered(ts_template! { (@{accepts}) }))
            })
            .collect::<Option<Vec<_>>>()?;
        return Some(accepts.join(" || "));
    }
    if let Some(accepts) = primitive_accepts(ts_type, raw) {
        return Some(accepts);
    }
    let member = format!("__member{depth}");
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Date => Some(rendered(ts_template! { @{raw} instanceof Date })),
        TypeCategory::Array(element) => {
            let accepts = structural_accepts(&element, &member, depth + 1)?;
            Some(rendered(ts_template! {
                Array.isArray(@{raw}) && @{raw}.every((@{&member}: unknown) => @{accepts})
            }))
        }
        TypeCategory::Record(_, value) => {
            let accepts = structural_accepts(&value, &member, depth + 1)?;
            Some(rendered(ts_template! {
                typeof @{raw} === "object" && @{raw} !== null && !Array.isArray(@{raw}) && Object.values(@{raw}).every((@{&member}: unknown) => @{accepts})
            }))
        }
        _ => matches!(ts_type, "unknown" | "any").then(|| "true".to_string()),
    }
}

/// The error for a raw value that is not of the field's type. A missing value
/// where the type requires one is reported as required rather than as the
/// wrong type.
pub(super) fn wrong_type_message(field: &DecodeField, raw: &str, context_name: &str) -> Expr {
    let wrong_type = js_string(&format!("expected {}", field.ts_type.trim()));
    let message = match field.missing() {
        Missing::Required => {
            let required = js_string(&format!("{context_name}.{} is required", field.json_key));
            rendered(ts_template! { @{raw} == null ? @{required} : @{wrong_type} })
        }
        Missing::Allowed | Missing::Excluded => wrong_type,
    };
    Expr::parse(&message).expect("wrong type message should parse")
}

/// A test that `raw` is text the template literal type `ts_type` holds.
/// `None` when `ts_type` is no template literal type. One whose text a derive
/// cannot tell is reported where its type is first read, and nothing passes
/// a test that cannot be made.
fn template_test(ts_type: &str, raw: &str) -> Option<String> {
    let Ok(template) = template_text(ts_type)? else {
        return Some("false".to_string());
    };
    let quoted = |texts: &[String]| {
        texts
            .iter()
            .map(|text| js_string(text))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let holes = template
        .holes
        .iter()
        .map(|hole| match hole {
            TemplateHole::Text => js_string("string"),
            TemplateHole::Number => js_string("number"),
            TemplateHole::BigInt => js_string("bigint"),
            TemplateHole::OneOf(texts) => format!("[{}]", quoted(texts)),
        })
        .collect::<Vec<_>>()
        .join(", ");
    let texts = quoted(&template.texts);
    let spells = crate::builtin::derive::endec::coder::endec_helper("spellsTemplate");
    Some(format!("{spells}({raw}, [{texts}], [{holes}])"))
}

/// [`primitive_keyword_test`] for a value being decoded: a `number` has to
/// be finite as well, since no encoding writes `NaN` or an infinity back.
pub(crate) fn decoded_keyword_test(keyword: &str, value: &str) -> String {
    // A template literal type is told by its text, not by a `typeof`.
    if let Some(spells) = template_test(keyword.trim(), value) {
        return spells;
    }
    let test = primitive_keyword_test(keyword, value);
    if keyword.trim() == "number" {
        rendered(ts_template! { (@{test} && globalThis.Number.isFinite(@{value})) })
    } else {
        test
    }
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

/// Extracts the base type name from a potentially generic type.
/// For example: "User<T>" -> "User", "Map<string, number>" -> "Map"
pub(crate) fn extract_base_type(ts_type: &str) -> String {
    if let Some(idx) = ts_type.find('<') {
        ts_type[..idx].to_string()
    } else {
        ts_type.to_string()
    }
}

/// Generates the DecodeWithContext function name for a nested decodable type.
/// For example: "User" -> "userDecodeWithContext"
pub(crate) fn nested_decode_fn_name(type_name: &str) -> String {
    // Strip generic parameters before camelCase conversion (see nested_encode_fn_name)
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}DecodeWithContext", base.to_case(Case::Camel))
}

/// Generates the DecodePartialWithContext function name for a nested
/// decodable type, which decodes a value lacking some of its fields.
/// For example: "User" -> "userDecodePartialWithContext"
pub(crate) fn nested_decode_partial_fn_name(type_name: &str) -> String {
    let base = type_name
        .find('<')
        .map_or(type_name, |generics| &type_name[..generics]);
    format!("{}DecodePartialWithContext", base.to_case(Case::Camel))
}

/// Generates the HasShape function name for a type.
/// For example: "DailyRecurrenceRule" -> "dailyRecurrenceRuleHasShape"
pub(crate) fn nested_has_shape_fn_name(type_name: &str) -> String {
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}HasShape", base.to_case(Case::Camel))
}
