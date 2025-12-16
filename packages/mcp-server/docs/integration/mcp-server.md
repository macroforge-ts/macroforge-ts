# MCP Server
 *The MCP (Model Context Protocol) server enables AI assistants to understand and work with Macroforge macros, providing documentation lookup, code validation, and macro expansion.*
 The local (stdio) version of the MCP server is available via the [`@macroforge/mcp-server`](https://www.npmjs.com/package/@macroforge/mcp-server) npm package. You can either install it globally and then reference it in your configuration or run it with `npx`:
 ```
npx -y @macroforge/mcp-server
``` Here's how to set it up in some common MCP clients:
 ## Claude Code
 To include the local MCP version in Claude Code, simply run the following command:
 ```
claude mcp add -t stdio -s [scope] macroforge -- npx -y @macroforge/mcp-server
``` The `[scope]` must be `user`, `project` or `local`.
 ## Claude Desktop
 In the Settings > Developer section, click on Edit Config. It will open the folder with a `claude_desktop_config.json` file in it. Edit the file to include the following configuration:
 ```
&#123;
    "mcpServers": &#123;
        "macroforge": &#123;
            "command": "npx",
            "args": ["-y", "@macroforge/mcp-server"]
        &#125;
    &#125;
&#125;
``` ## Codex CLI
 Add the following to your `config.toml` (which defaults to `~/.codex/config.toml`, but refer to [the configuration documentation](https://github.com/openai/codex/blob/main/docs/config.md) for more advanced setups):
 ```
[mcp_servers.macroforge]
command = "npx"
args = ["-y", "@macroforge/mcp-server"]
``` ## Gemini CLI
 To include the local MCP version in Gemini CLI, simply run the following command:
 ```
gemini mcp add -t stdio -s [scope] macroforge npx -y @macroforge/mcp-server
``` The `[scope]` must be `user`, `project` or `local`.
 ## Other Clients
 If we didn't include the MCP client you are using, refer to their documentation for `stdio` servers and use `npx` as the command and `-y @macroforge/mcp-server` as the arguments.
 ## Available Tools
 The MCP server provides five tools for AI assistants:
 | Tool | Description |
| --- | --- |
| `list-sections` | Lists all available Macroforge documentation sections |
| `get-documentation` | Retrieves full documentation for one or more sections |
| `macroforge-autofixer` | Validates code with `@derive` decorators and returns diagnostics |
| `expand-code` | Expands macros and returns the transformed TypeScript code |
| `get-macro-info` | Retrieves documentation for macros and field decorators |
 > **Note:** For code validation and expansion features (macroforge-autofixer, expand-code, get-macro-info), the MCP server requires macroforge as a peer dependency. Install it in your project with npm install macroforge.