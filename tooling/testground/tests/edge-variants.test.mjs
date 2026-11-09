/**
 * Unions whose members are objects, literals and template literal types, in
 * every tagging mode, and a class three levels deep whose fields are typed
 * by their initializers.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'variant-shapes.ts'));
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};
const circle = { radius: 1 };
const rect = { size: [1, 2], label: 'r-x' };

Deno.test('a union of objects, literals and template literal types tells each member apart', () => {
    for (const value of [circle, rect, 'none', 'id-5', 7, null]) {
        const text = mod.variantInternalEncode(value);
        assertEquals(decoded(mod.variantInternalFromString(text)), value, text);
        assert(mod.variantInternalEquals(value, mod.variantInternalClone(value)), text);
        assertStrictEquals(mod.variantInternalPartialCompare(value, value), 0, text);
        assertStrictEquals(
            mod.variantInternalHashCode(value),
            mod.variantInternalHashCode(mod.variantInternalClone(value)),
            text
        );
    }
    for (
        const wrong of [
            'other',
            'id-x',
            8,
            { radius: 'one' },
            { __type: 'VariantRect', size: [1] },
            { size: [1, 2], label: 'x' },
            [],
            true
        ]
    ) {
        assert(!mod.variantInternalDecode(wrong).success, JSON.stringify(wrong));
    }
});

Deno.test('each tagging mode round-trips its variants and refuses a wrong one', () => {
    for (const name of ['variantAdjacent', 'variantExternal', 'variantUntagged']) {
        for (const value of [circle, rect]) {
            const text = mod[`${name}Encode`](value);
            assertEquals(decoded(mod[`${name}FromString`](text)), value, `${name} ${text}`);
        }
        assert(!mod[`${name}Decode`]({ radius: 'one' }).success, name);
        assert(!mod[`${name}Decode`](true).success, name);
    }
    assertEquals(JSON.parse(mod.variantAdjacentEncode(circle)), {
        t: 'VariantCircle',
        c: { radius: 1 }
    });
    assertEquals(JSON.parse(mod.variantExternalEncode(rect)), {
        VariantRect: { size: [1, 2], label: 'r-x' }
    });
    assert(!mod.variantAdjacentDecode({ t: 'VariantCircle', c: { radius: 'x' } }).success);
    assert(!mod.variantAdjacentDecode({ t: 'Nope', c: { radius: 1 } }).success);
    assert(!mod.variantExternalDecode({ VariantRect: { size: [1] } }).success);
    assert(!mod.variantExternalDecode({ Nope: { radius: 1 } }).success);
    for (const value of ['text', 12]) {
        assertEquals(
            decoded(mod.variantUntaggedFromString(mod.variantUntaggedEncode(value))),
            value
        );
    }
    assert(!mod.variantUntaggedDecode(['a']).success);
    assert(!mod.variantUntaggedDecode(true).success);
    assert(!mod.variantUntaggedDecode({ size: [1, 2], label: 'x' }).success);
});

Deno.test('a class three levels deep codes and checks every inherited field', () => {
    const leaf = mod.VariantLeaf.defaultValue();
    assert(leaf instanceof mod.VariantLeaf && leaf instanceof mod.VariantBase);
    leaf.tags.add('a');
    leaf.shape = rect;
    const child = mod.VariantLeaf.defaultValue();
    child.id = 'child';
    child.shape = 'id-3';
    leaf.children.push(child);
    const text = mod.VariantLeaf.encode(leaf);
    const back = decoded(mod.VariantLeaf.fromString(text));
    assert(back instanceof mod.VariantLeaf);
    assert(back.children[0] instanceof mod.VariantLeaf);
    assert(back.created instanceof Date && back.tags instanceof Set);
    assert(mod.VariantLeaf.equals(leaf, back));
    assertStrictEquals(mod.VariantLeaf.hashCode(leaf), mod.VariantLeaf.hashCode(back));
    const encoded = JSON.parse(text);
    for (
        const [change, label] of [
            [{ id: 5 }, 'id'],
            [{ created: 'never' }, 'created'],
            [{ weight: 'heavy' }, 'weight'],
            [{ tags: [1] }, 'tags'],
            [{ note: 5 }, 'note'],
            [{ kind: 'branch' }, 'kind'],
            [{ shape: 'id-x' }, 'shape'],
            [{ children: [{ ...encoded.children[0], id: 5 }] }, 'children']
        ]
    ) {
        assert(!mod.VariantLeaf.decode({ ...encoded, ...change }).success, label);
    }
    const { weight: _weight, ...withoutWeight } = encoded;
    assert(!mod.VariantLeaf.decode(withoutWeight).success, 'missing inherited field');
});

Deno.test('members that derive only Encode are each encoded by their own encoder', () => {
    const born = new Date('2024-01-01T00:00:00.000Z');
    const encoded = (value) => JSON.parse(mod.variantPetEncode(value, true));
    assertEquals(encoded({ kind: 'cat', born }), {
        __id: 0,
        __type: 'VariantCat',
        kind: 'cat',
        born: born.toISOString()
    });
    assertEquals(encoded({ fins: 2, seen: born }), {
        __id: 0,
        __type: 'VariantFish',
        fins: 2,
        seen: born.toISOString()
    });
    assertEquals(encoded({ kind: 'bird', at: born }), { kind: 'bird', at: born.toISOString() });
});
