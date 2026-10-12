use crate::builtin::derive::endec::reserved_keys::check_reserved_keys;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::Direction;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DataInterface, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream};

use super::super::EndecContainerOptions;
use super::field::prepare::to_decode_field;
use super::object_decoder::object_decode_functions;
use super::types::DecodeField;
use crate::builtin::derive::common::{TypeNames, derive_interface_fields};

pub(super) fn handle_interface(
    input: &DeriveInput,
    interface: &DataInterface,
) -> Result<TsStream, MacroforgeError> {
    let container_opts = EndecContainerOptions::from_decorators(&interface.inner.decorators);
    let file_imports = input.context.import_registry.file_import_entries();
    let names = TypeNames::new(input.name(), interface.type_params());
    let interface_fields = derive_interface_fields(input, interface, "Decode")?;
    let mut diagnostics = DiagnosticCollector::new();
    check_reserved_keys(
        Direction::Decode,
        &container_opts,
        interface_fields.iter().map(SourceField::from),
        &mut diagnostics,
    );
    let fields: Vec<DecodeField> = interface_fields
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
