//! `Default` for a type alias, by the shape the alias stands for.

mod union;

use crate::builtin::derive::common::function_prefix;

use crate::ast::Expr;
use crate::builtin::derive::common::{DefaultFieldOptions, rendered};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::type_alias::TypeBody;
use crate::ts_syn::{
    DataTypeAlias, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::object::object_default;
use super::values::{DefaultSources, missing_default_derives};
use crate::builtin::derive::common::TypeNames;

pub(super) fn type_alias_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    sources: &DefaultSources,
) -> Result<TsStream, MacroforgeError> {
    let names = TypeNames::new(input.name(), type_alias.type_params());
    let effective_fields =
        crate::builtin::derive::common::get_effective_fields(type_alias, sources.declared());
    if let Some(fields) = &effective_fields {
        object_default(&names, fields, input.decorator_span(), sources)
    } else if let Some(members) = type_alias.as_union() {
        union::union_default(input, type_alias, members, sources)
    } else {
        fallback_default(input, type_alias, &names, sources)
    }
}

/// The default of an alias of another type, or of a tuple, with no
/// `@default(value)` of its own: what a field of the type it stands for
/// defaults to, with that type. `None` for a type the derive can write no
/// default for, and the errors of one whose default is missing something.
fn aliased_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    names: &TypeNames,
    sources: &DefaultSources,
) -> Result<Option<(String, String)>, MacroforgeError> {
    // A newtype defaults as the type it brands, which is its value.
    let aliased = match type_alias.body().unbranded() {
        TypeBody::Alias(target) => target.to_string(),
        // A template literal type has a default text; no other written-out
        // type has a default the derive can read off.
        TypeBody::Other(target) if target.trim_start().starts_with('`') => target.to_string(),
        TypeBody::Tuple(elements) => {
            let elements = elements.join(", ");
            rendered(ts_template! { [@{elements}] })
        }
        TypeBody::Other(_)
        | TypeBody::Object { .. }
        | TypeBody::Union(_)
        | TypeBody::Intersection(_)
        | TypeBody::Newtype(_) => return Ok(None),
    };
    // A function has no default, and a type parameter none the alias knows.
    if aliased.contains("=>") {
        return Ok(None);
    }
    let mut diagnostics = DiagnosticCollector::new();
    missing_default_derives(
        std::iter::once((
            names.type_name.as_str(),
            aliased.as_str(),
            input.decorator_span(),
        )),
        names,
        sources,
        &mut diagnostics,
    );
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    let default = crate::builtin::derive::common::get_type_default_with_registry(
        &aliased,
        sources.registry,
        sources.caller_file_path,
        sources.file_imports,
    );
    Ok((default != "undefined").then_some((default, aliased)))
}

/// A tuple, primitive or other alias: its `@default(value)`, or the default
/// of the primitive, the tuple or the type it stands for.
fn fallback_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    names: &TypeNames,
    sources: &DefaultSources,
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
        // A newtype's brand is in its type alone, so the value is cast to it.
        (Some(value), None) if type_alias.body().as_newtype().is_some() => {
            Some(rendered(ts_template! { (@{value}) as @{&full_type_name} }))
        }
        (Some(value), None) => Some(value),
        // An alias of another type is that type, and defaults as it does:
        // the value is written as that type first, which an empty array
        // alone would not be.
        (None, None) => {
            aliased_default(input, type_alias, names, sources)?.map(|(default, aliased)| {
                rendered(ts_template! { ((@{default}) as @{aliased}) as @{&full_type_name} })
            })
        }
    };

    if let Some(default_variant) = default_value {
        let fn_name_ident = ts_ident!("{}DefaultValue", function_prefix(type_name));
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
