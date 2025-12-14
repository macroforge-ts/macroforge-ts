# @macroforge/mcp-server

MCP server for Macroforge documentation and code analysis

[![npm version](https://badge.fury.io/js/%40macroforge%2Fmcp-server.svg)](https://www.npmjs.com/package/@macroforge/mcp-server)

## Overview

@macroforge/mcp-server

Macroforge MCP (Model Context Protocol) Server

This module provides the main entry point for the Macroforge MCP server,
which enables AI assistants and other MCP clients to access Macroforge
documentation, validate code with @derive decorators, and expand macros.

The server communicates over stdio transport and exposes the following tools:
- `list-sections` - List available documentation sections
- `get-documentation` - Retrieve documentation content
- `macroforge-autofixer` - Validate TypeScript code with @derive decorators
- `expand-code` - Expand Macroforge macros and show generated code
- `get-macro-info` - Get documentation for macros and decorators

@example
```bash
# Run the server directly
npx @macroforge/mcp-server

# Or configure in your MCP client settings
{
"mcpServers": {
"macroforge": {
"command": "npx",
"args": ["@macroforge/mcp-server"]
}
}
}
```

## Installation

```bash
npm install @macroforge/mcp-server
```

## Examples

```typescript
# Run the server directly
npx @macroforge/mcp-server
# Or configure in your MCP client settings
{
"mcpServers": {
"macroforge": {
"command": "npx",
"args": ["@macroforge/mcp-server"]
}
}
}
```

## Documentation

See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/mcp-server) on the Macroforge website.

## License

MIT
