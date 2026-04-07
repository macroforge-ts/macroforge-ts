<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>MCP Server - Macroforge Documentation</title>
	<meta name="description" content="Set up the Macroforge MCP server for AI-assisted macro development with Claude and other MCP-compatible clients." />
</svelte:head>

<h1>MCP Server</h1>

<p class="lead">
	The MCP (Model Context Protocol) server enables AI assistants to understand and work with Macroforge macros, providing documentation lookup, code validation, and macro expansion.
</p>

<p>
	The local (stdio) version of the MCP server is available via the <a href="https://www.npmjs.com/package/@macroforge/mcp-server"><code>@macroforge/mcp-server</code></a> npm package. You can either install it globally and then reference it in your configuration or run it with <code>npx</code>:
</p>

<CodeBlock code={`npx -y @macroforge/mcp-server`} lang="bash" />

<p>Here's how to set it up in some common MCP clients:</p>

<h2 id="claude-code">Claude Code</h2>

<p>To include the local MCP version in Claude Code, simply run the following command:</p>

<CodeBlock code={`claude mcp add -t stdio -s [scope] macroforge -- npx -y @macroforge/mcp-server`} lang="bash" />

<p>The <code>[scope]</code> must be <code>user</code>, <code>project</code> or <code>local</code>.</p>

<h2 id="claude-desktop">Claude Desktop</h2>

<p>In the Settings &gt; Developer section, click on Edit Config. It will open the folder with a <code>claude_desktop_config.json</code> file in it. Edit the file to include the following configuration:</p>

<CodeBlock code={`{
    "mcpServers": {
        "macroforge": {
            "command": "npx",
            "args": ["-y", "@macroforge/mcp-server"]
        }
    }
}`} lang="json" filename="claude_desktop_config.json" />

<h2 id="codex-cli">Codex CLI</h2>

<p>Add the following to your <code>config.toml</code> (which defaults to <code>~/.codex/config.toml</code>, but refer to <a href="https://github.com/openai/codex/blob/main/docs/config.md">the configuration documentation</a> for more advanced setups):</p>

<CodeBlock code={`[mcp_servers.macroforge]
command = "npx"
args = ["-y", "@macroforge/mcp-server"]`} lang="toml" filename="config.toml" />

<h2 id="gemini-cli">Gemini CLI</h2>

<p>To include the local MCP version in Gemini CLI, simply run the following command:</p>

<CodeBlock code={`gemini mcp add -t stdio -s [scope] macroforge npx -y @macroforge/mcp-server`} lang="bash" />

<p>The <code>[scope]</code> must be <code>user</code>, <code>project</code> or <code>local</code>.</p>

<h2 id="other-clients">Other Clients</h2>

<p>
	If we didn't include the MCP client you are using, refer to their documentation for <code>stdio</code> servers and use <code>npx</code> as the command and <code>-y @macroforge/mcp-server</code> as the arguments.
</p>

<h2 id="available-tools">Available Tools</h2>

<p>The MCP server provides five tools for AI assistants:</p>

<table>
	<thead>
		<tr>
			<th>Tool</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>list-sections</code></td>
			<td>Lists all available Macroforge documentation sections</td>
		</tr>
		<tr>
			<td><code>get-documentation</code></td>
			<td>Retrieves full documentation for one or more sections</td>
		</tr>
		<tr>
			<td><code>macroforge-autofixer</code></td>
			<td>Validates code with <code>@derive</code> decorators and returns diagnostics</td>
		</tr>
		<tr>
			<td><code>expand-code</code></td>
			<td>Expands macros and returns the transformed TypeScript code</td>
		</tr>
		<tr>
			<td><code>get-macro-info</code></td>
			<td>Retrieves documentation for macros and field decorators</td>
		</tr>
	</tbody>
</table>

<Alert type="note">
	For code validation and expansion features (<code>macroforge-autofixer</code>, <code>expand-code</code>, <code>get-macro-info</code>), the MCP server requires <code>macroforge</code> as a peer dependency. Install it in your project with <code>npm install macroforge</code>.
</Alert>
