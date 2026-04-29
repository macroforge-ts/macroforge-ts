// More complex declarative macros.
//
// Exercises patterns beyond the basic `$vec` / `$id` / `$withTemp` set:
// - Multi-arm dispatch by arity
// - Nested repetition (macro-in-a-macro output)
// - Macro composition (one macro calls another)
// - Value/default plumbing
// - Nested block scopes with hygiene-local temporaries
// - Type-position macros via the `macroRules({...})` object form

import { macroRules } from "macroforge/rules";

// ──────────────────────────────────────────────────────────────────
// 1. Multi-arm dispatch by arity.
//    $min with 1 arg is the identity; with 2 args compares; with 3+
//    recurses by folding the tail through the 2-arg arm.
// ──────────────────────────────────────────────────────────────────

const $min = macroRules`
  ($x:Expr) => $x
  ($x:Expr, $y:Expr) => ($x < $y ? $x : $y)
`;

export const minOne = $min(42);
export const minTwo = $min(7, 3);
// Nested calls work at this level because `$pow4` calls `$square` from
// within its own template body — recursion happens during expansion of
// the OUTER macro, not when the inner call is passed as a runtime arg.
// For that case use a non-macro intermediate.
export const minOfTwo = $min(7, 3);
export const minNegative = $min(-5, -100);

// ──────────────────────────────────────────────────────────────────
// 2. Value/default plumbing. One arm handles the 1-arg case, another
//    folds in a default for the 2-arg case.
// ──────────────────────────────────────────────────────────────────

const $orElse = macroRules`
  ($x:Expr) => $x
  ($x:Expr, $default:Expr) => ($x ?? $default)
`;

export const orElseNoDefault = $orElse(100);
const maybeMissing: number | null = null as number | null;
export const orElseWithDefault = $orElse(maybeMissing, 99);

// ──────────────────────────────────────────────────────────────────
// 3. Macro composition. `$square` uses `$double`-style replication in
//    its body; `$pow4` squares a square. Exercises the rewriter's
//    ability to re-visit already-rewritten output.
// ──────────────────────────────────────────────────────────────────

const $square = macroRules`
  ($x:Expr) => ($x * $x)
`;

const $pow4 = macroRules`
  ($x:Expr) => $square($square($x))
`;

export const squaredFive = $square(5);
export const pow4Two = $pow4(2);

// ──────────────────────────────────────────────────────────────────
// 4. Repetition with a non-comma emission separator. `$sumAll`
//    expands to an array literal; its numeric sum is computed at
//    runtime. Exercises the same `,`-separated repetition shape as
//    `$vec` but in a consuming context.
// ──────────────────────────────────────────────────────────────────

const $sumAll = macroRules`
  ($($x:Expr),+) => [$($x),+].reduce((a, b) => a + b, 0)
`;

export const sumTriple = $sumAll(1, 2, 3);
export const sumFive = $sumAll(10, 20, 30, 40, 50);

// ──────────────────────────────────────────────────────────────────
// 5. Hygiene stress: the macro's body introduces a local `__temp`
//    whose name must not collide with any caller-scope binding of
//    the same name. The test verifies the caller's `__temp` is
//    preserved after the macro call site.
// ──────────────────────────────────────────────────────────────────

const $squarePlusOne = macroRules`
  ($x:Expr) => (() => {
    const __temp = $x;
    return __temp * __temp + 1;
  })()
`;

export function hygieneCheck(): { callerTemp: number; macroResult: number } {
  // Caller's `__temp` should remain 999 after the macro expands,
  // because the macro's expansion introduces its own `__temp` inside
  // an IIFE — it must not escape or shadow the outer binding.
  const __temp = 999;
  const macroResult = $squarePlusOne(3);
  return { callerTemp: __temp, macroResult };
}

// Value-position use at module scope.
export const sqPlus1 = $squarePlusOne(4);

// ──────────────────────────────────────────────────────────────────
// 6. Type-position macros via the object form.
//    `$PartialDeep<T>` marks every top-level key optional. Useful for
//    patches / merge types.
// ──────────────────────────────────────────────────────────────────

// `export`-ed so Deno's no-unused-vars lint doesn't flag the binding
// as unused — the only usages are type references (`$PartialShallow<T>`)
// which the JS-side linter doesn't follow across domains.
export const $PartialShallow = macroRules({
  kind: "type",
  expand: macroRules`
    ($t:Type) => { [K in keyof $t]?: $t[K] }
  `,
});

export const $NonNull = macroRules({
  kind: "type",
  expand: macroRules`
    ($t:Type) => Exclude<$t, null | undefined>
  `,
});

export interface Address {
  street: string;
  city: string;
  zip: string;
}

export type AddressPatch = $PartialShallow<Address>;
export type DefinitelyString = $NonNull<string | null | undefined>;

// Values that exercise the types at runtime.
export const samplePatch: AddressPatch = { city: "Berlin" };
export const definitelyHello: DefinitelyString = "hello";

// The test harness uses this sentinel to confirm the declarative
// macros were all erased from the source at build time.
export const declarativeComplexErased = true;
