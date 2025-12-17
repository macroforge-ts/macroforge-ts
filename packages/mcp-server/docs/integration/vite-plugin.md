# Vite Plugin
 *The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.*
 ## Installation
 ```
npm install -D @macroforge/vite-plugin
``` ## Configuration
 Add the plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">vite.config.ts</code>:
 ```
import macroforge from "@macroforge/vite-plugin";
import &#123; defineConfig &#125; from "vite";

export default defineConfig(&#123;
  plugins: [
    macroforge()
  ]
&#125;);
``` ## Options
 ```
macroforge(&#123;
  // Generate .d.ts files for expanded code
  generateTypes: true,

  // Output directory for generated types
  typesOutputDir: ".macroforge/types",

  // Emit metadata files for debugging
  emitMetadata: false,

  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // File patterns to process
  include: ["**/*.ts", "**/*.tsx"],
  exclude: ["node_modules/**"]
&#125;)
``` ### Option Reference
 | Option | Type | Default | Description |
| --- | --- | --- | --- |
| generateTypes | boolean | true | Generate .d.ts files |
| typesOutputDir | string | .macroforge/types | Where to write type files |
| emitMetadata | boolean | false | Emit macro metadata files |
| keepDecorators | boolean | false | Keep decorators in output |
 ## Framework Integration
 ### React (Vite)
 ```
import macroforge from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import &#123; defineConfig &#125; from "vite";

export default defineConfig(&#123;
  plugins: [
    macroforge(),  // Before React plugin
    react()
  ]
&#125;);
``` ### SvelteKit
 ```
import macroforge from "@macroforge/vite-plugin";
import &#123; sveltekit &#125; from "@sveltejs/kit/vite";
import &#123; defineConfig &#125; from "vite";

export default defineConfig(&#123;
  plugins: [
    macroforge(),  // Before SvelteKit
    sveltekit()
  ]
&#125;);
``` > **Note:** Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first. ## Development Server
 During development, the plugin:
 - Watches for file changes
 - Expands macros on save
 - Provides HMR support for expanded code
 ## Production Build
 During production builds, the plugin:
 - Expands all macros in the source files
 - Generates type declaration files
 - Strips <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators from output