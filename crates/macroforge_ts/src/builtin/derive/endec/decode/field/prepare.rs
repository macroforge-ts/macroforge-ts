use crate::ast::Expr;
use crate::builtin::derive::common::{
    Member, get_type_default_with_registry, rendered, tuple_body,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry, resolve_generic_aliases};
use crate::ts_syn::config::ForeignHandler;

use crate::builtin::derive::endec::coder::{
    TypeScope, decode_expr, json_keys_of, nullable_foreign_decoder, object_members,
};
use crate::builtin::derive::endec::decode::helpers::{
    alias_primitive_arm_validators, parse_default_expr,
};
use crate::builtin::derive::endec::decode::types::{
    CompositeDecode, DecodeField, RawShape, raw_cast_type, raw_var_name,
};
use crate::builtin::derive::endec::nested::NestedType;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::{Direction, UnionGuard, union_guard};
use crate::builtin::derive::endec::value_kind::{
    EndecValueKind, classify_endec_value_kind, get_encodable_type_name,
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

    // A field skipped on decode is never read. With a default it is still
    // set, to that default; without one it is left to the caller.
    let never_read = !opts.should_decode();
    if never_read && (opts.skip || !opts.default) {
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
    let scope = TypeScope {
        registry: type_registry,
        file_path: caller_file_path,
        file_imports,
        type_params,
    };
    let mut type_cat = match TypeCategory::from_ts_type(&resolved_ts_type) {
        TypeCategory::Encodable(name) if is_param(&name) => TypeCategory::Unknown,
        TypeCategory::Unknown => match scope.derived(&resolved_ts_type, Direction::Decode) {
            Some(name) => TypeCategory::Encodable(name),
            None => TypeCategory::Unknown,
        },
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
    let nullable_encodable_type = match &type_cat {
        TypeCategory::Nullable(inner) => {
            encodable(inner).or_else(|| nullable_union.as_ref().map(|(_, name)| name.clone()))
        }
        _ => None,
    };
    let nullable_union_guard = nullable_union.map(|(guard, _)| guard);

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
            // A foreign type that may be `null` or `undefined` is decoded
            // when it is there.
            .or_else(
                || match nullable_foreign_decoder(&resolved_ts_type, &scope) {
                    Ok(arrow) => arrow,
                    Err(message) => {
                        diagnostics.error(field.span, format!("@derive(Decode): {message}"));
                        None
                    }
                },
            )
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

    // A collection or a tuple decodes through its members, and a utility
    // type over a user type through that type. A `decodeWith` function or a
    // foreign type's handler takes the raw value as it is instead.
    let raw_shape = match &type_cat {
        TypeCategory::Array(_) | TypeCategory::Set(_) => Some(RawShape::Array),
        TypeCategory::Map(_, _) | TypeCategory::Record(_, _) => Some(RawShape::Object),
        TypeCategory::Unknown if tuple_body(&resolved_ts_type).is_some() => Some(RawShape::Array),
        TypeCategory::Unknown if object_members(&resolved_ts_type).is_some() => {
            Some(RawShape::Object)
        }
        _ => None,
    };
    let raw_var_name = raw_var_name(field.name);
    let composite = match raw_shape.filter(|_| decode_with.is_none()) {
        Some(shape) => match decode_expr(&resolved_ts_type, &raw_var_name, &scope, 0) {
            Ok(decoded) => Some(CompositeDecode { shape, decoded }),
            Err(message) => {
                diagnostics.error(field.span, format!("@derive(Decode): {message}"));
                None
            }
        },
        None => None,
    };
    let wrapped = match &type_cat {
        TypeCategory::Wrapper(_) => {
            match NestedType::of(&resolved_ts_type, &|name| scope.is_param(name)) {
                Ok(wrapped) => wrapped,
                Err(message) => {
                    diagnostics.error(field.span, format!("@derive(Decode): {message}"));
                    None
                }
            }
        }
        _ => None,
    };

    // A flattened type reads keys of the enclosing object, which have to be
    // known to tell them from keys nothing reads.
    let flattened_type = match TypeCategory::from_ts_type(&resolved_ts_type) {
        TypeCategory::Optional(inner) | TypeCategory::Nullable(inner) => {
            TypeCategory::from_ts_type(&inner)
        }
        present => present,
    };
    let flattened_keys = match (&flattened_type, opts.flatten) {
        (TypeCategory::Encodable(name), true) => json_keys_of(name, &scope),
        _ => None,
    };
    if opts.flatten && container_opts.deny_unknown_fields && flattened_keys.is_none() {
        diagnostics.error(
            field.span,
            format!(
                "@endec(flatten) on '{}' cannot be combined with denyUnknownFields: the keys its type reads are not known here, so none can be told unknown",
                field.name
            ),
        );
    }

    // A written default goes through the shorthand that reads a bare word
    // as text; the `default` flag stands for the type's own default, which is
    // an expression as it is.
    let default_parsed = match (&opts.default_expr, opts.default) {
        (Some(expr_src), _) => Some(parse_default_expr(expr_src)),
        (None, true) => Some(Expr::parse(&get_type_default_with_registry(
            &resolved_ts_type,
            type_registry,
            caller_file_path,
            file_imports,
        ))),
        (None, false) => None,
    };
    let default_expr = match default_parsed {
        Some(Ok(expr)) => Some(expr),
        Some(Err(err)) => {
            diagnostics.error(
                field.span,
                format!(
                    "@endec({{default: ...}}): invalid expression for '{}': {err:?}",
                    field.name
                ),
            );
            None
        }
        None => None,
    };

    // A union with `null` or `undefined` is its other member once the value is
    // known to be present. A member the union's own cases do not decode is
    // prepared as a field of its own, so its category decodes it. Its
    // decorators stay with this field: only the type differs. One a
    // `decodeWith` already covers is left to that function.
    let member_of_its_own = match &type_cat {
        TypeCategory::Nullable(inner)
            if TypeCategory::from_ts_type(inner).is_composite()
                || tuple_body(inner).is_some()
                || object_members(inner).is_some() =>
        {
            Some(inner)
        }
        TypeCategory::Optional(inner)
            if classify_endec_value_kind(inner) != EndecValueKind::PrimitiveLike =>
        {
            Some(inner)
        }
        _ => None,
    };
    let present_inner = member_of_its_own
        .filter(|_| decode_with.is_none())
        .and_then(|inner| {
            to_decode_field(
                SourceField {
                    name: field.name,
                    ts_type: inner,
                    decorators: &[],
                    optional: false,
                    span: field.span,
                },
                container_opts,
                diagnostics,
                type_registry,
                caller_file_path,
                file_imports,
                type_params,
            )
        })
        .filter(|member| member.decode_with.is_none())
        .map(|member| {
            Box::new(DecodeField {
                json_key: json_key.clone(),
                validators: opts.validators.clone(),
                ..member
            })
        });

    Some(DecodeField {
        json_key,
        field_name: field.name.to_string(),
        member: Member::of(field.name),
        raw_cast_type: raw_cast_type(&resolved_ts_type, &type_cat),
        ts_type: resolved_ts_type,
        type_cat,
        optional: field.optional || opts.default || opts.default_expr.is_some(),
        default_expr,
        never_read,
        flattened_keys,
        flatten: opts.flatten,
        validators: opts.validators.clone(),
        nullable_inner_kind,
        nullable_encodable_type,
        present_inner,
        decode_with,
        decimal_format: opts.format == Some(EndecFormat::Decimal),
        composite,
        wrapped,
        union_guard,
        nullable_union_guard,
        union_string_validators,
    })
}
