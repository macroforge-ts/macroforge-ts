import { macroRules } from '@macroforge/core/rules';

const $identity = macroRules`
  ($x:Expr) => $x
`;

/** @derive(Debug) */
class User {
    name: string;
}

const greeting = $identity('hello');
