//! The statements that decode every field of an object-shaped type onto
//! `instance`, shared by classes, interfaces and object type aliases.
//!
//! The generated code runs inside a `decodeWithContext` body that has `ctx`,
//! `instance`, `obj` and `errors` in scope.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::value::decode_raw_value;
use crate::builtin::derive::common::{each_on_its_own, js_string, rendered};
use crate::builtin::derive::endec::coder::endec_helper;
use crate::builtin::derive::endec::decode::helpers::nested_has_shape_fn_name;
use crate::builtin::derive::endec::decode::types::DecodeField;
use crate::builtin::derive::endec::decode::validation::generate_field_validations;
use crate::builtin::derive::endec::nested::NestedType;
use crate::builtin::derive::endec::{TypeCategory, WrapperFields};
use crate::builtin::return_types::DECODE_ERROR;

/// Which of a type's fields the decoder being generated reads.
#[derive(Clone, Copy)]
pub(in crate::builtin::derive::endec::decode) enum Required {
    /// Every field, each required as its type says.
    All,
    /// Those the `__fields` requirement in scope keeps, when the caller gave
    /// one: the decoder also decodes a `Partial`, `Pick` or `Omit` of its type.
    Selected,
}

/// The condition under which the call being generated requires `field`.
pub(in crate::builtin::derive::endec::decode) fn requires(field: &DecodeField) -> Expr {
    let name = js_string(&field.field_name);
    rendered(ts_template! { (__fields === undefined || __fields.requires(@{name})) }).into()
}

/// Statements reporting each required key of `fields` the input lacks, once
/// a key, where the call requires its field.
pub(in crate::builtin::derive::endec::decode) fn missing_keys_check(
    fields: &[DecodeField],
) -> TsStream {
    // Each field as the name a selection keeps and the key it is read from.
    let required: Vec<String> = fields
        .iter()
        .filter(|field| field.key_required())
        .map(|field| {
            format!(
                "[{}, {}]",
                js_string(&field.field_name),
                js_string(&field.json_key)
            )
        })
        .collect();
    if required.is_empty() {
        return TsStream::merge_all(std::iter::empty());
    }
    let required = required.join(", ");
    ts_template! {
        const __requiredKeys: ReadonlyArray<readonly [string, string]> = [@{required}];
        for (const [__name, __key] of __requiredKeys) {
            if ((__fields === undefined || __fields.requires(__name)) && !(__key in obj)) {
                errors.push({ field: __key, message: "missing required field" });
            }
        }
    }
}

/// The type of the value `fields` are decoded onto: each field optional and
/// writable, as the value holds only those decoded so far. A field named like
/// a member of `Object.prototype` is `unknown`, as the value inherits that
/// member until the field is written.
pub(in crate::builtin::derive::endec::decode) fn assembly_type(fields: &[DecodeField]) -> String {
    const INHERITED: [&str; 7] = [
        "constructor",
        "toString",
        "toLocaleString",
        "valueOf",
        "hasOwnProperty",
        "isPrototypeOf",
        "propertyIsEnumerable",
    ];
    if fields.is_empty() {
        return "object".to_string();
    }
    let members: Vec<String> = fields
        .iter()
        .map(|field| {
            let key = js_string(&field.field_name);
            if INHERITED.contains(&field.field_name.as_str()) {
                format!("{key}?: unknown")
            } else {
                format!("{key}?: {} | undefined", field.ts_type)
            }
        })
        .collect();
    format!("{{ {} }}", members.join("; "))
}

/// Whether `obj` holds every key `fields` require, as one call over the
/// keys: a condition a key would nest as deep as the type is wide.
pub(in crate::builtin::derive::endec::decode) fn holds_required_keys(
    fields: &[DecodeField],
) -> String {
    let keys: Vec<String> = fields
        .iter()
        .filter(|field| field.key_required())
        .map(|field| js_string(&field.json_key))
        .collect();
    if keys.is_empty() {
        return "true".to_string();
    }
    let holds_all = endec_helper("holdsAll");
    let keys = keys.join(", ");
    rendered(ts_template! { @{holds_all}(obj, [@{keys}]) })
}

/// The condition under which the call being generated reads `field` at all.
fn keeps(field: &DecodeField) -> Expr {
    let name = js_string(&field.field_name);
    rendered(ts_template! { (__fields === undefined || __fields.keeps(@{name})) }).into()
}

/// Decodes each regular field from its JSON key, then each flattened field
/// from the whole object. `owner` names the decoded type in messages, and
/// `tag_field` the key its tag is written under.
pub(in crate::builtin::derive::endec::decode) fn decode_fields(
    fields: &[DecodeField],
    owner: &str,
    tag_field: &str,
    required: Required,
) -> TsStream {
    let regular = fields
        .iter()
        .filter(|field| !field.flatten)
        .map(|field| decode_field(field, owner, required));
    // What a flattened record leaves to the fields beside it.
    let sibling_keys: Vec<String> = [tag_field, "__id", "__ref"]
        .into_iter()
        .map(str::to_string)
        .chain(read_keys(fields))
        .collect();
    let flattened = fields.iter().filter(|field| field.flatten).map(|field| {
        when_kept(
            field,
            required,
            decode_flattened_field(field, owner, required, &sibling_keys),
        )
    });
    each_on_its_own(regular.chain(flattened))
}

/// The JSON keys `fields` read: each regular field's own, and those a
/// flattened type is known to read.
pub(in crate::builtin::derive::endec::decode) fn read_keys(
    fields: &[DecodeField],
) -> impl Iterator<Item = String> + '_ {
    fields.iter().flat_map(|field| {
        if field.flatten {
            field.flattened_keys.clone().unwrap_or_default()
        } else {
            vec![field.json_key.clone()]
        }
    })
}

/// One regular field: its `decodeWith` function, or its raw value decoded by
/// type. An optional field is decoded only when present, else its default.
/// A field the call does not keep is not read, so a value never carries a
/// field its shape lacks, and one it keeps without requiring is decoded only
/// when present and gets no default.
fn decode_field(field: &DecodeField, owner: &str, required: Required) -> TsStream {
    if field.never_read {
        let Some(default_expr) = field.default_expr.as_ref() else {
            return TsStream::merge_all(std::iter::empty());
        };
        let assigned = ts_template! {
            instance@{field.member} = (@{default_expr}) satisfies @{field.ts_type};
        };
        // A selection that keeps the field without requiring it gets no default.
        return match required {
            Required::All => assigned,
            Required::Selected => {
                let is_required = requires(field);
                ts_template! {
                    if (@{is_required}) {
                        {$typescript assigned}
                    }
                }
            }
        };
    }
    let present: Expr = rendered(ts_template! {
        ("@{field.json_key_text()}" in obj && obj["@{field.json_key_text()}"] !== undefined)
    })
    .into();
    let decoded = match &field.decode_with {
        Some(fn_expr) => decode_with(field, fn_expr, owner),
        None => {
            let raw_var_ident: Ident = ts_ident!(field.raw_var_name());
            let value = decode_raw_value(field, owner);
            ts_template! {
                const @{raw_var_ident} = obj["@{field.json_key_text()}"] as @{field.raw_cast_type};
                {$typescript value}
            }
        }
    };
    let default_expr = field
        .default_expr
        .as_ref()
        .filter(|_| field.decode_with.is_none());
    if field.optional {
        let defaulted: Expr = match required {
            Required::All => rendered(ts_template! { !@{&present} }),
            Required::Selected => {
                let is_required = requires(field);
                rendered(ts_template! { @{is_required} && !@{&present} })
            }
        }
        .into();
        return when_kept(
            field,
            required,
            ts_template! {
                if (@{&present}) {
                    {$typescript decoded}
                }
                {#if let Some(default_expr) = default_expr}
                    if (@{defaulted}) {
                        instance@{field.member} = (@{default_expr}) satisfies @{field.ts_type};
                    }
                {/if}
            },
        );
    }
    // A required key that is missing is reported as missing, once: there is
    // no value to decode and find wrong as well.
    let key = field.json_key_text();
    match (required, field.key_required()) {
        (Required::All, true) => ts_template! {
            if ("@{key}" in obj) {
                {$typescript decoded}
            }
        },
        (Required::All, false) => ts_template! {
            {
                {$typescript decoded}
            }
        },
        (Required::Selected, key_required) => {
            let is_required = requires(field);
            let name = js_string(&field.field_name);
            let read: Expr = if key_required {
                rendered(ts_template! {
                    (@{is_required} ? "@{&key}" in obj : (__fields.keeps(@{name}) && obj["@{&key}"] !== undefined))
                })
            } else {
                rendered(ts_template! {
                    (@{is_required} || (__fields.keeps(@{name}) && obj["@{&key}"] !== undefined))
                })
            }
            .into();
            ts_template! {
                if @{read} {
                    {$typescript decoded}
                }
            }
        }
    }
}

/// `statements`, run only when the call keeps `field`.
fn when_kept(field: &DecodeField, required: Required, statements: TsStream) -> TsStream {
    match required {
        Required::All => statements,
        Required::Selected => {
            let is_kept = keeps(field);
            ts_template! {
                if (@{is_kept}) {
                    {$typescript statements}
                }
            }
        }
    }
}

/// A field decoded by its `@endec(decodeWith)` function, validated after. A
/// conversion that throws is reported under the field.
fn decode_with(field: &DecodeField, fn_expr: &Expr, owner: &str) -> TsStream {
    if field.has_validators() {
        let validation_code = generate_field_validations(
            &field.validators,
            "__convertedVal",
            &field.json_key,
            owner,
            field.missing(),
        );
        ts_template! {
            try {
                const __convertedVal = @{fn_expr}(obj["@{field.json_key_text()}"]);
                {$typescript validation_code}
                instance@{field.member} = __convertedVal;
            } catch (__error) {
                errors.push({ field: "@{field.json_key_text()}", message: __error instanceof Error ? __error.message : String(__error) });
            }
        }
    } else {
        ts_template! {
            try {
                instance@{field.member} = @{fn_expr}(obj["@{field.json_key_text()}"]);
            } catch (__error) {
                errors.push({ field: "@{field.json_key_text()}", message: __error instanceof Error ? __error.message : String(__error) });
            }
        }
    }
}

/// An `@endec(flatten)` field, decoded from the enclosing object itself.
///
/// A flattened type is decoded by the fields it has, without being registered:
/// the enclosing object's `__id` names the enclosing value, not this one. One
/// that may be absent is decoded when the object has its shape. A flattened
/// record takes the members no other field reads, `sibling_keys` being the
/// keys those fields read.
fn decode_flattened_field(
    field: &DecodeField,
    owner: &str,
    required: Required,
    sibling_keys: &[String],
) -> TsStream {
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let (present_type, absent_allowed) = match &field.type_cat {
        TypeCategory::Optional(inner) | TypeCategory::Nullable(inner) => {
            (TypeCategory::from_ts_type(inner), true)
        }
        present => (present.clone(), field.optional),
    };
    match present_type {
        TypeCategory::Encodable(type_name) => {
            let has_shape: Expr = ts_ident!(nested_has_shape_fn_name(&type_name)).into();
            let unregistered = NestedType {
                fields: WrapperFields::Except(Vec::new()),
                ..NestedType::whole(type_name)
            };
            let decoded = unregistered.decode_call("obj");
            let decode = ts_template! {
                try {
                    const __result = @{decoded};
                    ctx.assignOrDefer(instance, "@{field.field_name_text()}", __result);
                } catch (__e) {
                    if (__e instanceof @{decode_error_expr}) {
                        for (const __err of __e.errors) {
                            errors.push(__err);
                        }
                    } else {
                        throw __e;
                    }
                }
            };
            let is_required = requires(field);
            // A value that is not there is `null` where the type says so.
            let absent_is_null = matches!(field.type_cat, TypeCategory::Nullable(_));
            match (absent_allowed, required) {
                (false, Required::All) => decode,
                (false, Required::Selected) => ts_template! {
                    if (@{is_required} || @{has_shape}(obj)) {
                        {$typescript decode}
                    }
                },
                (true, Required::All) if absent_is_null => ts_template! {
                    if (@{has_shape}(obj)) {
                        {$typescript decode}
                    } else {
                        instance@{field.member} = null;
                    }
                },
                (true, Required::Selected) if absent_is_null => ts_template! {
                    if (@{has_shape}(obj)) {
                        {$typescript decode}
                    } else if (@{is_required}) {
                        instance@{field.member} = null;
                    }
                },
                (true, Required::All | Required::Selected) => ts_template! {
                    if (@{has_shape}(obj)) {
                        {$typescript decode}
                    }
                },
            }
        }
        TypeCategory::Record(_, _) => {
            let raw_var_ident: Ident = ts_ident!(field.raw_var_name());
            let sibling_keys: Vec<String> = sibling_keys.iter().map(|key| js_string(key)).collect();
            let sibling_keys = sibling_keys.join(", ");
            let value = decode_raw_value(field, owner);
            ts_template! {
                {
                    const __siblingKeys = new Set<string>([@{sibling_keys}]);
                    const @{raw_var_ident} = Object.fromEntries(
                        Object.entries(obj).filter(([__key]) => !__siblingKeys.has(__key))
                    ) as @{field.raw_cast_type};
                    {$typescript value}
                }
            }
        }
        // Any other type is refused where the field is prepared.
        TypeCategory::Primitive
        | TypeCategory::Array(_)
        | TypeCategory::Optional(_)
        | TypeCategory::Nullable(_)
        | TypeCategory::Date
        | TypeCategory::Map(_, _)
        | TypeCategory::Set(_)
        | TypeCategory::Wrapper(_)
        | TypeCategory::Unknown => TsStream::merge_all(std::iter::empty()),
    }
}
