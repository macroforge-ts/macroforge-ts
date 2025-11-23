import { defineConfig } from 'vite'
import napiMacrosPlugin from 'vite-plugin-napi'

export default defineConfig({
  plugins: [napiMacrosPlugin()],
  server: {
    port: 3000
  }
})