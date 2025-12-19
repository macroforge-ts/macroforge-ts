import { highlightCode } from '../shiki-highlighter.js';

let expandSync: typeof import('macroforge').expandSync | null = null;

async function getExpandSync() {
	if (!expandSync) {
		const macroforge = await import('macroforge');
		expandSync = macroforge.expandSync;
	}
	return expandSync;
}

export interface ExpandedExample {
	before: string;
	after: string;
	beforeHtml: string;
	afterHtml: string;
}

/**
 * Expands macro code at build time and returns both before/after
 * Strips the import statement from the output for cleaner display
 */
export async function expandExample(code: string, filename = 'example.ts'): Promise<ExpandedExample> {
	const expand = await getExpandSync();
	const result = expand(code, filename);

	// Clean up the expanded code for display
	let after = result.code;

	// Remove the macroforge import line if present
	after = after.replace(/^import\s+\{[^}]+\}\s+from\s+['"]macroforge['"];\s*\n?/m, '');

	// Pre-highlight with Shiki
	const [beforeHtml, afterHtml] = await Promise.all([
		highlightCode(code.trim()),
		highlightCode(after.trim())
	]);

	return {
		before: code.trim(),
		after: after.trim(),
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
