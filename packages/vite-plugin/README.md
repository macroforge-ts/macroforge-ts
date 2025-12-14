# @macroforge/vite-plugin

[![npm version](https://badge.fury.io/js/%40macroforge%2Fvite-plugin.svg)](https://www.npmjs.com/package/@macroforge/vite-plugin)

## Overview

@macroforge/vite-plugin

Vite plugin for Macroforge compile-time TypeScript macro expansion.

This plugin integrates Macroforge's Rust-based macro expander into the Vite build pipeline,
enabling compile-time code generation through `@derive` decorators. It processes TypeScript
files during the build, expands macros, generates type definitions, and emits metadata.

@example
```typescript
import { defineConfig } from 'vite';
import macroforgePlugin from '@macroforge/vite-plugin';

export default defineConfig({
plugins: [
macroforgePlugin({
generateTypes: true,
typesOutputDir: 'src/types/generated',
emitMetadata: true,
}),
],
});
```

## Installation

```bash
npm install @macroforge/vite-plugin
```

## API

### Functions

- **`loadMacroConfig`** - Whether to preserve `@derive` decorators in the output code after macro expansion.
- **`napiMacrosPlugin`** - Creates a Vite plugin for Macroforge compile-time macro expansion.
- **`ensureDir`** - Ensures a directory exists, creating it recursively if necessary.
- **`writeTypeDefinitions`** - Writes generated TypeScript declaration files to the configured output directory.
- **`writeMetadata`** - Writes macro intermediate representation (IR) metadata to JSON files.
- **`formatTransformError`** - Formats transformation errors into user-friendly messages.

### Types

- **`NapiMacrosPluginOptions`** - Configuration options for the Macroforge Vite plugin.
- **`MacroConfig`** - Glob patterns, regular expressions, or arrays of either to specify which files

## Examples

```typescript
import { defineConfig } from 'vite';
import macroforgePlugin from '@macroforge/vite-plugin';
export default defineConfig({
plugins: [
macroforgePlugin({
generateTypes: true,
typesOutputDir: 'src/types/generated',
emitMetadata: true,
}),
],
});
```

## Documentation

See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/vite-plugin) on the Macroforge website.

## License

MIT
