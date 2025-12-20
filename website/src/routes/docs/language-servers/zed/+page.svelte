<script lang="ts">
    import CodeBlock from "$lib/components/ui/CodeBlock.svelte";
    import Alert from "$lib/components/ui/Alert.svelte";
</script>

<svelte:head>
    <title>Zed Extensions - Macroforge Documentation</title>
    <meta
        name="description"
        content="Macroforge extensions for the Zed editor."
    />
</svelte:head>

<h1>Zed Extensions</h1>

<p class="lead">
    Macroforge provides two extensions for the <a href="https://zed.dev"
        >Zed editor</a
    >: one for TypeScript via VTSLS, and one for Svelte.
</p>

<Alert type="warning" title="Developer Installation Required">
    These extensions are not yet in the Zed extension registry. You'll need to
    install them as developer extensions.
</Alert>

<h2 id="available-extensions">Available Extensions</h2>

<table>
    <thead>
        <tr>
            <th>Extension</th>
            <th>Description</th>
            <th>Location</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>vtsls-macroforge</code></td>
            <td>VTSLS with macroforge support for TypeScript</td>
            <td><code>crates/extensions/vtsls-macroforge</code></td>
        </tr>
        <tr>
            <td><code>svelte-macroforge</code></td>
            <td>Svelte language support with macroforge</td>
            <td><code>crates/extensions/svelte-macroforge</code></td>
        </tr>
    </tbody>
</table>

<h2 id="installation">Installation</h2>

<h3>1. Clone the Repository</h3>

<CodeBlock
    code={`git clone https://github.com/macroforge-ts/zed-extensions.git
cd macroforge-ts`}
    lang="bash"
/>

<h3>2. Build the Extension</h3>

<p>Build the extension you want to use:</p>

<CodeBlock
    code={`# For VTSLS (TypeScript)
cd crates/extensions/vtsls-macroforge

# Or for Svelte
cd crates/extensions/svelte-macroforge`}
    lang="bash"
/>

<h3>3. Install as Dev Extension in Zed</h3>

<p>
    In Zed, open the command palette and run <strong
        >zed: install dev extension</strong
    >, then select the extension directory.
</p>

<p>Alternatively, symlink the extension to your Zed extensions directory:</p>

<CodeBlock
    code={`# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/.config/zed/extensions/installed/vtsls-macroforge`}
    lang="bash"
/>

<h2 id="vtsls-macroforge">vtsls-macroforge</h2>

<p>
    This extension wraps <a href="https://github.com/yioneko/vtsls">VTSLS</a> (a TypeScript
    language server) with macroforge integration. It provides:
</p>

<ul>
    <li>Full TypeScript language features</li>
    <li>Macro expansion at edit time</li>
    <li>Accurate error positions in original source</li>
    <li>Completions for macro-generated methods</li>
</ul>

<h2 id="svelte-macroforge">svelte-macroforge</h2>

<p>
    This extension provides Svelte support using the <code
        >@macroforge/svelte-language-server</code
    >. It includes:
</p>

<ul>
    <li>Svelte component syntax support</li>
    <li>HTML, CSS, and TypeScript features</li>
    <li>Macroforge integration in script blocks</li>
</ul>

<h2 id="troubleshooting">Troubleshooting</h2>

<h3>Extension not loading</h3>

<p>
    Make sure you've restarted Zed after installing the extension. Check the Zed
    logs for any error messages.
</p>

<h3>Macros not expanding</h3>

<p>
    Ensure your project has the <code>macroforge</code> package installed and a
    valid <code>tsconfig.json</code> with the TypeScript plugin configured.
</p>
