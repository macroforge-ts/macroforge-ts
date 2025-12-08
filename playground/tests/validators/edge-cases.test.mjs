import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "edge-case-tests";

describe("Edge Cases", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // Multiple Validators on Single Field
  // ============================================================================
  describe("MultipleValidators", () => {
    test("accepts valid value passing all validators", () => {
      const result = mod.MultipleValidatorsTest.fromJSON({ text: "Hello World" });
      assertValidationSuccess(result, "text");
    });

    test("accepts value at max length boundary", () => {
      const result = mod.MultipleValidatorsTest.fromJSON({ text: "a".repeat(100) });
      assertValidationSuccess(result, "text");
    });

    test("rejects empty string (fails nonEmpty)", () => {
      const result = mod.MultipleValidatorsTest.fromJSON({ text: "" });
      assertValidationError(result, "text", "must not be empty");
    });

    test("rejects string exceeding max length", () => {
      const result = mod.MultipleValidatorsTest.fromJSON({ text: "a".repeat(101) });
      assertValidationError(result, "text", "must have at most 100 characters");
    });

    test("rejects untrimmed string", () => {
      const result = mod.MultipleValidatorsTest.fromJSON({ text: " hello " });
      assertValidationError(result, "text", "must be trimmed");
    });
  });

  // ============================================================================
  // Custom Error Message
  // ============================================================================
  describe("CustomMessage", () => {
    test("accepts valid email", () => {
      const result = mod.CustomMessageTest.fromJSON({ email: "test@example.com" });
      assertValidationSuccess(result, "email");
    });

    test("rejects invalid email with custom message", () => {
      const result = mod.CustomMessageTest.fromJSON({ email: "not-an-email" });
      assertValidationError(result, "email", "Please enter a valid email address");
    });
  });

  // ============================================================================
  // Mixed Validators with Custom Message
  // ============================================================================
  describe("MixedValidators", () => {
    test("accepts valid email", () => {
      const result = mod.MixedValidatorsTest.fromJSON({ email: "user@domain.com" });
      assertValidationSuccess(result, "email");
    });

    test("rejects empty string", () => {
      const result = mod.MixedValidatorsTest.fromJSON({ email: "" });
      assertValidationError(result, "email", "must not be empty");
    });

    test("rejects invalid email with custom message", () => {
      const result = mod.MixedValidatorsTest.fromJSON({ email: "invalid" });
      assertValidationError(result, "email", "Invalid email format");
    });
  });

  // ============================================================================
  // Combined String Validators
  // ============================================================================
  describe("CombinedStringValidators", () => {
    test("accepts valid lowercase username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "johndoe" });
      assertValidationSuccess(result, "username");
    });

    test("accepts minimum length username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "abc" });
      assertValidationSuccess(result, "username");
    });

    test("accepts maximum length username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "a".repeat(20) });
      assertValidationSuccess(result, "username");
    });

    test("rejects too short username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "ab" });
      assertValidationError(result, "username", "must have at least 3 characters");
    });

    test("rejects too long username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "a".repeat(21) });
      assertValidationError(result, "username", "must have at most 20 characters");
    });

    test("rejects uppercase username", () => {
      const result = mod.CombinedStringValidatorsTest.fromJSON({ username: "JohnDoe" });
      assertValidationError(result, "username", "must be lowercase");
    });
  });

  // ============================================================================
  // Combined Number Validators
  // ============================================================================
  describe("CombinedNumberValidators", () => {
    test("accepts valid positive integer", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 500 });
      assertValidationSuccess(result, "score");
    });

    test("accepts minimum positive integer", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 1 });
      assertValidationSuccess(result, "score");
    });

    test("accepts value just below max", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 999 });
      assertValidationSuccess(result, "score");
    });

    test("rejects zero (fails positive)", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 0 });
      assertValidationError(result, "score", "must be positive");
    });

    test("rejects negative number", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: -10 });
      assertValidationError(result, "score", "must be positive");
    });

    test("rejects float (fails int)", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 50.5 });
      assertValidationError(result, "score", "must be an integer");
    });

    test("rejects value at threshold (fails lessThan)", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 1000 });
      assertValidationError(result, "score", "must be less than");
    });

    test("rejects value above threshold", () => {
      const result = mod.CombinedNumberValidatorsTest.fromJSON({ score: 1500 });
      assertValidationError(result, "score", "must be less than");
    });
  });
});
