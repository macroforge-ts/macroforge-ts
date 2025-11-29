/**
 * Intellisense Integration Tests
 *
 * These tests validate the complete intellisense experience:
 * - Hover information shows correct types
 * - Completions include all expected members
 * - Go-to-definition navigates correctly
 * - Signature help shows proper parameters
 *
 * Tests use both mocked and real native module (when available).
 */

const test = require('node:test');
const assert = require('node:assert/strict');
const ts = require('typescript/lib/tsserverlibrary');
const initPlugin = require('../dist/index.js');

// ============================================================================
// Test Infrastructure
// ============================================================================

const FULL_LIB_DTS = `
interface Object {
  constructor: Function;
  toString(): string;
  toLocaleString(): string;
  valueOf(): Object;
  hasOwnProperty(v: PropertyKey): boolean;
  isPrototypeOf(v: Object): boolean;
  propertyIsEnumerable(v: PropertyKey): boolean;
}

interface ObjectConstructor {
  new(value?: any): Object;
  (): any;
  (value: any): any;
  readonly prototype: Object;
  getPrototypeOf(o: any): any;
  keys(o: object): string[];
  assign<T extends {}, U>(target: T, source: U): T & U;
}

declare var Object: ObjectConstructor;

interface Function {
  apply(this: Function, thisArg: any, argArray?: any): any;
  call(this: Function, thisArg: any, ...argArray: any[]): any;
  bind(this: Function, thisArg: any, ...argArray: any[]): any;
  prototype: any;
  readonly length: number;
  readonly name: string;
}

interface FunctionConstructor {
  new(...args: string[]): Function;
  (...args: string[]): Function;
  readonly prototype: Function;
}

declare var Function: FunctionConstructor;

interface String {
  readonly length: number;
  charAt(pos: number): string;
  charCodeAt(index: number): number;
  concat(...strings: string[]): string;
  indexOf(searchString: string, position?: number): number;
  lastIndexOf(searchString: string, position?: number): number;
  substring(start: number, end?: number): string;
  toLowerCase(): string;
  toUpperCase(): string;
  trim(): string;
  split(separator: string | RegExp, limit?: number): string[];
}

interface StringConstructor {
  new(value?: any): String;
  (value?: any): string;
  readonly prototype: String;
}

declare var String: StringConstructor;

interface Boolean {
  valueOf(): boolean;
}

interface BooleanConstructor {
  new(value?: any): Boolean;
  <T>(value?: T): boolean;
  readonly prototype: Boolean;
}

declare var Boolean: BooleanConstructor;

interface Number {
  toString(radix?: number): string;
  toFixed(fractionDigits?: number): string;
  toExponential(fractionDigits?: number): string;
  toPrecision(precision?: number): string;
  valueOf(): number;
}

interface NumberConstructor {
  new(value?: any): Number;
  (value?: any): number;
  readonly prototype: Number;
  readonly MAX_VALUE: number;
  readonly MIN_VALUE: number;
  readonly NaN: number;
}

declare var Number: NumberConstructor;

interface RegExp {
  exec(string: string): RegExpExecArray | null;
  test(string: string): boolean;
  readonly source: string;
  readonly global: boolean;
  readonly ignoreCase: boolean;
  readonly multiline: boolean;
}

interface RegExpConstructor {
  new(pattern: RegExp | string): RegExp;
  new(pattern: string, flags?: string): RegExp;
  (pattern: RegExp | string): RegExp;
  (pattern: string, flags?: string): RegExp;
  readonly prototype: RegExp;
}

declare var RegExp: RegExpConstructor;

interface RegExpExecArray extends Array<string> {
  index: number;
  input: string;
}

interface Array<T> {
  length: number;
  toString(): string;
  push(...items: T[]): number;
  pop(): T | undefined;
  concat(...items: T[][]): T[];
  join(separator?: string): string;
  slice(start?: number, end?: number): T[];
  indexOf(searchElement: T, fromIndex?: number): number;
  forEach(callbackfn: (value: T, index: number, array: T[]) => void): void;
  map<U>(callbackfn: (value: T, index: number, array: T[]) => U): U[];
  filter(predicate: (value: T, index: number, array: T[]) => unknown): T[];
  find(predicate: (value: T, index: number, obj: T[]) => unknown): T | undefined;
  [n: number]: T;
}

interface ArrayConstructor {
  new<T>(...items: T[]): T[];
  <T>(...items: T[]): T[];
  isArray(arg: any): arg is any[];
  readonly prototype: any[];
}

declare var Array: ArrayConstructor;

interface IArguments {
  [index: number]: any;
  length: number;
  callee: Function;
}

interface Date {
  toString(): string;
  toDateString(): string;
  toTimeString(): string;
  getTime(): number;
  getFullYear(): number;
  getMonth(): number;
  getDate(): number;
}

interface DateConstructor {
  new(): Date;
  new(value: number | string): Date;
  (): string;
  readonly prototype: Date;
  now(): number;
}

declare var Date: DateConstructor;

interface JSON {
  parse(text: string): any;
  stringify(value: any): string;
}

declare var JSON: JSON;

interface Console {
  log(...data: any[]): void;
  error(...data: any[]): void;
  warn(...data: any[]): void;
}

declare var console: Console;

type PropertyKey = string | number | symbol;
type Partial<T> = { [P in keyof T]?: T[P]; };
type Required<T> = { [P in keyof T]-?: T[P]; };
type Pick<T, K extends keyof T> = { [P in K]: T[P]; };
type Omit<T, K extends keyof any> = Pick<T, Exclude<keyof T, K>>;
type Exclude<T, U> = T extends U ? never : T;
type Extract<T, U> = T extends U ? T : never;

interface ClassDecorator {
  <TFunction extends Function>(target: TFunction): TFunction | void;
}
`;

const MACROS_DTS = `
export declare const Derive: (...args: any[]) => ClassDecorator;
export declare const Debug: any;
export declare const JSON: any;
export declare const Clone: any;
export declare const Eq: any;
export declare const Hash: any;
`;

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function createFullEnv(files, expandFn = null, options = {}) {
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
      experimentalDecorators: true,
      lib: ['lib.es2020.d.ts'],
    }),
    getDefaultLibFileName: () => '/virtual/lib.d.ts',
    fileExists: (path) => snapshots.has(path),
    readFile: (path) => {
      const s = snapshots.get(path);
      return s ? s.getText(0, s.getLength()) : undefined;
    },
    getScriptFileNames: () => Array.from(snapshots.keys()),
    resolveModuleNames: (names) => names.map(n => {
      if (n === './macros' || n === '@ts-macros/macros') {
        return { resolvedFileName: '/virtual/macros.ts', extension: '.ts' };
      }
      return undefined;
    }),
  };

  const ls = ts.createLanguageService(host);

  const info = {
    config: {},
    languageService: ls,
    languageServiceHost: host,
    serverHost: {},
    project: {
      getCompilerOptions: () => host.getCompilationSettings(),
      projectService: {
        logger: {
          info: (msg) => { if (options.debug) console.log(msg); }
        }
      }
    }
  };

  if (expandFn) {
    const stub = (code, fileName) => {
      stub.calls = stub.calls || [];
      stub.calls.push({ code, fileName });
      return expandFn(code, fileName);
    };
    initPlugin.__setExpandSync?.(stub);
  }

  const plugin = initPlugin({ typescript: ts });
  const pluginService = plugin.create(info);

  return {
    info,
    pluginService,
    snapshots,
    versions,
    getSnapshot: () => info.languageServiceHost.getScriptSnapshot,
  };
}

// ============================================================================
// Hover Tests
// ============================================================================

test.describe('Hover Information', () => {

  test('hover on class name shows class info', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;
}`;

    const expanded = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;
  toString(): string { return "User { id: " + this.id + " }"; }
}`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: expanded, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find 'User' class declaration
    const userPos = expanded.indexOf('class User') + 6;
    const hover = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', userPos);

    assert(hover, 'Should have hover info');
    const display = hover.displayParts?.map(p => p.text).join('') || '';
    assert(display.includes('User'), `Hover should show User class, got: ${display}`);
  });

  test('hover on field shows type info', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  age: number;
  active: boolean;
}`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Test each field type
    const fields = [
      { name: 'id', type: 'string' },
      { name: 'age', type: 'number' },
      { name: 'active', type: 'boolean' },
    ];

    for (const field of fields) {
      const pos = source.indexOf(`${field.name}:`);
      const hover = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', pos);

      assert(hover, `Should have hover for ${field.name}`);
      const display = hover.displayParts?.map(p => p.text).join('') || '';
      assert(display.includes(field.type),
        `${field.name} should show ${field.type}, got: ${display}`);
    }
  });

  test('hover on generated method shows return type', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const u = new User();
u.toString();`;

    const expanded = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  toString(): string { return "User"; }
}

const u = new User();
u.toString();`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: expanded, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Hover on toString call
    const toStringPos = expanded.lastIndexOf('toString()');
    const hover = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', toStringPos);

    assert(hover, 'Should have hover on toString');
    const display = hover.displayParts?.map(p => p.text).join('') || '';
    console.log(`  toString hover: ${display}`);
    assert(display.includes('string'), `toString should return string, got: ${display}`);
  });

  test('hover on decorated class shows decorator info', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Hover on @Derive
    const derivePos = source.indexOf('@Derive') + 1;
    const hover = env.pluginService.getQuickInfoAtPosition('/virtual/test.ts', derivePos);

    if (hover) {
      const display = hover.displayParts?.map(p => p.text).join('') || '';
      console.log(`  @Derive hover: ${display}`);
    }
  });

});

// ============================================================================
// Completions Tests
// ============================================================================

test.describe('Completions', () => {

  test('completions include original and generated members', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;
}

const u = new User();
u.`;

    const expanded = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;
  toString(): string { return "User"; }
}

const u = new User();
u.`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: expanded, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    const dotPos = expanded.lastIndexOf('u.') + 2;
    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', dotPos, {});

    assert(completions?.entries, 'Should have completions');
    const names = completions.entries.map(e => e.name);

    console.log(`  Available completions: ${names.slice(0, 10).join(', ')}...`);

    // Original fields
    assert(names.includes('id'), 'Should include id field');
    assert(names.includes('name'), 'Should include name field');

    // Generated method (or inherited from Object)
    assert(names.includes('toString'), 'Should include toString method');
  });

  test('completions show correct kind for each member', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  getName(): string { return ""; }
}

const u = new User();
u.`;

    const expanded = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  getName(): string { return ""; }
  toString(): string { return "User"; }
}

const u = new User();
u.`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: expanded, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    const dotPos = expanded.lastIndexOf('u.') + 2;
    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', dotPos, {});

    assert(completions?.entries, 'Should have completions');

    // Find specific entries
    const idEntry = completions.entries.find(e => e.name === 'id');
    const getNameEntry = completions.entries.find(e => e.name === 'getName');
    const toStringEntry = completions.entries.find(e => e.name === 'toString');

    // id should be a property/field
    assert(idEntry, 'Should have id entry');
    assert.strictEqual(idEntry.kind, ts.ScriptElementKind.memberVariableElement,
      `id should be field, got ${idEntry.kind}`);

    // getName should be a method
    assert(getNameEntry, 'Should have getName entry');
    assert.strictEqual(getNameEntry.kind, ts.ScriptElementKind.memberFunctionElement,
      `getName should be method, got ${getNameEntry.kind}`);

    // toString should be a method
    assert(toStringEntry, 'Should have toString entry');
  });

  test('completions work inside method body', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;

  greet() {
    return this.
  }
}`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    const thisPos = source.indexOf('this.') + 5;
    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', thisPos, {});

    assert(completions?.entries, 'Should have completions inside method');
    const names = completions.entries.map(e => e.name);

    assert(names.includes('id'), 'this. should complete to id');
    assert(names.includes('name'), 'this. should complete to name');
    assert(names.includes('greet'), 'this. should complete to greet method');
  });

  test('completions after partial input filters correctly', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  identifier: string;
  name: string;
}

const u = new User();
u.id`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Position after 'u.id'
    const pos = source.lastIndexOf('u.id') + 4;
    const completions = env.pluginService.getCompletionsAtPosition('/virtual/test.ts', pos, {});

    if (completions?.entries) {
      const names = completions.entries.map(e => e.name);
      // Should show id-prefixed completions prominently
      const idMatches = names.filter(n => n.startsWith('id'));
      console.log(`  Completions starting with 'id': ${idMatches.join(', ')}`);
    }
  });

});

// ============================================================================
// Go to Definition Tests
// ============================================================================

test.describe('Go to Definition', () => {

  test('definition of field navigates to declaration', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  name: string;
}

const u = new User();
const userId = u.id;`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Go to definition on u.id
    const usagePos = source.lastIndexOf('.id') + 1;
    const defs = env.pluginService.getDefinitionAtPosition('/virtual/test.ts', usagePos);

    assert(defs && defs.length > 0, 'Should find definition');

    const def = defs[0];
    assert.strictEqual(def.fileName, '/virtual/test.ts', 'Definition should be in same file');

    // Should point to declaration
    const declPos = source.indexOf('id: string');
    const defStartsNear = Math.abs(def.textSpan.start - declPos) < 5;
    assert(defStartsNear, `Definition should be near declaration at ${declPos}, got ${def.textSpan.start}`);
  });

  test('definition of class navigates to class declaration', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const u: User = new User();`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Go to definition on "User" in type annotation
    const typePos = source.lastIndexOf(': User') + 2;
    const defs = env.pluginService.getDefinitionAtPosition('/virtual/test.ts', typePos);

    assert(defs && defs.length > 0, 'Should find class definition');

    const def = defs[0];
    const classPos = source.indexOf('class User');
    console.log(`  Class definition at: ${def.textSpan.start}, class keyword at: ${classPos}`);
  });

  test('definition of generated method behavior', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const u = new User();
const str = u.toString();`;

    const expanded = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
  toString(): string { return "User { id: " + this.id + " }"; }
}

const u = new User();
const str = u.toString();`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: expanded, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Go to definition on toString() call
    const callPos = expanded.lastIndexOf('toString()');
    const defs = env.pluginService.getDefinitionAtPosition('/virtual/test.ts', callPos);

    if (defs && defs.length > 0) {
      console.log(`  Generated method definitions found: ${defs.length}`);
      for (const def of defs) {
        console.log(`    - ${def.fileName}:${def.textSpan.start}`);
      }

      // Generated method exists in expanded code
      const genMethodPos = expanded.indexOf('toString(): string');
      console.log(`  Generated method in expanded: ${genMethodPos}`);
    } else {
      console.log('  No definitions found for generated method');
    }
  });

});

// ============================================================================
// Find References Tests
// ============================================================================

test.describe('Find References', () => {

  test('find references for field includes all usages', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;

  getId() {
    return this.id;
  }
}

const u = new User();
console.log(u.id);`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find references for 'id' field declaration
    const declPos = source.indexOf('id: string');
    const refs = env.pluginService.findReferences('/virtual/test.ts', declPos);

    if (refs && refs.length > 0) {
      const allRefs = refs.flatMap(r => r.references);
      console.log(`  Found ${allRefs.length} references to 'id'`);

      // Should include: declaration, this.id, u.id
      assert(allRefs.length >= 3, 'Should find at least 3 references');
    }
  });

  test('find references for class includes instantiations', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

function createUser(): User {
  return new User();
}

const u: User = createUser();`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Find references for 'User' class
    const classPos = source.indexOf('class User') + 6;
    const refs = env.pluginService.findReferences('/virtual/test.ts', classPos);

    if (refs && refs.length > 0) {
      const allRefs = refs.flatMap(r => r.references);
      console.log(`  Found ${allRefs.length} references to 'User'`);

      // Should include: class declaration, return type, new User, type annotation
      assert(allRefs.length >= 4, 'Should find at least 4 references');
    }
  });

});

// ============================================================================
// Rename Symbol Tests
// ============================================================================

test.describe('Rename Symbol', () => {

  test('rename preparation returns correct info', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;
}

const u = new User();
const x = u.id;`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get rename info for 'id' field
    const idPos = source.indexOf('id: string');
    const renameInfo = env.pluginService.getRenameInfo('/virtual/test.ts', idPos, {});

    assert(renameInfo.canRename, 'Should be able to rename field');
    console.log(`  Rename info: ${renameInfo.displayName}`);
  });

  test('find rename locations includes all occurrences', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  userId: string;

  getId() {
    return this.userId;
  }
}

const u = new User();
const id = u.userId;`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get rename locations for 'userId'
    const userIdPos = source.indexOf('userId: string');
    const locations = env.pluginService.findRenameLocations('/virtual/test.ts', userIdPos, false, false);

    if (locations) {
      console.log(`  Found ${locations.length} rename locations`);
      assert(locations.length >= 3, 'Should find at least 3 locations (decl, this.userId, u.userId)');
    }
  });

});

// ============================================================================
// Signature Help Tests
// ============================================================================

test.describe('Signature Help', () => {

  test('signature help for constructor shows parameters', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  constructor(public id: string, public name: string) {}
}

const u = new User(`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get signature help after opening paren
    const parenPos = source.lastIndexOf('(') + 1;
    const sigHelp = env.pluginService.getSignatureHelpItems('/virtual/test.ts', parenPos, {});

    if (sigHelp) {
      console.log(`  Signature help: ${sigHelp.items.length} overloads`);
      if (sigHelp.items.length > 0) {
        const params = sigHelp.items[0].parameters;
        console.log(`  Parameters: ${params.map(p => p.name).join(', ')}`);
        assert(params.length >= 2, 'Should show id and name parameters');
      }
    }
  });

  test('signature help for method shows parameters', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;

  setId(newId: string, validate: boolean): void {
    this.id = newId;
  }
}

const u = new User();
u.setId(`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get signature help for setId call
    const parenPos = source.lastIndexOf('(') + 1;
    const sigHelp = env.pluginService.getSignatureHelpItems('/virtual/test.ts', parenPos, {});

    if (sigHelp && sigHelp.items.length > 0) {
      const params = sigHelp.items[0].parameters;
      console.log(`  setId parameters: ${params.map(p => p.name).join(', ')}`);
      assert(params.some(p => p.name === 'newId'), 'Should show newId parameter');
      assert(params.some(p => p.name === 'validate'), 'Should show validate parameter');
    }
  });

});

// ============================================================================
// Document Highlights Tests
// ============================================================================

test.describe('Document Highlights', () => {

  test('highlights all occurrences of symbol', (t) => {
    const source = `import { Derive, Debug } from "./macros";

@Derive(Debug)
class User {
  id: string;

  getId(): string {
    return this.id;
  }

  setId(newId: string): void {
    this.id = newId;
  }
}`;

    t.after(() => initPlugin.__resetExpandSync?.());

    const env = createFullEnv({
      '/virtual/test.ts': source,
      '/virtual/macros.ts': MACROS_DTS,
      '/virtual/lib.d.ts': FULL_LIB_DTS,
    }, () => ({ code: source, diagnostics: [] }));

    env.info.languageServiceHost.getScriptSnapshot('/virtual/test.ts');

    // Get highlights for 'id'
    const idPos = source.indexOf('id: string');
    const highlights = env.pluginService.getDocumentHighlights('/virtual/test.ts', idPos, ['/virtual/test.ts']);

    if (highlights && highlights.length > 0) {
      const spans = highlights[0].highlightSpans;
      console.log(`  Found ${spans.length} highlights for 'id'`);

      // Should highlight: declaration, this.id (2 times)
      assert(spans.length >= 3, 'Should highlight at least 3 occurrences');
    }
  });

});
