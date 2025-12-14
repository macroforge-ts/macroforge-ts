/**
 * @module tools
 *
 * MCP Tool Registry and Request Handlers
 *
 * This module registers and implements all MCP tools exposed by the Macroforge server.
 * It serves as the main business logic layer, handling tool requests from MCP clients.
 *
 * ## Registered Tools
 *
 * | Tool | Purpose |
 * |------|---------|
 * | `list-sections` | List all documentation sections with metadata |
 * | `get-documentation` | Retrieve full content for specific sections |
 * | `macroforge-autofixer` | Validate TypeScript code with @derive decorators |
 * | `expand-code` | Expand macros and show generated code |
 * | `get-macro-info` | Get documentation for macros and decorators |
 *
 * ## Architecture
 *
 * The module uses:
 * - `docs-loader.js` for documentation loading and search
 * - `@macroforge/core` (optional) for native code validation and expansion
 *
 * Native bindings are loaded dynamically and gracefully degrade if unavailable.
 *
 * @see {@link registerTools} for the main entry point
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  ErrorCode,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';
import { loadSections, getSection, searchSections, type Section } from './docs-loader.js';

/** Cached documentation sections loaded at server startup */
let sections: Section[] = [];

/**
 * Registers all Macroforge MCP tools with the server instance.
 *
 * This function:
 * 1. Loads documentation sections from disk
 * 2. Registers a `tools/list` handler returning all available tools
 * 3. Registers a `tools/call` handler routing requests to appropriate handlers
 *
 * The tools registered provide documentation access and code analysis capabilities
 * for AI assistants working with Macroforge.
 *
 * @param server - The MCP Server instance to register tools with
 *
 * @example
 * ```typescript
 * import { Server } from '@modelcontextprotocol/sdk/server/index.js';
 * import { registerTools } from './tools/index.js';
 *
 * const server = new Server({ name: 'macroforge', version: '1.0.0' }, { capabilities: { tools: {} } });
 * registerTools(server);
 * ```
 */
export function registerTools(server: Server): void {
  // Load documentation sections
  sections = loadSections();

  // Register tool listing
  server.setRequestHandler(ListToolsRequestSchema, async () => {
    return {
      tools: [
        {
          name: 'list-sections',
          description: `Lists all Macroforge documentation sections.

Returns sections with:
- title: Section name
- use_cases: When this doc is useful (comma-separated keywords)
- path: File path
- category: Category name

WORKFLOW:
1. Call list-sections FIRST for any Macroforge-related task
2. Analyze use_cases to find relevant sections
3. Call get-documentation with ALL relevant section names

Example use_cases: "setup, install", "serialization, json", "validation, email"`,
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'get-documentation',
          description: `Retrieves full documentation content for Macroforge sections.

Supports flexible search by:
- Title (e.g., "Debug", "Vite Plugin")
- ID (e.g., "debug", "vite-plugin")
- Partial matches

Can accept a single section name or an array of sections.
After calling list-sections, analyze the use_cases and fetch ALL relevant sections at once.`,
          inputSchema: {
            type: 'object',
            properties: {
              section: {
                anyOf: [
                  { type: 'string' },
                  { type: 'array', items: { type: 'string' } },
                ],
                description:
                  'Section name(s) to retrieve. Supports single string or array of strings.',
              },
            },
            required: ['section'],
          },
        },
        {
          name: 'macroforge-autofixer',
          description: `Validates TypeScript code with @derive decorators using Macroforge's native validation.

Returns structured JSON diagnostics with:
- level: error | warning | info
- message: What's wrong
- location: Line and column number (when available)
- help: Suggested fix (when available)
- notes: Additional context (when available)
- summary: Count of errors, warnings, and info messages

This tool MUST be used before sending Macroforge code to the user.
If require_another_tool_call_after_fixing is true, fix the issues and validate again.

Detects:
- Invalid/unknown macro names
- Malformed @derive decorators
- @serde validator issues (email, url, length, etc.)
- Macro expansion failures
- Syntax errors in generated code`,
          inputSchema: {
            type: 'object',
            properties: {
              code: {
                type: 'string',
                description: 'TypeScript code with @derive decorators to validate',
              },
              filename: {
                type: 'string',
                description: 'Filename for the code (default: input.ts)',
              },
            },
            required: ['code'],
          },
        },
        {
          name: 'expand-code',
          description: `Expands Macroforge macros in TypeScript code and returns the transformed result.

Shows:
- The fully expanded TypeScript code with all generated methods
- Any diagnostics (errors, warnings, info) with line/column locations
- Help text for fixing issues (when available)

Useful for:
- Seeing what code the macros generate
- Understanding how @derive decorators transform your classes
- Debugging macro expansion issues`,
          inputSchema: {
            type: 'object',
            properties: {
              code: {
                type: 'string',
                description: 'TypeScript code with @derive decorators to expand',
              },
              filename: {
                type: 'string',
                description: 'Filename for the code (default: input.ts)',
              },
            },
            required: ['code'],
          },
        },
        {
          name: 'get-macro-info',
          description: `Get documentation for Macroforge macros and decorators.

Returns information about:
- Macro descriptions (e.g., Debug, Serialize, Clone)
- Decorator documentation (e.g., @serde, @debug field decorators)
- Available macro options and configuration

Use without parameters to get the full manifest of all available macros and decorators.
Use with a name parameter to get info for a specific macro or decorator.`,
          inputSchema: {
            type: 'object',
            properties: {
              name: {
                type: 'string',
                description: 'Optional: specific macro or decorator name to look up',
              },
            },
          },
        },
      ],
    };
  });

  // Register tool call handler
  server.setRequestHandler(CallToolRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;

    switch (name) {
      case 'list-sections':
        return handleListSections();

      case 'get-documentation':
        return handleGetDocumentation(args as { section: string | string[] });

      case 'macroforge-autofixer':
        return handleAutofixer(args as { code: string; filename?: string });

      case 'expand-code':
        return handleExpandCode(args as { code: string; filename?: string });

      case 'get-macro-info':
        return handleGetMacroInfo(args as { name?: string });

      default:
        throw new McpError(ErrorCode.MethodNotFound, `Unknown tool: ${name}`);
    }
  });
}

/**
 * Handles the `list-sections` tool call.
 *
 * Returns a formatted list of all available documentation sections, filtering out
 * sub-chunks to show only top-level sections. Each section displays its title,
 * use cases, file path, and category.
 *
 * Sub-chunks (sections with a `parent_id`) are excluded from this list as they
 * are accessed through their parent section via `get-documentation`.
 *
 * @returns MCP response with formatted text listing all sections
 */
function handleListSections() {
  // Filter out sub-chunks (sections with parent_id) - only show top-level sections
  const topLevelSections = sections.filter((s) => !s.parent_id);

  const formatted = topLevelSections
    .map(
      (s) =>
        `* title: [${s.title}], use_cases: [${s.use_cases}], path: [${s.path}], category: [${s.category_title}]`
    )
    .join('\n');

  return {
    content: [
      {
        type: 'text' as const,
        text: `Available Macroforge documentation sections:\n\n${formatted}`,
      },
    ],
  };
}

/**
 * Handles the `get-documentation` tool call.
 *
 * Retrieves full documentation content for one or more sections. Supports flexible
 * lookups by ID, title, or partial match.
 *
 * ## Chunked Section Handling
 *
 * For large documentation sections that are split into chunks:
 * 1. Returns the first chunk's content as the main response
 * 2. Appends a list of additional chunk IDs that can be requested separately
 *
 * This allows clients to progressively load large documentation without
 * overwhelming context windows.
 *
 * @param args - Tool arguments
 * @param args.section - Section name(s) to retrieve (string or array of strings)
 * @returns MCP response with formatted markdown documentation content
 */
function handleGetDocumentation(args: { section: string | string[] }) {
  const sectionNames = Array.isArray(args.section) ? args.section : [args.section];
  const results: string[] = [];

  for (const name of sectionNames) {
    const section = getSection(sections, name);
    if (section) {
      // Check if this is a chunked section
      if (section.is_chunked && section.chunk_ids && section.chunk_ids.length > 0) {
        // Get the first chunk
        const firstChunkId = section.chunk_ids[0];
        const firstChunk = sections.find((s) => s.id === firstChunkId);

        if (firstChunk) {
          let result = `# ${section.title}\n\n${firstChunk.content}`;

          // Add list of other available chunks
          if (section.chunk_ids.length > 1) {
            const otherChunks = section.chunk_ids.slice(1);
            const chunkList = otherChunks
              .map((id) => {
                const chunk = sections.find((s) => s.id === id);
                return chunk ? `- \`${id}\`: ${chunk.title.replace(`${section.title}: `, '')}` : null;
              })
              .filter(Boolean)
              .join('\n');

            result += `\n\n---\n\n**This section has additional chunks available:**\n${chunkList}\n\nRequest specific chunks with \`get-documentation\` for more details.`;
          }

          results.push(result);
        } else {
          results.push(`# ${section.title}\n\nChunked content not found.`);
        }
      } else {
        // Regular section - return content directly
        results.push(`# ${section.title}\n\n${section.content}`);
      }
    } else {
      // Try fuzzy search
      const matches = searchSections(sections, name);
      if (matches.length > 0) {
        const match = matches[0];
        // Handle chunked sections in fuzzy match too
        if (match.is_chunked && match.chunk_ids && match.chunk_ids.length > 0) {
          const firstChunk = sections.find((s) => s.id === match.chunk_ids![0]);
          if (firstChunk) {
            let result = `# ${match.title}\n\n${firstChunk.content}`;
            if (match.chunk_ids.length > 1) {
              const otherChunks = match.chunk_ids.slice(1);
              const chunkList = otherChunks
                .map((id) => {
                  const chunk = sections.find((s) => s.id === id);
                  return chunk ? `- \`${id}\`: ${chunk.title.replace(`${match.title}: `, '')}` : null;
                })
                .filter(Boolean)
                .join('\n');
              result += `\n\n---\n\n**This section has additional chunks available:**\n${chunkList}\n\nRequest specific chunks with \`get-documentation\` for more details.`;
            }
            results.push(result);
          } else {
            results.push(`# ${match.title}\n\n${match.content}`);
          }
        } else {
          results.push(`# ${match.title}\n\n${match.content}`);
        }
      } else {
        results.push(`Documentation for "${name}" not found.`);
      }
    }
  }

  return {
    content: [
      {
        type: 'text' as const,
        text: results.join('\n\n---\n\n'),
      },
    ],
  };
}

/**
 * Handles the `macroforge-autofixer` tool call.
 *
 * Validates TypeScript code containing @derive decorators using Macroforge's
 * native Rust-based analyzer. Returns structured JSON diagnostics that clients
 * can use to provide error messages and fix suggestions.
 *
 * ## Diagnostic Levels
 *
 * - `error` - Invalid code that will fail to compile
 * - `warning` - Code that works but may have issues
 * - `info` - Informational messages about the code
 *
 * ## Response Format
 *
 * Returns JSON with:
 * - `diagnostics` - Array of issues with location, message, and help text
 * - `summary` - Counts of errors, warnings, and info messages
 * - `require_another_tool_call_after_fixing` - True if errors exist (should revalidate)
 *
 * @param args - Tool arguments
 * @param args.code - TypeScript source code to validate
 * @param args.filename - Optional filename for error messages (default: "input.ts")
 * @returns MCP response with JSON-formatted diagnostics
 */
async function handleAutofixer(args: { code: string; filename?: string }) {
  const filename = args.filename || 'input.ts';

  try {
    const macroforge = await importMacroforge();

    if (!macroforge) {
      return {
        content: [
          {
            type: 'text' as const,
            text: JSON.stringify({
              diagnostics: [{
                level: 'error',
                message: 'Native Macroforge bindings not available. Install @macroforge/core.',
              }],
              summary: { errors: 1, warnings: 0, info: 0 },
              require_another_tool_call_after_fixing: false,
            }, null, 2),
          },
        ],
      };
    }

    const result = macroforge.expandSync(args.code, filename, {});
    const diagnostics = result.diagnostics || [];

    const output: AutofixerResult = {
      diagnostics: diagnostics.map((d) => ({
        level: normalizeLevel(d.level),
        message: d.message,
        location: d.span ? { line: d.span.start.line, column: d.span.start.column } : undefined,
        help: d.help || undefined,
        notes: d.notes && d.notes.length > 0 ? d.notes : undefined,
      })),
      summary: {
        errors: diagnostics.filter((d) => d.level === 'Error').length,
        warnings: diagnostics.filter((d) => d.level === 'Warning').length,
        info: diagnostics.filter((d) => d.level === 'Info').length,
      },
      require_another_tool_call_after_fixing: diagnostics.some((d) => d.level === 'Error'),
    };

    return {
      content: [
        {
          type: 'text' as const,
          text: JSON.stringify(output, null, 2),
        },
      ],
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return {
      content: [
        {
          type: 'text' as const,
          text: JSON.stringify({
            diagnostics: [{
              level: 'error',
              message: `Error during analysis: ${message}`,
            }],
            summary: { errors: 1, warnings: 0, info: 0 },
            require_another_tool_call_after_fixing: false,
          }, null, 2),
        },
      ],
    };
  }
}

/**
 * Handles the `expand-code` tool call.
 *
 * Expands Macroforge macros in TypeScript code and returns the transformed result.
 * This is useful for understanding what code the macros generate and debugging
 * macro expansion issues.
 *
 * ## Output Format
 *
 * Returns human-readable markdown with:
 * - The fully expanded TypeScript code in a code block
 * - Any diagnostics (errors, warnings, info) with line/column locations
 * - Help text for fixing issues when available
 *
 * @param args - Tool arguments
 * @param args.code - TypeScript source code with @derive decorators to expand
 * @param args.filename - Optional filename for error messages (default: "input.ts")
 * @returns MCP response with formatted expanded code and diagnostics
 */
async function handleExpandCode(args: { code: string; filename?: string }) {
  const filename = args.filename || 'input.ts';

  try {
    const macroforge = await importMacroforge();

    if (!macroforge) {
      return {
        content: [
          {
            type: 'text' as const,
            text: 'Native Macroforge bindings not available. Install @macroforge/core to enable code expansion.',
          },
        ],
      };
    }

    const result = macroforge.expandSync(args.code, filename, {});
    const diagnostics = result.diagnostics || [];

    // Build structured output
    const output: ExpandResult = {
      expandedCode: result.code,
      diagnostics: diagnostics.map((d) => ({
        level: normalizeLevel(d.level),
        message: d.message,
        location: d.span?.start,
        help: d.help || undefined,
      })),
      hasErrors: diagnostics.some((d) => d.level === 'Error'),
    };

    // Format human-readable text
    let text = `## Expanded Code\n\n\`\`\`typescript\n${result.code}\n\`\`\``;

    if (diagnostics.length > 0) {
      text += '\n\n## Diagnostics\n\n';
      for (const d of diagnostics) {
        const loc = d.span ? ` (line ${d.span.start.line}, col ${d.span.start.column})` : '';
        text += `- **[${normalizeLevel(d.level)}]**${loc} ${d.message}\n`;
        if (d.help) {
          text += `  - Help: ${d.help}\n`;
        }
        if (d.notes && d.notes.length > 0) {
          for (const note of d.notes) {
            text += `  - Note: ${note}\n`;
          }
        }
      }
    }

    return {
      content: [{ type: 'text' as const, text }],
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return {
      content: [
        {
          type: 'text' as const,
          text: `Error expanding code: ${message}`,
        },
      ],
    };
  }
}

/**
 * Handles the `get-macro-info` tool call.
 *
 * Retrieves documentation for Macroforge macros and field decorators from the
 * native manifest. Can return info for a specific macro/decorator or the full
 * manifest of all available macros and decorators.
 *
 * ## Usage Modes
 *
 * - **Without name**: Returns full manifest with all macros and decorators
 * - **With name**: Returns detailed info for the specific macro or decorator
 *
 * ## Manifest Contents
 *
 * - **Macros**: @derive decorators like Debug, Serialize, Clone
 * - **Decorators**: Field decorators like @serde.skip, @serde.rename
 *
 * @param args - Tool arguments
 * @param args.name - Optional macro or decorator name to look up
 * @returns MCP response with formatted macro/decorator documentation
 */
async function handleGetMacroInfo(args: { name?: string }) {
  try {
    const macroforge = await importMacroforge();

    if (!macroforge || !macroforge.__macroforgeGetManifest) {
      return {
        content: [
          {
            type: 'text' as const,
            text: 'Native Macroforge bindings not available. Install @macroforge/core to access macro documentation.',
          },
        ],
      };
    }

    const manifest = macroforge.__macroforgeGetManifest();

    if (args.name) {
      // Look up specific macro or decorator
      const nameLower = args.name.toLowerCase();
      const macro = manifest.macros.find(m => m.name.toLowerCase() === nameLower);
      const decorator = manifest.decorators.find(d => d.export.toLowerCase() === nameLower);

      if (!macro && !decorator) {
        return {
          content: [
            {
              type: 'text' as const,
              text: `No macro or decorator found with name "${args.name}".

Available macros: ${manifest.macros.map(m => m.name).join(', ')}
Available decorators: ${manifest.decorators.map(d => d.export).join(', ')}`,
            },
          ],
        };
      }

      let result = '';

      if (macro) {
        result += `## Macro: @derive(${macro.name})\n\n`;
        result += `**Description:** ${macro.description || 'No description available'}\n`;
        result += `**Kind:** ${macro.kind}\n`;
        result += `**Package:** ${macro.package}\n`;
      }

      if (decorator) {
        if (result) result += '\n---\n\n';
        result += `## Decorator: @${decorator.export}\n\n`;
        result += `**Documentation:** ${decorator.docs || 'No documentation available'}\n`;
        result += `**Kind:** ${decorator.kind}\n`;
        result += `**Module:** ${decorator.module}\n`;
      }

      return {
        content: [{ type: 'text' as const, text: result }],
      };
    }

    // Return full manifest
    let result = '# Macroforge Macro Manifest\n\n';

    result += '## Available Macros\n\n';
    for (const macro of manifest.macros) {
      result += `### @derive(${macro.name})\n`;
      result += `${macro.description || 'No description'}\n\n`;
    }

    if (manifest.decorators.length > 0) {
      result += '## Available Field Decorators\n\n';
      for (const decorator of manifest.decorators) {
        result += `### @${decorator.export}\n`;
        result += `${decorator.docs || 'No documentation'}\n\n`;
      }
    }

    return {
      content: [{ type: 'text' as const, text: result }],
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return {
      content: [
        {
          type: 'text' as const,
          text: `Error getting macro info: ${message}`,
        },
      ],
    };
  }
}

// ============================================================================
// Types - Match Rust's Diagnostic structure from macroforge_ts_syn/src/abi/patch.rs
// ============================================================================

/**
 * Represents a location span in source code.
 *
 * Used by diagnostics to indicate where an error or warning occurred.
 * Matches the Rust `Span` type from the macroforge_ts_syn crate.
 *
 * @property start - Starting position (line and column, 1-indexed)
 * @property end - Ending position (line and column, 1-indexed)
 */
interface DiagnosticSpan {
  start: { line: number; column: number };
  end: { line: number; column: number };
}

/**
 * Represents a diagnostic message from the Macroforge analyzer.
 *
 * Diagnostics are produced during code validation and expansion to report
 * errors, warnings, and informational messages. Matches the Rust `Diagnostic`
 * type from macroforge_ts_syn/src/abi/patch.rs.
 *
 * @property level - Severity level: 'Error', 'Warning', or 'Info'
 * @property message - Human-readable description of the issue
 * @property span - Optional source location where the issue occurred
 * @property notes - Additional context or explanatory notes
 * @property help - Optional suggestion for how to fix the issue
 */
interface Diagnostic {
  level: 'Error' | 'Warning' | 'Info';
  message: string;
  span?: DiagnosticSpan;
  notes: string[];
  help?: string;
}

/**
 * Metadata for a Macroforge macro in the manifest.
 *
 * Describes a @derive macro that can be applied to classes.
 *
 * @property name - Macro name as used in @derive (e.g., "Debug", "Serialize")
 * @property kind - Type of macro (e.g., "derive")
 * @property description - Human-readable description of what the macro does
 * @property package - Package that provides this macro
 */
interface MacroManifestEntry {
  name: string;
  kind: string;
  description: string;
  package: string;
}

/**
 * Metadata for a field decorator in the manifest.
 *
 * Describes a decorator that can be applied to class fields to customize
 * macro behavior (e.g., @serde.skip, @debug.format).
 *
 * @property module - Module path where the decorator is defined
 * @property export - Export name of the decorator
 * @property kind - Type of decorator (e.g., "field")
 * @property docs - Documentation string for the decorator
 */
interface DecoratorManifestEntry {
  module: string;
  export: string;
  kind: string;
  docs: string;
}

/**
 * Complete manifest of available Macroforge macros and decorators.
 *
 * Returned by the native bindings to provide documentation and metadata
 * for all available macros and field decorators.
 *
 * @property version - Manifest format version
 * @property macros - Array of available @derive macros
 * @property decorators - Array of available field decorators
 */
interface MacroManifest {
  version: number;
  macros: MacroManifestEntry[];
  decorators: DecoratorManifestEntry[];
}

/**
 * Interface for the native Macroforge module (@macroforge/core).
 *
 * Defines the expected API surface of the optional native bindings that
 * provide code validation, expansion, and manifest access.
 *
 * @property expandSync - Synchronously expands macros in TypeScript code
 * @property __macroforgeGetManifest - Optional function to retrieve the macro manifest
 */
interface MacroforgeModule {
  /**
   * Synchronously expands Macroforge macros in TypeScript code.
   *
   * @param code - TypeScript source code with @derive decorators
   * @param filename - Filename for error reporting
   * @param options - Expansion options (currently unused)
   * @returns Object with expanded code and any diagnostics
   */
  expandSync: (code: string, filename: string, options: object) => {
    code: string;
    diagnostics?: Diagnostic[];
  };

  /**
   * Retrieves the macro manifest with all available macros and decorators.
   * Optional - may not be available in all versions.
   */
  __macroforgeGetManifest?: () => MacroManifest;
}

// ============================================================================
// Output types for structured responses
// ============================================================================

/**
 * Structured output format for the `macroforge-autofixer` tool.
 *
 * Provides a JSON response that clients can parse to display errors,
 * navigate to problem locations, and determine if re-validation is needed.
 *
 * @property diagnostics - Array of diagnostic messages with locations
 * @property summary - Counts of errors, warnings, and info messages
 * @property require_another_tool_call_after_fixing - True if errors exist and client should revalidate after fixing
 */
interface AutofixerResult {
  diagnostics: Array<{
    /** Severity level: "error", "warning", or "info" */
    level: string;
    /** Human-readable description of the issue */
    message: string;
    /** Source location (line and column) if available */
    location?: { line: number; column: number };
    /** Suggested fix for the issue */
    help?: string;
    /** Additional context or explanatory notes */
    notes?: string[];
  }>;
  /** Summary counts for quick overview */
  summary: {
    errors: number;
    warnings: number;
    info: number;
  };
  /** If true, client should fix issues and call autofixer again */
  require_another_tool_call_after_fixing: boolean;
}

/**
 * Structured output format for the `expand-code` tool.
 *
 * Contains the fully expanded code along with any diagnostics produced
 * during expansion.
 *
 * @property expandedCode - The TypeScript code after macro expansion
 * @property diagnostics - Array of diagnostic messages from expansion
 * @property hasErrors - True if any error-level diagnostics were produced
 */
interface ExpandResult {
  expandedCode: string;
  diagnostics: Array<{
    /** Severity level: "error", "warning", or "info" */
    level: string;
    /** Human-readable description of the issue */
    message: string;
    /** Source location (line and column) if available */
    location?: { line: number; column: number };
    /** Suggested fix for the issue */
    help?: string;
  }>;
  hasErrors: boolean;
}

// ============================================================================
// Helper functions
// ============================================================================

/**
 * Dynamically imports the native Macroforge bindings.
 *
 * The @macroforge/core package is an optional peer dependency that provides
 * native Rust-based code analysis and expansion. This function attempts to
 * load it at runtime and gracefully returns null if unavailable.
 *
 * Using dynamic import allows the MCP server to run and serve documentation
 * even when the native bindings are not installed.
 *
 * @returns The Macroforge module if available, or null if not installed
 *
 * @example
 * ```typescript
 * const macroforge = await importMacroforge();
 * if (macroforge) {
 *   const result = macroforge.expandSync(code, 'input.ts', {});
 * }
 * ```
 */
async function importMacroforge(): Promise<MacroforgeModule | null> {
  try {
    // Dynamic import to avoid build-time errors when @macroforge/core is not installed
    // @ts-expect-error - dynamic import of optional dependency
    const mod = await import('@macroforge/core');
    return mod as MacroforgeModule;
  } catch {
    // Package not installed - return null to indicate unavailability
    return null;
  }
}

/**
 * Normalizes a diagnostic level string to lowercase.
 *
 * The Rust analyzer returns levels as PascalCase ('Error', 'Warning', 'Info'),
 * but JSON output should use lowercase ('error', 'warning', 'info') for
 * consistency with common diagnostic formats.
 *
 * @param level - Diagnostic level from Rust (e.g., 'Error')
 * @returns Lowercase level string (e.g., 'error')
 */
function normalizeLevel(level: string): string {
  return level.toLowerCase();
}
