/**
 * Input nobody vouches for: keys that reach a prototype, values of the wrong
 * kind, members that throw, references to nothing and nesting deeper than
 * the stack. A decode fails for each without throwing, and takes nothing
 * from the input but what its type declares.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'hostile-shapes.ts'));
const bag = (change = {}) => ({
    name: 'n',
    item: { id: 'i', score: 1 },
    items: [{ id: 'a', score: 2 }],
    byName: { one: { id: 'o', score: 3 } },
    counts: { x: 1 },
    lookup: { k: 1 },
    tags: ['t'],
    pair: ['p', 1],
    at: '2020-01-01T00:00:00.000Z',
    big: '12',
    flag: true,
    nested: null,
    ...change
});
const check = (label, run) => {
    try {
        const result = run();
        return result;
    } catch (error) {
        throw new Error(`${label} threw: ${error && error.stack}`);
    }
};
const fails = (label, input) => {
    const result = check(label, () => mod.hostileBagDecode(input));
    assert(!result.success, `${label}: expected a failure`);
    return result.errors;
};
const passes = (label, input) => {
    const result = check(label, () => mod.hostileBagDecode(input));
    assert(result.success, `${label}: ${JSON.stringify(result.errors)}`);
    return result.value;
};

Deno.test('the baseline decodes', () => {
    const value = passes('baseline', bag());
    assertStrictEquals(value.big, 12n);
    assert(value.at instanceof Date);
    assert(value.lookup instanceof Map);
    assert(value.tags instanceof Set);
});

Deno.test('roots that are no object fail without throwing', () => {
    for (
        const root of [
            null,
            undefined,
            1,
            true,
            [],
            [bag()],
            'text',
            '',
            '{"name": 1',
            'null',
            '[]',
            () => 1,
            Symbol('s'),
            10n
        ]
    ) {
        fails(`root ${String(root)}`, root);
    }
});

Deno.test('__proto__ and prototype keys pollute nothing', () => {
    const text = JSON.stringify(bag()).replace('"one":', '"__proto__":');
    const result = check('proto record', () => mod.hostileBagDecode(text));
    assertStrictEquals({}.id, undefined);
    assertStrictEquals(Object.prototype.score, undefined);
    if (result.success) {
        assertStrictEquals(
            Object.getPrototypeOf(result.value.byName),
            Object.prototype,
            'record prototype replaced'
        );
        assert(Object.hasOwn(result.value.byName, '__proto__'), 'the __proto__ entry was dropped');
    }
    const counts = JSON.parse(
        '{"__proto__": 5, "constructor": 6, "toString": 7, "hasOwnProperty": 8}'
    );
    const kept = passes('proto counts', bag({ counts }));
    assertStrictEquals(Object.getPrototypeOf(kept.counts), Object.prototype);
    assertEquals(Object.keys(kept.counts).sort(), [
        '__proto__',
        'constructor',
        'hasOwnProperty',
        'toString'
    ]);
    assertStrictEquals(Object.getOwnPropertyDescriptor(kept.counts, '__proto__').value, 5);
    // And encoding it back keeps the keys.
    const again = JSON.parse(mod.hostileBagEncode(kept));
    assertEquals(Object.keys(again.counts).sort(), [
        '__proto__',
        'constructor',
        'hasOwnProperty',
        'toString'
    ]);

    const lookup = JSON.parse('{"__proto__": 5, "constructor": 6}');
    const mapped = passes('proto map', bag({ lookup }));
    assertEquals([...mapped.lookup.keys()].sort(), ['__proto__', 'constructor']);
    const encodedMap = JSON.parse(mod.hostileBagEncode(mapped));
    assertEquals(Object.keys(encodedMap.lookup).sort(), ['__proto__', 'constructor']);

    const rootProto = JSON.parse(
        JSON.stringify(bag()).replace(
            '{"name"',
            '{"__proto__": {"name": "evil", "extra": 1}, "name"'
        )
    );
    const rooted = passes('proto root', rootProto);
    assertStrictEquals(rooted.name, 'n');
    assertStrictEquals(Object.getPrototypeOf(rooted), Object.prototype);

    // A member the input only inherits is not there.
    const inherited = Object.create(bag());
    fails('inherited members', inherited);
});

Deno.test('arrays are arrays', () => {
    const sparse = [{ id: 'a', score: 1 }];
    sparse[2] = { id: 'c', score: 3 };
    fails('sparse', bag({ items: sparse }));
    fails('array-like', bag({ items: { 0: { id: 'a', score: 1 }, length: 1 } }));
    fails('string for array', bag({ items: 'ab' }));
    fails('string for set', bag({ tags: 'ab' }));
    fails('object for set', bag({ tags: { 0: 'a', length: 1 } }));
    fails('long tuple', bag({ pair: ['p', 1, 2] }));
    fails('short tuple', bag({ pair: ['p'] }));
    fails('tuple-like', bag({ pair: { 0: 'p', 1: 1, length: 2 } }));
    fails('array for record', bag({ counts: [1, 2] }));
    fails('array for map', bag({ lookup: [1, 2] }));
    fails('array for object', bag({ item: ['i', 1] }));
    fails('null for record', bag({ counts: null }));
});

Deno.test('scalars are checked, not coerced', () => {
    fails('number for string', bag({ name: 1 }));
    fails('String object', bag({ name: new String('n') }));
    fails('string for number', bag({ item: { id: 'i', score: '1' } }));
    fails('Number object', bag({ item: { id: 'i', score: new Number(1) } }));
    fails('number for boolean', bag({ flag: 1 }));
    fails('string for boolean', bag({ flag: 'true' }));
    fails('invalid date', bag({ at: '2020-13-45' }));
    fails('empty date', bag({ at: '' }));
    fails('boolean date', bag({ at: true }));
    fails('null date', bag({ at: null }));
    fails('fraction bigint', bag({ big: '1.5' }));
    fails('word bigint', bag({ big: 'abc' }));
    fails('empty bigint', bag({ big: '' }));
    fails('spaced bigint', bag({ big: ' 12 ' }));
    fails('hex bigint', bag({ big: '0x10' }));
    fails('fraction number bigint', bag({ big: 1.5 }));
    // A number past the safe integers has lost digits before it arrives.
    fails('unsafe number bigint', bag({ big: 1e21 }));
    fails('unsafe number bigint', bag({ big: Number.MAX_SAFE_INTEGER + 1 }));
    assertStrictEquals(
        passes('safe number bigint', bag({ big: Number.MAX_SAFE_INTEGER })).big,
        9007199254740991n
    );
    fails('boolean bigint', bag({ big: true }));
    fails('null bigint', bag({ big: null }));
    assertStrictEquals(passes('negative bigint', bag({ big: '-12' })).big, -12n);
    assertStrictEquals(
        passes('huge bigint', bag({ big: '123456789012345678901234567890' })).big,
        123456789012345678901234567890n
    );
});

Deno.test('getters, proxies and throwing members fail without throwing', () => {
    const trapped = bag();
    Object.defineProperty(trapped, 'name', {
        enumerable: true,
        get() {
            throw new Error('boom');
        }
    });
    fails('throwing getter', trapped);
    const proxy = new Proxy(bag(), {
        get() {
            throw new Error('trap');
        },
        has() {
            throw new Error('trap');
        },
        ownKeys() {
            throw new Error('trap');
        }
    });
    fails('throwing proxy', proxy);
    const counted = bag();
    let reads = 0;
    Object.defineProperty(counted, 'name', {
        enumerable: true,
        get() {
            reads += 1;
            return reads === 1 ? 'n' : 5;
        }
    });
    const result = check('changing getter', () => mod.hostileBagDecode(counted));
    if (result.success) {
        assertStrictEquals(
            typeof result.value.name,
            'string',
            'a getter read twice slipped a number in'
        );
    }
});

Deno.test('references are checked', () => {
    fails('root ref', { __ref: 1 });
    fails('dangling ref', bag({ nested: { __ref: 99 } }));
    fails('text ref', bag({ nested: { __ref: 'x' } }));
    fails(
        'ref to another type',
        bag({ __id: 1, item: { __id: 2, id: 'i', score: 1 }, nested: { __ref: 2 } })
    );
    const self = passes('self ref', bag({ __id: 1, nested: { __ref: 1 } }));
    assertStrictEquals(self.nested, self);
    const twice = check(
        'same id twice',
        () => mod.hostileBagDecode(bag({ __id: 1, nested: bag({ __id: 1 }) }))
    );
    if (twice.success) {
        assert(twice.value.nested !== twice.value, 'two values under one id became one');
    }
    fails('text id', bag({ __id: 'x', nested: { __ref: 'x' } }));
});

Deno.test('depth beyond the stack fails without throwing', () => {
    let deep = bag();
    for (let level = 0; level < 20000; level++) {
        deep = bag({ nested: deep });
    }
    fails('deep nesting', deep);
    let text = '';
    for (let level = 0; level < 20000; level++) text += '{"nested":';
    fails('deep text', text);
});

Deno.test('a class takes no member from its input but its fields', () => {
    const input = JSON.parse(
        '{"name": "n", "count": 1, "label": "evil", "constructor": 5, "__proto__": {"label": 6}, "toString": 7}'
    );
    const result = check('class members', () => mod.HostileClass.decode(input));
    assert(result.success, JSON.stringify(result.errors));
    assert(result.value instanceof mod.HostileClass);
    assertStrictEquals(result.value.label(), 'n:1');
    assertStrictEquals(result.value.constructor, mod.HostileClass);
    assertEquals(Object.keys(result.value).sort(), ['count', 'name']);
});

Deno.test('unknown keys are denied when asked, the keys of the encoding excepted', () => {
    assert(mod.hostileStrictDecode({ name: 'n' }).success);
    assert(mod.hostileStrictDecode(mod.hostileStrictEncode({ name: 'n' }, true)).success);
    assert(!mod.hostileStrictDecode({ name: 'n', extra: 1 }).success);
    assert(!mod.hostileStrictDecode(JSON.parse('{"name": "n", "__proto__": 1}')).success);
    assert(!mod.hostileStrictDecode(JSON.parse('{"name": "n", "constructor": 1}')).success);
});

Deno.test('freeze freezes everything decoded', () => {
    const result = mod.hostileBagDecode(bag({ nested: bag() }), { freeze: true });
    assert(result.success, JSON.stringify(result.errors));
    const value = result.value;
    for (
        const [label, part] of Object.entries({
            value,
            item: value.item,
            items: value.items,
            first: value.items[0],
            byName: value.byName,
            counts: value.counts,
            pair: value.pair,
            nested: value.nested
        })
    ) {
        assert(Object.isFrozen(part), `${label} is not frozen`);
    }
});
