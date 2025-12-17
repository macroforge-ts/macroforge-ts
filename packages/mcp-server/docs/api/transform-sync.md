# transformSync()
  *Synchronously transforms TypeScript code through the macro expansion system. This is similar to [`expand_sync`] but returns a [`TransformResult`] which includes source map information (when available).*
 ## Signature
 ```
function&nbsp;transformSync(
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;filepath:&nbsp;string
):&nbsp;TransformResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| code | string | TypeScript source code to transform |
| filepath | string | File path (used for error reporting) |
 ## TransformResult
 ```
interface&nbsp;TransformResult&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Transformed&nbsp;TypeScript&nbsp;code
&nbsp;&nbsp;code:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Source&nbsp;map&nbsp;(JSON&nbsp;string,&nbsp;not&nbsp;yet&nbsp;implemented)
&nbsp;&nbsp;map?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Generated&nbsp;type&nbsp;declarations
&nbsp;&nbsp;types?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Macro&nbsp;expansion&nbsp;metadata
&nbsp;&nbsp;metadata?:&nbsp;string;
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
import&nbsp;&#123;&nbsp;transformSync&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

const&nbsp;sourceCode&nbsp;=&nbsp;\`
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&#125;
\`;

const&nbsp;result&nbsp;=&nbsp;transformSync(sourceCode,&nbsp;"user.ts");

console.log(result.code);

if&nbsp;(result.types)&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Write&nbsp;to&nbsp;.d.ts&nbsp;file
&nbsp;&nbsp;fs.writeFileSync("user.d.ts",&nbsp;result.types);
&#125;

if&nbsp;(result.metadata)&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Parse&nbsp;and&nbsp;use&nbsp;metadata
&nbsp;&nbsp;const&nbsp;meta&nbsp;=&nbsp;JSON.parse(result.metadata);
&nbsp;&nbsp;console.log("Macros&nbsp;expanded:",&nbsp;meta);
&#125;
``` ## When to Use
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">transformSync</code> when:
 - Building custom integrations
 - You need raw output without diagnostics
 - You're implementing a build tool plugin
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">expandSync</code> for most other use cases, as it provides better error handling.