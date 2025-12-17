# Command Line Interface
  *This binary provides command-line utilities for working with Macroforge TypeScript macros. It is designed for development workflows, enabling macro expansion and type checking without requiring Node.js integration.*
 ## Installation
 The CLI is a Rust binary. You can install it using Cargo:
 ```
cargo&nbsp;install&nbsp;macroforge_ts
``` Or build from source:
 ```
git&nbsp;clone&nbsp;https://github.com/rymskip/macroforge-ts.git
cd&nbsp;macroforge-ts/crates
cargo&nbsp;build&nbsp;--release&nbsp;--bin&nbsp;macroforge

#&nbsp;The&nbsp;binary&nbsp;is&nbsp;at&nbsp;target/release/macroforge
``` ## Commands
 ### macroforge expand
 Expands macros in a TypeScript file and outputs the transformed code.
 ```
macroforge&nbsp;expand&nbsp;&#x3C;input>&nbsp;[options]
``` #### Arguments
 | Argument | Description |
| --- | --- |
| <input> | Path to the TypeScript or TSX file to expand |
 #### Options
 | Option | Description |
| --- | --- |
| --out <path> | Write the expanded JavaScript/TypeScript to a file |
| --types-out <path> | Write the generated .d.ts declarations to a file |
| --print | Print output to stdout even when --out is specified |
| --builtin-only | Use only built-in Rust macros (faster, but no external macro support) |
 #### Examples
 Expand a file and print to stdout:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts
``` Expand and write to a file:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--out&nbsp;dist/user.js
``` Expand with both runtime output and type declarations:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--out&nbsp;dist/user.js&nbsp;--types-out&nbsp;dist/user.d.ts
``` Use fast built-in macros only (no external macro support):
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--builtin-only
``` > **Note:** By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where macroforge and any external macro packages are installed in node_modules. ### macroforge tsc
 Runs TypeScript type checking with macro expansion. This wraps <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsc <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">noEmit</code> and expands macros before type checking, so your generated methods are properly type-checked.
 ```
macroforge&nbsp;tsc&nbsp;[options]
``` #### Options
 | Option | Description |
| --- | --- |
| -p, --project <path> | Path to tsconfig.json (defaults to tsconfig.json in current directory) |
 #### Examples
 Type check with default tsconfig.json:
 ```
macroforge&nbsp;tsc
``` Type check with a specific config:
 ```
macroforge&nbsp;tsc&nbsp;-p&nbsp;tsconfig.build.json
``` ## Output Format
 ### Expanded Code
 When expanding a file like this:
 ```
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;
``` The CLI outputs the expanded code with the generated methods:
 ```
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  [Symbol.for("nodejs.util.inspect.custom")](): string {
    return `User { name: ${this.name}, age: ${this.age} }`;
  }
}
``` ### Diagnostics
 Errors and warnings are printed to stderr in a readable format:
 ```
[macroforge]&nbsp;error&nbsp;at&nbsp;src/user.ts:5:1:&nbsp;Unknown&nbsp;derive&nbsp;macro:&nbsp;InvalidMacro
[macroforge]&nbsp;warning&nbsp;at&nbsp;src/user.ts:10:3:&nbsp;Field&nbsp;'unused'&nbsp;is&nbsp;never&nbsp;used
``` ## Use Cases
 ### CI/CD Type Checking
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> tsc</code> in your CI pipeline to type-check with macro expansion:
 ```
#&nbsp;package.json
&#123;
&nbsp;&nbsp;"scripts":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"typecheck":&nbsp;"macroforge&nbsp;tsc"
&nbsp;&nbsp;&#125;
&#125;
``` ### Debugging Macro Output
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> expand</code> to inspect what code your macros generate:
 ```
macroforge&nbsp;expand&nbsp;src/models/user.ts&nbsp;|&nbsp;less
``` ### Build Pipeline
 Generate expanded files as part of a custom build:
 ```
#!/bin/bash
for&nbsp;file&nbsp;in&nbsp;src/**/*.ts;&nbsp;do
&nbsp;&nbsp;outfile="dist/$(basename&nbsp;"$file"&nbsp;.ts).js"
&nbsp;&nbsp;macroforge&nbsp;expand&nbsp;"$file"&nbsp;--out&nbsp;"$outfile"
done
``` ## Built-in vs Full Mode
 By default, the CLI uses Node.js for full macro support including external macros. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">builtin<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">only</code> for faster expansion when you only need built-in macros:
 | Feature | Default (Node.js) | --builtin-only (Rust) |
| --- | --- | --- |
| Built-in macros | Yes | Yes |
| External macros | Yes | No |
| Performance | Standard | Faster |
| Dependencies | Requires macroforge in node_modules | None |