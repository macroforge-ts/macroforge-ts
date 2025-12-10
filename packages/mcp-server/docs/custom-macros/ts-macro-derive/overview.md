# ts_macro_derive

*The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*

## Basic Syntax

```rust
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}
```