<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>The Derive System - Macroforge Documentation</title>
	<meta name="description" content="Learn how the @derive decorator system works in Macroforge." />
</svelte:head>

<h1>The Derive System</h1>

<p class="lead">
	The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with <code>@derive</code>.
</p>

<h2 id="syntax">Syntax</h2>

<p>
	The derive decorator uses JSDoc comment syntax:
</p>

<CodeBlock code={`/** @derive(MacroName) */
class MyClass { }

/** @derive(Debug, Clone, Eq) */
class AnotherClass { }`} lang="typescript" />

<Alert type="note">
	The <code>@derive</code> decorator must be in a JSDoc comment (<code>/** */</code>) immediately before the class declaration.
</Alert>

<h2 id="how-it-works">How It Works</h2>

<ol>
	<li><strong>Declaration</strong>: You write <code>@derive(MacroName)</code> before a class</li>
	<li><strong>Discovery</strong>: Macroforge finds all derive decorators in your code</li>
	<li><strong>Expansion</strong>: Each named macro receives the class AST and generates code</li>
	<li><strong>Injection</strong>: Generated methods/properties are added to the class</li>
</ol>

<h2 id="multiple-macros">Multiple Macros</h2>

<p>
	You can derive multiple traits in one decorator:
</p>

<CodeBlock code={`/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;
}

// This generates:
// - toString() from Debug
// - clone() from Clone
// - equals() from Eq`} lang="typescript" />

<p>Or use separate decorators:</p>

<CodeBlock code={`/** @derive(Debug) */
/** @derive(Clone) */
/** @derive(Eq) */
class User {
  name: string;
  age: number;
}`} lang="typescript" />

<h2 id="field-decorators">Field-Level Decorators</h2>

<p>
	Many macros support field-level customization:
</p>

<CodeBlock code={`/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  password: string;
}`} lang="typescript" />

<p>
	Each macro defines its own field decorator options. Check the documentation for each macro to see available options.
</p>

<h2 id="what-can-be-derived">What Can Be Derived</h2>

<p>
	The derive system works on:
</p>

<ul>
	<li><strong>Classes</strong>: The primary target for derive macros</li>
	<li><strong>Interfaces</strong>: Some macros can generate companion functions</li>
</ul>

<Alert type="warning">
	Enums are not currently supported by the derive system.
</Alert>

<h2 id="built-in-vs-custom">Built-in vs Custom Macros</h2>

<p>
	Macroforge comes with built-in macros (Debug, Clone, Eq), but you can also create custom macros in Rust. Custom macros use the same derive syntax:
</p>

<CodeBlock code={`/** import macro { JSON } from "@my/macros"; */

/** @derive(JSON) */
class User {
  name: string;
}`} lang="typescript" />

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="/docs/builtin-macros">Explore built-in macros</a></li>
	<li><a href="/docs/custom-macros">Create custom macros</a></li>
</ul>
