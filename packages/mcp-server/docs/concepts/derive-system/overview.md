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
import { encodedText as __mf_encodedText, encodedTextWithoutMetadata as __mf_encodedTextWithoutMetadata, EncodeContext as __mf_EncodeContext, fieldRequirement as __mf_fieldRequirement } from "@macroforge/core/endec";
import type { FieldSelection as __mf_FieldSelection } from "@macroforge/core/endec";

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
    return keepMetadata ? __mf_encodedText(__raw) : __mf_encodedTextWithoutMetadata(__raw, "__type");
}

/** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context  */
export function userEncodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
    return userEncodePartialWithContext(value, ctx);
}

/** Encodes the fields a partial value has, with an existing context. @param value - The value to encode @param ctx - The encoding context @param fields - The fields a `Pick` or `Omit` keeps: every field when left out  */
export function userEncodePartialWithContext(value: Partial<User>, ctx: __mf_EncodeContext, fields?: __mf_FieldSelection<keyof User>): Record<string, unknown> {
    const __fields = fields === undefined ? undefined : __mf_fieldRequirement(fields);

    let __id: number | undefined;
    if (__fields === undefined) {
        const existingId = ctx.getId(value);
        if (existingId !== undefined) {
            return { __ref: existingId };
        }
        __id = ctx.register(value);
    }

    const result: Record<string, unknown> = {
        "__type": "User",
        __id,
    };

    if (__fields === undefined || __fields.keeps("id")) {

        if (value.id !== undefined) {
            result.user_id = value.id;
        }
    }
    if (__fields === undefined || __fields.keeps("name")) {

        if (value.name !== undefined) {
            result.name = value.name;
        }
    }
    if (__fields === undefined || __fields.keeps("metadata")) {

        if (value.metadata !== undefined) {
            result.metadata = value.metadata;
        }
    }

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
