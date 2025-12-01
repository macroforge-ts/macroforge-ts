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

  const baseDiagnostics = [
    {
      category: ts.DiagnosticCategory.Warning,
      code: 42,
      file: undefined,
      start: 0,
      length: 0,
      messageText: "base warning",
      source: "ts",
    },
  ];

  const languageService = {
    getSemanticDiagnostics: () => baseDiagnostics,
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
    project: {},
  };

  const plugin = initPlugin({ typescript: ts });
  const languageServiceWithPlugin = plugin.create(info);

  return {
    fileName,
    info,
    languageServiceWithPlugin,
  };
}

test("expands macro-enabled files through script snapshots", (t) => {
  const source = `
    import { Derive } from "@macro/derive";
    /** @derive(Debug) */
    class User {}
  `;

  const env = createPluginEnvironment(source);
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, "snapshot should exist");
  const expanded = snapshot.getText(0, snapshot.getLength());
  assert(
    expanded.includes("toString"),
    "expanded snapshot should include generated method",
  );
});

test("skips expansion for files without macros", (t) => {
  const env = createPluginEnvironment("class Plain {}");
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, "snapshot should exist");
  assert(snapshot.getText(0, snapshot.getLength()).includes("class Plain"));
});

test("merges macro diagnostics with TypeScript diagnostics preserving locations", (t) => {
  const source = `
    import { Derive } from "@macro/derive";
    /** @derive(BogusMacro) */
    class Broken {
      id: string;
    }
  `;

  const env = createPluginEnvironment(source);
  env.info.languageServiceHost.getScriptSnapshot(env.fileName);

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(
    env.fileName,
  );
  assert(diagnostics.length >= 1, "should return diagnostics");
  const macroDiag = diagnostics.find((diag) => diag.source === "ts-macros");
  assert(macroDiag, "macro diagnostic missing");
  assert.strictEqual(macroDiag.code, 9999);
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
});

test("falls back gracefully when expansion throws", (t) => {
  const env = createPluginEnvironment(`
    import { Derive } from "@macro/derive";
    /** @derive(Debug) */
    class User {
      // Intentional syntax error to force expansion failure
      broken: string = ;
    }
  `);

  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, "snapshot should exist");
  assert(snapshot.getText(0, snapshot.getLength()).includes("class User"));

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(
    env.fileName,
  );
  assert(diagnostics.length >= 1, "should surface diagnostics on failure");
  const macroDiag = diagnostics.find((diag) => diag.source === "ts-macros");
  assert(
    macroDiag,
    "macro diagnostic should be appended when expansion cannot proceed",
  );
});
