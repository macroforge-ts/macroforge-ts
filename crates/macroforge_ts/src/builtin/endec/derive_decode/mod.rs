//! # Decode Macro Implementation
//!
//! The `Decode` macro generates JSON decoding methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Output
//!
//! For **classes** (`class_handler`), the macro generates static methods plus
//! standalone functions (`{name}Decode`, `{name}DecodeWithContext`,
//! `{name}Is`) that delegate to them:
//!
//! - `static decode(input: unknown, opts?: DecodeOptions):
//!   { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
//!   Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all
//!   decoded objects.
//! - `static decodeWithContext(value, ctx): T | PendingRef` - Internal method used
//!   for nested/cyclic graphs; throws `DecodeError` on structural errors
//! - `static hasShape(obj): boolean` - Checks all required JSON keys are present
//! - `static is(value): value is T` - Type guard: instanceof check, then `hasShape`,
//!   then a full `decode`
//! - `static validateField(field, value)` / `static validateFields(partial)` - Run the
//!   field validators without decoding
//! - A synthesized `constructor(props)` that assigns all decoded fields
//!
//! **Interfaces** and **type aliases** get the standalone-function forms of the same
//! surface (there is no class to attach statics to). **Enums** (`enum_handler`) get
//! `{name}Decode`/`{name}DecodeWithContext`/`{name}Is`; unlike the other
//! shapes, the enum `decode` function **throws** an `Error` on invalid values
//! rather than returning a result union.
//!
//! A **type alias of a primitive**, plain (`type Port = number`) or branded
//! (`type Meters = $Newtype<number>`), decodes to that primitive: `decode` rejects
//! any other `typeof`, `hasShape` checks the `typeof`, and `is` additionally runs
//! the alias's validators. Validators go on the alias itself:
//!
//! ```typescript
//! /** @derive(Decode) */
//! /** @endec(nonNegative, finite) */
//! type Meters = $Newtype<number>;
//!
//! metersDecode(-1);   // { success: false, errors: [{ field: "_root", ... }] }
//! metersIs("12");     // false
//! ```
//!
//! Alias-level validators on any other alias shape are an expansion error; put
//! them on the fields instead.
//!
//! Validation (see `validation`) runs during decoding and reports failures
//! through the `errors` array of the result union. Field-level options are parsed by
//! `field_processing`; see the parent endec module for the option and validator
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

use crate::macros::ts_macro_derive;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    Decode,
    description = "Generates decoding methods with cycle/forward-reference support (decode, decodeWithContext)",
    attributes((endec, "Configure decoding for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_decode_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => class_handler::handle_class(&input),
        Data::Enum(enum_data) => enum_handler::handle_enum(&input, enum_data),
        Data::Interface(_) => interface_handler::handle_interface(&input),
        Data::TypeAlias(_) => type_alias_handler::handle_type_alias(&input),
    }
}
