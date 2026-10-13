/**
 * Records and collections by the kind of key they are over, whole and under
 * `Partial`, `Pick` and `Omit`, and a class whose fields are typed by their
 * initializers under each decorator option.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'keyed-shapes.ts'));
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};
const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};
const keyed = () => ({
    byLevel: { 0: 'low', 1: 'high' },
    byKind: { a: 1, b: 2 },
    some: { x: 1 },
    levels: new Map([[1, 'high']]),
    kinds: new Set(['a']),
    codes: new Set(['c-1']),
    deep: { inner: { at: new Date(0), tags: ['t'] } }
});

Deno.test('a record and a collection are checked by the keys and members their types take', () => {
    assertEquals(mod.keyedRecordsDefaultValue(), {
        byLevel: { 0: '', 1: '' },
        byKind: { a: 0, b: 0 },
        some: {},
        levels: new Map(),
        kinds: new Set(),
        codes: new Set()
    });
    const encoded = JSON.parse(mod.keyedRecordsEncode(keyed()));
    assertEquals(decoded(mod.keyedRecordsDecode(encoded)), keyed());
    assert(mod.keyedRecordsEquals(keyed(), mod.keyedRecordsClone(keyed())));
    assertStrictEquals(mod.keyedRecordsHashCode(keyed()), mod.keyedRecordsHashCode(keyed()));
    const fieldsOf = (change) => errorFields(mod.keyedRecordsDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ byLevel: { 0: 'low' } }), ['byLevel.1']);
    assertEquals(fieldsOf({ byLevel: { 0: 'low', 1: 'high', 2: 'no' } }), ['byLevel.2']);
    assertEquals(fieldsOf({ byKind: { a: 1 } }), ['byKind.b']);
    assertEquals(fieldsOf({ byKind: { a: 1, b: 2, c: 3 } }), ['byKind.c']);
    // A key outside a partial record's finite set is left behind, as an object's is.
    assertEquals(decoded(mod.keyedRecordsDecode({ ...encoded, some: { z: 1, y: 2 } })).some, {
        y: 2
    });
    assertEquals(fieldsOf({ some: { x: 'one' } }), ['some.x']);
    assertEquals(fieldsOf({ levels: { 5: 'no' } }), ['levels.5']);
    assertEquals(fieldsOf({ kinds: ['c'] }), ['kinds[0]']);
    assertEquals(fieldsOf({ codes: ['c-x'] }), ['codes[0]']);
    assertEquals(fieldsOf({ deep: { inner: { at: 'never' } } }), ['deep.inner.at']);
    assertEquals(fieldsOf({ deep: { inner: { tags: [1] } } }), ['deep.inner.tags[0]']);
    assertEquals(decoded(mod.keyedRecordsDecode({ ...encoded, deep: {} })).deep, {});
});

Deno.test('decorator options apply to fields typed by their initializers', () => {
    const blank = mod.KeyedOptions.defaultValue();
    assertStrictEquals(blank.five, 5);
    const text = mod.KeyedOptions.encode(blank);
    const encoded = JSON.parse(text);
    assertEquals(encoded, {
        n: 0,
        user_label: 'none',
        derived: 1,
        weight: 1,
        secret: 's',
        noise: 0,
        ignored: 0,
        five: 5,
        deepValue: 1
    });
    const back = decoded(mod.KeyedOptions.decode({ ...encoded, hidden: true }));
    assert(back instanceof mod.KeyedOptions);
    assertStrictEquals(back.count, 0);
    assertStrictEquals(back.hidden, true);
    assertEquals(back.inner, { deepValue: 1 });
    const { user_label: _label, ...withoutLabel } = encoded;
    // `default: true` fills in the default of the field's type.
    assertStrictEquals(
        decoded(mod.KeyedOptions.decode({ ...withoutLabel, hidden: false })).userLabel,
        ''
    );
    assertEquals(errorFields(mod.KeyedOptions.decode({ ...encoded, hidden: false, weight: -1 })), [
        'weight'
    ]);
    assertEquals(errorFields(mod.KeyedOptions.decode({ ...encoded, hidden: false, n: 'zero' })), [
        'n'
    ]);
    assert(!mod.KeyedOptions.toString(blank).includes('secret'));
    const other = mod.KeyedOptions.clone(blank);
    other.noise = 9;
    other.ignored = 9;
    assert(
        mod.KeyedOptions.equals(blank, Object.assign(mod.KeyedOptions.clone(blank), { ignored: 9 }))
    );
    assertStrictEquals(
        mod.KeyedOptions.hashCode(blank),
        mod.KeyedOptions.hashCode(Object.assign(mod.KeyedOptions.clone(blank), { noise: 9 }))
    );
});

Deno.test('a selection of a type checks the fields it keeps as the type does', () => {
    const value = {
        partial: { byKind: { a: 1, b: 2 } },
        picked: { byKind: { a: 1, b: 2 }, codes: new Set(['c-1']) },
        omitted: { byKind: { a: 1, b: 2 }, some: {}, kinds: new Set(['b']), codes: new Set() },
        required: { deep: { inner: {} } }
    };
    const encoded = JSON.parse(mod.keyedSelectionsEncode(value));
    assertEquals(decoded(mod.keyedSelectionsDecode(encoded)), value);
    const fieldsOf = (name, change) =>
        errorFields(
            mod.keyedSelectionsDecode({ ...encoded, [name]: { ...encoded[name], ...change } })
        );
    assertEquals(fieldsOf('partial', { byKind: { a: 1 } }), ['partial.byKind.b']);
    assertEquals(fieldsOf('picked', { codes: ['c-x'] }), ['picked.codes[0]']);
    assertEquals(fieldsOf('omitted', { kinds: ['z'] }), ['omitted.kinds[0]']);
    assertEquals(errorFields(mod.keyedSelectionsDecode({ ...encoded, required: {} })), [
        'required.deep'
    ]);
});
