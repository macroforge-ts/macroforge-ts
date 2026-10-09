/**
 * Runtime behavior of Ord and PartialOrd over array fields and tuple aliases,
 * expanded through the CLI cache like the validator suites.
 */

import { assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'ordering-tests.ts'));

Deno.test('Ord compares array fields lexicographically', () => {
    assertStrictEquals(mod.Scores.compare({ values: [1, 2] }, { values: [1, 3] }), -1);
    assertStrictEquals(mod.Scores.compare({ values: [1, 2] }, { values: [1, 2] }), 0);
    assertStrictEquals(mod.Scores.compare({ values: [1, 2, 0] }, { values: [1, 2] }), 1);
    assertStrictEquals(mod.Scores.partialCompare({ values: [2] }, { values: [1] }), 1);
});

Deno.test('Ord compares tuples element by element', () => {
    assertStrictEquals(mod.Entry.compare(['ada', 1], ['bob', 0]), -1);
    assertStrictEquals(mod.Entry.compare(['ada', 2], ['ada', 1]), 1);
    assertStrictEquals(mod.Entry.compare(['ada', 1, 'x'], ['ada', 1, 'y']), -1);
    assertStrictEquals(mod.Entry.compare(['ada', 1], ['ada', 1]), 0);
    assertStrictEquals(mod.Entry.partialCompare(['ada', 1, 'z'], ['ada', 1]), 1);
});

Deno.test('Ord orders an absent optional tuple element first', () => {
    assertStrictEquals(mod.Span.compare([1], [1, 2]), -1);
    assertStrictEquals(mod.Span.compare([1, 3], [1, 2]), 1);
    assertStrictEquals(mod.Span.partialCompare([1, 2], [1, 2]), 0);
});

Deno.test('Ord compares nested arrays and dates inside tuples', () => {
    const early = new Date('2020-01-01T00:00:00Z');
    const late = new Date('2021-01-01T00:00:00Z');
    assertStrictEquals(mod.Grid.compare([[1, 2], late], [[1, 3], early]), -1);
    assertStrictEquals(mod.Grid.compare([[1, 2], late], [[1, 2], early]), 1);
    assertStrictEquals(mod.Grid.partialCompare([[1], early], [[1], early]), 0);
});
