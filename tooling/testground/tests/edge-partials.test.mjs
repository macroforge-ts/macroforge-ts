/**
 * `Partial`, `Pick` and `Omit` of types from another module, in every place
 * a field can hold one, and over types with renamed, validated, defaulted and
 * flattened fields.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const edgeCases = join(vanillaRoot, 'src', 'edge-cases');
const mod = await expandAndCompile(join(edgeCases, 'partial-shapes.ts'));
const types = await expandAndCompile(join(edgeCases, 'edge-types.ts'));

const item = (id, score) => ({ id, score });

const partials = () => ({
    patch: { id: 'p' },
    patches: [{ id: 'a' }, { score: 2 }, {}],
    patchList: [{ id: 'l' }],
    readonlyPatches: [{ id: 'r' }],
    patchSet: new Set([{ score: 1 }]),
    byName: new Map([['k', { id: 'm' }]]),
    record: { k: { id: 'rec' } },
    maybe: { score: 3 },
    later: { id: 'later' },
    optional: { score: 4 },
    nullablePatches: [{ id: 'n' }, null],
    card: { title: 't' },
    cards: [{ title: 'c' }]
});

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

Deno.test('a Partial, Pick or Omit in a collection encodes the fields it has', () => {
    assertEquals(JSON.parse(mod.edgePartialsEncode(partials())), {
        patch: { id: 'p' },
        patches: [{ id: 'a' }, { score: 2 }, {}],
        patchList: [{ id: 'l' }],
        readonlyPatches: [{ id: 'r' }],
        patchSet: [{ score: 1 }],
        byName: { k: { id: 'm' } },
        record: { k: { id: 'rec' } },
        maybe: { score: 3 },
        later: { id: 'later' },
        optional: { score: 4 },
        nullablePatches: [{ id: 'n' }, null],
        card: { title: 't' },
        cards: [{ title: 'c' }]
    });
});

Deno.test('a Pick or Omit in a collection leaves out the fields its type does', () => {
    const whole = item('w', 9);
    const card = new types.EdgeCard({ title: 'whole', items: [whole] });
    const encoded = JSON.parse(
        mod.edgePartialsEncode({
            ...partials(),
            patchList: [whole],
            readonlyPatches: [whole],
            byName: new Map([['k', whole]]),
            record: { k: whole },
            later: whole,
            optional: whole,
            cards: [card]
        })
    );
    assertEquals(encoded.patchList, [{ id: 'w' }]);
    assertEquals(encoded.readonlyPatches, [{ id: 'w' }]);
    assertEquals(encoded.byName, { k: { id: 'w' } });
    assertEquals(encoded.record, { k: { id: 'w' } });
    assertEquals(encoded.later, { id: 'w' });
    assertEquals(encoded.optional, { score: 9 });
    assertEquals(encoded.cards, [{ title: 'whole' }]);
});

Deno.test('a Partial, Pick or Omit in a collection round-trips', () => {
    const result = mod.edgePartialsDecode(mod.edgePartialsEncode(partials()));
    assert(result.success, JSON.stringify(result.errors));
    // A partial class value is an instance holding the fields it was given.
    const { card, cards, ...plain } = result.value;
    const { card: _card, cards: _cards, ...expected } = partials();
    assertEquals(plain, expected);
    assert(plain.patchSet instanceof Set);
    assert(plain.byName instanceof Map);
    assert(card instanceof types.EdgeCard);
    assert(cards[0] instanceof types.EdgeCard);
    assertEquals({ ...card }, { title: 't' });
    assertEquals({ ...cards[0] }, { title: 'c' });
});

Deno.test('absent and null values of a Partial, Pick or Omit round-trip', () => {
    const { optional: _optional, ...rest } = partials();
    const sparse = { ...rest, maybe: null, later: undefined, nullablePatches: [null] };
    const encoded = JSON.parse(mod.edgePartialsEncode(sparse));
    assertStrictEquals(encoded.maybe, null);
    assert(!('later' in encoded));
    assert(!('optional' in encoded));

    const result = mod.edgePartialsDecode(encoded);
    assert(result.success, JSON.stringify(result.errors));
    assertStrictEquals(result.value.maybe, null);
    assertStrictEquals(result.value.later, undefined);
    assert(!('optional' in result.value));
    assertEquals(result.value.nullablePatches, [null]);
});

Deno.test('an element is decoded with the fields its type keeps, and no other', () => {
    const wire = { id: 'w', score: 9, stray: true };
    const result = mod.edgePartialsDecode({
        ...JSON.parse(mod.edgePartialsEncode(partials())),
        patchList: [wire],
        readonlyPatches: [wire],
        byName: { k: wire },
        record: { k: wire }
    });
    assert(result.success, JSON.stringify(result.errors));
    assertEquals(result.value.patchList, [{ id: 'w' }]);
    assertEquals(result.value.readonlyPatches, [{ id: 'w' }]);
    assertEquals(result.value.byName.get('k'), { id: 'w' });
    assertEquals(result.value.record.k, { id: 'w' });
});

Deno.test('an element is required to have the fields its type keeps', () => {
    const encoded = JSON.parse(mod.edgePartialsEncode(partials()));
    const fieldsOf = (change) => errorFields(mod.edgePartialsDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ patchList: [{ id: 'ok' }, {}] }), ['patchList[1].id']);
    assertEquals(fieldsOf({ readonlyPatches: [{ score: 1 }] }), ['readonlyPatches[0].id']);
    assertEquals(fieldsOf({ byName: { k: { score: 1 } } }), ['byName.k.id']);
    assertEquals(fieldsOf({ record: { k: {} } }), ['record.k.id']);
    assertEquals(fieldsOf({ cards: [{ items: [] }] }), ['cards[0].title']);
    assertEquals(fieldsOf({ later: {} }), ['later.id']);
    assertEquals(fieldsOf({ optional: { id: 'x' } }), ['optional.score']);
});

Deno.test('a field an element has is checked against its type', () => {
    const encoded = JSON.parse(mod.edgePartialsEncode(partials()));
    const fieldsOf = (change) => errorFields(mod.edgePartialsDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ patches: [{ id: 'ok' }, { id: 3 }] }), ['patches[1].id']);
    assertEquals(fieldsOf({ patchSet: [{ score: 'high' }] }), ['patchSet[0].score']);
    assertEquals(fieldsOf({ nullablePatches: [null, { score: 'high' }] }), [
        'nullablePatches[1].score'
    ]);
    assertEquals(fieldsOf({ maybe: { id: 3 } }), ['maybe.id']);
    assertEquals(fieldsOf({ patches: ['text'] }), ['patches[0]']);
});

Deno.test('the value traits treat a Partial, Pick or Omit by its structure', () => {
    const source = partials();
    const copy = mod.edgePartialsClone(source);
    assert(mod.edgePartialsEquals(source, copy));
    assertStrictEquals(mod.edgePartialsHashCode(source), mod.edgePartialsHashCode(copy));
    assertNotStrictEquals(copy.patches[0], source.patches[0]);
    assertNotStrictEquals(copy.byName.get('k'), source.byName.get('k'));

    copy.patches[0].id = 'changed';
    assert(!mod.edgePartialsEquals(source, copy));
    assertStrictEquals(source.patches[0].id, 'a');
    assert(mod.edgePartialsToString(source).includes('patches: ['));
});

const renamed = () => ({
    profile: { displayName: 'Ada' },
    contact: { email: 'ada@example.com', displayName: 'Ada' },
    anonymous: {
        displayName: 'Anon',
        joinedAt: new Date('2024-01-02T03:04:05.000Z'),
        tags: ['x'],
        level: 2
    },
    strict: { id: 's' },
    labeled: { label: 'l' },
    labelOnly: { label: 'only' },
    head: { name: 'head' },
    link: { next: null }
});

Deno.test('a Pick or Omit names its fields as written and encodes them as renamed', () => {
    assertEquals(JSON.parse(mod.edgeRenamedPartialsEncode(renamed())), {
        profile: { display_name: 'Ada' },
        contact: { display_name: 'Ada', mail: 'ada@example.com' },
        anonymous: {
            display_name: 'Anon',
            joined_at: '2024-01-02T03:04:05.000Z',
            tags: ['x'],
            level: 2
        },
        strict: { id: 's' },
        labeled: { label: 'l' },
        labelOnly: { label: 'only' },
        head: { name: 'head' },
        link: { next: null }
    });
});

Deno.test('a Partial, Pick or Omit over renamed fields round-trips', () => {
    const result = mod.edgeRenamedPartialsDecode(mod.edgeRenamedPartialsEncode(renamed()));
    assert(result.success, JSON.stringify(result.errors));
    assertEquals(result.value, renamed());
    assert(result.value.anonymous.joinedAt instanceof Date);
});

Deno.test('a field a Partial, Pick or Omit has is validated as its type validates it', () => {
    const encoded = JSON.parse(mod.edgeRenamedPartialsEncode(renamed()));
    const fieldsOf = (change) =>
        errorFields(mod.edgeRenamedPartialsDecode({ ...encoded, ...change }));

    assertEquals(fieldsOf({ contact: { display_name: 'Ada', mail: 'not mail' } }), [
        'contact.mail'
    ]);
    assertEquals(fieldsOf({ profile: { mail: 'not mail' } }), ['profile.mail']);
    assertEquals(fieldsOf({ contact: { display_name: 'Ada' } }), ['contact.mail']);
});

Deno.test('a default fills a field a Pick or Omit keeps, and none a Partial lacks', () => {
    const encoded = JSON.parse(mod.edgeRenamedPartialsEncode(renamed()));
    const { level: _level, ...withoutLevel } = encoded.anonymous;

    const result = mod.edgeRenamedPartialsDecode({
        ...encoded,
        anonymous: withoutLevel,
        profile: {}
    });
    assert(result.success, JSON.stringify(result.errors));
    assertStrictEquals(result.value.anonymous.level, 1);
    assertEquals(result.value.profile, {});
});

Deno.test('a Pick of a type that denies unknown fields still denies them', () => {
    const encoded = JSON.parse(mod.edgeRenamedPartialsEncode(renamed()));

    const known = mod.edgeRenamedPartialsDecode({ ...encoded, strict: { id: 's', note: 'n' } });
    assert(known.success, JSON.stringify(known.errors));
    assertEquals(known.value.strict, { id: 's' });

    const unknown = mod.edgeRenamedPartialsDecode({ ...encoded, strict: { id: 's', bogus: 1 } });
    assert(!unknown.success);
    assertEquals(
        unknown.errors.map((error) => error.field),
        ['strict.bogus']
    );
});

Deno.test('a Partial of a type with a flattened field codes that field when it has it', () => {
    const origin = item('o', 1);
    const encoded = JSON.parse(
        mod.edgeRenamedPartialsEncode({ ...renamed(), labeled: { label: 'l', origin } })
    );
    assertEquals(encoded.labeled, { label: 'l', id: 'o', score: 1 });

    const result = mod.edgeRenamedPartialsDecode(encoded);
    assert(result.success, JSON.stringify(result.errors));
    assertEquals(result.value.labeled, { label: 'l', origin });

    const whole = { label: 'kept', origin };
    const picked = JSON.parse(mod.edgeRenamedPartialsEncode({ ...renamed(), labelOnly: whole }));
    assertEquals(picked.labelOnly, { label: 'kept' });
});

const optional = (extra) => ({ id: 'o', ...extra });

const required = () => ({
    plain: { id: 'p', level: 1 },
    whole: { id: 'w', nick: 'n', age: 3, level: 2 },
    picked: { nick: 'p' },
    again: { id: 'a', nick: 'n', age: 4, level: 1 },
    loose: { nick: 'l' },
    wholes: [{ id: 'x', nick: 'n', age: 5, level: 3 }]
});

Deno.test('a field a type leaves optional may be left out, and is reported once when required', () => {
    const result = mod.edgeOptionalDecode(mod.edgeOptionalEncode(optional()));
    assert(result.success, JSON.stringify(result.errors));
    assertEquals(result.value, { id: 'o', level: 1 });

    const missing = mod.edgeOptionalDecode({});
    assert(!missing.success);
    assertEquals(missing.errors, [{ field: 'id', message: 'missing required field' }]);
});

Deno.test('a Required of a type has to have the fields the type leaves optional', () => {
    const encoded = JSON.parse(mod.edgeRequiredEncode(required()));
    assertEquals(encoded.whole, { id: 'w', nick: 'n', years: 3, level: 2 });

    const result = mod.edgeRequiredDecode(encoded);
    assert(result.success, JSON.stringify(result.errors));
    assertEquals(result.value, required());

    const fieldsOf = (change) => errorFields(mod.edgeRequiredDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ whole: { id: 'w' } }), ['whole.nick', 'whole.years']);
    assertEquals(fieldsOf({ whole: { nick: 'n', years: 3 } }), ['whole.id']);
    assertEquals(fieldsOf({ picked: {} }), ['picked.nick']);
    assertEquals(fieldsOf({ again: { id: 'a', nick: 'n' } }), ['again.years']);
    assertEquals(fieldsOf({ wholes: [encoded.wholes[0], { id: 'y', years: 1 }] }), [
        'wholes[1].nick'
    ]);

    // A default fills the field a `Required` would otherwise miss, and a
    // `Partial` around a `Required` asks for none.
    const filled = mod.edgeRequiredDecode({
        ...encoded,
        whole: { id: 'w', nick: 'n', years: 3 },
        loose: {}
    });
    assert(filled.success, JSON.stringify(filled.errors));
    assertStrictEquals(filled.value.whole.level, 1);
    assertEquals(filled.value.loose, {});
});

Deno.test('a reference standing for a Required has to name a value with every field', () => {
    const encoded = JSON.parse(mod.edgeRequiredEncode(required()));
    const complete = { __id: 40, id: 'c', nick: 'n', years: 2 };
    const sparse = { __id: 41, id: 's' };

    const shared = mod.edgeRequiredDecode({ ...encoded, plain: complete, whole: { __ref: 40 } });
    assert(shared.success, JSON.stringify(shared.errors));
    assertStrictEquals(shared.value.whole, shared.value.plain);

    const lacking = mod.edgeRequiredDecode({ ...encoded, plain: sparse, whole: { __ref: 41 } });
    assertEquals(errorFields(lacking), ['whole.nick', 'whole.years']);
});

Deno.test('a date field takes a date or the text of one, and nothing else', () => {
    const base = {
        display_name: 'Ada',
        mail: 'ada@example.com',
        joined_at: '2024-01-02T03:04:05.000Z',
        tags: []
    };
    const fieldsOf = (change) => errorFields(types.edgeProfileDecode({ ...base, ...change }));
    for (const bad of ['garbage', '', 5, true, { a: 1 }, [], null, new Date('nope')]) {
        assertEquals(fieldsOf({ joined_at: bad }), ['joined_at'], JSON.stringify(bad));
    }
    const given = types.edgeProfileDecode({ ...base, joined_at: new Date(0) });
    assert(given.success, JSON.stringify(given.errors));
    assertStrictEquals(given.value.joinedAt.getTime(), 0);

    // A hole in an array is an element that is not there.
    const sparse = ['a'];
    sparse[2] = 'c';
    assertEquals(fieldsOf({ tags: sparse }), ['tags[1]']);
});
