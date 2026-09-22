# @macroforge/svelte-language-server

[![npm version](https://badge.fury.io/js/%40macroforge%2Fsvelte-language-server.svg)](https://www.npmjs.com/package/@macroforge/svelte-language-server)
[![JSR](https://jsr.io/badges/@macroforge/svelte-language-server)](https://jsr.io/@macroforge/svelte-language-server)

## Overview

A language server for Svelte with macroforge integration

## Installation

```bash
deno add jsr:@macroforge/svelte-language-server
```

```bash
npm install @macroforge/svelte-language-server
```

## API

### Functions

- **`startServer`** - Starts the language server.

### Classes

- **`SvelteCheck`** - Small wrapper around PluginHost's Diagnostic Capabilities for svelte-check,
  without the overhead of the lsp.

### Interfaces

- **`LSOptions`** - Options for `startServer`.
- **`SvelteCheckOptions`** - Options for `SvelteCheck`.

### Types

- **`SvelteCheckDiagnosticSource`** - A kind of diagnostic svelte-check can report.

## Documentation

See the [full API documentation](https://jsr.io/@macroforge/svelte-language-server/doc) on JSR.

## License

MIT
