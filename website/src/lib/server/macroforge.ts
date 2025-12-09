import { expandSync } from 'macroforge';

export interface ExpandedExample {
	before: string;
	after: string;
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

	// Trim any leading/trailing whitespace
	after = after.trim();

	return {
		before: code.trim(),
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
