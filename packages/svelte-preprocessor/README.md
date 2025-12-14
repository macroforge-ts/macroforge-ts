# @macroforge/svelte-preprocessor

Svelte preprocessor for expanding Macroforge macros in component script blocks

[![npm version](https://badge.fury.io/js/%40macroforge%2Fsvelte-preprocessor.svg)](https://www.npmjs.com/package/@macroforge/svelte-preprocessor)

## Overview

@macroforge/svelte-preprocessor

Svelte preprocessor for expanding Macroforge macros in component script blocks.

This module provides integration between Macroforge's macro expansion system and
Svelte's preprocessing pipeline. It intercepts `<script>` blocks in `.svelte` files,
detects `@derive` decorators, and expands them into generated code before TypeScript
compilation occurs.

## How It Works

1. The preprocessor is registered in `svelte.config.js` as part of the preprocess array
2. When Svelte compiles a component, it passes each `<script>` block to this preprocessor
3. The preprocessor checks if the script contains `@derive` decorators
4. If found, it calls the native `macroforge` binding to expand the macros
5. The expanded code replaces the original script content

## Important Notes

- Must be placed BEFORE other preprocessors (like `vitePreprocess()`) in the chain
- Uses lazy-loading for native bindings to avoid initialization overhead
- Gracefully degrades if native bindings are unavailable
- Only processes TypeScript blocks by default (configurable via options)

## Installation

```bash
npm install @macroforge/svelte-preprocessor
```

## API

### Functions

- **`macroforgePreprocess`** - Whether to preserve `@derive` decorators in the expanded output.

### Types

- **`ExpandResult`** - Whether to preserve `@derive` decorators in the expanded output.
- **`MacroforgePreprocessorOptions`** - Configuration options for the Macroforge Svelte preprocessor.

## Examples

```typescript
macroforgePreprocess()
macroforgePreprocess({ keepDecorators: true })
macroforgePreprocess({ processJavaScript: true })
```

## Documentation

See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/svelte-preprocessor) on the Macroforge website.

## License

MIT
