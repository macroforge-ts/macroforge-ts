/**
 * Every validator in the serde validators documentation, checked against what
 * the documentation says it does. Each row names the class from
 * vanilla/src/validators/documented-validators.ts, the values the documented
 * check accepts, and the values it rejects, with the boundary on both sides.
 *
 * Values are passed as parsed objects rather than JSON text, so non-finite
 * numbers, which JSON cannot carry, are checked too.
 */
import {
    assert,
    assertValidationError,
    assertValidationSuccess,
    before,
    describe,
    loadValidatorModule,
    test
} from './helpers.mjs';

const MODULE_NAME = 'documented-validators';

/** `[class, documented check, accepted values, rejected values]` */
const DOCUMENTED = [
    // String validators
    ['DocEmail', 'valid email address', ['a@b.co', 'first.last@example.com'], [
        'a@b',
        'a b@c.de',
        ''
    ]],
    ['DocUrl', 'valid URL', ['https://example.com/x', 'ftp://host/file', 'mailto:a@b.co'], [
        'not a url',
        'http://not a url',
        ''
    ]],
    ['DocUuid', 'valid UUID', [
        '123e4567-e89b-12d3-a456-426614174000',
        '123E4567-E89B-12D3-A456-426614174000'
    ], ['123e4567', '123e4567-e89b-12d3-a456-42661417400g']],
    ['DocPattern', 'matches the regular expression', ['ABC'], ['AB', 'ABCD', 'abc']],
    ['DocPatternWithSlash', 'a `/` in the pattern matches itself', ['a/b'], ['ab', 'a\\b']],
    ['DocMinLength', 'length >= n', ['abc', 'abcd'], ['ab', '']],
    ['DocMaxLength', 'length <= n', ['abc', ''], ['abcd']],
    ['DocLength', 'length exactly n', ['abc'], ['ab', 'abcd']],
    ['DocLengthRange', 'length within range', ['ab', 'abcd'], ['a', 'abcde']],
    ['DocNonEmpty', 'not the empty string', ['a', ' '], ['']],
    ['DocTrimmed', 'no leading/trailing whitespace', ['a b', ''], [' a', 'a ', '\ta']],
    ['DocLowercase', 'all lowercase', ['abc', 'a1-b'], ['aBc', 'ABC']],
    ['DocUppercase', 'all uppercase', ['ABC', 'A1-B'], ['AbC', 'abc']],
    ['DocCapitalized', 'first character uppercase', ['Hello', 'HELLO', 'H', ''], ['hello', 'h']],
    ['DocUncapitalized', 'first character lowercase', ['hello', 'hELLO', 'h', ''], ['Hello', 'H']],
    ['DocStartsWith', 'has the prefix', ['prefix', 'pre'], ['xpre', 'pr']],
    ['DocStartsWithQuote', 'a quote in the prefix is part of it', ['a"bc'], ['abc', 'a\\"bc']],
    ['DocEndsWith', 'has the suffix', ['suffix', 'fix'], ['fixed', 'fi']],
    ['DocIncludes', 'contains the substring', ['amidst', 'mid'], ['mi d', '']],

    // Number validators
    ['DocGreaterThan', '> n', [6, 5.0001], [5, 4]],
    ['DocGreaterThanOrEqualTo', '>= n', [5, 6], [4.999]],
    ['DocLessThan', '< n', [4, 4.999], [5, 6]],
    ['DocLessThanOrEqualTo', '<= n', [5, 4], [5.001]],
    ['DocBetween', 'within range, ends included', [1, 5.5, 10], [0.999, 10.001]],
    ['DocInt', 'integer', [3, -3, 0], [3.5]],
    ['DocNonNegativeInt', 'integer and >= 0', [0, 3], [-1, 2.5]],
    ['DocUint8', 'integer in 0-255', [0, 255], [-1, 256, 1.5]],
    ['DocMultipleOfDecimal', 'divisible by n, decimals exact', [0.3, 1, 0, -0.2], [
        0.25,
        Infinity,
        NaN
    ]],
    ['DocMultipleOfInteger', 'divisible by n', [15, 0, -5], [7]],
    ['DocPositive', '> 0', [1, 0.001], [0, -1]],
    ['DocNonNegative', '>= 0', [0, 1], [-0.001]],
    ['DocNegative', '< 0', [-1, -0.001], [0, 1]],
    ['DocNonPositive', '<= 0', [0, -1], [0.001]],
    ['DocFinite', 'not Infinity, -Infinity or NaN', [1, -1, 0], [Infinity, -Infinity, NaN]],
    ['DocNonNaN', 'not NaN', [1, Infinity], [NaN]],

    // BigInt validators (bigints arrive as strings)
    ['DocPositiveBigInt', '> 0n', ['1'], ['0', '-1']],
    ['DocNonNegativeBigInt', '>= 0n', ['0', '1'], ['-1']],
    ['DocNegativeBigInt', '< 0n', ['-1'], ['0', '1']],
    ['DocNonPositiveBigInt', '<= 0n', ['0', '-1'], ['1']],
    ['DocGreaterThanBigInt', '> n', ['6'], ['5']],
    ['DocGreaterThanOrEqualToBigInt', '>= n', ['5'], ['4']],
    ['DocLessThanBigInt', '< n', ['4'], ['5']],
    ['DocLessThanOrEqualToBigInt', '<= n', ['5'], ['6']],
    ['DocBetweenBigInt', 'within range, ends included', ['1', '10'], ['0', '11']],

    // Date validators
    ['DocValidDate', 'parses to a valid date', ['2024-01-01', '2024-01-01T10:30:00Z'], [
        'not a date'
    ]],
    ['DocGreaterThanDate', 'after d', ['2020-01-02'], ['2020-01-01', '2019-12-31']],
    ['DocGreaterThanOrEqualToDate', 'at or after d', ['2020-01-01', '2020-01-02'], ['2019-12-31']],
    ['DocLessThanDate', 'before d', ['2019-12-31'], ['2020-01-01']],
    ['DocLessThanOrEqualToDate', 'at or before d', ['2020-01-01', '2019-12-31'], ['2020-01-02']],
    ['DocBetweenDate', 'within range, ends included', ['2020-01-01', '2020-06-15', '2020-12-31'], [
        '2019-12-31',
        '2021-01-01'
    ]],

    // Array validators
    ['DocMinItems', 'at least n elements', [[1, 2], [1, 2, 3]], [[1], []]],
    ['DocMaxItems', 'at most n elements', [[1, 2], []], [[1, 2, 3]]],
    ['DocItemsCount', 'exactly n elements', [[1, 2]], [[1], [1, 2, 3]]],

    // A nullable field is checked only when it holds a value
    ['DocNullableMaxLength', 'null passes, a value is checked', [null, 'abc'], ['abcd']],
    ['DocNullableDate', 'null passes, a date is checked', [null, '2020-01-02'], ['2019-12-31']]
];

/** A value as the test names it: strings quoted, everything else as written. */
function describeValue(value) {
    if (typeof value === 'string' || value === null) return JSON.stringify(value);
    if (Array.isArray(value)) return `[${value.join(', ')}]`;
    return String(value);
}

describe('Documented validators', () => {
    let mod;

    before(async () => {
        mod = await loadValidatorModule(MODULE_NAME);
    });

    for (const [className, check, accepted, rejected] of DOCUMENTED) {
        describe(`${className.slice(3)}: ${check}`, () => {
            for (const value of accepted) {
                test(`accepts ${describeValue(value)}`, () => {
                    assertValidationSuccess(mod[className].deserialize({ value }), 'value');
                });
            }
            for (const value of rejected) {
                test(`rejects ${describeValue(value)}`, () => {
                    assertValidationError(mod[className].deserialize({ value }), 'value', '');
                });
            }
        });
    }

    test('a required field given null fails as required', () => {
        assertValidationError(
            mod.DocMaxLength.deserialize({ value: null }),
            'value',
            'is required'
        );
    });

    describe('validateField on a nullable field', () => {
        test('accepts null', () => {
            assert.deepStrictEqual(mod.DocNullableMaxLength.validateField('value', null), []);
        });

        test('checks a value', () => {
            assert.equal(mod.DocNullableMaxLength.validateField('value', 'abcd').length, 1);
        });
    });

    test('a custom message keeps its quotes and newline', () => {
        const result = mod.DocMessageEscaping.deserialize({ value: '' });
        assert.equal(result.success, false);
        assert.deepStrictEqual(result.errors.map((error) => error.message), [
            'say "hi"\nthen stop'
        ]);
    });
});
