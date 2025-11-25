import { Plugin } from 'vite'
import { createRequire } from 'module'
import * as fs from 'fs'
import * as path from 'path'

export interface NapiMacrosPluginOptions {
  include?: string | RegExp | (string | RegExp)[]
  exclude?: string | RegExp | (string | RegExp)[]
  generateTypes?: boolean // Enable type generation (default: true)
  typesOutputDir?: string // Where to output generated types
  emitMetadata?: boolean // Write macro IR metadata (default: true)
  metadataOutputDir?: string // Where to output metadata JSON (defaults to types dir)
}

function napiMacrosPlugin(options: NapiMacrosPluginOptions = {}): Plugin {
  let rustTransformer: any
  let projectRoot: string
  const generateTypes = options.generateTypes !== false // Default to true
  const typesOutputDir = options.typesOutputDir || 'src/macros/generated'
  const emitMetadata = options.emitMetadata !== false
  const metadataOutputDir = options.metadataOutputDir || typesOutputDir

  // Ensure directory exists
  function ensureDir(dir: string) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true })
    }
  }

  function writeTypeDefinitions(id: string, types: string) {
    const relativePath = path.relative(projectRoot, id)
    const parsed = path.parse(relativePath)
    const outputBase = path.join(projectRoot, typesOutputDir, parsed.dir)
    ensureDir(outputBase)
    const targetPath = path.join(outputBase, `${parsed.name}.d.ts`)

    try {
      const existing = fs.existsSync(targetPath) ? fs.readFileSync(targetPath, 'utf-8') : null
      if (existing !== types) {
        fs.writeFileSync(targetPath, types, 'utf-8')
        console.log(
          `[vite-plugin-napi-macros] Wrote types for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`
        )
      }
    } catch (error) {
      console.error(`[vite-plugin-napi-macros] Failed to write type definitions for ${id}:`, error)
    }
  }

  function writeMetadata(id: string, metadata: string) {
    const relativePath = path.relative(projectRoot, id)
    const parsed = path.parse(relativePath)
    const outputBase = path.join(projectRoot, metadataOutputDir, parsed.dir)
    ensureDir(outputBase)
    const targetPath = path.join(outputBase, `${parsed.name}.macro-ir.json`)

    try {
      const existing = fs.existsSync(targetPath) ? fs.readFileSync(targetPath, 'utf-8') : null
      if (existing !== metadata) {
        fs.writeFileSync(targetPath, metadata, 'utf-8')
        console.log(
          `[vite-plugin-napi-macros] Wrote metadata for ${relativePath} -> ${path.relative(projectRoot, targetPath)}`
        )
      }
    } catch (error) {
      console.error(`[vite-plugin-napi-macros] Failed to write metadata for ${id}:`, error)
    }
  }

  function formatTransformError(error: unknown, id: string): string {
    const relative = projectRoot ? path.relative(projectRoot, id) || id : id
    if (error instanceof Error) {
      const details = error.stack && error.stack.includes(error.message) ? error.stack : `${error.message}\n${error.stack ?? ''}`
      return `[vite-plugin-napi-macros] Failed to transform ${relative}\n${details}`.trim()
    }
    return `[vite-plugin-napi-macros] Failed to transform ${relative}: ${String(error)}`
  }

  return {
    name: 'vite-plugin-napi-macros',

    enforce: 'pre',

    configResolved(config) {
      projectRoot = config.root

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

    transform(this, code: string, id: string) {
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
        const result: { code: string; map?: string; types?: string; metadata?: string } = rustTransformer.transformSync(code, id)

        if (result && result.code) {
          if (generateTypes && result.types) {
            writeTypeDefinitions(id, result.types)
          }
          if (emitMetadata && result.metadata) {
            writeMetadata(id, result.metadata)
          }
          return {
            code: result.code,
            map: result.map || null
          }
        }
      } catch (error) {
        const message = formatTransformError(error, id)
        this.error(message)
      }

      return null
    }
  }
}

export default napiMacrosPlugin
