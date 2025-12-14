<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>Installation - Macroforge Documentation</title>
	<meta name="description" content="Get started with Macroforge - install and set up TypeScript macros in your project." />
</svelte:head>

<h1>Installation</h1>

<p class="lead">
	Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros.
</p>

<h2 id="requirements">Requirements</h2>

<ul>
	<li>Node.js 24.0 or later</li>
	<li>TypeScript 5.9 or later</li>
</ul>

<h2 id="install">Install the Package</h2>

<p>Install Macroforge using your preferred package manager:</p>

<CodeBlock code="npm install macroforge" lang="bash" filename="npm" />

<CodeBlock code="bun add macroforge" lang="bash" filename="bun" />

<CodeBlock code="pnpm add macroforge" lang="bash" filename="pnpm" />

<Alert type="info">
	Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64).
</Alert>

<h2 id="basic-usage">Basic Usage</h2>

<p>
	The simplest way to use Macroforge is with the built-in derive macros. Add a <code>@derive</code> comment decorator to your class:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

// After macro expansion, User has:
// - toString(): string              (from Debug)
// - clone(): User                   (from Clone)
// - equals(other: unknown): boolean (from PartialEq)`} lang="typescript" filename="user.ts" />

<h2 id="ide-integration">IDE Integration</h2>

<p>
	For the best development experience, add the TypeScript plugin to your <code>tsconfig.json</code>:
</p>

<CodeBlock code={`{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}`} lang="json" filename="tsconfig.json" />

<p>
	This enables features like:
</p>

<ul>
	<li>Accurate error positions in your source code</li>
	<li>Autocompletion for generated methods</li>
	<li>Type checking for expanded code</li>
</ul>

<h2 id="build-integration">Build Integration (Vite)</h2>

<p>
	If you're using Vite, add the plugin to your config for automatic macro expansion during build:
</p>

<CodeBlock code={`import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge({
      generateTypes: true,
      typesOutputDir: ".macroforge/types"
    })
  ]
});`} lang="typescript" filename="vite.config.ts" />

<h2 id="next-steps">Next Steps</h2>

<p>
	Now that you have Macroforge installed, learn how to use it:
</p>

<ul>
	<li><a href="{base}/docs/getting-started/first-macro">Create your first macro</a></li>
	<li><a href="{base}/docs/concepts">Understand how macros work</a></li>
	<li><a href="{base}/docs/builtin-macros">Explore built-in macros</a></li>
</ul>
