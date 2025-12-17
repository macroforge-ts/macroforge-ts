import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { expandSync } from 'macroforge';
import { execSync } from 'child_process';
import { highlightCode } from '$lib/shiki-highlighter.js';

function formatCode(code: string): string {
	try {
		const result = execSync('npx @biomejs/biome format --stdin-file-path=example.ts', {
			input: code,
			encoding: 'utf-8',
			maxBuffer: 10 * 1024 * 1024
		});
		return result.trim();
	} catch {
		return code.trim();
	}
}

export const POST: RequestHandler = async ({ request }) => {
	try {
		const { code } = await request.json();

		if (!code || typeof code !== 'string') {
			return json({ error: 'Code is required' }, { status: 400 });
		}

		const result = expandSync(code, 'example.ts');

		// Clean up the expanded code
		let expanded = result.code;
		expanded = expanded.replace(/^import\s+\{[^}]+\}\s+from\s+['"]macroforge['"];\s*\n?/m, '');
		expanded = formatCode(expanded);

		// Highlight both source and expanded code
		const [codeHtml, expandedHtml] = await Promise.all([
			highlightCode(code),
			highlightCode(expanded)
		]);

		return json({
			code: expanded,
			codeHtml,
			expandedHtml,
			diagnostics: result.diagnostics || []
		});
	} catch (error) {
		return json(
			{ error: error instanceof Error ? error.message : 'Expansion failed' },
			{ status: 500 }
		);
	}
};
