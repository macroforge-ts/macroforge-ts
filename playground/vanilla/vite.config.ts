import { defineConfig } from 'vite'
import napiMacrosPlugin from 'vite-plugin-napi'

export default defineConfig({
  plugins: [
    napiMacrosPlugin({
      typesOutputDir: '.macroforge/types',
      metadataOutputDir: '.macroforge/meta'
    })
  ],
  server: {
    port: 3000
  }
})
