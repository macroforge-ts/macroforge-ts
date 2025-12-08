# Vite Plugin

*The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.*

## Installation

```bash
npm install -D @macroforge/vite-plugin
```

## Configuration

Add the plugin to your `vite.config.ts`:

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge()
  ]
});
```

## Options

```typescript
macroforge({
  // Generate .d.ts files for expanded code
  generateTypes: true,

  // Output directory for generated types
  typesOutputDir: ".macroforge/types",

  // Emit metadata files for debugging
  emitMetadata: false,

  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // File patterns to process
  include: ["**/*.ts", "**/*.tsx"],
  exclude: ["node_modules/**"]
})
```

### Option Reference

| `generateTypes` 
| `boolean` 
| `true` 
| Generate .d.ts files 

| `typesOutputDir` 
| `string` 
| `.macroforge/types` 
| Where to write type files 

| `emitMetadata` 
| `boolean` 
| `false` 
| Emit macro metadata files 

| `keepDecorators` 
| `boolean` 
| `false` 
| Keep decorators in output

## Framework Integration

### React (Vite)

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before React plugin
    react()
  ]
});
```

### SvelteKit

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before SvelteKit
    sveltekit()
  ]
});
```

>
> Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first.

## Development Server

During development, the plugin:

- Watches for file changes

- Expands macros on save

- Provides HMR support for expanded code

## Production Build

During production builds, the plugin:

- Expands all macros in the source files

- Generates type declaration files

- Strips `@derive` decorators from output