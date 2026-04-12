# @macroforge/shared

[![npm version](https://badge.fury.io/js/%40macroforge%2Fshared.svg)](https://www.npmjs.com/package/@macroforge/shared)

## Overview

Shared utilities for Macroforge plugins

## Installation

```bash
npm install @macroforge/shared
```

## API

### Functions

- **`collectExternalDecoratorModules`** - Collects decorator modules from external macro packages
  referenced in the code.
- **`hasMacroAnnotations`** - Checks whether source code contains `@derive(` as a real JSDoc
  directive.
- **`parseMacroImportComments`** - Parses macro import comments from TypeScript code.
- **`clearExternalManifestCache`** - Clears the external manifest cache.
- **`getExternalManifest`** - Attempts to load the manifest from an external macro package.
- **`getExternalMacroInfo`** - Looks up macro info from an external package manifest.
- **`getExternalDecoratorInfo`** - Looks up decorator info from an external package manifest.
- **`findConfigFile`** - Finds a macroforge config file in the directory tree.
- **`loadMacroConfig`** - Loads Macroforge configuration from `macroforge.config.js` (or
  .ts/.mjs/.cjs).

### Interfaces

- **`MacroManifestEntry`**
- **`DecoratorManifestEntry`**
- **`ExpandOptions`**
- **`MacroManifest`**
- **`ConfigLoadResult`** - Result from parsing a config file.
- **`VitePluginConfig`** - Vite plugin configuration options.
- **`MacroConfig`** - Configuration options loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).

### Types

- **`RequireFunction`** - Function type for requiring modules.
- **`ConfigLoader`** - Function type for loading config content.

### Constants

- **`CONFIG_FILES`** - Supported config file names in order of precedence.

## Documentation

See the [full documentation](https://macroforge.dev/docs/api/reference/typescript/shared) on the
Macroforge website.

## License

MIT
