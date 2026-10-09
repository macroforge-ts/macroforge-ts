import { sveltekit } from '@sveltejs/kit/vite';
import { macroforge } from '@macroforge/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [macroforge(), sveltekit()],
    server: {
        // Must agree with playwright.svelte.config.ts, which polls this port.
        port: Number(globalThis.process.env.TESTGROUND_SVELTE_PORT ?? 5173),
        strictPort: true
    },
    // The e2e suite drives the built app, so `preview` reads the same port.
    preview: {
        port: Number(globalThis.process.env.TESTGROUND_SVELTE_PORT ?? 5173),
        strictPort: true
    },
    ssr: {
        noExternal: ['effect', '@testground/macro']
    },
    optimizeDeps: {
        exclude: ['@testground/macro']
    },
    resolve: {
        dedupe: ['effect']
    }
});
