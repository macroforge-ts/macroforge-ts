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
