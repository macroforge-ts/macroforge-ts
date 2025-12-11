import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { expandSync } from 'macroforge';
import { execSync } from 'child_process';

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

		return json({
			code: expanded,
			diagnostics: result.diagnostics || []
		});
	} catch (error) {
		return json(
			{ error: error instanceof Error ? error.message : 'Expansion failed' },
			{ status: 500 }
		);
	}
};
