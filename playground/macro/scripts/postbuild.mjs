#!/usr/bin/env node
import { mkdir, writeFile } from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import initWasm, { macroMetadata } from '../pkg/playground_macros.js'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const pkgDir = path.resolve(__dirname, '../pkg')

await mkdir(pkgDir, { recursive: true })

await initWasm()
const metadata = macroMetadata()
if (!Array.isArray(metadata)) {
  throw new Error('[ts-macros] macroMetadata() did not return an array')
}

const jsSource = buildJs(metadata)
const dtsSource = buildDts(metadata)

await writeFile(path.join(pkgDir, 'index.js'), jsSource, 'utf8')
await writeFile(path.join(pkgDir, 'index.d.ts'), dtsSource, 'utf8')

function buildJs(metadata) {
  const decoratorsLiteral = JSON.stringify(metadata, null, 2)
  const exportsBlock = metadata
    .map((decorator) => {
      const name = decorator.export
      return `export const ${name} = createDecorator("${name}");`
    })
    .join('\n')

  return `import initWasm, { logDecorator } from './playground_macros.js';

let wasmReady;

function ensureWasm() {
  if (!wasmReady) {
    wasmReady = initWasm().catch((error) => {
      console.warn('[ts-macros] Failed to initialize wasm shim:', error);
    });
  }
  return wasmReady;
}

function logWith(name) {
  ensureWasm()?.then(() => {
    try {
      logDecorator(name);
    } catch (error) {
      console.warn('[ts-macros] Failed to log decorator usage:', error);
    }
  });
}

const DECORATORS = ${decoratorsLiteral};

function createDecorator(name) {
  return function (..._args) {
    logWith(name);
    return function () {
      return;
    };
  };
}

${exportsBlock}
`
}

function buildDts(metadata) {
  const exportsBlock = metadata
    .map((decorator) => {
      const name = decorator.export
      const tsType = decorator_kind_to_ts(decorator.kind)
      return `export declare const ${name}: (...args: unknown[]) => ${tsType};`
    })
    .join('\n')

  return `type MacroDecorator =
  | ClassDecorator
  | PropertyDecorator
  | MethodDecorator
  | ParameterDecorator;

${exportsBlock}
`
}

function decorator_kind_to_ts(kind) {
  switch (kind) {
    case 'class':
      return 'ClassDecorator'
    case 'method':
    case 'accessor':
      return 'MethodDecorator'
    case 'parameter':
      return 'ParameterDecorator'
    default:
      return 'PropertyDecorator'
  }
}
