import { assertEquals, assertNotStrictEquals, assertStrictEquals } from '@std/assert';
import {
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

describe('Default with field initializers', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule('default-initializer-tests');
    });

    test('a field defaults to its initializer', () => {
        const settings = mod.Settings.defaultValue();
        assertStrictEquals(settings.theme, 'dark');
        assertEquals(settings.tags, ['general']);
        assertStrictEquals(typeof settings.createdAt, 'number');
    });

    test('an optional field with an initializer defaults to it', () => {
        assertStrictEquals(mod.Settings.defaultValue().retries, 3);
    });

    test('an optional field without one stays absent', () => {
        assertStrictEquals('label' in mod.Settings.defaultValue(), false);
    });

    test('@default wins over the initializer', () => {
        assertStrictEquals(mod.Settings.defaultValue().limit, 7);
    });

    test('each default evaluates the initializers afresh', () => {
        assertNotStrictEquals(mod.Settings.defaultValue().tags, mod.Settings.defaultValue().tags);
    });

    test('a function initializer keeps its own this', () => {
        assertStrictEquals(mod.Greeter.defaultValue().greet(), 'hello world');
    });

    test('a default is an instance of the class', () => {
        assertStrictEquals(mod.Settings.defaultValue() instanceof mod.Settings, true);
    });

    test('a static property is neither defaulted nor decoded', () => {
        assertStrictEquals(Object.hasOwn(mod.Settings.defaultValue(), 'instances'), false);
        const result = mod.Settings.decode({
            theme: 'light',
            tags: [],
            limit: 1,
            createdAt: 0
        });
        assertValidationSuccess(result, 'Settings');
        assertStrictEquals(mod.Settings.instances, 0);
    });

    test('no derive treats a static property as a field', () => {
        const counter = mod.Counter.defaultValue();
        assertEquals(JSON.parse(mod.Counter.encode(counter)), { count: 1 });
        assertStrictEquals(mod.Counter.toString(counter), 'Counter { count: 1 }');
        assertStrictEquals(mod.Counter.equals(counter, mod.Counter.clone(counter)), true);
        assertStrictEquals(mod.Counter.compare(counter, counter), 0);
        assertStrictEquals(mod.Counter.partialCompare(counter, counter), 0);
        assertValidationSuccess(mod.Counter.decode({ count: 2 }), 'Counter');
        assertStrictEquals(mod.Counter.created, 0);
    });
});
