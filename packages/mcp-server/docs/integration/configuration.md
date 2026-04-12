# Configuration

Macroforge can be configured with a `macroforge.config.ts` (or `.js`) file in
your project root.

## Configuration File

Macroforge searches for config files in the following order, walking up from the
input file's directory:

- `macroforge.config.ts`
- `macroforge.config.mts`
- `macroforge.config.js`
- `macroforge.config.mjs`
- `macroforge.config.cjs`

Create a `macroforge.config.ts` file:

```typescript
import { defineConfig } from "macroforge/config";

export default defineConfig({
  keepDecorators: false,
  generateConvenienceConst: true,
});
```

## Options Reference

### keepDecorators

| Type | `boolean` | | Default | `false` |

Whether to preserve `@derive` decorators in the output code after macro
expansion. When `false`, decorators are removed after expansion since they serve
only as compile-time directives. When `true`, decorators are kept in the output,
which can be useful for debugging or when using runtime reflection.

### generateConvenienceConst

| Type | `boolean` | | Default | `true` |

Whether to generate a convenience const for non-class types. When `true`,
generates an `export const TypeName = { ... } as const;` that groups all
generated functions for a type into a single namespace-like object. For example:
`export const User = { clone: userClone, serialize: userSerialize } as const;`.

### foreignTypes

| Type | `Record<string, ForeignTypeHandler>` |

Configuration files can define foreign type handlers for external types like
Effect's `DateTime`. When a matching type is found during expansion, the
configured handlers are used automatically.

```typescript
// macroforge.config.ts
import { DateTime } from "effect";
import { defineConfig } from "macroforge/config";

export default defineConfig({
  foreignTypes: {
    "DateTime.DateTime": {
      from: ["effect"],
      serialize: (v) => DateTime.formatIso(v),
      deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
      default: () => DateTime.unsafeNow(),
    },
  },
});
```

### vite

| Type | `VitePluginConfig` |

These options configure the `@macroforge/vite-plugin` behavior.

```typescript
// macroforge.config.ts
import { defineConfig } from "macroforge/config";

export default defineConfig({
  vite: {
    // Whether to generate .d.ts type definition files from expanded code
    generateTypes: true,
    typesOutputDir: ".macroforge/types",

    // Whether to emit macro IR metadata as JSON files
    emitMetadata: true,
    metadataOutputDir: ".macroforge/meta",

    // Enable disk-based expansion cache in dev mode (vite dev)
    devCache: true,
  },
});
```
