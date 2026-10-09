//! `Default` for an object-shaped interface or type alias: an object literal
//! holding each required field's default.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::abi::{DiagnosticCollector, InterfaceFieldIR, SpanIR};
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use super::values::{DefaultSources, missing_default_derives, resolve_default_value};
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{DefaultFieldOptions, has_known_default};

/// `{camelName}DefaultValue`, returning every required field's `@default`,
/// else its type's default. Optional fields are left out.
pub(super) fn object_default(
    names: &TypeNames,
    fields: &[InterfaceFieldIR],
    decorator_span: SpanIR,
    sources: &DefaultSources,
) -> Result<TsStream, MacroforgeError> {
    let required: Vec<&InterfaceFieldIR> = fields.iter().filter(|field| !field.optional).collect();
    let undeclared = |field: &&&InterfaceFieldIR| {
        !DefaultFieldOptions::from_decorators(&field.decorators).has_default
    };

    let mut diagnostics = DiagnosticCollector::new();
    missing_default_derives(
        required
            .iter()
            .filter(undeclared)
            .map(|field| (field.name.as_str(), field.ts_type.as_str(), field.span)),
        names,
        sources.registry,
        &mut diagnostics,
    );
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }

    // Like Rust's derive(Default), a field with no default of its own needs
    // one written: a function type has none.
    let missing_defaults: Vec<&str> = required
        .iter()
        .filter(undeclared)
        .filter(|field| !has_known_default(&field.ts_type))
        .map(|field| field.name.as_str())
        .collect();
    if !missing_defaults.is_empty() {
        return Err(MacroforgeError::new(
            decorator_span,
            format!(
                "@derive(Default) cannot determine a default for function-typed fields. Add @default(value) to: {}",
                missing_defaults.join(", ")
            ),
        ));
    }

    let object_fields: Vec<(Ident, Expr)> = required
        .iter()
        .map(|field| {
            let opts = DefaultFieldOptions::from_decorators(&field.decorators);
            let value = resolve_default_value(opts.value, &field.ts_type, sources);
            let value_expr = Expr::parse(&value).map_err(|err| {
                MacroforgeError::new(
                    decorator_span,
                    format!(
                        "@derive(Default): invalid default expression for '{}': {err:?}",
                        field.name
                    ),
                )
            })?;
            Ok((ts_ident!(field.name.as_str()), value_expr))
        })
        .collect::<Result<_, MacroforgeError>>()?;

    let fn_name_ident = names.generic_function("DefaultValue");
    let full_type_ident = names.full_type_ident();
    Ok(ts_template! {
        export function @{fn_name_ident}(): @{full_type_ident} {
            return {
                {#for (name_ident, value_expr) in &object_fields}
                    @{name_ident}: @{value_expr},
                {/for}
            };
        }
    })
}
