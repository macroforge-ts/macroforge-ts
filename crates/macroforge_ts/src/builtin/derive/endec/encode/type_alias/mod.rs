//! `Encode` for a type alias, by the shape the alias stands for.

mod fallback;
mod primitive;
mod union;
mod view;

use crate::ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeParamIR, TypeRegistry};
use crate::ts_syn::{
    DataTypeAlias, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use convert_case::{Case, Casing};

use super::field::prepare::to_encode_field;
use super::object_encoder::object_encode_functions;
use super::types::EncodeField;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::return_types::ENCODE_CONTEXT;

/// What every type alias `Encode` generator shares.
struct AliasEncode<'a> {
    type_alias: &'a DataTypeAlias,
    type_registry: &'a TypeRegistry,
    caller_file_path: &'a str,
    file_imports: &'a [FileImportEntry],
    type_name: &'a str,
    type_params: &'a [TypeParamIR],
    type_params_ident: Option<Ident>,
    full_type_ident: Ident,
    fn_encode_ident: Ident,
    fn_encode_internal_ident: Ident,
    encode_context_ident: Ident,
    encode_context_expr: Expr,
}

pub(super) fn handle_type_alias(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
) -> Result<TsStream, MacroforgeError> {
    let type_registry = &input.context.type_registry;
    let caller_file_path = input.context.file_name.as_str();
    let file_imports = input.context.import_registry.file_import_entries();
    let type_name = input.name();

    // Build generic type signature if type has type params
    let type_params = type_alias.type_params();
    let names = TypeNames::new(type_name, type_params);
    // The declared parameters, inside the brackets the templates write.
    let type_params_ident: Option<Ident> = names
        .generic_decl
        .strip_prefix('<')
        .and_then(|declared| declared.strip_suffix('>'))
        .map(|declared| ts_ident!(declared));
    let full_type_ident = names.full_type_ident();

    // Generate function names based on naming style
    let fn_encode_ident = ts_ident!("{}Encode", type_name.to_case(Case::Camel));
    let fn_encode_internal_ident = ts_ident!("{}EncodeWithContext", type_name.to_case(Case::Camel));

    // Create Expr version of ENCODE_CONTEXT for expression positions
    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
    let encode_context_expr: Expr = encode_context_ident.clone().into();
    let alias = AliasEncode {
        type_alias,
        type_registry,
        caller_file_path,
        file_imports: &file_imports,
        type_name,
        type_params,
        type_params_ident,
        full_type_ident,
        fn_encode_ident,
        fn_encode_internal_ident,
        encode_context_ident,
        encode_context_expr,
    };

    if let Some(primitive) = type_alias.body().primitive_base() {
        return primitive::encode_primitive_alias(&alias, primitive);
    }
    let effective_fields =
        crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
    if let Some(ir_fields) = &effective_fields {
        let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
        let names = TypeNames::new(type_name, type_params);
        let mut diagnostics = DiagnosticCollector::new();
        let fields: Vec<EncodeField> = ir_fields
            .iter()
            .filter_map(|field| {
                to_encode_field(
                    field.into(),
                    &container_opts,
                    &mut diagnostics,
                    type_registry,
                    caller_file_path,
                    &file_imports,
                    &names.params,
                )
            })
            .collect();
        if diagnostics.has_errors() {
            return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
        }
        let mut result =
            object_encode_functions(&names, container_opts.tag_field_or_default(), &fields);
        result.add_diagnostics(diagnostics.into_vec());
        Ok(result)
    } else if type_alias.as_union().is_some() {
        union::encode_union_alias(&alias)
    } else {
        fallback::encode_fallback_alias(&alias)
    }
}
