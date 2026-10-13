use crate::ast::Expr;
use crate::builtin::derive::common::{Member, rendered};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeRegistry};

use crate::builtin::derive::endec::coder::{TypeScope, encode_expr};
use crate::builtin::derive::endec::encode::types::{EncodeField, FieldEncoding};
use crate::builtin::derive::endec::nested::NestedType;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::Direction;
use crate::builtin::derive::endec::{
    EndecContainerOptions, EndecFieldOptions, EndecFormat, TypeCategory, WrapperFields,
    get_foreign_types,
};

/// Converts a field into a `EncodeField`, or `None` when the field is
/// skipped.
///
/// This reads the field's `@endec` options and works out, from its type, the
/// expression that encodes its value.
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

    let scope = TypeScope {
        registry: type_registry,
        file_path: caller_file_path,
        file_imports,
        type_params,
    };
    let resolved_ts_type = scope.resolved(field.ts_type, Direction::Encode);
    let type_cat = TypeCategory::from_ts_type(&resolved_ts_type);

    // A near miss of a configured foreign type is worth a hint.
    let foreign_types = get_foreign_types();
    if let Some(warning) = TypeCategory::match_foreign_type(field.ts_type, &foreign_types).warning {
        diagnostics.warning(field.span, warning);
    }

    // `null` and `undefined` are written as they are; what encodes is the
    // member the value is once it is known to be there.
    let (present_type, undefinable) = match &type_cat {
        TypeCategory::Optional(inner) => (inner.as_str(), true),
        _ => (resolved_ts_type.as_str(), false),
    };
    let (present_type, nullable) = match TypeCategory::from_ts_type(present_type) {
        TypeCategory::Nullable(inner) => (inner, true),
        _ => (present_type.to_string(), false),
    };

    let member = Member::of(field.name);
    let encoding = if let Some(encode_with) = &opts.encode_with {
        // The user's expression is called as written, so it is parenthesized.
        match Expr::parse(&rendered(ts_template! { (@{encode_with}) })) {
            Ok(callee) => FieldEncoding::With(callee),
            Err(err) => {
                diagnostics.error(
                    field.span,
                    format!(
                        "@endec(encodeWith): invalid expression for '{}': {err:?}",
                        field.name
                    ),
                );
                return None;
            }
        }
    } else if opts.format == Some(EndecFormat::Decimal) && type_cat == TypeCategory::Primitive {
        FieldEncoding::Decimal
    } else {
        let value = rendered(ts_template! { value@{&member} });
        // A flattened value's members become the enclosing object's, whose
        // `__id` is the enclosing value's: it is written out by its fields,
        // with no identity of its own.
        let flattened = match scope.nested(&present_type, Direction::Encode, 0) {
            Ok(Some(nested)) if opts.flatten && nested.fields == WrapperFields::Whole => {
                Some(NestedType {
                    fields: WrapperFields::Any,
                    ..nested
                })
            }
            _ => None,
        };
        match flattened {
            Some(nested) => Ok(Some(nested.encode_call(&value))),
            None => encode_expr(&present_type, &value, &scope, 0),
        }
        .map(FieldEncoding::ByType)
        .unwrap_or_else(|message| {
            diagnostics.error(field.span, format!("@derive(Encode): {message}"));
            FieldEncoding::ByType(None)
        })
    };
    // An `encodeWith` function is handed whatever the field holds, `null`
    // and a `T | undefined` type's `undefined` included.
    let by_function = matches!(encoding, FieldEncoding::With(_));

    Some(EncodeField {
        json_member: Member::keyed(&json_key),
        field_name: field.name.to_string(),
        member,
        optional: field.optional || (undefinable && !by_function),
        nullable: nullable && !by_function,
        flatten: opts.flatten,
        is_record: matches!(
            TypeCategory::from_ts_type(&present_type),
            TypeCategory::Record(_, _)
        ),
        encoding,
    })
}
