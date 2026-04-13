// Mix of `export const` and plain `const` macro declarations.
//
// The expander must handle both forms in the same file. Both the
// exported and non-exported declarations should be erased cleanly.
import { macroRules } from 'macroforge/rules';

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

const $id = macroRules`
  ($x:Expr) => $x
`;

const empty = $vec();
const xs = $vec(1, 2, 3);
const forty = $id(40);
