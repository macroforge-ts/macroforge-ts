/**
 * Type aliases of every body a derive dispatches on, under each tagging
 * mode, and fields written through other type operators.
 */

import { assert, assertEquals, assertStrictEquals, assertThrows } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const edgeCases = join(vanillaRoot, 'src', 'edge-cases');
const mod = await expandAndCompile(join(edgeCases, 'alias-shapes.ts'));
const types = await expandAndCompile(join(edgeCases, 'edge-types.ts'));

const item = (id, score) => ({ id, score });
const card = (title) => new types.EdgeCard({ title, items: [item('in-card', 1)] });
const NEW_YEAR = '2024-01-01T00:00:00.000Z';
const at = () => new Date(NEW_YEAR);

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

/**
 * `value` encodes to `json`, which decodes back to it, and its clone equals
 * it and hashes the same. A decoder is handed the parsed value: text handed
 * to one whose type takes strings is that string, not JSON to parse.
 */
function assertCoded(name, value, json) {
    assertEquals(JSON.parse(mod[`${name}Encode`](value)), json, `${name} encodes`);
    assertEquals(decoded(mod[`${name}Decode`](json)), value, `${name} decodes`);
    const copy = mod[`${name}Clone`](value);
    assertEquals(copy, value, `${name} clones`);
    assert(mod[`${name}Equals`](value, copy), `${name}: a clone equals its source`);
    assertStrictEquals(
        mod[`${name}HashCode`](value),
        mod[`${name}HashCode`](copy),
        `${name} hashes`
    );
    assertStrictEquals(typeof mod[`${name}ToString`](value), 'string');
}

Deno.test('object, tuple and collection aliases are coded by their bodies', () => {
    assertCoded('aliasObject', { name: 'n', count: 1, at: at() }, {
        name: 'n',
        count: 1,
        at: NEW_YEAR
    });
    assertCoded('aliasObject', { name: 'n', count: 1 }, { name: 'n', count: 1 });
    assertCoded('aliasTuple', [item('a', 1), at(), null], [item('a', 1), NEW_YEAR, null]);
    assertCoded('aliasArray', [item('a', 1), item('b', 2)], [item('a', 1), item('b', 2)]);
    assertCoded('aliasDates', [at()], [NEW_YEAR]);
    assertCoded('aliasReadonlyStrings', ['a', 'b'], ['a', 'b']);
    assertCoded('aliasRow', ['label'], ['label']);
    assertCoded('aliasRow', ['label', 2, true, false], ['label', 2, true, false]);
    assertCoded('aliasRow', ['label', undefined, true], ['label', null, true]);
    assertCoded('aliasRow', ['label', undefined], ['label', null]);
    assertEquals(decoded(mod.aliasRowDecode(['label', undefined, true])), [
        'label',
        undefined,
        true
    ]);
    assertCoded('aliasRecord', { a: item('a', 1) }, { a: item('a', 1) });
    assertCoded('aliasMap', new Map([['a', item('a', 1)]]), { a: item('a', 1) });
    assertCoded('aliasSet', new Set([item('a', 1)]), [item('a', 1)]);
    assertCoded('aliasDate', at(), NEW_YEAR);
    assertCoded('aliasDateOrNull', at(), NEW_YEAR);
    assertCoded('aliasDateOrNull', null, null);
    assertCoded('aliasNullable', item('a', 1), item('a', 1));
    assertCoded('aliasNullable', null, null);
    assertCoded('aliasOptional', item('a', 1), item('a', 1));

    assert(decoded(mod.aliasMapDecode({ a: item('a', 1) })) instanceof Map);
    assert(decoded(mod.aliasSetDecode([item('a', 1)])) instanceof Set);
    assert(decoded(mod.aliasDatesDecode([NEW_YEAR]))[0] instanceof Date);
    assert(decoded(mod.aliasTupleDecode([item('a', 1), NEW_YEAR, 'text']))[1] instanceof Date);
});

Deno.test('a collection alias reports a bad member by its path', () => {
    const fieldsOf = (result) => {
        assert(!result.success, 'expected the decode to fail');
        return [...new Set(result.errors.map((error) => error.field))];
    };
    assertEquals(fieldsOf(mod.aliasArrayDecode([item('a', 1), { id: 'b' }])), ['_root[1].score']);
    assertEquals(fieldsOf(mod.aliasArrayDecode({ not: 'an array' })), ['_root']);
    assertEquals(fieldsOf(mod.aliasRecordDecode({ a: { score: 1 } })), ['_root.a.id']);
    assertEquals(fieldsOf(mod.aliasMapDecode({ a: 'text' })), ['_root.a']);
    assertEquals(fieldsOf(mod.aliasSetDecode([item('a', 1), 5])), ['_root[1]']);
    assertEquals(fieldsOf(mod.aliasDatesDecode([NEW_YEAR, false])), ['_root[1]']);
    assertEquals(fieldsOf(mod.aliasTupleDecode([item('a', 1), NEW_YEAR])), ['2']);
    assertEquals(fieldsOf(mod.aliasObjectDecode({ name: 'n' })), ['count']);
    assert(!mod.aliasArrayDecode('not an array').success);
});

Deno.test('intersections are coded as the fields of all their members', () => {
    const whole = { id: 'a', score: 1, extra: 'x' };
    assertCoded('aliasIntersection', whole, whole);
    assert(!mod.aliasIntersectionDecode({ id: 'a', score: 1 }).success);
    assert(!mod.aliasIntersectionDecode({ id: 'a', extra: 'x' }).success);
    assertCoded('aliasIntersectionOfObjects', { left: 'l', right: 1 }, { left: 'l', right: 1 });
});

Deno.test('unions of literals take only their members', () => {
    for (const size of ['small', 'medium', 'large']) assertCoded('aliasLiterals', size, size);
    assertStrictEquals(mod.aliasLiteralsDefaultValue(), 'small');
    assert(!mod.aliasLiteralsDecode('huge').success);
    assert(!mod.aliasLiteralsDecode(1).success);
    assertStrictEquals(mod.aliasLiteralsCompare('large', 'medium'), -1);
    assertStrictEquals(mod.aliasLiteralsCompare('small', 'medium'), 1);
    assertStrictEquals(mod.aliasLiteralsCompare('small', 'small'), 0);

    for (const member of ['auto', 0, 1, true, null]) {
        assertCoded('aliasMixedLiterals', member, member);
    }
    assertStrictEquals(mod.aliasMixedLiteralsDefaultValue(), 'auto');
    assert(!mod.aliasMixedLiteralsDecode(2).success);
    assert(!mod.aliasMixedLiteralsDecode(false).success);
    assert(!mod.aliasMixedLiteralsDecode('manual').success);
});

Deno.test('an undefined member of a union has no JSON text', () => {
    assertThrows(() => mod.aliasOptionalEncode(undefined), TypeError);
    assertStrictEquals(mod.aliasOptionalClone(undefined), undefined);
    assert(mod.aliasOptionalEquals(undefined, undefined));
    assert(!mod.aliasOptionalEquals(undefined, item('a', 1)));
});

Deno.test('a union of types is decoded as the member the value is', () => {
    assertCoded('aliasMembers', item('a', 1), item('a', 1));
    const asCard = decoded(mod.aliasMembersDecode(JSON.parse(mod.aliasMembersEncode(card('c')))));
    assert(asCard instanceof types.EdgeCard);
    assertEquals(asCard, card('c'));
    assert(!mod.aliasMembersDecode({ neither: true }).success);

    for (const member of ['text', 1, true, null, item('a', 1), [item('b', 2)]]) {
        assertEquals(decoded(mod.aliasDeepDecode(JSON.parse(mod.aliasDeepEncode(member)))), member);
    }
    assertEquals(JSON.parse(mod.aliasDeepEncode(at())), NEW_YEAR);
    assertEquals(decoded(mod.aliasDeepDecode(NEW_YEAR)), NEW_YEAR);
    assertEquals(decoded(mod.aliasDateDecode(NEW_YEAR)), at());
    assertEquals(decoded(mod.aliasDateOrNullDecode(NEW_YEAR)), at());
});

Deno.test("each tagging mode writes the member's name where it says", () => {
    const circle = { kind: 'circle', radius: 1 };
    const square = { kind: 'square', side: 2, at: at() };
    const group = { kind: 'group', members: [circle, square] };
    assertCoded('aliasTagged', circle, circle);
    assertCoded('aliasTagged', square, { kind: 'square', side: 2, at: NEW_YEAR });
    assertCoded('aliasTagged', group, {
        kind: 'group',
        members: [circle, { kind: 'square', side: 2, at: NEW_YEAR }]
    });
    assert(!mod.aliasTaggedDecode({ kind: 'triangle' }).success);
    assert(!mod.aliasTaggedDecode({ kind: 'circle' }).success);
    assert(!mod.aliasTaggedDecode({ radius: 1 }).success);

    assertEquals(JSON.parse(mod.aliasAdjacentEncode(item('a', 1))), {
        type: 'EdgeItem',
        value: item('a', 1)
    });
    const reparsed = (name, value) =>
        decoded(mod[`${name}Decode`](JSON.parse(mod[`${name}Encode`](value))));
    assertEquals(reparsed('aliasAdjacent', item('a', 1)), item('a', 1));
    const adjacentCard = reparsed('aliasAdjacent', card('c'));
    assert(adjacentCard instanceof types.EdgeCard);
    assert(!mod.aliasAdjacentDecode({ type: 'Other', value: {} }).success);

    assertEquals(JSON.parse(mod.aliasExternalEncode(item('a', 1))), { EdgeItem: item('a', 1) });
    assertEquals(reparsed('aliasExternal', item('a', 1)), item('a', 1));
    const externalCard = reparsed('aliasExternal', card('c'));
    assert(externalCard instanceof types.EdgeCard);
    assert(!mod.aliasExternalDecode({ Other: {} }).success);

    assertEquals(JSON.parse(mod.aliasUntaggedEncode(item('a', 1))), item('a', 1));
    assertEquals(JSON.parse(mod.aliasUntaggedEncode('text')), 'text');
    assertEquals(reparsed('aliasUntagged', item('a', 1)), item('a', 1));
    assertStrictEquals(reparsed('aliasUntagged', 'text'), 'text');
    const untaggedCard = reparsed('aliasUntagged', card('c'));
    assert(untaggedCard instanceof types.EdgeCard);
});

const operators = () => ({
    indexed: 'id-text',
    keys: 'score',
    template: 'item-1',
    literal: 'fixed',
    truth: true,
    zero: 0,
    anything: { any: ['thing', 1] },
    structure: { open: true },
    required: { name: 'n', count: 1, at: at() },
    readonlyItem: item('r', 1),
    nonNull: item('n', 2),
    inline: { x: 1, nested: { at: at(), item: item('i', 3) } },
    inlineList: [{ id: 'a' }]
});

Deno.test('fields written through type operators are coded by what the operator yields', () => {
    const encoded = JSON.parse(mod.aliasOperatorsEncode(operators()));
    assertEquals(encoded.required, { name: 'n', count: 1, at: NEW_YEAR });
    assertEquals(encoded.inline, { x: 1, nested: { at: NEW_YEAR, item: item('i', 3) } });

    const value = decoded(mod.aliasOperatorsDecode(encoded));
    assertEquals(value, operators());
    assert(value.required.at instanceof Date);
    assert(value.inline.nested.at instanceof Date);

    const copy = mod.aliasOperatorsClone(operators());
    assert(mod.aliasOperatorsEquals(operators(), copy));
    assertStrictEquals(mod.aliasOperatorsHashCode(operators()), mod.aliasOperatorsHashCode(copy));

    const fieldsOf = (change) => {
        const result = mod.aliasOperatorsDecode({ ...encoded, ...change });
        assert(!result.success, 'expected the decode to fail');
        return [...new Set(result.errors.map((error) => error.field))];
    };
    assertEquals(fieldsOf({ literal: 'other' }), ['literal']);
    // An operator over a declared type is read as what it yields.
    assertEquals(fieldsOf({ indexed: 5 }), ['indexed']);
    assertEquals(fieldsOf({ keys: 'neither' }), ['keys']);
    assertEquals(fieldsOf({ template: 'item-none' }), ['template']);
    assertEquals(fieldsOf({ structure: 5 }), ['structure']);
    assertEquals(fieldsOf({ truth: false }), ['truth']);
    assertEquals(fieldsOf({ zero: 1 }), ['zero']);
    assertEquals(fieldsOf({ readonlyItem: { id: 'r' } }), ['readonlyItem.score']);
    assertEquals(fieldsOf({ nonNull: null }), ['nonNull']);
    assertEquals(fieldsOf({ inline: { x: 1, nested: { at: 5, item: item('i', 3) } } }), [
        'inline.nested.at'
    ]);
    assertEquals(fieldsOf({ inline: { x: 'one', nested: { at: NEW_YEAR, item: item('i', 3) } } }), [
        'inline.x'
    ]);
    assertEquals(fieldsOf({ inlineList: [{ id: 1 }] }), ['inlineList[0].id']);
});

Deno.test('a holder of aliases codes and defaults each through its own functions', () => {
    const holder = {
        object: { name: 'n', count: 1 },
        size: 'large',
        sizes: ['small', 'medium'],
        shape: { kind: 'circle', radius: 1 },
        shapes: [{ kind: 'square', side: 2, at: at() }],
        tuple: [item('a', 1), at(), 'text'],
        list: [item('l', 2)],
        table: { t: item('t', 3) },
        members: [item('m', 4), card('c')]
    };
    const value = decoded(mod.aliasHolderDecode(JSON.parse(mod.aliasHolderEncode(holder))));
    assertEquals(value, holder);
    assert(value.members[1] instanceof types.EdgeCard);
    assert(value.shapes[0].at instanceof Date);
    assert(value.tuple[1] instanceof Date);

    assertEquals(mod.aliasHolderDefaultValue(), {
        object: { name: '', count: 0 },
        size: 'small',
        sizes: [],
        shape: null,
        shapes: [],
        tuple: null,
        list: [],
        table: {},
        members: []
    });

    const fieldsOf = (change) => {
        const result = mod.aliasHolderDecode({
            ...JSON.parse(mod.aliasHolderEncode(holder)),
            ...change
        });
        assert(!result.success, 'expected the decode to fail');
        return [...new Set(result.errors.map((error) => error.field))];
    };
    assertEquals(fieldsOf({ size: 'huge' }), ['size']);
    assertEquals(fieldsOf({ sizes: ['small', 'huge'] }), ['sizes[1]']);
    assertEquals(fieldsOf({ shapes: [{ kind: 'circle' }] }), ['shapes[0].radius']);
    assertEquals(fieldsOf({ object: { name: 'n' } }), ['object.count']);
    assertEquals(fieldsOf({ list: [{ id: 'l' }] }), ['list[0].score']);
});

Deno.test('a union member without a tag is told by its shape and coded field by field', () => {
    const at = new Date('2024-03-04T05:06:07.000Z');
    const ok = { ok: true, at, tags: new Set(['a', 'b']) };
    const failed = { ok: false, reason: 'offline' };

    const encoded = JSON.parse(mod.aliasOutcomeEncode(ok));
    assertEquals(encoded, { ok: true, at: '2024-03-04T05:06:07.000Z', tags: ['a', 'b'] });
    const value = decoded(mod.aliasOutcomeDecode(encoded));
    assert(value.at instanceof Date);
    assert(value.tags instanceof Set);
    assertEquals(value, ok);
    assertEquals(
        decoded(mod.aliasOutcomeDecode(JSON.parse(mod.aliasOutcomeEncode(failed)))),
        failed
    );
    assertEquals(
        decoded(mod.aliasOutcomeDecode({ ok: false, reason: 'again', retries: 2, stray: true })),
        { ok: false, reason: 'again', retries: 2 }
    );

    assert(!mod.aliasOutcomeDecode({ ok: true, at: 5, tags: [] }).success);
    assert(!mod.aliasOutcomeDecode({ ok: true, at: encoded.at, tags: [1] }).success);
    assert(!mod.aliasOutcomeDecode({ ok: false }).success);
    assert(!mod.aliasOutcomeDecode({ ok: 'yes', reason: 'r' }).success);

    const counts = { kind: 'b', counts: new Map([['x', 1]]) };
    const loose = JSON.parse(mod.aliasLooseEncode(counts));
    assertEquals(loose, { kind: 'b', counts: { x: 1 } });
    const back = decoded(mod.aliasLooseDecode(loose));
    assert(back.counts instanceof Map);
    assertEquals(back, counts);
    const dated = decoded(
        mod.aliasLooseDecode(JSON.parse(mod.aliasLooseEncode({ kind: 'a', when: at })))
    );
    assert(dated.when instanceof Date);
    assert(!mod.aliasLooseDecode({ kind: 'c', when: encoded.at }).success);
});

Deno.test('the text encode writes decodes to what was encoded, for a type that takes a string', async () => {
    const at = new Date('2024-03-04T05:06:07.000Z');
    const item = { id: 'a', score: 1 };
    const back = (name, value) => decoded(mod[`${name}Decode`](mod[`${name}Encode`](value)));

    assertStrictEquals(back('aliasName', 'Ada'), 'Ada');
    assertStrictEquals(back('aliasName', '"quoted"'), '"quoted"');
    assertStrictEquals(back('aliasName', '5'), '5');
    assertStrictEquals(back('aliasLiterals', 'medium'), 'medium');
    for (const member of ['auto', 0, 1, true, null]) {
        assertStrictEquals(back('aliasMixedLiterals', member), member);
    }
    assertStrictEquals(back('aliasDate', at).getTime(), at.getTime());
    assertStrictEquals(back('aliasDateOrNull', null), null);
    assertStrictEquals(back('aliasUntagged', 'plain'), 'plain');
    assertEquals(back('aliasUntagged', item), item);
    assertEquals(back('aliasDeep', item), item);
    assertStrictEquals(back('aliasDeep', 'text'), 'text');
    assertStrictEquals(back('aliasDeep', 7), 7);
    assertStrictEquals(back('aliasDeep', false), false);
    assertStrictEquals(back('aliasDeep', null), null);

    // A bare string is still the value it is, and one no member takes is refused.
    assertStrictEquals(decoded(mod.aliasNameDecode('Ada')), 'Ada');
    assertStrictEquals(decoded(mod.aliasLiteralsDecode('small')), 'small');
    assert(!mod.aliasLiteralsDecode('huge').success);
    assert(!mod.aliasLiteralsDecode('"huge"').success);
    assertStrictEquals(
        decoded(mod.aliasDateDecode('2024-03-04T05:06:07.000Z')).getTime(),
        at.getTime()
    );

    const enums = await expandAndCompile(join(edgeCases, 'enum-shapes.ts'));
    const { Empty, Quoted, Spaced, Slashed } = enums.EnumText;
    for (const member of [Empty, Quoted, Spaced, Slashed]) {
        assertStrictEquals(enums.enumTextDecode(enums.enumTextEncode(member)), member);
        assertStrictEquals(enums.enumTextDecode(member), member);
    }
    assertStrictEquals(
        enums.enumCountedDecode(enums.enumCountedEncode(enums.EnumCounted.Second)),
        enums.EnumCounted.Second
    );
    assertThrows(() => enums.enumTextDecode('no such member'));
});

Deno.test('a union carried as it is still has to be one of its members', () => {
    const at = new Date('2024-03-04T05:06:07.000Z');
    const carried = {
        tags: ['a', 'b'],
        size: 'auto',
        when: at,
        lookup: { a: 1 },
        anything: { any: [1] }
    };
    const encoded = JSON.parse(mod.aliasCarriedEncode(carried));
    const value = decoded(mod.aliasCarriedDecode(encoded));
    assertEquals(value, { ...carried, when: at.toISOString() });
    assertEquals(
        decoded(mod.aliasCarriedDecode({ ...encoded, tags: 'one', size: null, lookup: [1, 2] }))
            .lookup,
        [1, 2]
    );
    assertStrictEquals(decoded(mod.aliasCarriedDecode({ ...encoded, size: 3 })).size, 3);

    const fieldsOf = (change) => {
        const result = mod.aliasCarriedDecode({ ...encoded, ...change });
        assert(!result.success, JSON.stringify(change));
        return [...new Set(result.errors.map((error) => error.field))];
    };
    assertEquals(fieldsOf({ tags: 5 }), ['tags']);
    assertEquals(fieldsOf({ tags: ['a', 5] }), ['tags']);
    assertEquals(fieldsOf({ tags: null }), ['tags']);
    assertEquals(fieldsOf({ size: 'huge' }), ['size']);
    assertEquals(fieldsOf({ when: 5 }), ['when']);
    assertEquals(fieldsOf({ lookup: { a: 'one' } }), ['lookup']);
    assertEquals(fieldsOf({ lookup: ['one'] }), ['lookup']);
    assertEquals(fieldsOf({ lookup: 'none' }), ['lookup']);
});

Deno.test('each kind of alias refuses a value that is not of it', () => {
    const wrong = [null, 5, 'x', true, [], {}];
    const takes = {
        aliasArray: [[]],
        aliasDates: [[]],
        aliasRecord: [{}],
        aliasMap: [{}],
        aliasSet: [[]],
        aliasNullable: [null],
        aliasDateOrNull: [null],
        aliasMixedLiterals: [null, true],
        aliasName: ['x'],
        aliasUntagged: ['x']
    };
    const names = [
        'aliasObject',
        'aliasTuple',
        'aliasArray',
        'aliasDates',
        'aliasRecord',
        'aliasMap',
        'aliasSet',
        'aliasIntersection',
        'aliasLiterals',
        'aliasMixedLiterals',
        'aliasNullable',
        'aliasDate',
        'aliasDateOrNull',
        'aliasMembers',
        'aliasTagged',
        'aliasAdjacent',
        'aliasExternal',
        'aliasUntagged',
        'aliasOutcome',
        'aliasLoose',
        'aliasName',
        'aliasPicked',
        'aliasRequired',
        'aliasInline'
    ];
    const accepted = [];
    for (const name of names) {
        for (const value of wrong) {
            if (
                (takes[name] ?? []).some((taken) => JSON.stringify(taken) === JSON.stringify(value))
            ) continue;
            if (mod[`${name}Decode`](value).success) {
                accepted.push(`${name}: ${JSON.stringify(value)}`);
            }
        }
    }
    assertEquals(accepted, []);
});

Deno.test('an alias of a selection or an object type defaults to the fields it has to have', () => {
    assertEquals(mod.aliasPickedDefaultValue(), { id: '' });
    assertEquals(mod.aliasRequiredDefaultValue(), { id: '', score: 0 });
    const inline = mod.aliasInlineDefaultValue();
    assertStrictEquals(inline.x, 0);
    assert(inline.at instanceof Date);
    assert(!('label' in inline));

    const at = new Date('2024-03-04T05:06:07.000Z');
    const back = decoded(mod.aliasInlineDecode(mod.aliasInlineEncode({ x: 1, at })));
    assertEquals(back, { x: 1, at });
    assertEquals(decoded(mod.aliasPickedDecode(mod.aliasPickedEncode({ id: 'p' }))), { id: 'p' });
    assert(!mod.aliasRequiredDecode({ id: 'r' }).success);
});

Deno.test('a tagged union refuses input that is no member, in every tagging mode', () => {
    const item = { id: 'a', score: 1 };
    const at = '2024-03-04T05:06:07.000Z';
    const refused = {
        aliasTagged: [
            {},
            { kind: 5, radius: 1 },
            { kind: null, radius: 1 },
            { kind: 'circle', radius: 'wide' },
            { kind: 'circle', side: 2, at },
            { kind: 'square', side: 2 },
            { kind: 'square', side: 2, at: 'never' },
            { kind: 'group' },
            { kind: 'group', members: {} },
            { kind: 'group', members: [{ kind: 'circle' }] },
            { kind: 'group', members: [5] }
        ],
        aliasAdjacent: [
            {},
            { type: 'EdgeItem' },
            { type: 'EdgeItem', value: null },
            { type: 'EdgeItem', value: 5 },
            { type: 'EdgeItem', value: { id: 'a' } },
            { type: 'EdgeItem', value: { id: 5, score: 1 } },
            { type: 5, value: item },
            { value: item },
            item
        ],
        aliasExternal: [
            {},
            { EdgeItem: null },
            { EdgeItem: 5 },
            { EdgeItem: { id: 'a' } },
            { EdgeItem: { id: 5, score: 1 } },
            { Other: item },
            item
        ],
        aliasUntagged: [
            {},
            { id: 'a' },
            { id: 5, score: 1 },
            { title: 5, items: [] },
            5,
            true,
            null,
            []
        ],
        aliasMembers: [{}, { id: 'a' }, { id: 5, score: 1 }, { title: 't' }, 5, 'x', true, null, []]
    };
    const accepted = [];
    for (const [name, inputs] of Object.entries(refused)) {
        for (const input of inputs) {
            const result = mod[`${name}Decode`](input);
            if (result.success) {
                accepted.push(
                    `${name}: ${JSON.stringify(input)} -> ${JSON.stringify(result.value)}`
                );
            }
        }
    }
    assertEquals(accepted, []);
});

Deno.test('is says yes only to a value its decoder takes', () => {
    assert(mod.aliasDateIs(NEW_YEAR));
    assert(!mod.aliasDateIs('not a date'));
    assert(mod.aliasDateOrNullIs(null));
    assert(!mod.aliasDateOrNullIs('not a date'));
});
