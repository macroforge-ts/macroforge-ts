import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  ErrorCode,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';
import { loadSections, getSection, getSections, searchSections, type Section } from './docs-loader.js';

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
          description:
            'Lists all available Macroforge documentation sections in a structured format. Each section includes a "use_cases" field describing when the documentation is useful. Use this tool FIRST to discover relevant documentation for any Macroforge-related task.',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'get-documentation',
          description:
            'Retrieves full documentation content for Macroforge sections. Supports flexible search by title (e.g., "Debug", "Vite Plugin") or ID (e.g., "debug", "vite-plugin"). Can accept a single section name or an array of sections. After calling list-sections, analyze the use_cases and fetch ALL relevant sections.',
          inputSchema: {
            type: 'object',
            properties: {
              section: {
                anyOf: [
                  { type: 'string' },
                  { type: 'array', items: { type: 'string' } },
                ],
                description:
                  'The section name(s) to retrieve. Can search by title or ID. Supports single string or array of strings.',
              },
            },
            required: ['section'],
          },
        },
        {
          name: 'macroforge-autofixer',
          description:
            'Analyzes TypeScript code with @derive decorators and returns suggestions for issues. Uses the native Macroforge expansion to validate code and detect problems like invalid macro names, malformed decorators, or missing validators. Call this tool when writing Macroforge code to ensure correctness.',
          inputSchema: {
            type: 'object',
            properties: {
              code: {
                type: 'string',
                description: 'The TypeScript code to analyze',
              },
              filename: {
                type: 'string',
                description: 'Optional filename for the code (defaults to "input.ts")',
              },
            },
            required: ['code'],
          },
        },
        {
          name: 'expand-code',
          description:
            'Expands Macroforge macros in TypeScript code and returns the transformed result. Useful for seeing what code the macros generate.',
          inputSchema: {
            type: 'object',
            properties: {
              code: {
                type: 'string',
                description: 'The TypeScript code with @derive decorators to expand',
              },
              filename: {
                type: 'string',
                description: 'Optional filename for the code (defaults to "input.ts")',
              },
            },
            required: ['code'],
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

      default:
        throw new McpError(ErrorCode.MethodNotFound, `Unknown tool: ${name}`);
    }
  });
}

/**
 * Handle list-sections tool call
 */
function handleListSections() {
  const formatted = sections
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
 */
function handleGetDocumentation(args: { section: string | string[] }) {
  const sectionNames = Array.isArray(args.section) ? args.section : [args.section];
  const results: string[] = [];

  for (const name of sectionNames) {
    const section = getSection(sections, name);
    if (section) {
      results.push(`# ${section.title}\n\n${section.content}`);
    } else {
      // Try fuzzy search
      const matches = searchSections(sections, name);
      if (matches.length > 0) {
        results.push(`# ${matches[0].title}\n\n${matches[0].content}`);
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
 */
async function handleAutofixer(args: { code: string; filename?: string }) {
  const filename = args.filename || 'input.ts';
  const suggestions: string[] = [];

  try {
    // Try to import native bindings
    const macroforge = await importMacroforge();

    if (macroforge) {
      const result = macroforge.expandSync(args.code, filename, {});

      // Check for diagnostics
      if (result.diagnostics && result.diagnostics.length > 0) {
        for (const diag of result.diagnostics) {
          suggestions.push(`[${diag.level}] ${diag.message}`);
        }
      } else {
        suggestions.push('No issues found. Code validates successfully.');
      }
    } else {
      // Fallback: basic static analysis without native bindings
      suggestions.push(...analyzeCodeStatically(args.code));
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    suggestions.push(`Error during analysis: ${message}`);

    // Still try static analysis
    suggestions.push(...analyzeCodeStatically(args.code));
  }

  return {
    content: [
      {
        type: 'text' as const,
        text: suggestions.length > 0 ? suggestions.join('\n') : 'No issues found.',
      },
    ],
  };
}

/**
 * Handle expand-code tool call
 */
async function handleExpandCode(args: { code: string; filename?: string }) {
  const filename = args.filename || 'input.ts';

  try {
    const macroforge = await importMacroforge();

    if (macroforge) {
      const result = macroforge.expandSync(args.code, filename, {});

      let output = `## Expanded Code\n\n\`\`\`typescript\n${result.code}\n\`\`\``;

      if (result.diagnostics && result.diagnostics.length > 0) {
        output += '\n\n## Diagnostics\n\n';
        for (const diag of result.diagnostics) {
          output += `- [${diag.level}] ${diag.message}\n`;
        }
      }

      return {
        content: [{ type: 'text' as const, text: output }],
      };
    } else {
      return {
        content: [
          {
            type: 'text' as const,
            text: 'Native Macroforge bindings not available. Install @macroforge/core to enable code expansion.',
          },
        ],
      };
    }
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

interface MacroforgeModule {
  expandSync: (code: string, filename: string, options: object) => {
    code: string;
    diagnostics?: Array<{ level: string; message: string }>;
  };
}

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
 * Basic static analysis fallback when native bindings aren't available
 */
function analyzeCodeStatically(code: string): string[] {
  const suggestions: string[] = [];

  // Check for @derive decorator
  const deriveMatches = code.match(/@derive\(([^)]+)\)/g);
  if (deriveMatches) {
    const validMacros = ['Debug', 'Clone', 'Default', 'Hash', 'Ord', 'PartialEq', 'PartialOrd', 'Serialize', 'Deserialize'];

    for (const match of deriveMatches) {
      const macros = match.replace('@derive(', '').replace(')', '').split(',').map(m => m.trim());

      for (const macro of macros) {
        if (!validMacros.includes(macro)) {
          suggestions.push(`[warning] Unknown macro "${macro}". Valid macros are: ${validMacros.join(', ')}`);
        }
      }
    }
  }

  // Check for common issues
  if (code.includes('/** @derive') && !code.includes('*/')) {
    suggestions.push('[error] Unclosed JSDoc comment for @derive decorator');
  }

  if (code.includes('@serde(') && !code.includes('Serialize') && !code.includes('Deserialize')) {
    suggestions.push('[warning] @serde field options used but Serialize/Deserialize macro not applied');
  }

  if (suggestions.length === 0) {
    suggestions.push('[info] Static analysis found no obvious issues. For full validation, ensure @macroforge/core is installed.');
  }

  return suggestions;
}
