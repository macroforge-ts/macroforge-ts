/**
 * Edge case test classes for comprehensive decoder validation testing.
 */

// Multiple validators on single field
/** @derive(Decode) */
export class MultipleValidatorsTest {
    /** @endec({ validate: ["nonEmpty", "maxLength(100)", "trimmed"] }) */
    text: string;
}

// Custom error message
/** @derive(Decode) */
export class CustomMessageTest {
    /** @endec({ validate: [{ validate: "email", message: "Please enter a valid email address" }] }) */
    email: string;
}

// Mixed validators with custom message
/** @derive(Decode) */
export class MixedValidatorsTest {
    /** @endec({ validate: ["nonEmpty", { validate: "email", message: "Invalid email format" }] }) */
    email: string;
}

// Combined string validators
/** @derive(Decode) */
export class CombinedStringValidatorsTest {
    /** @endec({ validate: ["minLength(3)", "maxLength(20)", "lowercase"] }) */
    username: string;
}

// Combined number validators
/** @derive(Decode) */
export class CombinedNumberValidatorsTest {
    /** @endec({ validate: ["int", "positive", "lessThan(1000)"] }) */
    score: number;
}
