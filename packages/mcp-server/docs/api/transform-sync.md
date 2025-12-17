# transformSync()
  *Synchronously transforms TypeScript code through the macro expansion system. This is similar to [`expand_sync`] but returns a [`TransformResult`] which includes source map information (when available).*
 ## Signature
 ```
function transformSync(
  code: string,
  filepath: string
): TransformResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| code | string | TypeScript source code to transform |
| filepath | string | File path (used for error reporting) |
 ## TransformResult
 ```
interface TransformResult &#123;
  // Transformed TypeScript code
  code: string;

  // Source map (JSON string, not yet implemented)
  map?: string;

  // Generated type declarations
  types?: string;

  // Macro expansion metadata
  metadata?: string;
&#125;
``` ## Comparison with expandSync()
 | Feature | expandSync | transformSync |
| --- | --- | --- |
| Options | Yes | No |
| Diagnostics | Yes | No |
| Source Mapping | Yes | Limited |
| Use Case | General purpose | Build tools |
 ## Example
 ```
import &#123; transformSync &#125; from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User &#123;
  name: string;
&#125;
\`;

const result = transformSync(sourceCode, "user.ts");

console.log(result.code);

if (result.types) &#123;
  // Write to .d.ts file
  fs.writeFileSync("user.d.ts", result.types);
&#125;

if (result.metadata) &#123;
  // Parse and use metadata
  const meta = JSON.parse(result.metadata);
  console.log("Macros expanded:", meta);
&#125;
``` ## When to Use
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">transformSync</code> when:
 - Building custom integrations
 - You need raw output without diagnostics
 - You're implementing a build tool plugin
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">expandSync</code> for most other use cases, as it provides better error handling.