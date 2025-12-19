import { resolve } from 'node:path';
import macroforge from '@macroforge/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [macroforge()],
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
        dedupe: ['effect']
    }
});
