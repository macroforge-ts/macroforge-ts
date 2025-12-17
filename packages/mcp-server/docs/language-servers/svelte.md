# Svelte Language Server
 *<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">language<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">server</code> provides full Svelte IDE support with macroforge integration.*
  **Developer Installation Required This package is not yet published as an official extension. You'll need to build and install it manually. ## Features
 - **Svelte syntax diagnostics** - Errors and warnings in .svelte files
 - **HTML support** - Hover info, autocompletions, Emmet, outline symbols
 - **CSS/SCSS/LESS** - Diagnostics, hover, completions, formatting, Emmet, color picking
 - **TypeScript/JavaScript** - Full language features with macroforge macro expansion
 - **Go-to-definition** - Navigate to macro-generated code
 - **Code actions** - Quick fixes and refactorings
 ## Installation
 ### 1. Clone the Repository
 ```
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts
``` ### 2. Build the Language Server
 ```
# Install dependencies
npm install

# Build the Svelte language server
cd packages/svelte-language-server
npm run build
``` ### 3. Configure Your Editor
 The language server exposes a **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelteserver</code> binary that implements the Language Server Protocol (LSP). Configure your editor to use it:
 ```
# The binary is located at:
./packages/svelte-language-server/bin/server.js
``` ## Package Info
 | Package | @macroforge/svelte-language-server |
| Version | 0.1.7 |
| CLI Command | svelteserver |
| Node Version | >= 18.0.0 |
 ## How It Works
 The Svelte language server extends the standard Svelte language tooling with macroforge integration:
 1. Parses <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.svelte</code> files and extracts TypeScript/JavaScript blocks
 2. Expands macros using the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">typescript<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">plugin</code>
 3. Maps diagnostics back to original source positions
 4. Provides completions for macro-generated methods
 ## Using with Zed
 For Zed editor, see the [Zed Extensions](../../docs/language-servers/zed) page for the dedicated <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge</code> extension.