import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import macroforge from '@macroforge/vite-plugin';

export default defineConfig({
	plugins: [
		macroforge(),
		sveltekit(),
		tailwindcss(),
		svelteTesting(),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide',
			strategy: ['url', 'cookie', 'baseLocale']
		})
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'jsdom',
		globals: true,
		setupFiles: ['./vitest.setup.ts'],
		// Exclude demo/example files from tests
		exclude: [
			'**/node_modules/**',
			'**/.svelte-kit/**',
			'**/*.expanded.ts',
			'**/demo/**'
		]
	}
});
