# API Reference
 <span class="stats svelte-1c8t0id">55 exported items *Macroforge provides a programmatic API for expanding macros in TypeScript code.*
 ## Overview
 ```
import&nbsp;&#123;
&nbsp;&nbsp;expandSync,
&nbsp;&nbsp;transformSync,
&nbsp;&nbsp;checkSyntax,
&nbsp;&nbsp;parseImportSources,
&nbsp;&nbsp;NativePlugin,
&nbsp;&nbsp;PositionMapper
&#125;&nbsp;from&nbsp;"macroforge";
``` ## Core Functions
 | Function | Description |
| --- | --- |
| [expandSync()](../docs/api/expand-sync) | Expand macros synchronously |
| [transformSync()](../docs/api/transform-sync) | Transform code with additional metadata |
| checkSyntax() | Validate TypeScript syntax |
| parseImportSources() | Extract import information |
 ## Classes
 | Class | Description |
| --- | --- |
| [NativePlugin](../docs/api/native-plugin) | Stateful plugin with caching |
| [PositionMapper](../docs/api/position-mapper) | Maps positions between original and expanded code |
 ## Quick Example
 ```
import&nbsp;&#123;&nbsp;expandSync&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

const&nbsp;sourceCode&nbsp;=&nbsp;\`
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;constructor(name:&nbsp;string)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&#125;
&#125;
\`;

const&nbsp;result&nbsp;=&nbsp;expandSync(sourceCode,&nbsp;"user.ts",&nbsp;&#123;
&nbsp;&nbsp;keepDecorators:&nbsp;false
&#125;);

console.log(result.code);
//&nbsp;Output:&nbsp;class&nbsp;with&nbsp;toString()&nbsp;method&nbsp;generated

if&nbsp;(result.diagnostics.length&nbsp;>&nbsp;0)&nbsp;&#123;
&nbsp;&nbsp;console.error("Errors:",&nbsp;result.diagnostics);
&#125;
``` ## Detailed Reference
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">expandSync<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>](../docs/api/expand-sync) - Full options and return types
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">transformSync<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>](../docs/api/transform-sync) - Transform with source maps
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code>](../docs/api/native-plugin) - Caching for language servers
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">PositionMapper</code>](../docs/api/position-mapper) - Position mapping utilities