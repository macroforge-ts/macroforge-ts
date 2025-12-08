import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "custom-validator-tests";

describe("Custom Validators", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // Custom isEven Validator (CustomNumberValidator)
  // ============================================================================
  describe("CustomNumberValidator (isEven)", () => {
    test("accepts even number", () => {
      const result = mod.CustomNumberValidator.fromJSON({ evenNumber: 2 });
      assertValidationSuccess(result, "evenNumber");
    });

    test("accepts zero", () => {
      const result = mod.CustomNumberValidator.fromJSON({ evenNumber: 0 });
      assertValidationSuccess(result, "evenNumber");
    });

    test("accepts negative even number", () => {
      const result = mod.CustomNumberValidator.fromJSON({ evenNumber: -4 });
      assertValidationSuccess(result, "evenNumber");
    });

    test("rejects odd number", () => {
      const result = mod.CustomNumberValidator.fromJSON({ evenNumber: 3 });
      assertValidationError(result, "evenNumber", "");
    });

    test("rejects negative odd number", () => {
      const result = mod.CustomNumberValidator.fromJSON({ evenNumber: -5 });
      assertValidationError(result, "evenNumber", "");
    });
  });

  // ============================================================================
  // Custom isValidUsername Validator (CustomStringValidator)
  // ============================================================================
  describe("CustomStringValidator (isValidUsername)", () => {
    test("accepts valid username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "john_doe" });
      assertValidationSuccess(result, "username");
    });

    test("accepts username with numbers", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "user123" });
      assertValidationSuccess(result, "username");
    });

    test("accepts 3 character username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "abc" });
      assertValidationSuccess(result, "username");
    });

    test("accepts 16 character username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "a".repeat(16) });
      assertValidationSuccess(result, "username");
    });

    test("rejects 2 character username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "ab" });
      assertValidationError(result, "username", "");
    });

    test("rejects 17 character username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "a".repeat(17) });
      assertValidationError(result, "username", "");
    });

    test("rejects username with spaces", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "john doe" });
      assertValidationError(result, "username", "");
    });

    test("rejects username starting with number", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "1john" });
      assertValidationError(result, "username", "");
    });

    test("rejects uppercase username", () => {
      const result = mod.CustomStringValidator.fromJSON({ username: "JohnDoe" });
      assertValidationError(result, "username", "");
    });
  });

  // ============================================================================
  // Custom with Custom Message Validator
  // ============================================================================
  describe("CustomWithMessageValidator", () => {
    test("accepts even number", () => {
      const result = mod.CustomWithMessageValidator.fromJSON({ evenNumber: 4 });
      assertValidationSuccess(result, "evenNumber");
    });

    test("rejects odd with custom message", () => {
      const result = mod.CustomWithMessageValidator.fromJSON({ evenNumber: 3 });
      assertValidationError(result, "evenNumber", "Number must be even");
    });
  });
});
