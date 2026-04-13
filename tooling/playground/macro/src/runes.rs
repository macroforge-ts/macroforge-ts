//! Svelte-runes-style reactive proc macros.
//!
//! Function-like call macros invoked via `$state(...)`, `$derived(...)`,
//! `$effect(...)` in TypeScript. They transform rune-style invocations
//! into calls to a user-provided reactive runtime.
//!
//! ```typescript
//! /** import macro { $state, $derived, $effect } from "@playground/macro" */
//! import { createSignal, createDerived, createEffect } from './runes-runtime';
//!
//! let count = $state(0);
//! // → let count = createSignal(0);
//!
//! let doubled = $derived(count.value * 2);
//! // → let doubled = createDerived(() => count.value * 2);
//!
//! $effect(console.log(count.value));
//! // → createEffect(() => { console.log(count.value); });
//! ```

use macroforge_ts::macros::ts_macro;
use macroforge_ts::ts_syn::{MacroforgeError, TsStream};

/// `$state(initial)` → `createSignal(initial)`
///
/// Creates a reactive signal. The returned value has a `.value`
/// property for reads and writes.
#[ts_macro(state, description = "Create a reactive signal")]
pub fn state_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let args = input.source().trim();
    if args.is_empty() {
        return Err(MacroforgeError::new_global(
            "$state requires an initial value, e.g. $state(0)",
        ));
    }
    Ok(TsStream::from_string(format!("createSignal({})", args)))
}

/// `$derived(expression)` → `createDerived(() => expression)`
///
/// Creates a derived/computed value. The expression is wrapped in a
/// thunk so the reactive runtime can re-evaluate it when dependencies
/// change.
#[ts_macro(derived, description = "Create a derived reactive value")]
pub fn derived_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let args = input.source().trim();
    if args.is_empty() {
        return Err(MacroforgeError::new_global(
            "$derived requires an expression, e.g. $derived(count.value * 2)",
        ));
    }
    Ok(TsStream::from_string(format!(
        "createDerived(() => {})",
        args
    )))
}

/// `$effect(expression)` → `createEffect(() => { expression; })`
///
/// Creates a reactive side effect. The expression is wrapped in a
/// void-returning thunk that re-runs when its reactive dependencies
/// change. If the argument already looks like a function, it's passed
/// directly.
#[ts_macro(effect, description = "Create a reactive side effect")]
pub fn effect_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let args = input.source().trim();
    if args.is_empty() {
        return Err(MacroforgeError::new_global(
            "$effect requires an expression or function",
        ));
    }
    // If the argument already looks like a function/arrow, pass it directly.
    if args.starts_with("()") || args.starts_with("function") || args.starts_with("async") {
        Ok(TsStream::from_string(format!("createEffect({})", args)))
    } else {
        Ok(TsStream::from_string(format!(
            "createEffect(() => {{ {}; }})",
            args
        )))
    }
}
