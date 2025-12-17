# Installation
 *Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros.*
 ## Requirements
 - Node.js 24.0 or later
 - TypeScript 5.9 or later
 ## Install the Package
 Install Macroforge using your preferred package manager:
 ```
npm&nbsp;install&nbsp;macroforge
``` ```
bun&nbsp;add&nbsp;macroforge
``` ```
pnpm&nbsp;add&nbsp;macroforge
```  **Info Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64). ## Basic Usage
 The simplest way to use Macroforge is with the built-in derive macros. Add a **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> comment decorator to your class:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;

//&nbsp;After&nbsp;macro&nbsp;expansion,&nbsp;User&nbsp;has:
//&nbsp;-&nbsp;toString():&nbsp;string&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(from&nbsp;Debug)
//&nbsp;-&nbsp;clone():&nbsp;User&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(from&nbsp;Clone)
//&nbsp;-&nbsp;equals(other:&nbsp;unknown):&nbsp;boolean&nbsp;(from&nbsp;PartialEq)
``` ## IDE Integration
 For the best development experience, add the TypeScript plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>:
 ```
&#123;
&nbsp;&nbsp;"compilerOptions":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"plugins":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"name":&nbsp;"@macroforge/typescript-plugin"
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;]
&nbsp;&nbsp;&#125;
&#125;
``` This enables features like:
 - Accurate error positions in your source code
 - Autocompletion for generated methods
 - Type checking for expanded code
 ## Build Integration (Vite)
 If you're using Vite, add the plugin to your config for automatic macro expansion during build:
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;generateTypes:&nbsp;true,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;typesOutputDir:&nbsp;".macroforge/types"
&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;]
&#125;);
``` ## Next Steps
 Now that you have Macroforge installed, learn how to use it:
 - [Create your first macro](../docs/getting-started/first-macro)
 - [Understand how macros work](../docs/concepts)
 - [Explore built-in macros](../docs/builtin-macros)