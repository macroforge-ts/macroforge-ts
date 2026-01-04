import { macroforge } from '../../../packages/vite-plugin/src/index.js';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [macroforge(), sveltekit()],
    ssr: {
        noExternal: ['effect', '@playground/macro']
    },
    optimizeDeps: {
        exclude: ['@playground/macro']
    },
    resolve: {
        dedupe: ['effect']
    }
});
