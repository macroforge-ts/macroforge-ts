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

- **`parseMacroImportComments`** - Parses macro import comments to extract macro name to module path mappings.
- **`getExternalManifest`** - Attempts to load the manifest from an external macro package.
- **`getExternalMacroInfo`** - Looks up macro info from an external package manifest.
- **`getExternalDecoratorInfo`** - Looks up decorator info from an external package manifest.
- **`findDeriveAtPosition`** - Finds a macro name within `@derive(...)` decorators at a given cursor position.
- **`findDeriveKeywordAtPosition`** - Finds the `@derive` keyword at a given cursor position.
- **`findDecoratorAtPosition`** - Finds a field decorator (like `@serde` or `@debug`) at a given cursor position.
- **`findEnclosingDeriveContext`** - const lastCommentEnd = beforeMatch.lastIndexOf("*/
- **`getMacroHoverInfo`** - Generates hover information (QuickInfo) for macros and decorators at a cursor position.
- **`shouldProcess`** - Determines whether a file should be processed for macro expansion.
- ... and 7 more

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
