# Foreign Types

A foreign type is one you don't own and can't annotate — `DateTime` from `effect`, `Decimal` from a
math library, and so on. Instead of adding `serializeWith` to every field that uses it, configure it
once in `macroforge.config.ts` and every derive picks it up.

```typescript
// macroforge.config.ts
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v) => DateTime.formatIso(v),
            deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
            default: () => DateTime.unsafeNow(),
            hasShape: (v) => typeof v === 'string'
        }
    }
};
```

Now any field typed `DateTime.Utc` serializes through your handlers.

## Options

| Key           | Purpose                                                        |
| ------------- | -------------------------------------------------------------- |
| `from`        | Module paths this type may be imported from                    |
| `aliases`     | Additional `{ name, from }` pairs naming the same type         |
| `serialize`   | Value → JSON-safe representation                               |
| `deserialize` | Raw JSON → value                                               |
| `default`     | Value used by `@derive(Default)` and by missing-field defaults |
| `hasShape`    | Structural check used for untagged union variant matching      |

## Import-source validation

`from` exists to disambiguate types that share a name across packages. A `Decimal` from `decimal.js`
and one from your own module are different types that happen to collide.

Be aware of the current behavior: when a type's name matches a foreign-type entry but the **import
source doesn't**, the match falls through to generic handling **silently**. There's no diagnostic
today, so if a foreign type appears not to be applying, check that the module path in `from` matches
how the file actually imports it.

## Namespaced names

A dotted key like `"DateTime.Utc"` is interpreted as namespace `DateTime`, type `Utc`. The namespace
is derived from the key — there's no separate `namespace` option to set.

## Auto-imports

When a handler references a symbol that isn't imported in the target file, the engine emits an
aliased import for it (for example `import { Option as __mf_Option } from "effect"`) so the inlined
expression actually runs. Handlers can therefore reference library functions freely without
requiring every consuming file to import them.
