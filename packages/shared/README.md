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
- **`hasMacroAnnotations`** - Checks whether source code contains a macroforge JSDoc annotation —
  `@derive(...)`, `@cfg(...)`, `@deprecated`, `@mustUse`, or `@nonExhaustive`.
- **`parseMacroImportComments`** - Parses macro import comments from TypeScript code.
- **`clearExternalManifestCache`** - Clears the external manifest cache.
- **`getExternalManifest`** - Attempts to load the manifest from an external macro package.
- **`getExternalMacroInfo`** - Looks up macro info from an external package manifest.
- **`getExternalDecoratorInfo`** - Looks up decorator info from an external package manifest.
- **`findConfigFile`** - Finds a macroforge config file in the directory tree.
- **`loadMacroConfig`** - Loads Macroforge configuration from `macroforge.config.js` (or
  .ts/.mjs/.cjs).

### Interfaces

- **`MacroManifestEntry`** - One macro exported by an external macro package (from
  `__macroforgeGetManifest*`).
- **`DecoratorManifestEntry`** - One decorator exported by an external macro package.
- **`ExpandOptions`** - Options accepted by the native engine's
  `expandSync(code, filepath, options)`.
- **`MacroManifest`** - Aggregated manifest for an external macro package.
- **`ConfigLoadResult`** - Result from parsing a config file (as returned by the native
  `loadConfig`).
- **`CfgFlags`** - Build flags consumed by the `@cfg` attribute macro.
- **`DeprecatedConfig`** - Behavior knobs for the `@deprecated` attribute macro.
- **`MustUseConfig`** - Behavior knobs for the `@mustUse` attribute macro.
- **`NonExhaustiveConfig`** - Behavior knobs for the `@nonExhaustive` attribute macro.
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
