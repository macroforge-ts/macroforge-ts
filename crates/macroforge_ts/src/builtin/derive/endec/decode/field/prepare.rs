use crate::ast::Expr;
use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry, resolve_generic_aliases};
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::ts_ident;

use crate::builtin::derive::endec::decode::helpers::{
    alias_primitive_arm_validators, parse_default_expr, try_composite_foreign_decode,
};
use crate::builtin::derive::endec::decode::types::{DecodeField, raw_cast_type};
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::{Direction, UnionGuard, union_guard};
use crate::builtin::derive::endec::value_kind::{
    classify_endec_value_kind, get_encodable_type_name,
};
use crate::builtin::derive::endec::{
    EndecContainerOptions, EndecFieldOptions, EndecFormat, TypeCategory, get_foreign_types,
};

/// Converts a field into a `DecodeField`, or `None` when the field is
/// skipped.
///
/// This extracts endec options from decorators, computes the TypeCategory,
/// and populates all the inner-kind and encodable-type fields needed
/// for the field-decoding template.
pub(in crate::builtin::derive::endec::decode) fn to_decode_field(
    field: SourceField<'_>,
    container_opts: &EndecContainerOptions,
    diagnostics: &mut DiagnosticCollector,
    type_registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    type_params: &[String],
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
    // A type parameter has no decoder to call: its values pass through as
    // they are, wherever it appears.
    let is_param = |name: &str| type_params.iter().any(|param| param == name);
    let encodable = |ts_type: &str| get_encodable_type_name(ts_type).filter(|name| !is_param(name));
    // A union of a primitive or foreign type with a user type, as
    // `RecordLink<T>` resolves to, decodes through a runtime guard.
    let mut guard_of = |ts_type: &str| match union_guard(ts_type, Direction::Decode, is_param) {
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

    // Pull the primitive arm's validators (e.g. `nonEmpty` on a record-link
    // alias's `string` arm) so the primitive form of the union is validated.
    let union_string_validators = match union_guard.as_ref().and_then(UnionGuard::primitive) {
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
    let set_elem_kind = match &type_cat {
        TypeCategory::Set(inner) => Some(classify_endec_value_kind(inner)),
        _ => None,
    };
    let set_elem_encodable_type = match &type_cat {
        TypeCategory::Set(inner) => encodable(inner),
        _ => None,
    };
    let map_value_kind = match &type_cat {
        TypeCategory::Map(_, value) => Some(classify_endec_value_kind(value)),
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

    // Whatever decodes the field is called as written, so a user expression
    // or a composite arrow is parenthesized here, once.
    let decode_with_src = if let Some(decode_with) = &opts.decode_with {
        Some(rendered(ts_template! { (@{decode_with}) }))
    } else {
        let foreign_types = get_foreign_types();
        let ft_match = TypeCategory::match_foreign_type(field.ts_type, &foreign_types);
        if let Some(warning) = ft_match.warning {
            diagnostics.warning(field.span, warning);
        }
        ft_match
            .config
            .and_then(|ft| ft.handler_callee(ForeignHandler::Decode))
            .or_else(|| {
                try_composite_foreign_decode(field.ts_type)
                    .map(|arrow| rendered(ts_template! { (@{arrow}) }))
            })
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
        decimal_format: opts.format == Some(EndecFormat::Decimal),
        array_elem_encodable_type,
        set_elem_kind,
        set_elem_encodable_type,
        map_value_kind,
        map_value_encodable_type,
        record_value_encodable_type,
        wrapper_encodable_type,
        union_guard,
        nullable_union_guard,
        array_elem_union_guard,
        union_string_validators,
    })
}
