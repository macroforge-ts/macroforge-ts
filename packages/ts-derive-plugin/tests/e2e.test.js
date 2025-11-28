const test = require('node:test');
const assert = require('node:assert/strict');
const ts = require('typescript/lib/tsserverlibrary');
const initPlugin = require('../dist/index.js');

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
        interface Array<T> {}
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
  assert.strictEqual(diagnostics.length, 0, 'Should have no semantic errors for valid usage');
  
  // Verify that expansion didn't break the class structure (check constructor usage if we had checked diagnostics for 'user' creation)
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

// Function to escape backticks within a string
function escapeBackticks(str) {
  return str.replace(/`/g, '\`');
}

test('E2E: macro-user.ts example with Debug and JSON macros', (t) => {
  // Original content of @playground/svelte/src/lib/demo/macro-user.ts
  const macroUserSource = escapeBackticks(`import { Derive, Debug } from "@ts-macros/macros";
import { Schema } from "effect";
import {
  TaxRate,
  Site,
  Represents,
  Ordered,
  Did,
  AccountName,
  Sector,
  PhoneNumber,
  Email,
  Colors,
} from "../types/bindings";
import { JSON } from "@playground/macro";

@Derive(Debug, JSON)
export class MacroUser {
  @Debug({ rename: "userId" })
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  @Debug({ skip: true })
  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }
}

const showcaseUser = new MacroUser(
  "usr_2626",
  "Alya Vector",
  "Macro Explorer",
  "Derive",
  "2024-09-12",
  "svelte-secret-token",
);

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();

export class Account extends Schema.Class<Account>("Account")({
  id: Schema.propertySignature(Schema.String).annotations({
    missingMessage: () => \
`'Id' is required`,
  }),
  taxRate: Schema.propertySignature(
    Schema.Union(
      Schema.String.pipe(Schema.nonEmptyString()),
      TaxRate,
    ).annotations({
      message: () => ({
        message: \
`Please enter a valid value`,
        override: true,
      }),
    }),
  ).annotations({ missingMessage: () => \
`'Tax Rate' is required` }),
  site: Schema.propertySignature(
    Schema.Union(Schema.String.pipe(Schema.nonEmptyString()), Site).annotations(
      {
        message: () => ({
          message: \
`Please enter a valid value`,
          override: true,
        }),
      },
    ),
  ).annotations({ missingMessage: () => \
`'Site' is required` }),
  salesRep: Schema.OptionFromNullishOr(
    Schema.Array(Represents).annotations({
      identifier: \
`RepresentsRef`,
    }),
    null,
  ),
  orders: Schema.propertySignature(
    Schema.Array(
      Ordered.annotations({
        identifier: \
`OrderedRef`,
      }),
    ),
  ).annotations({ missingMessage: () => \
`'Orders' is required` }),
  activity: Schema.propertySignature(
    Schema.Array(
      Did.annotations({
        identifier: \
`DidRef`,
      }),
    ),
  ).annotations({ missingMessage: () => \
`'Activity' is required` }),
  customFields: Schema.propertySignature(
    Schema.Array(
      Schema.Tuple(
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
        ),
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
        ),
      ),
    ),
  ).annotations({ missingMessage: () => \
`'Custom Fields' is required` }),
  accountName: Schema.propertySignature(AccountName).annotations({
    missingMessage: () => \
`'Account Name' is required`,
  }),
  sector: Schema.propertySignature(Sector).annotations({ missingMessage: () => \
`'Sector' is required` }),
  memo: Schema.OptionFromNullishOr(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
    ),
    null,
  ),
  phones: Schema.propertySignature(Schema.Array(PhoneNumber)).annotations({
    missingMessage: () => \
`'Phones' is required`,
  }),
  email: Schema.propertySignature(Email).annotations({
    missingMessage: () => \
`'Email' is required`,
  }),
  leadSource: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => \
`'Lead Source' is required` }),
  colors: Schema.propertySignature(Colors).annotations({ missingMessage: () => \
`'Colors' is required` }),
  needsReview: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => \
`'Needs Review' is required`,
  }),
  hasAlert: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => \
`'Has Alert' is required`,
  }),
  accountType: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => \
`'Account Type' is required` }),
  subtype: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => \
`'Subtype' is required` }),
  isTaxExempt: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => \
`'Is Tax Exempt' is required`,
  }),
  paymentTerms: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => \
`'Payment Terms' is required` }),
  tags: Schema.propertySignature(
    Schema.Array(
      Schema.String.pipe(
        Schema.nonEmptyString({ message: () => \
`Please enter a value` }),
      ),
    ),
  ).annotations({ missingMessage: () => \
`'Tags' is required` }),
  dateAdded: Schema.propertySignature(Schema.DateTimeUtc).annotations({
    missingMessage: () => \
`'Date Added' is required`,
  }),
}) {}
`);

  // Expected expanded code (assuming prototype assignments for toString and toJSON)
  // This will NOT include the `Account` class for simplicity, focusing on MacroUser.
  const expandedMacroUserCode = escapeBackticks(`import { Derive, Debug } from "@ts-macros/macros";
import { Schema } from "effect";
import {
  TaxRate,
  Site,
  Represents,
  Ordered,
  Did,
  AccountName,
  Sector,
  PhoneNumber,
  Email,
  Colors,
} from "../types/bindings";
import { JSON } from "@playground/macro";

export class MacroUser {
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }
}

MacroUser.prototype.toString = function () {
    return 
`MacroUser { userId: ${this.id}, name: ${this.name} }`
;
};

MacroUser.prototype.toJSON = function () {
    return {
        userId: this.id,
        name: this.name,
        role: this.role,
        favoriteMacro: this.favoriteMacro,
        since: this.since,
    };
};

const showcaseUser = new MacroUser(
  "usr_2626",
  "Alya Vector",
  "Macro Explorer",
  "Derive",
  "2024-09-12",
  "svelte-secret-token",
);

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();

export class Account extends Schema.Class<Account>("Account")({
  id: Schema.propertySignature(Schema.String).annotations({
    missingMessage: () => 
`'Id' is required`,
  }),
  taxRate: Schema.propertySignature(
    Schema.Union(
      Schema.String.pipe(Schema.nonEmptyString()),
      TaxRate,
    ).annotations({
      message: () => ({
        message: 
`Please enter a valid value`,
        override: true,
      }),
    }),
  ).annotations({ missingMessage: () => 
`'Tax Rate' is required` }),
  site: Schema.propertySignature(
    Schema.Union(Schema.String.pipe(Schema.nonEmptyString()), Site).annotations(
      {
        message: () => ({
          message: 
`Please enter a valid value`,
          override: true,
        }),
      },
    ),
  ).annotations({ missingMessage: () => 
`'Site' is required` }),
  salesRep: Schema.OptionFromNullishOr(
    Schema.Array(Represents).annotations({
      identifier: 
`RepresentsRef`,
    }),
    null,
  ),
  orders: Schema.propertySignature(
    Schema.Array(
      Ordered.annotations({
        identifier: 
`OrderedRef`,
      }),
    ),
  ).annotations({ missingMessage: () => 
`'Orders' is required` }),
  activity: Schema.propertySignature(
    Schema.Array(
      Did.annotations({
        identifier: 
`DidRef`,
      }),
    ),
  ).annotations({ missingMessage: () => 
`'Activity' is required` }),
  customFields: Schema.propertySignature(
    Schema.Array(
      Schema.Tuple(
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
        ),
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
        ),
      ),
    ),
  ).annotations({ missingMessage: () => 
`'Custom Fields' is required` }),
  accountName: Schema.propertySignature(AccountName).annotations({
    missingMessage: () => 
`'Account Name' is required`,
  }),
  sector: Schema.propertySignature(Sector).annotations({ missingMessage: () => 
`'Sector' is required` }),
  memo: Schema.OptionFromNullishOr(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
    ),
    null,
  ),
  phones: Schema.propertySignature(Schema.Array(PhoneNumber)).annotations({
    missingMessage: () => 
`'Phones' is required`,
  }),
  email: Schema.propertySignature(Email).annotations({
    missingMessage: () => 
`'Email' is required`,
  }),
  leadSource: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => 
`'Lead Source' is required` }),
  colors: Schema.propertySignature(Colors).annotations({ missingMessage: () => 
`'Colors' is required` }),
  needsReview: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => 
`'Needs Review' is required`,
  }),
  hasAlert: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => 
`'Has Alert' is required`,
  }),
  accountType: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => 
`'Account Type' is required` }),
  subtype: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => 
`'Subtype' is required` }),
  isTaxExempt: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => 
`'Is Tax Exempt' is required`,
  }),
  paymentTerms: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => 
`'Payment Terms' is required` }),
  tags: Schema.propertySignature(
    Schema.Array(
      Schema.String.pipe(
        Schema.nonEmptyString({ message: () => 
`Please enter a value` }),
      ),
    ),
  ).annotations({ missingMessage: () => 
`'Tags' is required` }),
  dateAdded: Schema.propertySignature(Schema.DateTimeUtc).annotations({
    missingMessage: () => 
`'Date Added' is required`,
  }),
}) {}
`);

  const mainFile = '/virtual/macro-user.ts';
  
  if (initPlugin.__setExpandSync && initPlugin.__resetExpandSync) {
    initPlugin.__setExpandSync((code, fileName) => {
      if (fileName === mainFile) {
        return { code: expandedMacroUserCode, diagnostics: [] };
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
      // Stubbing the types from "../types/bindings"
      '/virtual/types/bindings.d.ts': `
        export type TaxRate = any;
        export type Site = any;
        export type Represents = any;
        export type Ordered = any;
        export type Did = any;
        export type AccountName = any;
        export type Sector = any;
        export type PhoneNumber = any;
        export type Email = any;
        export type Colors = any;
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
        interface Array<T> {}
        declare var Object: { new(value?: any): Object; prototype: Object; };
      `
  };

  const env = createPluginEnvironment(files);
  
  // Trigger expansion
  env.info.languageServiceHost.getScriptSnapshot(mainFile);
  
  // Check diagnostics
  const diagnostics = env.languageServiceWithPlugin.getSemanticDiagnostics(mainFile);
  
  if (diagnostics.length > 0) {
      console.log('All Diagnostics for macro-user.ts:', diagnostics.map(d => `${d.code}: ${d.messageText}`));
  }
  
  // Expect errors for toJSON and possibly toString due to prototype assignment
  // This validates the hypothesis that prototype assignments are not picked up by TS for type definitions.
  // The goal is to see TS2339 for toJSON and potentially toString.
  assert(diagnostics.length > 0, 'Expected diagnostics for prototype assignments not reflecting in type');
  assert(diagnostics.some(d => d.code === 2339 && d.messageText.includes("Property 'toJSON' does not exist")), 'Expected TS2339 for toJSON');
  
  // The error for toString might or might not appear depending on TS version/config
  // but toJSON definitely should.
});