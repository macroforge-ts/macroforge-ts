/**
 * Values at the edges of their types: numbers that are no ordinary number,
 * dates that are no date, and collections whose order says nothing.
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
