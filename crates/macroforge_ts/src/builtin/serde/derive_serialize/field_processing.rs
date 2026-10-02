use crate::ast::Expr;
use crate::builtin::derive_common::detect_primitive_serializable_union;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry, resolve_generic_aliases};
use crate::ts_syn::ts_ident;

use super::super::source_field::SourceField;
use super::super::{
    SerdeContainerOptions, SerdeFieldOptions, TypeCategory, get_foreign_types,
    rewrite_expression_namespaces,
};
use super::foreign_types::try_composite_foreign_serialize;
use super::types::{SerializeField, classify_serde_value_kind, get_serializable_type_name};

/// Converts a field into a `SerializeField`, or `None` when the field is
/// skipped.
///
/// This extracts serde options from decorators, computes the TypeCategory,
/// and populates all the inner-kind and serializable-type fields needed
/// for the field-serialization template.
pub(super) fn to_serialize_field(
    field: SourceField<'_>,
    container_opts: &SerdeContainerOptions,
    diagnostics: &mut DiagnosticCollector,
    type_registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> Option<SerializeField> {
    let parse_result = SerdeFieldOptions::from_decorators(field.decorators, field.name);
    diagnostics.extend(parse_result.diagnostics);
    let opts = parse_result.options;

    if !opts.should_serialize() {
        return None;
    }

    let json_key = opts
        .rename
        .clone()
        .unwrap_or_else(|| container_opts.rename_all.apply(field.name));

    let resolved_ts_type =
        resolve_generic_aliases(field.ts_type, type_registry, caller_file_path, file_imports);
    let mut type_cat = TypeCategory::from_ts_type(&resolved_ts_type);
    let primitive_union_guard = if matches!(
        type_cat,
        TypeCategory::Unknown | TypeCategory::Serializable(_)
    ) {
        detect_primitive_serializable_union(&resolved_ts_type).map(|(prim, ser)| {
            type_cat = TypeCategory::Serializable(ser);
            prim
        })
    } else {
        None
    };

    let optional_inner_kind = match &type_cat {
        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let nullable_inner_kind = match &type_cat {
        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let array_elem_kind = match &type_cat {
        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let set_elem_kind = match &type_cat {
        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };
    let map_value_kind = match &type_cat {
        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
        _ => None,
    };
    let record_value_kind = match &type_cat {
        TypeCategory::Record(_, value) => Some(classify_serde_value_kind(value)),
        _ => None,
    };
    let wrapper_inner_kind = match &type_cat {
        TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
        _ => None,
    };

    // Extract serializable type names for direct function calls
    let optional_serializable_type = match &type_cat {
        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let nullable_serializable_type = match &type_cat {
        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let array_elem_primitive_union_guard = match &type_cat {
        TypeCategory::Array(inner) => {
            detect_primitive_serializable_union(inner).map(|(prim, _)| prim)
        }
        _ => None,
    };
    let array_elem_serializable_type = match &type_cat {
        TypeCategory::Array(inner) => get_serializable_type_name(inner)
            .or_else(|| detect_primitive_serializable_union(inner).map(|(_, ser)| ser)),
        _ => None,
    };
    let set_elem_serializable_type = match &type_cat {
        TypeCategory::Set(inner) => get_serializable_type_name(inner),
        _ => None,
    };
    let map_value_serializable_type = match &type_cat {
        TypeCategory::Map(_, value) => get_serializable_type_name(value),
        _ => None,
    };
    let record_value_serializable_type = match &type_cat {
        TypeCategory::Record(_, value) => get_serializable_type_name(value),
        _ => None,
    };
    let wrapper_serializable_type = match &type_cat {
        TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
        _ => None,
    };

    // Check for foreign type serializer if no explicit serialize_with
    let serialize_with_src = if opts.serialize_with.is_some() {
        opts.serialize_with.clone()
    } else {
        // Check if the field's type matches a configured foreign type
        let foreign_types = get_foreign_types();
        let ft_match = TypeCategory::match_foreign_type(field.ts_type, &foreign_types);
        // Log warning for informational hints
        if let Some(warning) = ft_match.warning {
            diagnostics.warning(field.span, warning);
        }
        // Rewrite namespace references to use generated aliases
        ft_match
            .config
            .and_then(|ft| ft.serialize_expr.clone())
            .map(|expr| rewrite_expression_namespaces(&expr))
            // If no direct match, try composite patterns (e.g., DateTime.Utc[])
            .or_else(|| try_composite_foreign_serialize(field.ts_type))
    };

    let serialize_with =
        serialize_with_src
            .as_ref()
            .and_then(|expr_src| match Expr::parse(expr_src) {
                Ok(expr) => Some(expr),
                Err(err) => {
                    diagnostics.error(
                        field.span,
                        format!(
                            "@serde(serializeWith): invalid expression for '{}': {err:?}",
                            field.name
                        ),
                    );
                    None
                }
            });

    Some(SerializeField {
        json_key_ident: ts_ident!(&json_key),
        field_ident: ts_ident!(field.name),
        type_cat,
        optional: field.optional,
        flatten: opts.flatten,
        optional_inner_kind,
        nullable_inner_kind,
        array_elem_kind,
        set_elem_kind,
        map_value_kind,
        record_value_kind,
        wrapper_inner_kind,
        optional_serializable_type,
        nullable_serializable_type,
        array_elem_serializable_type,
        set_elem_serializable_type,
        map_value_serializable_type,
        record_value_serializable_type,
        wrapper_serializable_type,
        serialize_with,
        decimal_format: opts.format.as_deref() == Some("decimal"),
        primitive_union_guard,
        array_elem_primitive_union_guard,
    })
}
