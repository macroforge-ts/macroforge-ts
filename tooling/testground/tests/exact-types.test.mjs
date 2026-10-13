/**
 * The code the built-in derives generate type-checks under the compiler
 * options that are stricter than `strict`. Two are about absent values:
 * `exactOptionalPropertyTypes`, under which an optional property may not be
 * assigned `undefined`, and `noUncheckedIndexedAccess`, under which an
 * element read by index may be `undefined`. The others are about how code is
 * written: `noPropertyAccessFromIndexSignature`, `noImplicitOverride`, which
 * a static a subclass derives over its superclass's has to satisfy,
 * `noImplicitReturns`, `allowUnreachableCode` turned off, and
 * `verbatimModuleSyntax`.
 *
 * The check covers the shape and edge-case modules, whose every line of
 * interest is generated.
 */

import { assertEquals } from '@std/assert';
import { runCli, vanillaRoot } from './test-utils.mjs';

Deno.test("exact types: the derives' edge cases type-check", () => {
    const result = runCli(['tsc', '-p', './tsconfig.exact.json'], { cwd: vanillaRoot });
    assertEquals(
        result.success,
        true,
        `macroforge tsc failed under the exact options:\n${result.stdout}\n${result.stderr}`
    );
});
