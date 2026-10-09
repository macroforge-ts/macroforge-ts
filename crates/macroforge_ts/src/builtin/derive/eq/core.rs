use crate::macros::ts_macro_derive;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    Eq,
    description = "Marks PartialEq equality as total, like Rust's Eq; generates no code"
)]
pub fn derive_eq_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    // `Eq` holds for any type whose equality is total; it adds no behavior.
    match input.data {
        Data::Class(_) | Data::Enum(_) | Data::Interface(_) | Data::TypeAlias(_) => {
            Ok(TsStream::from_string(String::new()))
        }
    }
}
