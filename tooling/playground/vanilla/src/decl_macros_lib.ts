// Library file defining declarative macros that `cross-file-decl.ts` imports.
//
// The leading underscore keeps this file out of ambiguous alphabetical
// sorts — it's a library, not an entry point.

import { macroRules } from "macroforge/rules";

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

export const $identity = macroRules`
  ($x:Expr) => $x
`;
