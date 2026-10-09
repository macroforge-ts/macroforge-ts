//! `Default` for a class: a static `defaultValue()` that builds an instance
//! without running a constructor, and a standalone function calling it.

use crate::ast::{Expr, Ident};
use crate::builtin::derive::common::{DefaultFieldOptions, TypeNames, has_known_default};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DataClass, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::values::{DefaultSources, missing_default_derives, reads_this, resolve_default_value};

pub(super) fn class_default(
    input: &DeriveInput,
    class: &DataClass,
    sources: &DefaultSources,
) -> Result<TsStream, MacroforgeError> {
    let class_name = input.name();
    let names = TypeNames::new(class_name, class.type_params());
    let class_ident = ts_ident!(class_name);
    let full_type_ident = names.full_type_ident();
    let class_expr: Expr = class_ident.into();

    let mut diagnostics = DiagnosticCollector::new();
    missing_default_derives(
        class
            .fields()
            .iter()
            .filter(|field| {
                !field.optional
                    && field.initializer.is_none()
                    && !DefaultFieldOptions::from_decorators(&field.decorators).has_default
            })
            .map(|field| (field.name.as_str(), field.ts_type.as_str(), field.span)),
        &names,
        sources.registry,
        &mut diagnostics,
    );
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }

    // Check for required non-primitive fields missing @default (like Rust's derive(Default))
    let missing_defaults: Vec<&str> = class
        .fields()
        .iter()
        .filter(|field| {
            // Skip optional fields
            if field.optional {
                return false;
            }
            // Skip if has explicit @default or an initializer
            if DefaultFieldOptions::from_decorators(&field.decorators).has_default
                || field.initializer.is_some()
            {
                return false;
            }
            // Skip if type has known default (primitives, collections, nullable)
            if has_known_default(&field.ts_type) {
                return false;
            }
            // This field needs @default but doesn't have it
            true
        })
        .map(|f| f.name.as_str())
        .collect();

    if !missing_defaults.is_empty() {
        return Err(MacroforgeError::new(
            input.decorator_span(),
            format!(
                "@derive(Default) cannot determine a default for function-typed fields. Add @default(value) or an initializer to: {}",
                missing_defaults.join(", ")
            ),
        ));
    }

    // Each field's default is its @default, else its initializer, else
    // its type's default. An optional field with neither is left out.
    let mut field_data: Vec<(Ident, Expr)> = Vec::new();
    for field in class.fields() {
        let opts = DefaultFieldOptions::from_decorators(&field.decorators);
        let default_value = match (opts.value, &field.initializer) {
            (Some(value), _) => resolve_default_value(Some(value), &field.ts_type, sources),
            (None, Some(initializer)) if reads_this(initializer) => {
                return Err(MacroforgeError::new(
                    field.span,
                    format!(
                        "@derive(Default): the initializer of '{}' reads `this`, which a default has no instance for. Add @default(value) to '{}'",
                        field.name, field.name
                    ),
                ));
            }
            (None, Some(initializer)) => initializer.clone(),
            (None, None) if field.optional => continue,
            (None, None) => resolve_default_value(None, &field.ts_type, sources),
        };
        let value_expr = Expr::parse(&default_value).map_err(|err| {
            MacroforgeError::new(
                input.decorator_span(),
                format!(
                    "@derive(Default): invalid default expression for '{}': {err:?}",
                    field.name
                ),
            )
        })?;
        field_data.push((ts_ident!(field.name.as_str()), value_expr));
    }

    // A static member cannot name the class's parameters, so the method
    // declares its own of the same names.
    let static_method = ts_ident!(format!("defaultValue{}", names.generic_decl));
    let static_call: Expr = ts_ident!(format!("defaultValue{}", names.generic_args)).into();
    let class_body = ts_template!(Within {
        static @{static_method}(): @{full_type_ident.clone()} {
            // Like Rust's derive, a default runs no constructor: one that
            // takes arguments (such as Decode's) has nothing to give it.
            const instance: @{full_type_ident.clone()} = Object.create(@{class_expr.clone()}.prototype);
            {#for (name_ident, value_expr) in field_data}
                instance.@{name_ident} = @{value_expr};
            {/for}
            return instance;
        }
    });

    // The standalone function goes below the class and the static method
    // inside it; merging keeps each stream's placement, which composing
    // one into the other would lose.
    let fn_name_ident = names.generic_function("DefaultValue");
    let standalone = ts_template! {
        export function @{fn_name_ident}(): @{full_type_ident} {
            return @{class_expr}.@{static_call}();
        }
    };
    Ok(standalone.merge(class_body))
}
