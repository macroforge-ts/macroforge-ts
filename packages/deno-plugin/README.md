# @macroforge/deno-plugin

[![JSR](https://jsr.io/badges/@macroforge/deno-plugin)](https://jsr.io/@macroforge/deno-plugin)

## Overview

Deno-native macro expansion for projects that don't use Vite

## Installation

```bash
deno add jsr:@macroforge/deno-plugin
```

## API

### Functions

- **`residentRegistries`** - `options` with any registry it carries as JSON kept by the engine and
  named by id instead, so expanding many files sends each registry once.
- **`expand`** - Expand a single in-memory TypeScript source string.
- **`expandFile`** - Read a file from disk and expand it.

### Interfaces

- **`ExpandOptions`** - Options for `expand` and `expandFile`.
- **`ExpandResult`** - What expanding one source produced.

## Documentation

See the [full API documentation](https://jsr.io/@macroforge/deno-plugin/doc) on JSR.

## License

MIT
