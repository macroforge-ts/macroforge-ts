import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import remarkGfm from 'remark-gfm';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { createHighlighter } from './src/lib/shiki-highlighter.js';
import remarkMacroBlocks from './src/lib/remark-macro-blocks.js';
import { codeBlockPreprocess } from './src/lib/codeblock-preprocessor.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Create the Shiki highlighter (async initialization)
const highlighter = await createHighlighter();

/** @type {import('@sveltejs/kit').Config} */
const config = {
    extensions: ['.svelte', '.svx'],

    preprocess: [
        codeBlockPreprocess(),
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
            pages: 'build',
            assets: 'build',
            fallback: undefined,
            precompress: false,
            strict: true
        }),
        prerender: {
            entries: ['*']
        }
    }
};

export default config;
