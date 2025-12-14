# @macroforge/typescript-plugin

TypeScript language service plugin that augments classes decorated with @derive to include macro-generated methods.

[![npm version](https://badge.fury.io/js/%40macroforge%2Ftypescript-plugin.svg)](https://www.npmjs.com/package/@macroforge/typescript-plugin)

## Overview

TypeScript Language Service Plugin for Macroforge

This plugin integrates Macroforge's compile-time macro expansion with TypeScript's
Language Service to provide seamless IDE support for macro-decorated classes.

## Architecture Overview

The plugin operates by intercepting TypeScript's Language Service methods and
transforming source code on-the-fly:

1. **Macro Expansion**: When TypeScript requests a file's content via `getScriptSnapshot`,
this plugin intercepts the call and returns the macro-expanded version instead.

2. **Position Mapping**: Since expanded code has different positions than the original,
the plugin maintains a {@link PositionMapper} for each file to translate positions
between original and expanded coordinates.

3. **Virtual .d.ts Files**: For each macro-containing file, the plugin generates a
companion `.macroforge.d.ts` file containing type declarations for generated methods.

## Supported File Types

- `.ts` - TypeScript files
- `.tsx` - TypeScript JSX files
- `.svelte` - Svelte components (with `<script lang="ts">`)

## Hook Categories

The plugin hooks into three categories of Language Service methods:

- **Host-level hooks**: Control what TypeScript "sees" (`getScriptSnapshot`, `fileExists`, etc.)
- **Diagnostic hooks**: Map error positions back to original source (`getSemanticDiagnostics`)
- **Navigation hooks**: Handle go-to-definition, references, completions, etc.

@example
```typescript
{
"compilerOptions": {
"plugins": [{ "name": "@macroforge/typescript-plugin" }]
}
}
```

## Installation

```bash
npm install @macroforge/typescript-plugin
```

## API

### Functions

- **`findDeriveAtPosition`** - Finds a macro name within `@derive(...)` decorators at a given cursor position.
- **`findDecoratorAtPosition`** - Finds a field decorator (like `@serde` or `@debug`) at a given cursor position.
- **`getMacroHoverInfo`** - const lastCommentEnd = beforeMatch.lastIndexOf("*/
- **`shouldProcess`** - Determines whether a file should be processed for macro expansion.
- **`hasMacroDirectives`** - Performs a quick check to determine if a file contains any macro-related directives.
- **`loadMacroConfig`** - Whether to preserve decorator syntax in the expanded output.
- **`init`** - Main plugin factory function conforming to the TypeScript Language Service Plugin API.
- **`create`** - Creates the plugin instance for a TypeScript project.
- **`processFile`** - Processes a file through macro expansion via the native Rust plugin.
- **`toPlainDiagnostic`** - Converts a TypeScript diagnostic to a plain object for the native plugin.
- ... and 1 more

### Types

- **`MacroConfig`** - Configuration options loaded from `macroforge.json`.

## Examples

```typescript
{
"compilerOptions": {
"plugins": [{ "name": "@macroforge/typescript-plugin" }]
}
}
```

## Documentation

See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/typescript-plugin) on the Macroforge website.

## License

MIT
