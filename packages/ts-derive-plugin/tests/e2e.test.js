const test = require('node:test');
const assert = require('node:assert/strict');
const ts = require('typescript/lib/tsserverlibrary');
const initPlugin = require('../dist/index.js');
const fs = require('fs');
const path = require('path');

function createSnapshot(source) {
  return ts.ScriptSnapshot.fromString(source);
}

function createPluginEnvironment(files) {
  const snapshots = new Map();
  const versions = new Map();

  for (const [fileName, source] of Object.entries(files)) {
      snapshots.set(fileName, createSnapshot(source));
      versions.set(fileName, '1');
  }

  const host = {
    getScriptSnapshot: (name) => snapshots.get(name) ?? null,
    getScriptVersion: (name) => versions.get(name) ?? '0',
    getCurrentDirectory: () => process.cwd(),
    getCompilationSettings: () => ({ module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ESNext }),
    getDefaultLibFileName: (options) => '/virtual/lib.d.ts',
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
            if (moduleName === './macros') {
                return { resolvedFileName: '/virtual/macros.ts', isExternalLibraryImport: false, extension: '.ts' };
            }
            // Handle self-reference for augmentation
            if (moduleName === './macro-user') {
                 return { resolvedFileName: '/virtual/macro-user.ts', isExternalLibraryImport: false, extension: '.ts' };
            }
            // Fallback for basic node resolution simulation if needed, or return undefined to let TS try default (which might fail in virtual env)
            // For this test, we specifically need ./macro-user to resolve.
            return undefined;
        });
    },
  };

  const originalGetScriptSnapshot = host.getScriptSnapshot;
  host.getScriptSnapshot = (name) => originalGetScriptSnapshot(name);

  const languageService = ts.createLanguageService(host);

  const info = {
    config: {},
    languageService,
    languageServiceHost: host,
    serverHost: {},
    project: {
        getCompilerOptions: () => ({})
    }
  };

  const plugin = initPlugin({ typescript: ts });
  const languageServiceWithPlugin = plugin.create(info);

  return {
    info,
    languageServiceWithPlugin
  };
}

test('E2E: expands MacroUser and validates methods', (t) => {
  if (initPlugin.__resetExpandSync) {
      initPlugin.__resetExpandSync();
  }

  const mainFile = '/virtual/MacroUser.ts';
  const source = `
    import { Derive, Debug } from "./macros";
    
    @Derive(Debug)
    export class MacroUser {
      @Debug({ rename: "userId" })
      id: string;

      name: string;

      constructor(id: string, name: string) {
        this.id = id;
        this.name = name;
      }
    }

    const user = new MacroUser("123", "Test");
    const json = user.toString(); // Should be fine
    // user.nonExistent(); // This should fail
  `;

  const files = {
      [mainFile]: source,
      '/virtual/macros.ts': `
        export declare const Derive: (...args: any[]) => ClassDecorator;
        export declare const Debug: any;
      `,
      '/virtual/lib.d.ts': `
        interface Object {
            toString(): string;
            valueOf(): Object;
        }
        interface Function {}
        interface String {}
        interface Boolean {}
        interface Number {}
        interface RegExp {}
        interface Array<T> {
            push(...items: T[]): number;
            join(separator?: string): string;
        }
        declare var Object: { new(value?: any): Object; prototype: Object; };
      `
  };

  const env = createPluginEnvironment(files);
  
  // Trigger expansion
  const snapshot = env.info.languageServiceHost.getScriptSnapshot(mainFile);
  const text = snapshot.getText(0, snapshot.getLength());
  
  assert(text.includes('toString()'), 'Expanded code should contain toString()');

  // Check valid usage
  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(mainFile);
  if (diagnostics.length > 0) {
      console.log('Diagnostics found:', diagnostics.map(d => d.messageText));
  }
  assert.strictEqual(diagnostics.length, 0, 'Should have no semantic errors for valid usage');
});

test('E2E: reports error for non-existent methods', (t) => {
  if (initPlugin.__resetExpandSync) {
      initPlugin.__resetExpandSync();
  }

  const mainFile = '/virtual/MacroUserError.ts';
  const source = `
    import { Derive, Debug } from "./macros";
    
    @Derive(Debug)
    export class MacroUser {
      id: string;
      constructor(id: string) { this.id = id; }
    }

    const user = new MacroUser("123");
    user.invalidMethod();
  `;

  const files = {
      [mainFile]: source,
      '/virtual/macros.ts': `
        export declare const Derive: (...args: any[]) => ClassDecorator;
        export declare const Debug: any;
      `,
      '/virtual/lib.d.ts': `
        interface Object { toString(): string; }
        declare var Object: { new(value?: any): Object; prototype: Object; };
      `
  };

  const env = createPluginEnvironment(files);
  env.info.languageServiceHost.getScriptSnapshot(mainFile); // Trigger expansion

  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(mainFile);
  assert(diagnostics.length > 0, 'Should have diagnostics');
  assert(diagnostics.some(d => d.messageText.includes("Property 'invalidMethod' does not exist")), 'Should report missing property');
});

test('E2E: macro-user.ts example with Debug and JSON macros', (t) => {
  const fixturesDir = path.join(__dirname, 'fixtures');
  const macroUserSource = fs.readFileSync(path.join(fixturesDir, 'macro-user.test.ts'), 'utf-8');
  const expandedMacroUserCode = fs.readFileSync(path.join(fixturesDir, 'macro-user.expanded.test.ts'), 'utf-8');

  const mainFile = '/virtual/macro-user.ts';
  
  if (initPlugin.__setExpandSync && initPlugin.__resetExpandSync) {
    initPlugin.__setExpandSync((code, fileName) => {
      if (fileName === mainFile) {
        const types = `
          import { MacroUser } from './macro-user';
          declare module './macro-user' {
            interface MacroUser {
              toString(): string;
              toJSON(): {
                userId: string;
                name: string;
                role: string;
                favoriteMacro: "Derive" | "JsonNative";
                since: string;
              };
            }
          }
        `;
        
        return { 
            code: expandedMacroUserCode, 
            types: types,
            diagnostics: [] 
        };
      }
      return { code, diagnostics: [] };
    });
    t.after(() => initPlugin.__resetExpandSync && initPlugin.__resetExpandSync());
  }

  const files = {
      [mainFile]: macroUserSource,
      '/virtual/macros.ts': `
        export declare const Derive: (...args: any[]) => ClassDecorator;
        export declare const Debug: any;
        export declare const JSON: any; // Add JSON macro declaration
      `,
      '/virtual/effect.d.ts': `
        export declare namespace Schema {
            export function propertySignature(type: any): any;
            export function String: any;
            export function nonEmptyString(options?: { message?: () => string }): any;
            export function Union(...args: any[]): any;
            export function Array(type: any): any;
            export function Tuple(...args: any[]): any;
            export function OptionFromNullishOr(type: any, defaultValue: any): any;
            export function Boolean: any;
            export function DateTimeUtc: any;
            export class Class<A extends string, B> {
                constructor(name: A);
                (...args: any[]): B;
            }
        }
        export declare const Schema: Schema;
        // Stub all types from "../types/bindings" as any for now
        export declare const TaxRate: any;
        export declare const Site: any;
        export declare const Represents: any;
        export declare const Ordered: any;
        export declare const Did: any;
        export declare const AccountName: any;
        export declare const Sector: any;
        export declare const PhoneNumber: any;
        export declare const Email: any;
        export declare const Colors: any;
      `,
      '/virtual/lib.d.ts': `
        interface Object {
            toString(): string;
            valueOf(): Object;
        }
        interface Function {}
        interface String {}
        interface Boolean {}
        interface Number {}
        interface RegExp {}
        interface Array<T> {
            push(...items: T[]): number;
            join(separator?: string): string;
        }
        declare var Object: { new(value?: any): Object; prototype: Object; };
      `,
      '/virtual/macro-user.ts.ts-macros.d.ts': `
          import { MacroUser } from './macro-user';
          declare module '/virtual/macro-user' {
            interface MacroUser {
              toString(): string;
              toJSON(): {
                userId: string;
                name: string;
                role: string;
                favoriteMacro: "Derive" | "JsonNative";
                since: string;
              };
            }
          }
      `
  };

  const env = createPluginEnvironment(files);
  
  // Trigger expansion
  env.info.languageServiceHost.getScriptSnapshot(mainFile);
  
  // Check diagnostics
  const allDiagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(mainFile);
  
  // Filter out module resolution errors since we are in a virtual environment
  const diagnostics = allDiagnostics.filter(d => d.code !== 2307);
  
  if (diagnostics.length > 0) {
      console.log('All Diagnostics for macro-user.ts:', diagnostics.map(d => `${d.code}: ${d.messageText}`));
  }
  
  assert.strictEqual(diagnostics.length, 0, 'Should have no semantic errors for macro-user.ts example after type expansion');
});
