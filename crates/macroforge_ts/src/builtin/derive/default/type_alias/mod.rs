//! `Default` for a type alias, by the shape the alias stands for.

mod union;

use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::{DefaultFieldOptions, rendered};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DataTypeAlias, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::object::object_default;
use super::values::DefaultSources;
use crate::builtin::derive::common::TypeNames;

pub(super) fn type_alias_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    sources: &DefaultSources,
) -> Result<TsStream, MacroforgeError> {
    let names = TypeNames::new(input.name(), type_alias.type_params());
    let effective_fields =
        crate::builtin::derive::common::get_effective_fields(type_alias, sources.registry);
    if let Some(fields) = &effective_fields {
        object_default(&names, fields, input.decorator_span(), sources)
    } else if let Some(members) = type_alias.as_union() {
        union::union_default(input, type_alias, members, sources)
    } else {
        fallback_default(input, type_alias, &names)
    }
}

/// A tuple, primitive or other alias: its `@default(value)`, or for a
/// primitive alias the primitive's default.
fn fallback_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    names: &TypeNames,
) -> Result<TsStream, MacroforgeError> {
    let type_name = names.type_name.as_str();
    let full_type_name = names.full_type.as_str();
    let full_type_ident = names.full_type_ident();
    let generic_decl_ident = ts_ident!(names.generic_decl.as_str());
    // Tuple or simple alias: check for explicit @default(value)
    let default_opts = DefaultFieldOptions::from_decorators(
        &input
            .attrs
            .iter()
            .map(|a| a.inner.clone())
            .collect::<Vec<_>>(),
    );

    // A primitive alias defaults like a primitive field, and the
    // value is cast because a branded alias is not its base type.
    let primitive = type_alias.body().primitive_base();

    // The primitive's zero may fail the alias's validators, so a
    // validated alias names its default, as a Rust newtype without a
    // valid zero has no derived `Default`.
    if primitive.is_some() && default_opts.value.is_none() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = crate::builtin::derive::endec::decorator_validators(
            &type_alias.inner.decorators,
            &format!("type '{type_name}'"),
            &mut diagnostics,
        );
        if diagnostics.has_errors() {
            return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
        }
        if !validators.is_empty() {
            return Err(MacroforgeError::new(
                input.decorator_span(),
                format!(
                    "@derive(Default) on '{type_name}' requires @default(value): its validators may reject the default of its base type"
                ),
            ));
        }
    }

    let default_value = match (default_opts.value, primitive) {
        (Some(value), Some(_)) => Some(rendered(ts_template! { (@{value}) as @{&full_type_name} })),
        (None, Some(base)) => {
            let zero = crate::builtin::derive::common::get_type_default(base);
            Some(rendered(ts_template! { @{zero} as @{&full_type_name} }))
        }
        (value, None) => value,
    };

    if let Some(default_variant) = default_value {
        let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
        let return_expr = Expr::parse(&default_variant).map_err(|err| {
            MacroforgeError::new(
                input.decorator_span(),
                format!(
                    "@derive(Default): invalid default expression for '{}': {err:?}",
                    type_name
                ),
            )
        })?;
        Ok(ts_template! {
            export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                return @{return_expr};
            }
        })
    } else {
        Err(MacroforgeError::new(
            input.decorator_span(),
            format!(
                "@derive(Default) on type '{}' requires @default(value) to specify the default.",
                type_name
            ),
        ))
    }
}
