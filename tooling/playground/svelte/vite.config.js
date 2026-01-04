import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// Runtime imports from env variables (no fallbacks - must be set)
const {
    VITE_PLUGIN_PKG,
    MACROFORGE_TS_CRATE,
    PLAYGROUND_MACRO,
    SHARED_PKG,
    SVELTE_PREPROCESSOR_PKG,
    TYPESCRIPT_PLUGIN_PKG
} = process.env;

const { macroforge } = await import(`${VITE_PLUGIN_PKG}/src/index.js`);

export default defineConfig({
    plugins: [macroforge(), sveltekit()],
    ssr: {
        noExternal: ['effect', '@playground/macro']
    },
    optimizeDeps: {
        exclude: ['@playground/macro']
    },
    resolve: {
        dedupe: ['effect'],
        alias: [
            // macroforge subpaths (explicit mappings to match package.json exports)
            {
                find: 'macroforge/serde',
                replacement: `${MACROFORGE_TS_CRATE}/js/serde/index.mjs`
            },
            {
                find: 'macroforge/traits',
                replacement: `${MACROFORGE_TS_CRATE}/js/traits/index.mjs`
            },
            {
                find: 'macroforge/reexports/effect',
                replacement: `${MACROFORGE_TS_CRATE}/js/reexports/effect.mjs`
            },
            {
                find: 'macroforge/reexports',
                replacement: `${MACROFORGE_TS_CRATE}/js/reexports/index.mjs`
            },
            { find: 'macroforge', replacement: MACROFORGE_TS_CRATE },
            { find: '@playground/macro', replacement: PLAYGROUND_MACRO },
            { find: '@macroforge/vite-plugin', replacement: VITE_PLUGIN_PKG },
            { find: '@macroforge/shared', replacement: SHARED_PKG },
            {
                find: '@macroforge/svelte-preprocessor',
                replacement: SVELTE_PREPROCESSOR_PKG
            },
            {
                find: '@macroforge/typescript-plugin',
                replacement: TYPESCRIPT_PLUGIN_PKG
            }
        ]
    }
});
