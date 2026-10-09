//! `Decode` for a type alias, by the shape the alias stands for.

mod fallback;
mod primitive;
mod union;
mod view;

use crate::ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use super::super::{EndecContainerOptions, decorator_validators};
use super::field::prepare::to_decode_field;
use super::object_decoder::object_decode_functions;
use super::types::DecodeField;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::return_types::{DECODE_CONTEXT, DECODE_ERROR, DECODE_OPTIONS, PENDING_REF};
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeRegistry};

/// What every type alias `Decode` generator shares.
struct AliasDecode<'a> {
    type_alias: &'a crate::ts_syn::DataTypeAlias,
    type_name: &'a str,
    type_ident: Ident,
    decode_context_ident: Ident,
    decode_context_expr: Expr,
    decode_error_expr: Expr,
    pending_ref_ident: Ident,
    pending_ref_expr: Expr,
    decode_options_ident: Ident,
    generic_decl: String,
    generic_args: String,
    full_type_name: String,
    type_registry: &'a TypeRegistry,
    caller_file_path: &'a str,
    file_imports: &'a [FileImportEntry],
}

pub(super) fn handle_type_alias(
    input: &DeriveInput,
    type_alias: &crate::ts_syn::DataTypeAlias,
) -> Result<TsStream, MacroforgeError> {
    let file_imports = input.context.import_registry.file_import_entries();
    let type_name = input.name();
    let decode_context_ident = ts_ident!(DECODE_CONTEXT);
    let pending_ref_ident = ts_ident!(PENDING_REF);

    // Build generic type signature if type has type params
    let names = TypeNames::new(type_name, type_alias.type_params());
    let full_type_name = names.full_type.clone();

    let TypeNames {
        generic_decl,
        generic_args,
        ..
    } = names;

    let alias = AliasDecode {
        type_alias,
        type_name,
        type_ident: ts_ident!(type_name),
        decode_context_expr: decode_context_ident.clone().into(),
        decode_context_ident,
        decode_error_expr: ts_ident!(DECODE_ERROR).into(),
        pending_ref_expr: pending_ref_ident.clone().into(),
        pending_ref_ident,
        decode_options_ident: ts_ident!(DECODE_OPTIONS),
        generic_decl,
        generic_args,
        full_type_name,
        type_registry: &input.context.type_registry,
        caller_file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
    };

    let mut validator_diagnostics = DiagnosticCollector::new();
    let validators = decorator_validators(
        &type_alias.inner.decorators,
        &format!("type '{type_name}'"),
        &mut validator_diagnostics,
    );
    if validator_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(validator_diagnostics.into_vec()).into());
    }

    if let Some(primitive) = type_alias.body().primitive_base() {
        return primitive::handle_primitive_type_alias(&alias, primitive, &validators);
    }
    if !validators.is_empty() {
        return Err(MacroforgeError::new(
            type_alias.inner.span,
            format!(
                "@endec validators on type '{type_name}' only apply to primitive and branded primitive aliases; put them on the fields instead"
            ),
        ));
    }

    if let Some(fields) = type_alias.as_object() {
        handle_object_type_alias(&alias, fields)
    } else if let Some(members) = type_alias.as_intersection() {
        match crate::builtin::derive::common::flatten_intersection_fields(
            members,
            alias.type_registry,
        ) {
            Some(fields) => handle_object_type_alias(&alias, &fields),
            None => fallback::handle_fallback_type_alias(&alias),
        }
    } else if let Some(members) = type_alias.as_union() {
        union::handle_union_type_alias(&alias, members)
    } else {
        fallback::handle_fallback_type_alias(&alias)
    }
}

fn handle_object_type_alias(
    alias: &AliasDecode,
    ir_fields: &[crate::ts_syn::abi::InterfaceFieldIR],
) -> Result<TsStream, MacroforgeError> {
    let container_opts = EndecContainerOptions::from_decorators(&alias.type_alias.inner.decorators);
    let names = TypeNames::new(alias.type_name, alias.type_alias.type_params());
    let mut diagnostics = DiagnosticCollector::new();
    let fields: Vec<DecodeField> = ir_fields
        .iter()
        .filter_map(|field| {
            to_decode_field(
                field.into(),
                &container_opts,
                &mut diagnostics,
                alias.type_registry,
                alias.caller_file_path,
                alias.file_imports,
                &names.params,
            )
        })
        .collect();
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    let mut result = object_decode_functions(&names, &container_opts, &fields);
    result.add_diagnostics(diagnostics.into_vec());
    Ok(result)
}
