import { defineConfig } from 'vite'
import macroforge from 'vite-plugin-macroforge'

export default defineConfig({
  plugins: [
    macroforge({
      typesOutputDir: '.macroforge/types',
      metadataOutputDir: '.macroforge/meta'
    })
  ],
  server: {
    port: 3000
  }
})
