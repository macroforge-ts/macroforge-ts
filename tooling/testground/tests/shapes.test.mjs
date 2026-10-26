/**
 * Runtime behavior of the derives over the shapes in the vanilla testground's
 * `shape-tests.ts`, expanded through the CLI cache like the validator suites.
 * The same file is type-checked with the rest of the project, against
 * TypeScript's own library and its replacement.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { DecodeContext, PendingRef } from '@macroforge/core/endec';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'shape-tests.ts'));

const item = (id, score) => ({ id, score });

const fields = () => ({
    note: undefined,
    maybe: null,
    pair: ['a', 1],
    nullableTexts: ['x', null],
    nullableItems: [item('a', 1), null],
    parenthesizedItems: [null, item('b', 2)],
    readonlyItems: [item('c', 3)],
    readonlyNumbers: [1, 2],
    readonlyTags: new Set(['t']),
    readonlyCounts: new Map([['k', 1]]),
    level: mod.ShapeLevel.High,
    rank: mod.ShapeRank.Two
});

Deno.test('null elements and readonly collections round-trip', () => {
    const encoded = mod.shapeFieldsEncode(fields());
    assertEquals(JSON.parse(encoded), {
        maybe: null,
        pair: ['a', 1],
        nullableTexts: ['x', null],
        nullableItems: [{ id: 'a', score: 1 }, null],
        parenthesizedItems: [null, { id: 'b', score: 2 }],
        readonlyItems: [{ id: 'c', score: 3 }],
        readonlyNumbers: [1, 2],
        readonlyTags: ['t'],
        readonlyCounts: { k: 1 },
        level: 'high',
        rank: 2
    });

    const decoded = mod.shapeFieldsDecode(encoded);
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertEquals(decoded.value, fields());
    assert(decoded.value.readonlyTags instanceof Set);
    assert(decoded.value.readonlyCounts instanceof Map);
});

Deno.test('a null element is rendered, compared, hashed and cloned as null', () => {
    const value = fields();
    assert(mod.shapeFieldsToString(value).includes('nullableItems: ['));
    const copy = mod.shapeFieldsClone(value);
    assert(mod.shapeFieldsEquals(value, copy));
    assertStrictEquals(mod.shapeFieldsHashCode(value), mod.shapeFieldsHashCode(copy));
    assertStrictEquals(copy.nullableItems[1], null);
});

Deno.test('defaults follow the type: its absent value, an empty collection, a tuple of defaults', () => {
    assertEquals(mod.shapeFieldsDefaultValue(), {
        note: undefined,
        maybe: null,
        pair: ['', 0],
        nullableTexts: [],
        nullableItems: [],
        parenthesizedItems: [],
        readonlyItems: [],
        readonlyNumbers: [],
        readonlyTags: new Set(),
        readonlyCounts: new Map(),
        level: mod.ShapeLevel.Low,
        rank: mod.ShapeRank.One
    });
});

Deno.test('numeric and string enums order by their values', () => {
    assertStrictEquals(mod.shapeRankCompare(mod.ShapeRank.One, mod.ShapeRank.Two), -1);
    assertStrictEquals(mod.shapeRankCompare(mod.ShapeRank.Two, mod.ShapeRank.One), 1);
    assertStrictEquals(mod.shapeRankPartialCompare(mod.ShapeRank.Two, mod.ShapeRank.Two), 0);
    assertStrictEquals(mod.shapeLevelCompare(mod.ShapeLevel.High, mod.ShapeLevel.Low), -1);
    assertStrictEquals(mod.shapeLevelCompare(mod.ShapeLevel.Low, mod.ShapeLevel.High), 1);
    assertStrictEquals(mod.shapeLevelPartialCompare(mod.ShapeLevel.Low, mod.ShapeLevel.Low), 0);
});

Deno.test('a union of an interface and a class decodes each by its shape', () => {
    assert(mod.shapeCardHasShape({ title: 't', items: [] }));
    assert(!mod.shapeCardHasShape({ id: 'a', score: 1 }));

    const card = mod.shapeMemberDecode({ title: 't', items: [item('a', 1)] });
    assert(card.success, JSON.stringify(card.errors));
    assert(card.value instanceof mod.ShapeCard);

    const plain = mod.shapeMemberDecode(item('a', 1));
    assert(plain.success, JSON.stringify(plain.errors));
    assertEquals(plain.value, item('a', 1));
});

Deno.test('a readonly class field is decoded and defaulted like any other', () => {
    const decoded = mod.ShapeCard.decode({ title: 'kept', items: [] });
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertStrictEquals(decoded.value.title, 'kept');

    const blank = mod.ShapeCard.defaultValue();
    assert(blank instanceof mod.ShapeCard);
    assertStrictEquals(blank.title, '');
    assertEquals(blank.items, []);
});

Deno.test('null is a member of a mixed union', () => {
    assert(mod.shapeMixedHasShape(null));
    const nothing = mod.shapeMixedDecode(null);
    assert(nothing.success, JSON.stringify(nothing.errors));
    assertStrictEquals(nothing.value, null);

    const count = mod.shapeMixedDecode(3);
    assert(count.success, JSON.stringify(count.errors));
    assertStrictEquals(count.value, 3);

    assertStrictEquals(mod.shapeMixedEncode(null), 'null');
    assert(!mod.shapeMixedDecode(true).success);
});

const draft = () => ({
    patch: { id: 'p' },
    header: { id: 'h', at: new Date('2024-01-02T03:04:05.000Z') },
    rest: { id: 'r', tags: ['x'] },
    card: { title: 'kept' },
    frozen: item('f', 1)
});

Deno.test('a Partial, Pick or Omit encodes the fields it has', () => {
    assertEquals(JSON.parse(mod.shapeDraftEncode(draft())), {
        patch: { id: 'p' },
        header: { id: 'h', at: '2024-01-02T03:04:05.000Z' },
        rest: { id: 'r', tags: ['x'] },
        card: { title: 'kept' },
        frozen: { id: 'f', score: 1 }
    });
});

Deno.test('a Partial, Pick or Omit decodes with the fields its type keeps', () => {
    const decoded = mod.shapeDraftDecode(mod.shapeDraftEncode(draft()));
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertEquals(decoded.value.patch, { id: 'p' });
    assert(decoded.value.header.at instanceof Date);
    assertEquals(decoded.value.rest, { id: 'r', tags: ['x'] });
    assert(decoded.value.card instanceof mod.ShapeCard);
    assertEquals(Object.keys(decoded.value.card), ['title']);
    assertEquals(decoded.value.frozen, item('f', 1));
});

Deno.test('a Pick or Omit still requires the fields it keeps, and a Readonly all of them', () => {
    const encoded = JSON.parse(mod.shapeDraftEncode(draft()));
    // A missing primitive is reported as absent and as the wrong type.
    const errorsOf = (change) => {
        const result = mod.shapeDraftDecode({ ...encoded, ...change });
        assert(!result.success);
        return [...new Set(result.errors.map((error) => error.field))];
    };
    assertEquals(errorsOf({ header: { id: 'h' } }), ['header.at']);
    assertEquals(errorsOf({ rest: { tags: [] } }), ['rest.id']);
    assertEquals(errorsOf({ frozen: { id: 'f' } }), ['frozen.score']);
    assertEquals(errorsOf({ patch: { id: 3 } }), ['patch.id']);
});

Deno.test('a Pick or Omit is coded without the fields its type leaves out', () => {
    const whole = { id: 'w', at: new Date('2024-01-02T03:04:05.000Z'), tags: ['x'] };

    // A whole event is assignable to either, so its other fields are there to leak.
    const encoded = JSON.parse(mod.shapeDraftEncode({ ...draft(), header: whole, rest: whole }));
    assertEquals(encoded.header, { id: 'w', at: '2024-01-02T03:04:05.000Z' });
    assertEquals(encoded.rest, { id: 'w', tags: ['x'] });

    const wire = { id: 'w', at: '2024-01-02T03:04:05.000Z', tags: ['x'], owner: item('o', 1) };
    const decoded = mod.shapeDraftDecode({ ...encoded, header: wire, rest: wire });
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertEquals(Object.keys(decoded.value.header), ['id', 'at']);
    assertEquals(Object.keys(decoded.value.rest), ['id', 'tags', 'owner']);
});

Deno.test('a value decoded by its fields cannot be referred to, or be a reference', () => {
    const encoded = JSON.parse(mod.shapeDraftEncode(draft()));

    // A whole card standing in for the partial one would carry every field.
    const asReference = mod.shapeDraftDecode({ ...encoded, card: { __ref: 1 } });
    assert(!asReference.success);
    assert(asReference.errors[0].message.includes('cannot be a reference'));

    // The partial card lacks `items`, so nothing whole may point at it.
    const ctx = DecodeContext.create();
    const partial = mod.shapeCardDecodePartialWithContext({ __id: 7, title: 'kept' }, ctx);
    assertEquals(Object.keys(partial), ['title']);
    assert(PendingRef.is(ctx.getOrDefer(7)), 'the partial card is not registered');

    const whole = mod.shapeCardDecodeWithContext({ __id: 8, title: 'whole', items: [] }, ctx);
    assertStrictEquals(ctx.getOrDefer(8), whole);
});
