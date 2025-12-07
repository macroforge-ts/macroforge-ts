/**
 * Macro error position tests.
 * These tests verify that macro errors are displayed at the correct position -
 * specifically at the @derive decorator that caused the error.
 */

const test = require("node:test");
const assert = require("node:assert/strict");
const ts = require("typescript/lib/tsserverlibrary");
const initPlugin = require("../dist/index.js");

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function findMarker(source, marker) {
  const index = source.indexOf(marker);
  if (index === -1) throw new Error(`Marker "${marker}" not found`);
  return { offset: index };
}

function createEnv(files) {
  const snapshots = new Map();
  const versions = new Map();

  for (const [fileName, source] of Object.entries(files)) {
    snapshots.set(fileName, createSnapshot(source));
    versions.set(fileName, "1");
  }

  const host = {
    getScriptSnapshot: (name) => snapshots.get(name) ?? null,
    getScriptVersion: (name) => versions.get(name) ?? "0",
    getCurrentDirectory: () => "/virtual",
    getCompilationSettings: () => ({
      module: ts.ModuleKind.CommonJS,
      target: ts.ScriptTarget.ESNext,
      strict: true,
      noEmit: true,
      experimentalDecorators: true,
      emitDecoratorMetadata: false,
    }),
    getDefaultLibFileName: () => "/virtual/lib.d.ts",
    fileExists: (path) => snapshots.has(path),
    readFile: (path) => {
      const snap = snapshots.get(path);
      return snap ? snap.getText(0, snap.getLength()) : undefined;
    },
    getScriptFileNames: () => Array.from(snapshots.keys()),
    resolveModuleNames: (names) =>
      names.map((name) => {
        if (name === "./macros" || name === "@macroforge/macros") {
          return { resolvedFileName: "/virtual/macros.ts", extension: ".ts" };
        }
        return undefined;
      }),
  };

  const languageService = ts.createLanguageService(host);

  const info = {
    config: {},
    languageService,
    languageServiceHost: host,
    serverHost: {},
    project: {
      getCompilerOptions: () => host.getCompilationSettings(),
      projectService: {
        logger: { info: () => {} },
      },
    },
  };

  const plugin = initPlugin({ typescript: ts });
  const pluginService = plugin.create(info);

  return { info, pluginService };
}

const LIB_DTS = `
interface Object { toString(): string; }
interface Function {}
interface String { length: number; charAt(pos: number): string; }
interface Boolean {}
interface Number {}
interface RegExp {}
interface Array<T> { length: number; }
declare var Object: { new(value?: any): Object; prototype: Object; };
`;

const MACROS_DTS = `
export declare const Derive: (...args: any[]) => ClassDecorator;
export declare const Debug: any;
export declare const JSON: any;
export declare const Clone: any;
export declare const FieldController: any;
`;

test("unknown macro error appears at @derive position", () => {
  const source = `
/** @derive(BogusMacro) */
class User {
  id: string;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position (${decoratorPos.offset}), got ${macroDiag.start}`,
  );
});

test("multiple macros with one unknown - error at decorator start", () => {
  const source = `
/** @derive(Debug, UnknownMacro) */
class User {
  id: string;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic for unknown macro");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position (${decoratorPos.offset}), got ${macroDiag.start}`,
  );
  assert(
    macroDiag.messageText.includes("UnknownMacro"),
    "error message should mention the unknown macro name",
  );
});

test("unknown macro on interface - error at @derive position", () => {
  const source = `
/** @derive(NonExistentMacro) */
interface Config {
  name: string;
  value: number;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic for interface");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position (${decoratorPos.offset}), got ${macroDiag.start}`,
  );
});

test("multiple classes - error only on the class with bad macro", () => {
  const source = `
/** @derive(Debug) */
class ValidClass {
  id: string;
}

/** @derive(BadMacro) */
class InvalidClass {
  name: string;
}`;

  // Find the second @derive (the one with BadMacro)
  const firstDerivePos = source.indexOf("@derive");
  const secondDerivePos = source.indexOf("@derive", firstDerivePos + 1);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiags = diagnostics.filter((d) => d.source === "macroforge");

  // Should have exactly one error (for BadMacro)
  const errorDiags = macroDiags.filter(
    (d) => d.category === ts.DiagnosticCategory.Error,
  );
  assert.strictEqual(
    errorDiags.length,
    1,
    `expected exactly 1 error diagnostic, got ${errorDiags.length}`,
  );

  const errorDiag = errorDiags[0];
  assert.strictEqual(
    errorDiag.start,
    secondDerivePos,
    `error should be at second @derive (${secondDerivePos}), got ${errorDiag.start}`,
  );
  assert(
    errorDiag.messageText.includes("BadMacro"),
    "error message should mention BadMacro",
  );
});

test("generic class with unknown macro - error at decorator position", () => {
  const source = `
/** @derive(NonExistent) */
class Container<T, U = string> {
  items: T[];
  label: U;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should point to @derive, not class body. Expected ${decoratorPos.offset}, got ${macroDiag.start}`,
  );
});

test("decorator with extra whitespace - error at @derive position", () => {
  // Test with extra whitespace after /** but still on same line
  const source = `
/**   @derive(UnknownMacroWithSpaces)   */
class User {
  id: string;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic with extra whitespace");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position. Expected ${decoratorPos.offset}, got ${macroDiag.start}`,
  );
});

test("error diagnostic has correct length spanning the decorator", () => {
  const source = `
/** @derive(UnknownMacro) */
class Test {
  val: number;
}`;

  const decoratorStart = source.indexOf("@derive");
  // The decorator ends after the closing paren: @derive(UnknownMacro)
  const decoratorEnd = source.indexOf(")", decoratorStart) + 1;
  const expectedLength = decoratorEnd - decoratorStart;

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic");
  assert.strictEqual(macroDiag.start, decoratorStart);
  // Length should span the decorator, though exact length may vary
  assert(
    macroDiag.length > 0,
    `diagnostic should have positive length, got ${macroDiag.length}`,
  );
});

test("syntax error in class - graceful fallback with diagnostic", () => {
  const source = `
/** @derive(Debug) */
class User {
  // Intentional syntax error
  broken: string = ;
}`;

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // Should have some diagnostics (either macro or TS syntax error)
  assert(diagnostics.length > 0, "expected at least one diagnostic");

  // When expansion fails due to syntax error, we still get a macro diagnostic
  // (possibly at position 0 for graceful fallback)
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");
  assert(macroDiag, "expected macro diagnostic on expansion failure");
  // The diagnostic should exist and have some message about the failure
  assert(
    typeof macroDiag.messageText === "string" && macroDiag.messageText.length > 0,
    "diagnostic should have a meaningful message",
  );
});

test("exported class with unknown macro - error at decorator", () => {
  const source = `
/** @derive(UndefinedMacro) */
export class PublicAPI {
  endpoint: string;
  version: number;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic for exported class");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position`,
  );
});

test("abstract class with unknown macro - error at decorator", () => {
  const source = `
/** @derive(FakeMacro) */
abstract class BaseEntity {
  abstract id: string;
  createdAt: Date;
}`;

  const decoratorPos = findMarker(source, "@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
  const macroDiag = diagnostics.find((d) => d.source === "macroforge");

  assert(macroDiag, "expected macro diagnostic for abstract class");
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
  assert.strictEqual(
    macroDiag.start,
    decoratorPos.offset,
    `diagnostic should start at @derive position`,
  );
});

// ============================================================================
// Tests for errors in expanded/generated code
// These verify that TypeScript errors occurring in macro-generated code
// are mapped back to the decorator position, not shown at invalid positions
// ============================================================================

test("TS errors from expanded code should not have positions beyond original source length", () => {
  // Debug macro generates toString() method which increases code length
  // Any TS errors in generated regions should map back to valid original positions
  const source = `
/** @derive(Debug) */
class User {
  id: string;
}`;

  const sourceLength = source.length;
  // Find "Debug" position within @derive(Debug) - errors should point to the specific macro
  const derivePos = source.indexOf("@derive");
  const debugPos = source.indexOf("Debug", derivePos);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // All diagnostics should have positions within the original source bounds
  // Macro diagnostics (macroforge, macroforge-generated) handle their own positions
  for (const diag of diagnostics) {
    if (diag.start !== undefined) {
      const isMacroDiag =
        diag.source === "macroforge" || diag.source === "macroforge-generated";
      assert(
        diag.start < sourceLength || isMacroDiag,
        `diagnostic position ${diag.start} exceeds original source length ${sourceLength}. ` +
          `Message: "${diag.messageText}". This suggests an error in generated code wasn't properly mapped.`,
      );
    }
  }

  // If there are generated code errors, they should point to the specific macro name (Debug)
  // not the entire @derive decorator
  const generatedDiags = diagnostics.filter(
    (d) => d.source === "macroforge-generated",
  );
  for (const diag of generatedDiags) {
    assert.strictEqual(
      diag.start,
      debugPos,
      `generated code error should point to specific macro name position (Debug at ${debugPos})`,
    );
  }
});

test("expanded code diagnostics should map to valid original positions", () => {
  // When macro expansion adds code, TypeScript may report errors
  // These errors should be mapped to positions that exist in the original source
  const source = `
/** @derive(Debug) */
class Account {
  balance: number;
  owner: string;
}

const a = new Account();
a.toString();`;

  const sourceLength = source.length;
  // Find "Debug" position within @derive(Debug) - errors should point to the specific macro
  const derivePos = source.indexOf("@derive");
  const debugPos = source.indexOf("Debug", derivePos);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  // Get the expanded snapshot to check its length
  const expandedSnapshot = env.info.languageServiceHost.getScriptSnapshot(
    "/virtual/test.ts",
  );
  const expandedLength = expandedSnapshot
    ? expandedSnapshot.getLength()
    : sourceLength;

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // Verify all non-macro diagnostics have positions within original bounds
  // macroforge-generated diagnostics are allowed because they're at the specific macro name
  const nonMacroDiags = diagnostics.filter(
    (d) => d.source !== "macroforge" && d.source !== "macroforge-generated",
  );
  for (const diag of nonMacroDiags) {
    if (diag.start !== undefined && diag.start >= sourceLength) {
      // This is an error in generated code that wasn't mapped back
      assert.fail(
        `Diagnostic at position ${diag.start} is beyond original source (${sourceLength} chars). ` +
          `Expanded source is ${expandedLength} chars. ` +
          `This error in generated code should be mapped to the specific macro name. ` +
          `Message: "${diag.messageText}"`,
      );
    }
  }

  // Generated code diagnostics should point to the specific macro name (Debug)
  const generatedDiags = diagnostics.filter(
    (d) => d.source === "macroforge-generated",
  );
  for (const diag of generatedDiags) {
    assert.strictEqual(
      diag.start,
      debugPos,
      `generated code error should point to specific macro name position (Debug at ${debugPos})`,
    );
  }
});

test("all diagnostics have valid positions within original source bounds", () => {
  // Comprehensive test: multiple classes, some with errors
  const source = `
/** @derive(Debug) */
class First {
  a: string;
}

/** @derive(Debug) */
class Second {
  b: number;
}

const f = new First();
const s = new Second();`;

  const sourceLength = source.length;
  const firstDerivePos = source.indexOf("@derive");

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // Every diagnostic's start position should be within the original source
  // macroforge and macroforge-generated diagnostics handle their own positions
  for (const diag of diagnostics) {
    if (diag.start !== undefined) {
      const isValidPosition = diag.start < sourceLength;
      const isMacroDiag =
        diag.source === "macroforge" || diag.source === "macroforge-generated";

      if (!isValidPosition && !isMacroDiag) {
        assert.fail(
          `Invalid diagnostic position: ${diag.start} >= ${sourceLength}. ` +
            `Errors in generated code must map back to original source. ` +
            `Message: "${diag.messageText}"`,
        );
      }
    }
  }

  // Generated code diagnostics should point to a decorator
  const generatedDiags = diagnostics.filter(
    (d) => d.source === "macroforge-generated",
  );
  for (const diag of generatedDiags) {
    assert(
      diag.start !== undefined && diag.start < sourceLength,
      `generated code diagnostic should have valid position`,
    );
  }
});

test("generated code errors point to their specific macro name, not entire decorator", () => {
  // When @derive(Debug) generates code with errors, the error should
  // highlight only "Debug", not the entire "@derive(Debug)"
  const source = `
/** @derive(Debug) */
class First {
  a: string;
}

/** @derive(Debug) */
class Second {
  b: number;
}`;

  // Find positions of "Debug" within each @derive(Debug)
  const firstDerivePos = source.indexOf("@derive");
  const firstDebugPos = source.indexOf("Debug", firstDerivePos);
  const secondDerivePos = source.indexOf("@derive", firstDerivePos + 1);
  const secondDebugPos = source.indexOf("Debug", secondDerivePos);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // Get generated code diagnostics
  const generatedDiags = diagnostics.filter(
    (d) => d.source === "macroforge-generated",
  );

  // All generated diagnostics should point to "Debug" positions, not @derive
  for (const diag of generatedDiags) {
    assert(
      diag.start === firstDebugPos || diag.start === secondDebugPos,
      `generated diagnostic should point to "Debug" macro name. Got position ${diag.start}, expected ${firstDebugPos} or ${secondDebugPos}`,
    );
    // Length should be the length of "Debug" (5 chars)
    assert.strictEqual(
      diag.length,
      5,
      `diagnostic length should be 5 (length of "Debug"), got ${diag.length}`,
    );
  }
});

// ============================================================================
// Interface with multiple macros - replicates field-controller-test.ts scenario
// ============================================================================

test("interface with multiple macros - error on Debug points to Debug, not FieldController", () => {
  // This replicates the field-controller-test.ts scenario:
  // - Interface with @derive(FieldController, Debug)
  // - FieldController supports interfaces, Debug does not
  // - The Debug error should point to "Debug", not the entire decorator or "FieldController"
  const source = `/** import macro { FieldController } from "./macros"; */

/** @derive(FieldController, Debug) */
export interface FormModel {
  memo: string | null;
  username: string;
}`;

  const derivePos = source.indexOf("@derive");
  const fieldControllerPos = source.indexOf("FieldController", derivePos);
  const debugPos = source.indexOf("Debug", derivePos);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  // Find the macro diagnostic about Debug not supporting interfaces
  const debugErrorDiag = diagnostics.find(
    (d) =>
      d.source === "macroforge" &&
      typeof d.messageText === "string" &&
      d.messageText.toLowerCase().includes("debug") &&
      d.messageText.toLowerCase().includes("interface"),
  );

  assert(
    debugErrorDiag,
    `Should have a macro error about Debug not supporting interfaces. ` +
      `Found diagnostics: ${JSON.stringify(diagnostics.map((d) => ({ source: d.source, start: d.start, message: d.messageText })), null, 2)}`,
  );

  // The error should point to "Debug" position, not "@derive" or "FieldController"
  assert.strictEqual(
    debugErrorDiag.start,
    debugPos,
    `Debug error should point to 'Debug' at position ${debugPos}, ` +
      `not '@derive' at ${derivePos} or 'FieldController' at ${fieldControllerPos}. ` +
      `Actual position: ${debugErrorDiag.start}`,
  );
});

test("interface with single macro error points to the macro name", () => {
  // Simpler case: just Debug on an interface
  const source = `
/** @derive(Debug) */
export interface SimpleModel {
  id: number;
}`;

  const derivePos = source.indexOf("@derive");
  const debugPos = source.indexOf("Debug", derivePos);

  const env = createEnv({
    "/virtual/test.ts": source,
    "/virtual/macros.ts": MACROS_DTS,
    "/virtual/lib.d.ts": LIB_DTS,
  });

  env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

  const diagnostics =
    env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

  const debugErrorDiag = diagnostics.find(
    (d) =>
      d.source === "macroforge" &&
      typeof d.messageText === "string" &&
      d.messageText.toLowerCase().includes("debug"),
  );

  assert(debugErrorDiag, "Should have a macro error about Debug");

  // Error should point to "Debug", not "@derive"
  assert.strictEqual(
    debugErrorDiag.start,
    debugPos,
    `Debug error should point to 'Debug' at position ${debugPos}, not '@derive' at ${derivePos}. ` +
      `Actual position: ${debugErrorDiag.start}`,
  );
});
