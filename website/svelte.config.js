import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import remarkGfm from 'remark-gfm';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', '.svx'],

	preprocess: [
		vitePreprocess(),
		mdsvex({
			extensions: ['.svx'],
			remarkPlugins: [remarkGfm]
		})
	],

	kit: {
		adapter: adapter({
			out: 'build'
		})
	}
};

export default config;
