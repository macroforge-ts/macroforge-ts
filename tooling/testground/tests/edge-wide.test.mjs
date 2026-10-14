/**
 * Types wider than the checker reads one body of: an interface of 300 fields
 * and unions of 120 members, coded, compared and hashed like any other.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'wide-shapes.ts'));

const NEW_YEAR = '2024-01-01T00:00:00.000Z';
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

// The value of field `f{index}` for each kind of field, as `wide-shapes.ts` cycles through them.
const kinds = [
    (index) => index,
    (index) => `s${index}`,
    (index) => index % 2 === 0,
    (encoded) => (encoded ? NEW_YEAR : new Date(NEW_YEAR)),
    (index) => [`a${index}`],
    (index) => (index % 3 === 0 ? null : index),
    (index) => `u${index}`
];
const record = (encoded) =>
    Object.fromEntries(
        Array.from({ length: 300 }, (_, index) => {
            const kind = index % kinds.length;
            return [`f${index}`, kinds[kind](kind === 3 ? encoded : index)];
        })
    );

Deno.test('a record of 300 fields round-trips, compares and hashes', () => {
    const value = record(false);
    const json = JSON.parse(mod.wideRecordEncode(value));
    assertEquals(json, record(true));
    assertEquals(decoded(mod.wideRecordDecode(json)), value);

    const copy = mod.wideRecordClone(value);
    assertEquals(copy, value);
    assert(mod.wideRecordEquals(value, copy));
    assertStrictEquals(mod.wideRecordHashCode(value), mod.wideRecordHashCode(copy));
    assert(!mod.wideRecordEquals(value, { ...copy, f299: -1 }));

    const { f0: _first, f297: _late, ...missing } = json;
    const result = mod.wideRecordDecode(missing);
    assert(!result.success);
    assertEquals(result.errors.map((error) => error.field).sort(), ['f0', 'f297']);
    assertStrictEquals(mod.wideRecordDefaultValue().f1, '');
});

Deno.test('a union of 120 members tells each by its tag', () => {
    for (const index of [0, 77, 119]) {
        const value = { kind: `k${index}`, at: new Date(NEW_YEAR), count: index };
        const json = JSON.parse(mod.wideEventEncode(value));
        assertEquals(json, { kind: `k${index}`, at: NEW_YEAR, count: index });
        assertEquals(decoded(mod.wideEventDecode(json)), value);
        assert(mod.wideEventEquals(value, mod.wideEventClone(value)));
    }
    assert(!mod.wideEventDecode({ kind: 'k120', at: NEW_YEAR, count: 1 }).success);
});

Deno.test('a union of 120 members tells each by its shape', () => {
    for (const index of [0, 77, 119]) {
        const value = { shape: `s${index}`, at: new Date(NEW_YEAR), size: index };
        const json = JSON.parse(mod.wideShapeEncode(value));
        assertEquals(json, { shape: `s${index}`, at: NEW_YEAR, size: index });
        assertEquals(decoded(mod.wideShapeDecode(json)), value);
        assert(mod.wideShapeHasShape(json));
    }
    assert(!mod.wideShapeHasShape({ shape: 's120', at: NEW_YEAR, size: 1 }));
    assert(!mod.wideShapeDecode({ shape: 's120', at: NEW_YEAR, size: 1 }).success);
});
