/**
 * Date validator test classes for comprehensive deserializer validation testing.
 */

// ValidDate validator
/** @derive(Deserialize) */
export class ValidDateValidator {
  /** @serde({ validate: ["validDate"] }) */
  date: Date;
}

// GreaterThanDate validator
/** @derive(Deserialize) */
export class GreaterThanDateValidator {
  /** @serde({ validate: ['greaterThanDate("2020-01-01")'] }) */
  date: Date;
}

// GreaterThanOrEqualToDate validator
/** @derive(Deserialize) */
export class GreaterThanOrEqualToDateValidator {
  /** @serde({ validate: ['greaterThanOrEqualToDate("2020-01-01")'] }) */
  date: Date;
}

// LessThanDate validator
/** @derive(Deserialize) */
export class LessThanDateValidator {
  /** @serde({ validate: ['lessThanDate("2030-01-01")'] }) */
  date: Date;
}

// LessThanOrEqualToDate validator
/** @derive(Deserialize) */
export class LessThanOrEqualToDateValidator {
  /** @serde({ validate: ['lessThanOrEqualToDate("2030-01-01")'] }) */
  date: Date;
}

// BetweenDate validator
/** @derive(Deserialize) */
export class BetweenDateValidator {
  /** @serde({ validate: ['betweenDate("2020-01-01", "2030-01-01")'] }) */
  date: Date;
}
