import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  ErrorCode,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';
import { loadSections, getSection, searchSections, type Section } from './docs-loader.js';

let sections: Section[] = [];

/**
 * Register all Macroforge MCP tools with the server
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
 * Handle list-sections tool call
 * Filters out sub-chunks to show only top-level sections
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
 * Handle get-documentation tool call
 * For chunked sections, returns the first chunk with a list of available sub-chunks
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
 * Handle macroforge-autofixer tool call
 * Returns structured diagnostics from Macroforge's native validation
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
 * Handle expand-code tool call
 * Returns expanded code with structured diagnostics
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
 * Handle get-macro-info tool call
 * Returns documentation for macros and decorators from the manifest
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

interface DiagnosticSpan {
  start: { line: number; column: number };
  end: { line: number; column: number };
}

interface Diagnostic {
  level: 'Error' | 'Warning' | 'Info';
  message: string;
  span?: DiagnosticSpan;
  notes: string[];
  help?: string;
}

interface MacroManifestEntry {
  name: string;
  kind: string;
  description: string;
  package: string;
}

interface DecoratorManifestEntry {
  module: string;
  export: string;
  kind: string;
  docs: string;
}

interface MacroManifest {
  version: number;
  macros: MacroManifestEntry[];
  decorators: DecoratorManifestEntry[];
}

interface MacroforgeModule {
  expandSync: (code: string, filename: string, options: object) => {
    code: string;
    diagnostics?: Diagnostic[];
  };
  __macroforgeGetManifest?: () => MacroManifest;
}

// ============================================================================
// Output types for structured responses
// ============================================================================

interface AutofixerResult {
  diagnostics: Array<{
    level: string;
    message: string;
    location?: { line: number; column: number };
    help?: string;
    notes?: string[];
  }>;
  summary: {
    errors: number;
    warnings: number;
    info: number;
  };
  require_another_tool_call_after_fixing: boolean;
}

interface ExpandResult {
  expandedCode: string;
  diagnostics: Array<{
    level: string;
    message: string;
    location?: { line: number; column: number };
    help?: string;
  }>;
  hasErrors: boolean;
}

// ============================================================================
// Helper functions
// ============================================================================

/**
 * Try to import native Macroforge bindings
 */
async function importMacroforge(): Promise<MacroforgeModule | null> {
  try {
    // Dynamic import to avoid build-time errors
    // @ts-expect-error - dynamic import of optional dependency
    const mod = await import('@macroforge/core');
    return mod as MacroforgeModule;
  } catch {
    return null;
  }
}

/**
 * Normalize diagnostic level to lowercase for output
 */
function normalizeLevel(level: string): string {
  return level.toLowerCase();
}
