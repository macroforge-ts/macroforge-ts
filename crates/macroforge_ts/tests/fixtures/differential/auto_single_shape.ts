// Differential fixture: Auto mode with a single shape.
//
// Dev expands inline (returning the wrapped value directly). Prod runs
// the megamorphism analyzer, sees 1 distinct shape, and shares via the
// runtime helper. Both expansions must log the same final value.

import { macroRules } from "macroforge/rules";

class User {
  constructor(public id: number) {}
}

const $wrap = macroRules({
  mode: "auto",
  expand: macroRules`
    ($x:Expr) => ({ wrapped: $x })
  `,
  runtime: "function __wrap(v) { return { wrapped: v }; }",
  call: macroRules`
    ($x:Expr) => __wrap($x)
  `,
});

const a = $wrap(User);
const b = $wrap(User);

console.log(JSON.stringify({ a: typeof a.wrapped, b: typeof b.wrapped }));
