// Multiple `export const` macro declarations in one file.
//
// Both declarations should be fully erased — including their `export`
// keywords — so no orphan `export` remains in the output.
import { macroRules } from 'macroforge/rules';

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

export const $id = macroRules`
  ($x:Expr) => $x
`;

const empty = $vec();
const xs = $vec(1, 2, 3);
const forty = $id(40);
