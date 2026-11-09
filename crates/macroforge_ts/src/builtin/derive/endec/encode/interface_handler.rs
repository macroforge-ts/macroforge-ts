//! `Encode` for an interface: the shared object encoder.

use crate::builtin::derive::endec::coder::TypeScope;
use crate::builtin::derive::endec::reserved_keys::check_field_keys;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::Direction;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DataInterface, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream};

use super::field::prepare::to_encode_field;
use super::object_encoder::object_encode_functions;
use super::types::EncodeField;
use crate::builtin::derive::common::{TypeNames, derive_interface_fields};
use crate::builtin::derive::endec::EndecContainerOptions;

pub(super) fn handle_interface(
    input: &DeriveInput,
    interface: &DataInterface,
) -> Result<TsStream, MacroforgeError> {
    let container_opts = EndecContainerOptions::from_decorators(&interface.inner.decorators);
    let file_imports = input.context.import_registry.file_import_entries();
    let names = TypeNames::new(input.name(), interface.type_params());
    let interface_fields = derive_interface_fields(input, interface, "Encode")?;
    let mut diagnostics = DiagnosticCollector::new();
    let key_scope = TypeScope {
        registry: &input.context.type_registry,
        file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
        type_params: &names.params,
    };
    check_field_keys(
        Direction::Encode,
        &container_opts,
        interface_fields.iter().map(SourceField::from),
        &key_scope,
        &mut diagnostics,
    );
    let fields: Vec<EncodeField> = interface_fields
        .iter()
        .filter_map(|field| {
            to_encode_field(
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
    let mut result =
        object_encode_functions(&names, container_opts.tag_field_or_default(), &fields);
    result.add_diagnostics(diagnostics.into_vec());
    Ok(result)
}
