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
import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge()
  ]
});
```

## Options

TypeScript

```
macroforge({
  // Generate .d.ts files for expanded code
  generateTypes: true,

  // Output directory for generated types
  typesOutputDir: ".macroforge/types",

  // Emit metadata files for debugging
  emitMetadata: false,

  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // File patterns to process
  include: ["**/*.ts", "**/*.tsx"],
  exclude: ["node_modules/**"]
})
```

### Option Reference

| Option           | Type      | Default             | Description               |
| ---------------- | --------- | ------------------- | ------------------------- |
| `generateTypes`  | `boolean` | `true`              | Generate .d.ts files      |
| `typesOutputDir` | `string`  | `.macroforge/types` | Where to write type files |
| `emitMetadata`   | `boolean` | `false`             | Emit macro metadata files |
| `keepDecorators` | `boolean` | `false`             | Keep decorators in output |

## Framework Integration

### React (Vite)

vite.config.ts

```
import macroforge from "@macroforge/vite-plugin";
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
import macroforge from "@macroforge/vite-plugin";
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

### Custom Macro Packages with `file:` Dependencies

If your custom macro package is a local `file:` dependency (e.g. `"@my/macros": "file:./macros"`),
the expanded code may contain runtime imports pointing to files inside that package. Vite's dev
server restricts filesystem access to a set of allowed directories (`src/`, `.svelte-kit/`,
`node_modules/`, etc.), and local `file:` dependencies outside those paths will be blocked.

You must add the package directory to `server.fs.allow`:

vite.config.ts

```
export default defineConfig({
  plugins: [macroforge()],
  server: {
    fs: {
      allow: ['macros']  // path to your local macro package
    }
  }
});
```

The macro package also needs a `package.json` `exports` field so Vite can resolve subpath imports.
For example, if expanded code imports from `@my/macros/helpers`:

macros/package.json

```
{
  "name": "@my/macros",
  "exports": {
    ".": { "types": "./index.d.ts", "default": "./index.js" },
    "./helpers": "./helpers.ts"
  }
}
```

Without this, Vite's dev server will fail with `Pre-transform error: Failed to load url ...` even
though the file exists on disk.

## Production Build

During production builds, the plugin:

- Expands all macros in the source files
- Generates type declaration files
- Strips `@derive` decorators from output
