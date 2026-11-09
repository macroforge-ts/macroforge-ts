/**
 * Each built-in derive on the kinds of declaration no other edge case derives
 * it on.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'kind-matrix.ts'));

Deno.test('an enum defaults to the member marked as its default', () => {
    assertStrictEquals(mod.kindShadeDefaultValue(), 'light');
});

Deno.test('an object alias is compared and ordered field by field', () => {
    assert(mod.kindPointEquals({ x: 1, y: 2 }, { x: 1, y: 2 }));
    assertStrictEquals(mod.kindPointCompare({ x: 1, y: 2 }, { x: 1, y: 3 }), -1);
    assertStrictEquals(mod.kindPointPartialCompare({ x: 2, y: 0 }, { x: 1, y: 9 }), 1);
});

Deno.test('an intersection alias is cloned, compared and ordered over the fields of both members', () => {
    const person = { name: 'ada', age: 36 };
    const copy = mod.kindPersonClone(person);
    assertEquals(copy, person);
    assertNotStrictEquals(copy, person);
    assert(mod.kindPersonEquals(person, copy));
    assert(!mod.kindPersonEquals(person, { name: 'ada', age: 37 }));
    assertStrictEquals(mod.kindPersonCompare(person, { name: 'bob', age: 1 }), -1);
    assertStrictEquals(mod.kindPersonHashCode(person), mod.kindPersonHashCode(copy));
});
