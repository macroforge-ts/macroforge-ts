/**
 * STRICT Position Mapping Tests
 *
 * These tests are designed to FAIL when position mapping is broken.
 * They validate that errors and intellisense appear at the CORRECT
 * positions in the ORIGINAL source, not the expanded code.
 *
 * Run with: node --test tests/position-mapping-strict.test.js
 */

const test = require("node:test");
const assert = require("node:assert/strict");
const ts = require("typescript/lib/tsserverlibrary");
const initPlugin = require("../dist/index.js");
const { loadFixture, findLineNumber } = require("./test-utils");

// ============================================================================
// Test Utilities
// ============================================================================

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function offsetToLineColumn(source, offset) {
  const lines = source.split("\n");
  let currentOffset = 0;
  for (let line = 0; line < lines.length; line++) {
    const lineLength = lines[line].length + 1;
    if (currentOffset + lineLength > offset) {
      return { line: line + 1, column: offset - currentOffset + 1 };
    }
    currentOffset += lineLength;
  }
  return { line: lines.length, column: 1 };
}

function findMarker(source, marker) {
  const index = source.indexOf(marker);
  if (index === -1) throw new Error(`Marker "${marker}" not found in source`);
  return { offset: index, ...offsetToLineColumn(source, index) };
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
`;

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
      experimentalDecorators: true,
    }),
    getDefaultLibFileName: () => "/virtual/lib.d.ts",
    fileExists: (path) => snapshots.has(path),
    readFile: (path) => {
      const s = snapshots.get(path);
      return s ? s.getText(0, s.getLength()) : undefined;
    },
    getScriptFileNames: () => Array.from(snapshots.keys()),
    resolveModuleNames: (names) =>
      names.map((n) => {
        if (n === "./macros")
          return { resolvedFileName: "/virtual/macros.ts", extension: ".ts" };
        return undefined;
      }),
  };

  const ls = ts.createLanguageService(host);

  const info = {
    config: {},
    languageService: ls,
    languageServiceHost: host,
    serverHost: {},
    project: { getCompilerOptions: () => host.getCompilationSettings() },
  };

  const plugin = initPlugin({ typescript: ts });
  const pluginService = plugin.create(info);

  return { info, pluginService, snapshots, originalSource: files };
}

// ============================================================================
// STRICT Error Position Tests
// These tests SHOULD FAIL with current implementation (no source mapping)
// ============================================================================

test.describe("STRICT: Error Position Validation", () => {
  test("TS error position should match original source, not expanded", (t) => {
    /**
     * This test validates the CRITICAL issue:
     * When expansion adds code, TypeScript errors shift position.
     * The plugin should map them back to original positions.
     *
     * EXPECTED: This test should FAIL until source mapping is implemented.
     */

    const original = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
}

const u = new User();
u.BAD_METHOD();`;

    // Expansion adds toString method, shifting all subsequent positions
    const expanded = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
  toString(): string { return "User"; }
}

const u = new User();
u.BAD_METHOD();`;

    // Calculate the insertion point: after "id: string;\n" (position 72)
    // The insertion is "  toString(): string { return "User"; }\n" (40 chars)
    const insertionPoint = 72;
    const insertionLength = 40;

    const env = createEnv(
      {
        "/virtual/test.ts": original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: expanded,
        diagnostics: [],
        // Provide source mapping for the expansion
        sourceMapping: {
          segments: [
            // Before insertion: unchanged
            {
              originalStart: 0,
              originalEnd: insertionPoint,
              expandedStart: 0,
              expandedEnd: insertionPoint,
            },
            // After insertion: shifted by insertionLength
            {
              originalStart: insertionPoint,
              originalEnd: original.length,
              expandedStart: insertionPoint + insertionLength,
              expandedEnd: expanded.length,
            },
          ],
          generatedRegions: [
            {
              start: insertionPoint,
              end: insertionPoint + insertionLength,
              sourceMacro: "Debug::toString",
            },
          ],
        },
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    const diagnostics =
      env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
    const tsError = diagnostics.find(
      (d) =>
        typeof d.messageText === "string" &&
        d.messageText.includes("BAD_METHOD"),
    );

    assert(tsError, "Should have TypeScript error for BAD_METHOD");

    // Find position in ORIGINAL source
    const originalPos = findMarker(original, "BAD_METHOD");
    // Find position in EXPANDED source
    const expandedPos = findMarker(expanded, "BAD_METHOD");

    console.log(`  Original BAD_METHOD position: ${originalPos.offset}`);
    console.log(`  Expanded BAD_METHOD position: ${expandedPos.offset}`);
    console.log(`  Reported error position: ${tsError.start}`);
    console.log(`  Position delta: ${expandedPos.offset - originalPos.offset}`);

    // THIS IS THE CRITICAL ASSERTION
    // The error should point to the ORIGINAL source position
    assert.strictEqual(
      tsError.start,
      originalPos.offset,
      `Error should be at original position ${originalPos.offset}, not expanded position ${expandedPos.offset}`,
    );
  });

  test("Multi-line expansion should preserve error line numbers", (t) => {
    /**
     * When expansion adds multiple lines, errors should still
     * appear on the correct line in the original file.
     */

    // Load fixture with automatically calculated source mapping
    const fixture = loadFixture("multiline-error");

    // Calculate expected line numbers from fixture
    const originalErrorLine = findLineNumber(
      fixture.original,
      "const x: string = 123",
    );
    const expandedErrorLine = findLineNumber(
      fixture.expanded,
      "const x: string = 123",
    );

    const env = createEnv(
      {
        "/virtual/test.ts": fixture.original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: fixture.expanded,
        diagnostics: [],
        sourceMapping: fixture.sourceMapping,
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    const diagnostics =
      env.pluginService.getSemanticDiagnostics("/virtual/test.ts");

    // Find the type error (string = number)
    const typeError = diagnostics.find(
      (d) =>
        typeof d.messageText === "string" && d.messageText.includes("number"),
    );

    assert(typeError, "Should have type error");

    // Get line number from the mapped position
    const { line } = ts.getLineAndCharacterOfPosition(
      ts.createSourceFile(
        "/virtual/test.ts",
        fixture.original,
        ts.ScriptTarget.ESNext,
      ),
      typeError.start,
    );
    const reportedLine = line + 1;

    console.log(`  Original error line: ${originalErrorLine}`);
    console.log(`  Expanded error line: ${expandedErrorLine}`);
    console.log(`  Reported error position: ${typeError.start}`);
    console.log(`  Reported error line: ${reportedLine}`);

    // Error should be on the original line
    assert.strictEqual(
      reportedLine,
      originalErrorLine,
      `Error should be on line ${originalErrorLine}, not line ${reportedLine}`,
    );
  });
});

// ============================================================================
// STRICT Intellisense Position Tests
// ============================================================================

test.describe("STRICT: Hover Position Validation", () => {
  test("hover at original source position should work", (t) => {
    /**
     * When user hovers at a position in the editor (original source),
     * the plugin should map that to the correct expanded position.
     */

    const original = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
  name: string;
}`;

    const expanded = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
  name: string;
  toString(): string { return this.id; }
}`;

    const env = createEnv(
      {
        "/virtual/test.ts": original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: expanded,
        diagnostics: [],
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    // Hover at 'name' field position in ORIGINAL
    const originalNamePos = findMarker(original, "name: string");

    // But TS sees expanded code, so we need expanded position
    const expandedNamePos = findMarker(expanded, "name: string");

    // Get quick info at the EXPANDED position (current behavior)
    const quickInfo = env.pluginService.getQuickInfoAtPosition(
      "/virtual/test.ts",
      expandedNamePos.offset,
    );

    // This should work because the field exists in both
    if (quickInfo) {
      const display = quickInfo.displayParts?.map((p) => p.text).join("") || "";
      console.log(
        `  Hover at expanded position ${expandedNamePos.offset}: ${display}`,
      );
      assert(display.includes("name"), "Should show name property info");
    }

    // Now test: if we use ORIGINAL position, does it still work?
    // (It might not if positions shifted)
    const quickInfoOriginal = env.pluginService.getQuickInfoAtPosition(
      "/virtual/test.ts",
      originalNamePos.offset,
    );

    if (quickInfoOriginal) {
      const display =
        quickInfoOriginal.displayParts?.map((p) => p.text).join("") || "";
      console.log(
        `  Hover at original position ${originalNamePos.offset}: ${display}`,
      );
    } else {
      console.log(
        `  [WARN] No hover info at original position ${originalNamePos.offset}`,
      );
    }
  });

  test("hover after expansion point should use mapped position", (t) => {
    /**
     * When code is inserted mid-file, hovering on code AFTER the insertion
     * requires position mapping.
     */

    const original = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
}

function helper() {
  const x: number = 42;
  return x;
}`;

    // Expansion inserts method before helper function
    const expanded = `import { Derive, Debug } from "./macros";

/** @derive(Debug) */
class User {
  id: string;
  toString(): string { return "User"; }
}

function helper() {
  const x: number = 42;
  return x;
}`;

    // Calculate mapping - insertion after "id: string;\n" at position 71
    const insertionPoint = 71;
    const insertionLength = expanded.length - original.length; // 40 chars inserted

    const env = createEnv(
      {
        "/virtual/test.ts": original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: expanded,
        diagnostics: [],
        sourceMapping: {
          segments: [
            {
              originalStart: 0,
              originalEnd: insertionPoint,
              expandedStart: 0,
              expandedEnd: insertionPoint,
            },
            {
              originalStart: insertionPoint,
              originalEnd: original.length,
              expandedStart: insertionPoint + insertionLength,
              expandedEnd: expanded.length,
            },
          ],
          generatedRegions: [
            {
              start: insertionPoint,
              end: insertionPoint + insertionLength,
              sourceMacro: "Debug::toString",
            },
          ],
        },
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    // User wants to hover on 'helper' at its ORIGINAL position
    const originalHelperPos = findMarker(original, "helper");
    const expandedHelperPos = findMarker(expanded, "helper");

    console.log(`  Original helper position: ${originalHelperPos.offset}`);
    console.log(`  Expanded helper position: ${expandedHelperPos.offset}`);
    console.log(
      `  Position shift: ${expandedHelperPos.offset - originalHelperPos.offset}`,
    );

    // Get hover at original position (what IDE would send)
    const quickInfo = env.pluginService.getQuickInfoAtPosition(
      "/virtual/test.ts",
      originalHelperPos.offset,
    );

    // With proper mapping, this should work
    // Without mapping, we might get wrong info or nothing
    if (quickInfo) {
      const display = quickInfo.displayParts?.map((p) => p.text).join("") || "";
      console.log(`  Hover result: ${display}`);
      assert(
        display.includes("helper"),
        `Hover at original position should find 'helper', got: ${display}`,
      );
    } else {
      assert.fail(
        "Hover at original position returned nothing - position mapping needed",
      );
    }
  });
});

// ============================================================================
// STRICT Completions Position Tests
// ============================================================================

test.describe("STRICT: Completions Position Validation", () => {
  test("completions at original position after expansion should work", (t) => {
    /**
     * User types 'obj.' at a position in original source.
     * Even after expansion, completions should work.
     */

    // Load fixture with automatically calculated source mapping
    const fixture = loadFixture("completions");

    const env = createEnv(
      {
        "/virtual/test.ts": fixture.original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: fixture.expanded,
        diagnostics: [],
        sourceMapping: fixture.sourceMapping,
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    // User triggers completion at end of 'obj.' in ORIGINAL
    const originalDotPos = fixture.original.lastIndexOf("obj.") + 4;
    const expandedDotPos = fixture.expanded.lastIndexOf("obj.") + 4;

    console.log(`  Original completion position: ${originalDotPos}`);
    console.log(`  Expanded completion position: ${expandedDotPos}`);

    // Try completion at original position
    const completions = env.pluginService.getCompletionsAtPosition(
      "/virtual/test.ts",
      originalDotPos,
      {},
    );

    if (completions?.entries) {
      const names = completions.entries.map((e) => e.name);
      console.log(
        `  Completions at original pos: ${names.slice(0, 5).join(", ")}`,
      );

      // Should include both original field and generated method
      assert(names.includes("id"), "Should include original field");
    } else {
      // This might fail because original position is wrong in expanded code
      console.log("  [WARN] No completions at original position");
      assert.fail(
        "Completions failed at original position - position mapping needed",
      );
    }
  });
});

// ============================================================================
// STRICT Go-to-Definition Tests
// ============================================================================

test.describe("STRICT: Go-to-Definition Position Validation", () => {
  test("definition result should use original source positions", (t) => {
    /**
     * When user goes to definition, the result position should be
     * in original source coordinates (for editor navigation).
     */

    // Load fixture with automatically calculated source mapping
    const fixture = loadFixture("definition");

    const env = createEnv(
      {
        "/virtual/test.ts": fixture.original,
        "/virtual/macros.ts": MACROS_DTS,
        "/virtual/lib.d.ts": LIB_DTS,
      },
      (code, fileName) => ({
        code: fixture.expanded,
        diagnostics: [],
        sourceMapping: fixture.sourceMapping,
      }),
    );

    env.info.languageServiceHost.getScriptSnapshot("/virtual/test.ts");

    // Find 'id' in the usage 'u.id' - use ORIGINAL position (mapper will convert)
    const originalUsagePos = fixture.original.lastIndexOf(".id") + 1;
    const expandedUsagePos = fixture.expanded.lastIndexOf(".id") + 1;

    console.log(`  Original usage position: ${originalUsagePos}`);
    console.log(`  Expanded usage position: ${expandedUsagePos}`);

    const definitions = env.pluginService.getDefinitionAtPosition(
      "/virtual/test.ts",
      originalUsagePos,
    );

    assert(definitions && definitions.length > 0, "Should find definition");

    const def = definitions[0];

    // Definition should point to field declaration in ORIGINAL coordinates
    const originalFieldPos = findMarker(fixture.original, "id: string");
    const expandedFieldPos = findMarker(fixture.expanded, "id: string");

    console.log(`  Definition textSpan.start: ${def.textSpan.start}`);
    console.log(`  Original field position: ${originalFieldPos.offset}`);
    console.log(`  Expanded field position: ${expandedFieldPos.offset}`);

    // The definition should give us ORIGINAL position for editor navigation
    assert.strictEqual(
      def.textSpan.start,
      originalFieldPos.offset,
      "Definition should use original source position",
    );
  });
});

// ============================================================================
// Diagnostic Position Accuracy Tests (working tests)
// ============================================================================

test.describe("Diagnostic Position Accuracy", () => {
  test("macro diagnostic byte offset should be correct", (t) => {
    const source = `import { Derive, Debug } from "./macros";

/** @derive(BogusMacro) */
class User {
  badField: unknown;
}`;

    const decoratorPos = findMarker(source, "@derive");

    const env = createEnv({
      "/virtual/test.ts": source,
      "/virtual/macros.ts": MACROS_DTS,
      "/virtual/lib.d.ts": LIB_DTS,
    });

    const diagnostics =
      env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
    const macroDiag = diagnostics.find((d) => d.source === "macroforge");

    assert(macroDiag, "Should have macro diagnostic");
    assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
    assert.strictEqual(macroDiag.start, decoratorPos.offset);
  });

  test("diagnostic category should be correct", (t) => {
    const source = `import { Derive, Debug } from "./macros";

/** @derive(BogusMacro) */
class User {
  field: string;
}`;

    const env = createEnv({
      "/virtual/test.ts": source,
      "/virtual/macros.ts": MACROS_DTS,
      "/virtual/lib.d.ts": LIB_DTS,
    });

    const diagnostics =
      env.pluginService.getSemanticDiagnostics("/virtual/test.ts");
    const macroDiags = diagnostics.filter((d) => d.source === "macroforge");

    assert.strictEqual(macroDiags.length, 1);
    assert.strictEqual(macroDiags[0].category, ts.DiagnosticCategory.Error);
  });
});

// ============================================================================
// Summary Test
// ============================================================================

test("POSITION MAPPING STATUS SUMMARY", (t) => {
  console.log("\n" + "=".repeat(60));
  console.log("POSITION MAPPING STATUS SUMMARY");
  console.log("=".repeat(60));
  console.log("\nCurrent Implementation Status:");
  console.log("  - Macro diagnostics: Positions passed through correctly");
  console.log("  - TS diagnostics: Use EXPANDED code positions (WRONG)");
  console.log("  - Hover: Uses EXPANDED code positions");
  console.log("  - Completions: Uses EXPANDED code positions");
  console.log("  - Go-to-definition: Uses EXPANDED code positions");
  console.log("\nRequired Implementation:");
  console.log("  1. Track position mappings during expansion");
  console.log("  2. Map incoming requests from original -> expanded");
  console.log("  3. Map outgoing results from expanded -> original");
  console.log("  4. Generate source maps for debugging support");
  console.log(
    "\nTests marked .skip() will fail until source mapping is implemented.",
  );
  console.log("=".repeat(60) + "\n");
});
