/**
 * Fields typed by an operator over a type from another module: `keyof` it,
 * and an indexed access whose type that module names and this one does not
 * import.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'operator-shapes.ts'));

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const since = new Date('2024-01-01T00:00:00.000Z');
const owner = (name) => ({ name, since });
const holder = () => ({
    owner: owner('a'),
    owners: [owner('b'), owner('c')],
    label: 'l',
    weight: undefined,
    key: 'owners'
});

Deno.test('an indexed access is coded as the type it yields, from the module that declares it', () => {
    const encoded = JSON.parse(mod.operatorHolderEncode(holder()));
    assertEquals(encoded.owner, { name: 'a', since: since.toISOString() });
    assertEquals(encoded.owners.length, 2);

    const value = decoded(mod.operatorHolderDecode(encoded));
    assert(value.owner.since instanceof Date);
    assert(value.owners[1].since instanceof Date);
    assert(mod.operatorHolderEquals(value, holder()));
    assertStrictEquals(mod.operatorHolderHashCode(value), mod.operatorHolderHashCode(holder()));
    assert(mod.operatorHolderEquals(holder(), mod.operatorHolderClone(holder())));

    const fieldsOf = (change) => errorFields(mod.operatorHolderDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ owner: { name: 1, since: since.toISOString() } }), ['owner.name']);
    assertEquals(fieldsOf({ owners: [{ name: 'b', since: 'never' }] }), ['owners[0].since']);
    assertEquals(fieldsOf({ label: 5 }), ['label']);
    assertEquals(fieldsOf({ weight: 'heavy' }), ['weight']);
    assertEquals(fieldsOf({ key: 'missing' }), ['key']);
    assertStrictEquals(decoded(mod.operatorHolderDecode({ ...encoded, weight: 2 })).weight, 2);
});

Deno.test('a field typed by an operator defaults as the type the operator yields', () => {
    const value = mod.operatorHolderDefaultValue();
    assertStrictEquals(value.owner.name, '');
    assert(value.owner.since instanceof Date);
    assertEquals(value.owners, []);
    assertStrictEquals(value.label, '');
    assertStrictEquals(value.weight, undefined);
    assertStrictEquals(value.key, 'owner');
});
