# Expanded Config

Foreign-type handlers are written in `macroforge.config.ts`, but they run inside the code
`@derive(Encode)` and `@derive(Decode)` generate in your own modules. Macroforge connects the two by
expanding the config into `.macroforge/config/`, where every handler is a named export, and
generated code imports the handlers it calls from `#macroforge/config`.

## Before and after

Take a config whose handlers use a library and a helper of its own:

```typescript
// macroforge.config.ts
import { DateTime } from 'effect';

const isIsoString = (value: unknown): value is string =>
    typeof value === 'string' && !Number.isNaN(Date.parse(value));

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v: DateTime.Utc) => DateTime.formatIso(v),
            decode: (raw: unknown) => DateTime.unsafeFromDate(new Date(String(raw))),
            default: () => DateTime.unsafeNow(),
            hasShape: isIsoString
        }
    }
};
```

and a type that uses it:

```typescript
// event.ts
import type { DateTime } from 'effect';

/** @derive(Encode, Default) */
export interface Event {
    name: string;
    startsAt: DateTime.Utc;
}
```

### Before: handler bodies were copied

Each handler's source text was pasted into every generated module, and the names it read were
patched up with synthetic imports:

```typescript
// event.ts, expanded
import { DateTime as __mf_DateTime } from 'effect';

export function eventEncodeWithContext(value: Event, ctx: __mf_EncodeContext) {
    // ...
    result.startsAt = ((v) => __mf_DateTime.formatIso(v))(value.startsAt);
}

export function eventDefaultValue(): Event {
    return { name: '', startsAt: (() => __mf_DateTime.unsafeNow())() };
}
```

That only works while a handler reads nothing but imports macroforge can guess at.
`hasShape: isIsoString` copies as the bare name `isIsoString`, which the expanded module never
defines, so it fails at runtime with a `ReferenceError`. The same happens to any helper, constant or
local function the config declares.

### After: handlers are imported

The config is expanded into `.macroforge/config/handlers.ts`, with each handler hoisted to an export
named after its type:

```typescript
// .macroforge/config/handlers.ts (generated)
import { DateTime } from 'effect';

const isIsoString = (value: unknown): value is string =>
    typeof value === 'string' && !Number.isNaN(Date.parse(value));

export const __foreign__dateTimeUtcEncode = (v: DateTime.Utc) => DateTime.formatIso(v);
export const __foreign__dateTimeUtcDecode = (raw: unknown) =>
    DateTime.unsafeFromDate(new Date(String(raw)));
export const __foreign__dateTimeUtcDefault = () => DateTime.unsafeNow();
export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: __foreign__dateTimeUtcEncode,
            decode: __foreign__dateTimeUtcDecode,
            default: __foreign__dateTimeUtcDefault,
            hasShape: isIsoString
        }
    }
};

export { isIsoString as __foreign__dateTimeUtcHasShape };
```

Generated code imports what it calls:

```typescript
// event.ts, expanded
import { __foreign__dateTimeUtcDefault, __foreign__dateTimeUtcEncode } from '#macroforge/config';

export function eventEncodeWithContext(value: Event, ctx: __mf_EncodeContext) {
    // ...
    result.startsAt = __foreign__dateTimeUtcEncode(value.startsAt);
}

export function eventDefaultValue(): Event {
    return { name: '', startsAt: __foreign__dateTimeUtcDefault() };
}
```

Every handler now runs in the module it was written in, so it can use anything that module imports
or declares. Each is a separate export, so a bundler keeps only the handlers your code calls.
