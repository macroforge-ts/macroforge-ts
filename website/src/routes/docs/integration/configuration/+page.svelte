<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>Configuration - Macroforge Documentation</title>
	<meta name="description" content="Configure Macroforge with macroforge.config.ts." />
</svelte:head>

<h1>Configuration</h1>

<p class="lead">
	Macroforge can be configured with a <code>macroforge.config.ts</code> (or <code>.js</code>) file in your project root.
</p>

<h2 id="config-file">Configuration File</h2>

<p>
	Macroforge searches for config files in the following order, walking up from the input file's directory:
</p>

<ul>
	<li><code>macroforge.config.ts</code></li>
	<li><code>macroforge.config.mts</code></li>
	<li><code>macroforge.config.js</code></li>
	<li><code>macroforge.config.mjs</code></li>
	<li><code>macroforge.config.cjs</code></li>
</ul>

<p>Create a <code>macroforge.config.ts</code> file:</p>

<CodeBlock code={`import { defineConfig } from "macroforge/config";

export default defineConfig({
  keepDecorators: false,
  generateConvenienceConst: true,
});`} lang="typescript" filename="macroforge.config.ts" />

<h2 id="options">Options Reference</h2>

<h3>keepDecorators</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>boolean</code></td>
		</tr>
		<tr>
			<td>Default</td>
			<td><code>false</code></td>
		</tr>
	</tbody>
</table>

<p>
	Whether to preserve <code>@derive</code> decorators in the output code after macro expansion.
	When <code>false</code>, decorators are removed after expansion since they serve only as compile-time directives. When <code>true</code>, decorators are kept in the output, which can be useful for debugging or when using runtime reflection.
</p>

<h3>generateConvenienceConst</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>boolean</code></td>
		</tr>
		<tr>
			<td>Default</td>
			<td><code>true</code></td>
		</tr>
	</tbody>
</table>

<p>
	Whether to generate a convenience const for non-class types. When <code>true</code>, generates an <code>export const TypeName = &#123; ... &#125; as const;</code> that groups all generated functions for a type into a single namespace-like object. For example: <code>export const User = &#123; clone: userClone, serialize: userSerialize &#125; as const;</code>.
</p>

<h3>foreignTypes</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>Record&lt;string, ForeignTypeHandler&gt;</code></td>
		</tr>
	</tbody>
</table>

<p>
	Configuration files can define foreign type handlers for external types like Effect's <code>DateTime</code>. When a matching type is found during expansion, the configured handlers are used automatically.
</p>

<CodeBlock code={`// macroforge.config.ts
import { DateTime } from "effect";
import { defineConfig } from "macroforge/config";

export default defineConfig({
  foreignTypes: {
    "DateTime.DateTime": {
      from: ["effect"],
      aliases: [
        { name: "DateTime", from: "effect/DateTime" }
      ],
      serialize: (v) => DateTime.formatIso(v),
      deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
      default: () => DateTime.unsafeNow(),
      // Optional shape check for union variant matching
      hasShape: (v) => v instanceof Date || typeof v === "string"
    }
  }
});`} lang="typescript" filename="macroforge.config.ts" />

<p>
  Each foreign type handler supports the following properties:
</p>

<ul>
  <li><code>from</code>: Array of module paths this type can be imported from.</li>
  <li><code>serialize</code>: Function <code>(value) => unknown</code> for serialization.</li>
  <li><code>deserialize</code>: Function <code>(raw) => T</code> for deserialization.</li>
  <li><code>default</code>: Function <code>() => T</code> for default value generation.</li>
  <li><code>hasShape</code>: Optional function <code>(value) => boolean</code> used for shape-check predicate expression in union variant matching.</li>
  <li><code>aliases</code>: Array of <code>&#123; name: string, from: string &#125;</code> objects for alternative type-package pairs.</li>
</ul>

<h3>vite</h3>

<table>
	<tbody>
		<tr>
			<td>Type</td>
			<td><code>VitePluginConfig</code></td>
		</tr>
	</tbody>
</table>

<p>
	These options configure the <code>@macroforge/vite-plugin</code> behavior.
</p>

<CodeBlock code={`// macroforge.config.ts
import { defineConfig } from "macroforge/config";

export default defineConfig({
  vite: {
    // Whether to generate .d.ts type definition files from expanded code
    generateTypes: true,
    typesOutputDir: ".macroforge/types",

    // Whether to emit macro IR metadata as JSON files
    emitMetadata: true,
    metadataOutputDir: ".macroforge/meta",

    // Enable disk-based expansion cache in dev mode (vite dev)
    devCache: true
  }
});`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>generateTypes</code>: Whether to generate <code>.d.ts</code> type definition files from expanded code (default: <code>true</code>).</li>
  <li><code>typesOutputDir</code>: Output directory for generated type definitions, relative to project root (default: <code>".macroforge/types"</code>).</li>
  <li><code>emitMetadata</code>: Whether to emit macro IR metadata as JSON files (default: <code>true</code>).</li>
  <li><code>metadataOutputDir</code>: Output directory for metadata JSON files, relative to project root (default: <code>".macroforge/meta"</code>).</li>
  <li><code>devCache</code>: Enable disk-based expansion cache in dev mode (<code>vite dev</code>) (default: <code>true</code>).</li>
</ul>
