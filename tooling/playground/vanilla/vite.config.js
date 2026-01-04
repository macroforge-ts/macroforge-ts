import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { deno } from '@deno-plc/vite-plugin-deno';
import macroforge from '../../../packages/vite-plugin/src/index.js';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
    plugins: [deno(), macroforge()],
    server: {
        port: 3000
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
    resolve: {
        dedupe: ['effect'],
        alias: {
            'macroforge/serde': resolve(__dirname, '../../../crates/macroforge_ts/js/serde/index.mjs'),
            'macroforge/traits': resolve(__dirname, '../../../crates/macroforge_ts/js/traits/index.mjs'),
            'macroforge/reexports': resolve(__dirname, '../../../crates/macroforge_ts/js/reexports/index.mjs'),
            'macroforge/reexports/effect': resolve(__dirname, '../../../crates/macroforge_ts/js/reexports/effect.mjs'),
            'macroforge': resolve(__dirname, '../../../crates/macroforge_ts')
        }
    }
});
