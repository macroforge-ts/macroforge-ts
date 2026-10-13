/**
 * Directives whose arguments hold what a scan for brackets, commas and
 * option names could trip on, derives spread over several directives, and
 * fields a decoder never reads.
 */

import { assert, assertEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'directive-shapes.ts'));
const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};
const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

Deno.test('a bracket, a comma or an option name in a string is text', () => {
    const texts = { renamed: 'r', open: 'o', listed: 'l', matched: 'a, (b', words: ['w'] };
    const encoded = JSON.parse(mod.directiveTextsEncode(texts));
    // A field renamed to `skip` is written under that key, not skipped.
    assertEquals(encoded, { skip: 'r', open: 'o', listed: 'l', matched: 'a, (b', words: ['w'] });
    assertEquals(decoded(mod.directiveTextsDecode(encoded)), texts);

    const lacking = decoded(mod.directiveTextsDecode({ skip: 'r', matched: 'ab' }));
    assertEquals(lacking, {
        renamed: 'r',
        open: '(',
        listed: 'a, b]',
        matched: 'ab',
        words: ['skip', 'flatten']
    });
    assertEquals(errorFields(mod.directiveTextsDecode({ skip: 'r', matched: 'A1' })), ['matched']);
    assertEquals(errorFields(mod.directiveTextsDecode({ renamed: 'r', matched: 'ab' })), ['skip']);

    const blank = mod.directiveTextsDefaultValue();
    assertStrictEquals(blank.open, ':(');
    assertStrictEquals(blank.listed, 'a, b]');
    assert(mod.directiveTextsToString(texts).includes('flatten): r'));
});

Deno.test('every @derive directive of a declaration is expanded', () => {
    const stacked = { name: 'n' };
    assert(mod.directiveStackedToString(stacked).includes('name: n'));
    assert(mod.directiveStackedEquals(stacked, mod.directiveStackedClone(stacked)));

    const repeated = { x: 1 };
    assert(mod.directiveRepeatedToString(repeated).includes('x: 1'));
    assert(mod.directiveRepeatedEquals(repeated, mod.directiveRepeatedClone(repeated)));
    assert(!mod.directiveRepeatedEquals(repeated, { x: 2 }));
});

Deno.test('a field Decode never reads takes its default', () => {
    const whole = {
        name: 'n',
        secret: 's',
        count: 3,
        tags: ['t'],
        note: 'kept',
        optional: 'o',
        maybe: 'm'
    };
    const encoded = JSON.parse(mod.directiveSkippedEncode(whole));
    assertEquals(encoded, { name: 'n', count: 3, maybe: 'm' });

    // What the input says of a skipped field is not read either.
    const back = decoded(
        mod.directiveSkippedDecode({ ...encoded, secret: 'given', tags: 'wrong' })
    );
    assertEquals(back, { name: 'n', secret: '', count: 0, tags: [], note: 'none', maybe: null });
    assert(!('optional' in back));

    const instance = decoded(
        mod.DirectiveSkippedClass.decode({ name: 'n', derived: 9, cache: 'wrong' })
    );
    assert(instance instanceof mod.DirectiveSkippedClass);
    assertStrictEquals(instance.derived, 0);
    assert(instance.cache instanceof Map);
    assertStrictEquals(instance.cache.size, 0);
});

Deno.test('a flattened type that may be null is null when its fields are not there', () => {
    const present = { title: 't', inner: { depth: 2 } };
    const encoded = JSON.parse(mod.directiveFlatEncode(present));
    assertEquals(encoded, { title: 't', depth: 2 });
    assertEquals(decoded(mod.directiveFlatDecode(encoded)), present);

    const absent = { title: 't', inner: null };
    assertEquals(JSON.parse(mod.directiveFlatEncode(absent)), { title: 't' });
    assertEquals(decoded(mod.directiveFlatDecode({ title: 't' })), absent);
});
