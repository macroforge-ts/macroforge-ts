/**
 * A derive in a `.mts` module, and one in a `.ts` module holding its type
 * through the `.mjs` name it compiles to.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const kinds = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'module-kinds.mts'));
const held = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'module-held.ts'));
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

Deno.test('a .mts module is expanded', () => {
    assertEquals(kinds.kindedPointDefaultValue(), { x: 0, y: 0 });
    assertStrictEquals(kinds.kindedPointToString({ x: 1, y: 2 }), 'KindedPoint { x: 1, y: 2 }');
    assert(kinds.kindedPointEquals({ x: 1, y: 2 }, { x: 1, y: 2 }));
});

Deno.test('a type held from a .mts module is coded by the functions it generates', () => {
    const value = { point: { x: 3, y: 4 }, label: 'here' };
    assertEquals(held.kindedHolderDefaultValue(), { point: { x: 0, y: 0 }, label: '' });
    const copy = held.kindedHolderClone(value);
    assertEquals(copy, value);
    assertNotStrictEquals(copy.point, value.point);
    assert(held.kindedHolderEquals(value, copy));
    assert(!held.kindedHolderEquals(value, { point: { x: 3, y: 5 }, label: 'here' }));
    assertStrictEquals(held.kindedHolderHashCode(value), held.kindedHolderHashCode(copy));
    assertEquals(
        decoded(held.kindedHolderDecode(JSON.parse(held.kindedHolderEncode(value)))),
        value
    );
});
