## Options Reference

### keepDecorators

Type

`boolean`

Default

`false`

Whether to preserve `@derive` decorators in the output code after macro expansion. When `false`,
decorators are removed after expansion since they serve only as compile-time directives. When
`true`, decorators are kept in the output, which can be useful for debugging or when using runtime
reflection.

### generateConvenienceConst

Type

`boolean`

Default

`true`

Whether to generate a convenience const for non-class types. When `true`, generates an
`export const TypeName = { ... } as const;` that groups all generated functions for a type into a
single namespace-like object. For example:
`export const User = { clone: userClone, serialize: userSerialize } as const;`.

### foreignTypes

Type

`Record<string, ForeignTypeHandler>`

Configuration files can define foreign type handlers for external types like Effect's `DateTime`.
When a matching type is found during expansion, the configured handlers are used automatically.

macroforge.config.ts

```
// macroforge.config.ts
import { DateTime } from "effect";
export default {
  foreignTypes: {
    "DateTime.DateTime": {
      from: ["effect"],
      aliases: [
        { name: "DateTime", from: "effect/DateTime" }
      ],
      serialize: (v) => DateTime.formatIso(v),
      deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
      default: () => DateTime.unsafeNow(),
      // Optional shape check for union variant matching
      hasShape: (v) => v instanceof Date || typeof v === "string"
    }
  }
};
```

Each foreign type handler supports the following properties:

- `from`: Array of module paths this type can be imported from.
- `serialize`: Function `(value) => unknown` for serialization.
- `deserialize`: Function `(raw) => T` for deserialization.
- `default`: Function `() => T` for default value generation.
- `hasShape`: Optional function `(value) => boolean` used for shape-check predicate expression in
  union variant matching.
- `aliases`: Array of `{ name: string, from: string }` objects for alternative type-package pairs.

### vite

Type

`VitePluginConfig`

These options configure the `@macroforge/vite-plugin` behavior.

macroforge.config.ts

```
// macroforge.config.ts
export default {
  vite: {
    // Whether to generate .d.ts type definition files from expanded code
    generateTypes: true,
    typesOutputDir: ".macroforge/types",

    // Whether to emit macro IR metadata as JSON files
    emitMetadata: true,
    metadataOutputDir: ".macroforge/meta",

    // Enable disk-based expansion cache in dev mode (vite dev)
    devCache: true
  }
};
```

- `generateTypes`: Whether to generate `.d.ts` type definition files from expanded code (default:
  `true`).
- `typesOutputDir`: Output directory for generated type definitions, relative to project root
  (default: `".macroforge/types"`).
- `emitMetadata`: Whether to emit macro IR metadata as JSON files (default: `true`).
- `metadataOutputDir`: Output directory for metadata JSON files, relative to project root (default:
  `".macroforge/meta"`).
- `devCache`: Enable disk-based expansion cache in dev mode (`vite dev`) (default: `true`).

### cfg

Build flags consumed by the `@cfg` attribute macro. See [Attribute Macros](/docs/attributes) for the
annotation syntax.

macroforge.config.ts

```
export default {
  cfg: {
    features: ["beta", "experimental"],
    target: "node",
    debugAssertions: false,
    custom: { tier: "pro" }
  }
};
```

- `features`: Active feature flags; `@cfg(&lbrace; feature: "beta" &rbrace;)` passes when the value
  is a member (default: `[]`).
- `target`: Matched exactly against `@cfg(&lbrace; target: … &rbrace;)` (default: unset).
- `debugAssertions`: Boolean matched against `@cfg(&lbrace; debugAssertions: … &rbrace;)` (default:
  `false`).
- `custom`: Arbitrary keys matched exactly by any other annotation key (default:
  `&lbrace;&rbrace;`).

### deprecated

Behavior of the `@deprecated` attribute macro.

macroforge.config.ts

```
export default {
  deprecated: {
    runtimeWarn: true,
    failOnUse: false
  }
};
```

- `failOnUse`: Promote use of a deprecated symbol from an editor hint to a hard expansion error
  (default: `false`).
- `runtimeWarn`: Reserved. Defaults to `true` but currently has no effect — no runtime warning is
  injected.

### mustUse

Behavior of the `@mustUse` attribute macro.

macroforge.config.ts

```
export default {
  mustUse: { mode: "lint" }
};
```

- `mode`: Currently only `"lint"` is recognised, which emits a diagnostic when a return value is
  discarded (default: `"lint"`).

### nonExhaustive

Behavior of the `@nonExhaustive` attribute macro.

macroforge.config.ts

```
export default {
  nonExhaustive: { brand: "__nonExhaustive" }
};
```

- `brand`: Property name used in the branding intersection. Keep it stable across a project
  (default: `"__nonExhaustive"`).

### buildtime

Sandbox settings for `@buildtime` evaluation. See [Buildtime Evaluation](/docs/buildtime) for the
API.

macroforge.config.ts

```
export default {
  buildtime: {
    capabilities: {
      timeout: 5000,
      maxHeap: 256,
      filesystem: { read: ["src/**"], write: [] },
      env: ["NODE_ENV"],
      network: false
    },
    flags: { CHANNEL: "beta" }
  }
};
```

- `capabilities.timeout`: Evaluation budget in milliseconds, enforced (default: `5000`).
- `capabilities.maxHeap`: Heap ceiling in MiB. Advisory — not currently enforced (default: `256`).
- `capabilities.filesystem.read`: Globs readable via `buildtime.fs` (default: `["**"]`).
- `capabilities.filesystem.write`: Reserved; no write API is exposed (default: `[]`).
- `capabilities.env`: Environment variable names exposed as `buildtime.env.NAME`. Deny-by-default
  (default: `[]`).
- `capabilities.network`: Reserved; no network API is exposed (default: `false`).
- `flags`: Values returned by `buildtime.flags.has()` / `.get()` (default: `&lbrace;&rbrace;`).

Capability keys may also be written flat (`buildtime.timeout`); the nested form is canonical because
it matches the path sandbox diagnostics point at.
