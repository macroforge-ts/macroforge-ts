import { assertStrictEquals } from '@std/assert';
import {
    assertValidationError,
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

const MODULE_NAME = 'union-arm-validator-tests';

describe('Union Arm Validators', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule(MODULE_NAME);
    });

    describe('Primitive-only union', () => {
        test('accepts a valid string arm and an unvalidated number arm', () => {
            assertValidationSuccess(mod.Contact.decode('ada@example.com'), 'Contact');
            assertValidationSuccess(mod.Contact.decode(42), 'Contact');
        });

        test('runs the string arm validators', () => {
            assertValidationError(mod.Contact.decode('not-an-email'), '_root', 'valid email');
        });

        test('is runs the arm validators', () => {
            assertStrictEquals(mod.Contact.is('ada@example.com'), true);
            assertStrictEquals(mod.Contact.is('not-an-email'), false);
            assertStrictEquals(mod.Contact.is(7), true);
        });
    });

    describe('Literal beside a validated arm', () => {
        test('accepts the literal without running the arm validators', () => {
            assertValidationSuccess(mod.Fallback.decode('none'), 'Fallback');
            assertValidationSuccess(mod.Fallback.decode('ada@example.com'), 'Fallback');
            assertValidationError(mod.Fallback.decode('nobody'), '_root', 'valid email');
        });
    });

    describe('Mixed union', () => {
        test('accepts a valid string arm and an object arm', () => {
            assertValidationSuccess(mod.Reachable.decode('front desk'), 'Reachable');
            assertValidationSuccess(mod.Reachable.decode({ digits: '555' }), 'Reachable');
        });

        test('runs the string arm validators', () => {
            assertValidationError(mod.Reachable.decode(''), '_root', 'must not be empty');
            assertValidationError(
                mod.Reachable.decode('x'.repeat(33)),
                '_root',
                'at most 32 characters'
            );
            assertStrictEquals(mod.Reachable.is(''), false);
        });
    });
});
