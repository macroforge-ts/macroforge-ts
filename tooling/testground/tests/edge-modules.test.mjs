/**
 * Declarations by where and how a module writes them: types it keeps to
 * itself, classes with static members, a superclass that derives nothing,
 * overloads and optional methods.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'module-shapes.ts'));
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

Deno.test('a type its module keeps to itself is derived like any other', () => {
    const shown = mod.usePrivate();
    for (
        const part of [
            'ModulePrivate { a: x }',
            'ModulePrivateClass {',
            'ModulePrivateAlias { a: y }',
            'ModulePrivateEnum.B'
        ]
    ) {
        assert(shown.includes(part), `${part} in ${shown}`);
    }
});

Deno.test('static members are no fields of a value', () => {
    const made = mod.ModuleStatics.made();
    const value = mod.ModuleStatics.defaultValue();
    assertEquals(JSON.parse(mod.ModuleStatics.encode(value)), { name: '' });
    const back = decoded(mod.ModuleStatics.decode({ name: 'n', count: 5 }));
    assert(back instanceof mod.ModuleStatics);
    assertStrictEquals(mod.ModuleStatics.count, 0);
    assertStrictEquals(mod.ModuleStatics.made(), made);
    assert(mod.ModuleStatics.equals(back, mod.ModuleStatics.clone(back)));
});

Deno.test('a decoded class keeps its methods, its overloads and its superclass', () => {
    const named = decoded(mod.ModuleImplements.decode({ name: 'n' }));
    assertStrictEquals(named.describe(), 'n');

    const overloaded = decoded(mod.ModuleOverloads.decode({ name: 'n' }));
    assertStrictEquals(overloaded.pick(1), 'n:1');
    assertStrictEquals(overloaded.pick('k'), 'n:k');

    const plain = mod.ModuleExtendsPlain.defaultValue();
    assert(plain instanceof mod.ModulePlainBase);
    const encoded = JSON.parse(mod.ModuleExtendsPlain.encode(plain));
    assertEquals(encoded, { plain: 'p', extra: 0 });
    const back = decoded(mod.ModuleExtendsPlain.decode({ plain: 'q', extra: 2 }));
    assert(back instanceof mod.ModulePlainBase);
    assertStrictEquals(back.plain, 'q');
    assertStrictEquals(back.extra, 2);

    const optional = decoded(
        mod.ModuleOptionalMethods.decode({ name: 'n', tags: ['a'], pair: ['b', 2] })
    );
    assertEquals(optional.tags, ['a']);
    assertEquals(optional.pair, ['b', 2]);
    assert(mod.ModuleOptionalMethods.equals(optional, mod.ModuleOptionalMethods.clone(optional)));
});
