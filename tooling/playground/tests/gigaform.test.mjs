/**
 * Comprehensive tests for the Gigaform macro.
 *
 * Tests:
 * - Macro expansion (generated code structure)
 * - Type generation (Errors, Tainted, FieldController, Gigaform interfaces)
 * - Factory function generation (createForm)
 * - Field controller generation with closures
 * - Integration with Default, Serialize, Deserialize macros
 */

import { test, describe } from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { expandSync } = require("macroforge");

// Helper to normalize whitespace for robust string matching
const normalize = (s) => s.replace(/\s+/g, "");

// Helper to check if normalized code contains normalized pattern
const includesNormalized = (code, pattern) =>
  normalize(code).includes(normalize(pattern));

// Helper to wrap code with the required Gigaform macro import
const withGigaformImport = (code) => `
/** import macro { Gigaform } from "@playground/macro"; */
${code}
`;

// ============================================================================
// Gigaform Type Generation Tests
// ============================================================================

describe("Gigaform type generation", () => {
  test("generates Errors type with field error arrays", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface UserForm {
        name: string;
        email: string;
        age: number;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export type UserFormErrors"),
      "Should generate UserFormErrors type",
    );
    // Implementation uses __gf_Option<Array<string>> instead of Array<string> | undefined
    assert.ok(
      result.code.includes("_errors: __gf_Option<Array<string>>"),
      "Should have root _errors",
    );
    assert.ok(
      result.code.includes("name: __gf_Option<Array<string>>"),
      "Should have name error array",
    );
    assert.ok(
      result.code.includes("email: __gf_Option<Array<string>>"),
      "Should have email error array",
    );
    assert.ok(
      result.code.includes("age: __gf_Option<Array<string>>"),
      "Should have age error array",
    );
  });

  test("generates Tainted type with boolean flags", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface UserForm {
        name: string;
        email: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export type UserFormTainted"),
      "Should generate UserFormTainted type",
    );
    // Implementation uses __gf_Option<boolean> instead of boolean | undefined
    assert.ok(
      result.code.includes("name: __gf_Option<boolean>"),
      "Should have name tainted flag",
    );
    assert.ok(
      result.code.includes("email: __gf_Option<boolean>"),
      "Should have email tainted flag",
    );
  });

  test("imports FieldController from canonical location", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface SimpleForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // FieldController is now imported from the canonical location instead of being generated
    assert.ok(
      result.code.includes("import type { FieldController }") ||
        result.code.includes("import { FieldController }"),
      "Should import FieldController",
    );
    assert.ok(
      result.code.includes("@playground/macro/gigaform"),
      "Should import from @playground/macro/gigaform",
    );
  });

  test("generates FieldControllers interface with typed fields", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface TypedForm {
        name: string;
        count: number;
        active: boolean;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export interface TypedFormFieldControllers"),
      "Should generate TypedFormFieldControllers",
    );
    assert.ok(
      result.code.includes("readonly name: FieldController<string>"),
      "Should have string field",
    );
    assert.ok(
      result.code.includes("readonly count: FieldController<number>"),
      "Should have number field",
    );
    assert.ok(
      result.code.includes("readonly active: FieldController<boolean>"),
      "Should have boolean field",
    );
  });

  test("generates Gigaform interface", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface MyForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export interface MyFormGigaform"),
      "Should generate MyFormGigaform interface",
    );
    assert.ok(
      result.code.includes("readonly data: MyForm"),
      "Should have data property",
    );
    assert.ok(
      result.code.includes("readonly errors: MyFormErrors"),
      "Should have errors property",
    );
    assert.ok(
      result.code.includes("readonly tainted: MyFormTainted"),
      "Should have tainted property",
    );
    assert.ok(
      result.code.includes("readonly fields: MyFormFieldControllers"),
      "Should have fields property",
    );
    assert.ok(
      result.code.includes("validate():"),
      "Should have validate method",
    );
    assert.ok(result.code.includes("reset("), "Should have reset method");
  });

  test("does not generate Data type alias (removed)", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface UserForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Should NOT have "type Data = UserForm" since we removed it
    const dataAliasPattern = /export\s+type\s+Data\s*=/;
    assert.ok(
      !dataAliasPattern.test(result.code),
      "Should NOT generate Data type alias",
    );
  });
});

// ============================================================================
// Gigaform Factory Function Tests
// ============================================================================

describe("Gigaform createForm factory", () => {
  test("generates createForm function", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface TestForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export function testFormCreateForm("),
      "Should generate testFormCreateForm function",
    );
    assert.ok(
      result.code.includes("overrides?: Partial<TestForm>"),
      "Should accept optional overrides",
    );
    assert.ok(
      result.code.includes("): TestFormGigaform"),
      "Should return TestFormGigaform type",
    );
  });

  test("generates reactive $state for data", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ReactiveForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("let data = $state("),
      "Should use $state for data",
    );
    assert.ok(
      result.code.includes("reactiveFormDefaultValue()"),
      "Should call reactiveFormDefaultValue()",
    );
    assert.ok(
      !result.code.includes("ReactiveForm.defaultValue()"),
      "Should not use namespace-style defaultValue()",
    );
  });

  test("generates reactive $state for errors and tainted", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface StateForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Implementation initializes with optionNone() for each field
    assert.ok(
      result.code.includes("let errors = $state<StateFormErrors>({"),
      "Should use $state for errors",
    );
    assert.ok(
      result.code.includes("optionNone()"),
      "Should initialize with optionNone()",
    );
    assert.ok(
      result.code.includes("let tainted = $state<StateFormTainted>({"),
      "Should use $state for tainted",
    );
  });

  test("generates validate function that delegates to Deserialize", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ValidatedForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      includesNormalized(result.code, "function validate()"),
      "Should generate validate function",
    );
    assert.ok(
      includesNormalized(result.code, "toExit(validatedFormDeserialize(data))"),
      "Should call validatedFormDeserialize wrapped in toExit for validation",
    );
  });

  test("generates reset function", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ResettableForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("function reset("),
      "Should generate reset function",
    );
    assert.ok(
      result.code.includes("newOverrides?: Partial<ResettableForm>"),
      "Should accept overrides",
    );
    // Implementation resets to optionNone() instead of empty object
    assert.ok(result.code.includes("errors = {"), "Should reset errors");
    assert.ok(result.code.includes("tainted = {"), "Should reset tainted");
  });

  test("generates getter/setter return object", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface GetSetForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("get data()"), "Should have data getter");
    assert.ok(result.code.includes("set data(v)"), "Should have data setter");
    assert.ok(
      result.code.includes("get errors()"),
      "Should have errors getter",
    );
    assert.ok(
      result.code.includes("set errors(v)"),
      "Should have errors setter",
    );
    assert.ok(
      result.code.includes("get tainted()"),
      "Should have tainted getter",
    );
    assert.ok(
      result.code.includes("set tainted(v)"),
      "Should have tainted setter",
    );
  });
});

// ============================================================================
// Field Controller Generation Tests
// ============================================================================

describe("Gigaform field controllers", () => {
  test("generates field controllers with closure-based accessors", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ClosureForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Closure-based accessors (no parameters)
    assert.ok(
      includesNormalized(result.code, "get: () => data.name"),
      "Should generate closure get accessor",
    );
    assert.ok(
      includesNormalized(result.code, "set: (value: string) => { data.name = value; }"),
      "Should generate closure set accessor",
    );
  });

  test("generates error accessors with closures", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ErrorForm {
        email: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Implementation uses direct property access and Option type
    assert.ok(
      includesNormalized(result.code, "getError: () => errors.email"),
      "Should generate getError closure",
    );
    assert.ok(
      includesNormalized(
        result.code,
        "setError: (value: __gf_Option<Array<string>>) => { errors.email = value; }",
      ),
      "Should generate setError closure",
    );
  });

  test("generates tainted accessors with closures", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface TaintedForm {
        field: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Implementation uses direct property access and Option type
    assert.ok(
      includesNormalized(result.code, "getTainted: () => tainted.field"),
      "Should generate getTainted closure",
    );
    assert.ok(
      includesNormalized(
        result.code,
        "setTainted: (value: __gf_Option<boolean>) => { tainted.field = value; }",
      ),
      "Should generate setTainted closure",
    );
  });

  test("generates field-level validate using validateField", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface FilterForm {
        username: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      includesNormalized(result.code, "validate: ()"),
      "Should generate validate method",
    );
    assert.ok(
      includesNormalized(
        result.code,
        'filterFormValidateField("username", data.username)',
      ),
      "Should call per-field validation via filterFormValidateField",
    );
    assert.ok(
      !includesNormalized(
        result.code,
        'FilterForm.validateField("username", data.username)',
      ),
      "Should not use namespace-style FilterForm.validateField",
    );
    assert.ok(
      includesNormalized(
        result.code,
        ".map((e: { field: string; message: string }) => e.message)",
      ),
      "Should extract messages",
    );
  });

  test("generates path array for each field", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface PathForm {
        firstName: string;
        lastName: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      includesNormalized(result.code, 'path: ["firstName"]'),
      "Should have firstName path",
    );
    assert.ok(
      includesNormalized(result.code, 'path: ["lastName"]'),
      "Should have lastName path",
    );
    assert.ok(
      includesNormalized(result.code, 'name: "firstName"'),
      "Should have firstName name",
    );
    assert.ok(
      includesNormalized(result.code, 'name: "lastName"'),
      "Should have lastName name",
    );
  });

  test("generates constraints from validators", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ConstrainedForm {
        /** @serde({ validate: ["minLength(2)", "maxLength(50)"] }) */
        name: string;

        /** @serde({ validate: ["email"] }) */
        email: string;

        /** @serde({ validate: ["positive", "int"] }) */
        age: number;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("minlength: 2"),
      "Should have minlength constraint",
    );
    assert.ok(
      result.code.includes("maxlength: 50"),
      "Should have maxlength constraint",
    );
    assert.ok(
      result.code.includes('type: "email"'),
      "Should have email type constraint",
    );
    assert.ok(
      result.code.includes("min: 1"),
      "Should have positive min constraint",
    );
    assert.ok(
      result.code.includes("step: 1"),
      "Should have int step constraint",
    );
  });
});

// ============================================================================
// UI Metadata Tests
// ============================================================================

describe("Gigaform UI metadata", () => {
  test("generates label from controller", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface LabelForm {
        /** @textController({ label: "Full Name" }) */
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes('label: "Full Name"'),
      "Should include label",
    );
  });

  test("generates description from controller", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DescForm {
        /** @textController({ description: "Enter your full legal name" }) */
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes('description: "Enter your full legal name"'),
      "Should include description",
    );
  });

  test("generates placeholder from controller", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface PlaceholderForm {
        /** @textController({ placeholder: "john@example.com" }) */
        email: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes('placeholder: "john@example.com"'),
      "Should include placeholder",
    );
  });

  test("generates disabled flag from controller", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DisabledForm {
        /** @textController({ disabled: true }) */
        readOnlyField: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("disabled: true"),
      "Should include disabled flag",
    );
  });

  test("generates readonly flag from controller", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ReadonlyForm {
        /** @textController({ readonly: true }) */
        fixedField: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("readonly: true"),
      "Should include readonly flag",
    );
  });
});

// ============================================================================
// Array Field Tests
// ============================================================================

describe("Gigaform array fields", () => {
  test("generates array methods (at, push, remove, swap)", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ArrayForm {
        tags: string[];
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("at: (index: number)"),
      "Should generate at method",
    );
    assert.ok(
      result.code.includes("push: (item: string)"),
      "Should generate push method",
    );
    assert.ok(
      result.code.includes("remove: (index: number)"),
      "Should generate remove method",
    );
    assert.ok(
      result.code.includes("swap: (a: number, b: number)"),
      "Should generate swap method",
    );
  });

  test("generates at() with closure accessors", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface IndexedForm {
        items: number[];
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      includesNormalized(result.code, "get: () => data.items[index]"),
      "Should generate indexed get",
    );
    assert.ok(
      includesNormalized(
        result.code,
        "set: (value: number) => { data.items[index] = value; }",
      ),
      "Should generate indexed set",
    );
  });
});

// ============================================================================
// Options Tests
// ============================================================================

describe("Gigaform container options", () => {
  test("supports defaultOverride option", () => {
    const code = `
      /**
       * @derive(Default, Deserialize, Gigaform)
       * @gigaform({ defaultOverride: "getCustomDefaults" })
       */
      interface OverrideForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("getCustomDefaults()"),
      "Should call defaultOverride function",
    );
  });

  test("supports i18nPrefix option", () => {
    const code = `
      /**
       * @derive(Default, Deserialize, Gigaform)
       * @gigaform({ i18nPrefix: "userForm" })
       */
      interface I18nForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Should generate without errors
    assert.ok(
      result.diagnostics.length === 0 ||
        !result.diagnostics.some((d) => d.level === "error"),
      "Should parse i18nPrefix without errors",
    );
  });
});

// ============================================================================
// fromFormData Function Tests
// ============================================================================

describe("Gigaform fromFormData", () => {
  test("generates fromFormData function", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface FormDataForm {
        name: string;
        age: number;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("export function formDataFormFromFormData("),
      "Should generate formDataFormFromFormData",
    );
    assert.ok(
      result.code.includes("formData: FormData"),
      "Should accept FormData parameter",
    );
  });

  test("returns Exit with structured errors", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ResultForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Match Exit type with structured errors (Effect library type)
    assert.ok(
      includesNormalized(result.code, "Exit<ResultForm, Array<{ field: string; message: string }>>"),
      "Should return Exit with structured errors",
    );
  });

  test("delegates to Deserialize for validation", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DelegateForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Should delegate to the deserialize function and wrap with toExit
    assert.ok(
      result.code.includes("toExit(delegateFormDeserialize(obj))"),
      "Should delegate to delegateFormDeserialize wrapped in toExit",
    );
  });
});

// ============================================================================
// Import Handling Tests
// ============================================================================

describe("Gigaform imports", () => {
  test("adds Exit type import", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ImportForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Gigaform uses Effect's Exit type from @playground/macro/gigaform
    assert.ok(
      result.code.includes("import type { Exit }") ||
        result.code.includes("import type {Exit}"),
      "Should add Exit type import",
    );
    assert.ok(
      result.code.includes('from "@playground/macro/gigaform"'),
      "Should import from @playground/macro/gigaform",
    );
  });
});

// ============================================================================
// Integration Tests
// ============================================================================

describe("Gigaform integration with other macros", () => {
  test("works with Default macro for defaultValue()", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DefaultForm {
        name: string;
        count: number;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(
      result.code.includes("defaultFormDefaultValue()"),
      "Should use defaultFormDefaultValue()",
    );
    assert.ok(
      !result.code.includes("DefaultForm.defaultValue()"),
      "Should not use namespace-style defaultValue()",
    );
  });

  test("works with Deserialize macro for validation", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DeserializeForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Gigaform uses the Deserialize macro's function for validation
    assert.ok(
      result.code.includes("deserializeFormDeserialize("),
      "Should use deserializeFormDeserialize()",
    );
  });

  test("all three macros generate non-conflicting code", () => {
    const code = `
      /** @derive(Default, Serialize, Deserialize, Gigaform) */
      interface FullForm {
        name: string;
        email: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Default macro
    assert.ok(
      result.code.includes("defaultValue"),
      "Should have Default's defaultValue",
    );

    // Serialize macro
    assert.ok(
      result.code.includes("toStringifiedJSON") ||
        result.code.includes("SerializeWithContext"),
      "Should have Serialize methods",
    );

    // Deserialize macro
    assert.ok(
      result.code.includes("fromStringifiedJSON") ||
        result.code.includes("DeserializeWithContext"),
      "Should have Deserialize methods",
    );

    // Gigaform macro
    assert.ok(
      result.code.includes("fullFormCreateForm"),
      "Should have Gigaform's fullFormCreateForm",
    );
    assert.ok(
      result.code.includes("export type FullFormErrors"),
      "Should have Gigaform's FullFormErrors type",
    );

    // No expansion errors
    const errors = result.diagnostics.filter((d) => d.level === "error");
    assert.ok(
      errors.length === 0,
      `Should have no expansion errors, got: ${errors.map((e) => e.message).join(", ")}`,
    );
  });
});

// ============================================================================
// Error Handling Tests
// ============================================================================

describe("Gigaform error handling", () => {
  test("supports classes as well as interfaces", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      class SupportedClass {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    const errors = result.diagnostics.filter((d) => d.level === "error");
    assert.ok(
      errors.length === 0,
      "Classes should be supported without errors",
    );
    assert.ok(
      result.code.includes("export function supportedClassCreateForm("),
      "Should generate supportedClassCreateForm for class",
    );
  });

  test("reports error for empty interface", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface EmptyForm {}
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    const errors = result.diagnostics.filter((d) => d.level === "error");
    assert.ok(errors.length > 0, "Should report error for empty interface");
    assert.ok(
      errors.some((e) => e.message.includes("no fields")),
      "Error should mention no fields",
    );
  });
});

// NOTE: Structured error tests for Deserialize are not included here.
// The Gigaform macro depends on Deserialize returning string[] errors currently.
// When structured errors are implemented in Deserialize, those tests should be
// added to the serde tests, not the Gigaform tests.
