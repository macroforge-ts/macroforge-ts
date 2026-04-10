import { macroRules } from "macroforge/rules";

const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

const empty = $vec();
const xs = $vec(1, 2, 3);
const ys = $vec(a, b);
