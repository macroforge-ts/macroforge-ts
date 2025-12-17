# expandSync()
  *Synchronously expands macros in TypeScript code. This is the standalone macro expansion function that doesn't use caching. For cached expansion, use [`NativePlugin::process_file`] instead.*
 ## Signature
 ```
function&nbsp;expandSync(
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;options?:&nbsp;ExpandOptions
):&nbsp;ExpandResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| code | string | TypeScript source code to transform |
| filepath | string | File path (used for error reporting) |
| options | ExpandOptions | Optional configuration |
 ## ExpandOptions
 ```
interface&nbsp;ExpandOptions&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Keep&nbsp;@derive&nbsp;decorators&nbsp;in&nbsp;output&nbsp;(default:&nbsp;false)
&nbsp;&nbsp;keepDecorators?:&nbsp;boolean;
&#125;
``` ## ExpandResult
 ```
interface&nbsp;ExpandResult&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Transformed&nbsp;TypeScript&nbsp;code
&nbsp;&nbsp;code:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Generated&nbsp;type&nbsp;declarations&nbsp;(.d.ts&nbsp;content)
&nbsp;&nbsp;types?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Macro&nbsp;expansion&nbsp;metadata&nbsp;(JSON&nbsp;string)
&nbsp;&nbsp;metadata?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Warnings&nbsp;and&nbsp;errors&nbsp;from&nbsp;macro&nbsp;expansion
&nbsp;&nbsp;diagnostics:&nbsp;MacroDiagnostic[];

&nbsp;&nbsp;//&nbsp;Position&nbsp;mapping&nbsp;data&nbsp;for&nbsp;source&nbsp;maps
&nbsp;&nbsp;sourceMapping?:&nbsp;SourceMappingResult;
&#125;
``` ## MacroDiagnostic
 ```
interface&nbsp;MacroDiagnostic&nbsp;&#123;
&nbsp;&nbsp;message:&nbsp;string;
&nbsp;&nbsp;severity:&nbsp;"error"&nbsp;|&nbsp;"warning"&nbsp;|&nbsp;"info";
&nbsp;&nbsp;span:&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;start:&nbsp;number;
&nbsp;&nbsp;&nbsp;&nbsp;end:&nbsp;number;
&nbsp;&nbsp;&#125;;
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
 Syntax errors and macro errors are returned in the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">diagnostics</code> array, not thrown as exceptions:
 ```
const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.severity === "error") {
    console.error(`Error at ${diag.span.start}: ${diag.message}`);
  }
}
```