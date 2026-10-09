//! The body of `encodeWithContext` for a tuple or an alias of another type.
//! The value is viewed as an object whose fields the regular field encoders
//! handle, so each element encodes by its own type.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use super::super::field::prepare::to_encode_field;
use super::super::field::statements::encode_fields;
use super::super::types::EncodeField;
use super::AliasEncode;
use crate::builtin::derive::common::{TypeNames, rendered, tuple_element};
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::derive::endec::source_field::SourceField;

/// Statements returning the encoded tuple, each element encoded by its type
/// and an absent optional element left out.
pub(super) fn tuple_body(
    alias: &AliasEncode,
    elements: &[String],
) -> Result<TsStream, MacroforgeError> {
    let mut diagnostics = DiagnosticCollector::new();
    let mut fields = Vec::new();
    let mut slots: Vec<(String, Expr)> = Vec::new();
    let mut optional_slots: Vec<(String, Expr)> = Vec::new();
    let mut view_members: Vec<String> = Vec::new();
    let mut slot_names: Vec<String> = Vec::new();
    let mut rest_start: Option<Expr> = None;
    for (index, source) in elements.iter().enumerate() {
        let element = tuple_element(source);
        let slot = if element.rest {
            rest_start = Some(number(index));
            "rest".to_string()
        } else {
            let slot = format!("e{index}");
            slot_names.push(slot.clone());
            if element.optional {
                optional_slots.push((slot.clone(), number(index)));
            } else {
                slots.push((slot.clone(), number(index)));
            }
            slot
        };
        let slot_ident = ts_ident!(slot.as_str());
        view_members.push(rendered(if element.optional {
            ts_template! { @{&slot_ident}?: @{element.ts_type} }
        } else {
            ts_template! { @{&slot_ident}: @{element.ts_type} }
        }));
        if let Some(field) = view_field(
            alias,
            &slot,
            element.ts_type,
            element.optional,
            &mut diagnostics,
        ) {
            fields.push(field);
        }
    }
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    let statements = encode_fields(&fields, "");
    let view_members = view_members.join("; ");
    let view_type = ts_ident!(rendered(ts_template! { { @{view_members} } }));
    Ok(ts_template! {
        const __view: @{view_type.clone()} = {
            {#for (slot, index) in &slots}
                @{ts_ident!(slot.as_str())}: value[@{index.clone()}],
            {/for}
            {#if let Some(start) = &rest_start}
                rest: value.slice(@{start.clone()}),
            {/if}
        };
        {#for (slot, index) in &optional_slots}
            if (@{index.clone()} < value.length) {
                __view.@{ts_ident!(slot.as_str())} = value[@{index.clone()}];
            }
        {/for}
        return ((value: @{view_type}): unknown[] => {
            const result: Record<string, unknown> = {};
            {$typescript statements}
            const __tuple: unknown[] = [];
            {#for slot in slot_names}
                if ("@{slot}" in result) {
                    __tuple.push(result.@{ts_ident!(slot.as_str())});
                }
            {/for}
            {#if rest_start.is_some()}
                if (Array.isArray(result.rest)) {
                    __tuple.push(...result.rest);
                }
            {/if}
            return __tuple;
        })(__view);
    })
}

/// Statements returning the value encoded as a field of `ts_type` would be.
pub(super) fn whole_body(alias: &AliasEncode, ts_type: &str) -> Result<TsStream, MacroforgeError> {
    let mut diagnostics = DiagnosticCollector::new();
    let field = view_field(alias, "value", ts_type, false, &mut diagnostics);
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    let statements = encode_fields(&Vec::from_iter(field), "");
    let view_type = ts_ident!(rendered(ts_template! { { value: @{ts_type} } }));
    Ok(ts_template! {
        return ((value: @{view_type}): unknown => {
            const result: Record<string, unknown> = {};
            {$typescript statements}
            return result.value;
        })({ value });
    })
}

/// A field of the view, encoded like a field of the same type on an object,
/// under its own name whatever the container's renaming says.
fn view_field(
    alias: &AliasEncode,
    name: &str,
    ts_type: &str,
    optional: bool,
    diagnostics: &mut DiagnosticCollector,
) -> Option<EncodeField> {
    let container_opts = EndecContainerOptions::from_decorators(&alias.type_alias.inner.decorators);
    let names = TypeNames::new(alias.type_name, alias.type_params);
    let mut field = to_encode_field(
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
    )?;
    field.json_key_ident = ts_ident!(name);
    Some(field)
}

fn number(value: usize) -> Expr {
    ts_ident!(value.to_string()).into()
}
