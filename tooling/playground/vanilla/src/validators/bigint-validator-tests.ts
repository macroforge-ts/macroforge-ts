/**
 * BigInt validator test classes for comprehensive decoder validation testing.
 */

// GreaterThanBigInt validator
/** @derive(Decode) */
export class GreaterThanBigIntValidator {
    /** @endec({ validate: ["greaterThanBigInt(0)"] }) */
    value: bigint;
}

// GreaterThanOrEqualToBigInt validator
/** @derive(Decode) */
export class GreaterThanOrEqualToBigIntValidator {
    /** @endec({ validate: ["greaterThanOrEqualToBigInt(0)"] }) */
    value: bigint;
}

// LessThanBigInt validator
/** @derive(Decode) */
export class LessThanBigIntValidator {
    /** @endec({ validate: ["lessThanBigInt(1000)"] }) */
    value: bigint;
}

// LessThanOrEqualToBigInt validator
/** @derive(Decode) */
export class LessThanOrEqualToBigIntValidator {
    /** @endec({ validate: ["lessThanOrEqualToBigInt(1000)"] }) */
    value: bigint;
}

// BetweenBigInt validator
/** @derive(Decode) */
export class BetweenBigIntValidator {
    /** @endec({ validate: ["betweenBigInt(0, 1000)"] }) */
    value: bigint;
}

// PositiveBigInt validator
/** @derive(Decode) */
export class PositiveBigIntValidator {
    /** @endec({ validate: ["positiveBigInt"] }) */
    value: bigint;
}

// NonNegativeBigInt validator
/** @derive(Decode) */
export class NonNegativeBigIntValidator {
    /** @endec({ validate: ["nonNegativeBigInt"] }) */
    value: bigint;
}

// NegativeBigInt validator
/** @derive(Decode) */
export class NegativeBigIntValidator {
    /** @endec({ validate: ["negativeBigInt"] }) */
    value: bigint;
}

// NonPositiveBigInt validator
/** @derive(Decode) */
export class NonPositiveBigIntValidator {
    /** @endec({ validate: ["nonPositiveBigInt"] }) */
    value: bigint;
}
