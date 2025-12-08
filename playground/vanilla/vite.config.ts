import { defineConfig } from 'vite'
import macroforge from '@macroforge/vite-plugin'
import { resolve } from 'path'

export default defineConfig({
  plugins: [
    macroforge({
      typesOutputDir: '.macroforge/types',
      metadataOutputDir: '.macroforge/meta'
    })
  ],
  server: {
    port: 3000
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        'validator-form': resolve(__dirname, 'validator-form.html'),
      },
    },
  },
})
