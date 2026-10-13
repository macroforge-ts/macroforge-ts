/**
 * Types at the edge of what a type can say: template literal types, `object`,
 * a lone `null`, unions of literals of several kinds, types with nothing in
 * them, and an interface that declares methods.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'exotic-shapes.ts'));

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const scalars = (change) => ({
    id: 'id-7',
    tag: 'left-true',
    held: { any: 'thing' },
    nothing: null,
    choice: 3,
    ...change
});

Deno.test('a default holds text its template literal type takes', () => {
    assertEquals(mod.exoticScalarsDefaultValue(), {
        id: 'id-0',
        tag: '-false',
        held: {},
        nothing: null,
        choice: 'a'
    });
    assertEquals(
        decoded(mod.exoticScalarsDecode(mod.exoticScalarsEncode(mod.exoticScalarsDefaultValue()))),
        mod.exoticScalarsDefaultValue()
    );
});

Deno.test('a template literal type, an object and a literal are checked as they are decoded', () => {
    const encoded = JSON.parse(mod.exoticScalarsEncode(scalars()));
    assertEquals(decoded(mod.exoticScalarsDecode(encoded)), scalars());
    const fieldsOf = (change) => errorFields(mod.exoticScalarsDecode({ ...encoded, ...change }));

    // A number is spelled in decimal: no spaces, no `0x`, nothing too large.
    const misspelled = ['id- 7', 'id-7 ', 'id-0x10', 'id-+7', 'id-1e999', 'id-Infinity', 'id-NaN'];
    for (const id of [7, 'id-', 'id-seven', 'ID-7', 'id-7 and more', null, ...misspelled]) {
        assertEquals(fieldsOf({ id }), ['id'], JSON.stringify(id));
    }
    for (const id of ['id-1.5', 'id-1e3', 'id--2']) {
        assertStrictEquals(decoded(mod.exoticScalarsDecode({ ...encoded, id })).id, id);
    }
    assertEquals(fieldsOf({ tag: 'left-maybe' }), ['tag']);
    assertEquals(fieldsOf({ tag: 'true' }), ['tag']);
    assertStrictEquals(
        decoded(mod.exoticScalarsDecode({ ...encoded, tag: '-false' })).tag,
        '-false'
    );
    assertEquals(fieldsOf({ held: 5 }), ['held']);
    assertEquals(fieldsOf({ held: null }), ['held']);
    assertEquals(fieldsOf({ nothing: 0 }), ['nothing']);
    assertEquals(fieldsOf({ choice: 'c' }), ['choice']);
    assertEquals(fieldsOf({ choice: '3' }), ['choice']);
});

Deno.test('a union of literals of several kinds is compared and hashed by its value', () => {
    for (const choice of ['a', 'b', 3, true]) {
        const value = scalars({ choice });
        const copy = mod.exoticScalarsClone(value);
        assert(mod.exoticScalarsEquals(value, copy));
        assertStrictEquals(mod.exoticScalarsHashCode(value), mod.exoticScalarsHashCode(copy));
        assert(mod.exoticScalarsToString(value).includes(`choice: ${choice}`));
    }
    assert(!mod.exoticScalarsEquals(scalars({ choice: 3 }), scalars({ choice: true })));
    assertNotStrictEquals(mod.exoticScalarsClone(scalars()).held, scalars().held);
});

Deno.test('an alias of a template literal type is checked like a field of it', () => {
    assertStrictEquals(mod.exoticCodeEncode('code-12'), '"code-12"');
    assertStrictEquals(decoded(mod.exoticCodeDecode('"code-12"')), 'code-12');
    assertStrictEquals(decoded(mod.exoticCodeDecode('code-12')), 'code-12');
    assert(mod.exoticCodeIs('code--3'));
    for (
        const wrong of ['code-1.5', 'code-', 'kode-1', 'code-0x10', 'code- 1', 'code-1n', 12, null]
    ) {
        assert(!mod.exoticCodeDecode(wrong).success, JSON.stringify(wrong));
    }
    assert(mod.exoticCodeEquals('code-1', mod.exoticCodeClone('code-1')));
    assertStrictEquals(mod.exoticCodeHashCode('code-1'), mod.exoticCodeHashCode('code-1'));
});

Deno.test('types with nothing in them are derived all the same', () => {
    assertEquals(mod.exoticNoneDefaultValue(), []);
    assertEquals(mod.exoticNoneClone([]), []);
    assertStrictEquals(mod.exoticNoneEquals([], []), true);
    assertStrictEquals(typeof mod.exoticNoneHashCode([]), 'number');
    assertStrictEquals(mod.exoticNoneEncode([]), '[]');
    assertEquals(decoded(mod.exoticNoneDecode('[]')), []);
    assert(!mod.exoticNoneDecode([1]).success);
    assert(!mod.exoticNoneDecode({}).success);

    assertStrictEquals(mod.exoticEmptyToString({}), 'ExoticEmpty {}');
    assertStrictEquals(mod.exoticEmptyEquals({}, {}), true);
    assertEquals(mod.exoticEmptyDefaultValue(), {});
    assertEquals(decoded(mod.exoticEmptyDecode(mod.exoticEmptyEncode({}))), {});
    assert(!mod.exoticEmptyDecode(5).success);

    const blank = mod.ExoticBlank.defaultValue();
    assert(blank instanceof mod.ExoticBlank);
    assertStrictEquals(mod.ExoticBlank.toString(blank), 'ExoticBlank {}');
    assert(mod.ExoticBlank.equals(blank, mod.ExoticBlank.clone(blank)));
    assert(
        decoded(mod.ExoticBlank.decode(mod.ExoticBlank.encode(blank))) instanceof mod.ExoticBlank
    );
});

Deno.test('the methods an interface declares are members a clone keeps', () => {
    const describe = () => 'described';
    const value = { id: 'a', describe };
    const copy = mod.exoticMethodsClone(value);
    assertNotStrictEquals(copy, value);
    assertStrictEquals(copy.describe, describe);
    assertStrictEquals(copy.describe(), 'described');
    assert(!('maybe' in copy) || copy.maybe === undefined);
    assert(mod.exoticMethodsEquals(value, copy));
    assert(!mod.exoticMethodsEquals(value, { id: 'a', describe: () => 'other' }));
    assertStrictEquals(mod.exoticMethodsHashCode(value), mod.exoticMethodsHashCode(copy));
    assert(mod.exoticMethodsToString(value).includes('id: a'));
});

Deno.test('a union alias tells its template literal members by their text', () => {
    for (const value of ['a-1', 'b-anything', 'b-', 'plain', 5]) {
        assertStrictEquals(decoded(mod.exoticLabelsDecode(value)), value);
        assertStrictEquals(decoded(mod.exoticLabelsDecode(mod.exoticLabelsEncode(value))), value);
        assert(mod.exoticLabelsIs(value));
        assert(mod.exoticLabelsEquals(value, mod.exoticLabelsClone(value)));
        assertStrictEquals(typeof mod.exoticLabelsHashCode(value), 'number');
    }
    for (const wrong of ['a-x', 'a-', 'c-1', 'Plain', 6, null, {}]) {
        assert(!mod.exoticLabelsDecode(wrong).success, JSON.stringify(wrong));
        assert(!mod.exoticLabelsIs(wrong), JSON.stringify(wrong));
    }
});

Deno.test('template literal types are checked wherever a type holds them', () => {
    const texts = () => ({
        either: 'a-1',
        list: ['k-1', 'k-2'],
        mapped: new Map([['m-5', 'five']]),
        nullable: null
    });
    const encoded = JSON.parse(mod.exoticTextsEncode(texts()));
    assertEquals(encoded, {
        either: 'a-1',
        list: ['k-1', 'k-2'],
        mapped: { 'm-5': 'five' },
        nullable: null
    });
    assertEquals(decoded(mod.exoticTextsDecode(encoded)), texts());
    assert(mod.exoticTextsEquals(texts(), mod.exoticTextsClone(texts())));
    assertStrictEquals(mod.exoticTextsHashCode(texts()), mod.exoticTextsHashCode(texts()));
    assert(!mod.exoticTextsEquals(texts(), { ...texts(), list: ['k-1', 'k-3'] }));

    const fieldsOf = (change) => errorFields(mod.exoticTextsDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ either: 'c-1' }), ['either']);
    assertEquals(fieldsOf({ either: 'a-one' }), ['either']);
    assertEquals(fieldsOf({ list: ['k-1', 'k-x'] }), ['list[1]']);
    assertEquals(fieldsOf({ mapped: { 'm-1.5': 'no' } }), ['mapped.m-1.5']);
    assertEquals(fieldsOf({ optional: 'o-maybe' }), ['optional']);
    assertEquals(fieldsOf({ nullable: 5 }), ['nullable']);
    assertStrictEquals(
        decoded(mod.exoticTextsDecode({ ...encoded, optional: 'o-true' })).optional,
        'o-true'
    );
});

Deno.test('a record is checked by the keys its key type takes', () => {
    assertEquals(mod.exoticKeyedDefaultValue(), {
        finite: { a: 0, b: 0 },
        counted: {},
        patterned: {},
        open: {}
    });
    const keyed = {
        finite: { a: 1, b: 2 },
        counted: { 1: 'one', 2.5: 'more' },
        patterned: { 'k-1': true },
        open: { anything: 1 }
    };
    assertEquals(decoded(mod.exoticKeyedDecode(mod.exoticKeyedEncode(keyed))), keyed);

    const fieldsOf = (change) => errorFields(mod.exoticKeyedDecode({ ...keyed, ...change }));
    assertEquals(fieldsOf({ finite: { a: 1 } }), ['finite.b']);
    assertEquals(fieldsOf({ finite: { a: 1, b: 2, c: 3 } }), ['finite.c']);
    assertEquals(fieldsOf({ finite: { a: 'one', b: 2 } }), ['finite.a']);
    assertEquals(fieldsOf({ counted: { one: 'no' } }), ['counted.one']);
    assertEquals(fieldsOf({ patterned: { 'k-x': true } }), ['patterned.k-x']);
    assertEquals(fieldsOf({ open: { anything: 'no' } }), ['open.anything']);
});

Deno.test('a getter signature declares a field, and a method of an object type a member', () => {
    const blank = mod.exoticAccessorsDefaultValue();
    assertStrictEquals(blank.size, 0);
    assert(blank.stamp instanceof Date);

    const value = { id: 'a', size: 2, stamp: new Date(0) };
    const encoded = JSON.parse(mod.exoticAccessorsEncode(value));
    assertEquals(encoded, { id: 'a', size: 2, stamp: '1970-01-01T00:00:00.000Z' });
    assertEquals(decoded(mod.exoticAccessorsDecode(encoded)), value);
    assertEquals(errorFields(mod.exoticAccessorsDecode({ ...encoded, size: 'two' })), ['size']);
    assertEquals(errorFields(mod.exoticAccessorsDecode({ ...encoded, stamp: 'never' })), ['stamp']);
    assert(mod.exoticAccessorsEquals(value, mod.exoticAccessorsClone(value)));
    assert(!mod.exoticAccessorsEquals(value, { ...value, size: 3 }));

    const run = () => 'ran';
    const copy = mod.exoticMethodAliasClone({ id: 'a', run });
    assertStrictEquals(copy.run, run);
    assert(mod.exoticMethodAliasEquals({ id: 'a', run }, copy));
    assert(!mod.exoticMethodAliasEquals({ id: 'a', run }, { id: 'a', run: () => 'other' }));
});
