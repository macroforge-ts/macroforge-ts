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

    assert.ok(result.code.includes("export type Errors"), "Should generate Errors type");
    // Implementation uses Option<Array<string>> instead of Array<string> | undefined
    assert.ok(result.code.includes("_errors: Option<Array<string>>"), "Should have root _errors");
    assert.ok(result.code.includes("name: Option<Array<string>>"), "Should have name error array");
    assert.ok(result.code.includes("email: Option<Array<string>>"), "Should have email error array");
    assert.ok(result.code.includes("age: Option<Array<string>>"), "Should have age error array");
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

    assert.ok(result.code.includes("export type Tainted"), "Should generate Tainted type");
    // Implementation uses Option<boolean> instead of boolean | undefined
    assert.ok(result.code.includes("name: Option<boolean>"), "Should have name tainted flag");
    assert.ok(result.code.includes("email: Option<boolean>"), "Should have email tainted flag");
  });

  test("generates FieldController interface", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface SimpleForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("export interface FieldController<T>"), "Should generate FieldController");
    assert.ok(result.code.includes("readonly path:"), "Should have path property");
    assert.ok(result.code.includes("readonly name:"), "Should have name property");
    assert.ok(result.code.includes("readonly constraints:"), "Should have constraints property");
    assert.ok(result.code.includes("get(): T"), "Should have get method");
    assert.ok(result.code.includes("set(value: T): void"), "Should have set method");
    assert.ok(result.code.includes("getError():"), "Should have getError method");
    assert.ok(result.code.includes("setError("), "Should have setError method");
    assert.ok(result.code.includes("getTainted():"), "Should have getTainted method");
    assert.ok(result.code.includes("setTainted("), "Should have setTainted method");
    assert.ok(result.code.includes("validate():"), "Should have validate method");
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

    assert.ok(result.code.includes("export interface FieldControllers"), "Should generate FieldControllers");
    assert.ok(result.code.includes("readonly name: FieldController<string>"), "Should have string field");
    assert.ok(result.code.includes("readonly count: FieldController<number>"), "Should have number field");
    assert.ok(result.code.includes("readonly active: FieldController<boolean>"), "Should have boolean field");
  });

  test("generates Gigaform interface", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface MyForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("export interface Gigaform"), "Should generate Gigaform interface");
    assert.ok(result.code.includes("readonly data: MyForm"), "Should have data property");
    assert.ok(result.code.includes("readonly errors: Errors"), "Should have errors property");
    assert.ok(result.code.includes("readonly tainted: Tainted"), "Should have tainted property");
    assert.ok(result.code.includes("readonly fields: FieldControllers"), "Should have fields property");
    assert.ok(result.code.includes("validate():"), "Should have validate method");
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
    assert.ok(!dataAliasPattern.test(result.code), "Should NOT generate Data type alias");
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

    assert.ok(result.code.includes("export function createForm("), "Should generate createForm function");
    assert.ok(result.code.includes("overrides?: Partial<TestForm>"), "Should accept optional overrides");
    assert.ok(result.code.includes("): Gigaform"), "Should return Gigaform type");
  });

  test("generates reactive $state for data", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ReactiveForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("let data = $state("), "Should use $state for data");
    assert.ok(result.code.includes("ReactiveForm.defaultValue()"), "Should call defaultValue()");
  });

  test("generates reactive $state for errors and tainted", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface StateForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Implementation initializes with Option.none() for each field
    assert.ok(result.code.includes("let errors = $state<Errors>({"), "Should use $state for errors");
    assert.ok(result.code.includes("Option.none()"), "Should initialize with Option.none()");
    assert.ok(result.code.includes("let tainted = $state<Tainted>({"), "Should use $state for tainted");
  });

  test("generates validate function that delegates to fromObject", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ValidatedForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("function validate()"), "Should generate validate function");
    assert.ok(result.code.includes("ValidatedForm.fromObject(data)"), "Should call fromObject for validation");
  });

  test("generates reset function", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ResettableForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("function reset("), "Should generate reset function");
    assert.ok(result.code.includes("newOverrides?: Partial<ResettableForm>"), "Should accept overrides");
    // Implementation resets to Option.none() instead of empty object
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
    assert.ok(result.code.includes("get errors()"), "Should have errors getter");
    assert.ok(result.code.includes("set errors(v)"), "Should have errors setter");
    assert.ok(result.code.includes("get tainted()"), "Should have tainted getter");
    assert.ok(result.code.includes("set tainted(v)"), "Should have tainted setter");
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
    assert.ok(result.code.includes("get: () => data.name"), "Should generate closure get accessor");
    assert.ok(result.code.includes("set: (value: string) => { data.name = value; }"), "Should generate closure set accessor");
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
    assert.ok(result.code.includes("getError: () => errors.email"), "Should generate getError closure");
    assert.ok(result.code.includes("setError: (value: Option<Array<string>>) => { errors.email = value; }"), "Should generate setError closure");
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
    assert.ok(result.code.includes("getTainted: () => tainted.field"), "Should generate getTainted closure");
    assert.ok(result.code.includes("setTainted: (value: Option<boolean>) => { tainted.field = value; }"), "Should generate setTainted closure");
  });

  test("generates field-level validate using validateField", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface FilterForm {
        username: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("validate: ()"), "Should generate validate method");
    assert.ok(result.code.includes('FilterForm.validateField("username", data.username)'), "Should call per-field validation");
    assert.ok(result.code.includes(".map(e => e.message)"), "Should extract messages");
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

    assert.ok(result.code.includes('path: ["firstName"]'), "Should have firstName path");
    assert.ok(result.code.includes('path: ["lastName"]'), "Should have lastName path");
    assert.ok(result.code.includes('name: "firstName"'), "Should have firstName name");
    assert.ok(result.code.includes('name: "lastName"'), "Should have lastName name");
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

    assert.ok(result.code.includes("minlength: 2"), "Should have minlength constraint");
    assert.ok(result.code.includes("maxlength: 50"), "Should have maxlength constraint");
    assert.ok(result.code.includes('type: "email"'), "Should have email type constraint");
    assert.ok(result.code.includes("min: 1"), "Should have positive min constraint");
    assert.ok(result.code.includes("step: 1"), "Should have int step constraint");
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

    assert.ok(result.code.includes('label: "Full Name"'), "Should include label");
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

    assert.ok(result.code.includes('description: "Enter your full legal name"'), "Should include description");
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

    assert.ok(result.code.includes('placeholder: "john@example.com"'), "Should include placeholder");
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

    assert.ok(result.code.includes("disabled: true"), "Should include disabled flag");
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

    assert.ok(result.code.includes("readonly: true"), "Should include readonly flag");
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

    assert.ok(result.code.includes("at: (index: number)"), "Should generate at method");
    assert.ok(result.code.includes("push: (item: string)"), "Should generate push method");
    assert.ok(result.code.includes("remove: (index: number)"), "Should generate remove method");
    assert.ok(result.code.includes("swap: (a: number, b: number)"), "Should generate swap method");
  });

  test("generates at() with closure accessors", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface IndexedForm {
        items: number[];
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("get: () => data.items[index]"), "Should generate indexed get");
    assert.ok(result.code.includes("set: (value: number) => { data.items[index] = value; }"), "Should generate indexed set");
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

    assert.ok(result.code.includes("getCustomDefaults()"), "Should call defaultOverride function");
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
    assert.ok(result.diagnostics.length === 0 || !result.diagnostics.some(d => d.level === "error"),
      "Should parse i18nPrefix without errors");
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

    assert.ok(result.code.includes("export function fromFormData("), "Should generate fromFormData");
    assert.ok(result.code.includes("formData: FormData"), "Should accept FormData parameter");
  });

  test("returns Result with structured errors", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ResultForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    // Match Result type with structured errors
    assert.ok(result.code.includes("Result<ResultForm, Array<{field: string; message: string}>>"),
      "Should return Result with structured errors");
  });

  test("delegates to fromStringifiedJSON for validation", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DelegateForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("DelegateForm.fromStringifiedJSON(JSON.stringify(obj))"),
      "Should delegate to fromStringifiedJSON");
  });
});

// ============================================================================
// Import Handling Tests
// ============================================================================

describe("Gigaform imports", () => {
  test("adds Result import", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface ImportForm {
        value: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes('import { Result }') || result.code.includes('import {Result}'),
      "Should add Result import");
    assert.ok(result.code.includes('from "macroforge/utils"'),
      "Should import from macroforge/utils");
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

    assert.ok(result.code.includes("DefaultForm.defaultValue()"),
      "Should use defaultValue() from Default macro");
  });

  test("works with Deserialize macro for fromObject", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface DeserializeForm {
        name: string;
      }
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    assert.ok(result.code.includes("DeserializeForm.fromObject("),
      "Should use fromObject from Deserialize macro");
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
    assert.ok(result.code.includes("defaultValue"),
      "Should have Default's defaultValue");

    // Serialize macro
    assert.ok(result.code.includes("toStringifiedJSON") || result.code.includes("__serialize"),
      "Should have Serialize methods");

    // Deserialize macro
    assert.ok(result.code.includes("fromStringifiedJSON") || result.code.includes("__deserialize"),
      "Should have Deserialize methods");

    // Gigaform macro
    assert.ok(result.code.includes("createForm"),
      "Should have Gigaform's createForm");
    assert.ok(result.code.includes("export type Errors"),
      "Should have Gigaform's Errors type");

    // No expansion errors
    const errors = result.diagnostics.filter(d => d.level === "error");
    assert.ok(errors.length === 0,
      `Should have no expansion errors, got: ${errors.map(e => e.message).join(", ")}`);
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

    const errors = result.diagnostics.filter(d => d.level === "error");
    assert.ok(errors.length === 0, "Classes should be supported without errors");
    assert.ok(result.code.includes("export function createForm("), "Should generate createForm for class");
  });

  test("reports error for empty interface", () => {
    const code = `
      /** @derive(Default, Deserialize, Gigaform) */
      interface EmptyForm {}
    `;
    const result = expandSync(withGigaformImport(code), "test.ts");

    const errors = result.diagnostics.filter(d => d.level === "error");
    assert.ok(errors.length > 0, "Should report error for empty interface");
    assert.ok(errors.some(e => e.message.includes("no fields")),
      "Error should mention no fields");
  });
});

// NOTE: Structured error tests for Deserialize are not included here.
// The Gigaform macro depends on Deserialize returning string[] errors currently.
// When structured errors are implemented in Deserialize, those tests should be
// added to the serde tests, not the Gigaform tests.
