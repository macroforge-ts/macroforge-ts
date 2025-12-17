# Command Line Interface
  *This binary provides command-line utilities for working with Macroforge TypeScript macros. It is designed for development workflows, enabling macro expansion and type checking without requiring Node.js integration.*
 ## Installation
 The CLI is a Rust binary. You can install it using Cargo:
 ```
cargo install macroforge_ts
``` Or build from source:
 ```
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge
``` ## Commands
 ### macroforge expand
 Expands macros in a TypeScript file and outputs the transformed code.
 ```
macroforge expand &#x3C;input> [options]
``` #### Arguments
 | Argument | Description |
| --- | --- |
| &lt;input&gt; | Path to the TypeScript or TSX file to expand |
 #### Options
 | Option | Description |
| --- | --- |
| --out&lt;path&gt; | Write the expanded JavaScript/TypeScript to a file |
| --types-out&lt;path&gt; | Write the generated .d.ts declarations to a file |
| --print | Print output to stdout even when --out is specified |
| --builtin-only | Use only built-in Rust macros (faster, but no external macro support) |
 #### Examples
 Expand a file and print to stdout:
 ```
macroforge expand src/user.ts
``` Expand and write to a file:
 ```
macroforge expand src/user.ts --out dist/user.js
``` Expand with both runtime output and type declarations:
 ```
macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts
``` Use fast built-in macros only (no external macro support):
 ```
macroforge expand src/user.ts --builtin-only
``` > **Note:** By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where macroforge and any external macro packages are installed in node_modules. ### macroforge tsc
 Runs TypeScript type checking with macro expansion. This wraps <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsc<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">noEmit</code> and expands macros before type checking, so your generated methods are properly type-checked.
 ```
macroforge tsc [options]
``` #### Options
 | Option | Description |
| --- | --- |
| -p,--project&lt;path&gt; | Path to tsconfig.json (defaults to tsconfig.json in current directory) |
 #### Examples
 Type check with default tsconfig.json:
 ```
macroforge tsc
``` Type check with a specific config:
 ```
macroforge tsc -p tsconfig.build.json
``` ## Output Format
 ### Expanded Code
 When expanding a file like this:
 ```
/** @derive(Debug) */
class User &#123;
  name: string;
  age: number;

  constructor(name: string, age: number) &#123;
    this.name = name;
    this.age = age;
  &#125;
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
[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used
``` ## Use Cases
 ### CI/CD Type Checking
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">tsc</code> in your CI pipeline to type-check with macro expansion:
 ```
# package.json
&#123;
  "scripts": &#123;
    "typecheck": "macroforge tsc"
  &#125;
&#125;
``` ### Debugging Macro Output
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">expand</code> to inspect what code your macros generate:
 ```
macroforge expand src/models/user.ts | less
``` ### Build Pipeline
 Generate expanded files as part of a custom build:
 ```
#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done
``` ## Built-in vs Full Mode
 By default, the CLI uses Node.js for full macro support including external macros. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">builtin<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">only</code> for faster expansion when you only need built-in macros:
 | Feature | Default (Node.js) | --builtin-only (Rust) |
| --- | --- | --- |
| Built-in macros | Yes | Yes |
| External macros | Yes | No |
| Performance | Standard | Faster |
| Dependencies | Requires macroforge in node_modules | None |