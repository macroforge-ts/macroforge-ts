import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "array-validator-tests";

describe("Array Validators", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // MaxItems Validator
  // ============================================================================
  describe("MaxItems", () => {
    test("accepts array at max items", () => {
      const result = mod.MaxItemsValidator.fromJSON({ items: ["a", "b", "c", "d", "e"] });
      assertValidationSuccess(result, "items");
    });

    test("accepts array below max items", () => {
      const result = mod.MaxItemsValidator.fromJSON({ items: ["a", "b"] });
      assertValidationSuccess(result, "items");
    });

    test("accepts empty array", () => {
      const result = mod.MaxItemsValidator.fromJSON({ items: [] });
      assertValidationSuccess(result, "items");
    });

    test("rejects array exceeding max items", () => {
      const result = mod.MaxItemsValidator.fromJSON({ items: ["a", "b", "c", "d", "e", "f"] });
      assertValidationError(result, "items", "must have at most 5 items");
    });
  });

  // ============================================================================
  // MinItems Validator
  // ============================================================================
  describe("MinItems", () => {
    test("accepts array at min items", () => {
      const result = mod.MinItemsValidator.fromJSON({ items: ["a", "b"] });
      assertValidationSuccess(result, "items");
    });

    test("accepts array above min items", () => {
      const result = mod.MinItemsValidator.fromJSON({ items: ["a", "b", "c", "d"] });
      assertValidationSuccess(result, "items");
    });

    test("rejects array below min items", () => {
      const result = mod.MinItemsValidator.fromJSON({ items: ["a"] });
      assertValidationError(result, "items", "must have at least 2 items");
    });

    test("rejects empty array", () => {
      const result = mod.MinItemsValidator.fromJSON({ items: [] });
      assertValidationError(result, "items", "must have at least 2 items");
    });
  });

  // ============================================================================
  // ItemsCount Validator
  // ============================================================================
  describe("ItemsCount", () => {
    test("accepts array at exact count", () => {
      const result = mod.ItemsCountValidator.fromJSON({ items: ["a", "b", "c"] });
      assertValidationSuccess(result, "items");
    });

    test("rejects array below count", () => {
      const result = mod.ItemsCountValidator.fromJSON({ items: ["a", "b"] });
      assertValidationError(result, "items", "must have exactly 3 items");
    });

    test("rejects array above count", () => {
      const result = mod.ItemsCountValidator.fromJSON({ items: ["a", "b", "c", "d"] });
      assertValidationError(result, "items", "must have exactly 3 items");
    });

    test("rejects empty array", () => {
      const result = mod.ItemsCountValidator.fromJSON({ items: [] });
      assertValidationError(result, "items", "must have exactly 3 items");
    });
  });
});
