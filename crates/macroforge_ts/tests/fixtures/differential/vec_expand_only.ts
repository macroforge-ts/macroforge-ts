// Differential fixture: $vec used in ExpandOnly mode.
//
// Dev and prod both expand inline. This is the trivial case — the two
// outputs should be byte-identical, but the harness runs them both
// through Deno anyway as a regression guard.

import { macroRules } from 'macroforge/rules';

const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

const empty = $vec();
const nums = $vec(1, 2, 3);
const exprs = $vec(10 + 1, 20 - 5, 7 * 2);

console.log(JSON.stringify({ empty, nums, exprs }));
