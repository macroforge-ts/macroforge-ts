//! Test-only proc macros registered under the `test-macros` feature.
//!
//! These are gated behind a dedicated feature so production builds of
//! `macroforge_ts` never ship them. The `spec_tests` integration test
//! enables the feature (via `required-features` in Cargo.toml) so
//! snapshot fixtures under `tests/fixtures/ok/{attribute,call}/` can
//! exercise the attribute-macro and call-macro pipelines without
//! depending on an external crate.
//!
//! # Macros
//!
//! - `@traced` — attribute macro: wrap a function declaration with a
//!   counter increment so each invocation is observable via a global
//!   `__traced[fnName]` map.
//! - `$stringify` — call macro: quote its argument source as a string
//!   literal. `$stringify(a + b)` → `"a + b"`.
//! - `$concat_names` — call macro: join two identifier-shaped arguments
//!   with an underscore, emitted as a string literal.
//!   `$concat_names(foo, bar)` → `"foo_bar"`.

use crate::macros::{ts_macro, ts_macro_attribute};
use crate::ts_syn::{MacroforgeError, Patch, PatchCode, TargetIR, TsStream};

/// `@traced` — wrap a function so every call increments a counter on
/// `globalThis.__traced[fnName]`.
///
/// Emits a `Patch::Replace` spanning the decorated function. The
/// original signature is preserved verbatim; a single statement is
/// inserted at the start of the body that records the call.
#[ts_macro_attribute(traced, description = "Count calls to the decorated function")]
pub fn traced_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
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

    let signature = &full[..body_open];
    let body_inner = &full[body_open + 1..body_close];
    let fn_name = &f.name;

    let replacement = format!(
        "{sig}{{\n    (globalThis as any).__traced ??= {{}};\n    (globalThis as any).__traced[{name:?}] = ((globalThis as any).__traced[{name:?}] || 0) + 1;\n    {body}\n}}",
        sig = signature,
        name = fn_name,
        body = body_inner.trim(),
    );

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
/// A minimal call macro fixture: it takes arbitrary source text and
/// emits a double-quoted string literal containing that text.
#[ts_macro(
    stringify,
    description = "Quote the argument source as a string literal"
)]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    // Escape anything that would break out of a JS double-quoted string.
    let escaped = src
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    Ok(TsStream::from_string(format!("\"{escaped}\"")))
}

/// `$concat_names(a, b)` → `"a_b"`.
///
/// Another small call-macro fixture: splits its argument on the first
/// top-level comma, trims both halves, and emits them joined with an
/// underscore inside a string literal.
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
