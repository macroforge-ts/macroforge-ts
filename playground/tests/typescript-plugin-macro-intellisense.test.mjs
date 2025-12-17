import { describe, test } from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { createRequire } from "node:module";
import { repoRoot } from "./test-utils.mjs";

const require = createRequire(import.meta.url);
const ts = require("typescript");

function createMockLanguageService(tsModule, fileName, fileText) {
  const sourceFile = tsModule.createSourceFile(
    fileName,
    fileText,
    tsModule.ScriptTarget.Latest,
    true,
  );

  return {
    getProgram: () => ({
      getSourceFile: (requested) => (requested === fileName ? sourceFile : undefined),
    }),
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
    getNavigationTree: () => ({ text: "", kind: "", spans: [], childItems: [] }),
    getOutliningSpans: () => [],
  };
}

function createHost(tsModule, fileName, fileText, cwd) {
  const snapshot = tsModule.ScriptSnapshot.fromString(fileText);

  return {
    getCompilationSettings: () => ({
      strict: true,
      target: tsModule.ScriptTarget.ES2022,
      module: tsModule.ModuleKind.ESNext,
    }),
    getScriptFileNames: () => [fileName],
    getScriptVersion: () => "1",
    getScriptSnapshot: (requested) => (requested === fileName ? snapshot : undefined),
    getCurrentDirectory: () => cwd,
    getDefaultLibFileName: (opts) => tsModule.getDefaultLibFilePath(opts),
    fileExists: (p) => p === fileName,
    readFile: () => undefined,
    readDirectory: () => [],
  };
}

function initPluginForFile({ fileName, fileText }) {
  const pluginPath = path.resolve(repoRoot, "packages/typescript-plugin/dist/index.js");
  const tsPluginInit = require(pluginPath).default;
  const pluginFactory = tsPluginInit({ typescript: ts });

  const cwd = repoRoot;
  const host = createHost(ts, fileName, fileText, cwd);
  const languageService = createMockLanguageService(ts, fileName, fileText);

  const info = {
    project: {
      getCurrentDirectory: () => cwd,
      projectService: { logger: { info: () => {} } },
    },
    languageService,
    languageServiceHost: host,
    config: {},
  };

  pluginFactory.create(info);
  return info.languageService;
}

describe("TypeScript plugin macro hover + attribute diagnostics", () => {
  test("hover over @derive macro name returns QuickInfo docs", () => {
    const fileText = `
      /** @derive(Serialize, Deserialize) */
      export interface User {
        id: string;
      }
    `;
    const fileName = path.join(repoRoot, "playground/tests/.tmp-macro-hover.ts");
    const ls = initPluginForFile({ fileName, fileText });

    const pos = fileText.indexOf("Serialize") + 1;
    assert.ok(pos > 0, "sanity: expected to find Serialize");

    const info = ls.getQuickInfoAtPosition(fileName, pos);
    assert.ok(info, "expected QuickInfo for Serialize");
    const docText = (info.documentation ?? []).map((d) => d.text).join("\n");
    assert.ok(
      docText.toLowerCase().includes("serialization methods"),
      `expected Serialize documentation, got: ${docText}`,
    );
    assert.equal(fileText.slice(info.textSpan.start, info.textSpan.start + info.textSpan.length), "Serialize");
  });

  test("hover over @serde decorator returns QuickInfo docs", () => {
    const fileText = `
      /** @derive(Deserialize) */
      export interface User {
        /** @serde({ validate: ["email"] }) */
        email: string;
      }
    `;
    const fileName = path.join(repoRoot, "playground/tests/.tmp-decorator-hover.ts");
    const ls = initPluginForFile({ fileName, fileText });

    const pos = fileText.indexOf("@serde") + 2;
    assert.ok(pos > 1, "sanity: expected to find @serde");

    const info = ls.getQuickInfoAtPosition(fileName, pos);
    assert.ok(info, "expected QuickInfo for @serde");
    const docText = (info.documentation ?? []).map((d) => d.text).join("\n");
    assert.ok(
      docText.toLowerCase().includes("configure") &&
        docText.toLowerCase().includes("for this field"),
      `expected serde documentation, got: ${docText}`,
    );
    assert.equal(
      fileText.slice(info.textSpan.start, info.textSpan.start + info.textSpan.length),
      "@serde",
    );
  });

  test("invalid @serde validate attribute surfaces as semantic diagnostic", () => {
    const fileText = `
      /** @derive(Deserialize) */
      export interface User {
        /** @serde({ validate: ["doesNotExist"] }) */
        name: string;
      }
    `;
    const fileName = path.join(repoRoot, "playground/tests/.tmp-serde-diag.ts");
    const ls = initPluginForFile({ fileName, fileText });

    const diags = ls.getSemanticDiagnostics(fileName);
    const macroDiags = diags.filter((d) => d.source === "macroforge");
    assert.ok(macroDiags.length >= 1, "expected at least one macroforge diagnostic");

    const message = String(macroDiags[0].messageText);
    assert.ok(
      message.toLowerCase().includes("unknown validator"),
      `expected unknown validator error, got: ${message}`,
    );
    assert.ok(
      typeof macroDiags[0].start === "number" && macroDiags[0].start > 0,
      "expected diagnostic start to be set",
    );

    const serdeStart = fileText.indexOf("@serde");
    assert.ok(serdeStart >= 0, "sanity: expected @serde in file");
    assert.ok(
      macroDiags[0].start >= serdeStart - 4 && macroDiags[0].start <= serdeStart + 4,
      `expected diagnostic near @serde (got start=${macroDiags[0].start}, serdeStart=${serdeStart})`,
    );
  });
});
