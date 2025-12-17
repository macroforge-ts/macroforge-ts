# Vite Plugin
 *The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.*
 ## Installation
 ```
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/vite-plugin
``` ## Configuration
 Add the plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">vite.config.ts</code>:
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge()
&nbsp;&nbsp;]
&#125;);
``` ## Options
 ```
macroforge(&#123;
&nbsp;&nbsp;//&nbsp;Generate&nbsp;.d.ts&nbsp;files&nbsp;for&nbsp;expanded&nbsp;code
&nbsp;&nbsp;generateTypes:&nbsp;true,

&nbsp;&nbsp;//&nbsp;Output&nbsp;directory&nbsp;for&nbsp;generated&nbsp;types
&nbsp;&nbsp;typesOutputDir:&nbsp;".macroforge/types",

&nbsp;&nbsp;//&nbsp;Emit&nbsp;metadata&nbsp;files&nbsp;for&nbsp;debugging
&nbsp;&nbsp;emitMetadata:&nbsp;false,

&nbsp;&nbsp;//&nbsp;Keep&nbsp;@derive&nbsp;decorators&nbsp;in&nbsp;output&nbsp;(for&nbsp;debugging)
&nbsp;&nbsp;keepDecorators:&nbsp;false,

&nbsp;&nbsp;//&nbsp;File&nbsp;patterns&nbsp;to&nbsp;process
&nbsp;&nbsp;include:&nbsp;["**/*.ts",&nbsp;"**/*.tsx"],
&nbsp;&nbsp;exclude:&nbsp;["node_modules/**"]
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
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;react&nbsp;from&nbsp;"@vitejs/plugin-react";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),&nbsp;&nbsp;//&nbsp;Before&nbsp;React&nbsp;plugin
&nbsp;&nbsp;&nbsp;&nbsp;react()
&nbsp;&nbsp;]
&#125;);
``` ### SvelteKit
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;sveltekit&nbsp;&#125;&nbsp;from&nbsp;"@sveltejs/kit/vite";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),&nbsp;&nbsp;//&nbsp;Before&nbsp;SvelteKit
&nbsp;&nbsp;&nbsp;&nbsp;sveltekit()
&nbsp;&nbsp;]
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