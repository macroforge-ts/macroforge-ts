import { assertEquals, assertStrictEquals } from '@std/assert';
import {
    assertValidationError,
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

const MODULE_NAME = 'nested-decode-tests';

const fields = (result) => result.errors.map((error) => error.field);

describe('Nested decoding', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule(MODULE_NAME);
    });

    describe('Object type alias', () => {
        test('decodes array and record values through their decoder', () => {
            const result = mod.Route.decode({ stops: [{ x: 1 }], named: { home: { x: 2 } } });
            assertValidationSuccess(result, 'Route');
            assertStrictEquals(result.value.stops[0].x, 1);
            assertStrictEquals(result.value.named.home.x, 2);
        });

        test('reports an invalid array element under its index', () => {
            const result = mod.Route.decode({ stops: [{ x: 1 }, { x: -1 }], named: {} });
            assertValidationError(result, 'stops', 'must be non-negative');
            assertEquals(fields(result), ['stops[1].x']);
        });

        test('reports an invalid record value under its key', () => {
            const result = mod.Route.decode({ stops: [], named: { home: { x: -2 } } });
            assertValidationError(result, 'named', 'must be non-negative');
            assertEquals(fields(result), ['named.home.x']);
        });
    });

    describe('Interface with a flattened field', () => {
        test('decodes the flattened field from the enclosing object', () => {
            const result = mod.Labeled.decode({ label: 'start', x: 3 });
            assertValidationSuccess(result, 'Labeled');
            assertStrictEquals(result.value.origin.x, 3);
        });

        test('reports the flattened field validators', () => {
            assertValidationError(
                mod.Labeled.decode({ label: 'start', x: -3 }),
                'x',
                'must be non-negative'
            );
        });
    });

    describe('Generic object shapes', () => {
        test('decode their fields whatever the type argument', () => {
            const box = mod.Box.decode({ label: 'n', value: 3 });
            assertValidationSuccess(box, 'Box');
            assertStrictEquals(box.value.value, 3);
            const pair = mod.Pair.decode({ first: 'a', second: true });
            assertValidationSuccess(pair, 'Pair');
            assertStrictEquals(pair.value.second, true);
        });

        test('report a missing field', () => {
            assertValidationError(mod.Box.decode({ value: 3 }), 'label', 'missing required field');
        });
    });

    describe('Generic union', () => {
        test('decodes a member or a literal', () => {
            assertValidationSuccess(mod.Outcome.decode('none'), 'Outcome');
            const boxed = mod.Outcome.decode({ label: 'n', value: 1 });
            assertValidationSuccess(boxed, 'Outcome');
            assertStrictEquals(boxed.value.label, 'n');
        });
    });

    describe('Encoding round trip', () => {
        test('an object alias encodes its map and nested values for decode', () => {
            const route = {
                stops: [{ x: 1 }],
                named: { home: { x: 2 } },
                visits: new Map([['home', 3]])
            };
            const decoded = mod.Route.decode(mod.Route.encode(route));
            assertValidationSuccess(decoded, 'Route');
            assertStrictEquals(decoded.value.visits.get('home'), 3);
            assertStrictEquals(decoded.value.named.home.x, 2);
        });
    });
});
