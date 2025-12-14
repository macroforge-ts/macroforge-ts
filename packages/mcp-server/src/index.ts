#!/usr/bin/env node
/**
 * @module @macroforge/mcp-server
 *
 * Macroforge MCP (Model Context Protocol) Server
 *
 * This module provides the main entry point for the Macroforge MCP server,
 * which enables AI assistants and other MCP clients to access Macroforge
 * documentation, validate code with @derive decorators, and expand macros.
 *
 * The server communicates over stdio transport and exposes the following tools:
 * - `list-sections` - List available documentation sections
 * - `get-documentation` - Retrieve documentation content
 * - `macroforge-autofixer` - Validate TypeScript code with @derive decorators
 * - `expand-code` - Expand Macroforge macros and show generated code
 * - `get-macro-info` - Get documentation for macros and decorators
 *
 * @example
 * ```bash
 * # Run the server directly
 * npx @macroforge/mcp-server
 *
 * # Or configure in your MCP client settings
 * {
 *   "mcpServers": {
 *     "macroforge": {
 *       "command": "npx",
 *       "args": ["@macroforge/mcp-server"]
 *     }
 *   }
 * }
 * ```
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { registerTools } from './tools/index.js';

// Get package info for server metadata
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const pkg = JSON.parse(readFileSync(join(__dirname, '..', 'package.json'), 'utf8'));
const { name, version } = pkg;

/**
 * Initializes and starts the Macroforge MCP server.
 *
 * This function performs the following setup:
 * 1. Creates an MCP Server instance with name and version from package.json
 * 2. Registers all Macroforge tools (documentation, validation, expansion)
 * 3. Sets up error handling for server errors
 * 4. Configures graceful shutdown on SIGINT (Ctrl+C)
 * 5. Connects to stdio transport for client communication
 *
 * The server runs until terminated and logs status to stderr
 * (stdout is reserved for MCP protocol communication).
 *
 * @returns A promise that resolves when the server is connected
 * @throws Will exit the process with code 1 if server initialization fails
 */
async function main(): Promise<void> {
  // Create server instance with metadata
  const server = new Server(
    {
      name,
      version,
    },
    {
      capabilities: {
        tools: {},
      },
    }
  );

  // Register all tools
  registerTools(server);

  // Set up error handling
  server.onerror = (error) => {
    console.error('[MCP Error]', error);
  };

  // Handle process termination
  process.on('SIGINT', async () => {
    await server.close();
    process.exit(0);
  });

  // Connect to transport
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error('Macroforge MCP server running on stdio');
}

// Run the server
main().catch((error) => {
  console.error('Failed to start MCP server:', error);
  process.exit(1);
});
