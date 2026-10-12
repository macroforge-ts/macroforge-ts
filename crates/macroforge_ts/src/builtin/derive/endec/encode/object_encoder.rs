//! The standalone `Encode` functions of an object-shaped type: `encode`,
//! `encodeWithContext` and `encodePartialWithContext`.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::field::statements::encode_fields;
use super::text::encode_text_body;
use super::types::EncodeField;
use crate::builtin::derive::common::{TypeNames, js_string, rendered};
use crate::builtin::derive::endec::coder::endec_helper;
use crate::builtin::return_types::{
    ENCODE_CONTEXT, FIELD_REQUIREMENT, FIELD_REQUIREMENT_TYPE, FIELD_SELECTION,
};

/// `{camelName}Encode`, which encodes to JSON text without the tag and
/// `__id` unless asked to keep them, and the context-taking encoders of
/// [`encode_with_context_functions`].
pub(super) fn object_encode_functions(
    names: &TypeNames,
    tag_field: &str,
    fields: &[EncodeField],
) -> TsStream {
    let fn_encode_ident = names.generic_function("Encode");
    let fn_encode_internal_expr: Expr = names.generic_call("EncodeWithContext").into();
    let full_type_ident = names.full_type_ident();
    let encode_text = encode_text_body(&fn_encode_internal_expr, tag_field);
    let with_context = encode_with_context_functions(names, tag_field, fields, Home::Module);
    ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
            {$typescript encode_text}
        }

        {$typescript with_context}
    }
}

/// Where a type's context-taking encoders are written.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Home {
    /// Beside the type, as functions: an interface or an alias has nowhere
    /// else, and no field a function cannot read.
    Module,
    /// In the class, as static methods the functions run: only code in the
    /// class body reads its `private` and `protected` fields.
    Class,
}

/// `{camelName}EncodeWithContext`, which encodes a whole value and stands in
/// for it wherever it is held again, and `{camelName}EncodePartialWithContext`,
/// which encodes a `Partial` of the type, or a `Pick` or `Omit` given the
/// selection. Both write the fields through one writer, which writes each
/// field the value has and the selection keeps.
pub(super) fn encode_with_context_functions(
    names: &TypeNames,
    tag_field: &str,
    fields: &[EncodeField],
    home: Home,
) -> TsStream {
    let fn_encode_internal_ident = names.generic_function("EncodeWithContext");
    let fn_encode_partial_ident = names.generic_function("EncodePartialWithContext");
    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
    let field_selection_ident = ts_ident!(FIELD_SELECTION);
    let field_requirement_expr: Expr = ts_ident!(FIELD_REQUIREMENT).into();
    let field_requirement_type = ts_ident!(FIELD_REQUIREMENT_TYPE);
    let encode_selection: Expr = ts_ident!(endec_helper("encodeSelection")).into();
    let full_type_ident = names.full_type_ident();
    let type_name = names.type_name.as_str();
    let generic_args = names.generic_args.as_str();
    // The writer, as each home calls it.
    let write_fields: Expr = match home {
        Home::Module => names.generic_call("EncodeFields").into(),
        Home::Class => Expr::Source(rendered(
            ts_template! { @{type_name}.#encodeFields@{generic_args} },
        )),
    };
    // A partial value may lack any field, so each is written only when
    // present, and only when the selection keeps it.
    let field_statements = TsStream::merge_all(fields.iter().map(|field| {
        let present = EncodeField {
            optional: true,
            ..field.clone()
        };
        let statements = encode_fields(std::slice::from_ref(&present), tag_field);
        let name = js_string(&field.field_name);
        ts_template! {
            if (__fields === undefined || __fields.keeps(@{name})) {
                {$typescript statements}
            }
        }
    }));
    // A writer with nothing to write, or nothing nested to encode, leaves a
    // parameter unread, which is named as one a caller passes regardless.
    let statements_source = field_statements.source().to_string();
    let parameter = |name: &str| {
        if mentions(&statements_source, name) {
            ts_ident!(name)
        } else {
            ts_ident!("_{}", name)
        }
    };
    let (value_param, ctx_param, result_param, fields_param) = (
        parameter("value"),
        parameter("ctx"),
        parameter("result"),
        parameter("__fields"),
    );
    let whole_body = ts_template! {
        const existingId = ctx.getId(value);
        if (existingId !== undefined) {
            return { __ref: existingId };
        }
        const result: Record<string, unknown> = {
            "@{tag_field}": "@{type_name}",
            __id: ctx.register(value),
        };
        @{&write_fields}(value, ctx, result, undefined);
        return result;
    };
    // The writer reads the value as a whole one, testing each field for
    // `undefined` before it writes it: a `Partial` of a class cannot even
    // name the class's private fields.
    let partial_body = ts_template! {
        return @{encode_selection}(ctx, value, () => {
            const result: Record<string, unknown> = { "@{tag_field}": "@{type_name}" };
            @{&write_fields}(value as @{&full_type_ident}, ctx, result, fields === undefined ? undefined : @{field_requirement_expr}(fields));
            return result;
        });
    };
    let mut result = match home {
        Home::Module => {
            let fn_encode_fields_ident = names.generic_function("EncodeFields");
            ts_template! {
                /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
                export function @{fn_encode_internal_ident}(value: @{&full_type_ident}, ctx: @{&encode_context_ident}): Record<string, unknown> {
                    {$typescript whole_body}
                }

                /** Encodes the fields a partial value has, with an existing context. A value encoded this way is written out wherever it is held: no reference stands for it. @param value - The value to encode @param ctx - The encoding context @param fields - The fields a `Pick` or `Omit` keeps: every field when left out */
                export function @{fn_encode_partial_ident}(value: Partial<@{&full_type_ident}>, ctx: @{&encode_context_ident}, fields?: @{field_selection_ident}<keyof @{&full_type_ident}>): Record<string, unknown> {
                    {$typescript partial_body}
                }

                function @{fn_encode_fields_ident}(@{value_param}: @{&full_type_ident}, @{ctx_param}: @{&encode_context_ident}, @{result_param}: Record<string, unknown>, @{fields_param}: @{field_requirement_type} | undefined): void {
                    {$typescript field_statements}
                }
            }
        }
        Home::Class => {
            let whole_call = ts_ident!(format!("encodeWithContext{generic_args}"));
            let partial_call = ts_ident!(format!("encodePartialWithContext{generic_args}"));
            let class = ts_ident!(type_name);
            let standalone = ts_template! {
                /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
                export function @{fn_encode_internal_ident}(value: @{&full_type_ident}, ctx: @{&encode_context_ident}): Record<string, unknown> {
                    return @{&class}.@{whole_call}(value, ctx);
                }

                /** Encodes the fields a partial value has, with an existing context. A value encoded this way is written out wherever it is held: no reference stands for it. @param value - The value to encode @param ctx - The encoding context @param fields - The fields a `Pick` or `Omit` keeps: every field when left out */
                export function @{fn_encode_partial_ident}(value: Partial<@{&full_type_ident}>, ctx: @{&encode_context_ident}, fields?: @{&field_selection_ident}<keyof @{&full_type_ident}>): Record<string, unknown> {
                    return @{&class}.@{partial_call}(value, ctx, fields);
                }
            };
            let class_body = ts_template!(Within {
                /** @internal Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
                static @{names.generic_method("encodeWithContext")}(value: @{&full_type_ident}, ctx: @{&encode_context_ident}): Record<string, unknown> {
                    {$typescript whole_body}
                }

                /** @internal Encodes the fields a partial value has, with an existing context. @param value - The value to encode @param ctx - The encoding context @param fields - The fields a `Pick` or `Omit` keeps: every field when left out */
                static @{names.generic_method("encodePartialWithContext")}(value: Partial<@{&full_type_ident}>, ctx: @{&encode_context_ident}, fields?: @{&field_selection_ident}<keyof @{&full_type_ident}>): Record<string, unknown> {
                    {$typescript partial_body}
                }

                // Hash-private, so a subclass's own writer is a member apart.
                static @{names.generic_method("#encodeFields")}(@{value_param}: @{&full_type_ident}, @{ctx_param}: @{&encode_context_ident}, @{result_param}: Record<string, unknown>, @{fields_param}: @{field_requirement_type} | undefined): void {
                    {$typescript field_statements}
                }
            });
            standalone.merge(class_body)
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    result.add_aliased_type_import("FieldSelection", crate::package::ENDEC);
    result.add_aliased_type_import("FieldRequirement", crate::package::ENDEC);
    result.add_aliased_import("fieldRequirement", crate::package::ENDEC);
    result
}

/// Whether `source` reads the binding `name`: the word on its own, not as a
/// property of something else, as part of a longer name, or as the text of a
/// string that names a field.
fn mentions(source: &str, name: &str) -> bool {
    let is_name_char = |letter: char| letter.is_alphanumeric() || matches!(letter, '_' | '$');
    source.match_indices(name).any(|(start, _)| {
        let before = source[..start].chars().next_back();
        let after = source[start + name.len()..].chars().next();
        !before.is_some_and(|letter| is_name_char(letter) || matches!(letter, '.' | '"'))
            && !after.is_some_and(is_name_char)
    })
}

#[cfg(test)]
mod tests {
    use super::mentions;

    #[test]
    fn a_binding_is_read_only_as_a_word_of_its_own() {
        assert!(mentions("f(value.id, ctx)", "ctx"));
        assert!(mentions("ctx.register(value)", "ctx"));
        assert!(!mentions("result.ctx = value.ctx;", "ctx"));
        assert!(!mentions(r#"__fields.keeps("ctx")"#, "ctx"));
        assert!(!mentions("const ctxt = 1; context();", "ctx"));
        assert!(!mentions("", "ctx"));
    }
}
