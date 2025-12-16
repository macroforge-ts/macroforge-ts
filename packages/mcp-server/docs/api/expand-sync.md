# expandSync()
  *Synchronously expands macros in TypeScript code. This is the standalone macro expansion function that doesn't use caching. For cached expansion, use [`NativePlugin::process_file`] instead.*
 ## Signature
 ```
function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| `code` | `string` | TypeScript source code to transform |
| `filepath` | `string` | File path (used for error reporting) |
| `options` | `ExpandOptions` | Optional configuration |
 ## ExpandOptions
 ```
interface ExpandOptions &#123;
  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;
&#125;
``` ## ExpandResult
 ```
interface ExpandResult &#123;
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
&#125;
``` ## MacroDiagnostic
 ```
interface MacroDiagnostic &#123;
  message: string;
  severity: "error" | "warning" | "info";
  span: &#123;
    start: number;
    end: number;
  &#125;;
&#125;
``` ## Example
 ```
import { expandSync } from "macroforge";

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
    console.log(`[${diag.severity}] ${diag.message}`);
  }
}
``` ## Error Handling
 Syntax errors and macro errors are returned in the `diagnostics` array, not thrown as exceptions:
 ```
const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.severity === "error") {
    console.error(`Error at ${diag.span.start}: ${diag.message}`);
  }
}
```