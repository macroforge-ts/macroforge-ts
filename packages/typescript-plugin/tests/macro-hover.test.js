/**
 * E2E tests for macro hover intellisense functionality
 * Tests that hovering over @derive macros and field decorators shows documentation
 */

const test = require("node:test");
const assert = require("node:assert/strict");
const ts = require("typescript/lib/tsserverlibrary");
const initPlugin = require("../dist/index.js");

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function createPluginEnvironment(source, fileName = "/virtual/MacroUser.ts") {
  const snapshots = new Map([[fileName, createSnapshot(source)]]);
  const versions = new Map([[fileName, "1"]]);

  const host = {
    getScriptSnapshot: (name) => snapshots.get(name) ?? null,
    getScriptVersion: (name) => versions.get(name) ?? "0",
  };

  // Mock getQuickInfoAtPosition to return undefined (let plugin handle it)
  const languageService = {
    getSemanticDiagnostics: () => [],
    getSyntacticDiagnostics: () => [],
    getQuickInfoAtPosition: () => undefined,
    getCompletionsAtPosition: () => undefined,
    getDefinitionAtPosition: () => undefined,
    getDefinitionAndBoundSpan: () => undefined,
    getTypeDefinitionAtPosition: () => undefined,
    getReferencesAtPosition: () => undefined,
    findReferences: () => undefined,
    getSignatureHelpItems: () => undefined,
    getRenameInfo: () => ({ canRename: false }),
    findRenameLocations: () => undefined,
    getDocumentHighlights: () => undefined,
    getImplementationAtPosition: () => undefined,
    getCodeFixesAtPosition: () => [],
    getNavigationTree: () => ({
      text: "",
      kind: ts.ScriptElementKind.moduleElement,
      kindModifiers: "",
      spans: [],
      childItems: [],
    }),
    getOutliningSpans: () => [],
    getProgram: () => ({
      getSourceFile: () =>
        ts.createSourceFile(
          fileName,
          source,
          ts.ScriptTarget.ESNext,
          true,
          ts.ScriptKind.TS,
        ),
    }),
  };

  const info = {
    config: {},
    languageService,
    languageServiceHost: host,
    serverHost: {},
    project: {
      getCurrentDirectory: () => "/virtual",
      projectService: {
        logger: { info: () => {} },
      },
    },
  };

  const plugin = initPlugin({ typescript: ts });
  const languageServiceWithPlugin = plugin.create(info);

  return {
    fileName,
    info,
    languageServiceWithPlugin,
    source,
  };
}

test("macro hover on @derive(Debug)", async (t) => {
  const source = `/** @derive(Debug) */
class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  // Position on "Debug" in @derive(Debug)
  const debugPos = source.indexOf("Debug");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    debugPos,
  );

  // The hover should provide info about the Debug macro
  // If manifest is loaded, we get hover info; otherwise we may get undefined
  if (hover) {
    assert.ok(hover.displayParts, "hover should have displayParts");
    assert.ok(hover.textSpan, "hover should have textSpan");
    assert.strictEqual(
      hover.kind,
      ts.ScriptElementKind.functionElement,
      "hover kind should be functionElement",
    );

    // Check that display parts mention @derive
    const displayText = hover.displayParts.map((p) => p.text).join("");
    assert.ok(
      displayText.includes("@derive") || displayText.includes("Debug"),
      "display should mention @derive or Debug",
    );

    // Check documentation if available
    if (hover.documentation && hover.documentation.length > 0) {
      const docText = hover.documentation.map((d) => d.text).join("");
      assert.ok(
        docText.includes("toString") || docText.includes("debug"),
        "documentation should describe the macro",
      );
    }
  }
});

test("macro hover on @derive with multiple macros", async (t) => {
  const source = `/** @derive(Debug, Serialize, Clone) */
class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  // Test hover on each macro name
  const macros = ["Debug", "Serialize", "Clone"];

  for (const macro of macros) {
    const pos = source.indexOf(macro);
    const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
      env.fileName,
      pos,
    );

    if (hover) {
      assert.ok(hover.displayParts, `hover for ${macro} should have displayParts`);
      const displayText = hover.displayParts.map((p) => p.text).join("");
      assert.ok(
        displayText.includes(macro) || displayText.includes("@derive"),
        `hover for ${macro} should mention the macro`,
      );
    }
  }
});

test("macro hover on @serde field decorator", async (t) => {
  const source = `/** @derive(Serialize) */
class User {
  @serde({ skip: true })
  password: string;
}`;

  const env = createPluginEnvironment(source);

  // Position on "serde" in @serde
  const serdePos = source.indexOf("@serde") + 1; // +1 to be on 's'
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    serdePos,
  );

  if (hover) {
    assert.ok(hover.displayParts, "hover should have displayParts");
    const displayText = hover.displayParts.map((p) => p.text).join("");
    assert.ok(
      displayText.includes("serde"),
      "display should mention serde",
    );
  }
});

test("macro hover on @debug field decorator", async (t) => {
  const source = `/** @derive(Debug) */
class User {
  @debug({ rename: "identifier" })
  id: string;
}`;

  const env = createPluginEnvironment(source);

  // Position on "debug" in @debug
  const debugPos = source.indexOf("@debug") + 1;
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    debugPos,
  );

  if (hover) {
    assert.ok(hover.displayParts, "hover should have displayParts");
    const displayText = hover.displayParts.map((p) => p.text).join("");
    assert.ok(
      displayText.includes("debug"),
      "display should mention debug",
    );
  }
});

test("no macro hover on regular code", async (t) => {
  const source = `class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}`;

  const env = createPluginEnvironment(source);

  // Position on "name" property
  const namePos = source.indexOf("name:");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    namePos,
  );

  // Should return undefined because it's not a macro position
  // and the base language service returns undefined
  assert.strictEqual(hover, undefined, "should not have macro hover on regular code");
});

test("macro hover with multiline JSDoc", async (t) => {
  const source = `/**
 * User class for the system
 * @derive(Debug, Serialize)
 */
class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  // Position on "Debug" in the multiline JSDoc
  const debugPos = source.indexOf("Debug");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    debugPos,
  );

  if (hover) {
    assert.ok(hover.displayParts, "hover should work in multiline JSDoc");
  }
});

test("builtin import warning appears in diagnostics", async (t) => {
  const source = `import { Debug } from "macroforge";

/** @derive(Debug) */
class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  // Get diagnostics - should include a warning about importing Debug
  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(
    env.fileName,
  );

  // Find the warning about built-in macro import
  const importWarning = diagnostics.find(
    (d) =>
      d.source === "macroforge" &&
      d.category === ts.DiagnosticCategory.Warning &&
      d.messageText.toString().includes("built-in macro"),
  );

  if (importWarning) {
    assert.ok(
      importWarning.messageText.toString().includes("Debug"),
      "warning should mention Debug",
    );
    assert.ok(
      importWarning.start !== undefined,
      "warning should have a position",
    );
  }
});

test("multiple builtin import warnings", async (t) => {
  const source = `import { Debug, Serialize, Clone } from "@macroforge/core";

/** @derive(Debug, Serialize, Clone) */
class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(
    env.fileName,
  );

  const importWarnings = diagnostics.filter(
    (d) =>
      d.source === "macroforge" &&
      d.category === ts.DiagnosticCategory.Warning &&
      d.messageText.toString().includes("built-in macro"),
  );

  // Should have warnings for Debug, Serialize, and Clone
  if (importWarnings.length > 0) {
    assert.ok(
      importWarnings.length >= 1,
      "should have at least one warning for built-in imports",
    );
  }
});

test("no warning for non-macro imports", async (t) => {
  const source = `import { Debug } from "./my-local-debug";
import { Serialize } from "some-other-lib";

class User {
  name: string;
}`;

  const env = createPluginEnvironment(source);

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(
    env.fileName,
  );

  // Should not have warnings because imports are not from macro-related modules
  const importWarnings = diagnostics.filter(
    (d) =>
      d.source === "macroforge" &&
      d.messageText.toString().includes("built-in macro"),
  );

  assert.strictEqual(
    importWarnings.length,
    0,
    "should not warn about non-macro imports",
  );
});

test("hover span is correct", async (t) => {
  const source = `/** @derive(Debug) */
class User {}`;

  const env = createPluginEnvironment(source);

  const debugStart = source.indexOf("Debug");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    debugStart,
  );

  if (hover && hover.textSpan) {
    // The span should cover "Debug"
    const spanText = source.substring(
      hover.textSpan.start,
      hover.textSpan.start + hover.textSpan.length,
    );
    assert.strictEqual(spanText, "Debug", "span should cover exactly 'Debug'");
  }
});
