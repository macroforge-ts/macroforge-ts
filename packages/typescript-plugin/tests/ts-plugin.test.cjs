const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
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

test("skips expansion for files with JSDoc comments but no macros", (t) => {
  // This source has various JSDoc patterns that should NOT trigger macro expansion
  const source = `
/**
 * A user class with various JSDoc decorators that are NOT macros.
 * @description This is a user management class
 * @author John Doe
 * @version 1.0.0
 * @see https://example.com/docs
 */
export class User {
  /**
   * The user's unique identifier.
   * @type {string}
   * @readonly
   */
  readonly id: string;

  /**
   * The user's email address.
   * @type {string | null}
   * @default null
   */
  email: string | null;

  /**
   * The user's display name.
   * @param {string} firstName - First name
   * @param {string} lastName - Last name
   * @returns {string} Full name
   */
  getFullName(firstName: string, lastName: string): string {
    return firstName + " " + lastName;
  }

  /**
   * @deprecated Use getFullName instead
   * @throws {Error} When name is invalid
   * @example
   * const user = new User();
   * user.getName(); // "John Doe"
   */
  getName(): string {
    return "deprecated";
  }
}

/**
 * Configuration interface.
 * @interface
 * @property {string} host - The host name
 * @property {number} port - The port number
 */
export interface Config {
  host: string;
  port: number;
}

/**
 * Status enumeration.
 * @enum {string}
 */
export enum Status {
  /** Active status */
  Active = "active",
  /** Inactive status */
  Inactive = "inactive",
}

/**
 * Type alias for dimensions.
 * @typedef {Object} Dimensions
 */
export type Dimensions = {
  width: number;
  height: number;
};
`;

  const env = createPluginEnvironment(source, "/virtual/NoMacros.ts");
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, "snapshot should exist");

  const result = snapshot.getText(0, snapshot.getLength());

  // Should contain original code unchanged
  assert(result.includes("export class User"), "should contain original class");
  assert(result.includes("export interface Config"), "should contain original interface");
  assert(result.includes("export enum Status"), "should contain original enum");
  assert(result.includes("export type Dimensions"), "should contain original type");

  // Should NOT contain any generated macro code
  assert(!result.includes("toString()"), "should not generate toString method");
  assert(!result.includes("clone()"), "should not generate clone method");
  assert(!result.includes("equals("), "should not generate equals method");
  assert(!result.includes("defaultValue()"), "should not generate defaultValue method");
  assert(!result.includes("toJSON()"), "should not generate toJSON method");
  assert(!result.includes("fromJSON("), "should not generate fromJSON method");

  // Verify no macro diagnostics are generated
  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(env.fileName);
  const macroDiag = diagnostics.find((diag) => diag.source === "macroforge");
  assert(!macroDiag, "should not have any macro diagnostics for non-macro file");
});

test("merges macro diagnostics with TypeScript diagnostics preserving locations", (t) => {
  const source = `
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
  const macroDiag = diagnostics.find((diag) => diag.source === "macroforge");
  assert(macroDiag, "macro diagnostic missing");
  assert.strictEqual(macroDiag.code, 9999);
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
});

test("falls back gracefully when expansion throws", (t) => {
  const env = createPluginEnvironment(`
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
  const macroDiag = diagnostics.find((diag) => diag.source === "macroforge");
  assert(
    macroDiag,
    "macro diagnostic should be appended when expansion cannot proceed",
  );
});

test("skips expansion for real-world JSDoc file with @derive in comments (fixture)", (t) => {
  // Load the fixture file that contains @derive mentioned in JSDoc comments
  // but not as actual macro decorators
  const fixturePath = path.join(__dirname, "fixtures", "no-macros-jsdoc.ts");
  const source = fs.readFileSync(fixturePath, "utf-8");

  const env = createPluginEnvironment(source, "/virtual/no-macros-jsdoc.ts");
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, "snapshot should exist");

  const result = snapshot.getText(0, snapshot.getLength());

  // The file mentions @derive in JSDoc example comments, but should NOT trigger expansion
  // because they're inside code blocks/comments, not actual decorators
  assert(result.includes("export { createFormState }"), "should contain original exports");
  assert(result.includes("export { gigaformEnhance }"), "should contain original exports");
  assert(result.includes("@derive(Default, Serialize"), "should preserve JSDoc example content");

  // Should NOT have any macro-generated code
  assert(!result.includes("static defaultValue()"), "should not generate defaultValue method");
  assert(!result.includes("toJSON()"), "should not generate toJSON method");
  assert(!result.includes("fromJSON("), "should not generate fromJSON method");

  // Verify no macro diagnostics
  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(env.fileName);
  const macroDiag = diagnostics.find((diag) => diag.source === "macroforge");
  assert(!macroDiag, "should not have any macro diagnostics for file with @derive only in comments");
});
