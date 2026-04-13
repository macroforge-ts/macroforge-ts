// Single `export const $X = macroRules\`...\`` declaration.
//
// Regression fixture for a bug where the expander erased only the
// inner `const $X = macroRules\`...\`;` binding and left the `export`
// keyword orphaned, producing code that failed to parse:
//     "'export' modifier already seen.; Unexpected token"
import { macroRules } from 'macroforge/rules';

export const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

const empty = $vec();
const xs = $vec(1, 2, 3);
