# expandSync()

macroforge v0.3.1

Expands macros in TypeScript code synchronously and returns the transformed output.

## Signature

TypeScript

```
function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult
```

## Parameters

| Parameter  | Type            | Description                          |
| ---------- | --------------- | ------------------------------------ |
| `code`     | `string`        | TypeScript source code to transform  |
| `filepath` | `string`        | File path (used for error reporting) |
| `options`  | `ExpandOptions` | Optional configuration               |

## ExpandOptions

TypeScript

```
interface ExpandOptions {
  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;

  // External decorator module packages to load
  externalDecoratorModules?: Array<string>;

  // Path to a previously loaded config file
  configPath?: string;

  // JSON string of project-wide type registry for cross-file awareness
  typeRegistryJson?: string;

  // JSON string of the project-wide declarative macro registry, produced by
  // scanProjectSync(). Required for cross-file "import macro" JSDoc resolution.
  declarativeRegistryJson?: string;

  // "dev" | "prod" (default: "prod"). Controls declarative macro emission:
  // prod enables reverse monomorphization, dev expands inline.
  buildMode?: string;
}
```

## ExpandResult

TypeScript

```
interface ExpandResult {
  // Transformed TypeScript code
  code: string;

  // Generated type declarations (.d.ts content)
  types?: string;

  // Macro expansion metadata (JSON string)
  metadata?: string;

  // Warnings and errors from macro expansion
  diagnostics: MacroDiagnostic[];

  // Position mapping data for source maps
  sourceMapping?: SourceMappingResult;

  // Files read at build time via the buildtime API. Use these to invalidate
  // caches / trigger rebuilds when a dependency changes.
  buildtimeDependencies: string[];
}
```

## MacroDiagnostic

TypeScript

```
interface MacroDiagnostic {
  level: string;    // "error", "warning", or "info"
  message: string;
  start?: number;   // Start position in source
  end?: number;     // End position in source
}
```

## Example

TypeScript

```
import { expandSync } from "@macroforge/core";

const sourceCode = `
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
`;

const result = expandSync(sourceCode, "user.ts");

console.log("Transformed code:");
console.log(result.code);

if (result.types) {
  console.log("Type declarations:");
  console.log(result.types);
}

if (result.diagnostics.length > 0) {
  for (const diag of result.diagnostics) {
    console.log(`[${diag.level}] ${diag.message}`);
  }
}
```

## Error Handling

Syntax errors and macro errors are returned in the `diagnostics` array, not thrown as exceptions:

TypeScript

```
const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.level === "error") {
    console.error(`Error at ${diag.start}: ${diag.message}`);
  }
}
```
