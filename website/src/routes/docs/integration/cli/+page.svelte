<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
    import Alert from "$lib/components/ui/Alert.svelte";

    let { data } = $props();
</script>

<svelte:head>
    <title>CLI - Macroforge Documentation</title>
    <meta
        name="description"
        content="Command-line interface for macro expansion and TypeScript type checking."
    />
</svelte:head>

<h1>Command Line Interface</h1>

{#if data.version}
    <p class="version-badge">macroforge v{data.version}</p>
{/if}

<p class="lead">
    {data.cli?.description ||
        "The macroforge CLI provides commands for expanding macros and running type checks with macro support."}
</p>

<h2 id="installation">Installation</h2>

<p>The CLI is a Rust binary. You can install it using Cargo:</p>

<CodeBlock code={`cargo install macroforge_ts`} lang="bash" />

<p>Or build from source:</p>

<CodeBlock
    code={`git clone https://gitlab.com/macroforge-ts/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge`}
    lang="bash"
/>

<h2 id="commands">Commands</h2>

<h3 id="expand">macroforge expand</h3>

<p>Expands macros in a TypeScript file and outputs the transformed code.</p>

<CodeBlock code={`macroforge expand <input> [options]`} lang="bash" />

<h4>Arguments</h4>

<table>
    <thead>
        <tr>
            <th>Argument</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>&lt;input&gt;</code></td>
            <td>Path to the TypeScript or TSX file to expand</td>
        </tr>
    </tbody>
</table>

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>--out &lt;path&gt;</code></td>
            <td>Write the expanded JavaScript/TypeScript to a file</td>
        </tr>
        <tr>
            <td><code>--types-out &lt;path&gt;</code></td>
            <td
                >Write the generated <code>.d.ts</code> declarations to a file</td
            >
        </tr>
        <tr>
            <td><code>--print</code></td>
            <td
                >Print output to stdout even when <code>--out</code> is specified</td
            >
        </tr>
        <tr>
            <td><code>--scan</code></td>
            <td>Scan directory for TypeScript files with macros</td>
        </tr>
        <tr>
            <td><code>--include-ignored</code></td>
            <td>Include files ignored by .gitignore when scanning</td>
        </tr>
        <tr>
            <td><code>-q, --quiet</code></td>
            <td>Suppress output when no macros are found</td>
        </tr>
    </tbody>
</table>

<h4>Examples</h4>

<p>Expand a file (writes a sibling <code>src/user.expanded.ts</code>):</p>

<CodeBlock code={`macroforge expand src/user.ts`} lang="bash" />

<p>Print the expansion to stdout instead:</p>

<CodeBlock code={`macroforge expand src/user.ts --print`} lang="bash" />

<p>Expand and write to a file:</p>

<CodeBlock
    code={`macroforge expand src/user.ts --out dist/user.js`}
    lang="bash"
/>

<p>Expand with both runtime output and type declarations:</p>

<CodeBlock
    code={`macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts`}
    lang="bash"
/>

<Alert type="note">
    <span
        >Expansion runs natively in Rust — no Node.js process is spawned. External
        macro packages are loaded from <code>node_modules</code> via FFI, so run the
        CLI from your project root when your code uses them.</span
    >
</Alert>

<h3 id="tsc">macroforge tsc</h3>

<p>
    Runs TypeScript type checking with macro expansion. This wraps <code
        >tsc --noEmit</code
    > and expands macros before type checking, so your generated methods are properly
    type-checked.
</p>

<CodeBlock code={`macroforge tsc [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>-p, --project &lt;path&gt;</code></td>
            <td
                >Path to <code>tsconfig.json</code> (defaults to
                <code>tsconfig.json</code> in current directory)</td
            >
        </tr>
    </tbody>
</table>

<h4>Examples</h4>

<p>Type check with default tsconfig.json:</p>

<CodeBlock code={`macroforge tsc`} lang="bash" />

<p>Type check with a specific config:</p>

<CodeBlock code={`macroforge tsc -p tsconfig.build.json`} lang="bash" />

<h3 id="svelte-check">macroforge svelte-check</h3>

<p>
    Runs <code>svelte-check</code> with macro expansion, so Svelte components using macros are properly type-checked.
</p>

<CodeBlock code={`macroforge svelte-check [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>--workspace &lt;path&gt;</code></td>
            <td>Workspace directory (defaults to current directory)</td>
        </tr>
        <tr>
            <td><code>--tsconfig &lt;path&gt;</code></td>
            <td>Path to <code>tsconfig.json</code></td>
        </tr>
        <tr>
            <td><code>--output &lt;format&gt;</code></td>
            <td>Output format: <code>human</code>, <code>human-verbose</code>, <code>machine</code>, <code>machine-verbose</code></td>
        </tr>
        <tr>
            <td><code>--fail-on-warnings</code></td>
            <td>Exit with error on warnings (not just errors)</td>
        </tr>
    </tbody>
</table>

<h3 id="svelte-package">macroforge svelte-package</h3>

<p>
    Runs <code>svelte-package</code> with macro expansion, so a published Svelte
    library ships fully expanded source and type declarations.
</p>

<CodeBlock code={`macroforge svelte-package [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>-i, --input &lt;path&gt;</code></td>
            <td>Source directory (defaults to <code>src/lib</code>)</td>
        </tr>
        <tr>
            <td><code>-o, --output &lt;path&gt;</code></td>
            <td>Output directory (defaults to <code>dist</code>)</td>
        </tr>
        <tr>
            <td><code>--tsconfig &lt;path&gt;</code></td>
            <td>Path to <code>tsconfig.json</code></td>
        </tr>
        <tr>
            <td><code>--no-types</code></td>
            <td>Skip generating type declarations</td>
        </tr>
        <tr>
            <td><code>--full-rebuild</code></td>
            <td>Ignore the previous build and repackage everything</td>
        </tr>
    </tbody>
</table>

<h4>Incremental builds</h4>

<p>
    Packaging is incremental. Each run records what it consumed and produced under
    <code>.macroforge/svelte-package/</code>, and a run whose inputs all match the previous one
    exits without repackaging. When a rebuild is needed, only the files that changed are
    re-expanded — the rest keep the expanded output from last time.
</p>

<p>
    A file that differs only in formatting — trailing whitespace, runs of blank lines — does not
    count as a change. Note that <code>.ts</code> is transpiled on the way into the package so its
    layout is discarded anyway, but <code>.svelte</code> and <code>.js</code> are copied through
    verbatim: a formatting-only edit to those will not reach the package until the next real
    change or a <code>--full-rebuild</code>.
</p>

<p>Any of these forces a full rebuild on its own:</p>

<ul>
    <li>a changed macroforge version, <code>macroforge.config.*</code>, or external macro binary</li>
    <li>a changed <code>svelte.config.*</code>, <code>package.json</code>, or tsconfig</li>
    <li>
        a changed <code>@sveltejs/package</code>, <code>macroforge</code>, or
        <code>@macroforge/svelte-preprocessor</code> version
    </li>
    <li>different command-line options</li>
    <li>a changed project source outside the input directory</li>
    <li>an output directory that was deleted or modified behind the CLI's back</li>
</ul>

<p>
    A locally rebuilt linked package whose version did not change is the one thing this cannot
    see; <code>--full-rebuild</code> is the escape hatch. <code>macroforge refresh</code> also
    discards the build state along with the expansion cache.
</p>

<p>
    Expansion failures fail the build. A module that cannot be expanded has no correct packaged
    form, and shipping its unexpanded source publishes a library whose generated runtime is
    silently missing.
</p>

<h3 id="watch">macroforge watch</h3>

<p>
    Watches source files and maintains the macro expansion cache, keeping it up to date as files change.
</p>

<CodeBlock code={`macroforge watch [root] [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>--debounce-ms &lt;ms&gt;</code></td>
            <td>Debounce interval in milliseconds (default: 100)</td>
        </tr>
    </tbody>
</table>

<h3 id="cache">macroforge cache</h3>

<p>
    Builds the <code>.macroforge/cache</code> directory once for all source files. Useful for CI or pre-build steps.
</p>

<CodeBlock code={`macroforge cache [root] [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
    </tbody>
</table>

<h3 id="refresh">macroforge refresh</h3>

<p>
    Deletes and rebuilds the macro cache from scratch.
</p>

<CodeBlock code={`macroforge refresh [root] [options]`} lang="bash" />

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
    </tbody>
</table>

<h3 id="build">macroforge build</h3>

<p>
    Builds a macro crate to WebAssembly with <code>wasm-bindgen</code> and
    post-processes the output to add <code>$</code>-prefixed re-exports for
    function-like (Call) macros. Used when distributing your own macro
    packages.
</p>

<CodeBlock code={`macroforge build [crate_dir] [options]`} lang="bash" />

<p>Steps:</p>

<ol>
    <li><code>cargo build --release --target wasm32-unknown-unknown</code></li>
    <li>Runs <code>wasm-bindgen --target nodejs</code> into <code>pkg/</code>
        (or the directory given via <code>-o</code>)</li>
    <li>Parses the generated <code>.d.ts</code> to discover Call macros and
        appends <code>export &#123; state as $state &#125;</code>-style
        aliases so consumers can import both forms.</li>
</ol>

<h4>Options</h4>

<table>
    <thead>
        <tr>
            <th>Option</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>[crate_dir]</code></td>
            <td>Path to the macro crate (defaults to <code>.</code>)</td>
        </tr>
        <tr>
            <td><code>-o, --out &lt;out&gt;</code></td>
            <td>Output directory for the WASM package (defaults to
                <code>&lt;crate_dir&gt;/pkg</code>)</td>
        </tr>
    </tbody>
</table>

<h4>Examples</h4>

<CodeBlock
    code={`# Build the current macro crate
macroforge build

# Build a specific crate into a custom directory
macroforge build ./packages/my-macros -o dist/wasm`}
    lang="bash"
/>

<h2 id="output-format">Output Format</h2>

<h3>Expanded Code</h3>

<p>When expanding a file like this:</p>

<CodeBlock
    code={`/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`}
    lang="typescript"
/>

<p>The CLI outputs the expanded code with the generated methods:</p>

<CodeBlock
    code={`class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  [Symbol.for("nodejs.util.inspect.custom")](): string {
    return \`User { name: \${this.name}, age: \${this.age} }\`;
  }
}`}
    lang="typescript"
/>

<h3>Diagnostics</h3>

<p>Errors and warnings are printed to stderr in a readable format:</p>

<CodeBlock
    code={`[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used`}
    lang="text"
/>

<h2 id="use-cases">Use Cases</h2>

<h3>CI/CD Type Checking</h3>

<p>
    Use <code>macroforge tsc</code> in your CI pipeline to type-check with macro expansion:
</p>

<CodeBlock
    code={`# package.json
{
  "scripts": {
    "typecheck": "macroforge tsc"
  }
}`}
    lang="json"
/>

<h3>Debugging Macro Output</h3>

<p>
    Use <code>macroforge expand</code> to inspect what code your macros generate:
</p>

<CodeBlock code={`macroforge expand src/models/user.ts --print | less`} lang="bash" />

<h3>Build Pipeline</h3>

<p>Generate expanded files as part of a custom build:</p>

<CodeBlock
    code={`#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done`}
    lang="bash"
/>

<style>
    .version-badge {
        display: inline-block;
        background: var(--color-primary);
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 9999px;
        font-size: 0.75rem;
        font-weight: 500;
        margin-bottom: 1rem;
    }
</style>
