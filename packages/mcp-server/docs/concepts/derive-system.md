# The Derive System

*The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with `@derive`.*

## Syntax Reference

Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.

### The @derive Statement

The `@derive` decorator triggers macro expansion on a class or interface:

```typescript
/** @derive(MacroName) */
class MyClass { }

/** @derive(Debug, Clone, PartialEq) */
class AnotherClass { }
```

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)

- Must appear immediately before the class/interface declaration

- Multiple macros can be comma-separated: `@derive(A, B, C)`

- Multiple `@derive` statements can be stacked

```typescript
// Single derive with multiple macros
/** @derive(Debug, Clone) */
class User { }

// Multiple derive statements (equivalent)
/** @derive(Debug) */
/** @derive(Clone) */
class User { }
```

### The import macro Statement

To use macros from external packages, you must declare them with `import macro`:

```typescript
/** import macro { MacroName } from "package-name"; */
```

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)

- Can appear anywhere in the file (typically at the top)

- Multiple macros can be imported: `import macro { A, B } from "pkg";`

- Multiple import statements can be used for different packages

```typescript
/** import macro { JSON, Validate } from "@my/macros"; */
/** import macro { Builder } from "@other/macros"; */

/** @derive(JSON, Validate, Builder) */
class User {
  name: string;
  email: string;
}
```

>
> Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement.

### Field Attributes

Macros can define field-level attributes to customize behavior per field:

```typescript
/** @attributeName(options) */
```

The attribute name and available options depend on the macro. Common patterns:

```typescript
/** @derive(Debug, Serialize) */
class User {
  /** @debug({ rename: "userId" }) */
  /** @serde({ rename: "user_id" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ flatten: true }) */
  metadata: Record<string, unknown>;
}
```

Syntax rules:

- Must be inside a JSDoc comment immediately before the field

- Options use object literal syntax: `@attr({ key: value })`

- Boolean options: `@attr({ skip: true })`

- String options: `@attr({ rename: "newName" })`

- Multiple attributes can be on separate lines or combined

Common field attributes by macro:

| Debug 
| `@debug` 
| `skip`, `rename` 

| Clone 
| `@clone` 
| `skip`, `clone_with` 

| Serialize/Deserialize 
| `@serde` 
| `skip`, `rename`, `flatten`, `default` 

| Hash 
| `@hash` 
| `skip` 

| PartialEq/Ord 
| `@eq`, `@ord` 
| `skip`

## How It Works

1. **Declaration**: You write `@derive(MacroName)` before a class

2. **Discovery**: Macroforge finds all derive decorators in your code

3. **Expansion**: Each named macro receives the class AST and generates code

4. **Injection**: Generated methods/properties are added to the class

## What Can Be Derived

The derive system works on:

- **Classes**: The primary target for derive macros

- **Interfaces**: Some macros can generate companion functions

> **Warning:**
> Enums are not currently supported by the derive system.

## Built-in vs Custom Macros

Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the `import macro` statement.

| Built-in 
| No 
| Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize 

| Custom 
| Yes 
| Any macro from an external package

## Next Steps

- [Explore built-in macros](/docs/builtin-macros)

- [Create custom macros](/docs/custom-macros)