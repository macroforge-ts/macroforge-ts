// End-to-end cross-file declarative macro fixture.
//
// `$vec` and `$identity` are declared in `./_decl_macros_lib.ts`. This
// file imports them via the JSDoc `/** import macro */` directive, and
// the macroforge build pass should rewrite every call site below using
// the definitions from the library file — the runtime values in this
// module should be plain arrays / numbers with no trace of any tag
// function.

/** import macro { $vec, $identity } from "./_decl_macros_lib" */

export const crossFileEmpty = $vec();
export const crossFileThree = $vec(1, 2, 3);
export const crossFileExpr = $vec(10 + 1, 20 - 5, 7 * 2);
export const crossFileId = $identity(42);
