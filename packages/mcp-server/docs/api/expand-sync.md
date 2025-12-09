# expandSync()

*Expands macros in TypeScript code synchronously and returns the transformed output.*

## Signature

```typescript
function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult
```

## Parameters

| `code` 
| `string` 
| TypeScript source code to transform 

| `filepath` 
| `string` 
| File path (used for error reporting) 

| `options` 
| `ExpandOptions` 
| Optional configuration

## ExpandOptions

```typescript
interface ExpandOptions {
  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;
}
```

## ExpandResult

```typescript
interface ExpandResult {
  // Transformed TypeScript code
  code: string;

  // Generated type declarations (.d.ts content)
  types?: string;

  // Macro expansion metadata (JSON string)
  metadata?: string;

  // Warnings and errors from macro expansion
  diagnostics: MacroDiagnostic[];

  // Position mapping data for source maps
  sourceMapping?: SourceMappingResult;
}
```

## MacroDiagnostic

```typescript
interface MacroDiagnostic {
  message: string;
  severity: "error" | "warning" | "info";
  span: {
    start: number;
    end: number;
  };
}
```

## Example

```typescript
import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts");

console.log("Transformed code:");
console.log(result.code);

if (result.types) {
  console.log("Type declarations:");
  console.log(result.types);
}

if (result.diagnostics.length > 0) {
  for (const diag of result.diagnostics) {
    console.log(\`[\${diag.severity}] \${diag.message}\`);
  }
}
```

## Error Handling

Syntax errors and macro errors are returned in the `diagnostics` array, not thrown as exceptions:

```typescript
const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.severity === "error") {
    console.error(\`Error at \${diag.span.start}: \${diag.message}\`);
  }
}
```