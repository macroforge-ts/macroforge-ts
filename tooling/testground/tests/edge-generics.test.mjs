/**
 * Generic types: parameters in every position a field can hold one, and
 * fields that instantiate another generic type.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'generic-shapes.ts'));

const item = (id, score) => ({ id, score });

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const box = (held, other) => ({
    held,
    list: [held, other],
    maybe: other,
    optional: held,
    byName: new Map([['k', other]]),
    set: new Set([held]),
    record: { k: held },
    pair: [held, other]
});

Deno.test('a value of a type parameter is carried as it is, wherever it is held', () => {
    for (const [held, other] of [['text', 'other'], [1, 2], [true, false]]) {
        const source = box(held, other);
        const value = decoded(mod.genericBoxDecode(mod.genericBoxEncode(source)));
        assertEquals(value, source);
        assert(value.byName instanceof Map);
        assert(value.set instanceof Set);

        const copy = mod.genericBoxClone(source);
        assertEquals(copy, source);
        assertNotStrictEquals(copy.list, source.list);
        assert(mod.genericBoxEquals(source, copy));
        assertStrictEquals(mod.genericBoxHashCode(source), mod.genericBoxHashCode(copy));
        assert(mod.genericBoxToString(source).includes('held'));
    }

    const { optional: _optional, ...rest } = box('text', 'other');
    const sparse = decoded(mod.genericBoxDecode(mod.genericBoxEncode({ ...rest, maybe: null })));
    assertStrictEquals(sparse.maybe, null);
    assert(!('optional' in sparse));
    const encoded = JSON.parse(mod.genericBoxEncode(rest));
    assertEquals(errorFields(mod.genericBoxDecode({ ...encoded, list: 'none' })), ['list']);
});

Deno.test('several parameters, defaults and constraints each keep their own fields', () => {
    const pair = { left: 'l', right: 2 };
    assertEquals(decoded(mod.genericPairDecode(mod.genericPairEncode(pair))), pair);
    assert(mod.genericPairEquals(pair, mod.genericPairClone(pair)));
    assert(!mod.genericPairEquals(pair, { left: 'l', right: 3 }));
    assertEquals(errorFields(mod.genericPairDecode({ left: 'l' })), ['right']);

    const constrained = { item: item('a', 1), name: 'n', items: [item('b', 2)] };
    assertEquals(
        decoded(mod.genericConstrainedDecode(mod.genericConstrainedEncode(constrained))),
        constrained
    );
    assert(mod.genericConstrainedEquals(constrained, mod.genericConstrainedClone(constrained)));
});

Deno.test('a generic class is built, coded and compared as an instance', () => {
    const blank = mod.GenericStack.defaultValue();
    assert(blank instanceof mod.GenericStack);
    assertEquals({ ...blank }, { items: [], top: null, depth: 0 });

    const stack = decoded(mod.GenericStack.decode({ items: [1, 2], top: 2, depth: 2 }));
    assert(stack instanceof mod.GenericStack);
    assertEquals(JSON.parse(mod.GenericStack.encode(stack)), { items: [1, 2], top: 2, depth: 2 });

    const copy = mod.GenericStack.clone(stack);
    assert(copy instanceof mod.GenericStack);
    assertNotStrictEquals(copy.items, stack.items);
    assert(mod.GenericStack.equals(stack, copy));
    assertStrictEquals(mod.GenericStack.hashCode(stack), mod.GenericStack.hashCode(copy));
    copy.items.push(3);
    assert(!mod.GenericStack.equals(stack, copy));
});

Deno.test('generic unions, lists and nullable aliases are coded by their bodies', () => {
    const ok = { ok: true, value: 1 };
    const failed = { ok: false, failure: 'reason' };
    assertEquals(decoded(mod.genericResultDecode(mod.genericResultEncode(ok))), ok);
    assertEquals(decoded(mod.genericResultDecode(mod.genericResultEncode(failed))), failed);
    assert(mod.genericResultEquals(ok, mod.genericResultClone(ok)));
    assert(!mod.genericResultEquals(ok, failed));

    assertEquals(decoded(mod.genericListDecode(mod.genericListEncode([1, 2]))), [1, 2]);
    assertEquals(decoded(mod.genericMaybeDecode(mod.genericMaybeEncode('text'))), 'text');
    assertStrictEquals(decoded(mod.genericMaybeDecode(mod.genericMaybeEncode(null))), null);
});

const uses = () => ({
    text: box('text', 'other'),
    item: box(item('a', 1), item('b', 2)),
    nested: box(box(1, 2), box(3, 4)),
    pair: { left: 'l', right: item('r', 1) },
    same: { left: 1, right: 2 },
    stack: mod.GenericStack.decode({ items: [item('s', 1)], top: null, depth: 1 }).value,
    stacks: [mod.GenericStack.decode({ items: ['x'], top: 'x', depth: 1 }).value],
    list: [item('l', 1)],
    maybe: item('m', 1),
    boxes: new Map()
});

Deno.test('a field that instantiates a generic type is coded through that type', () => {
    // `nested` holds boxes as another box's `T`: see the test below.
    const { nested: _nested, ...value } = decoded(
        mod.genericUsesDecode(mod.genericUsesEncode(uses()))
    );
    const { nested: _expectedNested, ...expected } = uses();
    assertEquals(value, expected);
    assert(value.stack instanceof mod.GenericStack);
    assert(value.stacks[0] instanceof mod.GenericStack);
    assert(value.item.byName instanceof Map);
    assert(value.item.set instanceof Set);

    const copy = mod.genericUsesClone(uses());
    assert(mod.genericUsesEquals(uses(), copy));
    assertStrictEquals(mod.genericUsesHashCode(uses()), mod.genericUsesHashCode(copy));
    assert(copy.stack instanceof mod.GenericStack);

    const encoded = JSON.parse(mod.genericUsesEncode(uses()));
    const fieldsOf = (change) => errorFields(mod.genericUsesDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ list: [{ id: 'l' }] }), ['list[0].score']);
    assertEquals(fieldsOf({ maybe: { score: 1 } }), ['maybe.id']);
    assertEquals(fieldsOf({ stack: { items: [], top: null } }), ['stack.depth']);
    assertEquals(fieldsOf({ stacks: [{ top: null, depth: 0 }] }), ['stacks[0].items']);
    assertEquals(fieldsOf({ pair: { left: 'l' } }), ['pair.right']);
});

Deno.test('a value of a type parameter is not decoded as the type it was given', () => {
    // Nothing tells `GenericBox` how its `T` is coded, so a date held as one
    // is written as JSON writes it and read back as that text.
    const held = new Date('2024-01-01T00:00:00.000Z');
    const encoded = JSON.parse(
        mod.genericUsesEncode({ ...uses(), boxes: new Map([['b', box(held, held)]]) })
    );
    assertStrictEquals(encoded.boxes.b.held, '2024-01-01T00:00:00.000Z');
    const value = decoded(mod.genericUsesDecode(encoded));
    assertStrictEquals(value.boxes.get('b').held, '2024-01-01T00:00:00.000Z');

    // The same holds for a box held as another box's `T`: the outer box is
    // coded, and the inner one is carried as JSON carries it.
    assert(value.nested.byName instanceof Map);
    assert(!(value.nested.held.byName instanceof Map));
});
