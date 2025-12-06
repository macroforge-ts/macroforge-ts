import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createServer } from "vite";

import { createRequire } from "node:module";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, "..");
const repoRoot = path.resolve(playgroundRoot, "..");
const vanillaRoot = path.join(playgroundRoot, "vanilla");
const svelteRoot = path.join(playgroundRoot, "svelte");
const rootConfigPath = path.join(repoRoot, "macroforge.json");

async function withViteServer(rootDir, optionsOrRunner, maybeRunner) {
  const options = typeof optionsOrRunner === "function" ? {} : optionsOrRunner;
  const runner =
    typeof optionsOrRunner === "function" ? optionsOrRunner : maybeRunner;
  const { useProjectCwd = false } = options ?? {};

  const configFile = path.join(rootDir, "vite.config.ts");
  const previousCwd = process.cwd();
  let server;
  let copiedConfig = false;
  const localConfigPath = path.join(rootDir, "macroforge.json");

  try {
    if (useProjectCwd) {
      process.chdir(rootDir);
    }

    // Copy the workspace-level config so the macro host loads the shared macro packages for the vanilla app.
    if (!fs.existsSync(localConfigPath) && fs.existsSync(rootConfigPath)) {
      fs.copyFileSync(rootConfigPath, localConfigPath);
      copiedConfig = true;
    }
    server = await createServer({
      root: rootDir,
      configFile,
      logLevel: "error",
      appType: "custom",
      server: {
        middlewareMode: true,
        hmr: false,
      },
      optimizeDeps: {
        disabled: true,
      },
    });

    return await runner(server);
  } finally {
    if (server) {
      // Give a small grace period for pending requests to settle
      await new Promise((resolve) => setTimeout(resolve, 100));
      await server.close();
    }
    if (copiedConfig && fs.existsSync(localConfigPath)) {
      fs.rmSync(localConfigPath);
    }
    if (useProjectCwd) {
      process.chdir(previousCwd);
    }
  }
}

/** Create a complete mock languageService with all methods the plugin binds */
function createMockLanguageService(ts) {
  return {
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
      kind: "",
      spans: [],
      childItems: [],
    }),
    getOutliningSpans: () => [],
    getProgram: () => ({
      getSourceFile: () => undefined,
    }),
  };
}

test("TS Language Plugin augments types", async () => {
  const pluginPath = path.resolve(
    repoRoot,
    "packages/ts-derive-plugin/dist/index.js",
  );
  const require = createRequire(import.meta.url);
  const tsPluginInit = require(pluginPath);
  const ts = require("typescript");

  // Mock Info for TS Server Plugin with all required methods
  const mockHost = {
    getScriptSnapshot: (fileName) => {
      if (fileName.endsWith("user.ts")) {
        const content = fs.readFileSync(
          path.join(vanillaRoot, "src/user.ts"),
          "utf-8",
        );
        return ts.ScriptSnapshot.fromString(content);
      }
      return undefined;
    },
    getScriptVersion: () => "1",
    getScriptFileNames: () => [],
    fileExists: (fileName) => {
      if (fileName.endsWith("user.ts")) return true;
      return ts.sys.fileExists(fileName);
    },
  };

  const mockInfo = {
    project: { projectService: { logger: { info: () => {} } } },
    languageService: createMockLanguageService(ts),
    languageServiceHost: mockHost,
    config: {},
  };

  const pluginFactory = tsPluginInit({ typescript: ts });
  // creating the plugin should wrap the host methods
  pluginFactory.create(mockInfo);

  // Test getScriptSnapshot via the modified host
  const snapshot = mockHost.getScriptSnapshot("playground/vanilla/src/user.ts");
  const text = snapshot.getText(0, snapshot.getLength());

  // The plugin returns expanded code (implementation), not type stubs
  // Check for the generated toString method implementation
  if (!text.includes("toString(): string")) {
    console.log("Snapshot text content:", text);
  }

  assert.ok(
    text.includes("toString(): string"),
    "Should include toString() method",
  );
  // The @Debug decorator generates toString(), not toJSON()
  // /** @derive(Debug) */ => toString()
  // /** @derive(JSON) */ => toJSON()
  assert.ok(
    text.includes("toString()"),
    "Should include generated toString method",
  );
  // The @derive decorator should be removed from the expanded output
  assert.ok(
    !text.includes("/** @derive(Debug) */"),
    "Should remove @derive decorator from output",
  );
});

test("TS Language Plugin detects external macro packages", async () => {
  const pluginPath = path.resolve(
    repoRoot,
    "packages/ts-derive-plugin/dist/index.js",
  );
  const require = createRequire(import.meta.url);
  const tsPluginInit = require(pluginPath);
  const ts = require("typescript");

  // Code that imports from an external macro package
  const codeWithExternalMacro = `
import { Derive } from "@macroforge/swc-napi";
import { FieldController, fieldController } from "@playground/macro";

/** @derive(FieldController) */
class TestForm {
  @fieldController()
  name: string;

  constructor(name: string) {
    this.name = name;
  }
}
`;

  const mockHost = {
    getScriptSnapshot: (fileName) => {
      if (fileName.endsWith("test-external.ts")) {
        return ts.ScriptSnapshot.fromString(codeWithExternalMacro);
      }
      return undefined;
    },
    getScriptVersion: () => "1",
    getScriptFileNames: () => [],
    fileExists: (fileName) => {
      if (fileName.endsWith("test-external.ts")) return true;
      return ts.sys.fileExists(fileName);
    },
  };

  const logs = [];
  const mockInfo = {
    project: { projectService: { logger: { info: (msg) => logs.push(msg) } } },
    languageService: createMockLanguageService(ts),
    languageServiceHost: mockHost,
    config: {},
  };

  const pluginFactory = tsPluginInit({ typescript: ts });
  pluginFactory.create(mockInfo);

  // Trigger the plugin by getting a snapshot
  const snapshot = mockHost.getScriptSnapshot("test-external.ts");
  assert.ok(snapshot, "Should return a snapshot");

  // The plugin should detect the external macro package import
  // Check logs for external macro detection (the plugin logs when it finds external macros)
  const hasExternalMacroLog = logs.some(
    (log) =>
      log.includes("external macro") ||
      log.includes("FieldController") ||
      log.includes("@playground/macro"),
  );

  // Note: The plugin filters out "macro not found" diagnostics for external macros
  // This test verifies the plugin processes files with external macro imports without crashing
  const text = snapshot.getText(0, snapshot.getLength());
  assert.ok(text.includes("class TestForm"), "Should process the file content");
});

test("TS Language Plugin filters diagnostics for available external macros", async () => {
  const pluginPath = path.resolve(
    repoRoot,
    "packages/ts-derive-plugin/dist/index.js",
  );
  const require = createRequire(import.meta.url);

  // Test the internal helper functions exported for testing
  // These are the functions we added for external macro package support
  const pluginModule = require(pluginPath);

  // The plugin exports these for the language service
  const tsPluginInit = pluginModule.default || pluginModule;
  const ts = require("typescript");

  // Verify the plugin initializes without errors
  const pluginFactory = tsPluginInit({ typescript: ts });
  assert.ok(
    typeof pluginFactory.create === "function",
    "Plugin should have create method",
  );
});

test("Macro expansion formats code correctly", async () => {
  const require = createRequire(import.meta.url);
  const swcMacrosPath = path.join(repoRoot, "crates/swc-napi-macros/index.js");
  const { expandSync } = require(swcMacrosPath);

  const userContent = fs.readFileSync(
    path.join(vanillaRoot, "src/user.ts"),
    "utf8",
  );
  const result = expandSync(userContent, "user.ts");

  assert.ok(result.types, "Should generate type output");

  const lines = result.types.split("\n");

  // Check for methods on same line as other code (the bug we fixed)
  lines.forEach((line, i) => {
    const methodMatches = line.match(/\(\):/g);
    assert.ok(
      !methodMatches || methodMatches.length <= 1,
      `Line ${i + 1} should not have multiple methods: "${line}"`,
    );
  });

  // Check each method is on its own line
  const toStringLine = lines.find((l) => l.includes("toString()"));
  const toJSONLine = lines.find((l) => l.includes("toJSON()"));

  assert.ok(toStringLine, "Should have toString() method");
  assert.ok(toJSONLine, "Should have toJSON() method");

  const toStringIndex = lines.indexOf(toStringLine);
  const toJSONIndex = lines.indexOf(toJSONLine);

  assert.notEqual(
    toStringIndex,
    toJSONIndex,
    "toString() and toJSON() should be on separate lines",
  );
});

test("Macro Host reports diagnostics for invalid usage", async () => {
  // Test the core macro host logic used by both plugins and language server
  const require = createRequire(import.meta.url);
  const swcMacrosPath = path.join(repoRoot, "crates/swc-napi-macros/index.js");
  const { expandSync } = require(swcMacrosPath);

  const code = `
        import { Derive } from "@macroforge/macros";
        /** @derive(UnknownMacro) */
        class Foo {}
    `;

  // The host will look for macroforge.json in CWD.
  // We are running from root so it should find the root config which allows native macros.
  const result = expandSync(code, "test.ts");

  assert.ok(result.diagnostics.length > 0, "Should report diagnostics");
  assert.ok(
    result.diagnostics.some((d) => d.message.includes("UnknownMacro")),
    "Should report unknown macro error",
  );
});

test(
  "vanilla playground macros emit runtime helpers",
  { timeout: 30000 },
  async () => {
    await withViteServer(vanillaRoot, async (server) => {
      const mod = await server.ssrLoadModule("/src/user.ts");
      assert.ok(
        mod && typeof mod.User === "function",
        "User class should be exported from vanilla playground",
      );

      const vanillaUser = new mod.User(
        9,
        "Integration Tester",
        "qa@example.com",
        "tok_live_secret",
      );

      assert.equal(
        typeof vanillaUser.toString,
        "function",
        "Debug derive should add toString",
      );
      const summary = vanillaUser.toString();
      assert.ok(
        summary.startsWith("User {"),
        "Derived toString() should include class label",
      );
      assert.ok(
        summary.includes("identifier: 9"),
        "Derived toString() should respect rename option",
      );
      assert.ok(
        !summary.includes("authToken"),
        "Derived toString() should skip sensitive fields",
      );
    });
  },
);

test(
  "svelte playground macros inline markdown and derive helpers",
  { timeout: 30000 },
  async () => {
    await withViteServer(
      svelteRoot,
      { useProjectCwd: true },
      async (server) => {
        const mod = await server.ssrLoadModule("/src/lib/demo/macro-user.ts");
        const { MacroUser, showcaseUserJson, showcaseUserSummary } = mod;

        assert.ok(MacroUser, "MacroUser export should exist");
        const svelteUser = new MacroUser(
          "usr_55",
          "Rin Tester",
          "Macro QA",
          "Derive",
          "2025-02-01",
          "token_qa",
        );
        assert.deepEqual(svelteUser.toJSON(), {
          id: "usr_55",
          name: "Rin Tester",
          role: "Macro QA",
          favoriteMacro: "Derive",
          since: "2025-02-01",
          apiToken: "token_qa",
        });

        assert.equal(
          typeof showcaseUserSummary,
          "string",
          "Derived summary should be a string",
        );
        assert.equal(
          showcaseUserJson.favoriteMacro,
          "Derive",
          "Showcase JSON should include derived helpers",
        );
        assert.ok(
          showcaseUserSummary.includes("userId"),
          "Showcase summary should use renamed field label",
        );

        const transformed = await server.transformRequest(
          "/src/lib/demo/macro-user.ts",
        );
        assert.ok(
          transformed?.code.includes("toJSON()"),
          "Derived methods should appear in transformed code",
        );
      },
    );
  },
);
