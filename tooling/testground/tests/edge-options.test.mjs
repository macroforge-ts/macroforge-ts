/**
 * Every option a derive's decorator takes, alone and in the combinations
 * that interact.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'option-shapes.ts'));

const item = (id, score) => ({ id, score });
const JOINED = '2024-01-02T03:04:05.000Z';

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const traits = () => ({ id: 'a', label: 'shown', secret: 'hidden', cached: 1, unordered: 'x' });

Deno.test('skip and rename shape what each value trait reads', () => {
    const text = mod.optionTraitsToString(traits());
    assert(text.includes('shown: shown'), text);
    assert(!text.includes('label:'), text);
    assert(!text.includes('hidden'), text);

    const recached = { ...traits(), cached: 2 };
    assert(mod.optionTraitsEquals(traits(), recached));
    assertStrictEquals(mod.optionTraitsHashCode(traits()), mod.optionTraitsHashCode(recached));
    assertStrictEquals(mod.optionTraitsCompare(traits(), recached), 0);

    const reordered = { ...traits(), unordered: 'y' };
    assert(!mod.optionTraitsEquals(traits(), reordered));
    assertStrictEquals(mod.optionTraitsCompare(traits(), reordered), 0);
    assertStrictEquals(mod.optionTraitsCompare(traits(), { ...traits(), secret: 'z' }), -1);
});

Deno.test('a field defaults to its @default, else its initializer, else its type', () => {
    const first = mod.optionDefaultsDefaultValue();
    assertEquals(first, {
        theme: 'light',
        pageSize: 10,
        enabled: true,
        tags: ['a', 'b'],
        item: item('seed', 1),
        maybe: null,
        at: new Date(0),
        total: 10n,
        plain: 0
    });
    assertNotStrictEquals(mod.optionDefaultsDefaultValue().tags, first.tags);
    assertNotStrictEquals(mod.optionDefaultsDefaultValue().item, first.item);

    const blank = mod.OptionDefaultsClass.defaultValue();
    assert(blank instanceof mod.OptionDefaultsClass);
    assertStrictEquals(blank.theme, 'light');
    assertStrictEquals(blank.pageSize, 25);
    assertEquals(blank.counts, new Map([['a', 1]]));
    assert(!('label' in blank));
});

const fields = () => ({
    name: 'Ada',
    skipped: 'never coded',
    writeOnly: 'decoded only',
    readOnly: 'encoded only',
    count: 1,
    status: 'ok',
    tags: ['t'],
    item: item('i', 1),
    balance: 12.5,
    optionalBalance: 3.25,
    shout: 'loud',
    optionalShout: 'hey'
});

Deno.test('field options decide what is written and how', () => {
    assertEquals(JSON.parse(mod.optionFieldsEncode(fields())), {
        user_name: 'Ada',
        readOnly: 'encoded only',
        count: 1,
        status: 'ok',
        tags: ['t'],
        item: item('i', 1),
        balance: '12.5',
        optionalBalance: '3.25',
        shout: 'LOUD',
        optionalShout: 'HEY'
    });

    const { optionalBalance: _balance, optionalShout: _shout, ...required } = fields();
    const sparse = JSON.parse(mod.optionFieldsEncode(required));
    assert(!('optionalBalance' in sparse));
    assert(!('optionalShout' in sparse));
});

Deno.test('field options decide what is read, and what stands in for a missing key', () => {
    const value = decoded(
        mod.optionFieldsDecode({
            user_name: 'Ada',
            skipped: 'ignored',
            writeOnly: 'decoded only',
            readOnly: 'ignored',
            balance: '12.5',
            shout: 'LOUD'
        })
    );
    assertEquals(value, {
        name: 'Ada',
        writeOnly: 'decoded only',
        readOnly: 'computed',
        count: 0,
        status: 'pending',
        tags: [],
        item: item('', 0),
        balance: 12.5,
        shout: 'loud'
    });

    const full = decoded(
        mod.optionFieldsDecode({ ...JSON.parse(mod.optionFieldsEncode(fields())), writeOnly: 'w' })
    );
    assertStrictEquals(full.optionalBalance, 3.25);
    assertStrictEquals(full.optionalShout, 'hey');
    assertStrictEquals(full.count, 1);
    assertEquals(full.tags, ['t']);
});

Deno.test('a missing or malformed field is reported under its JSON key', () => {
    const wire = { user_name: 'Ada', writeOnly: 'w', balance: '12.5', shout: 'LOUD' };
    const fieldsOf = (change) => errorFields(mod.optionFieldsDecode({ ...wire, ...change }));

    assertEquals(fieldsOf({ user_name: undefined }), ['user_name']);
    assertEquals(fieldsOf({ balance: 'not a number' }), ['balance']);
    assertEquals(fieldsOf({ optionalBalance: 'not a number' }), ['optionalBalance']);
    // A decimal is a number or the text of one: nothing `Number()` merely coerces.
    for (const bad of ['', '  ', true, null, [], {}]) {
        assertEquals(fieldsOf({ balance: bad }), ['balance'], JSON.stringify(bad));
    }
    const numeric = mod.optionFieldsDecode({ ...wire, balance: 7, optionalBalance: ' 2.50 ' });
    assert(numeric.success, JSON.stringify(numeric.errors));
    assertStrictEquals(numeric.value.balance, 7);
    assertStrictEquals(numeric.value.optionalBalance, 2.5);
    assertEquals(fieldsOf({ count: 'one' }), ['count']);
    assertEquals(fieldsOf({ tags: 'none' }), ['tags']);
    assertEquals(fieldsOf({ item: { id: 'i' } }), ['item.score']);
});

Deno.test('renameAll spells every key its way, and rename wins over it', () => {
    const screaming = JSON.parse(
        mod.optionScreamingEncode({ firstName: 'A', lastName: 'B', homeAddress2: 'C' })
    );
    assertStrictEquals(screaming.FIRST_NAME, 'A');
    assertStrictEquals(screaming.kept, 'B');
    assertEquals(Object.values(screaming), ['A', 'B', 'C']);
    assertEquals(decoded(mod.optionScreamingDecode(screaming)), {
        firstName: 'A',
        lastName: 'B',
        homeAddress2: 'C'
    });

    const kebab = { firstName: 'A', isActive: true, items: [item('i', 1)] };
    assertEquals(JSON.parse(mod.optionKebabEncode(kebab)), {
        'first-name': 'A',
        'is-active': true,
        items: [item('i', 1)]
    });
    assertEquals(decoded(mod.optionKebabDecode(mod.optionKebabEncode(kebab))), kebab);
    assertEquals(errorFields(mod.optionKebabDecode({ 'first-name': 'A', items: [] })), [
        'is-active'
    ]);

    const pascal = decoded(mod.OptionPascal.decode({ FirstName: 'A', HomeTown: null }));
    assert(pascal instanceof mod.OptionPascal);
    assertEquals(JSON.parse(mod.OptionPascal.encode(pascal)), { FirstName: 'A', HomeTown: null });

    const camel = { first_name: 'A', home_town: 'B' };
    assertEquals(JSON.parse(mod.optionCamelEncode(camel)), { firstName: 'A', homeTown: 'B' });
    assertEquals(decoded(mod.optionCamelDecode(mod.optionCamelEncode(camel))), camel);
});

const flattened = () => ({
    title: 't',
    item: item('i', 1),
    profile: {
        displayName: 'Ada',
        email: 'ada@example.com',
        joinedAt: new Date(JOINED),
        tags: ['x'],
        level: 2
    }
});

Deno.test("a flattened field's members are written beside the others, under their own keys", () => {
    const encoded = JSON.parse(mod.optionFlattenedEncode(flattened()));
    assertEquals(encoded, {
        title: 't',
        id: 'i',
        score: 1,
        display_name: 'Ada',
        mail: 'ada@example.com',
        joined_at: JOINED,
        tags: ['x'],
        level: 2
    });
    assertEquals(decoded(mod.optionFlattenedDecode(encoded)), flattened());
    assertEquals(errorFields(mod.optionFlattenedDecode({ ...encoded, mail: 'not mail' })), [
        'mail'
    ]);

    const { score: _score, ...withoutScore } = encoded;
    assertEquals(errorFields(mod.optionFlattenedDecode(withoutScore)), ['score']);
});

Deno.test('an optional flattened field is left out when absent, and a record takes its members', () => {
    const withItem = { title: 't', item: item('i', 1), extra: {} };
    assertEquals(JSON.parse(mod.optionFlattenedOptionalEncode(withItem)), {
        title: 't',
        id: 'i',
        score: 1
    });

    const withExtra = { title: 't', extra: { a: 1, b: 2 } };
    assertEquals(JSON.parse(mod.optionFlattenedOptionalEncode(withExtra)), {
        title: 't',
        a: 1,
        b: 2
    });

    const value = decoded(mod.optionFlattenedOptionalDecode({ title: 't', id: 'i', score: 1 }));
    assertStrictEquals(value.title, 't');
    assertEquals(value.item, item('i', 1));
});

Deno.test('denyUnknownFields rejects a key no field reads, and takes the metadata keys', () => {
    const strict = { firstName: 'A', item: item('i', 1), nested: item('n', 2) };
    const encoded = JSON.parse(mod.optionStrictEncode(strict));
    assertEquals(encoded, { first_name: 'A', item: item('i', 1), nested: item('n', 2) });
    assertEquals(decoded(mod.optionStrictDecode(encoded)), strict);
    assertEquals(decoded(mod.optionStrictDecode(mod.optionStrictEncode(strict, true))), strict);

    assertEquals(errorFields(mod.optionStrictDecode({ ...encoded, bogus: 1 })), ['bogus']);
    assertEquals(errorFields(mod.optionStrictDecode({ ...encoded, firstName: 'A' })), [
        'firstName'
    ]);
    // The nested type does not deny unknown fields, so it takes them.
    assertEquals(
        decoded(mod.optionStrictDecode({ ...encoded, item: { ...item('i', 1), stray: true } }))
            .item,
        item('i', 1)
    );
});

Deno.test("a tag names the field a value's type is written under", () => {
    const parent = { name: 'parent', child: { name: 'child', child: null } };
    const tagged = JSON.parse(mod.optionTaggedEncode(parent, true));
    assertStrictEquals(tagged.kind, 'OptionTagged');
    assertStrictEquals(tagged.child.kind, 'OptionTagged');
    assert(!('__type' in tagged));
    assertEquals(JSON.parse(mod.optionTaggedEncode(parent)), parent);
    assertEquals(decoded(mod.optionTaggedDecode(tagged)), parent);

    const loop = { name: 'loop', child: null };
    loop.child = loop;
    const back = decoded(mod.optionTaggedDecode(mod.optionTaggedEncode(loop, true)));
    assertStrictEquals(back.child, back);
});

const validated = () => ({
    email: 'ada@example.com',
    optionalName: 'Ada',
    nullableCount: 2,
    laterCount: 3,
    tags: ['t'],
    at: JOINED,
    maybeAt: JOINED,
    retries: 5
});

Deno.test('validators run on a value that is there, and not on one that may be absent', () => {
    const value = decoded(mod.optionValidatedDecode(validated()));
    assert(value.at instanceof Date);
    assert(value.maybeAt instanceof Date);
    assertStrictEquals(value.retries, 5);

    const { optionalName: _name, laterCount: _later, retries: _retries, ...required } = validated();
    const sparse = decoded(
        mod.optionValidatedDecode({ ...required, nullableCount: null, maybeAt: null })
    );
    assertStrictEquals(sparse.nullableCount, null);
    assertStrictEquals(sparse.maybeAt, null);
    assertStrictEquals(sparse.laterCount, undefined);
    assertStrictEquals(sparse.retries, 3);
    assert(!('optionalName' in sparse));
});

Deno.test('a value that fails its validator is reported under its field', () => {
    const fieldsOf = (change) =>
        errorFields(mod.optionValidatedDecode({ ...validated(), ...change }));

    assertEquals(fieldsOf({ email: 'not mail' }), ['email']);
    assertEquals(fieldsOf({ optionalName: '' }), ['optionalName']);
    assertEquals(fieldsOf({ nullableCount: -1 }), ['nullableCount']);
    assertEquals(fieldsOf({ laterCount: 1.5 }), ['laterCount']);
    assertEquals(fieldsOf({ tags: [] }), ['tags']);
    assertEquals(fieldsOf({ at: 'not a date' }), ['at']);
    assertEquals(fieldsOf({ maybeAt: 'not a date' }), ['maybeAt']);
    // A nullable date takes `null`, a date or the text of one, and no other value.
    assertEquals(fieldsOf({ maybeAt: 5 }), ['maybeAt']);
    assertEquals(fieldsOf({ maybeAt: { year: 2024 } }), ['maybeAt']);
    assertEquals(fieldsOf({ at: null }), ['at']);
    assertEquals(fieldsOf({ at: true }), ['at']);
    assertEquals(fieldsOf({ retries: -1 }), ['retries']);
    assertEquals(fieldsOf({ email: 'not mail', tags: [], retries: 0 }).sort(), [
        'email',
        'retries',
        'tags'
    ]);
});

Deno.test('every kind of field refuses a value that is not of its type', () => {
    const valid = {
        text: 't',
        count: 1,
        flag: true,
        big: '12',
        at: '2024-01-02T03:04:05.000Z',
        item: { id: 'i', score: 1 },
        size: 'small',
        list: ['a'],
        set: [1],
        map: { a: 1 },
        record: { a: true },
        pair: ['a', 1],
        inline: { x: 1 },
        maybeText: null,
        partial: {},
        link: 'https://example.com/path',
        query: 'a=1&b=2',
        pattern: { source: 'a+', flags: 'g' },
        failure: { name: 'RangeError', message: 'out of range' },
        bytes: [1, 2, 255],
        bigs: ['1', '-2'],
        buffer: [3, 4]
    };
    const whole = mod.optionKindsDecode(valid);
    assert(whole.success, JSON.stringify(whole.errors));
    assert(whole.value.link instanceof URL);
    assertStrictEquals(whole.value.query.get('b'), '2');
    assertStrictEquals(whole.value.pattern.flags, 'g');
    assertStrictEquals(whole.value.failure.name, 'RangeError');
    assert(whole.value.bytes instanceof Uint8Array);
    assertStrictEquals(whole.value.bigs[1], -2n);
    assertStrictEquals(new Uint8Array(whole.value.buffer)[1], 4);
    assertStrictEquals(whole.value.big, 12n);
    // What was decoded encodes back to what it was decoded from.
    const again = JSON.parse(mod.optionKindsEncode(whole.value));
    for (const field of ['big', 'link', 'query', 'pattern', 'bytes', 'bigs', 'buffer']) {
        assertEquals(again[field], valid[field], field);
    }

    const wrong = {
        text: [null, 5, true, [], {}],
        count: [null, 'x', true, [], {}],
        flag: [null, 5, 'x', [], {}],
        big: [null, 'x', true, [], {}, 1.5],
        at: [null, 5, 'x', true, [], {}],
        item: [null, 5, 'x', true, [], {}],
        size: [null, 5, 'x', true, [], {}],
        list: [null, 5, 'x', true, {}, [1]],
        set: [null, 5, 'x', true, {}, ['a']],
        map: [null, 5, 'x', true, [], { a: 'x' }],
        record: [null, 5, 'x', true, [], { a: 1 }],
        pair: [null, 5, 'x', true, {}, [], ['a'], ['a', 1, 2], [1, 'a']],
        inline: [null, 5, 'x', true, [], {}, { x: '1' }],
        maybeText: [5, true, [], {}],
        laterText: [null, 5, true, [], {}],
        partial: [null, 5, 'x', true, [], { id: 5 }],
        link: [null, 5, true, [], {}, 'not a url'],
        query: [null, 5, true, [], {}],
        pattern: [null, 5, 'x', true, [], {}, { source: 5 }],
        failure: [null, 5, 'x', true, [], {}, { message: 5 }],
        bytes: [null, 5, 'x', true, {}, ['a']],
        bigs: [null, 5, 'x', true, {}, [1], ['x'], ['1.5']],
        buffer: [null, 5, 'x', true, {}, ['a']]
    };
    const accepted = [];
    for (const [field, values] of Object.entries(wrong)) {
        for (const value of values) {
            const result = mod.optionKindsDecode({ ...valid, [field]: value });
            const label = `${field}: ${JSON.stringify(value)}`;
            if (result.success) {
                accepted.push(label);
                continue;
            }
            for (const error of result.errors) {
                assert(
                    error.field === field || error.field.startsWith(field),
                    `${label} reported under ${error.field}`
                );
            }
        }
    }
    assertEquals(accepted, []);
});

Deno.test('a default holds what each kind of field has to have', () => {
    const value = mod.optionDefaultKindsDefaultValue();
    assertEquals(value.required, { id: '', score: 0 });
    assertEquals(value.partial, {});
    assertEquals(value.picked, { id: '' });
    assertEquals(value.dictionary, {});
    assertEquals(value.inline, { x: 0 });
    assertStrictEquals(value.carried, '');
    assertStrictEquals(value.dated, '');
    assertStrictEquals(value.size, 'small');
    assertStrictEquals(value.linked, '');
    assertStrictEquals(value.big, 0n);
    assert(value.bytes instanceof Uint8Array);
    assertStrictEquals(value.bytes.length, 0);
    assertEquals(value.pair, ['', 0]);
    assert(value.nested instanceof Map);
    assertStrictEquals(value.nested.size, 0);
    // Each default is a value of its own.
    assertNotStrictEquals(mod.optionDefaultKindsDefaultValue().inline, value.inline);
    assertNotStrictEquals(mod.optionDefaultKindsDefaultValue().picked, value.picked);
});

Deno.test('a class refuses a value of the wrong type in each kind of field, and defaults each', () => {
    const valid = {
        text: 't',
        count: 1,
        at: '2024-01-02T03:04:05.000Z',
        big: '12',
        item: { id: 'i', score: 1 },
        list: ['a'],
        map: { a: 1 },
        pair: ['a', 1],
        inline: { x: 1 },
        picked: { id: 'p' },
        required: { id: 'r', score: 2 },
        maybe: null
    };
    const whole = mod.OptionKindsClass.decode(valid);
    assert(whole.success, JSON.stringify(whole.errors));
    assert(whole.value instanceof mod.OptionKindsClass);
    assert(whole.value.at instanceof Date);
    assertEquals(JSON.parse(mod.OptionKindsClass.encode(whole.value)), valid);

    const wrong = {
        text: [null, 5, true, [], {}],
        count: [null, 'x', true, [], {}],
        at: [null, 5, 'x', true, [], {}],
        big: [null, 'x', true, [], {}],
        item: [null, 5, 'x', true, [], {}],
        list: [null, 5, 'x', true, {}, [1]],
        map: [null, 5, 'x', true, [], { a: 'x' }],
        pair: [null, 5, 'x', true, {}, [], ['a'], [1, 'a']],
        inline: [null, 5, 'x', true, [], {}, { x: '1' }],
        picked: [null, 5, 'x', true, [], {}, { id: 5 }],
        required: [null, 5, 'x', true, [], {}, { id: 'r' }],
        maybe: [5, true, [], {}],
        later: [null, 5, true, [], {}]
    };
    const accepted = [];
    for (const [field, values] of Object.entries(wrong)) {
        for (const value of values) {
            const result = mod.OptionKindsClass.decode({ ...valid, [field]: value });
            const label = `${field}: ${JSON.stringify(value)}`;
            if (result.success) accepted.push(label);
            else {
                for (const error of result.errors) {
                    assert(error.field.startsWith(field), `${label} reported under ${error.field}`);
                }
            }
        }
    }
    assertEquals(accepted, []);

    const blank = mod.OptionKindsClass.defaultValue();
    assert(blank instanceof mod.OptionKindsClass);
    assertStrictEquals(blank.text, '');
    assertStrictEquals(blank.count, 0);
    assert(blank.at instanceof Date);
    assertStrictEquals(blank.big, 0n);
    assertEquals(blank.item, { id: '', score: 0 });
    assertEquals(blank.list, []);
    assert(blank.map instanceof Map);
    assertEquals(blank.pair, ['', 0]);
    assertEquals(blank.inline, { x: 0 });
    assertEquals(blank.picked, { id: '' });
    assertEquals(blank.required, { id: '', score: 0 });
    assertStrictEquals(blank.maybe, null);
    assert(!('later' in blank));
});
