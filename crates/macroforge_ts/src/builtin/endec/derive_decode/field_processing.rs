use crate::ast::Expr;
use crate::builtin::derive_common::detect_primitive_encodable_union;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry, resolve_generic_aliases};
use crate::ts_syn::ts_ident;

use super::super::source_field::SourceField;
use super::super::value_kind::{classify_endec_value_kind, get_encodable_type_name};
use super::super::{EndecContainerOptions, EndecFieldOptions, TypeCategory};
use super::super::{get_foreign_types, rewrite_expression_namespaces};
use super::helpers::{
    alias_primitive_arm_validators, parse_default_expr, try_composite_foreign_decode,
};
use super::types::{DecodeField, raw_cast_type};

/// Converts a field into a `DecodeField`, or `None` when the field is
/// skipped.
///
/// This extracts endec options from decorators, computes the TypeCategory,
/// and populates all the inner-kind and encodable-type fields needed
/// for the field-decoding template.
pub(super) fn to_decode_field(
    field: SourceField<'_>,
    container_opts: &EndecContainerOptions,
    diagnostics: &mut DiagnosticCollector,
    type_registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> Option<DecodeField> {
    let parse_result = EndecFieldOptions::from_decorators(field.decorators, field.name);
    diagnostics.extend(parse_result.diagnostics);
    let opts = parse_result.options;

    if !opts.should_decode() {
        return None;
    }

    let json_key = opts
        .rename
        .clone()
        .unwrap_or_else(|| container_opts.rename_all.apply(field.name));

    let resolved_ts_type =
        resolve_generic_aliases(field.ts_type, type_registry, caller_file_path, file_imports);
    let mut type_cat = TypeCategory::from_ts_type(&resolved_ts_type);
    // `string | SomeEncodable`: the resolved shape of `RecordLink<T>`.
    // Downgrade to `Encodable(inner)` + a typeof-guard flag so downstream
    // templates keep a single code path.
    let primitive_union_guard =
        if matches!(type_cat, TypeCategory::Unknown | TypeCategory::Encodable(_)) {
            detect_primitive_encodable_union(&resolved_ts_type).map(|(prim, ser)| {
                type_cat = TypeCategory::Encodable(ser);
                prim
            })
        } else {
            None
        };

    // Pull the primitive arm's validators (e.g. `nonEmpty` on a record-link
    // alias's `string` arm) so the primitive form of the union is validated.
    let union_string_validators = match &primitive_union_guard {
        Some(prim) => alias_primitive_arm_validators(
            field.ts_type,
            prim,
            type_registry,
            caller_file_path,
            file_imports,
        ),
        None => Vec::new(),
    };

    let nullable_inner_kind = match &type_cat {
        TypeCategory::Nullable(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let array_elem_kind = match &type_cat {
        TypeCategory::Array(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let nullable_encodable_type = match &type_cat {
        TypeCategory::Nullable(inner) => get_encodable_type_name(inner),
        _ => None,
    };
    // When the element type is itself a primitive-plus-encodable union
    // (the resolved shape of any `Alias<T> = primitive | T` generic),
    // detect the union and capture both halves: the encodable side feeds
    // the element-decode call, the primitive side gates the per-element
    // typeof guard so primitive elements pass through unchanged.
    let array_elem_primitive_union = match &type_cat {
        TypeCategory::Array(inner) => detect_primitive_encodable_union(inner),
        _ => None,
    };
    let array_elem_encodable_type = match &type_cat {
        TypeCategory::Array(inner) => get_encodable_type_name(inner).or_else(|| {
            array_elem_primitive_union
                .as_ref()
                .map(|(_, ser)| ser.clone())
        }),
        _ => None,
    };
    let array_elem_primitive_union_guard = array_elem_primitive_union
        .as_ref()
        .map(|(prim, _)| prim.clone());
    let set_elem_kind = match &type_cat {
        TypeCategory::Set(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let set_elem_encodable_type = match &type_cat {
        TypeCategory::Set(inner) => get_encodable_type_name(inner),
        _ => None,
    };
    let map_value_kind = match &type_cat {
        TypeCategory::Map(_, value) => Some(classify_endec_value_kind(value)),
        _ => None,
    };
    let map_value_encodable_type = match &type_cat {
        TypeCategory::Map(_, value) => get_encodable_type_name(value),
        _ => None,
    };
    let record_value_encodable_type = match &type_cat {
        TypeCategory::Record(_, value) => get_encodable_type_name(value),
        _ => None,
    };
    let wrapper_encodable_type = match &type_cat {
        TypeCategory::Wrapper(inner) => get_encodable_type_name(inner),
        _ => None,
    };

    // Check for foreign type decoder if no explicit decode_with
    let decode_with_src = if opts.decode_with.is_some() {
        opts.decode_with.clone()
    } else {
        let foreign_types = get_foreign_types();
        let ft_match = TypeCategory::match_foreign_type(field.ts_type, &foreign_types);
        if let Some(warning) = ft_match.warning {
            diagnostics.warning(field.span, warning);
        }
        ft_match
            .config
            .and_then(|ft| ft.decode_expr.clone())
            .map(|expr| rewrite_expression_namespaces(&expr))
            .or_else(|| try_composite_foreign_decode(field.ts_type))
    };

    let decode_with = decode_with_src
        .as_ref()
        .and_then(|expr_src| match Expr::parse(expr_src) {
            Ok(expr) => Some(expr),
            Err(err) => {
                diagnostics.error(
                    field.span,
                    format!(
                        "@endec(decodeWith): invalid expression for '{}': {err:?}",
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
                            "@endec({{default: ...}}): invalid expression for '{}': {err:?}",
                            field.name
                        ),
                    );
                    None
                }
            });

    Some(DecodeField {
        json_key,
        field_name: field.name.to_string(),
        field_ident: ts_ident!(field.name),
        raw_cast_type: raw_cast_type(&resolved_ts_type, &type_cat),
        ts_type: resolved_ts_type,
        type_cat,
        optional: field.optional || opts.default || opts.default_expr.is_some(),
        default_expr,
        flatten: opts.flatten,
        validators: opts.validators.clone(),
        nullable_inner_kind,
        array_elem_kind,
        nullable_encodable_type,
        decode_with,
        decimal_format: opts.format.as_deref() == Some("decimal"),
        array_elem_encodable_type,
        set_elem_kind,
        set_elem_encodable_type,
        map_value_kind,
        map_value_encodable_type,
        record_value_encodable_type,
        wrapper_encodable_type,
        primitive_union_guard,
        array_elem_primitive_union_guard,
        union_string_validators,
    })
}
