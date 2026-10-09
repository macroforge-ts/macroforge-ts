import { assertEquals, assertStrictEquals } from '@std/assert';
import {
    assertValidationError,
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

const MODULE_NAME = 'newtype-validator-tests';

describe('Newtype Validators', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule(MODULE_NAME);
    });

    describe('Number newtype', () => {
        test('decodes a valid value', () => {
            const result = mod.Meters.decode(12.5);
            assertValidationSuccess(result, 'Meters');
            assertStrictEquals(result.value, 12.5);
        });

        test('rejects a value of the wrong type', () => {
            const result = mod.Meters.decode({ meters: 3 });
            assertValidationError(result, '_root', 'expected number');
        });

        test('runs the alias validators', () => {
            assertValidationError(mod.Meters.decode(-1), '_root', 'Meters must be non-negative');
            assertValidationError(mod.Meters.decode(Infinity), '_root', 'Meters must be finite');
        });

        test('is narrows only valid values', () => {
            assertStrictEquals(mod.Meters.is(3), true);
            assertStrictEquals(mod.Meters.is(-3), false);
            assertStrictEquals(mod.Meters.is('3'), false);
            assertStrictEquals(mod.Meters.hasShape(-3), true);
        });

        test('encodes the bare primitive and defaults to zero', () => {
            assertStrictEquals(mod.Meters.encode(4), '4');
            assertStrictEquals(mod.Meters.defaultValue(), 0);
        });
    });

    describe('String newtype', () => {
        test('decodes a raw string without parsing it as JSON', () => {
            const result = mod.Username.decode('ada');
            assertValidationSuccess(result, 'Username');
            assertStrictEquals(result.value, 'ada');
        });

        test('runs the alias validators', () => {
            assertValidationError(mod.Username.decode(''), '_root', 'must not be empty');
            assertValidationError(
                mod.Username.decode('a'.repeat(17)),
                '_root',
                'at most 16 characters'
            );
        });

        test('rejects a number', () => {
            assertValidationError(mod.Username.decode(7), '_root', 'expected string');
        });
    });

    describe('Plain primitive alias', () => {
        test('checks the base type', () => {
            assertValidationSuccess(mod.Port.decode(8080), 'Port');
            assertValidationError(mod.Port.decode({ port: 8080 }), '_root', 'expected number');
            assertStrictEquals(mod.Port.is('8080'), false);
        });
    });

    describe('BigInt newtype', () => {
        test('encodes as a decimal string and decodes it back', () => {
            const big = 12345678901234567890n;
            assertStrictEquals(mod.Cents.encode(big), '"12345678901234567890"');
            const result = mod.Cents.decode(JSON.parse(mod.Cents.encode(big)));
            assertValidationSuccess(result, 'Cents');
            assertStrictEquals(result.value, big);
        });

        test('keeps every digit of a raw string input', () => {
            const result = mod.Cents.decode('123456789012345678901');
            assertValidationSuccess(result, 'Cents');
            assertStrictEquals(result.value, 123456789012345678901n);
        });

        test('accepts an in-memory bigint and an integral number', () => {
            assertValidationSuccess(mod.Cents.decode(7n), 'Cents');
            assertStrictEquals(mod.Cents.decode(7).value, 7n);
        });

        test('rejects a fractional number', () => {
            assertValidationError(mod.Cents.decode(1.5), '_root', 'expected bigint');
        });

        test('reports an unparseable value and runs the alias validators', () => {
            assertValidationError(mod.Cents.decode('twelve'), '_root', 'expected bigint');
            assertValidationError(mod.Cents.decode('-5'), '_root', 'Cents must be non-negative');
        });

        test('is narrows only an in-memory bigint', () => {
            assertStrictEquals(mod.Cents.is(5n), true);
            assertStrictEquals(mod.Cents.is('5'), false);
            assertStrictEquals(mod.Cents.is(-5n), false);
        });
    });

    describe('Newtype from another module', () => {
        let consumer;

        before(async () => {
            consumer = await loadValidatorModule('newtype-consumer');
        });

        test('decodes and validates a field typed with an imported newtype', () => {
            assertValidationSuccess(consumer.Trip.decode({ leg: 3, legs: [1, 2] }), 'Trip');
            const result = consumer.Trip.decode({ leg: -3, legs: [1, 'x'] });
            assertStrictEquals(result.success, false);
            assertEquals(result.errors.map((error) => error.field).sort(), ['leg', 'legs[1]']);
        });
    });

    describe('Newtype fields', () => {
        const run = {
            runner: 'ada',
            distance: 10,
            splits: [4, 6],
            best: 4
        };

        test('round-trips through encode and decode', () => {
            const decoded = mod.Run.decode(run);
            assertValidationSuccess(decoded, 'Run');
            assertEquals(mod.Run.decode(mod.Run.encode(decoded.value)).value, decoded.value);
        });

        test('reports alias validator failures under the field name', () => {
            const result = mod.Run.decode({ ...run, distance: -2, splits: [1, -1] });
            assertStrictEquals(result.success, false);
            const fields = result.errors.map((error) => error.field).sort();
            assertEquals(fields, ['distance', 'splits[1]']);
        });

        test('reports a wrong base type under the field name', () => {
            const result = mod.Run.decode({ ...run, runner: 42 });
            assertStrictEquals(result.success, false);
            assertEquals(result.errors.map((error) => error.field), ['runner']);
        });
    });
});
