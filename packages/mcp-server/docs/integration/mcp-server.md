# MCP Server
 *The MCP (Model Context Protocol) server enables AI assistants to understand and work with Macroforge macros, providing documentation lookup, code validation, and macro expansion.*
 The local (stdio) version of the MCP server is available via the [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">mcp<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">server</code>](https://www.npmjs.com/package/@macroforge/mcp-server) npm package. You can either install it globally and then reference it in your configuration or run it with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">npx</code>:
 ```
npx&nbsp;-y&nbsp;@macroforge/mcp-server
``` Here's how to set it up in some common MCP clients:
 ## Claude Code
 To include the local MCP version in Claude Code, simply run the following command:
 ```
claude&nbsp;mcp&nbsp;add&nbsp;-t&nbsp;stdio&nbsp;-s&nbsp;[scope]&nbsp;macroforge&nbsp;--&nbsp;npx&nbsp;-y&nbsp;@macroforge/mcp-server
``` The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">[scope]</code> must be <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">user</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">project</code> or <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">local</code>.
 ## Claude Desktop
 In the Settings > Developer section, click on Edit Config. It will open the folder with a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">claude_desktop_config.json</code> file in it. Edit the file to include the following configuration:
 ```
&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"mcpServers":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"macroforge":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"command":&nbsp;"npx",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"args":&nbsp;["-y",&nbsp;"@macroforge/mcp-server"]
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Codex CLI
 Add the following to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#FDAEB7;--shiki-dark-font-style:italic;--shiki-light:#B31D28;--shiki-light-font-style:italic">config.toml</code> (which defaults to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#FDAEB7;--shiki-dark-font-style:italic;--shiki-light:#B31D28;--shiki-light-font-style:italic">~/.codex/config.toml</code>, but refer to [the configuration documentation](https://github.com/openai/codex/blob/main/docs/config.md) for more advanced setups):
 ```
[mcp_servers.macroforge]
command&nbsp;=&nbsp;"npx"
args&nbsp;=&nbsp;["-y",&nbsp;"@macroforge/mcp-server"]
``` ## Gemini CLI
 To include the local MCP version in Gemini CLI, simply run the following command:
 ```
gemini&nbsp;mcp&nbsp;add&nbsp;-t&nbsp;stdio&nbsp;-s&nbsp;[scope]&nbsp;macroforge&nbsp;npx&nbsp;-y&nbsp;@macroforge/mcp-server
``` The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">[scope]</code> must be <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">user</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">project</code> or <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">local</code>.
 ## Other Clients
 If we didn't include the MCP client you are using, refer to their documentation for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">stdio</code> servers and use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">npx</code> as the command and <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">y @macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">mcp<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">server</code> as the arguments.
 ## Available Tools
 The MCP server provides five tools for AI assistants:
 | Tool | Description |
| --- | --- |
| list-sections | Lists all available Macroforge documentation sections |
| get-documentation | Retrieves full documentation for one or more sections |
| macroforge-autofixer | Validates code with @derive decorators and returns diagnostics |
| expand-code | Expands macros and returns the transformed TypeScript code |
| get-macro-info | Retrieves documentation for macros and field decorators |
 > **Note:** For code validation and expansion features (macroforge-autofixer, expand-code, get-macro-info), the MCP server requires macroforge as a peer dependency. Install it in your project with npm install macroforge.