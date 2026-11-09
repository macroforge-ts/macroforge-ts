/**
 * Type text a derive reads by scanning it: a literal ending in an escaped
 * backslash, a function returning a union, a template literal alias, and an
 * empty pattern, which every text matches.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'scanned-text.ts'));

Deno.test('a backslash literal is one member of its union', () => {
    for (const separator of ['\\', '/']) {
        const decoded = mod.scannedTextFromString(
            JSON.stringify({ separator, key: 'key-a', note: '' })
        );
        assert(decoded.success, JSON.stringify(decoded));
        assertStrictEquals(decoded.value.separator, separator);
    }
    assert(
        !mod.scannedTextFromString(JSON.stringify({ separator: '|', key: 'key-a', note: '' }))
            .success
    );
    assert(mod.scannedTextIs({ separator: '\\', key: 'key-b', note: '' }));
});

Deno.test('a template literal alias is checked with its argument in place', () => {
    assert(
        mod.scannedTextFromString(
            JSON.stringify({ separator: '/', key: 'key-b', note: 'any text' })
        )
            .success
    );
    assert(
        !mod.scannedTextFromString(JSON.stringify({ separator: '/', key: 'key-c', note: '' }))
            .success
    );
    assert(!mod.scannedTextIs({ separator: '/', key: 'key-T', note: '' }));
});

Deno.test('a function returning a union is held and compared as one function', () => {
    const handlers = { pick: () => 1, when: (value) => value === 'yes' };
    const copy = mod.scannedHandlersClone(handlers);
    assertNotStrictEquals(copy, handlers);
    assertStrictEquals(copy.pick, handlers.pick);
    assert(mod.scannedHandlersEquals(handlers, copy));
    assert(!mod.scannedHandlersEquals(handlers, { ...handlers, pick: () => 'one' }));
    assertEquals(mod.scannedTextClone({ separator: '\\', key: 'key-a', note: 'n' }), {
        separator: '\\',
        key: 'key-a',
        note: 'n'
    });
});

Deno.test('an empty pattern takes any text', () => {
    for (const note of ['', 'text', '//', '\n']) {
        assert(
            mod.scannedTextFromString(JSON.stringify({ separator: '/', key: 'key-a', note }))
                .success,
            note
        );
    }
});

Deno.test('a type in parentheses or holding brackets in a string is read as written', () => {
    const groups = mod.scannedGroupsDefaultValue();
    assertEquals(groups, { shade: 'light', either: '', pair: [']?)'] });
    const decoded = mod.scannedGroupsFromString(
        JSON.stringify({ shade: 'dark', either: { a: '' }, pair: [']?)', 4] })
    );
    assert(decoded.success, JSON.stringify(decoded));
    assertEquals(decoded.value, { shade: 'dark', either: { a: '' }, pair: [']?)', 4] });
    assert(
        !mod.scannedGroupsFromString(JSON.stringify({ shade: 'dark', either: '', pair: [']'] }))
            .success
    );
    assert(mod.scannedGroupsEquals(groups, mod.scannedGroupsDefaultValue()));
});

Deno.test('a union of a primitive and a tuple is coded as its values are', () => {
    for (
        const value of [
            { pair: 'text', rest: 'r' },
            { pair: [1, 'one'], rest: ['a', 1, 2] },
            { pair: [2], rest: ['a'] }
        ]
    ) {
        const decoded = mod.scannedUnionsFromString(mod.scannedUnionsEncode(value));
        assert(decoded.success, JSON.stringify(decoded));
        assert(
            mod.scannedUnionsEquals(decoded.value, value),
            mod.scannedUnionsToString(decoded.value)
        );
    }
    for (const pair of [[1, 2], ['one'], [1, 'one', 'two']]) {
        assert(
            !mod.scannedUnionsFromString(JSON.stringify({ pair, rest: 'r' })).success,
            JSON.stringify(pair)
        );
    }
});

Deno.test('a bigint literal is coded as its bigint, and checked to be the literal', () => {
    for (
        const value of [{ one: 1n, some: -1n, maybe: null }, { one: 1n, some: 1000n, maybe: 2n }]
    ) {
        const encoded = mod.scannedBigintsEncode(value);
        const decoded = mod.scannedBigintsFromString(encoded);
        assert(decoded.success, decoded.success ? '' : JSON.stringify(decoded.errors));
        assert(mod.scannedBigintsEquals(decoded.value, value), encoded);
    }
    assert(
        !mod.scannedBigintsFromString(JSON.stringify({ one: '2', some: '-1', maybe: null })).success
    );
    assert(
        !mod.scannedBigintsFromString(JSON.stringify({ one: '1', some: '5', maybe: null })).success
    );
});

Deno.test('a field of nothing but undefined takes no value', () => {
    assert(mod.scannedNothingFromString(JSON.stringify({ nothing: null, other: 'x' })).success);
    assert(
        !mod.scannedNothingFromString(JSON.stringify({ none: 1, nothing: null, other: 'x' }))
            .success
    );
});

Deno.test('an array of functions is copied as one, and a union defaults to its object', () => {
    const pick = (value) => value;
    const picks = { pick: [pick], picked: ['a'], listed: [{ 'key with space': 'k' }] };
    const copy = mod.scannedPicksClone(picks);
    assertStrictEquals(copy.pick[0], pick);
    assert(mod.scannedPicksEquals(picks, copy));
    const defaults = mod.scannedPicksDefaultValue();
    assertEquals(defaults.pick, null);
    assertEquals(defaults.picked, { 'key with space': undefined });
    assertEquals(defaults.listed, []);
    assert('key with space' in defaults.picked);
});

Deno.test('an intersection ending in an array is cloned and compared as it is written', () => {
    const value = { tagged: Object.assign([1n], { a: 'tag' }) };
    const copy = mod.scannedTaggedClone(value);
    assert(mod.scannedTaggedEquals(value, copy));
});
