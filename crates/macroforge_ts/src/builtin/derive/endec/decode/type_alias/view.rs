//! `decodeWithContext` and `hasShape` for a tuple or an alias of another
//! type. The value is viewed as an object whose fields the regular field
//! decoders handle, so each element decodes by its own type.

use convert_case::{Case, Casing};

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use super::AliasDecode;
use crate::builtin::derive::common::{TypeNames, tuple_element};
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::derive::endec::decode::field::prepare::to_decode_field;
use crate::builtin::derive::endec::decode::field::statements::decode_fields;
use crate::builtin::derive::endec::decode::reference::decode_reference;
use crate::builtin::derive::endec::decode::types::DecodeField;
use crate::builtin::derive::endec::source_field::SourceField;

/// How a tuple's elements map onto the view's fields.
struct TupleLayout {
    fields: Vec<DecodeField>,
    /// Elements before any optional or rest element.
    required: usize,
    /// Elements other than a rest element.
    fixed: usize,
    rest: bool,
}

/// The view functions for a tuple alias, decoding each element by its type.
pub(super) fn tuple_view(
    alias: &AliasDecode,
    elements: &[String],
) -> Result<TsStream, MacroforgeError> {
    let layout = tuple_layout(alias, elements)?;
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        alias.type_name.to_case(Case::Camel),
        alias.generic_decl
    );
    let fn_has_shape_ident = ts_ident!("{}HasShape", alias.type_name.to_case(Case::Camel));
    let decode_context_ident = &alias.decode_context_ident;
    let decode_error_expr = &alias.decode_error_expr;
    let full_type_ident = ts_ident!(alias.full_type_name.as_str());
    let type_name = alias.type_name;
    let fixed = number(layout.fixed);
    let required = number(layout.required);
    let has_rest = layout.rest;
    let required_slots: Vec<(String, Expr)> = (0..layout.required)
        .map(|index| (index.to_string(), number(index)))
        .collect();
    let slots: Vec<(String, Expr)> = (0..layout.fixed)
        .map(|index| (format!("e{index}"), number(index)))
        .collect();
    let field_statements = decode_fields(&layout.fields, type_name);
    let reference = decode_reference(type_name, &alias.full_type_name);
    Ok(ts_template! {
        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{full_type_ident.clone()} {
            {$typescript reference}
            if (!Array.isArray(value)) {
                throw new @{decode_error_expr}([{ field: "_root", message: "@{type_name}.decodeWithContext: expected an array" }]);
            }
            const errors: Array<{ field: string; message: string }> = [];
            {#for (key, index) in &required_slots}
                if (value.length <= @{index.clone()}) {
                    errors.push({ field: "@{key}", message: "missing required element" });
                }
            {/for}
            {#if !has_rest}
                if (value.length > @{fixed.clone()}) {
                    errors.push({ field: "_root", message: "too many elements" });
                }
            {/if}
            const obj: Record<string, unknown> = {};
            for (let __index = 0; __index < Math.min(value.length, @{fixed.clone()}); __index++) {
                obj[__index] = value[__index];
            }
            {#if has_rest}
                obj.rest = value.slice(@{fixed.clone()});
            {/if}
            // Each element decodes onto `instance`, whose properties write
            // through to the tuple: a reference resolved later lands there too.
            const tuple: unknown[] = [];
            const instance: any = {};
            {#for (slot, index) in &slots}
                Object.defineProperty(instance, "@{slot}", {
                    get: () => tuple[@{index.clone()}],
                    set: (item: unknown) => {
                        tuple[@{index.clone()}] = item;
                    }
                });
            {/for}
            {#if has_rest}
                Object.defineProperty(instance, "rest", {
                    get: () => tuple.slice(@{fixed.clone()}),
                    set: (items: unknown[]) => {
                        tuple.splice(@{fixed.clone()}, tuple.length, ...items);
                    }
                });
            {/if}
            ctx.trackForFreeze(tuple);

            {$typescript field_statements}

            ctx.pushErrors(errors);
            return tuple as @{full_type_ident};
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            {#if has_rest}
                return Array.isArray(value) && value.length >= @{required};
            {:else}
                return Array.isArray(value) && value.length >= @{required} && value.length <= @{fixed};
            {/if}
        }
    })
}

/// The view functions for an alias of another type: the whole value is one
/// field, reported as `_root`.
pub(super) fn whole_view(alias: &AliasDecode, ts_type: &str) -> Result<TsStream, MacroforgeError> {
    let mut diagnostics = DiagnosticCollector::new();
    let field = view_field(alias, "_root", ts_type, false, &mut diagnostics);
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        alias.type_name.to_case(Case::Camel),
        alias.generic_decl
    );
    let fn_has_shape_ident = ts_ident!("{}HasShape", alias.type_name.to_case(Case::Camel));
    let decode_context_ident = &alias.decode_context_ident;
    let full_type_ident = ts_ident!(alias.full_type_name.as_str());
    let field_statements = decode_fields(&Vec::from_iter(field), alias.type_name);
    let reference = decode_reference(alias.type_name, &alias.full_type_name);
    Ok(ts_template! {
        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{full_type_ident.clone()} {
            {$typescript reference}
            const obj: Record<string, unknown> = { _root: value };
            const errors: Array<{ field: string; message: string }> = [];
            const instance: any = {};

            {$typescript field_statements}

            ctx.pushErrors(errors);
            return instance._root as @{full_type_ident};
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            return value != null;
        }
    })
}

fn tuple_layout(alias: &AliasDecode, elements: &[String]) -> Result<TupleLayout, MacroforgeError> {
    let mut diagnostics = DiagnosticCollector::new();
    let mut layout = TupleLayout {
        fields: Vec::new(),
        required: 0,
        fixed: 0,
        rest: false,
    };
    for (index, source) in elements.iter().enumerate() {
        let element = tuple_element(source);
        let (name, key) = if element.rest {
            layout.rest = true;
            ("rest".to_string(), "rest".to_string())
        } else {
            layout.fixed += 1;
            if !element.optional && layout.required == index {
                layout.required += 1;
            }
            (format!("e{index}"), index.to_string())
        };
        if let Some(mut field) = view_field(
            alias,
            &name,
            element.ts_type,
            element.optional,
            &mut diagnostics,
        ) {
            field.json_key = key;
            layout.fields.push(field);
        }
    }
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    Ok(layout)
}

/// A field of the view, decoded like a field of the same type on an object.
fn view_field(
    alias: &AliasDecode,
    name: &str,
    ts_type: &str,
    optional: bool,
    diagnostics: &mut DiagnosticCollector,
) -> Option<DecodeField> {
    let container_opts = EndecContainerOptions::from_decorators(&alias.type_alias.inner.decorators);
    let names = TypeNames::new(alias.type_name, alias.type_alias.type_params());
    to_decode_field(
        SourceField {
            name,
            ts_type,
            decorators: &[],
            optional,
            span: alias.type_alias.inner.span,
        },
        &container_opts,
        diagnostics,
        alias.type_registry,
        alias.caller_file_path,
        alias.file_imports,
        &names.params,
    )
}

fn number(value: usize) -> Expr {
    let ident: Ident = ts_ident!(value.to_string());
    ident.into()
}
