use crate::ts_syn::TsStream;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{parse_ts_expr, ts_ident};

use super::super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory};
use super::helpers::{
    classify_serde_value_kind, get_serializable_type_name, nested_deserialize_fn_name,
    parse_default_expr, try_composite_foreign_deserialize,
};
use super::super::{get_foreign_types, rewrite_expression_namespaces};
use super::types::{DeserializeField, ObjectVariant, SerdeValueKind, raw_cast_type};

/// Converts an `InterfaceFieldIR` into a `DeserializeField`.
///
/// This extracts serde options from decorators, computes the TypeCategory,
/// and populates all the inner-kind and serializable-type fields needed
/// for the field-deserialization template.
pub(super) fn interface_field_to_deserialize_field(
    field: &crate::ts_syn::abi::ir::interface::InterfaceFieldIR,
    container_opts: &SerdeContainerOptions,
    diagnostics: &mut DiagnosticCollector,
) -> Option<DeserializeField> {
    let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
    diagnostics.extend(parse_result.diagnostics);
    let opts = parse_result.options;

    if !opts.should_deserialize() {
        return None;
    }

    let json_key = opts
        .rename
        .clone()
        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

    let nullable_inner_kind = match &type_cat {
        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let array_elem_kind = match &type_cat {
        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let nullable_serializable_type = match &type_cat {
        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let array_elem_serializable_type = match &type_cat {
        TypeCategory::Array(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let set_elem_kind = match &type_cat {
        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let set_elem_serializable_type = match &type_cat {
        TypeCategory::Set(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let map_value_kind = match &type_cat {
        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
        _ => None,
    };
    let map_value_serializable_type = match &type_cat {
        TypeCategory::Map(_, value) => get_serializable_type_name(value),
        _ => None,
    };
    let record_value_kind = match &type_cat {
        TypeCategory::Record(_, value) => Some(classify_serde_value_kind(value)),
        _ => None,
    };
    let record_value_serializable_type = match &type_cat {
        TypeCategory::Record(_, value) => get_serializable_type_name(value),
        _ => None,
    };
    let wrapper_inner_kind = match &type_cat {
        TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let wrapper_serializable_type = match &type_cat {
        TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let optional_inner_kind = match &type_cat {
        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let optional_serializable_type = match &type_cat {
        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
        _ => None,
    };

    // Check for foreign type deserializer if no explicit deserialize_with
    let deserialize_with_src = if opts.deserialize_with.is_some() {
        opts.deserialize_with.clone()
    } else {
        let foreign_types = get_foreign_types();
        let ft_match = TypeCategory::match_foreign_type(&field.ts_type, &foreign_types);
        if let Some(error) = ft_match.error {
            diagnostics.error(field.span, error);
        }
        if let Some(warning) = ft_match.warning {
            diagnostics.warning(field.span, warning);
        }
        ft_match
            .config
            .and_then(|ft| ft.deserialize_expr.clone())
            .map(|expr| rewrite_expression_namespaces(&expr))
            .or_else(|| try_composite_foreign_deserialize(&field.ts_type))
    };

    let deserialize_with = deserialize_with_src
        .as_ref()
        .and_then(|expr_src| match parse_ts_expr(expr_src) {
            Ok(expr) => Some(*expr),
            Err(err) => {
                diagnostics.error(
                    field.span,
                    format!(
                        "@serde(deserializeWith): invalid expression for '{}': {err:?}",
                        field.name
                    ),
                );
                None
            }
        });

    let default_expr =
        opts.default_expr
            .as_ref()
            .and_then(|expr_src| match parse_default_expr(expr_src) {
                Ok(expr) => Some(expr),
                Err(err) => {
                    diagnostics.error(
                        field.span,
                        format!(
                            "@serde({{default: ...}}): invalid expression for '{}': {err:?}",
                            field.name
                        ),
                    );
                    None
                }
            });

    Some(DeserializeField {
        json_key,
        field_name: field.name.clone(),
        field_ident: ts_ident!(field.name.as_str()),
        raw_cast_type: raw_cast_type(&field.ts_type, &type_cat),
        ts_type: field.ts_type.clone(),
        type_cat,
        optional: field.optional || opts.default || opts.default_expr.is_some(),
        has_default: opts.default || opts.default_expr.is_some(),
        default_expr,
        flatten: opts.flatten,
        validators: opts.validators.clone(),
        nullable_inner_kind,
        array_elem_kind,
        nullable_serializable_type,
        deserialize_with,
        decimal_format: opts.format.as_deref() == Some("decimal"),
        array_elem_serializable_type,
        set_elem_kind,
        set_elem_serializable_type,
        map_value_kind,
        map_value_serializable_type,
        record_value_kind,
        record_value_serializable_type,
        wrapper_inner_kind,
        wrapper_serializable_type,
        optional_inner_kind,
        optional_serializable_type,
    })
}

/// Generates a TypeScript code block string that deserializes an inline object variant's fields.
///
/// The generated code reads fields from `source_var` (e.g., "value" or "__content"),
/// builds an `instance` object with proper type conversions, and returns it.
///
/// Used by the union deserialization template to handle `TypeMemberKind::Object` members
/// inline, without needing a separate named-type deserializer function.
#[allow(dead_code)]
pub(super) fn generate_object_variant_deser_block(
    variant: &ObjectVariant,
    source_var: &str,
    tag_field: &str,
    full_type_name: &str,
) -> TsStream {
    let mut lines = Vec::new();
    lines.push("{".to_string());
    lines.push(format!(
        "  const __obj = {} as Record<string, unknown>;",
        source_var
    ));
    lines.push("  const __inst: any = {};".to_string());

    // Set the tag field
    if let Some(tv) = &variant.tag_value {
        lines.push(format!("  __inst[\"{}\"] = \"{}\";", tag_field, tv));
    }

    // Deserialize each field
    for field in &variant.fields {
        let key = &field.json_key;

        if field.optional {
            lines.push(format!(
                "  if (\"{}\" in __obj && __obj[\"{}\"] !== undefined) {{",
                key, key
            ));
            generate_field_assignment(&mut lines, field, "    ");
            lines.push("  }".to_string());
        } else {
            lines.push(format!("  if (\"{}\" in __obj) {{", key));
            generate_field_assignment(&mut lines, field, "    ");
            lines.push("  }".to_string());
        }
    }

    lines.push(format!(
        "  ctx.trackForFreeze(__inst);  return __inst as {};",
        full_type_name
    ));
    lines.push("}".to_string());
    TsStream::from_string(lines.join("\n"))
}

/// Helper for `generate_object_variant_deser_block`: emits the field assignment
/// line(s) for a single field, based on its `TypeCategory`.
#[allow(dead_code)]
pub(super) fn generate_field_assignment(
    lines: &mut Vec<String>,
    field: &DeserializeField,
    indent: &str,
) {
    let key = &field.json_key;
    let fname = &field.field_name;

    match &field.type_cat {
        TypeCategory::Primitive => {
            lines.push(format!(
                "{}__inst.{} = __obj[\"{}\"] as {};",
                indent, fname, key, field.ts_type
            ));
        }
        TypeCategory::Date => {
            lines.push(format!(
                "{}__inst.{} = typeof __obj[\"{}\"] === \"string\" ? new Date(__obj[\"{}\"] as string) : __obj[\"{}\"] as Date;",
                indent, fname, key, key, key
            ));
        }
        TypeCategory::Serializable(type_name) => {
            let deser_fn = nested_deserialize_fn_name(type_name);
            lines.push(format!(
                "{}__inst.{} = {}(__obj[\"{}\"], ctx) as {};",
                indent, fname, deser_fn, key, field.ts_type
            ));
        }
        TypeCategory::Nullable(inner) => {
            let inner_cat = TypeCategory::from_ts_type(inner);
            match inner_cat {
                TypeCategory::Date => {
                    lines.push(format!(
                        "{}__inst.{} = __obj[\"{}\"] === null ? null : (typeof __obj[\"{}\"] === \"string\" ? new Date(__obj[\"{}\"] as string) : __obj[\"{}\"] as Date);",
                        indent, fname, key, key, key, key
                    ));
                }
                TypeCategory::Serializable(ser_name) => {
                    let deser_fn = nested_deserialize_fn_name(&ser_name);
                    lines.push(format!(
                        "{}__inst.{} = __obj[\"{}\"] === null ? null : {}(__obj[\"{}\"], ctx) as {};",
                        indent, fname, key, deser_fn, key, field.ts_type
                    ));
                }
                _ => {
                    lines.push(format!(
                        "{}__inst.{} = __obj[\"{}\"] as {};",
                        indent, fname, key, field.raw_cast_type
                    ));
                }
            }
        }
        TypeCategory::Array(inner) => {
            let elem_kind = field.array_elem_kind.unwrap_or(SerdeValueKind::Other);
            match elem_kind {
                SerdeValueKind::PrimitiveLike => {
                    lines.push(format!(
                        "{}__inst.{} = __obj[\"{}\"] as {}[];",
                        indent, fname, key, inner
                    ));
                }
                SerdeValueKind::Date => {
                    lines.push(format!(
                        "{}__inst.{} = Array.isArray(__obj[\"{}\"]) ? (__obj[\"{}\"] as any[]).map((item: any) => typeof item === \"string\" ? new Date(item) : item as Date) : [];",
                        indent, fname, key, key
                    ));
                }
                _ => {
                    if let Some(ref elem_type) = field.array_elem_serializable_type {
                        let deser_fn = nested_deserialize_fn_name(elem_type);
                        lines.push(format!(
                            "{}__inst.{} = Array.isArray(__obj[\"{}\"]) ? (__obj[\"{}\"] as any[]).map((item: any) => {}(item, ctx)) : [];",
                            indent, fname, key, key, deser_fn
                        ));
                    } else {
                        lines.push(format!(
                            "{}__inst.{} = __obj[\"{}\"] as {};",
                            indent, fname, key, field.raw_cast_type
                        ));
                    }
                }
            }
        }
        // For all other categories (Map, Set, Record, Wrapper, Optional, etc.),
        // fall back to a simple cast assignment. These cover less common inline
        // object field types and can be enhanced later if needed.
        _ => {
            lines.push(format!(
                "{}__inst.{} = __obj[\"{}\"] as {};",
                indent, fname, key, field.raw_cast_type
            ));
        }
    }
}
