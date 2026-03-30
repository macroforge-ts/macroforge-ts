/**
 * String validator test classes for comprehensive deserializer validation testing.
 * Each class tests a single validator for isolation.
 */

// Email validator
/** @derive(Deserialize) */
export class EmailValidator {
  /** @serde({ validate: ["email"] }) */
  email: string;
}

// URL validator
/** @derive(Deserialize) */
export class UrlValidator {
  /** @serde({ validate: ["url"] }) */
  url: string;
}

// UUID validator
/** @derive(Deserialize) */
export class UuidValidator {
  /** @serde({ validate: ["uuid"] }) */
  id: string;
}

// MaxLength validator
/** @derive(Deserialize) */
export class MaxLengthValidator {
  /** @serde({ validate: ["maxLength(10)"] }) */
  shortText: string;
}

// MinLength validator
/** @derive(Deserialize) */
export class MinLengthValidator {
  /** @serde({ validate: ["minLength(5)"] }) */
  longText: string;
}

// Length validator (exact)
/** @derive(Deserialize) */
export class LengthValidator {
  /** @serde({ validate: ["length(8)"] }) */
  fixedText: string;
}

// LengthRange validator (use length with 2 args)
/** @derive(Deserialize) */
export class LengthRangeValidator {
  /** @serde({ validate: ["length(5, 10)"] }) */
  rangedText: string;
}

// Pattern validator
/** @derive(Deserialize) */
export class PatternValidator {
  /** @serde({ validate: ['pattern("^[A-Z]{3}$")'] }) */
  code: string;
}

// NonEmpty validator
/** @derive(Deserialize) */
export class NonEmptyValidator {
  /** @serde({ validate: ["nonEmpty"] }) */
  required: string;
}

// Trimmed validator
/** @derive(Deserialize) */
export class TrimmedValidator {
  /** @serde({ validate: ["trimmed"] }) */
  trimmed: string;
}

// Lowercase validator
/** @derive(Deserialize) */
export class LowercaseValidator {
  /** @serde({ validate: ["lowercase"] }) */
  lower: string;
}

// Uppercase validator
/** @derive(Deserialize) */
export class UppercaseValidator {
  /** @serde({ validate: ["uppercase"] }) */
  upper: string;
}

// Capitalized validator
/** @derive(Deserialize) */
export class CapitalizedValidator {
  /** @serde({ validate: ["capitalized"] }) */
  cap: string;
}

// Uncapitalized validator
/** @derive(Deserialize) */
export class UncapitalizedValidator {
  /** @serde({ validate: ["uncapitalized"] }) */
  uncap: string;
}

// StartsWith validator
/** @derive(Deserialize) */
export class StartsWithValidator {
  /** @serde({ validate: ['startsWith("https://")'] }) */
  secureUrl: string;
}

// EndsWith validator
/** @derive(Deserialize) */
export class EndsWithValidator {
  /** @serde({ validate: ['endsWith(".json")'] }) */
  filename: string;
}

// Includes validator
/** @derive(Deserialize) */
export class IncludesValidator {
  /** @serde({ validate: ['includes("@")'] }) */
  emailLike: string;
}
