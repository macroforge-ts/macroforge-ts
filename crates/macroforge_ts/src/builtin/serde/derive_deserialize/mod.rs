//! # Deserialize Macro Implementation
//!
//! The `Deserialize` macro generates JSON deserialization methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Output
//!
//! For **classes** (`class_handler`), the macro generates static methods plus
//! standalone functions (`{name}Deserialize`, `{name}DeserializeWithContext`,
//! `{name}Is`) that delegate to them:
//!
//! - `static deserialize(input: unknown, opts?: DeserializeOptions):
//!   { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
//!   Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all
//!   deserialized objects.
//! - `static deserializeWithContext(value, ctx): T | PendingRef` - Internal method used
//!   for nested/cyclic graphs; throws `DeserializeError` on structural errors
//! - `static hasShape(obj): boolean` - Checks all required JSON keys are present
//! - `static is(value): value is T` - Type guard: instanceof check, then `hasShape`,
//!   then a full `deserialize`
//! - `static validateField(field, value)` / `static validateFields(partial)` - Run the
//!   field validators without deserializing
//! - A synthesized `constructor(props)` that assigns all deserialized fields
//!
//! **Interfaces** and **type aliases** get the standalone-function forms of the same
//! surface (there is no class to attach statics to). **Enums** (`enum_handler`) get
//! `{name}Deserialize`/`{name}DeserializeWithContext`/`{name}Is`; unlike the other
//! shapes, the enum `deserialize` function **throws** an `Error` on invalid values
//! rather than returning a result union.
//!
//! Validation (see `validation`) runs during deserialization and reports failures
//! through the `errors` array of the result union. Field-level options are parsed by
//! `field_processing`; see the parent serde module for the option and validator
//! reference.

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
    description = "Generates deserialization methods with cycle/forward-reference support (deserialize, deserializeWithContext)",
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
