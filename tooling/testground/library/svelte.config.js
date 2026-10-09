import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/vite-plugin-svelte').SvelteConfig} */
const config = {
    preprocess: [
        macroforgePreprocess(), // Expand macros FIRST
        vitePreprocess() // Then handle TypeScript/CSS
    ]
};

export default config;
