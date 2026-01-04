/**
 * BigInt validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThanBigInt validator
/** @derive(Deserialize) */
export class GreaterThanBigIntValidator {
    /** @serde({ validate: ["greaterThanBigInt(0)"] }) */
    value: bigint;
}

// GreaterThanOrEqualToBigInt validator
/** @derive(Deserialize) */
export class GreaterThanOrEqualToBigIntValidator {
    /** @serde({ validate: ["greaterThanOrEqualToBigInt(0)"] }) */
    value: bigint;
}

// LessThanBigInt validator
/** @derive(Deserialize) */
export class LessThanBigIntValidator {
    /** @serde({ validate: ["lessThanBigInt(1000)"] }) */
    value: bigint;
}

// LessThanOrEqualToBigInt validator
/** @derive(Deserialize) */
export class LessThanOrEqualToBigIntValidator {
    /** @serde({ validate: ["lessThanOrEqualToBigInt(1000)"] }) */
    value: bigint;
}

// BetweenBigInt validator
/** @derive(Deserialize) */
export class BetweenBigIntValidator {
    /** @serde({ validate: ["betweenBigInt(0, 1000)"] }) */
    value: bigint;
}

// PositiveBigInt validator
/** @derive(Deserialize) */
export class PositiveBigIntValidator {
    /** @serde({ validate: ["positiveBigInt"] }) */
    value: bigint;
}

// NonNegativeBigInt validator
/** @derive(Deserialize) */
export class NonNegativeBigIntValidator {
    /** @serde({ validate: ["nonNegativeBigInt"] }) */
    value: bigint;
}

// NegativeBigInt validator
/** @derive(Deserialize) */
export class NegativeBigIntValidator {
    /** @serde({ validate: ["negativeBigInt"] }) */
    value: bigint;
}

// NonPositiveBigInt validator
/** @derive(Deserialize) */
export class NonPositiveBigIntValidator {
    /** @serde({ validate: ["nonPositiveBigInt"] }) */
    value: bigint;
}
