## How It Works

1. **Rust compile time:** the template is turned into Rust code that writes TypeScript text, with
   the control flow as ordinary Rust `if`, `for` and `match`.
2. **Macro run time:** that code runs and builds the text, interpolating your values.
3. **Result:** a `TsStream` that can be returned directly as macro output. The text is not checked
   here, so a syntax error in it shows up when the expanded file is compiled;
   [`macroforge expand`](../../docs/custom-macros/testing-and-debugging#expand-a-file) shows exactly
   what was generated.

## Choosing a Macro

- Use `ts_template!` to generate code, especially with loops and conditions.
- Use `ts_quote!` when you need an AST node: to analyse or transform syntax, or to have a fixed
  snippet checked when your crate compiles.
- Keep templates readable: compute values in Rust before the template.
- Split large outputs into several templates and combine them with `{$typescript}`.
