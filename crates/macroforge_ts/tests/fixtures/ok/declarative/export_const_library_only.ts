// Library-only file: declares `export const` macros with no call sites.
//
// This is the shape of a cross-file macro library — the definitions
// get consumed by other files via `/** import macro */` but the
// library file itself also goes through expansion in the Vite
// pipeline. With no call sites there's nothing to expand, so the
// expander just erases the declarations + `macroRules` import.
import { macroRules } from 'macroforge/rules';

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

export const $identity = macroRules`
  ($x:Expr) => $x
`;
