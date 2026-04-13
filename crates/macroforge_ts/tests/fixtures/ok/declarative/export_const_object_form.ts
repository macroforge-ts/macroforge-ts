// `export const` with the object form of `macroRules({...})`.
//
// Exercises that the `export` + object-form combination is handled
// by the visitor — not just the tag-form template.
import { macroRules } from 'macroforge/rules';

export const $ReadOnly = macroRules({
    kind: 'type',
    expand: macroRules`
    ($t:Type) => { readonly [K in keyof $t]: $t[K] }
  `
});

export const $Tuple = macroRules({
    kind: 'type',
    expand: macroRules`
    ($($t:Type),+) => [$($t),+]
  `
});

interface User {
    id: string;
    name: string;
}

type StrictUser = $ReadOnly<User>;
type Pair = $Tuple<string, number>;
