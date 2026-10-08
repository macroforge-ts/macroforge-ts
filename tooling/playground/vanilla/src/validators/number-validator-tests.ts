/**
 * Number validator test classes for comprehensive decoder validation testing.
 */

// GreaterThan validator
/** @derive(Decode) */
export class GreaterThanValidator {
    /** @endec({ validate: ["greaterThan(0)"] }) */
    positive: number;
}

// GreaterThanOrEqualTo validator
/** @derive(Decode) */
export class GreaterThanOrEqualToValidator {
    /** @endec({ validate: ["greaterThanOrEqualTo(0)"] }) */
    nonNegative: number;
}

// LessThan validator
/** @derive(Decode) */
export class LessThanValidator {
    /** @endec({ validate: ["lessThan(100)"] }) */
    capped: number;
}

// LessThanOrEqualTo validator
/** @derive(Decode) */
export class LessThanOrEqualToValidator {
    /** @endec({ validate: ["lessThanOrEqualTo(100)"] }) */
    maxed: number;
}

// Between validator
/** @derive(Decode) */
export class BetweenValidator {
    /** @endec({ validate: ["between(1, 100)"] }) */
    ranged: number;
}

// Int validator
/** @derive(Decode) */
export class IntValidator {
    /** @endec({ validate: ["int"] }) */
    integer: number;
}

// NonNaN validator
/** @derive(Decode) */
export class NonNaNValidator {
    /** @endec({ validate: ["nonNaN"] }) */
    valid: number;
}

// Finite validator
/** @derive(Decode) */
export class FiniteValidator {
    /** @endec({ validate: ["finite"] }) */
    finite: number;
}

// Positive validator
/** @derive(Decode) */
export class PositiveValidator {
    /** @endec({ validate: ["positive"] }) */
    positive: number;
}

// NonNegative validator
/** @derive(Decode) */
export class NonNegativeValidator {
    /** @endec({ validate: ["nonNegative"] }) */
    nonNegative: number;
}

// Negative validator
/** @derive(Decode) */
export class NegativeValidator {
    /** @endec({ validate: ["negative"] }) */
    negative: number;
}

// NonPositive validator
/** @derive(Decode) */
export class NonPositiveValidator {
    /** @endec({ validate: ["nonPositive"] }) */
    nonPositive: number;
}

// MultipleOf validator
/** @derive(Decode) */
export class MultipleOfValidator {
    /** @endec({ validate: ["multipleOf(5)"] }) */
    multiple: number;
}

// Uint8 validator
/** @derive(Decode) */
export class Uint8Validator {
    /** @endec({ validate: ["uint8"] }) */
    byte: number;
}
