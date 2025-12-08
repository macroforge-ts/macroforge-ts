# The Derive System

*The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with `@derive`.*

## Syntax

The derive decorator uses JSDoc comment syntax:

```typescript
/** @derive(MacroName) */
class MyClass { }

/** @derive(Debug, Clone, Eq) */
class AnotherClass { }
```

>
> The `@derive` decorator must be in a JSDoc comment (`/** */`) immediately before the class declaration.

## How It Works

1. **Declaration**: You write `@derive(MacroName)` before a class

2. **Discovery**: Macroforge finds all derive decorators in your code

3. **Expansion**: Each named macro receives the class AST and generates code

4. **Injection**: Generated methods/properties are added to the class

## Multiple Macros

You can derive multiple traits in one decorator:

```typescript
/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;
}

// This generates:
// - toString() from Debug
// - clone() from Clone
// - equals() from Eq
```

Or use separate decorators:

```typescript
/** @derive(Debug) */
/** @derive(Clone) */
/** @derive(Eq) */
class User {
  name: string;
  age: number;
}
```

## Field-Level Decorators

Many macros support field-level customization:

```typescript
/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  password: string;
}
```

Each macro defines its own field decorator options. Check the documentation for each macro to see available options.

## What Can Be Derived

The derive system works on:

- **Classes**: The primary target for derive macros

- **Interfaces**: Some macros can generate companion functions

> **Warning:**
> Enums are not currently supported by the derive system.

## Built-in vs Custom Macros

Macroforge comes with built-in macros (Debug, Clone, Eq), but you can also create custom macros in Rust. Custom macros use the same derive syntax:

```typescript
/** import macro { JSON } from "@my/macros"; */

/** @derive(JSON) */
class User {
  name: string;
}
```

## Next Steps

- [Explore built-in macros](/docs/builtin-macros)

- [Create custom macros](/docs/custom-macros)