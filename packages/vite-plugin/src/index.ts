import { Plugin } from 'vite'
import { createRequire } from 'module'
import * as fs from 'fs'
import * as path from 'path'
import type ts from 'typescript'
import { ExpandResult } from '@ts-macros/swc-napi'

const moduleRequire = createRequire(import.meta.url)
let tsModule: typeof ts | undefined
try {
  tsModule = moduleRequire('typescript') as typeof ts
} catch (error) {
  tsModule = undefined
  console.warn('[vite-plugin-napi-macros] TypeScript not found. Generated .d.ts files will be skipped.')
}

const compilerOptionsCache = new Map<string, ts.CompilerOptions>()

export interface NapiMacrosPluginOptions {
  include?: string | RegExp | (string | RegExp)[]
  exclude?: string | RegExp | (string | RegExp)[]
  generateTypes?: boolean // Enable type generation (default: true)
  typesOutputDir?: string // Where to output generated types
  emitMetadata?: boolean // Write macro IR metadata (default: true)
  metadataOutputDir?: string // Where to output metadata JSON (defaults to types dir)
}

function getCompilerOptions(projectRoot: string): ts.CompilerOptions | undefined {
  if (!tsModule) {
    return undefined
  }
  const cached = compilerOptionsCache.get(projectRoot)
  if (cached) {
    return cached
  }

  let configPath: string | undefined
  try {
    configPath = tsModule.findConfigFile(projectRoot, tsModule.sys.fileExists, 'tsconfig.json')
  } catch {
    configPath = undefined
  }

  let options: ts.CompilerOptions
  if (configPath) {
    const configFile = tsModule.readConfigFile(configPath, tsModule.sys.readFile)
    if (configFile.error) {
      const formatted = tsModule.formatDiagnosticsWithColorAndContext([configFile.error], {
        getCurrentDirectory: () => projectRoot,
        getCanonicalFileName: (fileName) => fileName,
        getNewLine: () => tsModule.sys.newLine
      })
      console.warn(`[vite-plugin-napi-macros] Failed to read tsconfig at ${configPath}\n${formatted}`)
      options = {}
    } else {
      const parsed = tsModule.parseJsonConfigFileContent(
        configFile.config,
        tsModule.sys,
        path.dirname(configPath)
      )
      options = parsed.options
    }
  } else {
    options = {}
  }

  const normalized: ts.CompilerOptions = {
    ...options,
    declaration: true,
    emitDeclarationOnly: true,
    noEmitOnError: false,
    incremental: false
  }

  delete normalized.outDir
  delete normalized.outFile
  normalized.moduleResolution ??= tsModule.ModuleResolutionKind.Bundler
  normalized.module ??= tsModule.ModuleKind.ESNext
  normalized.target ??= tsModule.ScriptTarget.ESNext
  normalized.strict ??= true
  normalized.skipLibCheck ??= true

  compilerOptionsCache.set(projectRoot, normalized)
  return normalized
}

function emitDeclarationsFromCode(code: string, fileName: string, projectRoot: string): string | undefined {
  if (!tsModule) {
    return undefined
  }

  const compilerOptions = getCompilerOptions(projectRoot)
  if (!compilerOptions) {
    return undefined
  }

  const normalizedFileName = path.resolve(fileName)
  const sourceText = code
  const compilerHost = tsModule.createCompilerHost(compilerOptions, true)

  compilerHost.getSourceFile = (requestedFileName, languageVersion) => {
    if (path.resolve(requestedFileName) === normalizedFileName) {
      return tsModule!.createSourceFile(requestedFileName, sourceText, languageVersion, true)
    }
    const text = tsModule!.sys.readFile(requestedFileName)
    return text !== undefined
      ? tsModule!.createSourceFile(requestedFileName, text, languageVersion, true)
      : undefined
  }

  compilerHost.readFile = (requestedFileName) => {
    return path.resolve(requestedFileName) === normalizedFileName
      ? sourceText
      : tsModule!.sys.readFile(requestedFileName)
  }

  compilerHost.fileExists = (requestedFileName) => {
    return (
      path.resolve(requestedFileName) === normalizedFileName || tsModule!.sys.fileExists(requestedFileName)
    )
  }

  let output: string | undefined
  const writeFile = (outputName: string, text: string) => {
    if (outputName.endsWith('.d.ts')) {
      output = text
    }
  }

  const program = tsModule.createProgram([normalizedFileName], compilerOptions, compilerHost)
  const emitResult = program.emit(undefined, writeFile, undefined, true)

  if (emitResult.emitSkipped && emitResult.diagnostics.length > 0) {
    const formatted = tsModule.formatDiagnosticsWithColorAndContext(emitResult.diagnostics, {
      getCurrentDirectory: () => projectRoot,
      getCanonicalFileName: (fileName) => fileName,
      getNewLine: () => tsModule.sys.newLine
    })
    console.warn(
      `[vite-plugin-napi-macros] Declaration emit failed for ${path.relative(
        projectRoot,
        fileName
      )}\n${formatted}`
    )
    return undefined
  }

  return output
}

function napiMacrosPlugin(options: NapiMacrosPluginOptions = {}): Plugin {
  let rustTransformer: {
    expandSync: (code: string, filepath: string) => ExpandResult
  } | undefined
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
        rustTransformer = moduleRequire('@ts-macros/swc-napi')
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
      if (!rustTransformer || !rustTransformer.expandSync) {
        // Return unchanged if transformer not available
        return null
      }

      try {
        const result: ExpandResult = rustTransformer.expandSync(code, id)

        // Report diagnostics
        for (const diag of result.diagnostics) {
          if (diag.level === 'error') {
            this.error({
              message: `Macro error at ${id}:${diag.start}-${diag.end}: ${diag.message}`,
              id: id,
              loc: diag.start !== undefined && diag.end !== undefined
                ? {
                    file: id,
                    line: 0, // Vite expects line numbers. MacroDiagnostics are byte offsets.
                    column: diag.start // Need to map byte offset to line/column. For now, just use start.
                  }
                : undefined,
              // Other error properties can be added if needed
            })
          } else {
            // Log warnings and info messages
            console.warn(`[vite-plugin-napi-macros] ${diag.level}: ${diag.message}`)
          }
        }

        if (result && result.code) {
          if (generateTypes) {
            const emitted = emitDeclarationsFromCode(result.code, id, projectRoot)
            if (emitted) {
              writeTypeDefinitions(id, emitted)
            }
          }
          if (emitMetadata && result.metadata) {
            writeMetadata(id, result.metadata)
          }
          return {
            code: result.code,
            map: null // expandSync does not generate source maps yet
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
