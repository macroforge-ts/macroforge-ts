use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroOne, description = "First test macro")]
pub fn macro_one(input: TsStream) -> Result<TsStream, MacroforgeError> {
    Ok(input)
}

#[ts_macro_derive(MacroTwo, description = "Second test macro")]
pub fn macro_two(input: TsStream) -> Result<TsStream, MacroforgeError> {
    Ok(input)
}

fn main() {}
