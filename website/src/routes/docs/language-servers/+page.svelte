<script lang="ts">
    import Alert from "$lib/components/ui/Alert.svelte";
    import { resolve } from "$app/paths";
</script>

<svelte:head>
    <title>Language Servers - Macroforge Documentation</title>
    <meta
        name="description"
        content="Language server integrations for Macroforge in various editors."
    />
</svelte:head>

<h1>Language Servers</h1>

<p class="lead">
    Macroforge provides language server integrations for enhanced IDE support
    beyond the TypeScript plugin.
</p>

<Alert type="warning" title="Work in Progress">
    Language server integrations are currently experimental. They work in the
    repository but are not yet published as official extensions. You'll need to
    fork the repo and install them as developer extensions.
</Alert>

<h2 id="overview">Overview</h2>

<p>
    While the <a href={resolve("/docs/integration/typescript-plugin")}
        >TypeScript Plugin</a
    > provides macro support in any TypeScript-aware editor, dedicated language servers
    offer deeper integration for specific frameworks and editors.
</p>

<table>
    <thead>
        <tr>
            <th>Integration</th>
            <th>Purpose</th>
            <th>Status</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td
                ><a href={resolve("/docs/language-servers/svelte")}
                    >Svelte Language Server</a
                ></td
            >
            <td>Full Svelte support with macroforge</td>
            <td>Working (dev install)</td>
        </tr>
        <tr>
            <td
                ><a href={resolve("/docs/language-servers/zed")}
                    >Zed Extensions</a
                ></td
            >
            <td>VTSLS and Svelte for Zed editor</td>
            <td>Working (dev install)</td>
        </tr>
    </tbody>
</table>

<h2 id="current-status">Current Status</h2>

<p>
    The language servers are functional and used during development of
    macroforge itself. However, they require manual installation:
</p>

<ol>
    <li>
        Fork or clone the <a href="https://github.com/rymskip/macroforge-ts"
            >macroforge-ts/zed-extensions repository</a
        >
    </li>
    <li>Build the extension you need</li>
    <li>Install it as a developer extension in your editor</li>
</ol>

<p>See the individual pages for detailed installation instructions.</p>

<h2 id="roadmap">Roadmap</h2>

<p>We're working on official extension releases for:</p>

<ul>
    <li>VS Code (via VTSLS)</li>
    <li>Zed (native extensions)</li>
    <li>Other editors with LSP support</li>
</ul>

<h2 id="next-steps">Detailed Guides</h2>

<ul>
    <li>
        <a href={resolve("/docs/language-servers/svelte")}
            >Svelte Language Server</a
        > - Full Svelte IDE support
    </li>
    <li>
        <a href={resolve("/docs/language-servers/zed")}>Zed Extensions</a> - VTSLS
        and Svelte for Zed
    </li>
</ul>
