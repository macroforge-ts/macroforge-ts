<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
</script>

<svelte:head>
	<title>NativePlugin - Macroforge Documentation</title>
	<meta name="description" content="API reference for the NativePlugin class." />
</svelte:head>

<h1>NativePlugin</h1>

<p class="lead">
	A stateful plugin class with version-based caching, designed for integration with language servers and IDEs.
</p>

<h2 id="constructor">Constructor</h2>

<CodeBlock code={`const plugin = new NativePlugin();`} lang="typescript" />

<h2 id="methods">Methods</h2>

<h3>processFile()</h3>

<p>Process a file with version-based caching:</p>

<CodeBlock code={`processFile(
  filepath: string,
  code: string,
  options?: ProcessFileOptions
): ExpandResult`} lang="typescript" />

<CodeBlock code={`interface ProcessFileOptions {
  // Cache key - if unchanged, returns cached result
  version?: string;
}`} lang="typescript" />

<h3>getMapper()</h3>

<p>Get the position mapper for a previously processed file:</p>

<CodeBlock code={`getMapper(filepath: string): NativeMapper | null`} lang="typescript" />

<h3>mapDiagnostics()</h3>

<p>Map diagnostics from expanded positions to original positions:</p>

<CodeBlock code={`mapDiagnostics(
  filepath: string,
  diagnostics: JsDiagnostic[]
): JsDiagnostic[]`} lang="typescript" />

<h3>log() / setLogFile()</h3>

<p>Logging utilities for debugging:</p>

<CodeBlock code={`log(message: string): void
setLogFile(path: string): void`} lang="typescript" />

<h2 id="caching">Caching Behavior</h2>

<p>
	The plugin caches expansion results by file path and version:
</p>

<CodeBlock code={`const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });`} lang="typescript" />

<h2 id="example">Example: Language Server Integration</h2>

<CodeBlock code={`import { NativePlugin } from "macroforge";

class MacroforgeLanguageService {
  private plugin = new NativePlugin();

  processDocument(uri: string, content: string, version: number) {
    // Process with version-based caching
    const result = this.plugin.processFile(uri, content, {
      version: String(version)
    });

    // Get mapper for position translation
    const mapper = this.plugin.getMapper(uri);

    return { result, mapper };
  }

  getSemanticDiagnostics(uri: string, diagnostics: Diagnostic[]) {
    // Map positions from expanded to original
    return this.plugin.mapDiagnostics(uri, diagnostics);
  }
}`} lang="typescript" />

<h2 id="thread-safety">Thread Safety</h2>

<p>
	The <code>NativePlugin</code> class is thread-safe and can be used from multiple async contexts.
	Each file is processed in an isolated thread with its own stack space.
</p>
