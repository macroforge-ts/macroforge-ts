# The Derive System

The derive system is inspired by Rust's derive macros. It allows you to automatically implement
common patterns by annotating your classes with `@derive`.

## Syntax Reference

Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard
TypeScript tooling.

### The @derive Statement

The `@derive` decorator triggers macro expansion on a class or interface:

Source

TypeScript

```
/** @derive(Debug) */
class MyClass {
  value: string;
}
```

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)
- Must appear immediately before the class/interface declaration
- Multiple macros can be comma-separated: `@derive(A, B, C)`
- Multiple `@derive` statements can be stacked

Source

TypeScript

```
/** @derive(Debug, Clone) */
class User {
  name: string;
  email: string;
}
```

### The import macro Statement

To use macros from external packages, you must declare them with `import macro`:

TypeScript

```
/** import macro { MacroName } from "package-name"; */
```

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)
- Can appear anywhere in the file (typically at the top)
- Multiple macros can be imported: `import macro { A, B } from "pkg";`
- Multiple import statements can be used for different packages

TypeScript

```
/** import macro { JSON, Validate } from "@my/macros"; */
/** import macro { Builder } from "@other/macros"; */

/** @derive(JSON, Validate, Builder) */
class User {
  name: string;
  email: string;
}
```

Built-in macros

Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, and the
type-position `$Newtype`) do not require an import statement.

### Field Attributes

Macros can define field-level attributes to customize behavior per field:

Before (Your Code)

```
/** @derive(Debug, Encode) */
class User {
  /** @debug({ rename: "userId" }) */
  /** @endec({ rename: "user_id" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  /** @endec({ skip: true }) */
  password: string;

  metadata: Record<string, unknown>;
}
```

After (Generated)

```
import { structuralDebug as __mf_structuralDebug } from "@macroforge/core/structural";
import { EncodeContext as __mf_EncodeContext } from "@macroforge/core/endec";

class User {
  
  
  id: number;

  name: string;

  
  
  password: string;

  metadata: Record<string, unknown>;

    static toString(value: User): string {
        return userToString(value);
    }

    /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation  */
    static encode(value: User, keepMetadata?: boolean): string {
        return userEncode(value, keepMetadata);
    }

    /** @internal Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context  */
    static encodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
        return userEncodeWithContext(value, ctx);
    }
}

export function userToString(value: User): string {
    const parts: string[] = [];
    parts.push("userId: " + String(value.id));
    parts.push("name: " + String(value.name));
    parts.push("metadata: " + __mf_structuralDebug(value.metadata));
    return "User { " + parts.join(", ") + " }";
}

/** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation  */
export function userEncode(value: User, keepMetadata?: boolean): string {
    const ctx = __mf_EncodeContext.create();
    const __raw = userEncodeWithContext(value, ctx);
    if (keepMetadata) return JSON.stringify(__raw);
    return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
}

/** @internal Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context  */
export function userEncodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {

    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }

    const __id = ctx.register(value);

    const result: Record<string, unknown> = {
        "__type": "User",
        __id,
    };

    
    result.user_id = value.id;
    result.name = value.name;
    result.metadata = value.metadata;

    return result;
}
```

Syntax rules:

- Must be inside a JSDoc comment immediately before the field
- Options use object literal syntax: `@attr({ key: value })`
- Boolean options: `@attr({ skip: true })`
- String options: `@attr({ rename: "newName" })`
- Multiple attributes can be on separate lines or combined

Common field attributes by macro:

| Macro         | Attribute     | Options                                |
| ------------- | ------------- | -------------------------------------- |
| Debug         | `@debug`      | `skip`, `rename`                       |
| Clone         | `@clone`      | `skip`, `clone_with`                   |
| Encode/Decode | `@endec`      | `skip`, `rename`, `flatten`, `default` |
| Hash          | `@hash`       | `skip`                                 |
| PartialEq/Ord | `@eq`, `@ord` | `skip`                                 |

## How It Works

1. **Declaration**: You write `@derive(MacroName)` before a class
2. **Discovery**: Macroforge finds all derive decorators in your code
3. **Expansion**: Each named macro receives the class AST and generates code
4. **Injection**: Generated methods/properties are added to the class

## What Can Be Derived

The derive system works on:

- **Classes**: The primary target for derive macros
- **Interfaces**: Macros generate companion namespace functions
- **Enums**: Macros generate namespace functions for enum values
- **Type aliases**: Both object types and union types are supported

## Built-in vs Custom Macros

Macroforge comes with built-in macros that work out of the box. You can also create custom macros in
Rust and use them via the `import macro` statement.

| Type     | Import Required | Examples                                                                            |
| -------- | --------------- | ----------------------------------------------------------------------------------- |
| Built-in | No              | Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, `$Newtype` |
| Custom   | Yes             | Any macro from an external package                                                  |

## Next Steps

- [Explore built-in macros](../../docs/builtin-macros)
- [Create custom macros](../../docs/custom-macros)
