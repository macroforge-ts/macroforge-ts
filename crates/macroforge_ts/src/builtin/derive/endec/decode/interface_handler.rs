use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DataInterface, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream};

use super::super::EndecContainerOptions;
use super::field::prepare::to_decode_field;
use super::object_decoder::object_decode_functions;
use super::types::DecodeField;
use crate::builtin::derive::common::TypeNames;

pub(super) fn handle_interface(
    input: &DeriveInput,
    interface: &DataInterface,
) -> Result<TsStream, MacroforgeError> {
    let container_opts = EndecContainerOptions::from_decorators(&interface.inner.decorators);
    let file_imports = input.context.import_registry.file_import_entries();
    let names = TypeNames::new(input.name(), interface.type_params());
    let mut diagnostics = DiagnosticCollector::new();
    let fields: Vec<DecodeField> = interface
        .fields()
        .iter()
        .filter_map(|field| {
            to_decode_field(
                field.into(),
                &container_opts,
                &mut diagnostics,
                &input.context.type_registry,
                input.context.file_name.as_str(),
                &file_imports,
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
