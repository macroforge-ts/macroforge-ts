import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { macroforge } from '@macroforge/vite-plugin';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));

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
    // The e2e suite drives the built app, so `preview` reads the same port.
    preview: {
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
        dedupe: ['effect']
    }
});
