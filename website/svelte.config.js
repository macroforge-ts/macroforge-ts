import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import remarkGfm from 'remark-gfm';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { createHighlighter } from './src/lib/shiki-highlighter.js';
import remarkMacroBlocks from './src/lib/remark-macro-blocks.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Create the Shiki highlighter (async initialization)
const highlighter = await createHighlighter();

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', '.svx'],

	preprocess: [
		vitePreprocess(),
		mdsvex({
			extensions: ['.svx'],
			layout: join(__dirname, 'src/lib/mdsvex-layout.svelte'),
			remarkPlugins: [remarkGfm, remarkMacroBlocks],
			highlight: {
				highlighter
			}
		})
	],

	kit: {
		adapter: adapter({
			out: 'build'
		})
	}
};

export default config;
