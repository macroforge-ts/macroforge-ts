use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MyMacro, description = "A test macro")]
pub fn my_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    Ok(input)
}

fn main() {}
