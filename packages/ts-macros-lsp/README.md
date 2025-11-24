# ts-macros Language Server

Minimal Language Server Protocol (LSP) implementation that augments IntelliSense for classes decorated with `@Derive(...)`.

## Features

- Parses open TypeScript/TSX documents with the TypeScript compiler API
- Detects classes decorated with `@Derive`
- Tracks identifiers that reference decorated classes (constructor calls, type annotations, class fields, and parameters)
- Injects completion items for macro-generated methods like `toString()` and `toJSON()`

## Development

```bash
npm install
npm run build -w @ts-macros/ts-macros-lsp
```

The compiled server lives at `packages/ts-macros-lsp/dist/server.js` and is consumed by the Zed extension.
