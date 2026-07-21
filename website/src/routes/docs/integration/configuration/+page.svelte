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

<CodeBlock code={`export default {
  keepDecorators: false,
  generateConvenienceConst: true,
};`} lang="typescript" filename="macroforge.config.ts" />

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
export default {
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
};`} lang="typescript" filename="macroforge.config.ts" />

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
export default {
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
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>generateTypes</code>: Whether to generate <code>.d.ts</code> type definition files from expanded code (default: <code>true</code>).</li>
  <li><code>typesOutputDir</code>: Output directory for generated type definitions, relative to project root (default: <code>".macroforge/types"</code>).</li>
  <li><code>emitMetadata</code>: Whether to emit macro IR metadata as JSON files (default: <code>true</code>).</li>
  <li><code>metadataOutputDir</code>: Output directory for metadata JSON files, relative to project root (default: <code>".macroforge/meta"</code>).</li>
  <li><code>devCache</code>: Enable disk-based expansion cache in dev mode (<code>vite dev</code>) (default: <code>true</code>).</li>
</ul>

<h3>cfg</h3>

<p>
	Build flags consumed by the <code>@cfg</code> attribute macro. See
	<a href="/docs/attributes">Attribute Macros</a> for the annotation syntax.
</p>

<CodeBlock code={`export default {
  cfg: {
    features: ["beta", "experimental"],
    target: "node",
    debugAssertions: false,
    custom: { tier: "pro" }
  }
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>features</code>: Active feature flags; <code>@cfg(&lbrace; feature: "beta" &rbrace;)</code> passes when the value is a member (default: <code>[]</code>).</li>
  <li><code>target</code>: Matched exactly against <code>@cfg(&lbrace; target: … &rbrace;)</code> (default: unset).</li>
  <li><code>debugAssertions</code>: Boolean matched against <code>@cfg(&lbrace; debugAssertions: … &rbrace;)</code> (default: <code>false</code>).</li>
  <li><code>custom</code>: Arbitrary keys matched exactly by any other annotation key (default: <code>&lbrace;&rbrace;</code>).</li>
</ul>

<h3>deprecated</h3>

<p>Behavior of the <code>@deprecated</code> attribute macro.</p>

<CodeBlock code={`export default {
  deprecated: {
    runtimeWarn: true,
    failOnUse: false
  }
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>failOnUse</code>: Promote use of a deprecated symbol from an editor hint to a hard expansion error (default: <code>false</code>).</li>
  <li><code>runtimeWarn</code>: Reserved. Defaults to <code>true</code> but currently has no effect — no runtime warning is injected.</li>
</ul>

<h3>mustUse</h3>

<p>Behavior of the <code>@mustUse</code> attribute macro.</p>

<CodeBlock code={`export default {
  mustUse: { mode: "lint" }
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>mode</code>: Currently only <code>"lint"</code> is recognised, which emits a diagnostic when a return value is discarded (default: <code>"lint"</code>).</li>
</ul>

<h3>nonExhaustive</h3>

<p>Behavior of the <code>@nonExhaustive</code> attribute macro.</p>

<CodeBlock code={`export default {
  nonExhaustive: { brand: "__nonExhaustive" }
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>brand</code>: Property name used in the branding intersection. Keep it stable across a project (default: <code>"__nonExhaustive"</code>).</li>
</ul>

<h3>buildtime</h3>

<p>
	Sandbox settings for <code>@buildtime</code> evaluation. See
	<a href="/docs/buildtime">Buildtime Evaluation</a> for the API.
</p>

<CodeBlock code={`export default {
  buildtime: {
    capabilities: {
      timeout: 5000,
      maxHeap: 256,
      filesystem: { read: ["src/**"], write: [] },
      env: ["NODE_ENV"],
      network: false
    },
    flags: { CHANNEL: "beta" }
  }
};`} lang="typescript" filename="macroforge.config.ts" />

<ul>
  <li><code>capabilities.timeout</code>: Evaluation budget in milliseconds, enforced (default: <code>5000</code>).</li>
  <li><code>capabilities.maxHeap</code>: Heap ceiling in MiB. Advisory — not currently enforced (default: <code>256</code>).</li>
  <li><code>capabilities.filesystem.read</code>: Globs readable via <code>buildtime.fs</code> (default: <code>["**"]</code>).</li>
  <li><code>capabilities.filesystem.write</code>: Reserved; no write API is exposed (default: <code>[]</code>).</li>
  <li><code>capabilities.env</code>: Environment variable names exposed as <code>buildtime.env.NAME</code>. Deny-by-default (default: <code>[]</code>).</li>
  <li><code>capabilities.network</code>: Reserved; no network API is exposed (default: <code>false</code>).</li>
  <li><code>flags</code>: Values returned by <code>buildtime.flags.has()</code> / <code>.get()</code> (default: <code>&lbrace;&rbrace;</code>).</li>
</ul>

<p>
	Capability keys may also be written flat (<code>buildtime.timeout</code>); the nested
	form is canonical because it matches the path sandbox diagnostics point at.
</p>
