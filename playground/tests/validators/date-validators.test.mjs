import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "date-validator-tests";

describe("Date Validators", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // ValidDate Validator
  // ============================================================================
  describe("ValidDate", () => {
    test("accepts valid ISO date string", () => {
      const result = mod.ValidDateValidator.fromJSON({ date: "2023-06-15" });
      assertValidationSuccess(result, "date");
    });

    test("accepts valid date with time", () => {
      const result = mod.ValidDateValidator.fromJSON({ date: "2023-06-15T10:30:00" });
      assertValidationSuccess(result, "date");
    });

    test("rejects invalid date string", () => {
      const result = mod.ValidDateValidator.fromJSON({ date: "not-a-date" });
      assertValidationError(result, "date", "must be a valid date");
    });

    test("rejects invalid date format", () => {
      const result = mod.ValidDateValidator.fromJSON({ date: "2023-13-45" });
      assertValidationError(result, "date", "must be a valid date");
    });
  });

  // ============================================================================
  // GreaterThanDate Validator
  // ============================================================================
  describe("GreaterThanDate", () => {
    test("accepts date after threshold", () => {
      const result = mod.GreaterThanDateValidator.fromJSON({ date: "2021-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts far future date", () => {
      const result = mod.GreaterThanDateValidator.fromJSON({ date: "2099-12-31" });
      assertValidationSuccess(result, "date");
    });

    test("rejects date on threshold", () => {
      const result = mod.GreaterThanDateValidator.fromJSON({ date: "2020-01-01" });
      assertValidationError(result, "date", "must be after");
    });

    test("rejects date before threshold", () => {
      const result = mod.GreaterThanDateValidator.fromJSON({ date: "2019-06-15" });
      assertValidationError(result, "date", "must be after");
    });
  });

  // ============================================================================
  // GreaterThanOrEqualToDate Validator
  // ============================================================================
  describe("GreaterThanOrEqualToDate", () => {
    test("accepts date on threshold", () => {
      const result = mod.GreaterThanOrEqualToDateValidator.fromJSON({ date: "2020-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts date after threshold", () => {
      const result = mod.GreaterThanOrEqualToDateValidator.fromJSON({ date: "2021-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("rejects date before threshold", () => {
      const result = mod.GreaterThanOrEqualToDateValidator.fromJSON({ date: "2019-06-15" });
      assertValidationError(result, "date", "must be on or after");
    });
  });

  // ============================================================================
  // LessThanDate Validator
  // ============================================================================
  describe("LessThanDate", () => {
    test("accepts date before threshold", () => {
      const result = mod.LessThanDateValidator.fromJSON({ date: "2029-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts far past date", () => {
      const result = mod.LessThanDateValidator.fromJSON({ date: "2000-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("rejects date on threshold", () => {
      const result = mod.LessThanDateValidator.fromJSON({ date: "2030-01-01" });
      assertValidationError(result, "date", "must be before");
    });

    test("rejects date after threshold", () => {
      const result = mod.LessThanDateValidator.fromJSON({ date: "2031-01-01" });
      assertValidationError(result, "date", "must be before");
    });
  });

  // ============================================================================
  // LessThanOrEqualToDate Validator
  // ============================================================================
  describe("LessThanOrEqualToDate", () => {
    test("accepts date on threshold", () => {
      const result = mod.LessThanOrEqualToDateValidator.fromJSON({ date: "2030-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts date before threshold", () => {
      const result = mod.LessThanOrEqualToDateValidator.fromJSON({ date: "2029-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("rejects date after threshold", () => {
      const result = mod.LessThanOrEqualToDateValidator.fromJSON({ date: "2031-01-01" });
      assertValidationError(result, "date", "must be on or before");
    });
  });

  // ============================================================================
  // BetweenDate Validator
  // ============================================================================
  describe("BetweenDate", () => {
    test("accepts date at min boundary", () => {
      const result = mod.BetweenDateValidator.fromJSON({ date: "2020-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts date at max boundary", () => {
      const result = mod.BetweenDateValidator.fromJSON({ date: "2030-01-01" });
      assertValidationSuccess(result, "date");
    });

    test("accepts date in middle", () => {
      const result = mod.BetweenDateValidator.fromJSON({ date: "2025-06-15" });
      assertValidationSuccess(result, "date");
    });

    test("rejects date before min", () => {
      const result = mod.BetweenDateValidator.fromJSON({ date: "2019-06-15" });
      assertValidationError(result, "date", "must be between");
    });

    test("rejects date after max", () => {
      const result = mod.BetweenDateValidator.fromJSON({ date: "2031-01-01" });
      assertValidationError(result, "date", "must be between");
    });
  });
});
