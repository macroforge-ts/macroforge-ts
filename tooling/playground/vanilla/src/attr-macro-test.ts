// End-to-end playground fixture for attribute and callable proc macros.
//
// `@traced`, `$stringify`, and `$concat_names` are `#[ts_macro_attribute]`
// and `#[ts_macro]` macros in the playground crate
// (`tooling/playground/macro/src/attrs.rs`).
//
// - `@traced` wraps a function so each call increments a counter on
//   `globalThis.__traced[fnName]`.
// - `$stringify(expr)` emits a string literal with the raw source of
//   `expr`.
// - `$concat_names(a, b)` emits `"a_b"` as a string literal.

/** import macro { traced, $stringify, $concat_names } from "@playground/macro" */

/** @traced */
export function tracedAdd(a: number, b: number): number {
    return a + b;
}

/** @traced */
export function tracedGreet(name: string): string {
    return `hello, ${name}`;
}

export const stringifiedExpr = $stringify(1 + 2 * 3);
export const stringifiedIdent = $stringify(myVariable);
export const concatUserName = $concat_names(user, name);
export const concatDbHost = $concat_names(db, host);
