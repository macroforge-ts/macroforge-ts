import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "number-validator-tests";

describe("Number Validators", () => {
  let mod;

  before(async () => {
    mod = await loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // GreaterThan Validator
  // ============================================================================
  describe("GreaterThan", () => {
    test("accepts value greater than threshold", () => {
      const result = mod.GreaterThanValidator.fromStringifiedJSON(JSON.stringify({ positive: 1 }));
      assertValidationSuccess(result, "positive");
    });

    test("accepts large positive value", () => {
      const result = mod.GreaterThanValidator.fromStringifiedJSON(JSON.stringify({ positive: 1000 }));
      assertValidationSuccess(result, "positive");
    });

    test("rejects value equal to threshold", () => {
      const result = mod.GreaterThanValidator.fromStringifiedJSON(JSON.stringify({ positive: 0 }));
      assertValidationError(result, "positive", "must be greater than");
    });

    test("rejects value below threshold", () => {
      const result = mod.GreaterThanValidator.fromStringifiedJSON(JSON.stringify({ positive: -1 }));
      assertValidationError(result, "positive", "must be greater than");
    });
  });

  // ============================================================================
  // GreaterThanOrEqualTo Validator
  // ============================================================================
  describe("GreaterThanOrEqualTo", () => {
    test("accepts value equal to threshold", () => {
      const result = mod.GreaterThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: 0 }));
      assertValidationSuccess(result, "nonNegative");
    });

    test("accepts value greater than threshold", () => {
      const result = mod.GreaterThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: 1 }));
      assertValidationSuccess(result, "nonNegative");
    });

    test("rejects value below threshold", () => {
      const result = mod.GreaterThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: -1 }));
      assertValidationError(result, "nonNegative", "must be greater than or equal to");
    });
  });

  // ============================================================================
  // LessThan Validator
  // ============================================================================
  describe("LessThan", () => {
    test("accepts value less than threshold", () => {
      const result = mod.LessThanValidator.fromStringifiedJSON(JSON.stringify({ capped: 99 }));
      assertValidationSuccess(result, "capped");
    });

    test("accepts negative value", () => {
      const result = mod.LessThanValidator.fromStringifiedJSON(JSON.stringify({ capped: -10 }));
      assertValidationSuccess(result, "capped");
    });

    test("rejects value equal to threshold", () => {
      const result = mod.LessThanValidator.fromStringifiedJSON(JSON.stringify({ capped: 100 }));
      assertValidationError(result, "capped", "must be less than");
    });

    test("rejects value greater than threshold", () => {
      const result = mod.LessThanValidator.fromStringifiedJSON(JSON.stringify({ capped: 101 }));
      assertValidationError(result, "capped", "must be less than");
    });
  });

  // ============================================================================
  // LessThanOrEqualTo Validator
  // ============================================================================
  describe("LessThanOrEqualTo", () => {
    test("accepts value equal to threshold", () => {
      const result = mod.LessThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ maxed: 100 }));
      assertValidationSuccess(result, "maxed");
    });

    test("accepts value less than threshold", () => {
      const result = mod.LessThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ maxed: 50 }));
      assertValidationSuccess(result, "maxed");
    });

    test("rejects value greater than threshold", () => {
      const result = mod.LessThanOrEqualToValidator.fromStringifiedJSON(JSON.stringify({ maxed: 101 }));
      assertValidationError(result, "maxed", "must be less than or equal to");
    });
  });

  // ============================================================================
  // Between Validator
  // ============================================================================
  describe("Between", () => {
    test("accepts value at min boundary", () => {
      const result = mod.BetweenValidator.fromStringifiedJSON(JSON.stringify({ ranged: 1 }));
      assertValidationSuccess(result, "ranged");
    });

    test("accepts value at max boundary", () => {
      const result = mod.BetweenValidator.fromStringifiedJSON(JSON.stringify({ ranged: 100 }));
      assertValidationSuccess(result, "ranged");
    });

    test("accepts value in middle", () => {
      const result = mod.BetweenValidator.fromStringifiedJSON(JSON.stringify({ ranged: 50 }));
      assertValidationSuccess(result, "ranged");
    });

    test("rejects value below min", () => {
      const result = mod.BetweenValidator.fromStringifiedJSON(JSON.stringify({ ranged: 0 }));
      assertValidationError(result, "ranged", "must be between");
    });

    test("rejects value above max", () => {
      const result = mod.BetweenValidator.fromStringifiedJSON(JSON.stringify({ ranged: 101 }));
      assertValidationError(result, "ranged", "must be between");
    });
  });

  // ============================================================================
  // Int Validator
  // ============================================================================
  describe("Int", () => {
    test("accepts positive integer", () => {
      const result = mod.IntValidator.fromStringifiedJSON(JSON.stringify({ integer: 42 }));
      assertValidationSuccess(result, "integer");
    });

    test("accepts negative integer", () => {
      const result = mod.IntValidator.fromStringifiedJSON(JSON.stringify({ integer: -42 }));
      assertValidationSuccess(result, "integer");
    });

    test("accepts zero", () => {
      const result = mod.IntValidator.fromStringifiedJSON(JSON.stringify({ integer: 0 }));
      assertValidationSuccess(result, "integer");
    });

    test("rejects float", () => {
      const result = mod.IntValidator.fromStringifiedJSON(JSON.stringify({ integer: 42.5 }));
      assertValidationError(result, "integer", "must be an integer");
    });

    test("rejects negative float", () => {
      const result = mod.IntValidator.fromStringifiedJSON(JSON.stringify({ integer: -3.14 }));
      assertValidationError(result, "integer", "must be an integer");
    });
  });

  // ============================================================================
  // NonNaN Validator
  // ============================================================================
  describe("NonNaN", () => {
    test("accepts regular number", () => {
      const result = mod.NonNaNValidator.fromStringifiedJSON(JSON.stringify({ valid: 42 }));
      assertValidationSuccess(result, "valid");
    });

    test("accepts zero", () => {
      const result = mod.NonNaNValidator.fromStringifiedJSON(JSON.stringify({ valid: 0 }));
      assertValidationSuccess(result, "valid");
    });

    test("accepts negative number", () => {
      const result = mod.NonNaNValidator.fromStringifiedJSON(JSON.stringify({ valid: -42 }));
      assertValidationSuccess(result, "valid");
    });
  });

  // ============================================================================
  // Finite Validator
  // ============================================================================
  describe("Finite", () => {
    test("accepts regular number", () => {
      const result = mod.FiniteValidator.fromStringifiedJSON(JSON.stringify({ finite: 42 }));
      assertValidationSuccess(result, "finite");
    });

    test("accepts zero", () => {
      const result = mod.FiniteValidator.fromStringifiedJSON(JSON.stringify({ finite: 0 }));
      assertValidationSuccess(result, "finite");
    });

    test("accepts large number", () => {
      const result = mod.FiniteValidator.fromStringifiedJSON(JSON.stringify({ finite: 1e100 }));
      assertValidationSuccess(result, "finite");
    });
  });

  // ============================================================================
  // Positive Validator
  // ============================================================================
  describe("Positive", () => {
    test("accepts positive number", () => {
      const result = mod.PositiveValidator.fromStringifiedJSON(JSON.stringify({ positive: 1 }));
      assertValidationSuccess(result, "positive");
    });

    test("accepts large positive number", () => {
      const result = mod.PositiveValidator.fromStringifiedJSON(JSON.stringify({ positive: 1000000 }));
      assertValidationSuccess(result, "positive");
    });

    test("rejects zero", () => {
      const result = mod.PositiveValidator.fromStringifiedJSON(JSON.stringify({ positive: 0 }));
      assertValidationError(result, "positive", "must be positive");
    });

    test("rejects negative number", () => {
      const result = mod.PositiveValidator.fromStringifiedJSON(JSON.stringify({ positive: -1 }));
      assertValidationError(result, "positive", "must be positive");
    });
  });

  // ============================================================================
  // NonNegative Validator
  // ============================================================================
  describe("NonNegative", () => {
    test("accepts zero", () => {
      const result = mod.NonNegativeValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: 0 }));
      assertValidationSuccess(result, "nonNegative");
    });

    test("accepts positive number", () => {
      const result = mod.NonNegativeValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: 1 }));
      assertValidationSuccess(result, "nonNegative");
    });

    test("rejects negative number", () => {
      const result = mod.NonNegativeValidator.fromStringifiedJSON(JSON.stringify({ nonNegative: -1 }));
      assertValidationError(result, "nonNegative", "must be non-negative");
    });
  });

  // ============================================================================
  // Negative Validator
  // ============================================================================
  describe("Negative", () => {
    test("accepts negative number", () => {
      const result = mod.NegativeValidator.fromStringifiedJSON(JSON.stringify({ negative: -1 }));
      assertValidationSuccess(result, "negative");
    });

    test("accepts large negative number", () => {
      const result = mod.NegativeValidator.fromStringifiedJSON(JSON.stringify({ negative: -1000000 }));
      assertValidationSuccess(result, "negative");
    });

    test("rejects zero", () => {
      const result = mod.NegativeValidator.fromStringifiedJSON(JSON.stringify({ negative: 0 }));
      assertValidationError(result, "negative", "must be negative");
    });

    test("rejects positive number", () => {
      const result = mod.NegativeValidator.fromStringifiedJSON(JSON.stringify({ negative: 1 }));
      assertValidationError(result, "negative", "must be negative");
    });
  });

  // ============================================================================
  // NonPositive Validator
  // ============================================================================
  describe("NonPositive", () => {
    test("accepts zero", () => {
      const result = mod.NonPositiveValidator.fromStringifiedJSON(JSON.stringify({ nonPositive: 0 }));
      assertValidationSuccess(result, "nonPositive");
    });

    test("accepts negative number", () => {
      const result = mod.NonPositiveValidator.fromStringifiedJSON(JSON.stringify({ nonPositive: -1 }));
      assertValidationSuccess(result, "nonPositive");
    });

    test("rejects positive number", () => {
      const result = mod.NonPositiveValidator.fromStringifiedJSON(JSON.stringify({ nonPositive: 1 }));
      assertValidationError(result, "nonPositive", "must be non-positive");
    });
  });

  // ============================================================================
  // MultipleOf Validator
  // ============================================================================
  describe("MultipleOf", () => {
    test("accepts multiple of 5", () => {
      const result = mod.MultipleOfValidator.fromStringifiedJSON(JSON.stringify({ multiple: 15 }));
      assertValidationSuccess(result, "multiple");
    });

    test("accepts zero", () => {
      const result = mod.MultipleOfValidator.fromStringifiedJSON(JSON.stringify({ multiple: 0 }));
      assertValidationSuccess(result, "multiple");
    });

    test("accepts negative multiple", () => {
      const result = mod.MultipleOfValidator.fromStringifiedJSON(JSON.stringify({ multiple: -10 }));
      assertValidationSuccess(result, "multiple");
    });

    test("rejects non-multiple", () => {
      const result = mod.MultipleOfValidator.fromStringifiedJSON(JSON.stringify({ multiple: 17 }));
      assertValidationError(result, "multiple", "must be a multiple of");
    });
  });

  // ============================================================================
  // Uint8 Validator
  // ============================================================================
  describe("Uint8", () => {
    test("accepts 0", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: 0 }));
      assertValidationSuccess(result, "byte");
    });

    test("accepts 255", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: 255 }));
      assertValidationSuccess(result, "byte");
    });

    test("accepts middle value", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: 128 }));
      assertValidationSuccess(result, "byte");
    });

    test("rejects -1", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: -1 }));
      assertValidationError(result, "byte", "must be a uint8");
    });

    test("rejects 256", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: 256 }));
      assertValidationError(result, "byte", "must be a uint8");
    });

    test("rejects non-integer", () => {
      const result = mod.Uint8Validator.fromStringifiedJSON(JSON.stringify({ byte: 100.5 }));
      assertValidationError(result, "byte", "must be a uint8");
    });
  });
});
