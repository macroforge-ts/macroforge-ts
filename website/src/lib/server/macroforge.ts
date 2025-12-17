import { expandSync } from 'macroforge';
import { execSync } from 'child_process';
import { highlightCode } from '../shiki-highlighter.js';

export interface ExpandedExample {
	before: string;
	after: string;
	beforeHtml: string;
	afterHtml: string;
}

/**
 * Format TypeScript code using Biome CLI
 * Fails during production builds to catch code generation bugs early
 */
function formatCode(code: string, debugName?: string): string {
	const isProduction = process.env.NODE_ENV === 'production';

	try {
		const result = execSync('npx @biomejs/biome format --stdin-file-path=example.ts', {
			input: code,
			encoding: 'utf-8',
			maxBuffer: 10 * 1024 * 1024,
			stdio: ['pipe', 'pipe', 'pipe'] // Suppress stderr output
		});
		return result.trim();
	} catch (error) {
		const snippet = code.slice(0, 500).replace(/\n/g, '\\n');
		const errorMessage = error instanceof Error ? error.message : String(error);

		if (isProduction) {
			// Fail build in production to catch code generation bugs
			console.error(`[macroforge] Biome format failed for "${debugName || 'unknown'}"`);
			console.error(`  Code snippet: ${snippet}...`);
			console.error(`  Error: ${errorMessage}`);
			throw new Error(
				`Build failed: Generated code could not be formatted. ` +
					`This usually indicates a bug in macro expansion. ` +
					`File: ${debugName || 'unknown'}`
			);
		}

		// In development, log warning and continue
		if (debugName) {
			console.warn(`[macroforge] Biome format failed for "${debugName}"`);
			console.warn(`  Code snippet: ${snippet}...`);
		}
		return code.trim();
	}
}

/**
 * Expands macro code at build time and returns both before/after
 * Strips the import statement from the output for cleaner display
 */
export async function expandExample(code: string, filename = 'example.ts'): Promise<ExpandedExample> {
	const result = expandSync(code, filename);

	// Clean up the expanded code for display
	let after = result.code;

	// Remove the macroforge import line if present
	after = after.replace(/^import\s+\{[^}]+\}\s+from\s+['"]macroforge['"];\s*\n?/m, '');

	// Format the output with Biome
	const beforeFormatted = formatCode(code.trim(), `${filename}:before`);
	const afterFormatted = formatCode(after, `${filename}:after`);

	// Pre-highlight with Shiki
	const [beforeHtml, afterHtml] = await Promise.all([
		highlightCode(beforeFormatted),
		highlightCode(afterFormatted)
	]);

	return {
		before: beforeFormatted,
		after: afterFormatted,
		beforeHtml,
		afterHtml
	};
}

/**
 * Expand multiple examples at once
 */
export async function expandExamples(
	examples: Record<string, string>
): Promise<Record<string, ExpandedExample>> {
	const result: Record<string, ExpandedExample> = {};
	const entries = Object.entries(examples);
	const expanded = await Promise.all(
		entries.map(([key, code]) => expandExample(code, `${key}.ts`))
	);
	for (let i = 0; i < entries.length; i++) {
		result[entries[i][0]] = expanded[i];
	}
	return result;
}
