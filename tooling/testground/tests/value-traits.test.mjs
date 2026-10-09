/**
 * Runtime behavior of PartialEq, Hash and Clone, expanded through the CLI
 * cache like the validator suites.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'value-traits-tests.ts'));

const at = (iso) => new Date(iso);

const timeline = (labels) => ({
    events: [at('2020-01-01T00:00:00Z'), at('2021-01-01T00:00:00Z')],
    labels: new Map(labels),
    note: null
});

Deno.test('dates inside arrays and maps compare by value', () => {
    const entries = [['launch', at('2020-06-01T00:00:00Z')]];
    assert(mod.Timeline.equals(timeline(entries), timeline(entries)));
    const moved = [['launch', at('2020-07-01T00:00:00Z')]];
    assert(!mod.Timeline.equals(timeline(entries), timeline(moved)));
});

Deno.test('maps equal regardless of insertion order hash the same', () => {
    const first = timeline([
        ['a', at('2020-01-01T00:00:00Z')],
        ['b', at('2020-02-01T00:00:00Z')]
    ]);
    const second = timeline([
        ['b', at('2020-02-01T00:00:00Z')],
        ['a', at('2020-01-01T00:00:00Z')]
    ]);
    assert(mod.Timeline.equals(first, second));
    assertStrictEquals(mod.Timeline.hashCode(first), mod.Timeline.hashCode(second));
});

Deno.test('an absent optional field compares, hashes and clones without throwing', () => {
    const bare = timeline([]);
    const started = { ...timeline([]), started: at('2020-01-01T00:00:00Z') };
    assert(mod.Timeline.equals(bare, timeline([])));
    assert(!mod.Timeline.equals(bare, started));
    assert(mod.Timeline.equals(started, { ...timeline([]), started: at('2020-01-01T00:00:00Z') }));
    assertStrictEquals(mod.Timeline.hashCode(bare), mod.Timeline.hashCode(timeline([])));
    assert(!('started' in mod.Timeline.clone(bare)));
});

Deno.test('a clone is deep and equals its source', () => {
    const source = {
        ...timeline([['a', at('2020-01-01T00:00:00Z')]]),
        started: at('2019-01-01T00:00:00Z')
    };
    const copy = mod.Timeline.clone(source);
    assert(mod.Timeline.equals(source, copy));
    assertNotStrictEquals(copy.events[0], source.events[0]);
    assertNotStrictEquals(copy.labels.get('a'), source.labels.get('a'));
    assertNotStrictEquals(copy.started, source.started);
    assertStrictEquals(mod.Timeline.hashCode(source), mod.Timeline.hashCode(copy));
});

Deno.test('union members compare by structure, not property order', () => {
    const square = { kind: 'square', side: 2, at: at('2020-01-01T00:00:00Z') };
    const reordered = { at: at('2020-01-01T00:00:00Z'), side: 2, kind: 'square' };
    assert(mod.Shape.equals(square, reordered));
    assertStrictEquals(mod.Shape.hashCode(square), mod.Shape.hashCode(reordered));
    assert(!mod.Shape.equals(square, { ...square, at: at('2021-01-01T00:00:00Z') }));
    assert(!mod.Shape.equals(square, { kind: 'circle', radius: 2 }));
    const copy = mod.Shape.clone(square);
    assert(mod.Shape.equals(square, copy));
    assertNotStrictEquals(copy.at, square.at);
});

Deno.test('tuple elements compare and clone by their own types', () => {
    const moment = ['launch', at('2020-01-01T00:00:00Z')];
    assert(mod.Moment.equals(moment, ['launch', at('2020-01-01T00:00:00Z')]));
    assert(!mod.Moment.equals(moment, ['launch', at('2020-01-02T00:00:00Z')]));
    const copy = mod.Moment.clone(moment);
    assertEquals(copy.length, 2);
    assertNotStrictEquals(copy[1], moment[1]);
    assert(mod.Moment.equals(moment, copy));
});

Deno.test('cloning an array member of a union keeps an array', () => {
    const copy = mod.Reading.clone([1, 2, 3]);
    assert(Array.isArray(copy));
    assertEquals(copy, [1, 2, 3]);
    assertStrictEquals(mod.Reading.clone('raw'), 'raw');
    assert(mod.Reading.equals([1, 2], [1, 2]));
    assert(!mod.Reading.equals([1, 2], '1,2'));
});

Deno.test('sets and unknown fields reach a derived class through its own traits', () => {
    const badge = (label) => Object.assign(new mod.Badge(), { label });
    const wall = { badges: new Set([badge('gold'), badge('silver')]), pinned: badge('gold') };
    const copy = mod.Wall.clone(wall);
    assert(mod.Wall.equals(wall, copy));
    assertStrictEquals(mod.Wall.hashCode(wall), mod.Wall.hashCode(copy));
    assert(copy.pinned instanceof mod.Badge);
    assertNotStrictEquals(copy.pinned, wall.pinned);
    assert(!mod.Wall.equals(wall, { ...wall, pinned: badge('bronze') }));
    assert(!mod.Wall.equals(wall, { ...wall, badges: new Set([badge('gold'), badge('bronze')]) }));
});

Deno.test('Debug renders collections, object literals and bigints by their contents', () => {
    const inventory = { counts: new Map([['apples', 3]]), owner: { name: 'Ada' }, total: 3n };
    assertStrictEquals(
        mod.inventoryToString(inventory),
        'Inventory { counts: Map { apples: 3 }, owner: { name: Ada }, total: 3 }'
    );
    assertStrictEquals(mod.amountToString(5n), 'Amount(5)');
    assertStrictEquals(mod.amountToString({ value: 7n }), 'Amount({ value: 7 })');
});

Deno.test('a class inside a tuple or union alias encodes through its own encoder', () => {
    const ledger = (entries) => new mod.Ledger({ entries: new Map(entries) });
    const first = ledger([['rent', 900]]);
    const second = ledger([['food', 200]]);
    const own = (value) => JSON.parse(mod.ledgerEncode(value));
    assertEquals(JSON.parse(mod.ledgerPairEncode([first, second])), [own(first), own(second)]);
    assertEquals(JSON.parse(mod.ledgerOrNoteEncode(first)), own(first));
    assertStrictEquals(JSON.parse(mod.ledgerOrNoteEncode('note')), 'note');
});

Deno.test('an inline object member of a tagged union encodes each field by its type', () => {
    const counts = JSON.parse(mod.tallyEncode({ kind: 'counts', counts: new Map([['a', 1]]) }));
    assertEquals(counts.kind, 'counts');
    assert(Object.keys(counts.counts).length > 0, 'the map keeps its entries');
    assertEquals(JSON.parse(mod.tallyEncode({ kind: 'note', text: 'hi' })), {
        kind: 'note',
        text: 'hi'
    });
});

Deno.test('a generic class gets a default through its own parameters', () => {
    const shelf = mod.shelfDefaultValue();
    assert(shelf instanceof mod.Shelf);
    assertEquals(shelf.items, []);
    assertStrictEquals(shelf.label, '');
});

Deno.test('ordering is zero exactly when the values are equal', () => {
    const rank = (label, tier, at) => ({ label, meta: { tier }, ...(at ? { at } : {}) });
    const pairs = [
        [rank('a', 1), rank('a', 1)],
        [rank('a', 1), rank('a', 2)],
        [rank('a', 1), rank('a', 1, at('2020-01-01T00:00:00Z'))],
        [rank('é', 1), rank('é', 1)]
    ];
    for (const [left, right] of pairs) {
        const equal = mod.Rank.equals(left, right);
        assertStrictEquals(mod.Rank.compare(left, right) === 0, equal);
        assertStrictEquals(mod.Rank.partialCompare(left, right) === 0, equal);
        assertStrictEquals(mod.Rank.compare(left, right), -mod.Rank.compare(right, left) || 0);
    }
    assertStrictEquals(
        mod.Rank.compare(rank('a', 1), rank('a', 1, at('2020-01-01T00:00:00Z'))),
        -1
    );
});

Deno.test('union members order consistently with their equality', () => {
    const values = [null, { kind: 'a', count: 2 }, { kind: 'a', count: 1 }, {
        kind: 'b',
        name: 'x'
    }];
    for (const left of values) {
        for (const right of values) {
            const order = mod.Choice.compare(left, right);
            assertStrictEquals(order === 0, mod.Choice.equals(left, right));
            assertStrictEquals(order, -mod.Choice.compare(right, left) || 0);
        }
    }
    assertStrictEquals(mod.Choice.compare({ count: 1, kind: 'a' }, { kind: 'a', count: 1 }), 0);
    assertStrictEquals(mod.Choice.compare(null, { kind: 'a', count: 1 }), -1);
});

Deno.test('tuple and plain aliases round-trip through their element types', () => {
    const ledger = (entries) => new mod.Ledger({ entries: new Map(entries) });
    const pair = mod.ledgerPairDecode(mod.ledgerPairEncode([ledger([['rent', 900]]), ledger([])]));
    assert(pair.success, JSON.stringify(pair.errors));
    assert(pair.value[0] instanceof mod.Ledger);
    assertEquals(pair.value[0].entries.get('rent'), 900);
    assertEquals(pair.value.length, 2);

    const main = mod.mainLedgerDecode(mod.mainLedgerEncode(ledger([['food', 200]])));
    assert(main.success, JSON.stringify(main.errors));
    assert(main.value instanceof mod.Ledger);
    assertEquals(main.value.entries.get('food'), 200);

    const totals = mod.totalsDecode(mod.totalsEncode(new Map([['a', 1]])));
    assert(totals.success, JSON.stringify(totals.errors));
    assert(totals.value instanceof Map);
    assertEquals(totals.value.get('a'), 1);

    const short = mod.ledgerPairDecode([JSON.parse(mod.ledgerEncode(ledger([])))]);
    assert(!short.success);
    assertEquals(short.errors[0].field, '1');
});

Deno.test('constrained generic types derive with their constraints', () => {
    const labeled = { label: 'gold', weight: 2 };
    assert(mod.Labeled.equals(labeled, { ...labeled }));
    assertEquals(mod.Labeled.clone(labeled), labeled);
    const decoded = mod.labeledDecode(mod.labeledEncode(labeled));
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertEquals(decoded.value, labeled);
    assertEquals(mod.crateDefaultValue().items, []);
});

Deno.test('a type with a required constrained parameter derives every trait', () => {
    const keyed = { key: 'a', rank: 1 };
    assert(mod.Keyed.equals(keyed, { ...keyed }));
    assertStrictEquals(mod.Keyed.compare(keyed, { key: 'a', rank: 2 }), -1);
    const decoded = mod.keyedDecode(mod.keyedEncode(keyed));
    assert(decoded.success, JSON.stringify(decoded.errors));
    assertEquals(decoded.value, keyed);
    const slot = new mod.Slot({ held: { id: 1 }, index: 2 });
    assert(mod.Slot.equals(slot, mod.Slot.clone(slot)));
    assertStrictEquals(mod.Slot.compare(slot, slot), 0);
});

Deno.test('constrained generic aliases encode and decode', () => {
    const counted = mod.countedDecode(mod.countedEncode([{ id: 1 }, 2]));
    assert(counted.success, JSON.stringify(counted.errors));
    assertEquals(counted.value, [{ id: 1 }, 2]);
    const code = mod.codeDecode(mod.codeEncode('a'));
    assert(code.success, JSON.stringify(code.errors));
    assertStrictEquals(code.value, 'a');
    const boxed = mod.boxedDecode(mod.boxedEncode([{ id: 1 }]));
    assert(boxed.success, JSON.stringify(boxed.errors));
    assertEquals(boxed.value, [{ id: 1 }]);
});
