import { macroRules } from "macroforge/rules";

const $identity = macroRules`
  ($x:Expr) => $x
`;

/** @derive(Debug) */
class User {
    name: string;
}

const greeting = $identity("hello");
