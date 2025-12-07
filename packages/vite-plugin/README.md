# @macroforge/vite-plugin

> **Warning:** This is a work in progress and probably won't work for you. Use at your own risk!

Vite plugin for macroforge compile-time TypeScript macros.

Part of the [macroforge](https://github.com/rymskip/macroforge-ts) project.

## Installation

```bash
npm install @macroforge/vite-plugin
```

## Usage

```typescript
// vite.config.ts
import macroforge from "@macroforge/vite-plugin";

export default defineConfig({
  plugins: [
    macroforge({
      typesOutputDir: ".macroforge/types",
      metadataOutputDir: ".macroforge/meta",
      generateTypes: true,
      emitMetadata: true,
    }),
  ],
});
```

## License

MIT
