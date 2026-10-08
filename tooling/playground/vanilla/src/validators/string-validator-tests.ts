/**
 * String validator test classes for comprehensive decoder validation testing.
 * Each class tests a single validator for isolation.
 */

// Email validator
/** @derive(Decode) */
export class EmailValidator {
    /** @endec({ validate: ["email"] }) */
    email: string;
}

// URL validator
/** @derive(Decode) */
export class UrlValidator {
    /** @endec({ validate: ["url"] }) */
    url: string;
}

// UUID validator
/** @derive(Decode) */
export class UuidValidator {
    /** @endec({ validate: ["uuid"] }) */
    id: string;
}

// MaxLength validator
/** @derive(Decode) */
export class MaxLengthValidator {
    /** @endec({ validate: ["maxLength(10)"] }) */
    shortText: string;
}

// MinLength validator
/** @derive(Decode) */
export class MinLengthValidator {
    /** @endec({ validate: ["minLength(5)"] }) */
    longText: string;
}

// Length validator (exact)
/** @derive(Decode) */
export class LengthValidator {
    /** @endec({ validate: ["length(8)"] }) */
    fixedText: string;
}

// LengthRange validator (use length with 2 args)
/** @derive(Decode) */
export class LengthRangeValidator {
    /** @endec({ validate: ["length(5, 10)"] }) */
    rangedText: string;
}

// Pattern validator
/** @derive(Decode) */
export class PatternValidator {
    /** @endec({ validate: ['pattern("^[A-Z]{3}$")'] }) */
    code: string;
}

// NonEmpty validator
/** @derive(Decode) */
export class NonEmptyValidator {
    /** @endec({ validate: ["nonEmpty"] }) */
    required: string;
}

// Trimmed validator
/** @derive(Decode) */
export class TrimmedValidator {
    /** @endec({ validate: ["trimmed"] }) */
    trimmed: string;
}

// Lowercase validator
/** @derive(Decode) */
export class LowercaseValidator {
    /** @endec({ validate: ["lowercase"] }) */
    lower: string;
}

// Uppercase validator
/** @derive(Decode) */
export class UppercaseValidator {
    /** @endec({ validate: ["uppercase"] }) */
    upper: string;
}

// Capitalized validator
/** @derive(Decode) */
export class CapitalizedValidator {
    /** @endec({ validate: ["capitalized"] }) */
    cap: string;
}

// Uncapitalized validator
/** @derive(Decode) */
export class UncapitalizedValidator {
    /** @endec({ validate: ["uncapitalized"] }) */
    uncap: string;
}

// StartsWith validator
/** @derive(Decode) */
export class StartsWithValidator {
    /** @endec({ validate: ['startsWith("https://")'] }) */
    secureUrl: string;
}

// EndsWith validator
/** @derive(Decode) */
export class EndsWithValidator {
    /** @endec({ validate: ['endsWith(".json")'] }) */
    filename: string;
}

// Includes validator
/** @derive(Decode) */
export class IncludesValidator {
    /** @endec({ validate: ['includes("@")'] }) */
    emailLike: string;
}
