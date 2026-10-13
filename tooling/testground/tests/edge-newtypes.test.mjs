/**
 * Newtypes over each primitive under every derive, and the fields and
 * collections that hold one.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'newtype-shapes.ts'));

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

/** `lower` and `higher` are values of the newtype `name`, in ascending order. */
function assertNewtype(name, lower, higher, wrong, defaultValue) {
    for (const value of [lower, higher]) {
        assertStrictEquals(decoded(mod[`${name}Decode`](value)), value, `${name} decodes`);
        assert(mod[`${name}Is`](value));
        assertStrictEquals(mod[`${name}Clone`](value), value);
        assert(mod[`${name}Equals`](value, value));
        assertStrictEquals(mod[`${name}Compare`](value, value), 0);
        assertStrictEquals(typeof mod[`${name}HashCode`](value), 'number');
        assert(mod[`${name}ToString`](value).includes(String(value)));
    }
    assert(!mod[`${name}Equals`](lower, higher));
    assertStrictEquals(mod[`${name}Compare`](lower, higher), -1);
    assertStrictEquals(mod[`${name}Compare`](higher, lower), 1);
    assertStrictEquals(mod[`${name}PartialCompare`](lower, higher), -1);
    assert(mod[`${name}HashCode`](lower) !== mod[`${name}HashCode`](higher));
    assertStrictEquals(mod[`${name}DefaultValue`](), defaultValue);
    assert(!mod[`${name}Decode`](wrong).success, `${name} rejects ${typeof wrong}`);
    assert(!mod[`${name}Is`](wrong));
}

Deno.test('a newtype over each primitive is that primitive under every derive', () => {
    assertNewtype('newText', 'a', 'b', 1, '');
    assertNewtype('newCount', 1, 2, true, 0);
    assertNewtype('newTotal', 1n, 2n, 'no digits', 0n);
    assertNewtype('newFlag', false, true, 1, false);
    assertNewtype('newNested', 1, 2, true, 0);
    assertNewtype('newPlain', 'a', 'b', 1, '');

    assertStrictEquals(mod.newTextEncode('a'), '"a"');
    assertStrictEquals(mod.newCountEncode(1.5), '1.5');
    assertStrictEquals(mod.newFlagEncode(true), 'true');
    assertStrictEquals(mod.newTotalEncode(1n), '"1"');
});

Deno.test("a newtype's validators and default are its own", () => {
    assertNewtype('newSlug', 'alpha', 'beta', 1, 'slug');
    assert(!mod.newSlugDecode('').success);
    assert(!mod.newSlugDecode('UPPER').success);
    assert(!mod.newSlugIs('UPPER'));
    assert(mod.newSlugDecode('UPPER').errors[0].message.includes('lowercase'));
});

const holder = () => ({
    text: 'text',
    count: 1,
    flag: true,
    slug: 'slug-one',
    nested: 2,
    plain: 'plain',
    maybe: 'maybe',
    later: 3,
    optional: 'optional',
    list: ['one', 'two'],
    set: new Set(['a', 'b']),
    bySlug: new Map([['key', 4]]),
    record: { k: 5 },
    pair: ['left', 6]
});

Deno.test('newtypes round-trip in fields, collections, map keys and tuples', () => {
    const encoded = JSON.parse(mod.newHolderEncode(holder()));
    assertEquals(encoded, {
        ...holder(),
        set: ['a', 'b'],
        bySlug: { key: 4 }
    });
    assertEquals(decoded(mod.newHolderDecode(encoded)), holder());

    const { optional: _optional, ...rest } = holder();
    const sparse = decoded(
        mod.newHolderDecode(mod.newHolderEncode({ ...rest, maybe: null, later: undefined }))
    );
    assertStrictEquals(sparse.maybe, null);
    assertStrictEquals(sparse.later, undefined);
    assert(!('optional' in sparse));
});

Deno.test('a value its newtype rejects is reported where it is held', () => {
    const encoded = JSON.parse(mod.newHolderEncode(holder()));
    const fieldsOf = (change) => errorFields(mod.newHolderDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ text: 1 }), ['text']);
    assertEquals(fieldsOf({ count: '1' }), ['count']);
    assertEquals(fieldsOf({ slug: 'UPPER' }), ['slug']);
    assertEquals(fieldsOf({ maybe: 'UPPER' }), ['maybe']);
    assertEquals(fieldsOf({ later: 'three' }), ['later']);
    assertEquals(fieldsOf({ optional: 1 }), ['optional']);
    assertEquals(fieldsOf({ list: ['one', 'UPPER'] }), ['list[1]']);
    assertEquals(fieldsOf({ list: ['one', ''] }), ['list[1]']);
    assertEquals(fieldsOf({ set: ['a', 1] }), ['set[1]']);
    assertEquals(fieldsOf({ bySlug: { UPPER: 4 } }), ['bySlug.UPPER']);
    assertEquals(fieldsOf({ bySlug: { key: 'four' } }), ['bySlug.key']);
    assertEquals(fieldsOf({ record: { k: 'five' } }), ['record.k']);
    assertEquals(fieldsOf({ pair: [1, 6] }), ['pair[0]']);
    assertEquals(fieldsOf({ pair: ['left', 'six'] }), ['pair[1]']);
});

Deno.test('a holder of newtypes defaults, clones, compares and hashes by them', () => {
    assertEquals(mod.newHolderDefaultValue(), {
        text: '',
        count: 0,
        flag: false,
        slug: 'slug',
        nested: 0,
        plain: '',
        maybe: null,
        later: undefined,
        list: [],
        set: new Set(),
        bySlug: new Map(),
        record: {},
        pair: ['', 0]
    });

    const copy = mod.newHolderClone(holder());
    assert(mod.newHolderEquals(holder(), copy));
    assertStrictEquals(mod.newHolderHashCode(holder()), mod.newHolderHashCode(copy));
    copy.bySlug.set('key', 9);
    assert(!mod.newHolderEquals(holder(), copy));
    assert(mod.newHolderToString(holder()).includes('slug-one'));
});

Deno.test('a type orders by its newtype fields in turn, bigints included', () => {
    const low = { slug: 'a', count: 1, total: 1n, flag: false };
    assertStrictEquals(mod.newOrderedCompare(low, { ...low, slug: 'b' }), -1);
    assertStrictEquals(mod.newOrderedCompare(low, { ...low, count: 2 }), -1);
    assertStrictEquals(mod.newOrderedCompare(low, { ...low, total: 2n }), -1);
    assertStrictEquals(mod.newOrderedCompare(low, { ...low, flag: true }), -1);
    assertStrictEquals(mod.newOrderedCompare({ ...low, total: 2n }, low), 1);
    assertStrictEquals(mod.newOrderedCompare(low, { ...low }), 0);
    assert(mod.newOrderedEquals(low, mod.newOrderedClone(low)));
});

Deno.test('bigint newtypes are written as their digits and read back from them', () => {
    const ledger = { total: 10n, totals: [1n, 2n] };
    const encoded = JSON.parse(mod.newLedgerEncode(ledger));
    assertEquals(encoded, { total: '10', totals: ['1', '2'] });
    assertEquals(decoded(mod.newLedgerDecode(encoded)), ledger);
    assertEquals(errorFields(mod.newLedgerDecode({ total: 'ten', totals: [] })), ['total']);
    assertEquals(errorFields(mod.newLedgerDecode({ total: '10', totals: ['1', 'two'] })), [
        'totals[1]'
    ]);
    assert(mod.newLedgerEquals(ledger, mod.newLedgerClone(ledger)));
    assertStrictEquals(mod.newLedgerHashCode(ledger), mod.newLedgerHashCode({ ...ledger }));
});

Deno.test('a newtype over a date, an array, an object or a tuple is coded as what it brands', () => {
    const at = new Date('2024-01-01T00:00:00.000Z');

    assertStrictEquals(mod.newStampEncode(at), '"2024-01-01T00:00:00.000Z"');
    const stamp = decoded(mod.newStampDecode(mod.newStampEncode(at)));
    assert(stamp instanceof Date);
    assertStrictEquals(stamp.getTime(), at.getTime());
    assert(!mod.newStampDecode('never').success);
    assert(!mod.newStampDecode(5).success);
    assert(mod.newStampEquals(at, mod.newStampClone(at)));

    assertEquals(decoded(mod.newTagsDecode(mod.newTagsEncode(['a', 'b']))), ['a', 'b']);
    assertEquals(errorFields(mod.newTagsDecode(['a', 1])), ['_root[1]']);
    assert(!mod.newTagsDecode('a').success);

    const point = { x: 1, at };
    assertEquals(JSON.parse(mod.newPointEncode(point)), { x: 1, at: '2024-01-01T00:00:00.000Z' });
    const read = decoded(mod.newPointDecode(mod.newPointEncode(point)));
    assertEquals(read, point);
    assert(read.at instanceof Date);
    assertEquals(errorFields(mod.newPointDecode({ x: 'one', at: at.toISOString() })), ['x']);
    assertEquals(errorFields(mod.newPointDecode({ x: 1 })), ['at']);
    assert(mod.newPointEquals(point, mod.newPointClone(point)));
    assertStrictEquals(mod.newPointHashCode(point), mod.newPointHashCode(read));

    assert(mod.newStampDefaultValue() instanceof Date);
    assertEquals(mod.newTagsDefaultValue(), []);
    assertEquals(mod.newPointDefaultValue(), { x: 0, at: new Date(0) });

    assertEquals(JSON.parse(mod.newPairEncode([2, at])), [2, '2024-01-01T00:00:00.000Z']);
    const pair = decoded(mod.newPairDecode(mod.newPairEncode([2, at])));
    assert(pair[1] instanceof Date);
    assert(!mod.newPairDecode([2]).success);
    assert(!mod.newPairDecode(['two', at.toISOString()]).success);
});
