/**
 * Enums by how their values come about, under every derive, and the fields
 * and collections that hold them.
 */

import { assert, assertEquals, assertStrictEquals, assertThrows } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'enum-shapes.ts'));

const {
    EnumCounted,
    EnumSigned,
    EnumFlags,
    EnumMixed,
    EnumRepeated,
    EnumLone,
    EnumText
} = mod;

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

/** Every value of an enum round-trips, is its own clone, and equals and hashes as itself. */
function assertValueTraits(name, values) {
    for (const value of values) {
        const text = mod[`${name}Encode`](value);
        assertStrictEquals(mod[`${name}Decode`](JSON.parse(text)), value, `${name} ${text}`);
        assert(mod[`${name}Is`](value));
        assertStrictEquals(mod[`${name}Clone`](value), value);
        assert(mod[`${name}Equals`](value, value));
        assertStrictEquals(mod[`${name}Compare`](value, value), 0);
        assertStrictEquals(mod[`${name}PartialCompare`](value, value), 0);
        assertStrictEquals(typeof mod[`${name}HashCode`](value), 'number');
        assertStrictEquals(typeof mod[`${name}ToString`](value), 'string');
    }
}

/** `values`, given in ascending order, compare as that order says, both ways. */
function assertAscending(name, values) {
    values.forEach((lower, index) => {
        for (const higher of values.slice(index + 1)) {
            assertStrictEquals(
                mod[`${name}Compare`](lower, higher),
                -1,
                `${name} ${lower} < ${higher}`
            );
            assertStrictEquals(
                mod[`${name}Compare`](higher, lower),
                1,
                `${name} ${higher} > ${lower}`
            );
            assertStrictEquals(mod[`${name}PartialCompare`](lower, higher), -1);
            assert(!mod[`${name}Equals`](lower, higher));
        }
    });
}

Deno.test('a counted enum codes and orders by its numbers', () => {
    const values = [EnumCounted.First, EnumCounted.Second, EnumCounted.Third];
    assertValueTraits('enumCounted', values);
    assertAscending('enumCounted', values);
    assertStrictEquals(mod.enumCountedEncode(EnumCounted.Second), '1');
    assertStrictEquals(mod.enumCountedDefaultValue(), EnumCounted.First);
    assert(mod.enumCountedToString(EnumCounted.Second).includes('Second'));
    assertThrows(() => mod.enumCountedDecode(3));
    assertThrows(() => mod.enumCountedDecode('First'));
    // The text `encode` writes for a member reads back as it; held in a
    // value, the same text is no member.
    assertStrictEquals(mod.enumCountedDecode('1'), EnumCounted.Second);
    assertStrictEquals(
        mod.enumCountedDecode(mod.enumCountedEncode(EnumCounted.Third)),
        EnumCounted.Third
    );
    assertThrows(() => mod.enumCountedDecodeWithContext('1'));
    assert(!mod.enumCountedIs(3));
});

Deno.test('negative, fractional and computed values are values like any other', () => {
    const signed = [EnumSigned.Below, EnumSigned.Level, EnumSigned.Half, EnumSigned.Above];
    assertValueTraits('enumSigned', signed);
    assertAscending('enumSigned', signed);
    assertStrictEquals(mod.enumSignedDefaultValue(), EnumSigned.Level);
    assertStrictEquals(mod.enumSignedDecode(-1), EnumSigned.Below);
    assertStrictEquals(mod.enumSignedDecode(0.5), EnumSigned.Half);
    assert(mod.enumSignedToString(EnumSigned.Below).includes('Below'));

    const flags = [EnumFlags.None, EnumFlags.Read, EnumFlags.Write, EnumFlags.All];
    assertValueTraits('enumFlags', flags);
    assertAscending('enumFlags', flags);
    assertStrictEquals(mod.enumFlagsDecode(3), EnumFlags.All);
    assertStrictEquals(mod.enumFlagsDefaultValue(), EnumFlags.None);
    assert(mod.enumFlagsToString(EnumFlags.All).includes('All'));
    assertThrows(() => mod.enumFlagsDecode(4));
});

Deno.test('an enum of numbers and strings codes each as it is and orders them all', () => {
    const values = [EnumMixed.Zero, EnumMixed.Text, EnumMixed.Five, EnumMixed.Other];
    assertValueTraits('enumMixed', values);
    assertStrictEquals(mod.enumMixedDecode('text'), EnumMixed.Text);
    assertStrictEquals(mod.enumMixedDecode(5), EnumMixed.Five);
    for (const value of values) {
        assertStrictEquals(mod.enumMixedDecode(mod.enumMixedEncode(value)), value);
    }
    assertStrictEquals(mod.enumMixedDecode('5'), EnumMixed.Five);
    assertThrows(() => mod.enumMixedDecodeWithContext('5'));
    assertThrows(() => mod.enumMixedDecode('6'));
    assertStrictEquals(mod.enumMixedDefaultValue(), EnumMixed.Zero);

    // The order is total: it sorts the values one way, whatever way they come in.
    const sorted = [...values].sort(mod.enumMixedCompare);
    assertEquals([...values].reverse().sort(mod.enumMixedCompare), sorted);
    assertAscending('enumMixed', sorted);
    assert(sorted.indexOf(EnumMixed.Zero) < sorted.indexOf(EnumMixed.Five));
    assert(sorted.indexOf(EnumMixed.Other) < sorted.indexOf(EnumMixed.Text));
});

Deno.test('members that share a value are one value', () => {
    assertValueTraits('enumRepeated', [EnumRepeated.One, EnumRepeated.Uno, EnumRepeated.Two]);
    assert(mod.enumRepeatedEquals(EnumRepeated.One, EnumRepeated.Uno));
    assertStrictEquals(mod.enumRepeatedCompare(EnumRepeated.One, EnumRepeated.Uno), 0);
    assertStrictEquals(
        mod.enumRepeatedHashCode(EnumRepeated.One),
        mod.enumRepeatedHashCode(EnumRepeated.Uno)
    );
    assertStrictEquals(mod.enumRepeatedDecode(1), EnumRepeated.One);
    assertAscending('enumRepeated', [EnumRepeated.One, EnumRepeated.Two]);
});

Deno.test('a lone member and text that needs escaping are coded exactly', () => {
    assertValueTraits('enumLone', [EnumLone.Only]);
    assertStrictEquals(mod.enumLoneDefaultValue(), EnumLone.Only);
    assertThrows(() => mod.enumLoneDecode('other'));

    const text = [EnumText.Empty, EnumText.Slashed, EnumText.Quoted, EnumText.Spaced];
    assertValueTraits('enumText', text);
    assertAscending('enumText', [...text].sort());
    assertStrictEquals(mod.enumTextDecode(''), EnumText.Empty);
    assertStrictEquals(mod.enumTextDecode('it\'s "quoted"'), EnumText.Quoted);
    assertStrictEquals(mod.enumTextDecode('back\\slash'), EnumText.Slashed);
    assertStrictEquals(mod.enumTextDefaultValue(), EnumText.Empty);
    assertThrows(() => mod.enumTextDecode('missing'));
});

const holder = () => ({
    counted: EnumCounted.Third,
    mixed: EnumMixed.Text,
    flags: EnumFlags.All,
    maybe: EnumCounted.Second,
    later: EnumText.Quoted,
    optional: EnumSigned.Below,
    list: [EnumMixed.Zero, EnumMixed.Other, EnumMixed.Five],
    set: new Set([EnumCounted.First, EnumCounted.Third]),
    byName: new Map([['a', EnumText.Spaced]]),
    byKind: new Map([
        [EnumText.Empty, 1],
        [EnumText.Quoted, 2]
    ]),
    labels: { low: EnumSigned.Below, half: EnumSigned.Half },
    pair: [EnumCounted.Second, EnumText.Slashed]
});

Deno.test('enums round-trip in fields, collections, map keys and tuples', () => {
    const encoded = JSON.parse(mod.enumHolderEncode(holder()));
    assertEquals(encoded, {
        counted: 2,
        mixed: 'text',
        flags: 3,
        maybe: 1,
        later: 'it\'s "quoted"',
        optional: -1,
        list: [0, 'other', 5],
        set: [0, 2],
        byName: { a: 'two words' },
        byKind: { '': 1, 'it\'s "quoted"': 2 },
        labels: { low: -1, half: 0.5 },
        pair: [1, 'back\\slash']
    });
    assertEquals(decoded(mod.enumHolderDecode(encoded)), holder());

    const { optional: _optional, ...rest } = holder();
    const sparse = decoded(
        mod.enumHolderDecode(mod.enumHolderEncode({ ...rest, maybe: null, later: undefined }))
    );
    assertStrictEquals(sparse.maybe, null);
    assertStrictEquals(sparse.later, undefined);
    assert(!('optional' in sparse));
});

Deno.test('a value that is no member of its enum is reported where it is held', () => {
    const encoded = JSON.parse(mod.enumHolderEncode(holder()));
    const fieldsOf = (change) => errorFields(mod.enumHolderDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ counted: 9 }), ['counted']);
    assertEquals(fieldsOf({ mixed: '5' }), ['mixed']);
    assertEquals(fieldsOf({ maybe: 9 }), ['maybe']);
    assertEquals(fieldsOf({ later: 'missing' }), ['later']);
    assertEquals(fieldsOf({ optional: 2 }), ['optional']);
    assertEquals(fieldsOf({ list: [0, 'nope'] }), ['list[1]']);
    assertEquals(fieldsOf({ set: [7] }), ['set[0]']);
    assertEquals(fieldsOf({ byName: { a: 'missing' } }), ['byName.a']);
    assertEquals(fieldsOf({ byKind: { missing: 1 } }), ['byKind.missing']);
    assertEquals(fieldsOf({ labels: { low: 9 } }), ['labels.low']);
    assertEquals(fieldsOf({ pair: [9, ''] }), ['pair[0]']);
});

Deno.test('a holder of enums defaults, clones, compares and hashes by them', () => {
    assertEquals(mod.enumHolderDefaultValue(), {
        counted: EnumCounted.First,
        mixed: EnumMixed.Zero,
        flags: EnumFlags.None,
        maybe: null,
        later: undefined,
        list: [],
        set: new Set(),
        byName: new Map(),
        byKind: new Map(),
        labels: {},
        pair: [EnumCounted.First, EnumText.Empty]
    });

    const source = holder();
    const copy = mod.enumHolderClone(source);
    assert(mod.enumHolderEquals(source, copy));
    assertStrictEquals(mod.enumHolderHashCode(source), mod.enumHolderHashCode(copy));
    copy.byKind.set(EnumText.Empty, 9);
    assert(!mod.enumHolderEquals(source, copy));
    assert(mod.enumHolderToString(source).includes('counted'));
});

Deno.test('a type orders by its enum fields in turn', () => {
    const low = { counted: EnumCounted.First, mixed: EnumMixed.Zero, text: EnumText.Spaced };
    const byCounted = { ...low, counted: EnumCounted.Second };
    const byMixed = { ...low, mixed: EnumMixed.Five };
    const byText = { ...low, text: EnumText.Empty };

    assertStrictEquals(mod.enumOrderedCompare(low, byCounted), -1);
    assertStrictEquals(mod.enumOrderedCompare(byMixed, byCounted), -1);
    assertStrictEquals(mod.enumOrderedCompare(low, byMixed), -1);
    assertStrictEquals(mod.enumOrderedCompare(byText, low), -1);
    assertStrictEquals(mod.enumOrderedCompare(low, { ...low }), 0);
    assertStrictEquals(mod.enumOrderedPartialCompare(byCounted, low), 1);
});
