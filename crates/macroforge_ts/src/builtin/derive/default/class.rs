//! `Default` for a class: a static `defaultValue()` that builds an instance
//! without running a constructor, and a standalone function calling it.

use crate::ast::Expr;
use crate::builtin::derive::common::{
    ClassUse, DefaultFieldOptions, TypeNames, derive_class_fields, has_known_default, property_key,
    rendered, structural_helper,
};
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
    let class_expr: Expr = class_ident.into();
    let class_fields = derive_class_fields(input, class, "Default", ClassUse::Builds)?;

    let mut diagnostics = DiagnosticCollector::new();
    missing_default_derives(
        class_fields
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
    let missing_defaults: Vec<&str> = class_fields
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
    let mut field_data: Vec<(String, String, Expr)> = Vec::new();
    for field in &class_fields {
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
        field_data.push((field.name.clone(), field.ts_type.clone(), value_expr));
    }

    // The fields are written out under their declared types, so a default
    // of the wrong type fails to type-check, and handed to a new instance
    // whole: assignment would be refused for a `readonly` field, and no type
    // over the instance names a private one.
    let members: Vec<String> = field_data
        .iter()
        .map(|(name, ts_type, _)| {
            let key = property_key(name);
            // An unannotated field takes whatever its initializer gives.
            let checked = if ts_type == "any" { "unknown" } else { ts_type };
            rendered(ts_template! { @{key}: @{checked} })
        })
        .collect();
    let members = members.join("; ");
    let instance_of: Expr = ts_ident!(structural_helper("instanceOf")).into();
    // Like Rust's derive, a default runs no constructor: one that takes
    // arguments (such as Decode's) has nothing to give it.
    let body = ts_template! {
        const fields: { @{members} } = {
            {#for (name, _, value_expr) in &field_data}
                @{property_key(name)}: @{value_expr},
            {/for}
        };
        return @{instance_of}(@{class_expr}.prototype, fields);
    };
    Ok(names.class_function(
        "defaultValue",
        "DefaultValue",
        "",
        "",
        &names.full_type,
        body,
    ))
}
