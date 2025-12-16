# API Reference
 <span class="stats svelte-1c8t0id">52 exported items *Macroforge provides a programmatic API for expanding macros in TypeScript code.*
 ## Overview
 ```
import &#123;
  expandSync,
  transformSync,
  checkSyntax,
  parseImportSources,
  NativePlugin,
  PositionMapper
&#125; from "macroforge";
``` ## Core Functions
 | Function | Description |
| --- | --- |
| [`expandSync()`](../docs/api/expand-sync) | Expand macros synchronously |
| [`transformSync()`](../docs/api/transform-sync) | Transform code with additional metadata |
| `checkSyntax()` | Validate TypeScript syntax |
| `parseImportSources()` | Extract import information |
 ## Classes
 | Class | Description |
| --- | --- |
| [`NativePlugin`](../docs/api/native-plugin) | Stateful plugin with caching |
| [`PositionMapper`](../docs/api/position-mapper) | Maps positions between original and expanded code |
 ## Quick Example
 ```
import &#123; expandSync &#125; from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User &#123;
  name: string;
  constructor(name: string) &#123;
    this.name = name;
  &#125;
&#125;
\`;

const result = expandSync(sourceCode, "user.ts", &#123;
  keepDecorators: false
&#125;);

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) &#123;
  console.error("Errors:", result.diagnostics);
&#125;
``` ## Detailed Reference
 - [`expandSync()`](../docs/api/expand-sync) - Full options and return types
 - [`transformSync()`](../docs/api/transform-sync) - Transform with source maps
 - [`NativePlugin`](../docs/api/native-plugin) - Caching for language servers
 - [`PositionMapper`](../docs/api/position-mapper) - Position mapping utilities