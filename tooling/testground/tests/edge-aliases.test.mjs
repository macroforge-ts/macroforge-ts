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
