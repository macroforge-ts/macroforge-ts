/**
 * Edge case test classes for comprehensive deserializer validation testing.
 */

// Multiple validators on single field
/** @derive(Deserialize) */
export class MultipleValidatorsTest {
  /** @serde({ validate: ["nonEmpty", "maxLength(100)", "trimmed"] }) */
  text: string;
}

// Custom error message
/** @derive(Deserialize) */
export class CustomMessageTest {
  /** @serde({ validate: [{ validate: "email", message: "Please enter a valid email address" }] }) */
  email: string;
}

// Mixed validators with custom message
/** @derive(Deserialize) */
export class MixedValidatorsTest {
  /** @serde({ validate: ["nonEmpty", { validate: "email", message: "Invalid email format" }] }) */
  email: string;
}

// Combined string validators
/** @derive(Deserialize) */
export class CombinedStringValidatorsTest {
  /** @serde({ validate: ["minLength(3)", "maxLength(20)", "lowercase"] }) */
  username: string;
}

// Combined number validators
/** @derive(Deserialize) */
export class CombinedNumberValidatorsTest {
  /** @serde({ validate: ["int", "positive", "lessThan(1000)"] }) */
  score: number;
}
