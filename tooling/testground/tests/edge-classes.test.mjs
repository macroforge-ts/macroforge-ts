/**
 * Class members a derive has to tell from plain fields, and the classes a
 * derive has to build without their constructors: guarded fields, a
 * constructor of the class's own, parameter properties and inherited fields.
 */

import { assert, assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import { join } from 'node:path';
import { expandAndCompile } from './validators/helpers.mjs';
import { vanillaRoot } from './test-utils.mjs';

const mod = await expandAndCompile(join(vanillaRoot, 'src', 'edge-cases', 'class-shapes.ts'));

const item = (id, score) => ({ id, score });

const decoded = (result) => {
    assert(result.success, JSON.stringify(result.errors));
    return result.value;
};

const errorFields = (result) => {
    assert(!result.success, 'expected the decode to fail');
    return [...new Set(result.errors.map((error) => error.field))];
};

Deno.test('static properties, accessors and methods are not fields', () => {
    const members = decoded(
        mod.ClassMembers.decode({ open: 'o', fixed: 'f', definite: 2, items: [item('a', 1)] })
    );
    assert(members instanceof mod.ClassMembers);
    assertStrictEquals(members.label, 'o:f');
    assertStrictEquals(members.describe(), 'o (1)');
    assertEquals(JSON.parse(mod.ClassMembers.encode(members)), {
        open: 'o',
        fixed: 'f',
        definite: 2,
        items: [item('a', 1)]
    });
    const text = mod.ClassMembers.toString(members);
    assert(!text.includes('created'), text);
    assert(!text.includes('label'), text);
    assert(!text.includes('describe'), text);

    const blank = mod.ClassMembers.defaultValue();
    assert(blank instanceof mod.ClassMembers);
    assertEquals({ ...blank }, { open: 'open', fixed: 'fixed', definite: 0, items: [] });
    assert(!('optional' in blank));
    assertStrictEquals(mod.ClassMembers.created, 0);
});

Deno.test('readonly, optional and definite fields are cloned, compared and hashed', () => {
    const source = decoded(
        mod.ClassMembers.decode({
            open: 'o',
            fixed: 'f',
            optional: 'here',
            definite: 2,
            items: [item('a', 1)]
        })
    );
    const copy = mod.ClassMembers.clone(source);
    assert(copy instanceof mod.ClassMembers);
    assertNotStrictEquals(copy, source);
    assertNotStrictEquals(copy.items, source.items);
    assertEquals(copy, source);
    assert(mod.ClassMembers.equals(source, copy));
    assertStrictEquals(mod.ClassMembers.hashCode(source), mod.ClassMembers.hashCode(copy));

    const { optional: _optional, ...without } = JSON.parse(mod.ClassMembers.encode(source));
    const absent = decoded(mod.ClassMembers.decode(without));
    assert(!('optional' in absent));
    assert(!('optional' in mod.ClassMembers.clone(absent)));
    assert(!mod.ClassMembers.equals(source, absent));
    assertEquals(errorFields(mod.ClassMembers.decode({ ...without, fixed: 1 })), ['fixed']);
});

Deno.test('private, protected and readonly fields are fields to every derive', () => {
    const blank = mod.ClassGuarded.defaultValue();
    assert(blank instanceof mod.ClassGuarded);
    assertEquals(blank.reveal(), [1, true, 'sealed']);

    const encoded = JSON.parse(mod.ClassGuarded.encode(blank));
    assertEquals(encoded, { name: '', guarded: 1, hidden: true, sealed: 'sealed' });

    const value = decoded(
        mod.ClassGuarded.decode({ name: 'n', guarded: 2, hidden: false, sealed: 'open' })
    );
    assert(value instanceof mod.ClassGuarded);
    assertEquals(value.reveal(), [2, false, 'open']);
    assertEquals(errorFields(mod.ClassGuarded.decode({ name: 'n', guarded: 2, hidden: false })), [
        'sealed'
    ]);

    const copy = mod.ClassGuarded.clone(value);
    assert(copy instanceof mod.ClassGuarded);
    assertEquals(copy.reveal(), [2, false, 'open']);
    assert(mod.ClassGuarded.equals(value, copy));
    assertStrictEquals(mod.ClassGuarded.hashCode(value), mod.ClassGuarded.hashCode(copy));
    assertStrictEquals(mod.ClassGuarded.compare(value, copy), 0);
    assert(!mod.ClassGuarded.equals(value, blank));
    assertStrictEquals(mod.ClassGuarded.compare(blank, value), -1);
    assert(mod.ClassGuarded.toString(value).includes('hidden: false'));
    assertStrictEquals(mod.classGuardedEquals(value, copy), true);
    assertStrictEquals(mod.classGuardedHashCode(value), mod.ClassGuarded.hashCode(value));
});

Deno.test('a class with a constructor of its own keeps it, and is built without it', () => {
    const made = new mod.ClassConstructed('made', 2);
    assertEquals(JSON.parse(mod.ClassConstructed.encode(made)), { label: 'made', weight: 2 });

    const value = decoded(mod.ClassConstructed.decode({ label: 'read', weight: 3 }));
    assert(value instanceof mod.ClassConstructed);
    assertEquals({ ...value }, { label: 'read', weight: 3 });
    assertStrictEquals(mod.ClassConstructed.compare(made, value), -1);

    const copy = mod.ClassConstructed.clone(made);
    assert(copy instanceof mod.ClassConstructed);
    assert(mod.ClassConstructed.equals(made, copy));

    const blank = mod.ClassConstructed.defaultValue();
    assert(blank instanceof mod.ClassConstructed);
    assertEquals({ ...blank }, { label: '', weight: 0 });
});

Deno.test('constructor parameter properties are fields', () => {
    const made = new mod.ClassParameters('made', 2, 'kept');
    assertEquals(JSON.parse(mod.ClassParameters.encode(made)), {
        name: 'made',
        rank: 2,
        secret: 'kept'
    });

    const copy = mod.ClassParameters.clone(made);
    assert(copy instanceof mod.ClassParameters);
    assertEquals({ ...copy }, { name: 'made', rank: 2, secret: 'kept' });
    assertStrictEquals(copy.reveal(), 'kept');
    assert(mod.ClassParameters.equals(made, copy));
    assert(!mod.ClassParameters.equals(made, new mod.ClassParameters('made', 2, 'other')));
    assertStrictEquals(mod.ClassParameters.hashCode(made), mod.ClassParameters.hashCode(copy));
    assert(mod.ClassParameters.toString(made).includes('rank: 2'));

    const value = decoded(mod.ClassParameters.decode({ name: 'read', rank: 3, secret: 's' }));
    assert(value instanceof mod.ClassParameters);
    assertStrictEquals(value.reveal(), 's');
    assertEquals(errorFields(mod.ClassParameters.decode({ name: 'read', secret: 's' })), ['rank']);

    const blank = mod.ClassParameters.defaultValue();
    assert(blank instanceof mod.ClassParameters);
    assertEquals({ ...blank }, { name: '', rank: 0, secret: 'secret' });
});

Deno.test('a subclass is coded, cloned and compared with the fields it inherits', () => {
    const value = decoded(mod.ClassDerived.decode({ id: 'base', extra: 2 }));
    assert(value instanceof mod.ClassDerived);
    assert(value instanceof mod.ClassBase);
    assertEquals({ ...value }, { id: 'base', extra: 2 });
    assertEquals(JSON.parse(mod.ClassDerived.encode(value)), { id: 'base', extra: 2 });
    assertEquals(errorFields(mod.ClassDerived.decode({ extra: 2 })), ['id']);

    const copy = mod.ClassDerived.clone(value);
    assert(copy instanceof mod.ClassDerived);
    assertEquals({ ...copy }, { id: 'base', extra: 2 });
    assert(mod.ClassDerived.equals(value, copy));
    assertStrictEquals(mod.ClassDerived.hashCode(value), mod.ClassDerived.hashCode(copy));

    const other = decoded(mod.ClassDerived.decode({ id: 'other', extra: 2 }));
    assert(!mod.ClassDerived.equals(value, other));
    assert(mod.ClassDerived.toString(value).includes('id: base'));

    const blank = mod.ClassDerived.defaultValue();
    assert(blank instanceof mod.ClassDerived);
    assertEquals({ ...blank }, { id: '', extra: 0 });
});

Deno.test("an abstract class is cloned and compared through its subclass's instances", () => {
    class Square extends mod.ClassAbstract {
        area() {
            return 4;
        }
    }
    const square = new Square();
    const copy = mod.ClassAbstract.clone(square);
    assert(copy instanceof Square);
    assertStrictEquals(copy.area(), 4);
    assert(mod.ClassAbstract.equals(square, copy));
    assertStrictEquals(mod.ClassAbstract.hashCode(square), mod.ClassAbstract.hashCode(copy));
    assert(mod.ClassAbstract.toString(square).includes('kind: abstract'));
    assertStrictEquals(mod.ClassAbstract.defaultValue().kind, 'abstract');
});

Deno.test('a class holding other classes codes each through its own functions', () => {
    const blank = mod.ClassHolder.defaultValue();
    assert(blank.card instanceof mod.ClassMembers);
    assertEquals(blank.cards, []);
    assertStrictEquals(blank.maybe, null);

    const base = decoded(mod.ClassBase.decode({ id: 'b' }));
    const holder = decoded(
        mod.ClassHolder.decode({
            card: { open: 'o', fixed: 'f', definite: 1, items: [] },
            cards: [{ open: 'p', fixed: 'g', definite: 2, items: [item('a', 1)] }],
            byName: { b: { id: 'b' } },
            maybe: { id: 'm' }
        })
    );
    assert(holder.card instanceof mod.ClassMembers);
    assert(holder.cards[0] instanceof mod.ClassMembers);
    assert(holder.byName.get('b') instanceof mod.ClassBase);
    assert(holder.maybe instanceof mod.ClassBase);
    assert(mod.ClassBase.equals(holder.byName.get('b'), base));

    const copy = mod.ClassHolder.clone(holder);
    assert(copy.cards[0] instanceof mod.ClassMembers);
    assertNotStrictEquals(copy.cards[0], holder.cards[0]);
    assertNotStrictEquals(copy.byName.get('b'), holder.byName.get('b'));
    assert(mod.ClassHolder.equals(holder, copy));
    assertStrictEquals(mod.ClassHolder.hashCode(holder), mod.ClassHolder.hashCode(copy));
    assertEquals(
        errorFields(
            mod.ClassHolder.decode({
                ...JSON.parse(mod.ClassHolder.encode(holder)),
                cards: [{ open: 'p', fixed: 'g', items: [] }]
            })
        ),
        ['cards[0].definite']
    );
});

Deno.test('a function held in a field is carried by reference', () => {
    const blank = mod.ClassCallback.defaultValue();
    assert(blank instanceof mod.ClassCallback);
    assertStrictEquals(typeof blank.onChange, 'function');

    const copy = mod.ClassCallback.clone(blank);
    assertStrictEquals(copy.onChange, blank.onChange);
    assert(mod.ClassCallback.equals(blank, copy));
    assertStrictEquals(mod.ClassCallback.hashCode(blank), mod.ClassCallback.hashCode(copy));

    const other = mod.ClassCallback.clone(blank);
    other.onChange = () => {};
    assert(!mod.ClassCallback.equals(blank, other));
    assert(mod.ClassCallback.toString(blank).includes('name'));
});

Deno.test('an interface is coded, cloned and compared with the fields of what it extends', () => {
    const at = new Date('2024-01-01T00:00:00.000Z');
    const account = { id: 'a', kind: 'account', at, balance: 5 };
    assertEquals(JSON.parse(mod.classAccountEncode(account)), {
        id: 'a',
        kind: 'account',
        at: '2024-01-01T00:00:00.000Z',
        balance: 5
    });
    const value = decoded(mod.classAccountDecode(mod.classAccountEncode(account)));
    assertEquals(value, account);
    assert(value.at instanceof Date);
    assertEquals(Object.keys(value), ['id', 'kind', 'at', 'balance']);

    const { id: _id, ...withoutId } = JSON.parse(mod.classAccountEncode(account));
    assertEquals(errorFields(mod.classAccountDecode(withoutId)), ['id']);
    assertEquals(
        errorFields(mod.classAccountDecode({ ...withoutId, id: 'a', kind: 'entity' })),
        ['kind']
    );

    const copy = mod.classAccountClone(account);
    assertEquals(copy, account);
    assertNotStrictEquals(copy.at, account.at);
    assert(mod.classAccountEquals(account, copy));
    assertStrictEquals(mod.classAccountHashCode(account), mod.classAccountHashCode(copy));
    assert(!mod.classAccountEquals(account, { ...account, id: 'b' }));
    assert(mod.classAccountToString(account).includes('id: a'));

    const blank = mod.classAccountDefaultValue();
    assertEquals(Object.keys(blank), ['id', 'kind', 'at', 'balance']);
    assertStrictEquals(blank.kind, 'account');
    assertStrictEquals(blank.balance, 0);
});
