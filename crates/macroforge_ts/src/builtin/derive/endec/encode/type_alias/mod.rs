//! `Encode` for a type alias, by the shape the alias stands for.

mod fallback;
mod primitive;
mod union;
mod view;

use crate::ast::Ident;
use crate::builtin::derive::endec::coder::{TypeScope, unread_intersection};
use crate::builtin::derive::endec::reserved_keys::check_field_keys;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::type_alias::TypeBody;
use crate::ts_syn::abi::ir::{FileImportEntry, TypeParamIR, TypeRegistry};
use crate::ts_syn::{
    DataTypeAlias, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use convert_case::{Case, Casing};

use super::field::prepare::to_encode_field;
use super::object_encoder::object_encode_functions;
use super::types::EncodeField;
use crate::builtin::derive::common::{Declared, TypeNames};
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::derive::endec::type_arguments::{declared_param_coders, passed_param_coders};
use crate::builtin::derive::endec::union_guard::Direction;
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
    /// The coders of the alias's type parameters, as `encodeWithContext`
    /// declares them after its context.
    coders: String,
    /// The same coders, as `encode` hands them on.
    passed_coders: String,
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

    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
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
        coders: declared_param_coders(Direction::Encode, &names.params),
        passed_coders: passed_param_coders(Direction::Encode, &names.params),
    };

    if let Some(primitive) = type_alias.body().primitive_base() {
        return primitive::encode_primitive_alias(&alias, primitive);
    }
    // A newtype's value is the value of the type it brands.
    let body = type_alias.body().unbranded();
    let effective_fields = match body {
        TypeBody::Object { fields } => Some(fields.clone()),
        TypeBody::Intersection(members) => {
            crate::builtin::derive::common::flatten_intersection_fields(
                members,
                Declared::of(input, &file_imports),
            )
        }
        TypeBody::Union(_)
        | TypeBody::Tuple(_)
        | TypeBody::Alias(_)
        | TypeBody::Other(_)
        | TypeBody::Newtype(_) => None,
    };
    if let Some(ir_fields) = &effective_fields {
        let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
        let names = TypeNames::new(type_name, type_params);
        let mut diagnostics = DiagnosticCollector::new();
        let key_scope = TypeScope {
            registry: type_registry,
            file_path: caller_file_path,
            file_imports: &file_imports,
            type_params: &names.params,
        };
        check_field_keys(
            Direction::Encode,
            &container_opts,
            ir_fields.iter().map(SourceField::from),
            &key_scope,
            &mut diagnostics,
        );
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
    } else if body.as_union().is_some() {
        union::encode_union_alias(&alias)
    } else if body.as_intersection().is_some() {
        Err(MacroforgeError::new(
            type_alias.inner.span,
            format!("@derive(Encode): {}", unread_intersection(type_name)),
        ))
    } else {
        fallback::encode_fallback_alias(&alias)
    }
}
