import { macroRules } from 'macroforge/rules';

// Type-position macro: wrap a type in a readonly mapped type.
const $ReadOnly = macroRules({
    kind: 'type',
    expand: macroRules`
    ($t:Type) => { readonly [K in keyof $t]: $t[K] }
  `
});

// Type-position macro with repetition: build a tuple of types.
const $Tuple = macroRules({
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
type Triple = $Tuple<string, number, boolean>;
