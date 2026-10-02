# Output and Imports

A macro returns a `TsStream`: generated TypeScript plus everything that travels with it, such as
imports, patches and warnings. This page covers each part.

## What a Macro Returns

A function marked with `#[ts_macro_derive]`, `#[ts_macro_attribute]` or `#[ts_macro]` returns
`Result<TsStream, E>`. On `Ok`, the host turns the stream into a `MacroResult`; on `Err`, the error
becomes the result's diagnostics. `E` is anything that converts into a `MacroResult`, usually
`MacroforgeError`; see [Errors and Diagnostics](../../docs/custom-macros/diagnostics).

What happens to the stream's source depends on the kind of macro:

| Kind      | The stream's source                                                               |
| --------- | --------------------------------------------------------------------------------- |
| Derive    | Inserted next to the target, at the stream's [insert position](#insert-positions) |
| Attribute | Ignored: an attribute macro changes code through [patches](#patches)              |
| Call      | Replaces the `$name(...)` call expression                                         |

Imports, patches, cross-module suffixes and warnings on the stream apply for every kind.
