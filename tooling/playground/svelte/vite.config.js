import { sveltekit } from '@sveltejs/kit/vite';
import { macroforge } from '@macroforge/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [macroforge(), sveltekit()],
    server: {
        // Must agree with playwright.svelte.config.ts, which polls this port.
        port: Number(globalThis.process.env.PLAYGROUND_SVELTE_PORT ?? 5173),
        strictPort: true
    },
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
