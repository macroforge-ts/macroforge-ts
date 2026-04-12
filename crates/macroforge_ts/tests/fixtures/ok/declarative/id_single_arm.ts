import { macroRules } from 'macroforge/rules';

const $id = macroRules`
  ($x:Expr) => $x
`;

const a = $id(1 + 2);
const b = $id(someFunction(arg1, arg2));
