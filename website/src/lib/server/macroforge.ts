import { expandSync } from 'macroforge';
import { execSync } from 'child_process';

export interface ExpandedExample {
	before: string;
	after: string;
}

/**
 * Format TypeScript code using Biome CLI
 */
function formatCode(code: string): string {
	try {
		const result = execSync('npx biome format --stdin-file-path=example.ts', {
			input: code,
			encoding: 'utf-8',
			maxBuffer: 10 * 1024 * 1024
		});
		return result.trim();
	} catch {
		return code.trim();
	}
}

/**
 * Expands macro code at build time and returns both before/after
 * Strips the import statement from the output for cleaner display
 */
export function expandExample(code: string, filename = 'example.ts'): ExpandedExample {
	const result = expandSync(code, filename);

	// Clean up the expanded code for display
	let after = result.code;

	// Remove the macroforge import line if present
	after = after.replace(/^import\s+\{[^}]+\}\s+from\s+['"]macroforge['"];\s*\n?/m, '');

	// Format the output with Biome
	after = formatCode(after);

	return {
		before: formatCode(code.trim()),
		after
	};
}

/**
 * Expand multiple examples at once
 */
export function expandExamples(
	examples: Record<string, string>
): Record<string, ExpandedExample> {
	const result: Record<string, ExpandedExample> = {};
	for (const [key, code] of Object.entries(examples)) {
		result[key] = expandExample(code, `${key}.ts`);
	}
	return result;
}
