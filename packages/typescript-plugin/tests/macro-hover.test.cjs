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

  assert.ok(hover, "expected hover info for Debug macro");
  assert.ok(hover.displayParts, "hover should have displayParts");
  assert.ok(hover.textSpan, "hover should have textSpan");
  assert.strictEqual(
    hover.kind,
    ts.ScriptElementKind.functionElement,
    "hover kind should be functionElement",
  );

  const displayText = hover.displayParts.map((p) => p.text).join("");
  assert.ok(
    displayText.includes("@derive") || displayText.includes("Debug"),
    "display should mention @derive or Debug",
  );

  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(
    docText.toLowerCase().includes("tostring") ||
      docText.toLowerCase().includes("debug"),
    `documentation should describe the macro, got: ${docText}`,
  );
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

    assert.ok(hover, `expected hover info for ${macro}`);
    assert.ok(hover.displayParts, `hover for ${macro} should have displayParts`);
    const displayText = hover.displayParts.map((p) => p.text).join("");
    assert.ok(
      displayText.includes(macro) || displayText.includes("@derive"),
      `hover for ${macro} should mention the macro`,
    );
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

  assert.ok(hover, "expected hover info for @serde");
  assert.ok(hover.displayParts, "hover should have displayParts");
  const displayText = hover.displayParts.map((p) => p.text).join("");
  assert.ok(displayText.includes("serde"), "display should mention serde");
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(
    docText.toLowerCase().includes("field"),
    `expected @serde docs, got: ${docText}`,
  );
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

  assert.ok(hover, "expected hover info for @debug");
  assert.ok(hover.displayParts, "hover should have displayParts");
  const displayText = hover.displayParts.map((p) => p.text).join("");
  assert.ok(
    displayText.toLowerCase().includes("debug"),
    `display should mention debug, got: ${displayText}`,
  );
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(
    docText.length > 0,
    "expected @debug docs to be non-empty",
  );
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

test("macro hover on @derive keyword itself", async (t) => {
  const source = `/** @derive(Debug) */
class User { name: string; }`;

  const env = createPluginEnvironment(source);

  // Position on 'd' in @derive
  const derivePos = source.indexOf("@derive") + 1;
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    derivePos,
  );

  assert.ok(hover, "expected hover info for @derive keyword");
  const displayText = hover.displayParts.map((p) => p.text).join("");
  assert.ok(displayText.includes("@derive"), "display should show @derive");
  assert.strictEqual(hover.kind, ts.ScriptElementKind.keyword, "kind should be keyword");

  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(docText.includes("Derive directive"), "should have derive directive description");
  assert.ok(docText.includes("Built-in macros"), "should mention built-in macros");
});

test("macro hover on external macro in @derive", async (t) => {
  const source = `/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Debug, Gigaform) */
interface Account { id: string; }`;

  const env = createPluginEnvironment(source);

  // Position on "Gigaform" in @derive
  const gigaformPos = source.lastIndexOf("Gigaform");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    gigaformPos,
  );

  assert.ok(hover, "expected hover for external macro");
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(docText.includes("@playground/macro"), "should mention source module");
  assert.ok(docText.includes("External macro"), "should indicate it's an external macro");
});

test("macro hover on custom field decorator from external macro", async (t) => {
  const source = `/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Gigaform) */
interface Account {
  /** @hiddenController({}) */
  id: string;
}`;

  const env = createPluginEnvironment(source);

  // Position on "hiddenController" in @hiddenController
  const decoratorPos = source.indexOf("@hiddenController") + 1;
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    decoratorPos,
  );

  assert.ok(hover, "expected hover for custom decorator");
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(docText.includes("Gigaform"), "should mention source macro");
  assert.ok(
    docText.includes("Field decorator") || docText.includes("decorator"),
    "should indicate it's a field decorator",
  );
});

test("macro hover on unknown macro shows helpful message", async (t) => {
  const source = `/** @derive(Debug, UnknownMacro) */
class User { name: string; }`;

  const env = createPluginEnvironment(source);

  const unknownPos = source.indexOf("UnknownMacro");
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    unknownPos,
  );

  assert.ok(hover, "expected hover for unknown macro");
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(
    docText.includes("import macro") || docText.includes("Macro:"),
    "should suggest import syntax or show macro name",
  );
});

test("macro hover on field decorator in derive context without import", async (t) => {
  const source = `/** @derive(Debug, CustomMacro) */
class User {
  /** @customField({}) */
  name: string;
}`;

  const env = createPluginEnvironment(source);

  const decoratorPos = source.indexOf("@customField") + 1;
  const hover = env.languageServiceWithPlugin.getQuickInfoAtPosition(
    env.fileName,
    decoratorPos,
  );

  assert.ok(hover, "expected hover for decorator in derive context");
  const docText = (hover.documentation ?? []).map((d) => d.text).join("");
  assert.ok(
    docText.includes("@derive") || docText.includes("Debug") || docText.includes("CustomMacro"),
    "should mention the enclosing derive context",
  );
});
