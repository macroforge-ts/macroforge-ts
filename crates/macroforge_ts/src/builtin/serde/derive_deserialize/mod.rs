//! # Deserialize Macro Implementation
//!
//! The `Deserialize` macro generates JSON deserialization methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! See the original module documentation for full details on generated output,
//! return types, cycle/forward-reference support, validation, field-level options,
//! container-level options, and union type deserialization.

mod class_handler;
mod enum_handler;
pub(crate) mod field_processing;
pub(crate) mod helpers;
mod interface_handler;
mod type_alias_handler;
pub(crate) mod types;
pub(crate) mod validation;

#[cfg(test)]
mod tests;

pub use validation::generate_validation_condition;

use crate::macros::ts_macro_derive;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    Deserialize,
    description = "Generates deserialization methods with cycle/forward-reference support (fromStringifiedJSON, deserializeWithContext)",
    attributes((serde, "Configure deserialization for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_deserialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => class_handler::handle_class(&input),
        Data::Enum(_) => enum_handler::handle_enum(&input),
        Data::Interface(_) => interface_handler::handle_interface(&input),
        Data::TypeAlias(_) => type_alias_handler::handle_type_alias(&input),
    }
}
