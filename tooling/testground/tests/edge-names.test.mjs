/**
 * Names that could collide with generated code or that need quoting: fields
 * named like generated locals, keys that are not identifiers, acronym and
 * lowercase type names, and type parameters named like generated ones.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'name-shapes.ts'));

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const locals = () => ({
    value: 'value',
    ctx: 'ctx',
    result: 'result',
    obj: 'obj',
    errors: ['errors'],
    instance: 'instance',
    fields: 'fields',
    hash: 1,
    parts: ['parts'],
    a: 2,
    b: 3,
    input: 'input',
    opts: 'opts',
    data: 'data',
    item: 'item',
    key: 'key',
    idx: 4,
    other: 'other'
});

Deno.test("fields named like the generated code's own locals are fields like any other", () => {
    const source = locals();
    assertEquals(JSON.parse(mod.nameLocalsEncode(source)), source);
    assertEquals(decoded(mod.nameLocalsFromString(mod.nameLocalsEncode(source))), source);

    const copy = mod.nameLocalsClone(source);
    assertEquals(copy, source);
    assertNotStrictEquals(copy.errors, source.errors);
    assert(mod.nameLocalsEquals(source, copy));
    assertStrictEquals(mod.nameLocalsHashCode(source), mod.nameLocalsHashCode(copy));
    assertStrictEquals(mod.nameLocalsCompare(source, copy), 0);

    for (const [field, value] of Object.entries(source)) {
        const changed = { ...source, [field]: typeof value === 'number' ? value + 1 : ['changed'] };
        if (typeof value === 'string') changed[field] = `${value}!`;
        assert(!mod.nameLocalsEquals(source, changed), `${field} takes part in equality`);
        assert(mod.nameLocalsCompare(source, changed) !== 0, `${field} takes part in the order`);
        assert(mod.nameLocalsToString(source).includes(`${field}: `), `${field} is shown`);
    }

    assertEquals(Object.keys(mod.nameLocalsDefaultValue()), Object.keys(source));
    for (const field of Object.keys(source)) {
        const { [field]: _dropped, ...rest } = source;
        assertEquals(errorFields(mod.nameLocalsDecode(rest)), [field]);
    }
});

const quoted = () => ({
    'kebab-key': 'kebab',
    'with space': 1,
    'dotted.key': true,
    0: 'zero',
    class: 'class',
    default: 2,
    new: false,
    function: 'function',
    typeof: 'typeof',
    $dollar: 'dollar',
    _underscore: 'underscore',
    ünïcode: 'unicode'
});

Deno.test('keys that are not identifiers and reserved words are fields like any other', () => {
    const source = quoted();
    assertEquals(JSON.parse(mod.nameQuotedEncode(source)), source);
    assertEquals(decoded(mod.nameQuotedFromString(mod.nameQuotedEncode(source))), source);
    assertEquals(mod.nameQuotedClone(source), source);
    assert(mod.nameQuotedEquals(source, quoted()));
    assertStrictEquals(mod.nameQuotedHashCode(source), mod.nameQuotedHashCode(quoted()));

    for (const [field, value] of Object.entries(source)) {
        const other = typeof value === 'string'
            ? `${value}!`
            : typeof value === 'number'
            ? value + 1
            : !value;
        const changed = { ...source, [field]: other };
        assert(!mod.nameQuotedEquals(source, changed), `${field} takes part in equality`);
        assert(mod.nameQuotedCompare(source, changed) !== 0, `${field} takes part in the order`);
        assert(mod.nameQuotedToString(source).includes(`${field}: `), `${field} is shown`);

        const { [field]: _dropped, ...rest } = source;
        assertEquals(errorFields(mod.nameQuotedDecode(rest)), [field]);
    }

    assertEquals(
        Object.keys(mod.nameQuotedDefaultValue()).sort(),
        Object.keys(source).sort()
    );
    assertEquals(errorFields(mod.nameQuotedDecode({ ...source, 'kebab-key': 5 })), ['kebab-key']);
});

Deno.test('a class with quoted and reserved-word fields is built with them', () => {
    const blank = mod.NameQuotedClass.defaultValue();
    assert(blank instanceof mod.NameQuotedClass);
    assertEquals({ ...blank }, { 'kebab-key': '', class: '', default: 0 });

    const value = decoded(
        mod.NameQuotedClass.decode({ 'kebab-key': 'kebab', class: 'class', default: 3 })
    );
    assert(value instanceof mod.NameQuotedClass);
    assertStrictEquals(value['kebab-key'], 'kebab');
    assertEquals(JSON.parse(mod.NameQuotedClass.encode(value)), {
        'kebab-key': 'kebab',
        class: 'class',
        default: 3
    });

    const copy = mod.NameQuotedClass.clone(value);
    assert(copy instanceof mod.NameQuotedClass);
    assert(mod.NameQuotedClass.equals(value, copy));
    assertStrictEquals(mod.NameQuotedClass.hashCode(value), mod.NameQuotedClass.hashCode(copy));
    assert(mod.NameQuotedClass.toString(value).includes('kebab-key: kebab'));
});

Deno.test('acronym and lowercase type names give functions that find one another', () => {
    assertStrictEquals(typeof mod.httpResponseEncode, 'function');
    assertStrictEquals(typeof mod.point2DEncode, 'function');

    const holder = {
        response: { statusCode: 200 },
        responses: [{ statusCode: 404 }],
        point: { x: 1, y: 2 },
        points: new Map([['origin', { x: 0, y: 0 }]])
    };
    const value = decoded(mod.nameHolderFromString(mod.nameHolderEncode(holder)));
    assertEquals(value, holder);

    const copy = mod.nameHolderClone(holder);
    assertNotStrictEquals(copy.response, holder.response);
    assertNotStrictEquals(copy.points.get('origin'), holder.points.get('origin'));
    assert(mod.nameHolderEquals(holder, copy));
    assertStrictEquals(mod.nameHolderHashCode(holder), mod.nameHolderHashCode(copy));
    assertEquals(mod.nameHolderDefaultValue(), {
        response: { statusCode: 0 },
        responses: [],
        point: { x: 0, y: 0 },
        points: new Map()
    });

    assertEquals(
        errorFields(
            mod.nameHolderDecode({ ...JSON.parse(mod.nameHolderEncode(holder)), point: { x: 1 } })
        ),
        ['point.y']
    );
    assertEquals(
        errorFields(
            mod.nameHolderDecode({
                ...JSON.parse(mod.nameHolderEncode(holder)),
                responses: [{ statusCode: 'ok' }]
            })
        ),
        ['responses[0].statusCode']
    );
});

Deno.test('type parameters named like generated ones do not collide with them', () => {
    const params = { key: 'k', other: 1, held: { nested: true }, values: [1, 2] };
    assertEquals(decoded(mod.nameParamsFromString(mod.nameParamsEncode(params))), params);
    const copy = mod.nameParamsClone(params);
    assert(mod.nameParamsEquals(params, copy));
    assertNotStrictEquals(copy.values, params.values);
    assertStrictEquals(mod.nameParamsHashCode(params), mod.nameParamsHashCode(copy));

    const instance = decoded(mod.NameParamsClass.decode({ key: 'k', other: null, held: [1] }));
    assert(instance instanceof mod.NameParamsClass);
    assertEquals(JSON.parse(mod.NameParamsClass.encode(instance)), {
        key: 'k',
        other: null,
        held: [1]
    });
    assert(mod.NameParamsClass.equals(instance, mod.NameParamsClass.clone(instance)));
});

Deno.test('a type named in lowercase is coded through its functions from another module', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-imports.ts')
    );
    const point = (x, y) => ({ x, y });
    const source = {
        origin: point(0, 0),
        path: [point(1, 2), point(3, 4)],
        byName: new Map([['a', point(5, 6)]]),
        maybe: point(7, 8)
    };
    const value = decoded(imports.nameImportsFromString(imports.nameImportsEncode(source)));
    assertEquals(value, source);
    assert(value.byName instanceof Map);

    const encoded = JSON.parse(imports.nameImportsEncode(source));
    assertEquals(errorFields(imports.nameImportsDecode({ ...encoded, origin: { x: 1 } })), [
        'origin.y'
    ]);
    assertEquals(
        errorFields(imports.nameImportsDecode({ ...encoded, path: [{ x: 1, y: 'two' }] })),
        ['path[0].y']
    );
    assertEquals(errorFields(imports.nameImportsDecode({ ...encoded, maybe: { y: 1 } })), [
        'maybe.x'
    ]);

    const copy = imports.nameImportsClone(source);
    assertNotStrictEquals(copy.origin, source.origin);
    assert(imports.nameImportsEquals(source, copy));
    assertStrictEquals(imports.nameImportsHashCode(source), imports.nameImportsHashCode(copy));
    assert(imports.nameImportsToString(source).includes('origin'));
    assertEquals(imports.nameImportsDefaultValue().origin, point(0, 0));
});

Deno.test('fields named like what every object inherits are read from the input alone', () => {
    const value = {
        constructor: 'built',
        toString: 'text',
        valueOf: 7,
        hasOwnProperty: true,
        length: 2,
        then: 'later'
    };
    const names = Object.keys(value);

    // An empty object answers to all of these names, and has none of them.
    const empty = mod.namePrototypeDecode({});
    assertEquals(errorFields(empty).sort(), [...names].sort());
    assert(empty.errors.every((error) => error.message === 'missing required field'));
    assert(!mod.namePrototypeHasShape({}));
    assert(mod.namePrototypeHasShape(value));

    const encoded = JSON.parse(mod.namePrototypeEncode(value));
    assertEquals(Object.keys(encoded).sort(), [...names].sort());
    const back = decoded(mod.namePrototypeDecode(encoded));
    for (const name of names) assertStrictEquals(back[name], value[name]);

    const copy = mod.namePrototypeClone(value);
    assert(mod.namePrototypeEquals(value, copy));
    assertStrictEquals(mod.namePrototypeHashCode(value), mod.namePrototypeHashCode(copy));
    assert(!mod.namePrototypeEquals(value, { ...value, constructor: 'other' }));
    const blank = mod.namePrototypeDefaultValue();
    assertStrictEquals(blank.constructor, '');
    assertStrictEquals(blank.valueOf, 0);
    assert(mod.namePrototypeToString(value).includes('constructor: built'));
});

Deno.test('a type imported under another name is coded through its own functions', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-imports.ts')
    );
    const item = (id) => ({ id, score: 1 });
    const source = {
        held: item('a'),
        listed: [item('b'), item('c')],
        maybe: null,
        some: { id: 'p' }
    };
    const encoded = JSON.parse(imports.nameRenamedEncode(source));
    assertEquals(encoded, source);
    assertEquals(decoded(imports.nameRenamedDecode(encoded)), source);
    assertEquals(
        errorFields(imports.nameRenamedDecode({ ...encoded, held: { id: 1, score: 1 } })),
        ['held.id']
    );
    assertEquals(
        errorFields(imports.nameRenamedDecode({ ...encoded, listed: [{ id: 'b', score: 'one' }] })),
        ['listed[0].score']
    );
    assertEquals(errorFields(imports.nameRenamedDecode({ ...encoded, some: { id: 2 } })), [
        'some.id'
    ]);

    const copy = imports.nameRenamedClone(source);
    assertNotStrictEquals(copy.held, source.held);
    assert(imports.nameRenamedEquals(source, copy));
    assertStrictEquals(imports.nameRenamedHashCode(source), imports.nameRenamedHashCode(copy));
    assertEquals(imports.nameRenamedDefaultValue().held, { id: '', score: 0 });
});

Deno.test('a type its module exports under another name is coded through its own functions', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-imports.ts')
    );
    const at = new Date(1000);
    const source = {
        chosen: { title: 't', at },
        shown: { note: 'n', at, stamp: 1 },
        listed: [{ note: 'm', at, stamp: 2 }],
        maybe: null
    };
    const encoded = JSON.parse(imports.nameExportedEncode(source));
    assertEquals(encoded.shown, { note: 'n', at: at.toISOString(), stamp: 1 });
    assertEquals(encoded.chosen.at, at.toISOString());

    const value = decoded(imports.nameExportedDecode(encoded));
    assert(value.shown.at instanceof Date);
    assert(value.chosen.at instanceof Date);
    assertEquals(value.listed, source.listed);
    assertEquals(
        errorFields(
            imports.nameExportedDecode({
                ...encoded,
                shown: { note: 1, at: at.toISOString(), stamp: 1 }
            })
        ),
        ['shown.note']
    );

    // A field the type leaves out of its equality is left out through the
    // name it is imported under too.
    const restamped = { ...source, shown: { ...source.shown, stamp: 9 } };
    assert(imports.nameExportedEquals(source, restamped));
    assertStrictEquals(
        imports.nameExportedHashCode(source),
        imports.nameExportedHashCode(restamped)
    );
    assert(
        !imports.nameExportedEquals(source, {
            ...source,
            shown: { ...source.shown, note: 'other' }
        })
    );
    assertEquals(
        errorFields(
            imports.nameExportedDecode({ ...encoded, chosen: { title: 't', at: 'never' } })
        ),
        ['chosen.at']
    );

    const blank = imports.nameExportedDefaultValue();
    assertStrictEquals(blank.shown.note, '');
    assertStrictEquals(blank.chosen.title, '');
});

Deno.test('a type a barrel exports again is coded through its own functions', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-imports.ts')
    );
    const at = new Date(2000);
    const source = {
        model: { title: 't', at },
        shown: { note: 'n', at, stamp: 1 },
        item: { id: 'i', score: 1 },
        items: [{ id: 'j', score: 2 }]
    };
    const encoded = JSON.parse(imports.nameBarrelledEncode(source));
    assertEquals(encoded.model.at, at.toISOString());
    assertEquals(encoded.shown.at, at.toISOString());

    const value = decoded(imports.nameBarrelledDecode(encoded));
    assert(value.model.at instanceof Date);
    assert(value.shown.at instanceof Date);
    assertEquals(value.items, source.items);
    assertEquals(
        errorFields(imports.nameBarrelledDecode({ ...encoded, item: { id: 1, score: 1 } })),
        ['item.id']
    );
    assertEquals(
        errorFields(
            imports.nameBarrelledDecode({ ...encoded, model: { title: 't', at: 'never' } })
        ),
        ['model.at']
    );
    assertStrictEquals(imports.nameBarrelledDefaultValue().item.id, '');
    assert(
        imports.nameBarrelledEquals(source, { ...source, shown: { ...source.shown, stamp: 7 } })
    );
});

Deno.test('a type a barrel imports and then exports is coded through its own functions', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-imports.ts')
    );
    const at = new Date(3000);
    const source = { handed: { note: 'n', at, stamp: 1 }, listed: [{ note: 'm', at, stamp: 2 }] };
    const encoded = JSON.parse(imports.nameHandedEncode(source));
    assertEquals(encoded.handed, { note: 'n', at: at.toISOString(), stamp: 1 });

    const value = decoded(imports.nameHandedDecode(encoded));
    assert(value.handed.at instanceof Date);
    assertEquals(value.listed, source.listed);
    assertEquals(
        errorFields(
            imports.nameHandedDecode({
                ...encoded,
                handed: { note: 1, at: at.toISOString(), stamp: 1 }
            })
        ),
        ['handed.note']
    );
    assert(imports.nameHandedEquals(source, { ...source, handed: { ...source.handed, stamp: 7 } }));
    assertStrictEquals(imports.nameHandedDefaultValue().handed.note, '');
});

Deno.test('an intersection takes the fields of a type imported under another name', async () => {
    const imports = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'name-intersections.ts')
    );
    const source = { id: 'a', score: 1, extra: 'more' };
    assertEquals(JSON.parse(imports.nameMixedEncode(source)), source);
    assertEquals(decoded(imports.nameMixedFromString(imports.nameMixedEncode(source))), source);
    assertEquals(errorFields(imports.nameMixedDecode({ id: 'a', extra: 'more' })), ['score']);
    assertEquals(errorFields(imports.nameMixedDecode({ id: 1, score: 1, extra: 'more' })), ['id']);

    const copy = imports.nameMixedClone(source);
    assertEquals(copy, source);
    assertNotStrictEquals(copy, source);
    assert(imports.nameMixedEquals(source, copy));
    assert(!imports.nameMixedEquals(source, { ...source, score: 2 }));
    assertStrictEquals(imports.nameMixedHashCode(source), imports.nameMixedHashCode(copy));
    assertEquals(imports.nameMixedDefaultValue(), { id: '', score: 0, extra: '' });
    assert(imports.nameMixedToString(source).includes('score: 1'));
});
