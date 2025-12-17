# TypeScript Plugin
 *The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.*
 ## Installation
 ```
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/typescript-plugin
``` ## Configuration
 Add the plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>:
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
``` ## VS Code Setup
 VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):
 1. Open the Command Palette (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Cmd<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Ctrl <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> Shift <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> P</code>)
 2. Search for "TypeScript: Select TypeScript Version"
 3. Choose "Use Workspace Version"
  **Tip Add this setting to your **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.vscode/settings.json</code> to make it permanent: ```
&#123;
&nbsp;&nbsp;"typescript.tsdk":&nbsp;"node_modules/typescript/lib"
&#125;
``` ## Features
 ### Error Reporting
 Errors in macro-generated code are reported at the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator position:
 ```
/**&nbsp;@derive(Debug)&nbsp;*/&nbsp;&nbsp;//&nbsp;&#x3C;-&nbsp;Errors&nbsp;appear&nbsp;here
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&#125;
``` ### Completions
 The plugin provides completions for generated methods:
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice");
user.to&nbsp;&nbsp;//&nbsp;Suggests:&nbsp;toString(),&nbsp;toJSON(),&nbsp;etc.
``` ### Type Information
 Hover over generated methods to see their types:
 ```
//&nbsp;Hover&nbsp;over&nbsp;'clone'&nbsp;shows:
//&nbsp;(method)&nbsp;User.clone():&nbsp;User
const&nbsp;copy&nbsp;=&nbsp;user.clone();
``` ## Troubleshooting
 ### Plugin Not Loading
 1. Ensure you're using the workspace TypeScript version
 2. Restart the TypeScript server (Command Palette → "TypeScript: Restart TS Server")
 3. Check that the plugin is listed in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>
 ### Errors Not Showing
 If errors from macros aren't appearing:
 1. Make sure the Vite plugin is also installed (for source file watching)
 2. Check that your file is saved (plugins process on save)