import { macroRules } from "macroforge/rules";

const $double = macroRules`
  ($x:Expr) => ($x * 2)
`;

const $quad = macroRules`
  ($x:Expr) => $double($double($x))
`;

const a = $double(5);
const b = $quad(3);
