const test = require('node:test');
const assert = require('node:assert/strict');
const ts = require('typescript/lib/tsserverlibrary');
const initPlugin = require('../dist/index.js');

function createExpandStub(result) {
  const stub = (code, fileName) => {
    stub.calls.push({ code, fileName });
    if (result instanceof Error) {
      throw result;
    }
    return result;
  };
  stub.calls = [];
  return stub;
}

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function createPluginEnvironment(source, fileName = '/virtual/MacroUser.ts') {
  const snapshots = new Map([[fileName, createSnapshot(source)]]);
  const versions = new Map([[fileName, '1']]);

  const host = {
    getScriptSnapshot: (name) => snapshots.get(name) ?? null,
    getScriptVersion: (name) => versions.get(name) ?? '0'
  };

  const baseDiagnostics = [
    {
      category: ts.DiagnosticCategory.Warning,
      code: 42,
      file: undefined,
      start: 0,
      length: 0,
      messageText: 'base warning',
      source: 'ts'
    }
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
    getNavigationTree: () => ({ text: '', kind: ts.ScriptElementKind.moduleElement, kindModifiers: '', spans: [], childItems: [] }),
    getOutliningSpans: () => [],
    getProgram: () => ({
      getSourceFile: () =>
        ts.createSourceFile(fileName, source, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS)
    })
  };

  const info = {
    config: {},
    languageService,
    languageServiceHost: host,
    serverHost: {},
    project: {}
  };

  const plugin = initPlugin({ typescript: ts });
  const languageServiceWithPlugin = plugin.create(info);

  return {
    fileName,
    info,
    languageServiceWithPlugin
  };
}

test('expands macro-enabled files through script snapshots', (t) => {
  const expandStub = createExpandStub({
    code: 'class User { toString() { return "User"; } }',
    diagnostics: []
  });
  initPlugin.__setExpandSync?.(expandStub);
  t.after(() => initPlugin.__resetExpandSync?.());

  const source = `
    import { Derive } from "@macro/derive";
    @Derive(Debug)
    class User {}
  `;

  const env = createPluginEnvironment(source);
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, 'snapshot should exist');
  assert(snapshot.getText(0, snapshot.getLength()).includes('toString()'));
  assert.strictEqual(expandStub.calls.length, 1);
});

test('skips expansion for files without macros', (t) => {
  const expandStub = createExpandStub({
    code: 'class Plain {}',
    diagnostics: []
  });
  initPlugin.__setExpandSync?.(expandStub);
  t.after(() => initPlugin.__resetExpandSync?.());

  const env = createPluginEnvironment('class Plain {}');
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, 'snapshot should exist');
  assert(snapshot.getText(0, snapshot.getLength()).includes('class Plain'));
  assert.strictEqual(expandStub.calls.length, 0);
});

test('merges macro diagnostics with TypeScript diagnostics preserving locations', (t) => {
  const expandStub = createExpandStub({
    code: 'class Broken { toString() { return `${this.id}`; } }',
    diagnostics: [
      {
        level: 'error',
        message: 'Macro exploded',
        start: 12,
        end: 20
      }
    ]
  });
  initPlugin.__setExpandSync?.(expandStub);
  t.after(() => initPlugin.__resetExpandSync?.());

  const source = `
    import { Derive } from "@macro/derive";
    @Derive(Debug)
    class Broken {
      id: string;
    }
  `;

  const env = createPluginEnvironment(source);
  env.info.languageServiceHost.getScriptSnapshot(env.fileName);

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(env.fileName);
  assert.strictEqual(diagnostics.length, 2);
  const macroDiag = diagnostics.find((diag) => diag.source === 'ts-macros');
  assert(macroDiag, 'macro diagnostic missing');
  assert.strictEqual(macroDiag.code, 9999);
  assert.strictEqual(macroDiag.start, 12);
  assert.strictEqual(macroDiag.length, 8);
  assert.strictEqual(macroDiag.category, ts.DiagnosticCategory.Error);
});

test('falls back gracefully when expansion throws', (t) => {
  const expandStub = createExpandStub(new Error('native failure'));
  initPlugin.__setExpandSync?.(expandStub);
  t.after(() => initPlugin.__resetExpandSync?.());

  const env = createPluginEnvironment(`
    import { Derive } from "@macro/derive";
    @Derive(Debug)
    class User {}
  `);

  const snapshot = env.info.languageServiceHost.getScriptSnapshot(env.fileName);
  assert(snapshot, 'snapshot should exist');
  assert(snapshot.getText(0, snapshot.getLength()).includes('class User'));

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(env.fileName);
  assert.strictEqual(diagnostics.length, 1);
  assert.strictEqual(diagnostics[0].source, 'ts');
});
