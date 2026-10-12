/**
 * Encode and Decode over object graphs: a value held more than once, a child
 * that points back at its parent through a collection, a value that holds
 * itself, and a `Partial`, `Pick` or `Omit` standing where a cycle closes.
 */

import { assert, assertEquals, assertStrictEquals, assertThrows } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'graph-shapes.ts'));

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

Deno.test('children that point back at their parent round-trip as one graph', () => {
    const root = { name: 'root', parent: null, children: [] };
    const left = { name: 'left', parent: root, children: [] };
    const leaf = { name: 'leaf', parent: left, children: [] };
    left.children.push(leaf);
    root.children.push(left, { name: 'right', parent: root, children: [] });

    const tree = decoded(mod.graphTreeDecode(mod.graphTreeEncode(root, true)));
    assertEquals(tree.children.map((child) => child.name), ['left', 'right']);
    assertStrictEquals(tree.children[0].parent, tree);
    assertStrictEquals(tree.children[1].parent, tree);
    assertStrictEquals(tree.children[0].children[0].parent, tree.children[0]);
});

Deno.test('a class instance reached through an array, a set and a map is one instance', () => {
    const ada = new mod.GraphPerson({
        name: 'ada',
        best: null,
        friends: [],
        circle: new Set(),
        byName: new Map()
    });
    const bo = new mod.GraphPerson({
        name: 'bo',
        best: ada,
        friends: [ada],
        circle: new Set([ada]),
        byName: new Map([['ada', ada]])
    });
    ada.best = bo;
    ada.friends.push(bo, bo);
    ada.circle.add(bo);
    ada.byName.set('bo', bo);

    const person = decoded(mod.GraphPerson.decode(mod.GraphPerson.encode(ada, true)));
    const friend = person.best;
    assert(person instanceof mod.GraphPerson);
    assert(friend instanceof mod.GraphPerson);
    assertStrictEquals(friend.name, 'bo');
    assertStrictEquals(friend.best, person);
    assertEquals(person.friends.length, 2);
    assertStrictEquals(person.friends[0], friend);
    assertStrictEquals(person.friends[1], friend);
    assertStrictEquals(friend.friends[0], person);
    assert(person.circle.has(friend));
    assertStrictEquals(person.circle.size, 1);
    assert(friend.circle.has(person));
    assertStrictEquals(person.byName.get('bo'), friend);
    assertStrictEquals(friend.byName.get('ada'), person);
});

Deno.test('a value held in several fields and collections decodes to one value', () => {
    const item = { id: 'shared', score: 1 };
    const other = { id: 'other', score: 2 };
    const encoded = mod.graphSharedEncode(
        {
            first: item,
            again: item,
            maybe: item,
            list: [item, other, item],
            set: new Set([item, other]),
            byName: new Map([
                ['a', item],
                ['b', other]
            ]),
            record: { a: item, b: other },
            pair: [item, other]
        },
        true
    );

    const shared = decoded(mod.graphSharedDecode(encoded));
    assertEquals(shared.first, item);
    assertStrictEquals(shared.again, shared.first);
    assertStrictEquals(shared.maybe, shared.first);
    assertStrictEquals(shared.list[0], shared.first);
    assertStrictEquals(shared.list[2], shared.first);
    assertEquals(shared.list[1], other);
    assert(shared.set.has(shared.first));
    assertStrictEquals(shared.set.size, 2);
    assertStrictEquals(shared.byName.get('a'), shared.first);
    assertStrictEquals(shared.byName.get('b'), shared.list[1]);
    assertStrictEquals(shared.record.a, shared.first);
    assertStrictEquals(shared.record.b, shared.list[1]);
    assertStrictEquals(shared.pair[0], shared.first);
    assertStrictEquals(shared.pair[1], shared.list[1]);
});

Deno.test('a shared value encodes without its metadata as a copy at each place', () => {
    const item = { id: 'shared', score: 1 };
    const plain = JSON.parse(
        mod.graphPatchedEncode({ whole: item, patch: item, patches: [item, { id: 'only' }] })
    );
    assertEquals(plain, {
        whole: item,
        patch: item,
        patches: [item, { id: 'only' }]
    });
});

Deno.test('decoding freezes every value of the graph when asked to', () => {
    const item = { id: 'shared', score: 1 };
    const encoded = mod.graphSharedEncode(
        {
            first: item,
            again: item,
            maybe: null,
            list: [{ id: 'listed', score: 2 }],
            set: new Set([{ id: 'in-set', score: 3 }]),
            byName: new Map([['a', { id: 'mapped', score: 4 }]]),
            record: { a: { id: 'recorded', score: 5 } },
            pair: [item, item]
        },
        true
    );
    const result = mod.graphSharedDecode(encoded, { freeze: true });
    const shared = decoded(result);
    assert(Object.isFrozen(shared));
    assert(Object.isFrozen(shared.first));
    assert(Object.isFrozen(shared.list[0]));
    assert(Object.isFrozen([...shared.set][0]));
    assert(Object.isFrozen(shared.byName.get('a')));
    assert(Object.isFrozen(shared.record.a));
});

Deno.test('a value that holds itself round-trips', () => {
    const loop = { name: 'loop', self: null, view: null, rest: null, patch: null };
    loop.self = loop;
    const back = decoded(mod.graphLoopDecode(mod.graphLoopEncode(loop, true)));
    assertStrictEquals(back.self, back);
    assertStrictEquals(back.name, 'loop');
});

Deno.test('a Pick, Omit or Partial of a value that holds itself has no encoding', () => {
    const blank = () => ({ name: 'loop', self: null, view: null, rest: null, patch: null });

    const viewed = blank();
    viewed.view = viewed;
    assertThrows(() => mod.graphLoopEncode(viewed), TypeError, 'holds itself');

    const rested = blank();
    rested.rest = rested;
    assertThrows(() => mod.graphLoopEncode(rested), TypeError, 'holds itself');

    const patched = blank();
    patched.patch = patched;
    assertThrows(() => mod.graphLoopEncode(patched), TypeError, 'holds itself');
});

Deno.test('a Pick or Omit that reaches a whole value in a cycle refers back to it', () => {
    const loop = { name: 'loop', self: null, view: null, rest: null, patch: null };
    // The view keeps `name` and `view`; its own `view` ends the chain.
    loop.view = { name: 'view', view: null };
    // The rest keeps `self`, which closes the cycle on the whole value.
    loop.rest = { self: loop, view: null, rest: null, patch: null };

    const back = decoded(mod.graphLoopDecode(mod.graphLoopEncode(loop, true)));
    assertEquals(back.view, { name: 'view', view: null });
    assertStrictEquals(back.rest.self, back);
    assertEquals(Object.keys(back.rest).sort(), ['patch', 'rest', 'self', 'view']);
});

Deno.test('a value held whole and as a Partial is encoded at both places', () => {
    const item = { id: 'shared', score: 1 };

    const wholeFirst = mod.graphPatchedEncode({ whole: item, patch: item, patches: [item] }, true);
    const first = decoded(mod.graphPatchedDecode(wholeFirst));
    assertEquals(first.whole, item);
    assertEquals(first.patch, item);
    assertEquals(first.patches, [item]);

    const partialFirst = mod.graphPatchedFirstEncode({ patch: item, whole: item }, true);
    const second = decoded(mod.graphPatchedFirstDecode(partialFirst));
    assertEquals(second.patch, item);
    assertEquals(second.whole, item);
});

Deno.test('a reference to a value that was never encoded fails to decode', () => {
    const result = mod.graphSharedDecode({
        first: { __ref: 99 },
        again: { id: 'a', score: 1 },
        maybe: null,
        list: [],
        set: [],
        byName: {},
        record: {},
        pair: [
            { id: 'a', score: 1 },
            { id: 'b', score: 2 }
        ]
    });
    assert(!result.success);
    assert(result.errors[0].message.includes('99'), JSON.stringify(result.errors));
});

Deno.test('a reference ahead of the value it names resolves once the value is read', () => {
    const forward = decoded(
        mod.graphSharedDecode({
            first: { __ref: 5 },
            again: { __id: 5, id: 'later', score: 1 },
            maybe: null,
            list: [{ __ref: 6 }, { __id: 6, id: 'listed', score: 2 }],
            set: [{ __ref: 7 }],
            byName: { early: { __ref: 7 }, late: { __id: 7, id: 'mapped', score: 3 } },
            record: { early: { __ref: 8 }, late: { __id: 8, id: 'recorded', score: 4 } },
            pair: [{ __ref: 5 }, { __ref: 8 }]
        })
    );
    assertStrictEquals(forward.first, forward.again);
    assertStrictEquals(forward.list[0], forward.list[1]);
    assertStrictEquals(forward.byName.get('early'), forward.byName.get('late'));
    assertEquals([...forward.byName.keys()].sort(), ['early', 'late']);
    assert(forward.set.has(forward.byName.get('late')));
    assertStrictEquals(forward.record.early, forward.record.late);
    assertStrictEquals(forward.pair[0], forward.again);
    assertStrictEquals(forward.pair[1], forward.record.late);
});

Deno.test("a value shaped like the decoder's own markers is data, not a marker", async () => {
    const generics = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'generic-shapes.ts')
    );
    const lookalike = { __pendingRef: true, id: 99 };
    const box = decoded(
        generics.genericBoxDecode({
            held: 1,
            list: [],
            maybe: null,
            byName: { k: lookalike },
            set: [lookalike],
            record: {},
            pair: [1, 2]
        })
    );
    assertEquals(box.byName.get('k'), lookalike);
    assertEquals([...box.set], [lookalike]);
});

Deno.test('a reference to a value of another type fails to decode', async () => {
    const classes = await expandAndCompile(
        join(vanillaRoot, 'src', 'edge-cases', 'class-shapes.ts')
    );
    const card = { __id: 1, open: 'o', fixed: 'f', definite: 1, items: [] };
    const holder = { __id: 0, card, cards: [], byName: {}, maybe: null };

    const again = decoded(classes.ClassHolder.decode({ ...holder, cards: [{ __ref: 1 }] }));
    assertStrictEquals(again.cards[0], again.card);

    for (
        const change of [
            { maybe: { __ref: 1 } },
            { byName: { k: { __ref: 1 } } },
            { byName: { early: { __ref: 2 } }, cards: [{ ...card, __id: 2 }] }
        ]
    ) {
        const result = classes.ClassHolder.decode({ ...holder, ...change });
        assert(!result.success, JSON.stringify(change));
        assert(
            result.errors.some((error) => error.message.includes('where a ClassBase is expected')),
            JSON.stringify(result.errors)
        );
    }
});

Deno.test('the value traits follow a value that holds itself', () => {
    const cyclic = () => {
        const root = { name: 'root', parent: null, children: [] };
        root.children.push({ name: 'child', parent: root, children: [] });
        return root;
    };
    const root = cyclic();

    const copy = mod.graphTreeClone(root);
    assert(copy !== root);
    assertStrictEquals(copy.name, 'root');
    assertStrictEquals(copy.children[0].parent, copy);
    assert(copy.children[0] !== root.children[0]);

    assert(mod.graphTreeEquals(root, copy));
    assert(mod.graphTreeEquals(root, cyclic()));
    const other = cyclic();
    other.children[0].name = 'changed';
    assert(!mod.graphTreeEquals(root, other));
    assert(!mod.graphTreeEquals(other, root));

    assertStrictEquals(mod.graphTreeHashCode(root), mod.graphTreeHashCode(cyclic()));
    assert(mod.graphTreeHashCode(root) !== mod.graphTreeHashCode(other));
    assertEquals(
        mod.graphTreeToString(root),
        'GraphTree { name: root, parent: null, children: [GraphTree { name: child, parent: [Circular], children: [] }] }'
    );

    // A value held twice is one value in the copy, as it is in the original.
    const shared = { name: 'shared', parent: null, children: [] };
    const tree = { name: 'tree', parent: null, children: [shared, shared] };
    const twin = mod.graphTreeClone(tree);
    assertStrictEquals(twin.children[0], twin.children[1]);
    assert(twin.children[0] !== shared);
    assert(mod.graphTreeEquals(tree, twin));
    assertStrictEquals(mod.graphTreeHashCode(tree), mod.graphTreeHashCode(twin));
    // The next clone starts afresh.
    assert(mod.graphTreeClone(tree).children[0] !== twin.children[0]);
});

Deno.test('the value traits follow class instances that hold each other', () => {
    const person = (name) =>
        decoded(mod.GraphPerson.decode({ name, best: null, friends: [], circle: [], byName: {} }));
    const pair = () => {
        const ada = person('Ada');
        const grace = person('Grace');
        ada.best = grace;
        grace.best = ada;
        ada.friends.push(grace, ada);
        ada.circle.add(grace);
        grace.byName.set('ada', ada);
        return ada;
    };
    const ada = pair();

    const copy = mod.GraphPerson.clone(ada);
    assert(copy instanceof mod.GraphPerson);
    assert(copy !== ada);
    assert(copy.best instanceof mod.GraphPerson);
    assertStrictEquals(copy.best.best, copy);
    assertStrictEquals(copy.friends[0], copy.best);
    assertStrictEquals(copy.friends[1], copy);
    assert(copy.circle.has(copy.best));
    assertStrictEquals(copy.best.byName.get('ada'), copy);

    assert(mod.GraphPerson.equals(ada, copy));
    assert(mod.GraphPerson.equals(ada, pair()));
    assertStrictEquals(mod.GraphPerson.hashCode(ada), mod.GraphPerson.hashCode(pair()));
    const other = pair();
    other.best.name = 'Hopper';
    assert(!mod.GraphPerson.equals(ada, other));
    assert(mod.GraphPerson.toString(ada).includes('[Circular]'));
});

Deno.test('without its metadata a value held twice is written out at each place', () => {
    const item = { id: 'shared', score: 1 };
    const other = { id: 'other', score: 2 };
    const source = {
        first: item,
        again: item,
        maybe: item,
        list: [item, other, item],
        set: new Set([item, other]),
        byName: new Map([
            ['a', item],
            ['b', other]
        ]),
        record: { a: item, b: other },
        pair: [item, other]
    };
    const text = mod.graphSharedEncode(source);
    assert(!text.includes('__ref'));
    assert(!text.includes('__id'));
    assertEquals(JSON.parse(text).again, item);
    assertEquals(JSON.parse(text).list, [item, other, item]);

    // Nothing says the copies were one value, so they decode as equal ones.
    const shared = decoded(mod.graphSharedDecode(text));
    assertEquals(shared, source);
    assert(shared.again !== shared.first);

    const person = (name) =>
        decoded(mod.GraphPerson.decode({ name, best: null, friends: [], circle: [], byName: {} }));
    const ada = person('Ada');
    const grace = person('Grace');
    ada.friends.push(grace, grace);
    ada.byName.set('g', grace);
    const people = JSON.parse(mod.GraphPerson.encode(ada));
    assertEquals(people.friends.map((friend) => friend.name), ['Grace', 'Grace']);
    assertStrictEquals(people.byName.g.name, 'Grace');
    assert(decoded(mod.GraphPerson.decode(people)).friends[1] instanceof mod.GraphPerson);
});

Deno.test('without its metadata a value that holds itself has no text', () => {
    const loop = { name: 'loop', self: null, view: null, rest: null, patch: null };
    loop.self = loop;
    assertThrows(() => mod.graphLoopEncode(loop), TypeError, 'keepMetadata');

    const root = { name: 'root', parent: null, children: [] };
    root.children.push({ name: 'child', parent: root, children: [] });
    assertThrows(() => mod.graphTreeEncode(root), TypeError, 'keepMetadata');
    assertStrictEquals(typeof mod.graphTreeEncode(root, true), 'string');
});

Deno.test('the ordering derives follow a value that holds itself', () => {
    const cyclic = (childName) => {
        const root = { name: 'root', parent: null, children: [] };
        root.children.push({ name: childName, parent: root, children: [] });
        return root;
    };
    const first = cyclic('a');
    const same = cyclic('a');
    const later = cyclic('b');

    assertStrictEquals(mod.graphTreeCompare(first, same), 0);
    assertStrictEquals(mod.graphTreePartialCompare(first, same), 0);
    assert(mod.graphTreeCompare(first, later) < 0);
    assert(mod.graphTreeCompare(later, first) > 0);
    assert(mod.graphTreePartialCompare(first, later) < 0);
    assert(mod.graphTreePartialCompare(later, first) > 0);

    // A sort over such values ends, and agrees with their equality.
    const sorted = [later, first, same].sort(mod.graphTreeCompare);
    assertStrictEquals(sorted[2], later);
    assert(mod.graphTreeEquals(sorted[0], sorted[1]));
});
