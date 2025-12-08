import {
  test,
  describe,
  before,
  loadValidatorModule,
  assertValidationError,
  assertValidationSuccess,
} from "./helpers.mjs";

const MODULE_NAME = "string-validator-tests";

describe("String Validators", () => {
  let mod;

  before(() => {
    mod = loadValidatorModule(MODULE_NAME);
  });

  // ============================================================================
  // Email Validator
  // ============================================================================
  describe("Email", () => {
    test("accepts valid email", () => {
      const result = mod.EmailValidator.fromJSON({ email: "test@example.com" });
      assertValidationSuccess(result, "email");
    });

    test("accepts email with subdomain", () => {
      const result = mod.EmailValidator.fromJSON({ email: "user@mail.example.com" });
      assertValidationSuccess(result, "email");
    });

    test("accepts email with plus sign", () => {
      const result = mod.EmailValidator.fromJSON({ email: "user+tag@example.com" });
      assertValidationSuccess(result, "email");
    });

    test("rejects email without @", () => {
      const result = mod.EmailValidator.fromJSON({ email: "testexample.com" });
      assertValidationError(result, "email", "must be a valid email");
    });

    test("rejects email without domain", () => {
      const result = mod.EmailValidator.fromJSON({ email: "test@" });
      assertValidationError(result, "email", "must be a valid email");
    });

    test("rejects email with spaces", () => {
      const result = mod.EmailValidator.fromJSON({ email: "test @example.com" });
      assertValidationError(result, "email", "must be a valid email");
    });

    test("rejects email without local part", () => {
      const result = mod.EmailValidator.fromJSON({ email: "@example.com" });
      assertValidationError(result, "email", "must be a valid email");
    });
  });

  // ============================================================================
  // URL Validator
  // ============================================================================
  describe("Url", () => {
    test("accepts valid HTTPS URL", () => {
      const result = mod.UrlValidator.fromJSON({ url: "https://example.com" });
      assertValidationSuccess(result, "url");
    });

    test("accepts valid HTTP URL", () => {
      const result = mod.UrlValidator.fromJSON({ url: "http://example.com" });
      assertValidationSuccess(result, "url");
    });

    test("accepts URL with path", () => {
      const result = mod.UrlValidator.fromJSON({ url: "https://example.com/path/to/resource" });
      assertValidationSuccess(result, "url");
    });

    test("accepts URL with query string", () => {
      const result = mod.UrlValidator.fromJSON({ url: "https://example.com?foo=bar&baz=qux" });
      assertValidationSuccess(result, "url");
    });

    test("accepts URL with port", () => {
      const result = mod.UrlValidator.fromJSON({ url: "https://example.com:8080" });
      assertValidationSuccess(result, "url");
    });

    test("rejects string without protocol", () => {
      const result = mod.UrlValidator.fromJSON({ url: "example.com" });
      assertValidationError(result, "url", "must be a valid URL");
    });

    test("rejects plain text", () => {
      const result = mod.UrlValidator.fromJSON({ url: "not-a-url" });
      assertValidationError(result, "url", "must be a valid URL");
    });
  });

  // ============================================================================
  // UUID Validator
  // ============================================================================
  describe("Uuid", () => {
    test("accepts valid v4 UUID", () => {
      const result = mod.UuidValidator.fromJSON({ id: "123e4567-e89b-42d3-a456-426614174000" });
      assertValidationSuccess(result, "id");
    });

    test("accepts valid v1 UUID", () => {
      const result = mod.UuidValidator.fromJSON({ id: "6ba7b810-9dad-11d1-80b4-00c04fd430c8" });
      assertValidationSuccess(result, "id");
    });

    test("accepts lowercase UUID", () => {
      const result = mod.UuidValidator.fromJSON({ id: "550e8400-e29b-41d4-a716-446655440000" });
      assertValidationSuccess(result, "id");
    });

    test("accepts uppercase UUID", () => {
      const result = mod.UuidValidator.fromJSON({ id: "550E8400-E29B-41D4-A716-446655440000" });
      assertValidationSuccess(result, "id");
    });

    test("rejects invalid UUID format", () => {
      const result = mod.UuidValidator.fromJSON({ id: "not-a-uuid" });
      assertValidationError(result, "id", "must be a valid UUID");
    });

    test("rejects UUID with wrong segment lengths", () => {
      const result = mod.UuidValidator.fromJSON({ id: "123e4567-e89b-12d3-a456-42661417400" });
      assertValidationError(result, "id", "must be a valid UUID");
    });

    test("rejects UUID without dashes", () => {
      const result = mod.UuidValidator.fromJSON({ id: "550e8400e29b41d4a716446655440000" });
      assertValidationError(result, "id", "must be a valid UUID");
    });
  });

  // ============================================================================
  // MaxLength Validator
  // ============================================================================
  describe("MaxLength", () => {
    test("accepts string at max length", () => {
      const result = mod.MaxLengthValidator.fromJSON({ shortText: "1234567890" });
      assertValidationSuccess(result, "shortText");
    });

    test("accepts string below max length", () => {
      const result = mod.MaxLengthValidator.fromJSON({ shortText: "short" });
      assertValidationSuccess(result, "shortText");
    });

    test("accepts empty string", () => {
      const result = mod.MaxLengthValidator.fromJSON({ shortText: "" });
      assertValidationSuccess(result, "shortText");
    });

    test("rejects string exceeding max length", () => {
      const result = mod.MaxLengthValidator.fromJSON({ shortText: "12345678901" });
      assertValidationError(result, "shortText", "must have at most 10 characters");
    });
  });

  // ============================================================================
  // MinLength Validator
  // ============================================================================
  describe("MinLength", () => {
    test("accepts string at min length", () => {
      const result = mod.MinLengthValidator.fromJSON({ longText: "12345" });
      assertValidationSuccess(result, "longText");
    });

    test("accepts string above min length", () => {
      const result = mod.MinLengthValidator.fromJSON({ longText: "1234567890" });
      assertValidationSuccess(result, "longText");
    });

    test("rejects string below min length", () => {
      const result = mod.MinLengthValidator.fromJSON({ longText: "1234" });
      assertValidationError(result, "longText", "must have at least 5 characters");
    });

    test("rejects empty string", () => {
      const result = mod.MinLengthValidator.fromJSON({ longText: "" });
      assertValidationError(result, "longText", "must have at least 5 characters");
    });
  });

  // ============================================================================
  // Length Validator (exact)
  // ============================================================================
  describe("Length", () => {
    test("accepts string at exact length", () => {
      const result = mod.LengthValidator.fromJSON({ fixedText: "12345678" });
      assertValidationSuccess(result, "fixedText");
    });

    test("rejects string below length", () => {
      const result = mod.LengthValidator.fromJSON({ fixedText: "1234567" });
      assertValidationError(result, "fixedText", "must have exactly 8 characters");
    });

    test("rejects string above length", () => {
      const result = mod.LengthValidator.fromJSON({ fixedText: "123456789" });
      assertValidationError(result, "fixedText", "must have exactly 8 characters");
    });
  });

  // ============================================================================
  // LengthRange Validator
  // ============================================================================
  describe("LengthRange", () => {
    test("accepts string at min boundary", () => {
      const result = mod.LengthRangeValidator.fromJSON({ rangedText: "12345" });
      assertValidationSuccess(result, "rangedText");
    });

    test("accepts string at max boundary", () => {
      const result = mod.LengthRangeValidator.fromJSON({ rangedText: "1234567890" });
      assertValidationSuccess(result, "rangedText");
    });

    test("accepts string in middle of range", () => {
      const result = mod.LengthRangeValidator.fromJSON({ rangedText: "1234567" });
      assertValidationSuccess(result, "rangedText");
    });

    test("rejects string below min", () => {
      const result = mod.LengthRangeValidator.fromJSON({ rangedText: "1234" });
      assertValidationError(result, "rangedText", "must have between 5 and 10 characters");
    });

    test("rejects string above max", () => {
      const result = mod.LengthRangeValidator.fromJSON({ rangedText: "12345678901" });
      assertValidationError(result, "rangedText", "must have between 5 and 10 characters");
    });
  });

  // ============================================================================
  // Pattern Validator
  // ============================================================================
  describe("Pattern", () => {
    test("accepts string matching pattern", () => {
      const result = mod.PatternValidator.fromJSON({ code: "ABC" });
      assertValidationSuccess(result, "code");
    });

    test("accepts another matching pattern", () => {
      const result = mod.PatternValidator.fromJSON({ code: "XYZ" });
      assertValidationSuccess(result, "code");
    });

    test("rejects lowercase letters", () => {
      const result = mod.PatternValidator.fromJSON({ code: "abc" });
      assertValidationError(result, "code", "must match the required pattern");
    });

    test("rejects wrong length", () => {
      const result = mod.PatternValidator.fromJSON({ code: "ABCD" });
      assertValidationError(result, "code", "must match the required pattern");
    });

    test("rejects numbers", () => {
      const result = mod.PatternValidator.fromJSON({ code: "123" });
      assertValidationError(result, "code", "must match the required pattern");
    });
  });

  // ============================================================================
  // NonEmpty Validator
  // ============================================================================
  describe("NonEmpty", () => {
    test("accepts non-empty string", () => {
      const result = mod.NonEmptyValidator.fromJSON({ required: "hello" });
      assertValidationSuccess(result, "required");
    });

    test("accepts single character", () => {
      const result = mod.NonEmptyValidator.fromJSON({ required: "x" });
      assertValidationSuccess(result, "required");
    });

    test("accepts whitespace only (nonEmpty only checks length)", () => {
      const result = mod.NonEmptyValidator.fromJSON({ required: "   " });
      assertValidationSuccess(result, "required");
    });

    test("rejects empty string", () => {
      const result = mod.NonEmptyValidator.fromJSON({ required: "" });
      assertValidationError(result, "required", "must not be empty");
    });
  });

  // ============================================================================
  // Trimmed Validator
  // ============================================================================
  describe("Trimmed", () => {
    test("accepts trimmed string", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: "hello" });
      assertValidationSuccess(result, "trimmed");
    });

    test("accepts string with internal spaces", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: "hello world" });
      assertValidationSuccess(result, "trimmed");
    });

    test("accepts empty string", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: "" });
      assertValidationSuccess(result, "trimmed");
    });

    test("rejects leading whitespace", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: " hello" });
      assertValidationError(result, "trimmed", "must be trimmed");
    });

    test("rejects trailing whitespace", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: "hello " });
      assertValidationError(result, "trimmed", "must be trimmed");
    });

    test("rejects both leading and trailing whitespace", () => {
      const result = mod.TrimmedValidator.fromJSON({ trimmed: " hello " });
      assertValidationError(result, "trimmed", "must be trimmed");
    });
  });

  // ============================================================================
  // Lowercase Validator
  // ============================================================================
  describe("Lowercase", () => {
    test("accepts lowercase string", () => {
      const result = mod.LowercaseValidator.fromJSON({ lower: "hello" });
      assertValidationSuccess(result, "lower");
    });

    test("accepts lowercase with numbers", () => {
      const result = mod.LowercaseValidator.fromJSON({ lower: "hello123" });
      assertValidationSuccess(result, "lower");
    });

    test("accepts empty string", () => {
      const result = mod.LowercaseValidator.fromJSON({ lower: "" });
      assertValidationSuccess(result, "lower");
    });

    test("rejects uppercase letters", () => {
      const result = mod.LowercaseValidator.fromJSON({ lower: "Hello" });
      assertValidationError(result, "lower", "must be lowercase");
    });

    test("rejects all uppercase", () => {
      const result = mod.LowercaseValidator.fromJSON({ lower: "HELLO" });
      assertValidationError(result, "lower", "must be lowercase");
    });
  });

  // ============================================================================
  // Uppercase Validator
  // ============================================================================
  describe("Uppercase", () => {
    test("accepts uppercase string", () => {
      const result = mod.UppercaseValidator.fromJSON({ upper: "HELLO" });
      assertValidationSuccess(result, "upper");
    });

    test("accepts uppercase with numbers", () => {
      const result = mod.UppercaseValidator.fromJSON({ upper: "HELLO123" });
      assertValidationSuccess(result, "upper");
    });

    test("accepts empty string", () => {
      const result = mod.UppercaseValidator.fromJSON({ upper: "" });
      assertValidationSuccess(result, "upper");
    });

    test("rejects lowercase letters", () => {
      const result = mod.UppercaseValidator.fromJSON({ upper: "Hello" });
      assertValidationError(result, "upper", "must be uppercase");
    });

    test("rejects all lowercase", () => {
      const result = mod.UppercaseValidator.fromJSON({ upper: "hello" });
      assertValidationError(result, "upper", "must be uppercase");
    });
  });

  // ============================================================================
  // Capitalized Validator
  // ============================================================================
  describe("Capitalized", () => {
    test("accepts capitalized string", () => {
      const result = mod.CapitalizedValidator.fromJSON({ cap: "Hello" });
      assertValidationSuccess(result, "cap");
    });

    test("accepts all uppercase", () => {
      const result = mod.CapitalizedValidator.fromJSON({ cap: "HELLO" });
      assertValidationSuccess(result, "cap");
    });

    test("rejects lowercase first letter", () => {
      const result = mod.CapitalizedValidator.fromJSON({ cap: "hello" });
      assertValidationError(result, "cap", "must be capitalized");
    });

    test("accepts number first character", () => {
      // Numbers don't have uppercase, so this should pass (1 === 1.toUpperCase())
      const result = mod.CapitalizedValidator.fromJSON({ cap: "1hello" });
      assertValidationSuccess(result, "cap");
    });
  });

  // ============================================================================
  // Uncapitalized Validator
  // ============================================================================
  describe("Uncapitalized", () => {
    test("accepts uncapitalized string", () => {
      const result = mod.UncapitalizedValidator.fromJSON({ uncap: "hello" });
      assertValidationSuccess(result, "uncap");
    });

    test("accepts all lowercase", () => {
      const result = mod.UncapitalizedValidator.fromJSON({ uncap: "hello world" });
      assertValidationSuccess(result, "uncap");
    });

    test("rejects capitalized first letter", () => {
      const result = mod.UncapitalizedValidator.fromJSON({ uncap: "Hello" });
      assertValidationError(result, "uncap", "must not be capitalized");
    });

    test("rejects all uppercase", () => {
      const result = mod.UncapitalizedValidator.fromJSON({ uncap: "HELLO" });
      assertValidationError(result, "uncap", "must not be capitalized");
    });
  });

  // ============================================================================
  // StartsWith Validator
  // ============================================================================
  describe("StartsWith", () => {
    test("accepts string starting with prefix", () => {
      const result = mod.StartsWithValidator.fromJSON({ secureUrl: "https://example.com" });
      assertValidationSuccess(result, "secureUrl");
    });

    test("accepts exact prefix", () => {
      const result = mod.StartsWithValidator.fromJSON({ secureUrl: "https://" });
      assertValidationSuccess(result, "secureUrl");
    });

    test("rejects HTTP URL", () => {
      const result = mod.StartsWithValidator.fromJSON({ secureUrl: "http://example.com" });
      assertValidationError(result, "secureUrl", "must start with");
    });

    test("rejects string without prefix", () => {
      const result = mod.StartsWithValidator.fromJSON({ secureUrl: "example.com" });
      assertValidationError(result, "secureUrl", "must start with");
    });
  });

  // ============================================================================
  // EndsWith Validator
  // ============================================================================
  describe("EndsWith", () => {
    test("accepts string ending with suffix", () => {
      const result = mod.EndsWithValidator.fromJSON({ filename: "config.json" });
      assertValidationSuccess(result, "filename");
    });

    test("accepts exact suffix", () => {
      const result = mod.EndsWithValidator.fromJSON({ filename: ".json" });
      assertValidationSuccess(result, "filename");
    });

    test("rejects wrong extension", () => {
      const result = mod.EndsWithValidator.fromJSON({ filename: "config.yaml" });
      assertValidationError(result, "filename", "must end with");
    });

    test("rejects no extension", () => {
      const result = mod.EndsWithValidator.fromJSON({ filename: "config" });
      assertValidationError(result, "filename", "must end with");
    });
  });

  // ============================================================================
  // Includes Validator
  // ============================================================================
  describe("Includes", () => {
    test("accepts string containing substring", () => {
      const result = mod.IncludesValidator.fromJSON({ emailLike: "user@domain" });
      assertValidationSuccess(result, "emailLike");
    });

    test("accepts exact substring", () => {
      const result = mod.IncludesValidator.fromJSON({ emailLike: "@" });
      assertValidationSuccess(result, "emailLike");
    });

    test("accepts multiple occurrences", () => {
      const result = mod.IncludesValidator.fromJSON({ emailLike: "a@b@c" });
      assertValidationSuccess(result, "emailLike");
    });

    test("rejects string without substring", () => {
      const result = mod.IncludesValidator.fromJSON({ emailLike: "userdomain" });
      assertValidationError(result, "emailLike", "must include");
    });

    test("rejects empty string", () => {
      const result = mod.IncludesValidator.fromJSON({ emailLike: "" });
      assertValidationError(result, "emailLike", "must include");
    });
  });
});
