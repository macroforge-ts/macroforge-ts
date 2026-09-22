# @macroforge/shared

[![npm version](https://badge.fury.io/js/%40macroforge%2Fshared.svg)](https://www.npmjs.com/package/@macroforge/shared)
[![JSR](https://jsr.io/badges/@macroforge/shared)](https://jsr.io/@macroforge/shared)

## Overview

Shared utilities for Macroforge plugins

## Installation

```bash
deno add jsr:@macroforge/shared
```

```bash
npm install @macroforge/shared
```

## API

### Functions

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

See the [full API documentation](https://jsr.io/@macroforge/shared/doc) on JSR.

## License

MIT
