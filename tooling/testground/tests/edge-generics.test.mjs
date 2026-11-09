/**
 * Generic types: parameters in every position a field can hold one, and
 * fields that instantiate another generic type.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'generic-shapes.ts'));
const types = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'edge-types.ts'));

const item = (id, score) => ({ id, score });

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

const box = (held, other) => ({
    held,
    list: [held, other],
    maybe: other,
    optional: held,
    byName: new Map([['k', other]]),
    set: new Set([held]),
    record: { k: held },
    pair: [held, other]
});

Deno.test('a value of a type parameter is carried as it is, wherever it is held', () => {
    for (const [held, other] of [['text', 'other'], [1, 2], [true, false]]) {
        const source = box(held, other);
        const value = decoded(mod.genericBoxFromString(mod.genericBoxEncode(source)));
        assertEquals(value, source);
        assert(value.byName instanceof Map);
        assert(value.set instanceof Set);

        const copy = mod.genericBoxClone(source);
        assertEquals(copy, source);
        assertNotStrictEquals(copy.list, source.list);
        assert(mod.genericBoxEquals(source, copy));
        assertStrictEquals(mod.genericBoxHashCode(source), mod.genericBoxHashCode(copy));
        assert(mod.genericBoxToString(source).includes('held'));
    }

    const { optional: _optional, ...rest } = box('text', 'other');
    const sparse = decoded(
        mod.genericBoxFromString(mod.genericBoxEncode({ ...rest, maybe: null }))
    );
    assertStrictEquals(sparse.maybe, null);
    assert(!('optional' in sparse));
    const encoded = JSON.parse(mod.genericBoxEncode(rest));
    assertEquals(errorFields(mod.genericBoxDecode({ ...encoded, list: 'none' })), ['list']);
});

Deno.test('several parameters, defaults and constraints each keep their own fields', () => {
    const pair = { left: 'l', right: 2 };
    assertEquals(decoded(mod.genericPairFromString(mod.genericPairEncode(pair))), pair);
    assert(mod.genericPairEquals(pair, mod.genericPairClone(pair)));
    assert(!mod.genericPairEquals(pair, { left: 'l', right: 3 }));
    assertEquals(errorFields(mod.genericPairDecode({ left: 'l' })), ['right']);

    const constrained = { item: item('a', 1), name: 'n', items: [item('b', 2)] };
    assertEquals(
        decoded(mod.genericConstrainedFromString(mod.genericConstrainedEncode(constrained))),
        constrained
    );
    assert(mod.genericConstrainedEquals(constrained, mod.genericConstrainedClone(constrained)));
});

Deno.test('a generic class is built, coded and compared as an instance', () => {
    const blank = mod.GenericStack.defaultValue();
    assert(blank instanceof mod.GenericStack);
    assertEquals({ ...blank }, { items: [], top: null, depth: 0 });

    const stack = decoded(mod.GenericStack.decode({ items: [1, 2], top: 2, depth: 2 }));
    assert(stack instanceof mod.GenericStack);
    assertEquals(JSON.parse(mod.GenericStack.encode(stack)), { items: [1, 2], top: 2, depth: 2 });

    const copy = mod.GenericStack.clone(stack);
    assert(copy instanceof mod.GenericStack);
    assertNotStrictEquals(copy.items, stack.items);
    assert(mod.GenericStack.equals(stack, copy));
    assertStrictEquals(mod.GenericStack.hashCode(stack), mod.GenericStack.hashCode(copy));
    copy.items.push(3);
    assert(!mod.GenericStack.equals(stack, copy));
});

Deno.test('generic unions, lists and nullable aliases are coded by their bodies', () => {
    const ok = { ok: true, value: 1 };
    const failed = { ok: false, failure: 'reason' };
    assertEquals(decoded(mod.genericResultFromString(mod.genericResultEncode(ok))), ok);
    assertEquals(decoded(mod.genericResultFromString(mod.genericResultEncode(failed))), failed);
    assert(mod.genericResultEquals(ok, mod.genericResultClone(ok)));
    assert(!mod.genericResultEquals(ok, failed));

    assertEquals(decoded(mod.genericListFromString(mod.genericListEncode([1, 2]))), [1, 2]);
    assertEquals(decoded(mod.genericMaybeFromString(mod.genericMaybeEncode('text'))), 'text');
    assertStrictEquals(decoded(mod.genericMaybeFromString(mod.genericMaybeEncode(null))), null);
});

const uses = () => ({
    text: box('text', 'other'),
    item: box(item('a', 1), item('b', 2)),
    nested: box(box(1, 2), box(3, 4)),
    pair: { left: 'l', right: item('r', 1) },
    same: { left: 1, right: 2 },
    stack: mod.GenericStack.decode({ items: [item('s', 1)], top: null, depth: 1 }).value,
    stacks: [mod.GenericStack.decode({ items: ['x'], top: 'x', depth: 1 }).value],
    list: [item('l', 1)],
    maybe: item('m', 1),
    boxes: new Map()
});

Deno.test('a field that instantiates a generic type is coded through that type', () => {
    const value = decoded(mod.genericUsesFromString(mod.genericUsesEncode(uses())));
    assertEquals(value, uses());
    assert(value.stack instanceof mod.GenericStack);
    assert(value.stacks[0] instanceof mod.GenericStack);
    assert(value.item.byName instanceof Map);
    assert(value.item.set instanceof Set);

    const copy = mod.genericUsesClone(uses());
    assert(mod.genericUsesEquals(uses(), copy));
    assertStrictEquals(mod.genericUsesHashCode(uses()), mod.genericUsesHashCode(copy));
    assert(copy.stack instanceof mod.GenericStack);

    const encoded = JSON.parse(mod.genericUsesEncode(uses()));
    const fieldsOf = (change) => errorFields(mod.genericUsesDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ list: [{ id: 'l' }] }), ['list[0].score']);
    assertEquals(fieldsOf({ maybe: { score: 1 } }), ['maybe.id']);
    assertEquals(fieldsOf({ stack: { items: [], top: null } }), ['stack.depth']);
    assertEquals(fieldsOf({ stacks: [{ top: null, depth: 0 }] }), ['stacks[0].items']);
    assertEquals(fieldsOf({ pair: { left: 'l' } }), ['pair.right']);
});

Deno.test('a type argument is coded as its type, wherever the generic type holds it', () => {
    const held = new Date('2024-01-01T00:00:00.000Z');
    const other = new Date('2024-06-01T00:00:00.000Z');
    const encoded = JSON.parse(
        mod.genericUsesEncode({ ...uses(), boxes: new Map([['b', box(held, other)]]) })
    );
    assertStrictEquals(encoded.boxes.b.held, '2024-01-01T00:00:00.000Z');
    assertEquals(encoded.boxes.b.set, ['2024-01-01T00:00:00.000Z']);

    const value = decoded(mod.genericUsesDecode(encoded));
    const dated = value.boxes.get('b');
    assert(dated.held instanceof Date);
    assertEquals(dated, box(held, other));
    assert(dated.byName.get('k') instanceof Date);
    assert([...dated.set][0] instanceof Date);
    assert(dated.pair[1] instanceof Date);

    // A box held as another box's `T` is coded as a box.
    assertEquals(value.nested, uses().nested);
    assert(value.nested.held.byName instanceof Map);
    assert(value.nested.list[1].set instanceof Set);

    const fieldsOf = (change) => errorFields(mod.genericUsesDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ item: rawBox({ id: 'a' }, { id: 'b', score: 2 }) }), [
        'item.held.score'
    ]);
    assertEquals(fieldsOf({ boxes: { b: { ...encoded.boxes.b, held: 5 } } }), ['boxes.b.held']);
    assertEquals(fieldsOf({ pair: { left: 'l', right: { id: 'r' } } }), ['pair.right.score']);
});

// A box as it is encoded, with nothing held twice.
const rawBox = (held, other) => ({
    held,
    list: [other],
    maybe: other,
    byName: { k: other },
    set: [other],
    record: { k: other },
    pair: [other, other]
});

const card = (title) => types.EdgeCard.decode({ title, items: [item(title, 1)] }).value;

const argumentsOf = () => {
    const at = new Date('2024-02-03T04:05:06.000Z');
    const later = new Date('2025-02-03T04:05:06.000Z');
    return {
        dated: box(at, later),
        maybe: box(later, at),
        optional: box(card('a'), card('b')),
        pair: { left: at, right: new Map([['k', later]]) },
        stack: mod.GenericStack.decode({ items: [], top: null, depth: 0 }).value,
        result: { ok: true, value: at },
        partial: { held: at, list: [later] },
        picked: { right: 12345678901234567890n },
        wrapper: { inner: box(at, later), pairs: [{ left: at, right: [later] }] },
        plain: box('text', 'other'),
        link: box(at, later),
        linkOrNull: box(later, at),
        outcome: { left: at, right: [later] },
        outcomes: ['none', { left: later, right: [] }]
    };
};

Deno.test('every kind of generic type takes the coders of its arguments', () => {
    const source = argumentsOf();
    source.stack.items.push(source.dated.held);
    source.stack.top = source.dated.held;
    source.stack.depth = 1;

    const encoded = JSON.parse(mod.genericArgumentsEncode(source));
    assertStrictEquals(encoded.dated.held, '2024-02-03T04:05:06.000Z');
    assertStrictEquals(encoded.pair.right.k, '2025-02-03T04:05:06.000Z');
    assertEquals(encoded.stack.items, ['2024-02-03T04:05:06.000Z']);
    assertStrictEquals(encoded.result.value, '2024-02-03T04:05:06.000Z');
    assertEquals(encoded.partial, {
        held: '2024-02-03T04:05:06.000Z',
        list: ['2025-02-03T04:05:06.000Z']
    });
    assertEquals(encoded.picked, { right: '12345678901234567890' });
    assertStrictEquals(encoded.wrapper.pairs[0].right[0], '2025-02-03T04:05:06.000Z');
    assertStrictEquals(encoded.optional.held.title, 'a');

    const value = decoded(mod.genericArgumentsDecode(encoded));
    assertEquals(value.dated, source.dated);
    assertEquals(value.maybe, source.maybe);
    assert(value.optional.held instanceof types.EdgeCard);
    assert(value.optional.byName.get('k') instanceof types.EdgeCard);
    assert(value.pair.left instanceof Date);
    assert(value.pair.right instanceof Map);
    assert(value.pair.right.get('k') instanceof Date);
    assert(value.stack instanceof mod.GenericStack);
    assert(value.stack.items[0] instanceof Date);
    assert(value.stack.top instanceof Date);
    assert(value.result.value instanceof Date);
    assertEquals(value.partial, source.partial);
    assertStrictEquals(value.picked.right, 12345678901234567890n);
    assertEquals(value.wrapper, source.wrapper);
    assert(value.wrapper.pairs[0].right[0] instanceof Date);
    assertEquals(value.plain, source.plain);
    assertEquals(value.link, source.link);
    assert(value.link.held instanceof Date);
    assert(value.linkOrNull.list[1] instanceof Date);

    // A generic union alias is coded by its own coders, handed its argument's.
    assertStrictEquals(encoded.outcome.left, '2024-02-03T04:05:06.000Z');
    assertEquals(value.outcome, source.outcome);
    assert(value.outcome.right[0] instanceof Date);
    assertEquals(value.outcomes, source.outcomes);
    assert(value.outcomes[1].left instanceof Date);

    // The other member of each union is carried as it is.
    const texts = decoded(
        mod.genericArgumentsDecode({ ...encoded, link: 'plain', linkOrNull: null })
    );
    assertStrictEquals(texts.link, 'plain');
    assertStrictEquals(texts.linkOrNull, null);
    assertStrictEquals(
        decoded(mod.genericArgumentsDecode({ ...encoded, linkOrNull: 'text' })).linkOrNull,
        'text'
    );

    const failed = decoded(
        mod.genericArgumentsDecode({
            ...encoded,
            result: { ok: false, failure: { title: 'failed', items: [] } }
        })
    );
    assert(failed.result.failure instanceof types.EdgeCard);

    const sparse = decoded(
        mod.genericArgumentsDecode({ ...encoded, maybe: null, optional: undefined })
    );
    assertStrictEquals(sparse.maybe, null);
    assert(!('optional' in sparse));

    const fieldsOf = (change) => errorFields(mod.genericArgumentsDecode({ ...encoded, ...change }));
    assertEquals(fieldsOf({ dated: { ...encoded.dated, held: 5 } }), ['dated.held']);
    assertEquals(
        fieldsOf({ maybe: { ...encoded.maybe, list: [encoded.maybe.held, 5] } }),
        ['maybe.list[1]']
    );
    assertEquals(fieldsOf({ stack: { items: [5], top: null, depth: 1 } }), ['stack.items[0]']);
    assertEquals(fieldsOf({ result: { ok: true, value: 5 } }), ['result.value']);
    assertEquals(fieldsOf({ picked: { right: 'not a number' } }), ['picked.right']);
    assertEquals(
        fieldsOf({ wrapper: { ...encoded.wrapper, inner: { ...encoded.wrapper.inner, held: 5 } } }),
        ['wrapper.inner.held']
    );
    assertEquals(
        fieldsOf({ optional: rawBox({ items: [] }, { title: 'b', items: [] }) }),
        ['optional.held.title']
    );
});

Deno.test('a generic type given no coder carries its parameter as it is', () => {
    const at = '2024-02-03T04:05:06.000Z';
    const value = decoded(mod.genericBoxFromString(mod.genericBoxEncode(box(at, at))));
    assertStrictEquals(value.held, at);
    const stack = decoded(mod.GenericStack.decode({ items: [at], top: at, depth: 1 }));
    assertStrictEquals(stack.top, at);
});

Deno.test('a generic type encoded and decoded from the top takes the coders of its parameters', () => {
    const at = new Date('2024-02-03T04:05:06.000Z');
    const toText = (date) => date.toISOString();
    const toDate = (raw) => {
        if (typeof raw !== 'string') throw new Error('expected the text of a date');
        return new Date(raw);
    };

    const text = mod.genericBoxEncode(box(at, at), false, toText);
    assertStrictEquals(JSON.parse(text).held, '2024-02-03T04:05:06.000Z');
    const value = decoded(mod.genericBoxFromString(text, undefined, toDate));
    assertEquals(value, box(at, at));
    assert(value.byName.get('k') instanceof Date);
    assertEquals(
        errorFields(mod.genericBoxDecode({ ...JSON.parse(text), held: 5 }, undefined, toDate)),
        ['held']
    );

    const stack = decoded(
        mod.GenericStack.decode({ items: [toText(at)], top: null, depth: 1 }, undefined, toDate)
    );
    assert(stack instanceof mod.GenericStack);
    assert(stack.items[0] instanceof Date);
    assertEquals(JSON.parse(mod.GenericStack.encode(stack, false, toText)).items, [toText(at)]);
    assert(
        decoded(
            mod.genericStackFromString(
                mod.genericStackEncode(stack, false, toText),
                undefined,
                toDate
            )
        ).items[0] instanceof Date
    );

    const pair = { left: at, right: [at] };
    const outcome = decoded(
        mod.genericOutcomeFromString(
            mod.genericOutcomeEncode(pair, false, toText),
            undefined,
            toDate
        )
    );
    assert(outcome.left instanceof Date);
    assert(outcome.right[0] instanceof Date);
});

Deno.test('a union alias hands its parameter on to a member written with it', () => {
    const pair = { left: 1, right: [2, 3] };
    const back = (value) => mod.genericOutcomeFromString(mod.genericOutcomeEncode(value));
    assertEquals(decoded(back(pair)), pair);
    assertStrictEquals(decoded(back('none')), 'none');
    assert(!mod.genericOutcomeDecode({ left: 1 }).success);
    assert(!mod.genericOutcomeDecode('other').success);
    assert(mod.genericOutcomeEquals(pair, mod.genericOutcomeClone(pair)));
});

Deno.test('the value traits treat a type argument by what it is', () => {
    const source = argumentsOf();
    const copy = mod.genericArgumentsClone(source);
    assert(mod.genericArgumentsEquals(source, copy));
    assertStrictEquals(mod.genericArgumentsHashCode(source), mod.genericArgumentsHashCode(copy));

    // A date, a class instance and a map held as a `T` are copied as such.
    assert(copy.dated.held instanceof Date);
    assertNotStrictEquals(copy.dated.held, source.dated.held);
    assert(copy.optional.held instanceof types.EdgeCard);
    assertNotStrictEquals(copy.optional.held, source.optional.held);
    assert(copy.pair.right instanceof Map);
    assertNotStrictEquals(copy.pair.right, source.pair.right);
    assert(copy.stack instanceof mod.GenericStack);
    assert(copy.result.value instanceof Date);
    assert(copy.dated.set instanceof Set);

    // A change inside a `T` is a difference.
    copy.dated.held.setFullYear(1999);
    assert(!mod.genericArgumentsEquals(source, copy));
    assert(mod.genericArgumentsHashCode(source) !== mod.genericArgumentsHashCode(copy));
    assertStrictEquals(source.dated.held.getFullYear(), 2024);

    const other = mod.genericArgumentsClone(source);
    other.optional.held.title = 'changed';
    assert(!mod.genericArgumentsEquals(source, other));
    other.optional.held.title = 'a';
    other.pair.right.set('k', new Date(0));
    assert(!mod.genericArgumentsEquals(source, other));

    const text = mod.genericArgumentsToString(source);
    assert(text.includes(String(source.dated.held)), text);
    assert(text.includes('EdgeCard'), text);
});

Deno.test('a type argument left to its default is coded and checked as that default', () => {
    const filled = (change) => Object.assign(mod.GenericDefaults.defaultValue(), change);
    const holder = {
        text: filled({ value: 'a', many: ['b'], pair: ['x', 'y'] }),
        counted: filled({ value: 1, many: [2], pair: [3, 4] }),
        explicit: filled({ value: 's', many: ['t'] })
    };
    const encoded = JSON.parse(mod.genericDefaultedEncode(holder));
    const back = decoded(mod.genericDefaultedDecode(encoded));
    assert(back.text instanceof mod.GenericDefaults);
    assert(mod.genericDefaultedEquals(holder, back));

    const fieldsOf = (name, change) =>
        errorFields(
            mod.genericDefaultedDecode({ ...encoded, [name]: { ...encoded[name], ...change } })
        );
    // `T` defaults to `string`, and `U` to an array of whatever `T` is.
    assertEquals(fieldsOf('text', { value: 1 }), ['text.value']);
    assertEquals(fieldsOf('text', { pair: ['x', 2] }), ['text.pair[1]']);
    assertEquals(fieldsOf('text', { many: [1] }), ['text.many[0]']);
    assertEquals(fieldsOf('counted', { value: 'one' }), ['counted.value']);
    assertEquals(fieldsOf('counted', { many: ['two'] }), ['counted.many[0]']);
    assertEquals(fieldsOf('explicit', { many: [1] }), ['explicit.many[0]']);
});

Deno.test('a generic type is what the guards handed in say of each value of its parameters', () => {
    const isNumber = (value) => typeof value === 'number';
    const isText = (value) => typeof value === 'string';
    assert(mod.genericMaybeIs(5, isNumber));
    assert(mod.genericMaybeIs(null, isNumber));
    assert(!mod.genericMaybeIs('x', isNumber));
    assert(mod.genericListIs([1, 2], isNumber));
    assert(!mod.genericListIs([1, 'x'], isNumber));

    const box = (held) => ({
        held,
        list: [held],
        maybe: null,
        byName: { a: held },
        set: [held],
        record: { a: held },
        pair: [held, held]
    });
    assert(mod.genericBoxIs(box(1), isNumber));
    assert(!mod.genericBoxIs(box('x'), isNumber));
    assert(!mod.genericBoxIs({ ...box(1), pair: [1, 'x'] }, isNumber));

    assert(mod.genericResultIs({ ok: true, value: 1 }, isNumber, isText));
    assert(!mod.genericResultIs({ ok: false, failure: 1 }, isNumber, isText));
    assert(mod.genericStackIs({ items: [1], top: 1, depth: 1 }, isNumber));
    assert(!mod.genericStackIs({ items: ['x'], top: null, depth: 1 }, isNumber));
});

Deno.test('a member tagged in place over a type parameter is coded by the coders handed in for it', () => {
    const at = '2024-01-01T00:00:00.000Z';
    const encodeAt = (value) => ({ at: value.at.toISOString() });
    const decodeAt = (raw) => ({ at: new Date(raw.at) });
    const held = { kind: 'Held', at: new Date(at) };
    assertEquals(JSON.parse(mod.genericTaggedEncode(held, false, encodeAt)), { kind: 'Held', at });
    const decoded = mod.genericTaggedDecode({ kind: 'Held', at }, undefined, decodeAt);
    assert(decoded.success, JSON.stringify(decoded));
    assert(decoded.value.at instanceof Date);
    assert(mod.genericTaggedHeldIs(held));
    assert(!mod.genericTaggedHeldIs({ kind: 'Empty', size: 1 }));
});
