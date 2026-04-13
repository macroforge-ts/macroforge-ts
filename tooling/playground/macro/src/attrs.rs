//! Attribute macros for the playground.
//!
//! These demonstrate `#[ts_macro_attribute]` — proc macros invoked via
//! `/** @MacroName */` decorators on declarations.

use macroforge_ts::macros::{ts_macro, ts_macro_attribute};
use macroforge_ts::ts_syn::{MacroforgeError, Patch, PatchCode, TargetIR, TsStream};

/// `@traced` — wrap a function so every call increments a counter on
/// `globalThis.__traced[fnName]`.
///
/// Input:
/// ```ts
/// /** @traced */
/// export function add(a: number, b: number): number { return a + b; }
/// ```
///
/// Output replaces the entire function declaration with an equivalent
/// declaration whose body starts with a counter bump:
/// ```ts
/// export function add(a: number, b: number): number {
///     (globalThis as any).__traced ??= {};
///     (globalThis as any).__traced["add"] =
///         ((globalThis as any).__traced["add"] || 0) + 1;
///     return a + b;
/// }
/// ```
#[ts_macro_attribute(traced, description = "Count calls to the decorated function")]
pub fn traced_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    // The expander passes the function's full source text as `input.source()`
    // and the structured `FunctionIR` via `input.context().target`. We prefer
    // the structured data for the name and the raw text for signature/body
    // slicing — this avoids relying on the shape of OXC's span math.
    let ctx = input
        .context()
        .ok_or_else(|| MacroforgeError::new_global("@traced: no macro context available"))?;

    let TargetIR::Function(f) = &ctx.target else {
        return Err(MacroforgeError::new_global(
            "@traced can only be applied to function declarations",
        ));
    };

    let full = input.source();
    let body_open = full
        .find('{')
        .ok_or_else(|| MacroforgeError::new_global("@traced: function has no body"))?;
    let body_close = full
        .rfind('}')
        .ok_or_else(|| MacroforgeError::new_global("@traced: unterminated function"))?;
    if body_close <= body_open {
        return Err(MacroforgeError::new_global("@traced: malformed function"));
    }

    let signature = &full[..body_open]; // includes whitespace before `{`
    let body_inner = &full[body_open + 1..body_close];
    let fn_name = &f.name;

    let replacement = format!(
        "{sig}{{\n    (globalThis as any).__traced ??= {{}};\n    (globalThis as any).__traced[{name:?}] = ((globalThis as any).__traced[{name:?}] || 0) + 1;\n    {body}\n}}",
        sig = signature,
        name = fn_name,
        body = body_inner.trim(),
    );

    // Attribute macros emit their output by pushing a `Patch::Replace` over
    // the target span. The TsStream's tokens are only consumed for derive
    // macros; attribute patches are applied verbatim to the runtime output.
    let mut out = TsStream::from_string(String::new());
    out.runtime_patches.push(Patch::Replace {
        span: f.span,
        code: PatchCode::Text(replacement),
        source_macro: Some("traced".to_string()),
    });

    Ok(out)
}

/// `$stringify(expr)` → `"<raw source of expr>"`.
///
/// Quotes its argument source as a double-quoted string literal.
#[ts_macro(
    stringify,
    description = "Quote the argument source as a string literal"
)]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    let escaped = src
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    Ok(TsStream::from_string(format!("\"{escaped}\"")))
}

/// `$concat_names(a, b)` → `"a_b"`.
///
/// Splits args on the first top-level comma and joins them with `_`.
#[ts_macro(
    concat_names,
    description = "Join two identifier args with an underscore"
)]
pub fn concat_names_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source();
    let Some(comma_idx) = src.find(',') else {
        return Err(MacroforgeError::new_global(
            "$concat_names expects exactly two comma-separated arguments",
        ));
    };
    let left = src[..comma_idx].trim();
    let right = src[comma_idx + 1..].trim();
    if left.is_empty() || right.is_empty() {
        return Err(MacroforgeError::new_global(
            "$concat_names requires both arguments to be non-empty",
        ));
    }
    Ok(TsStream::from_string(format!("\"{left}_{right}\"")))
}
