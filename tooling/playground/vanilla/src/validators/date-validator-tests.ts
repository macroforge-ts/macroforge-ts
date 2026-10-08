/**
 * Date validator test classes for comprehensive decoder validation testing.
 */

// ValidDate validator
/** @derive(Decode) */
export class ValidDateValidator {
    /** @endec({ validate: ["validDate"] }) */
    date: Date;
}

// GreaterThanDate validator
/** @derive(Decode) */
export class GreaterThanDateValidator {
    /** @endec({ validate: ['greaterThanDate("2020-01-01")'] }) */
    date: Date;
}

// GreaterThanOrEqualToDate validator
/** @derive(Decode) */
export class GreaterThanOrEqualToDateValidator {
    /** @endec({ validate: ['greaterThanOrEqualToDate("2020-01-01")'] }) */
    date: Date;
}

// LessThanDate validator
/** @derive(Decode) */
export class LessThanDateValidator {
    /** @endec({ validate: ['lessThanDate("2030-01-01")'] }) */
    date: Date;
}

// LessThanOrEqualToDate validator
/** @derive(Decode) */
export class LessThanOrEqualToDateValidator {
    /** @endec({ validate: ['lessThanOrEqualToDate("2030-01-01")'] }) */
    date: Date;
}

// BetweenDate validator
/** @derive(Decode) */
export class BetweenDateValidator {
    /** @endec({ validate: ['betweenDate("2020-01-01", "2030-01-01")'] }) */
    date: Date;
}
