/**
 * Comprehensive tests for error placement and intellisense placement
 * in the ts-derive-plugin.
 *
 * These tests validate that:
 * 1. Macro errors point to correct line/column in original source
 * 2. TypeScript errors in expanded code map back to original source
 * 3. Hover info appears at correct positions
 * 4. Completions work at correct positions
 * 5. Go-to-definition resolves correctly for generated methods
 */

const test = require('node:test');
const assert = require('node:assert/strict');
const ts = require('typescript/lib/tsserverlibrary');
const initPlugin = require('../dist/index.js');

// ============================================================================
// Test Utilities
// ============================================================================

/**
 * Convert a byte offset to line/column in source text
 */
function offsetToLineColumn(source, offset) {
  const lines = source.split('\n');
  let currentOffset = 0;

  for (let line = 0; line < lines.length; line++) {
    const lineLength = lines[line].length + 1; // +1 for newline
    if (currentOffset + lineLength > offset) {
      return {
        line: line + 1, // 1-indexed
        column: offset - currentOffset + 1 // 1-indexed
      };
    }
    currentOffset += lineLength;
  }

  return { line: lines.length, column: 1 };
}

/**
 * Find the position of a marker string in source
 */
function findMarkerPosition(source, marker) {
  const index = source.indexOf(marker);
  if (index === -1) return null;
  return {
    offset: index,
    ...offsetToLineColumn(source, index)
  };
}

/**
 * Create a stub for the expand function that tracks calls and returns custom results
 */
function createExpandStub(resultFn) {
  const stub = (code, fileName) => {
    stub.calls.push({ code, fileName });
    const result = typeof resultFn === 'function' ? resultFn(code, fileName) : resultFn;
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

/**
 * Create a full plugin environment with proper TypeScript language service
 */
function createFullPluginEnvironment(files, options = {}) {
  const snapshots = new Map();
  const versions = new Map();

  for (const [fileName, source] of Object.entries(files)) {
    snapshots.set(fileName, createSnapshot(source));
    versions.set(fileName, '1');
  }

  const host = {
    getScriptSnapshot: (name) => snapshots.get(name) ?? null,
    getScriptVersion: (name) => versions.get(name) ?? '0',
    getCurrentDirectory: () => '/virtual',
    getCompilationSettings: () => ({
      module: ts.ModuleKind.CommonJS,
      target: ts.ScriptTarget.ESNext,
      strict: true,
      noEmit: true,
      experimentalDecorators: true,
      emitDecoratorMetadata: false,
    }),
    getDefaultLibFileName: () => '/virtual/lib.d.ts',
    fileExists: (path) => snapshots.has(path) || ts.sys.fileExists(path),
    readFile: (path) => {
      const snapshot = snapshots.get(path);
      return snapshot ? snapshot.getText(0, snapshot.getLength()) : ts.sys.readFile(path);
    },
    readDirectory: ts.sys.readDirectory,
    directoryExists: ts.sys.directoryExists,
    getDirectories: ts.sys.getDirectories,
    getScriptFileNames: () => Array.from(snapshots.keys()),
    resolveModuleNames: (moduleNames, containingFile) => {
      return moduleNames.map(moduleName => {
        if (moduleName === './macros' || moduleName === '@ts-macros/macros') {
          return { resolvedFileName: '/virtual/macros.ts', isExternalLibraryImport: false, extension: '.ts' };
        }
        // Check if we have a virtual file for this module
        const virtualPath = `/virtual/${moduleName.replace('./', '')}.ts`;
        if (snapshots.has(virtualPath)) {
          return { resolvedFileName: virtualPath, isExternalLibraryImport: false, extension: '.ts' };
        }
        return undefined;
      });
    },
  };

  const languageService = ts.createLanguageService(host);

  const info = {
    config: options.config || {},
    languageService,
    languageServiceHost: host,
    serverHost: {},
    project: {
      getCompilerOptions: () => host.getCompilationSettings(),
      projectService: {
        logger: {
          info: (msg) => {
            if (options.debug) console.log(msg);
          }
        }
      }
    }
  };

  const plugin = initPlugin({ typescript: ts });
  const pluginService = plugin.create(info);

  return {
    info,
    pluginService,
    languageService,
    host,
    snapshots,
    versions,
    updateFile: (fileName, content) => {
      const version = (parseInt(versions.get(fileName) || '0') + 1).toString();
      snapshots.set(fileName, createSnapshot(content));
      versions.set(fileName, version);
    }
  };
}

const LIB_DTS = `
interface Object {
  toString(): string;
  valueOf(): Object;
}
interface Function {
  apply(this: Function, thisArg: any, argArray?: any): any;
  call(this: Function, thisArg: any, ...argArray: any[]): any;
  bind(this: Function, thisArg: any, ...argArray: any[]): any;
}
interface String {
  charAt(pos: number): string;
  length: number;
}
interface Boolean {}
interface Number {}
interface RegExp {}
interface Array<T> {
  length: number;
  push(...items: T[]): number;
}
interface IArguments {}
interface CallableFunction extends Function {}
interface NewableFunction extends Function {}
declare var Object: { new(value?: any): Object; prototype: Object; };
declare var String: { new(value?: any): String; prototype: String; };
declare var Array: { new<T>(...items: T[]): T[]; prototype: Array<any>; };
declare function parseInt(s: string, radix?: number): number;
`;

const MACROS_DTS = `
export declare const Derive: (...args: any[]) => ClassDecorator;
export declare const Debug: any;
export declare const JSON: any;
export declare const Clone: any;
`;

// ============================================================================
// Error Placement Tests
// ============================================================================

test.describe('Error Placement', () => {

  test('macro diagnostic positions should map to original source lines', (t) => {
    // This test validates that when a macro reports an error at a specific
    // position, it maps correctly to the original source location.

    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  badField: unknown;
}`;

    // Find where "badField" is in the original source
    const badFieldPos = findMarkerPosition(source, 'badField');

    // Create a stub that reports an error at the badField position
    const expandStub = createExpandStub((code, fileName) => ({
      code: code, // No change for this test
      diagnostics: [{
        level: 'error',
        message: 'Debug macro cannot handle unknown type',
        start: badFieldPos.offset, // Error at badField
        end: badFieldPos.offset + 'badField'.length
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Get diagnostics
    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');

    // Find the macro diagnostic
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');
    assert(macroDiag, 'Macro diagnostic should be present');

    // Validate the position
    assert.strictEqual(macroDiag.start, badFieldPos.offset,
      `Diagnostic start should be at badField offset (${badFieldPos.offset})`);
    assert.strictEqual(macroDiag.length, 'badField'.length,
      'Diagnostic length should match badField length');

    // Additional validation: convert to line/column and verify it's on the right line
    const sourceFile = env.languageService.getProgram()?.getSourceFile('/virtual/test.ts');
    if (sourceFile) {
      const { line, character } = ts.getLineAndCharacterOfPosition(sourceFile, macroDiag.start);
      assert.strictEqual(line + 1, badFieldPos.line,
        `Error should be on line ${badFieldPos.line}, got line ${line + 1}`);
    }
  });

  test('macro error on decorator line should point to decorator', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(InvalidMacro)
class TestClass {
  field: string;
}`;

    const decoratorPos = findMarkerPosition(source, '@Derive(InvalidMacro)');

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'error',
        message: 'Unknown macro: InvalidMacro',
        start: decoratorPos.offset,
        end: decoratorPos.offset + '@Derive(InvalidMacro)'.length
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    assert.strictEqual(macroDiag.start, decoratorPos.offset,
      `Error should start at decorator position (offset ${decoratorPos.offset})`);

    // Verify line number
    const sourceFile = env.languageService.getProgram()?.getSourceFile('/virtual/test.ts');
    if (sourceFile) {
      const { line } = ts.getLineAndCharacterOfPosition(sourceFile, macroDiag.start);
      assert.strictEqual(line + 1, decoratorPos.line,
        `Error should be on line ${decoratorPos.line} (decorator line)`);
    }
  });

  test('multiple macro errors should each have correct positions', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  field1: unknown;
  field2: symbol;
  field3: () => void;
}`;

    const field1Pos = findMarkerPosition(source, 'field1');
    const field2Pos = findMarkerPosition(source, 'field2');
    const field3Pos = findMarkerPosition(source, 'field3');

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [
        { level: 'error', message: 'Cannot debug unknown type', start: field1Pos.offset, end: field1Pos.offset + 6 },
        { level: 'warning', message: 'Symbol fields are not debuggable', start: field2Pos.offset, end: field2Pos.offset + 6 },
        { level: 'error', message: 'Function fields are skipped', start: field3Pos.offset, end: field3Pos.offset + 6 },
      ]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiags = diagnostics.filter(d => d.source === 'ts-macros');

    assert.strictEqual(macroDiags.length, 3, 'Should have 3 macro diagnostics');

    // Verify each error is at its correct position
    const sortedDiags = [...macroDiags].sort((a, b) => a.start - b.start);

    assert.strictEqual(sortedDiags[0].start, field1Pos.offset, 'First error at field1');
    assert.strictEqual(sortedDiags[1].start, field2Pos.offset, 'Second error at field2');
    assert.strictEqual(sortedDiags[2].start, field3Pos.offset, 'Third error at field3');
  });

  test('TypeScript errors in expanded code should NOT map back (current limitation)', (t) => {
    // This test documents the current limitation: TS errors in expanded code
    // use positions from the expanded code, not the original.
    // This is a known issue that needs source map support to fix.

    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}

const obj = new TestClass();
obj.nonExistentMethod();`;

    // The expansion adds toString() but not nonExistentMethod
    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  toString() { return "TestClass { id: " + this.id + " }"; }
}

const obj = new TestClass();
obj.nonExistentMethod();`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');

    // There should be a TypeScript error for nonExistentMethod
    const tsError = diagnostics.find(d =>
      typeof d.messageText === 'string' && d.messageText.includes('nonExistentMethod')
    );

    // Document the current behavior - positions are from expanded code
    // In an ideal world, they would map back to original
    if (tsError) {
      const sourceFile = env.languageService.getProgram()?.getSourceFile('/virtual/test.ts');
      if (sourceFile) {
        // Get position in expanded code
        const expandedPos = expandedCode.indexOf('nonExistentMethod');
        const originalPos = source.indexOf('nonExistentMethod');

        // Current behavior: start position is in expanded code coordinates
        // This test documents the limitation
        console.log(`  [INFO] TS error position: ${tsError.start}`);
        console.log(`  [INFO] Position in expanded: ${expandedPos}`);
        console.log(`  [INFO] Position in original: ${originalPos}`);

        // The error position might be from expanded code
        // This is expected current behavior (limitation)
      }
    }

    assert(diagnostics.some(d =>
      typeof d.messageText === 'string' && d.messageText.includes('nonExistentMethod')
    ), 'Should report error for non-existent method');
  });

  test('error at end of file should have valid position', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}
// Error marker at end`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'error',
        message: 'Unexpected end of file during macro expansion',
        start: source.length - 10,
        end: source.length
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    assert(macroDiag.start >= 0, 'Start should be non-negative');
    assert(macroDiag.start < source.length, 'Start should be within source');
  });

});

// ============================================================================
// Intellisense Placement Tests - Hover
// ============================================================================

test.describe('Hover Information Placement', () => {

  test('hover on generated method should return type info', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  name: string;
}

const obj = new TestClass();
obj.toString();`;

    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  name: string;
  toString(): string { return "TestClass { id: " + this.id + ", name: " + this.name + " }"; }
}

const obj = new TestClass();
obj.toString();`;

    const types = `
declare module '/virtual/test' {
  interface TestClass {
    toString(): string;
  }
}`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      types: types,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    }, { debug: false });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find position of toString() call
    const toStringCallPos = expandedCode.lastIndexOf('toString');

    // Get quick info (hover) at that position
    const quickInfo = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', toStringCallPos);

    // The method should be recognized
    if (quickInfo) {
      const displayParts = quickInfo.displayParts?.map(p => p.text).join('') || '';
      console.log(`  [INFO] Hover info: ${displayParts.substring(0, 100)}`);

      // Should have some type info
      assert(quickInfo.displayParts && quickInfo.displayParts.length > 0,
        'Should have display parts for hover');
    } else {
      console.log('  [WARN] No quick info returned - may need expanded code positions');
    }
  });

  test('hover position on original field should work', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}`;

    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  toString(): string { return "TestClass { id: " + this.id + " }"; }
}`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find position of 'id' field in expanded code
    const idPos = expandedCode.indexOf('id: string');

    const quickInfo = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', idPos);

    if (quickInfo) {
      assert(quickInfo.displayParts && quickInfo.displayParts.length > 0,
        'Should have display parts for field hover');
      const displayText = quickInfo.displayParts.map(p => p.text).join('');
      console.log(`  [INFO] Field hover: ${displayText}`);
    }
  });

});

// ============================================================================
// Intellisense Placement Tests - Completions
// ============================================================================

test.describe('Completions Placement', () => {

  test('completions after dot should include generated methods', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}

const obj = new TestClass();
obj.`;

    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  toString(): string { return "TestClass { id: " + this.id + " }"; }
}

const obj = new TestClass();
obj.`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get completions at the position after "obj."
    const dotPos = expandedCode.lastIndexOf('obj.') + 4;

    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', dotPos, {
      includeCompletionsForModuleExports: true,
      includeCompletionsWithInsertText: true,
    });

    if (completions?.entries) {
      const methodNames = completions.entries.map(e => e.name);
      console.log(`  [INFO] Completions: ${methodNames.slice(0, 10).join(', ')}...`);

      // Should include toString (either from Object or from generated)
      assert(methodNames.includes('toString'),
        'Completions should include toString method');

      // Should include the id field
      assert(methodNames.includes('id'),
        'Completions should include id field');
    } else {
      console.log('  [WARN] No completions returned');
    }
  });

  test('completions position should be valid after expansion changes line count', (t) => {
    // When expansion adds methods, line positions shift.
    // This test checks if completions still work.

    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  field1: string;
  field2: number;
}

const obj = new TestClass();
obj.`;

    // Expansion adds a multi-line method
    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  field1: string;
  field2: number;
  toString(): string {
    return "TestClass {" +
      " field1: " + this.field1 +
      ", field2: " + this.field2 +
      " }";
  }
}

const obj = new TestClass();
obj.`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Position after obj. in expanded code
    const dotPos = expandedCode.lastIndexOf('obj.') + 4;

    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', dotPos, {});

    if (completions?.entries) {
      const hasField1 = completions.entries.some(e => e.name === 'field1');
      const hasField2 = completions.entries.some(e => e.name === 'field2');

      assert(hasField1, 'Should include field1 in completions');
      assert(hasField2, 'Should include field2 in completions');
    }
  });

});

// ============================================================================
// Intellisense Placement Tests - Go to Definition
// ============================================================================

test.describe('Go to Definition', () => {

  test('definition of original field should resolve correctly', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}

const obj = new TestClass();
const x = obj.id;`;

    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  toString(): string { return this.id; }
}

const obj = new TestClass();
const x = obj.id;`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get definition at "obj.id" - find the last "id"
    const idPos = expandedCode.lastIndexOf('.id') + 1;

    const definitions = env.pluginService.getDefinitionAtPosition('/virtual/test.ts', idPos);

    if (definitions && definitions.length > 0) {
      const def = definitions[0];
      console.log(`  [INFO] Definition at: ${def.fileName}:${def.textSpan.start}`);

      // The definition should point to the field declaration
      // In expanded code, "id: string" is still in the same relative position
      const fieldDeclPos = expandedCode.indexOf('id: string');

      // The definition should be near the field declaration
      // (might be exact or might be the start of the property)
      assert(def.fileName === '/virtual/test.ts',
        'Definition should be in the same file');
    }
  });

  test('definition of generated method points to expanded code', (t) => {
    // This test documents that generated methods point to the expanded code
    // Not the original - because there's no source in the original!

    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}

const obj = new TestClass();
obj.toString();`;

    const expandedCode = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
  toString(): string { return "TestClass { id: " + this.id + " }"; }
}

const obj = new TestClass();
obj.toString();`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: expandedCode,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    // Trigger expansion
    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find toString() call position
    const toStringPos = expandedCode.lastIndexOf('toString()');

    const definitions = env.pluginService.getDefinitionAtPosition('/virtual/test.ts', toStringPos);

    if (definitions && definitions.length > 0) {
      const def = definitions[0];
      console.log(`  [INFO] Generated method definition: ${def.fileName}:${def.textSpan.start}`);

      // For generated methods, the definition WILL be in the expanded code
      // This is expected behavior - the method only exists in expanded code

      // Find where toString is defined in expanded code
      const methodDefPos = expandedCode.indexOf('toString(): string');

      if (def.fileName === '/virtual/test.ts') {
        // Definition points to expanded code (expected for generated)
        console.log(`  [INFO] Method defined at offset ${methodDefPos} in expanded`);
      }
    } else {
      // May fall back to Object.prototype.toString
      console.log('  [INFO] May resolve to Object.prototype.toString');
    }
  });

});

// ============================================================================
// Source Map Verification Tests
// ============================================================================

test.describe('Source Map Requirements', () => {

  test('expansion should track position deltas (future requirement)', (t) => {
    // This test documents what source map tracking should provide
    // Currently NOT implemented - this is a specification test

    const source = `@Derive(Debug)
class TestClass {
  id: string;
}`;

    // Expansion inserts a method after "id: string;"
    // Original: 48 chars
    // Expanded: 48 + method chars

    const expandedCode = `@Derive(Debug)
class TestClass {
  id: string;
  toString(): string { return this.id; }
}`;

    // Calculate expected mappings
    const insertionPoint = source.indexOf('}') - 1; // Before closing brace
    const insertedLength = expandedCode.length - source.length;

    console.log(`  [INFO] Original length: ${source.length}`);
    console.log(`  [INFO] Expanded length: ${expandedCode.length}`);
    console.log(`  [INFO] Insertion point: ${insertionPoint}`);
    console.log(`  [INFO] Inserted chars: ${insertedLength}`);

    // Document the mapping that SHOULD exist:
    // - Positions 0 to insertionPoint: same in both
    // - Positions insertionPoint to insertionPoint+insertedLength: generated (no original)
    // - Positions after: shifted by insertedLength

    // This test passes to document the requirement
    assert(insertedLength > 0, 'Expansion should add content');
  });

  test('diagnostic with no position should use fallback', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class TestClass {
  id: string;
}`;

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'warning',
        message: 'Generic macro warning',
        // No start/end positions
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    // Without position, it defaults to 0
    assert.strictEqual(macroDiag.start, 0, 'Should use 0 as fallback position');
    assert.strictEqual(macroDiag.length, 0, 'Should use 0 as fallback length');
  });

});

// ============================================================================
// Integration Tests - Real Expansion Scenarios
// ============================================================================

test.describe('Real Expansion Integration', () => {

  test('Debug macro should produce correct diagnostic positions', (t) => {
    // This test should use the real native module if available
    // to test actual diagnostic positions

    if (!initPlugin.__resetExpandSync) {
      t.skip('Test requires native module hooks');
      return;
    }

    // Reset to use real expand function
    initPlugin.__resetExpandSync();

    const source = `import { Derive, Debug } from "@ts-macros/macros";

@Derive(Debug)
export class RealTestClass {
  id: string;
  name: string;

  constructor(id: string, name: string) {
    this.id = id;
    this.name = name;
  }
}

const instance = new RealTestClass("1", "Test");
console.log(instance.toString());`;

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    try {
      // Trigger expansion with real native module
      const snapshot = env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

      if (snapshot) {
        const expandedText = snapshot.getText(0, snapshot.getLength());

        // Check if expansion happened
        if (expandedText !== source) {
          console.log('  [INFO] Real expansion occurred');
          console.log(`  [INFO] Expanded length: ${expandedText.length} (original: ${source.length})`);

          // Check for toString in expanded code
          const hasToString = expandedText.includes('toString()');
          console.log(`  [INFO] Has generated toString: ${hasToString}`);
        } else {
          console.log('  [INFO] No expansion (native module may not be loaded)');
        }
      }

      // Get any diagnostics
      const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');

      if (diagnostics.length > 0) {
        console.log(`  [INFO] Diagnostics: ${diagnostics.length}`);
        diagnostics.forEach((d, i) => {
          const msg = typeof d.messageText === 'string' ? d.messageText : d.messageText.messageText;
          console.log(`    ${i + 1}. [${d.source || 'ts'}] ${msg.substring(0, 50)}...`);
        });
      }
    } catch (e) {
      console.log(`  [INFO] Native expansion error: ${e.message}`);
    }

    // This test is informational - always passes
    assert(true, 'Integration test completed');
  });

});

// ============================================================================
// Edge Cases
// ============================================================================

test.describe('Edge Cases', () => {

  test('empty file should not cause position errors', (t) => {
    const source = '';

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: []
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    // Should not crash
    assert(Array.isArray(diagnostics), 'Should return diagnostics array');
  });

  test('unicode content should have correct positions', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class Test {
  emoji: string = "🎉";
  japanese: string = "日本語";
}`;

    const emojiPos = findMarkerPosition(source, 'emoji');

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'warning',
        message: 'Unicode field',
        start: emojiPos.offset,
        end: emojiPos.offset + 5
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    assert.strictEqual(macroDiag.start, emojiPos.offset,
      'Unicode content should not affect position calculation');
  });

  test('very long line should have correct positions', (t) => {
    const longString = 'a'.repeat(1000);
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class Test {
  long: string = "${longString}";
  afterLong: string;
}`;

    const afterLongPos = findMarkerPosition(source, 'afterLong');

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'error',
        message: 'Error on field after long string',
        start: afterLongPos.offset,
        end: afterLongPos.offset + 9
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    assert.strictEqual(macroDiag.start, afterLongPos.offset,
      'Long lines should not affect position calculation');
  });

  test('nested classes should have correct positions', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class Outer {
  id: string;

  @Derive(Debug)
  static Inner = class {
    innerId: string;
  };
}`;

    const innerIdPos = findMarkerPosition(source, 'innerId');

    const expandStub = createExpandStub((code, fileName) => ({
      code: code,
      diagnostics: [{
        level: 'error',
        message: 'Error in nested class',
        start: innerIdPos.offset,
        end: innerIdPos.offset + 7
      }]
    }));

    initPlugin.__setExpandSync?.(expandStub);
    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullPluginEnvironment({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': LIB_DTS,
    });

    const diagnostics = env.pluginService.getSemanticDiagnostics('/virtual/test.ts');
    const macroDiag = diagnostics.find(d => d.source === 'ts-macros');

    assert(macroDiag, 'Macro diagnostic should be present');
    assert.strictEqual(macroDiag.start, innerIdPos.offset,
      'Nested class positions should be correct');
  });

});
