# Vite Plugin

The Vite plugin provides build-time macro expansion, transforming your code during development and
production builds.

## Installation

Bash

```
npm install -D @macroforge/vite-plugin
```

## Configuration

Add the plugin to your `vite.config.ts`:

vite.config.ts

```
import { macroforge } from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge()
  ]
});
```

## Options

The vite plugin reads its configuration from a `macroforge.config.ts` (or `.js`, `.mjs`, `.cjs`)
file in your project root. Vite-specific options go under the `vite` key:

macroforge.config.ts

```
export default {
  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // Vite-specific options
  vite: {
    // Generate .d.ts files for expanded code
    generateTypes: true,

    // Output directory for generated types
    typesOutputDir: ".macroforge/types",

    // Emit metadata files for debugging
    emitMetadata: true,

    // Output directory for metadata files
    metadataOutputDir: ".macroforge/meta",

    // Enable disk cache in dev mode
    devCache: true,
  }
};
```

### Option Reference

| Option                   | Type      | Default             | Description                          |
| ------------------------ | --------- | ------------------- | ------------------------------------ |
| `vite.generateTypes`     | `boolean` | `true`              | Generate .d.ts files                 |
| `vite.typesOutputDir`    | `string`  | `.macroforge/types` | Where to write type files            |
| `vite.emitMetadata`      | `boolean` | `true`              | Emit macro metadata files            |
| `vite.metadataOutputDir` | `string`  | `.macroforge/meta`  | Where to write metadata files        |
| `vite.devCache`          | `boolean` | `true`              | Enable disk cache during development |
| `keepDecorators`         | `boolean` | `false`             | Keep decorators in output            |

## Framework Integration

### React (Vite)

vite.config.ts

```
import { macroforge } from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before React plugin
    react()
  ]
});
```

### SvelteKit

vite.config.ts

```
import { macroforge } from "@macroforge/vite-plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before SvelteKit
    sveltekit()
  ]
});
```

Note

Always place the Macroforge plugin before other framework plugins to ensure macros are expanded
first.

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
