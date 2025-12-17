# Svelte Preprocessor
 *The Svelte preprocessor expands Macroforge macros in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"><<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">script<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">></code> blocks before Svelte compilation, enabling seamless macro usage in Svelte components.*
 ## Installation
 ```
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/svelte-preprocessor
``` ## Configuration
 Add the preprocessor to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte.config.js</code>:
 ```
import&nbsp;adapter&nbsp;from&nbsp;'@sveltejs/adapter-auto';
import&nbsp;&#123;&nbsp;vitePreprocess&nbsp;&#125;&nbsp;from&nbsp;'@sveltejs/vite-plugin-svelte';
import&nbsp;&#123;&nbsp;macroforgePreprocess&nbsp;&#125;&nbsp;from&nbsp;'@macroforge/svelte-preprocessor';

/**&nbsp;@type&nbsp;&#123;import('@sveltejs/kit').Config&#125;&nbsp;*/
const&nbsp;config&nbsp;=&nbsp;&#123;
&nbsp;&nbsp;preprocess:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforgePreprocess(),&nbsp;&nbsp;//&nbsp;Expand&nbsp;macros&nbsp;FIRST
&nbsp;&nbsp;&nbsp;&nbsp;vitePreprocess()&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Then&nbsp;handle&nbsp;TypeScript/CSS
&nbsp;&nbsp;],

&nbsp;&nbsp;kit:&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;adapter:&nbsp;adapter()
&nbsp;&nbsp;&#125;
&#125;;

export&nbsp;default&nbsp;config;
```  **Warning Always place **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforgePreprocess<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code> **before** other preprocessors like <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">vitePreprocess<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>. This ensures macros are expanded before TypeScript compilation. ## Usage
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators directly in your Svelte component scripts:
 ```
&#x3C;script&nbsp;lang="ts">
&nbsp;&nbsp;/**&nbsp;@derive(Debug,&nbsp;Clone)&nbsp;*/
&nbsp;&nbsp;class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;&nbsp;&nbsp;email:&nbsp;string;

&nbsp;&nbsp;&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;email:&nbsp;string)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.email&nbsp;=&nbsp;email;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;

&nbsp;&nbsp;let&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;"alice@example.com");
&nbsp;&nbsp;console.log(user.toString());&nbsp;&nbsp;//&nbsp;Generated&nbsp;by&nbsp;Debug&nbsp;macro
&#x3C;/script>

&#x3C;p>User:&nbsp;&#123;user.name&#125;&#x3C;/p>
``` ## Options
 ```
macroforgePreprocess(&#123;
&nbsp;&nbsp;//&nbsp;Keep&nbsp;@derive&nbsp;decorators&nbsp;in&nbsp;output&nbsp;(for&nbsp;debugging)
&nbsp;&nbsp;keepDecorators:&nbsp;false,

&nbsp;&nbsp;//&nbsp;Process&nbsp;JavaScript&nbsp;files&nbsp;(not&nbsp;just&nbsp;TypeScript)
&nbsp;&nbsp;processJavaScript:&nbsp;false
&#125;)
``` ### Option Reference
 | Option | Type | Default | Description |
| --- | --- | --- | --- |
| keepDecorators | boolean | false | Keep decorators in output |
| processJavaScript | boolean | false | Process <script> blocks without lang="ts" |
 ## How It Works
 The preprocessor:
 1. Intercepts <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49"><<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">script lang<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">=<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"ts"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">></code> blocks in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.svelte</code> files
 2. Checks for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators (skips files without them)
 3. Expands macros using the native Macroforge binary
 4. Returns the transformed code for Svelte compilation
  **Tip Files without **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators are passed through unchanged with zero overhead. ## SvelteKit Integration
 For SvelteKit projects, you can use both the preprocessor (for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.svelte</code> files) and the Vite plugin (for standalone <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.ts</code> files):
 ```
//&nbsp;svelte.config.js
import&nbsp;&#123;&nbsp;macroforgePreprocess&nbsp;&#125;&nbsp;from&nbsp;'@macroforge/svelte-preprocessor';
import&nbsp;&#123;&nbsp;vitePreprocess&nbsp;&#125;&nbsp;from&nbsp;'@sveltejs/vite-plugin-svelte';

export&nbsp;default&nbsp;&#123;
&nbsp;&nbsp;preprocess:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforgePreprocess(),
&nbsp;&nbsp;&nbsp;&nbsp;vitePreprocess()
&nbsp;&nbsp;]
&#125;;
``` ```
//&nbsp;vite.config.ts
import&nbsp;macroforge&nbsp;from&nbsp;'@macroforge/vite-plugin';
import&nbsp;&#123;&nbsp;sveltekit&nbsp;&#125;&nbsp;from&nbsp;'@sveltejs/kit/vite';
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;'vite';

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),&nbsp;&nbsp;//&nbsp;For&nbsp;.ts&nbsp;files
&nbsp;&nbsp;&nbsp;&nbsp;sveltekit()
&nbsp;&nbsp;]
&#125;);
``` ## Using with Vitest
 The preprocessor works seamlessly with Vitest for testing Svelte components:
 ```
//&nbsp;vitest.config.ts
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;'vitest/config';
import&nbsp;&#123;&nbsp;sveltekit&nbsp;&#125;&nbsp;from&nbsp;'@sveltejs/kit/vite';
import&nbsp;&#123;&nbsp;svelteTesting&nbsp;&#125;&nbsp;from&nbsp;'@testing-library/svelte/vite';
import&nbsp;macroforge&nbsp;from&nbsp;'@macroforge/vite-plugin';

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),
&nbsp;&nbsp;&nbsp;&nbsp;sveltekit(),
&nbsp;&nbsp;&nbsp;&nbsp;svelteTesting()
&nbsp;&nbsp;],
&nbsp;&nbsp;test:&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;environment:&nbsp;'jsdom',
&nbsp;&nbsp;&nbsp;&nbsp;include:&nbsp;['src/**/*.&#123;test,spec&#125;.&#123;js,ts&#125;']
&nbsp;&nbsp;&#125;
&#125;);
``` ## Svelte 5 Runes Compatibility
 The preprocessor is fully compatible with Svelte 5 runes (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$state</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$derived</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$props</code>, etc.). Files using runes but without <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators are skipped entirely.
 ```
&#x3C;script&nbsp;lang="ts">
&nbsp;&nbsp;//&nbsp;Runes&nbsp;work&nbsp;normally
&nbsp;&nbsp;let&nbsp;count&nbsp;=&nbsp;$state(0);
&nbsp;&nbsp;let&nbsp;doubled&nbsp;=&nbsp;$derived(count&nbsp;*&nbsp;2);

&nbsp;&nbsp;//&nbsp;Macros&nbsp;expand&nbsp;correctly
&nbsp;&nbsp;/**&nbsp;@derive(Debug)&nbsp;*/
&nbsp;&nbsp;class&nbsp;Counter&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;value:&nbsp;number;
&nbsp;&nbsp;&nbsp;&nbsp;constructor(value:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.value&nbsp;=&nbsp;value;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;
&#x3C;/script>
```