/**
 * Values at the edges of their types: numbers that are no ordinary number,
 * dates that are no date, collections whose order says nothing, and
 * collections that hold `undefined`.
 */

import { assert, assertEquals, assertStrictEquals, assertThrows } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'special-shapes.ts'));

const scalars = (change) => ({
    value: 1,
    at: new Date(0),
    big: 1n,
    text: 't',
    flag: true,
    list: [1],
    ...change
});

Deno.test('a value is equal to its own copy, whatever number or date it holds', () => {
    const odd = [
        { value: NaN },
        { value: -0 },
        { value: Infinity },
        { value: -Infinity },
        { at: new Date('never') },
        { list: [NaN, -0, Infinity] },
        { big: 2n ** 80n }
    ];
    for (const change of odd) {
        const value = scalars(change);
        const copy = mod.specialScalarsClone(value);
        const label = mod.specialScalarsToString(value);
        assert(mod.specialScalarsEquals(value, copy), label);
        assert(mod.specialScalarsEquals(copy, value), label);
        assertStrictEquals(
            mod.specialScalarsHashCode(value),
            mod.specialScalarsHashCode(copy),
            label
        );
        assertStrictEquals(mod.specialScalarsCompare(value, copy), 0, label);
        assertStrictEquals(mod.specialScalarsPartialCompare(value, copy), 0, label);
    }
    assert(mod.specialScalarsEquals(scalars({ value: 0 }), scalars({ value: -0 })));
    assert(!mod.specialScalarsEquals(scalars({ value: NaN }), scalars({ value: 1 })));
    assert(!mod.specialScalarsEquals(scalars({ at: new Date('never') }), scalars({})));
});

Deno.test('Ord orders every pair, with NaN and an invalid date after the rest', () => {
    const numbers = [NaN, 3, -Infinity, 1, Infinity, NaN, -0];
    const sorted = numbers.map((value) => scalars({ value })).sort(mod.specialScalarsCompare);
    assertEquals(
        sorted.map((entry) => entry.value),
        [-Infinity, -0, 1, 3, Infinity, NaN, NaN]
    );
    assert(mod.specialScalarsCompare(scalars({ value: NaN }), scalars({ value: 1 })) > 0);
    assert(mod.specialScalarsCompare(scalars({ value: 1 }), scalars({ value: NaN })) < 0);

    const never = scalars({ at: new Date('never') });
    assert(mod.specialScalarsCompare(never, scalars({})) > 0);
    assert(mod.specialScalarsCompare(scalars({}), never) < 0);

    // PartialOrd leaves NaN and a number unordered.
    assertStrictEquals(
        mod.specialScalarsPartialCompare(scalars({ value: NaN }), scalars({ value: 1 })),
        null
    );
    assertStrictEquals(
        mod.specialScalarsPartialCompare(scalars({ value: 1 }), scalars({ value: NaN })),
        null
    );
    assert(mod.specialScalarsPartialCompare(scalars({ value: 1 }), scalars({ value: 2 })) < 0);
});

Deno.test('encode refuses a number or a date that JSON cannot write', () => {
    for (const value of [NaN, Infinity, -Infinity]) {
        assertThrows(
            () => mod.specialScalarsEncode(scalars({ value })),
            TypeError,
            'JSON cannot represent'
        );
        assertThrows(
            () => mod.specialScalarsEncode(scalars({ value }), true),
            TypeError,
            'JSON cannot represent'
        );
        assertThrows(() => mod.specialScalarsEncode(scalars({ list: [1, value] })), TypeError);
    }
    assertThrows(() => mod.specialScalarsEncode(scalars({ at: new Date('never') })), RangeError);

    const back = mod.specialScalarsDecode(
        mod.specialScalarsEncode(scalars({ value: -0, big: 2n ** 80n }))
    );
    assert(back.success, JSON.stringify(back.errors));
    assert(mod.specialScalarsEquals(back.value, scalars({ value: -0, big: 2n ** 80n })));
});

Deno.test('decode refuses the numbers encode cannot write back', () => {
    const encoded = JSON.parse(mod.specialScalarsEncode(scalars()));
    const fieldsOf = (result) => {
        assert(!result.success, 'expected the decode to fail');
        return [...new Set(result.errors.map((error) => error.field))];
    };

    // Handed in as values.
    for (const odd of [NaN, Infinity, -Infinity]) {
        assertEquals(fieldsOf(mod.specialScalarsDecode({ ...encoded, value: odd })), ['value']);
        assertEquals(fieldsOf(mod.specialScalarsDecode({ ...encoded, list: [1, odd] })), [
            'list[1]'
        ]);
        assert(!mod.specialNumberDecode(odd).success, String(odd));
        assert(!mod.specialNumberIs(odd), String(odd));
        assert(!mod.specialEitherDecode(odd).success, String(odd));
        const carried = { data: null, amounts: { a: odd }, pair: [odd, ''], mixed: odd };
        assertEquals(fieldsOf(mod.specialCarriedDecode(carried)), [
            'amounts.a',
            'pair[0]',
            'mixed'
        ]);
    }

    // Spelled in JSON text, where a literal too large for a number parses to an infinity.
    const text = mod.specialScalarsEncode(scalars());
    for (
        const literal of [
            '1e999',
            '-1e999',
            '1E+400',
            `1${'0'.repeat(400)}`,
            `1${'0'.repeat(250)}e99`
        ]
    ) {
        const spelled = text.replace('"value":1', `"value":${literal}`);
        assert(spelled !== text);
        assertEquals(fieldsOf(mod.specialScalarsDecode(spelled)), ['_root'], literal);
        assert(!mod.specialNumberDecode(literal).success, literal);
        // A field carried as it is holds no such number either.
        const carried = `{"data":{"deep":[${literal}]},"amounts":{},"pair":[1,""],"mixed":""}`;
        assertEquals(fieldsOf(mod.specialCarriedDecode(carried)), ['_root'], literal);
    }

    // Large numbers that fit, and text that only looks like one, are read as they are.
    for (
        const literal of [
            '1e308',
            '1.7976931348623157e308',
            '1e-999',
            '123456789012345678901234567890'
        ]
    ) {
        const result = mod.specialNumberDecode(literal);
        assert(result.success, literal);
        assertStrictEquals(result.value, Number(literal));
    }
    const lookalike = {
        data: '1e999',
        amounts: {},
        pair: [1, `e${'9'.repeat(300)}`],
        mixed: '1e999'
    };
    const read = mod.specialCarriedDecode(mod.specialCarriedEncode(lookalike));
    assert(read.success, JSON.stringify(read.errors));
    assertEquals(read.value, lookalike);
});

Deno.test('a collection is equal and hashes alike whatever order it was filled in', () => {
    const filled = (set, entries) => ({
        set: new Set(set),
        map: new Map(entries),
        record: { a: new Date(0), b: new Date(1) },
        nested: new Map([['k', new Set(['x', 'y'])]]),
        nullable: null
    });
    const one = filled([1, 2, NaN], [['a', [1]], ['b', [2]]]);
    const other = filled([NaN, 2, 1], [['b', [2]], ['a', [1]]]);
    other.record = { b: new Date(1), a: new Date(0) };
    other.nested = new Map([['k', new Set(['y', 'x'])]]);
    assert(mod.specialCollectionsEquals(one, other));
    assertStrictEquals(mod.specialCollectionsHashCode(one), mod.specialCollectionsHashCode(other));
    assert(mod.specialCollectionsEquals(one, mod.specialCollectionsClone(one)));

    assert(!mod.specialCollectionsEquals(one, filled([1, 2], [['a', [1]], ['b', [2]]])));
    assert(!mod.specialCollectionsEquals(one, filled([1, 2, NaN], [['a', [1]], ['b', [3]]])));
    assert(!mod.specialCollectionsEquals(one, { ...other, optional: 1 }));
    assert(!mod.specialCollectionsEquals(one, { ...other, nullable: 0 }));

    const blank = mod.specialCollectionsDefaultValue();
    assertStrictEquals(blank.set.size, 0);
    assertStrictEquals(blank.nullable, null);
    assert(!('optional' in blank));
    const back = mod.specialCollectionsDecode(
        mod.specialCollectionsEncode(filled([1, 2], [['a', [1]]]))
    );
    assert(back.success, JSON.stringify(back.errors));
    assert(mod.specialCollectionsEquals(back.value, filled([1, 2], [['a', [1]]])));
});

Deno.test('an undefined held in a collection comes back as one', () => {
    const holes = (change) => ({
        list: [1],
        pair: [1, 'a'],
        record: { a: 1 },
        finite: { a: new Date(0), b: new Date(1) },
        set: new Set([1]),
        held: new Map([['a', 1]]),
        either: [1],
        plain: 'p',
        grouped: 'g',
        ...change
    });
    const changes = [
        { list: [1, undefined, 3] },
        { pair: [undefined, undefined] },
        { pair: [undefined] },
        { record: { a: undefined, b: 2 } },
        { finite: { a: undefined, b: new Date(1) } },
        { set: new Set([undefined, 2]) },
        { held: new Map([['a', undefined], ['b', 2]]) },
        { plain: undefined },
        { grouped: undefined }
    ];
    for (const change of changes) {
        const value = holes(change);
        const label = mod.specialHolesToString(value);
        const result = mod.specialHolesDecode(mod.specialHolesEncode(value));
        assert(result.success, `${label}: ${JSON.stringify(result.errors)}`);
        assert(mod.specialHolesEquals(value, result.value), label);
        assertStrictEquals(
            mod.specialHolesHashCode(value),
            mod.specialHolesHashCode(result.value),
            label
        );
    }

    // An entry that holds `undefined` is still an entry.
    const kept = mod.specialHolesDecode(
        mod.specialHolesEncode(holes({ held: new Map([['a', undefined]]) }))
    );
    assert(kept.success && kept.value.held.has('a'));
    assertStrictEquals(kept.value.held.get('a'), undefined);

    // A `null` the type takes stays `null`, and takes the place of an
    // `undefined` beside it, which JSON cannot tell from it.
    const either = mod.specialHolesDecode(
        mod.specialHolesEncode(holes({ either: [null, undefined, 2] }))
    );
    assert(either.success);
    assertEquals(either.value.either, [null, null, 2]);

    // A `null` where the type takes none is still refused.
    const refused = mod.specialHolesDecode({
        ...JSON.parse(mod.specialHolesEncode(holes({}))),
        plain: null
    });
    assert(!refused.success);
});
