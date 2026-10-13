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
- **`declarationFileOf`** - The name of the declaration file a TypeScript module's types are emitted
  to: `user.ts` and `user.tsx` give `user.d.ts...
- **`sourceMapOf`** - The Source Map v3 of `expanded` against `original`, named `source`, or
  `undefined` when the mapping has no segment.
- **`lineAndColumn`** - The 1-based line and column of `offset` in `text`, as an error message names
  a place.

### Interfaces

- **`MacroManifestEntry`** - One macro exported by an external macro package (from
  `__macroforgeGetManifest*`).
- **`DecoratorManifestEntry`** - One decorator exported by an external macro package.
- **`ExpandOptions`** - Options accepted by the engine's `expandSync(code, filepath, options)`.
- **`MacroManifest`** - Aggregated manifest for an external macro package.
- **`ConfigLoadResult`** - Result from parsing a config file (as returned by the native
  `loadConfig`).
- **`CfgFlags`** - Build flags consumed by the `@cfg` attribute macro.
- **`DeprecatedConfig`** - Behavior knobs for the `@deprecated` attribute macro.
- **`MustUseConfig`** - Behavior knobs for the `@mustUse` attribute macro.
- **`NonExhaustiveConfig`** - Behavior knobs for the `@nonExhaustive` attribute macro.
- **`VitePluginConfig`** - Vite plugin configuration options.
- **`MacroConfig`** - Configuration options loaded from `macroforge.config.js` (or .ts/.mjs/.cjs).
- **`SourceMappingSegment`** - One range of the expanded code holding a range of the original as
  written.
- **`SourceMapping`** - The engine's source mapping of an expansion: offsets in UTF-16 code units.
- **`SourceMapV3`** - A Source Map v3 of expanded code against one original source.

### Types

- **`RequireFunction`** - Function type for requiring modules.
- **`ConfigLoader`** - Function type for loading config content.

### Constants

- **`CONFIG_FILES`** - Supported config file names in order of precedence.
- **`TYPESCRIPT_EXTENSIONS`** - The extensions of a TypeScript module, a declaration file among
  them.
- **`JAVASCRIPT_EXTENSIONS`** - The extensions of a JavaScript module.
- **`DECLARATION_ENDINGS`** - The endings of a declaration file.

## Documentation

See the [full API documentation](https://jsr.io/@macroforge/shared/doc) on JSR.

## License

MIT
