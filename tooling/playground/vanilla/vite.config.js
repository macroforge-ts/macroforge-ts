import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Runtime imports from env variables (no fallbacks - must be set)
const { VITE_PLUGIN_PKG, MACROFORGE_TS_CRATE, PLAYGROUND_MACRO, SHARED_PKG } =
    globalThis.process.env;

const { macroforge } = await import(`${VITE_PLUGIN_PKG}/src/index.js`);

export default defineConfig({
    plugins: [macroforge()],
    server: {
        // Env-driven so a run can move off a port something else already holds.
        // `strictPort` makes that collision an error instead of a silent shift
        // to the next free port, which would leave the e2e suite polling an
        // address this server never bound.
        port: Number(globalThis.process.env.PLAYGROUND_VANILLA_PORT ?? 3000),
        strictPort: true
    },
    build: {
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'index.html'),
                'validator-form': resolve(__dirname, 'validator-form.html')
            }
        }
    },
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
                find: '@macroforge/core/serde',
                replacement: `${MACROFORGE_TS_CRATE}/js/serde/index.mjs`
            },
            {
                find: '@macroforge/core/traits',
                replacement: `${MACROFORGE_TS_CRATE}/js/traits/index.mjs`
            },
            {
                find: '@macroforge/core/rules',
                replacement: `${MACROFORGE_TS_CRATE}/js/rules/index.mjs`
            },
            {
                find: '@macroforge/core/buildtime',
                replacement: `${MACROFORGE_TS_CRATE}/js/buildtime/index.mjs`
            },
            { find: '@macroforge/core', replacement: MACROFORGE_TS_CRATE },
            { find: '@playground/macro', replacement: PLAYGROUND_MACRO },
            { find: '@macroforge/vite-plugin', replacement: VITE_PLUGIN_PKG },
            { find: '@macroforge/shared', replacement: SHARED_PKG }
        ]
    }
});
