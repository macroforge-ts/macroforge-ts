import { Plugin } from 'vite'
import { createRequire } from 'module'
import path from 'path'

export interface NapiMacrosPluginOptions {
  include?: string | RegExp | (string | RegExp)[]
  exclude?: string | RegExp | (string | RegExp)[]
}

function napiMacrosPlugin(options: NapiMacrosPluginOptions = {}): Plugin {
  let rustTransformer: any

  return {
    name: 'vite-plugin-napi-macros',

    enforce: 'pre',

    configResolved() {
      // Load the Rust binary
      try {
        const require = createRequire(import.meta.url)
        // This will load the compiled .node binary
        rustTransformer = require('@ts-macros/swc-napi')
      } catch (error) {
        console.warn('[vite-plugin-napi-macros] Rust binary not found. Please run `npm run build:rust` first.')
        console.warn(error)
      }
    },

    transform(code: string, id: string) {
      // Only transform TypeScript files
      if (!id.endsWith('.ts') && !id.endsWith('.tsx')) {
        return null
      }

      // Skip node_modules by default
      if (id.includes('node_modules')) {
        return null
      }

      // Check if Rust transformer is available
      if (!rustTransformer || !rustTransformer.transformSync) {
        // Return unchanged if transformer not available
        return null
      }

      try {
        // Call the Rust transformer
        const result = rustTransformer.transformSync(code, id)

        if (result && result.code) {
          return {
            code: result.code,
            map: result.map || null
          }
        }
      } catch (error) {
        console.error(`[vite-plugin-napi-macros] Transform error in ${id}:`, error)
        // Return unchanged on error
        return null
      }

      return null
    }
  }
}

export default napiMacrosPlugin