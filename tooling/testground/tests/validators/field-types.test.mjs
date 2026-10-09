import { assertEquals, assertStrictEquals } from '@std/assert';
import {
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

const MODULE_NAME = 'field-type-tests';

const valid = {
    name: 'ada',
    age: 36,
    active: true,
    kind: 'user',
    mode: 'dark',
    nickname: null,
    rank: undefined,
    height: 1.7,
    balance: '120',
    site: 'https://example.com/'
};

function errorFields(result) {
    assertStrictEquals(result.success, false, 'Expected decoding to fail');
    return result.errors.map((error) => error.field).sort();
}

describe('Field Types', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule(MODULE_NAME);
    });

    describe('Primitive-like fields', () => {
        test('accepts values of every declared type', () => {
            const result = mod.Profile.decode(valid);
            assertValidationSuccess(result, 'Profile');
            assertStrictEquals(result.value.balance, 120n);
            assertStrictEquals(result.value.site.href, 'https://example.com/');
        });

        test('accepts a present optional and a present nullable', () => {
            assertValidationSuccess(
                mod.Profile.decode({ ...valid, bio: 'hi', nickname: 'a', rank: 2 }),
                'Profile'
            );
        });

        test('reports each mistyped field by name', () => {
            const result = mod.Profile.decode({
                ...valid,
                name: 1,
                age: '36',
                active: 'yes',
                kind: 'admin',
                mode: 'blue',
                nickname: 5,
                bio: null,
                rank: 'x',
                height: '1.7'
            });
            assertEquals(errorFields(result), [
                'active',
                'age',
                'bio',
                'height',
                'kind',
                'mode',
                'name',
                'nickname',
                'rank'
            ]);
        });

        test('is rejects a mistyped value', () => {
            assertStrictEquals(mod.Profile.is({ ...valid, age: '36' }), false);
        });
    });

    describe('Foreign-type fields', () => {
        test('report a failed conversion under the field name', () => {
            assertEquals(errorFields(mod.Profile.decode({ ...valid, balance: 'nope' })), [
                'balance'
            ]);
            assertEquals(errorFields(mod.Profile.decode({ ...valid, site: 'not a url' })), [
                'site'
            ]);
        });
    });

    describe('String-keyed brand', () => {
        test('checks the base primitive', () => {
            assertValidationSuccess(mod.Tagged.decode(4), 'Tagged');
            assertStrictEquals(mod.Tagged.decode({ value: 4 }).success, false);
            assertStrictEquals(mod.Tagged.is('x'), false);
            assertStrictEquals(mod.Tagged.is(4), true);
        });
    });

    describe('BigInt newtype', () => {
        test('hashes and renders without throwing', () => {
            assertStrictEquals(mod.Cents.hashCode(12n), mod.Cents.hashCode(12n));
            assertStrictEquals(mod.Cents.hashCode(12n) === mod.Cents.hashCode(13n), false);
            assertStrictEquals(mod.Cents.toString(12n), 'Cents(12)');
        });
    });
});
