import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "bigint-validator-tests";

describe("BigInt Validators", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // GreaterThanBigInt Validator (threshold is 0)
  // ============================================================================
  describe("GreaterThanBigInt", () => {
    test("accepts value greater than threshold", () => {
      const result = mod.GreaterThanBigIntValidator.fromJSON({ value: "1" });
      assertValidationSuccess(result, "value");
    });

    test("accepts large value", () => {
      const result = mod.GreaterThanBigIntValidator.fromJSON({ value: "999999999999" });
      assertValidationSuccess(result, "value");
    });

    test("rejects value equal to threshold", () => {
      const result = mod.GreaterThanBigIntValidator.fromJSON({ value: "0" });
      assertValidationError(result, "value", "must be greater than");
    });

    test("rejects value below threshold", () => {
      const result = mod.GreaterThanBigIntValidator.fromJSON({ value: "-1" });
      assertValidationError(result, "value", "must be greater than");
    });
  });

  // ============================================================================
  // GreaterThanOrEqualToBigInt Validator (threshold is 0)
  // ============================================================================
  describe("GreaterThanOrEqualToBigInt", () => {
    test("accepts value equal to threshold", () => {
      const result = mod.GreaterThanOrEqualToBigIntValidator.fromJSON({ value: "0" });
      assertValidationSuccess(result, "value");
    });

    test("accepts value greater than threshold", () => {
      const result = mod.GreaterThanOrEqualToBigIntValidator.fromJSON({ value: "1" });
      assertValidationSuccess(result, "value");
    });

    test("rejects value below threshold", () => {
      const result = mod.GreaterThanOrEqualToBigIntValidator.fromJSON({ value: "-1" });
      assertValidationError(result, "value", "must be greater than or equal to");
    });
  });

  // ============================================================================
  // LessThanBigInt Validator (threshold is 1000)
  // ============================================================================
  describe("LessThanBigInt", () => {
    test("accepts value less than threshold", () => {
      const result = mod.LessThanBigIntValidator.fromJSON({ value: "999" });
      assertValidationSuccess(result, "value");
    });

    test("accepts negative value", () => {
      const result = mod.LessThanBigIntValidator.fromJSON({ value: "-100" });
      assertValidationSuccess(result, "value");
    });

    test("rejects value equal to threshold", () => {
      const result = mod.LessThanBigIntValidator.fromJSON({ value: "1000" });
      assertValidationError(result, "value", "must be less than");
    });

    test("rejects value above threshold", () => {
      const result = mod.LessThanBigIntValidator.fromJSON({ value: "1001" });
      assertValidationError(result, "value", "must be less than");
    });
  });

  // ============================================================================
  // LessThanOrEqualToBigInt Validator (threshold is 1000)
  // ============================================================================
  describe("LessThanOrEqualToBigInt", () => {
    test("accepts value equal to threshold", () => {
      const result = mod.LessThanOrEqualToBigIntValidator.fromJSON({ value: "1000" });
      assertValidationSuccess(result, "value");
    });

    test("accepts value less than threshold", () => {
      const result = mod.LessThanOrEqualToBigIntValidator.fromJSON({ value: "50" });
      assertValidationSuccess(result, "value");
    });

    test("rejects value above threshold", () => {
      const result = mod.LessThanOrEqualToBigIntValidator.fromJSON({ value: "1001" });
      assertValidationError(result, "value", "must be less than or equal to");
    });
  });

  // ============================================================================
  // BetweenBigInt Validator
  // ============================================================================
  describe("BetweenBigInt", () => {
    test("accepts value at min boundary", () => {
      const result = mod.BetweenBigIntValidator.fromJSON({ value: "0" });
      assertValidationSuccess(result, "value");
    });

    test("accepts value at max boundary", () => {
      const result = mod.BetweenBigIntValidator.fromJSON({ value: "1000" });
      assertValidationSuccess(result, "value");
    });

    test("accepts value in middle", () => {
      const result = mod.BetweenBigIntValidator.fromJSON({ value: "500" });
      assertValidationSuccess(result, "value");
    });

    test("rejects value below min", () => {
      const result = mod.BetweenBigIntValidator.fromJSON({ value: "-1" });
      assertValidationError(result, "value", "must be between");
    });

    test("rejects value above max", () => {
      const result = mod.BetweenBigIntValidator.fromJSON({ value: "1001" });
      assertValidationError(result, "value", "must be between");
    });
  });

  // ============================================================================
  // PositiveBigInt Validator
  // ============================================================================
  describe("PositiveBigInt", () => {
    test("accepts positive value", () => {
      const result = mod.PositiveBigIntValidator.fromJSON({ value: "1" });
      assertValidationSuccess(result, "value");
    });

    test("accepts large positive value", () => {
      const result = mod.PositiveBigIntValidator.fromJSON({ value: "999999999999999999" });
      assertValidationSuccess(result, "value");
    });

    test("rejects zero", () => {
      const result = mod.PositiveBigIntValidator.fromJSON({ value: "0" });
      assertValidationError(result, "value", "must be positive");
    });

    test("rejects negative value", () => {
      const result = mod.PositiveBigIntValidator.fromJSON({ value: "-1" });
      assertValidationError(result, "value", "must be positive");
    });
  });

  // ============================================================================
  // NonNegativeBigInt Validator
  // ============================================================================
  describe("NonNegativeBigInt", () => {
    test("accepts zero", () => {
      const result = mod.NonNegativeBigIntValidator.fromJSON({ value: "0" });
      assertValidationSuccess(result, "value");
    });

    test("accepts positive value", () => {
      const result = mod.NonNegativeBigIntValidator.fromJSON({ value: "100" });
      assertValidationSuccess(result, "value");
    });

    test("rejects negative value", () => {
      const result = mod.NonNegativeBigIntValidator.fromJSON({ value: "-1" });
      assertValidationError(result, "value", "must be non-negative");
    });
  });

  // ============================================================================
  // NegativeBigInt Validator
  // ============================================================================
  describe("NegativeBigInt", () => {
    test("accepts negative value", () => {
      const result = mod.NegativeBigIntValidator.fromJSON({ value: "-1" });
      assertValidationSuccess(result, "value");
    });

    test("accepts large negative value", () => {
      const result = mod.NegativeBigIntValidator.fromJSON({ value: "-999999999999999999" });
      assertValidationSuccess(result, "value");
    });

    test("rejects zero", () => {
      const result = mod.NegativeBigIntValidator.fromJSON({ value: "0" });
      assertValidationError(result, "value", "must be negative");
    });

    test("rejects positive value", () => {
      const result = mod.NegativeBigIntValidator.fromJSON({ value: "1" });
      assertValidationError(result, "value", "must be negative");
    });
  });

  // ============================================================================
  // NonPositiveBigInt Validator
  // ============================================================================
  describe("NonPositiveBigInt", () => {
    test("accepts zero", () => {
      const result = mod.NonPositiveBigIntValidator.fromJSON({ value: "0" });
      assertValidationSuccess(result, "value");
    });

    test("accepts negative value", () => {
      const result = mod.NonPositiveBigIntValidator.fromJSON({ value: "-100" });
      assertValidationSuccess(result, "value");
    });

    test("rejects positive value", () => {
      const result = mod.NonPositiveBigIntValidator.fromJSON({ value: "1" });
      assertValidationError(result, "value", "must be non-positive");
    });
  });
});
