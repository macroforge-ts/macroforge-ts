/**
 * Number validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThan validator
/** @derive(Deserialize) */
export class GreaterThanValidator {
    /** @serde({ validate: ["greaterThan(0)"] }) */
    positive: number;
}

// GreaterThanOrEqualTo validator
/** @derive(Deserialize) */
export class GreaterThanOrEqualToValidator {
    /** @serde({ validate: ["greaterThanOrEqualTo(0)"] }) */
    nonNegative: number;
}

// LessThan validator
/** @derive(Deserialize) */
export class LessThanValidator {
    /** @serde({ validate: ["lessThan(100)"] }) */
    capped: number;
}

// LessThanOrEqualTo validator
/** @derive(Deserialize) */
export class LessThanOrEqualToValidator {
    /** @serde({ validate: ["lessThanOrEqualTo(100)"] }) */
    maxed: number;
}

// Between validator
/** @derive(Deserialize) */
export class BetweenValidator {
    /** @serde({ validate: ["between(1, 100)"] }) */
    ranged: number;
}

// Int validator
/** @derive(Deserialize) */
export class IntValidator {
    /** @serde({ validate: ["int"] }) */
    integer: number;
}

// NonNaN validator
/** @derive(Deserialize) */
export class NonNaNValidator {
    /** @serde({ validate: ["nonNaN"] }) */
    valid: number;
}

// Finite validator
/** @derive(Deserialize) */
export class FiniteValidator {
    /** @serde({ validate: ["finite"] }) */
    finite: number;
}

// Positive validator
/** @derive(Deserialize) */
export class PositiveValidator {
    /** @serde({ validate: ["positive"] }) */
    positive: number;
}

// NonNegative validator
/** @derive(Deserialize) */
export class NonNegativeValidator {
    /** @serde({ validate: ["nonNegative"] }) */
    nonNegative: number;
}

// Negative validator
/** @derive(Deserialize) */
export class NegativeValidator {
    /** @serde({ validate: ["negative"] }) */
    negative: number;
}

// NonPositive validator
/** @derive(Deserialize) */
export class NonPositiveValidator {
    /** @serde({ validate: ["nonPositive"] }) */
    nonPositive: number;
}

// MultipleOf validator
/** @derive(Deserialize) */
export class MultipleOfValidator {
    /** @serde({ validate: ["multipleOf(5)"] }) */
    multiple: number;
}

// Uint8 validator
/** @derive(Deserialize) */
export class Uint8Validator {
    /** @serde({ validate: ["uint8"] }) */
    byte: number;
}
