import { macroRules } from '@macroforge/core/rules';

const $withTemp = macroRules`
  ($x:Expr) => {
    const __v = $x;
    __v + 1
  }
`;

const result = $withTemp(10);
