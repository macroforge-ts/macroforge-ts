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
        const decoded = mod.scannedTextDecode(
            JSON.stringify({ separator, key: 'key-a', note: '' })
        );
        assert(decoded.success, JSON.stringify(decoded));
        assertStrictEquals(decoded.value.separator, separator);
    }
    assert(
        !mod.scannedTextDecode(JSON.stringify({ separator: '|', key: 'key-a', note: '' })).success
    );
    assert(mod.scannedTextIs({ separator: '\\', key: 'key-b', note: '' }));
});

Deno.test('a template literal alias is checked with its argument in place', () => {
    assert(
        mod.scannedTextDecode(JSON.stringify({ separator: '/', key: 'key-b', note: 'any text' }))
            .success
    );
    assert(
        !mod.scannedTextDecode(JSON.stringify({ separator: '/', key: 'key-c', note: '' })).success
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
            mod.scannedTextDecode(JSON.stringify({ separator: '/', key: 'key-a', note })).success,
            note
        );
    }
});
