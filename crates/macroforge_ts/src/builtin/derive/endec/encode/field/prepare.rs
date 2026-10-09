use crate::ast::Expr;
use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry, resolve_generic_aliases};
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::ts_ident;

use crate::builtin::derive::endec::encode::foreign_types::try_composite_foreign_encode;
use crate::builtin::derive::endec::encode::types::EncodeField;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::{Direction, union_guard};
use crate::builtin::derive::endec::value_kind::{
    classify_endec_value_kind, get_encodable_type_name,
};
use crate::builtin::derive::endec::{
    EndecContainerOptions, EndecFieldOptions, EndecFormat, TypeCategory, get_foreign_types,
};

/// Converts a field into a `EncodeField`, or `None` when the field is
/// skipped.
///
/// This extracts endec options from decorators, computes the TypeCategory,
/// and populates all the inner-kind and encodable-type fields needed
/// for the field-encoding template.
pub(in crate::builtin::derive::endec::encode) fn to_encode_field(
    field: SourceField<'_>,
    container_opts: &EndecContainerOptions,
    diagnostics: &mut DiagnosticCollector,
    type_registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    type_params: &[String],
) -> Option<EncodeField> {
    let parse_result = EndecFieldOptions::from_decorators(field.decorators, field.name);
    diagnostics.extend(parse_result.diagnostics);
    let opts = parse_result.options;

    if !opts.should_encode() {
        return None;
    }

    let json_key = opts
        .rename
        .clone()
        .unwrap_or_else(|| container_opts.rename_all.apply(field.name));

    let resolved_ts_type =
        resolve_generic_aliases(field.ts_type, type_registry, caller_file_path, file_imports);
    // A type parameter has no encoder to call: its values pass through as
    // they are, wherever it appears.
    let is_param = |name: &str| type_params.iter().any(|param| param == name);
    let encodable = |ts_type: &str| get_encodable_type_name(ts_type).filter(|name| !is_param(name));
    // A union of a primitive or foreign type with a user type, as
    // `RecordLink<T>` resolves to, encodes through a runtime guard.
    let mut guard_of = |ts_type: &str| match union_guard(ts_type, Direction::Encode, is_param) {
        Ok(found) => found,
        Err(message) => {
            diagnostics.error(field.span, message);
            None
        }
    };
    let mut type_cat = match TypeCategory::from_ts_type(&resolved_ts_type) {
        TypeCategory::Encodable(name) if is_param(&name) => TypeCategory::Unknown,
        type_cat => type_cat,
    };
    let union_guard = if matches!(type_cat, TypeCategory::Unknown | TypeCategory::Encodable(_)) {
        guard_of(&resolved_ts_type).map(|(guard, name)| {
            type_cat = TypeCategory::Encodable(name);
            guard
        })
    } else {
        None
    };
    let nullable_union = match &type_cat {
        TypeCategory::Nullable(inner) => guard_of(inner),
        _ => None,
    };
    let array_elem_union = match &type_cat {
        TypeCategory::Array(inner) => guard_of(inner),
        _ => None,
    };

    let optional_inner_kind = match &type_cat {
        TypeCategory::Optional(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let nullable_inner_kind = match &type_cat {
        TypeCategory::Nullable(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let array_elem_kind = match &type_cat {
        TypeCategory::Array(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let set_elem_kind = match &type_cat {
        TypeCategory::Set(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let map_value_kind = match &type_cat {
        TypeCategory::Map(_, value) => Some(classify_endec_value_kind(value)),
        _ => None,
    };
    let record_value_kind = match &type_cat {
        TypeCategory::Record(_, value) => Some(classify_endec_value_kind(value)),
        _ => None,
    };
    let wrapper_inner_kind = match &type_cat {
        TypeCategory::Wrapper(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };

    // Extract encodable type names for direct function calls
    let optional_encodable_type = match &type_cat {
        TypeCategory::Optional(inner) => encodable(inner),
        _ => None,
    };
    let nullable_encodable_type = match &type_cat {
        TypeCategory::Nullable(inner) => {
            encodable(inner).or_else(|| nullable_union.as_ref().map(|(_, name)| name.clone()))
        }
        _ => None,
    };
    let nullable_union_guard = nullable_union.map(|(guard, _)| guard);
    let array_elem_encodable_type = match &type_cat {
        TypeCategory::Array(inner) => {
            encodable(inner).or_else(|| array_elem_union.as_ref().map(|(_, name)| name.clone()))
        }
        _ => None,
    };
    let array_elem_union_guard = array_elem_union.map(|(guard, _)| guard);
    let set_elem_encodable_type = match &type_cat {
        TypeCategory::Set(inner) => encodable(inner),
        _ => None,
    };
    let map_value_encodable_type = match &type_cat {
        TypeCategory::Map(_, value) => encodable(value),
        _ => None,
    };
    let record_value_encodable_type = match &type_cat {
        TypeCategory::Record(_, value) => encodable(value),
        _ => None,
    };
    let wrapper_encodable_type = match &type_cat {
        TypeCategory::Wrapper(inner) => encodable(inner),
        _ => None,
    };

    // Whatever encodes the field is called as written, so a user expression
    // or a composite arrow is parenthesized here, once.
    let encode_with_src = if let Some(encode_with) = &opts.encode_with {
        Some(rendered(ts_template! { (@{encode_with}) }))
    } else {
        // Check if the field's type matches a configured foreign type
        let foreign_types = get_foreign_types();
        let ft_match = TypeCategory::match_foreign_type(field.ts_type, &foreign_types);
        // Log warning for informational hints
        if let Some(warning) = ft_match.warning {
            diagnostics.warning(field.span, warning);
        }
        ft_match
            .config
            .and_then(|ft| ft.handler_callee(ForeignHandler::Encode))
            // If no direct match, try composite patterns (e.g., DateTime.Utc[])
            .or_else(|| {
                try_composite_foreign_encode(field.ts_type)
                    .map(|arrow| rendered(ts_template! { (@{arrow}) }))
            })
    };

    let encode_with = encode_with_src
        .as_ref()
        .and_then(|expr_src| match Expr::parse(expr_src) {
            Ok(expr) => Some(expr),
            Err(err) => {
                diagnostics.error(
                    field.span,
                    format!(
                        "@endec(encodeWith): invalid expression for '{}': {err:?}",
                        field.name
                    ),
                );
                None
            }
        });

    Some(EncodeField {
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
        optional_encodable_type,
        nullable_encodable_type,
        array_elem_encodable_type,
        set_elem_encodable_type,
        map_value_encodable_type,
        record_value_encodable_type,
        wrapper_encodable_type,
        encode_with,
        decimal_format: opts.format == Some(EndecFormat::Decimal),
        union_guard,
        nullable_union_guard,
        array_elem_union_guard,
    })
}
