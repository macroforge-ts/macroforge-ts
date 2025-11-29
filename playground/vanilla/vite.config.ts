import { defineConfig } from 'vite'
import napiMacrosPlugin from 'vite-plugin-napi'

export default defineConfig({
  plugins: [
    napiMacrosPlugin({
      typesOutputDir: '.ts-macros/types',
      metadataOutputDir: '.ts-macros/meta'
    })
  ],
  server: {
    port: 3000
  }
})
