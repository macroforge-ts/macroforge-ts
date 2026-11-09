/**
 * Collections nested in one another, keyed by something other than a string,
 * and holding unions, tuples and bigints, under every derive.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const edgeCases = join(vanillaRoot, 'src', 'edge-cases');
const mod = await expandAndCompile(join(edgeCases, 'collection-shapes.ts'));
const types = await expandAndCompile(join(edgeCases, 'edge-types.ts'));

const item = (id, score) => ({ id, score });
const at = (iso) => new Date(iso);
const NEW_YEAR = '2024-01-01T00:00:00.000Z';
const MIDSUMMER = '2024-06-21T12:00:00.000Z';

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const nested = () => ({
    grid: [[1, 2], [3], []],
    dateGrid: [[at(NEW_YEAR)], []],
    itemGrid: [[item('a', 1)], [item('b', 2), item('c', 3)]],
    datesByName: new Map([['d', [at(NEW_YEAR), at(MIDSUMMER)]]]),
    itemsByName: { k: [item('a', 1)] },
    setsByName: new Map([['s', new Set(['x', 'y'])]]),
    mapOfMaps: new Map([['m', new Map([['n', 1]])]]),
    cardsByName: new Map([['c', new types.EdgeCard({ title: 't', items: [item('a', 1)] })]]),
    optionalItems: [item('o', 1)],
    maybeItems: [item('m', 2)],
    laterItems: [item('l', 3)]
});

Deno.test('collections nested in collections encode to plain JSON', () => {
    assertEquals(JSON.parse(mod.collectionNestedEncode(nested())), {
        grid: [[1, 2], [3], []],
        dateGrid: [[NEW_YEAR], []],
        itemGrid: [[item('a', 1)], [item('b', 2), item('c', 3)]],
        datesByName: { d: [NEW_YEAR, MIDSUMMER] },
        itemsByName: { k: [item('a', 1)] },
        setsByName: { s: ['x', 'y'] },
        mapOfMaps: { m: { n: 1 } },
        cardsByName: { c: { title: 't', items: [item('a', 1)] } },
        optionalItems: [item('o', 1)],
        maybeItems: [item('m', 2)],
        laterItems: [item('l', 3)]
    });
});

Deno.test('collections nested in collections decode to the collections their types name', () => {
    const value = decoded(mod.collectionNestedFromString(mod.collectionNestedEncode(nested())));
    assertEquals(value, nested());
    assert(value.dateGrid[0][0] instanceof Date);
    assert(value.datesByName.get('d')[1] instanceof Date);
    assert(value.setsByName.get('s') instanceof Set);
    assert(value.mapOfMaps.get('m') instanceof Map);
    assert(value.cardsByName.get('c') instanceof types.EdgeCard);
});

Deno.test('absent, null and undefined collections round-trip', () => {
    const { optionalItems: _optional, ...rest } = nested();
    const sparse = { ...rest, maybeItems: null, laterItems: undefined };
    const encoded = JSON.parse(mod.collectionNestedEncode(sparse));
    assertStrictEquals(encoded.maybeItems, null);
    assert(!('optionalItems' in encoded));
    assert(!('laterItems' in encoded));

    const value = decoded(mod.collectionNestedDecode(encoded));
    assertStrictEquals(value.maybeItems, null);
    assertStrictEquals(value.laterItems, undefined);
    assert(!('optionalItems' in value));
});

Deno.test('a member of a nested collection is checked, and reported by its path', () => {
    const encoded = JSON.parse(mod.collectionNestedEncode(nested()));
    const fieldsOf = (change) => errorFields(mod.collectionNestedDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ grid: [[1, 'two']] }), ['grid[0][1]']);
    assertEquals(fieldsOf({ grid: [1] }), ['grid[0]']);
    assertEquals(fieldsOf({ dateGrid: [[5]] }), ['dateGrid[0][0]']);
    assertEquals(fieldsOf({ itemGrid: [[item('a', 1), { id: 'b' }]] }), ['itemGrid[0][1].score']);
    assertEquals(fieldsOf({ setsByName: { s: ['x', 2] } }), ['setsByName.s[1]']);
    assertEquals(fieldsOf({ mapOfMaps: { m: { n: 'one' } } }), ['mapOfMaps.m.n']);
    assertEquals(fieldsOf({ itemsByName: { k: [{ score: 1 }] } }), ['itemsByName.k[0].id']);
    assertEquals(fieldsOf({ cardsByName: { c: { items: [] } } }), ['cardsByName.c.title']);
    assertEquals(fieldsOf({ maybeItems: [{ id: 1, score: 1 }] }), ['maybeItems[0].id']);
    assertEquals(fieldsOf({ grid: 'not a grid' }), ['grid']);
});

const keys = () => ({
    byNumber: new Map([
        [1, 'one'],
        [-2.5, 'negative']
    ]),
    byKind: new Map([[mod.CollectionKind.Fancy, 2]]),
    byDate: new Map([[at(NEW_YEAR), 'new year']]),
    kindLabels: { plain: 'Plain', fancy: 'Fancy' },
    numbered: { 1: item('a', 1) },
    kinds: new Set([mod.CollectionKind.Plain, mod.CollectionKind.Fancy]),
    dates: new Set([at(NEW_YEAR)]),
    nullableDates: [at(NEW_YEAR), null],
    dateSet: new Set([at(MIDSUMMER), null]),
    dateRecord: { start: at(NEW_YEAR) }
});

Deno.test('a map keyed by a number, an enum or a date decodes with keys of that type', () => {
    const encoded = JSON.parse(mod.collectionKeysEncode(keys()));
    assertEquals(encoded.byNumber, { 1: 'one', '-2.5': 'negative' });
    assertEquals(encoded.byKind, { fancy: 2 });
    assertEquals(encoded.byDate, { [NEW_YEAR]: 'new year' });
    assertEquals(encoded.dates, [NEW_YEAR]);
    assertEquals(encoded.nullableDates, [NEW_YEAR, null]);
    assertEquals(encoded.dateSet, [MIDSUMMER, null]);
    assertEquals(encoded.dateRecord, { start: NEW_YEAR });

    const value = decoded(mod.collectionKeysDecode(encoded));
    assertEquals(value, keys());
    assertStrictEquals(value.byNumber.get(1), 'one');
    assertStrictEquals(value.byNumber.get(-2.5), 'negative');
    assertStrictEquals(value.byKind.get(mod.CollectionKind.Fancy), 2);
    assert([...value.byDate.keys()][0] instanceof Date);
    assert([...value.dates][0] instanceof Date);
    assert(value.dateRecord.start instanceof Date);
    assert(value.nullableDates[0] instanceof Date);
});

Deno.test('a key or member that is not of its type is reported by its path', () => {
    const encoded = JSON.parse(mod.collectionKeysEncode(keys()));
    const fieldsOf = (change) => errorFields(mod.collectionKeysDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ byNumber: { one: 'x' } }), ['byNumber.one']);
    assertEquals(fieldsOf({ byKind: { nope: 1 } }), ['byKind.nope']);
    assertEquals(fieldsOf({ kinds: ['plain', 'nope'] }), ['kinds[1]']);
    assertEquals(fieldsOf({ dates: [NEW_YEAR, 7] }), ['dates[1]']);
    assertEquals(fieldsOf({ numbered: { 1: { id: 'a' } } }), ['numbered.1.score']);
    assertEquals(fieldsOf({ dateRecord: { start: false } }), ['dateRecord.start']);
});

const unions = () => ({
    mixed: ['a', 1],
    itemOrText: [item('a', 1), 'text'],
    itemOrNull: new Set([item('s', 2), null]),
    valueOrNull: new Map([
        ['some', item('v', 3)],
        ['none', null]
    ]),
    recordOrNull: { some: item('r', 4), none: null },
    literal: ['on', 'off'],
    kindOrNull: [mod.CollectionKind.Fancy, null]
});

Deno.test('collections of unions round-trip', () => {
    const value = decoded(mod.collectionUnionsFromString(mod.collectionUnionsEncode(unions())));
    assertEquals(value, unions());
});

Deno.test('a member of a union in a collection is decoded as the member it is', () => {
    const encoded = JSON.parse(mod.collectionUnionsEncode(unions()));
    const fieldsOf = (change) => errorFields(mod.collectionUnionsDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ itemOrText: [{ id: 'a' }] }), ['itemOrText[0].score']);
    assertEquals(fieldsOf({ itemOrNull: [{ score: 1 }] }), ['itemOrNull[0].id']);
    assertEquals(fieldsOf({ valueOrNull: { some: { id: 3, score: 1 } } }), ['valueOrNull.some.id']);
    assertEquals(fieldsOf({ recordOrNull: { some: 'text' } }), ['recordOrNull.some']);
    assertEquals(fieldsOf({ kindOrNull: ['nope'] }), ['kindOrNull[0]']);
});

const tuples = () => ({
    pair: ['a', 1],
    named: ['label', at(NEW_YEAR)],
    itemPair: [item('a', 1), null],
    readonlyPair: [1, 2],
    optionalTail: ['only'],
    rest: ['head', 1, 2, 3],
    pairs: [
        ['a', item('a', 1)],
        ['b', item('b', 2)]
    ],
    nested: [
        [1, 2],
        ['at', at(MIDSUMMER)]
    ]
});

Deno.test('tuples round-trip element by element', () => {
    const encoded = JSON.parse(mod.collectionTuplesEncode(tuples()));
    assertEquals(encoded.named, ['label', NEW_YEAR]);
    assertEquals(encoded.nested, [
        [1, 2],
        ['at', MIDSUMMER]
    ]);

    const value = decoded(mod.collectionTuplesDecode(encoded));
    assertEquals(value, tuples());
    assert(value.named[1] instanceof Date);
    assert(value.nested[1][1] instanceof Date);

    const longer = decoded(
        mod.collectionTuplesDecode({ ...encoded, optionalTail: ['both', 2], rest: ['head'] })
    );
    assertEquals(longer.optionalTail, ['both', 2]);
    assertEquals(longer.rest, ['head']);
});

const lengths = () => ({
    none: [],
    nested: [[]],
    loose: [1, 'two'],
    mixedRest: [item('a', 1), at(NEW_YEAR), at(MIDSUMMER)],
    laterRest: ['head', item('b', 2), at(NEW_YEAR)]
});

Deno.test('a tuple is held to its length when it has no element or none that is checked', () => {
    const encoded = JSON.parse(mod.collectionLengthsEncode(lengths()));
    assertEquals(encoded.mixedRest, [item('a', 1), NEW_YEAR, MIDSUMMER]);
    assertEquals(encoded.laterRest, ['head', item('b', 2), NEW_YEAR]);

    const value = decoded(mod.collectionLengthsDecode(encoded));
    assertEquals(value, lengths());
    assert(value.mixedRest[2] instanceof Date);
    assert(value.laterRest[2] instanceof Date);

    // An optional element is written only where the tuple reaches it.
    const shortened = { ...lengths(), laterRest: ['head'] };
    assertEquals(JSON.parse(mod.collectionLengthsEncode(shortened)).laterRest, ['head']);
    assertEquals(
        decoded(mod.collectionLengthsFromString(mod.collectionLengthsEncode(shortened))),
        shortened
    );
    const skipped = { ...lengths(), laterRest: ['head', undefined, at(NEW_YEAR)] };
    assertEquals(JSON.parse(mod.collectionLengthsEncode(skipped)).laterRest, [
        'head',
        null,
        NEW_YEAR
    ]);
    assertEquals(
        decoded(mod.collectionLengthsFromString(mod.collectionLengthsEncode(skipped))),
        skipped
    );

    const copy = mod.collectionLengthsClone(lengths());
    assertEquals(copy, lengths());
    assertNotStrictEquals(copy.nested[0], lengths().nested[0]);
    assertEquals(mod.collectionLengthsDefaultValue().none, []);
    assertEquals(mod.collectionLengthsDefaultValue().nested, [[]]);

    const fieldsOf = (change) =>
        errorFields(mod.collectionLengthsDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ none: [1] }), ['none']);
    assertEquals(fieldsOf({ nested: [[1]] }), ['nested[0]']);
    assertEquals(fieldsOf({ nested: [] }), ['nested']);
    assertEquals(fieldsOf({ loose: [1] }), ['loose']);
    assertEquals(fieldsOf({ loose: [1, 2, 3] }), ['loose']);
    assertEquals(fieldsOf({ mixedRest: [item('a', 1), 5] }), ['mixedRest[1]']);
});

Deno.test('a tuple of the wrong length or with a wrong element fails to decode', () => {
    const encoded = JSON.parse(mod.collectionTuplesEncode(tuples()));
    const fieldsOf = (change) => errorFields(mod.collectionTuplesDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ pair: ['a'] }), ['pair']);
    assertEquals(fieldsOf({ pair: ['a', 1, 2] }), ['pair']);
    assertEquals(fieldsOf({ pair: [1, 1] }), ['pair[0]']);
    assertEquals(fieldsOf({ named: ['label', 5] }), ['named[1]']);
    assertEquals(fieldsOf({ itemPair: [item('a', 1), { id: 'b' }] }), ['itemPair[1].score']);
    assertEquals(fieldsOf({ rest: ['head', 'tail'] }), ['rest[1]']);
    assertEquals(fieldsOf({ pairs: [['a', { score: 1 }]] }), ['pairs[0][1].id']);
    assertEquals(fieldsOf({ optionalTail: [] }), ['optionalTail']);
});

Deno.test('a bigint is written as its digits, wherever it is held', () => {
    const big = {
        total: 10n,
        totals: [1n, -2n],
        maybeTotal: 2n ** 70n,
        byName: new Map([['a', 3n]])
    };
    const encoded = JSON.parse(mod.collectionBigEncode(big));
    assertEquals(encoded, {
        total: '10',
        totals: ['1', '-2'],
        maybeTotal: '1180591620717411303424',
        byName: { a: '3' }
    });
    assertEquals(decoded(mod.collectionBigDecode(encoded)), big);
    assertEquals(
        decoded(mod.collectionBigDecode({ ...encoded, maybeTotal: null })).maybeTotal,
        null
    );

    const fieldsOf = (change) => errorFields(mod.collectionBigDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ total: 'ten' }), ['total']);
    assertEquals(fieldsOf({ totals: ['1', 'two'] }), ['totals[1]']);
    assertEquals(fieldsOf({ byName: { a: 'three' } }), ['byName.a']);
    assertEquals(fieldsOf({ maybeTotal: 'many' }), ['maybeTotal']);
});

Deno.test('the value traits reach into nested collections', () => {
    for (
        const [name, make] of [
            ['collectionNested', nested],
            ['collectionKeys', keys],
            ['collectionUnions', unions],
            ['collectionTuples', tuples]
        ]
    ) {
        const source = make();
        const copy = mod[`${name}Clone`](source);
        assert(mod[`${name}Equals`](source, copy), `${name}: a clone equals its source`);
        assertStrictEquals(mod[`${name}HashCode`](source), mod[`${name}HashCode`](copy), name);
        assert(mod[`${name}ToString`](source).length > 0, name);
    }

    const source = nested();
    const copy = mod.collectionNestedClone(source);
    assertNotStrictEquals(copy.grid[0], source.grid[0]);
    assertNotStrictEquals(copy.dateGrid[0][0], source.dateGrid[0][0]);
    assertNotStrictEquals(copy.setsByName.get('s'), source.setsByName.get('s'));
    assertNotStrictEquals(copy.cardsByName.get('c'), source.cardsByName.get('c'));

    copy.mapOfMaps.get('m').set('n', 2);
    assert(!mod.collectionNestedEquals(source, copy));
    copy.mapOfMaps.get('m').set('n', 1);
    copy.dateGrid[0][0].setFullYear(2000);
    assert(!mod.collectionNestedEquals(source, copy));
});

Deno.test('defaults are empty collections and tuples of defaults', () => {
    assertEquals(mod.collectionNestedDefaultValue(), {
        grid: [],
        dateGrid: [],
        itemGrid: [],
        datesByName: new Map(),
        itemsByName: {},
        setsByName: new Map(),
        mapOfMaps: new Map(),
        cardsByName: new Map(),
        maybeItems: null,
        laterItems: undefined
    });
    assertEquals(mod.collectionKeysDefaultValue().kindLabels, { plain: '', fancy: '' });
    assertEquals(mod.collectionUnionsDefaultValue().itemOrNull, new Set());

    const tuple = mod.collectionTuplesDefaultValue();
    assertEquals(tuple.pair, ['', 0]);
    assertEquals(tuple.itemPair, [item('', 0), null]);
    assertEquals(tuple.readonlyPair, [0, 0]);
    assertEquals(tuple.optionalTail, ['']);
    assertEquals(tuple.rest, ['']);
    assertEquals(tuple.pairs, []);
    assertEquals(tuple.nested[0], [0, 0]);
    assertStrictEquals(tuple.nested[1][0], '');
    assert(tuple.named[1] instanceof Date);
});
