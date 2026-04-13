// End-to-end playground fixture for declarative (pattern-matching) macros.
//
// This file defines a few `const $name = macroRules`...`` declarations and
// invokes them. The macroforge build pipeline is expected to erase the
// definitions and inline the expansions at each call site, so by the
// time Vite serves this file the runtime values below should be plain
// arrays / numbers / expressions with no trace of the macro template.

import { macroRules } from "macroforge/rules";

// Variadic array constructor with two arms: empty and one-or-more.
const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

// Identity macro with a single-arm pattern.
const $id = macroRules`
  ($x:Expr) => $x
`;

// Block-bodied macro with a macro-local temporary — tests hygiene.
const $withTemp = macroRules`
  ($x:Expr) => {
    const __acc = $x;
    __acc + 1
  }
`;

export const emptyVec = $vec();
export const threeVec = $vec(1, 2, 3);
export const exprVec = $vec(10 + 1, 20 - 5, 7 * 2);
export const identityCall = $id(42);
export const withTempResult = $withTemp(10);

// The test harness uses this to confirm the macro definition was erased.
// If the macroforge pipeline skipped declarative macros this would throw
// at import time because `macroRules` is a sentinel that throws when
// called.
export const declarativeMacrosErased = true;
