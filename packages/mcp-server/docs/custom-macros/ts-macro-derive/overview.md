# ts\_macro\_derive

The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a
Macroforge derive macro.

## Basic Syntax

Rust

```
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}
```

Note

The generated code refers to `macroforge_ts` by name, so depend on it under that name, without
renaming it in `Cargo.toml`.
