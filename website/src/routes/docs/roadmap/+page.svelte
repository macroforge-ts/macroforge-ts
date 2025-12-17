<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Roadmap - Macroforge Documentation</title>
	<meta name="description" content="Upcoming features and planned improvements for Macroforge." />
</svelte:head>

<h1>Roadmap</h1>

<p class="lead">
	Planned features and improvements for Macroforge. This roadmap reflects our current priorities but may change based on community feedback.
</p>

<h2 id="ide-extensions">IDE Extensions</h2>

<p>
	Bring Macroforge support directly into your favorite editors with native extensions.
</p>

<table>
	<thead>
		<tr>
			<th>Extension</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>VS Code / Cursor</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Native extension with macro expansion preview, error highlighting, and completions</td>
		</tr>
		<tr>
			<td>Zed</td>
			<td><span class="text-success">Available</span></td>
			<td>Full support via vtsls-macroforge and svelte-macroforge extensions</td>
		</tr>
		<tr>
			<td>Neovim</td>
			<td><span class="text-muted-foreground">Considering</span></td>
			<td>LSP integration for Neovim users</td>
		</tr>
		<tr>
			<td>JetBrains (WebStorm)</td>
			<td><span class="text-muted-foreground">Considering</span></td>
			<td>Plugin for WebStorm and other JetBrains IDEs</td>
		</tr>
	</tbody>
</table>

<h2 id="framework-support">Framework Support</h2>

<p>
	Expanding Macroforge to work seamlessly with popular frontend frameworks.
</p>

<table>
	<thead>
		<tr>
			<th>Framework</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Svelte / SvelteKit</td>
			<td><span class="text-success">Available</span></td>
			<td>Full support via svelte-language-server and Vite plugin</td>
		</tr>
		<tr>
			<td>React</td>
			<td><span class="text-warning">Planned</span></td>
			<td>React-specific macros and integration with React tooling</td>
		</tr>
		<tr>
			<td>Vue</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Vue SFC support and Vue-specific derive macros</td>
		</tr>
		<tr>
			<td>Angular</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Angular decorator integration and CLI support</td>
		</tr>
		<tr>
			<td>Solid</td>
			<td><span class="text-warning">Planned</span></td>
			<td>SolidJS integration</td>
		</tr>
	</tbody>
</table>

<h2 id="typescript-macros">Pure TypeScript Macro Creation</h2>

<p>
	While Rust provides the best performance and type safety, we recognize that not everyone wants to write Rust. We're exploring options for writing macros in pure TypeScript.
</p>

<table>
	<thead>
		<tr>
			<th>Feature</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>TypeScript Macro API</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Define macros using TypeScript with a simple API</td>
		</tr>
		<tr>
			<td>Template Strings</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Generate code using tagged template literals</td>
		</tr>
		<tr>
			<td>AST Helpers</td>
			<td><span class="text-warning">Planned</span></td>
			<td>TypeScript utilities for working with the AST</td>
		</tr>
	</tbody>
</table>

<h2 id="built-in-macros">Built-in Macros</h2>

<p>
	Expanding the library of built-in derive macros.
</p>

<table>
	<thead>
		<tr>
			<th>Macro</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Debug, Clone, PartialEq, Ord, Hash</td>
			<td><span class="text-success">Available</span></td>
			<td>Core derive macros</td>
		</tr>
		<tr>
			<td>Serialize, Deserialize</td>
			<td><span class="text-success">Available</span></td>
			<td>JSON serialization with validation</td>
		</tr>
		<tr>
			<td>Builder</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Generate builder pattern for classes</td>
		</tr>
		<tr>
			<td>Immutable</td>
			<td><span class="text-muted-foreground">Considering</span></td>
			<td>Generate immutable update methods (with, set)</td>
		</tr>
	</tbody>
</table>

<h2 id="distribution">Distribution & Packaging</h2>

<p>
	Making it easier to publish and share custom macros.
</p>

<table>
	<thead>
		<tr>
			<th>Feature</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Native Node Binaries</td>
			<td><span class="text-success">Available</span></td>
			<td>Platform-specific binaries for maximum performance</td>
		</tr>
		<tr>
			<td>WASM Binary Generation</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Cross-platform WebAssembly binaries for easier macro distribution</td>
		</tr>
		<tr>
			<td>Macro Registry</td>
			<td><span class="text-muted-foreground">Considering</span></td>
			<td>Central registry for discovering and sharing community macros</td>
		</tr>
	</tbody>
</table>

<h2 id="tooling">Tooling & DX</h2>

<p>
	Improvements to the developer experience.
</p>

<table>
	<thead>
		<tr>
			<th>Feature</th>
			<th>Status</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>CLI Expansion</td>
			<td><span class="text-success">Available</span></td>
			<td>Expand macros from the command line</td>
		</tr>
		<tr>
			<td>Macro Playground</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Web-based playground to test macros</td>
		</tr>
		<tr>
			<td>create-macroforge</td>
			<td><span class="text-warning">Planned</span></td>
			<td>Scaffolding tool for new macro projects</td>
		</tr>
		<tr>
			<td>Macro Debugging</td>
			<td><span class="text-muted-foreground">Considering</span></td>
			<td>Step-through debugging for macro expansion</td>
		</tr>
	</tbody>
</table>

<h2 id="contributing">Contributing</h2>

<p>
	Interested in helping? We welcome contributions of all kinds:
</p>

<ul>
	<li>Feature requests and feedback on <a href="https://github.com/rymskip/macroforge-ts/issues">GitHub Issues</a></li>
	<li>Pull requests for new macros or improvements</li>
	<li>Documentation improvements</li>
	<li>Framework integrations</li>
</ul>
