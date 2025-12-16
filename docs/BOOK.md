# Macroforge Documentation

_TypeScript Macros - Rust-Powered Code Generation_

---

## Table of Contents

### Getting Started

- [Installation](#installation)
- [First Macro](#first-macro)

### Core Concepts

- [How Macros Work](#how-macros-work)
- [The Derive System](#the-derive-system)
- [Architecture](#architecture)

### Built-in Macros

- [Overview](#overview)
- [Debug](#debug)
- [Clone](#clone)
- [Default](#default)
- [Hash](#hash)
- [Ord](#ord)
- [PartialEq](#partialeq)
- [PartialOrd](#partialord)
- [Serialize](#serialize)
- [Deserialize](#deserialize)

### Custom Macros

- [Overview](#overview)
- [Rust Setup](#rust-setup)
- [ts_macro_derive](#ts-macro-derive)
- [Template Syntax](#template-syntax)

### Integration

- [Overview](#overview)
- [CLI](#cli)
- [TypeScript Plugin](#typescript-plugin)
- [Vite Plugin](#vite-plugin)
- [Configuration](#configuration)

### Language Servers

- [Overview](#overview)
- [Svelte](#svelte)
- [Zed Extensions](#zed-extensions)

### API Reference

- [Overview](#overview)
- [expandSync()](#expandsync-)
- [transformSync()](#transformsync-)
- [NativePlugin](#nativeplugin)
- [PositionMapper](#positionmapper)

---

# Getting Started

# Installation

_Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros._

## Requirements

- Node.js 24.0 or later
- TypeScript 5.9 or later

## Install the Package

Install Macroforge using your preferred package manager:

````
npm install macroforge
``` ```
bun add macroforge
``` ```
pnpm add macroforge
```  **Info Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64). ## Basic Usage
The simplest way to use Macroforge is with the built-in derive macros. Add a `@derive` comment decorator to your class:
````

/\*_ @derive(Debug, Clone, PartialEq) _/
class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}
}

// After macro expansion, User has:
// - toString(): string (from Debug)
// - clone(): User (from Clone)
// - equals(other: unknown): boolean (from PartialEq)

```## IDE Integration
 For the best development experience, add the TypeScript plugin to your `tsconfig.json`:
```

{
"compilerOptions": {
"plugins": [
{
"name": "@macroforge/typescript-plugin"
}
]
}
}

```This enables features like:
 - Accurate error positions in your source code
 - Autocompletion for generated methods
 - Type checking for expanded code
 ## Build Integration (Vite)
 If you're using Vite, add the plugin to your config for automatic macro expansion during build:
```

import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
plugins: [
macroforge({
generateTypes: true,
typesOutputDir: ".macroforge/types"
})
]
});

```## Next Steps
 Now that you have Macroforge installed, learn how to use it:
 - [Create your first macro](../docs/getting-started/first-macro)
 - [Understand how macros work](../docs/concepts)
 - [Explore built-in macros](../docs/builtin-macros)
**

---

# Your First Macro
 *Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*
 ## Creating a Class with Derive Macros
 Start by creating a simple `User` class. We'll use the `@derive` decorator to automatically generate methods.

**Before:**
```

/\*_ @derive(Debug, Clone, PartialEq) _/
export class User {
name: string;
age: number;
email: string;

    constructor(name: string, age: number, email: string) {
        this.name = name;
        this.age = age;
        this.email = email;
    }

}

```
**After:**
```

export class User {
name: string;
age: number;
email: string;

    constructor(name: string, age: number, email: string) {
        this.name = name;
        this.age = age;
        this.email = email;
    }

    toString(): string {
        const parts: string[] = [];
        parts.push('name: ' + this.name);
        parts.push('age: ' + this.age);
        parts.push('email: ' + this.email);
        return 'User { ' + parts.join(', ') + ' }';
    }

    clone(): User {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.name = this.name;
        cloned.age = this.age;
        cloned.email = this.email;
        return cloned;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return (
            this.name === typedOther.name &&
            this.age === typedOther.age &&
            this.email === typedOther.email
        );
    }

}

```## Using the Generated Methods

```

const user = new User("Alice", 30, "alice@example.com");

// Debug: toString()
console.log(user.toString());
// Output: User { name: Alice, age: 30, email: alice@example.com }

// Clone: clone()
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq: equals()
console.log(user.equals(copy)); // true

const different = new User("Bob", 25, "bob@example.com");
console.log(user.equals(different)); // false

```## Customizing Behavior
 You can customize how macros work using field-level decorators. For example, with the Debug macro:

**Before:**
```

/** @derive(Debug) \*/
export class User {
/** @debug({ rename: "userId" }) \*/
id: number;

    name: string;

    /** @debug({ skip: true }) */
    password: string;

    constructor(id: number, name: string, password: string) {
        this.id = id;
        this.name = name;
        this.password = password;
    }

}

```
**After:**
```

export class User {
id: number;

    name: string;

    password: string;

    constructor(id: number, name: string, password: string) {
        this.id = id;
        this.name = name;
        this.password = password;
    }

    toString(): string {
        const parts: string[] = [];
        parts.push('userId: ' + this.id);
        parts.push('name: ' + this.name);
        return 'User { ' + parts.join(', ') + ' }';
    }

}
` `
const user = new User(42, "Alice", "secret123");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
// Note: 'id' is renamed to 'userId', 'password' is skipped

```**Field-level decorators Field-level decorators let you control exactly how each field is handled by the macro. ## Next Steps
 - [Learn how macros work under the hood](../../docs/concepts)
 - [Explore all Debug options](../../docs/builtin-macros/debug)
 - [Create your own custom macros](../../docs/custom-macros)
**

---

# Core Concepts

# How Macros Work
 *Macroforge performs compile-time code generation by parsing your TypeScript, expanding macros, and outputting transformed code. This happens before your code runs, resulting in zero runtime overhead.*
 ## Compile-Time Expansion
 Unlike runtime solutions that use reflection or proxies, Macroforge expands macros at compile time:
 1. **Parse**: Your TypeScript code is parsed into an AST using SWC
 2. **Find**: Macroforge finds `@derive` decorators and their associated items
 3. **Expand**: Each macro generates new code based on the class structure
 4. **Output**: The transformed TypeScript is written out, ready for normal compilation

**Before:**
```

/\*_ @derive(Debug) _/
class User {
name: string;
}

```
**After:**
```

class User {
name: string;

    toString(): string {
        const parts: string[] = [];
        parts.push('name: ' + this.name);
        return 'User { ' + parts.join(', ') + ' }';
    }

}

```## Zero Runtime Overhead
 Because code generation happens at compile time, there's no:
 - Runtime reflection or metadata
 - Proxy objects or wrappers
 - Additional dependencies in your bundle
 - Performance cost at runtime
 The generated code is plain TypeScript that compiles to efficient JavaScript.
 ## Source Mapping
 Macroforge tracks the relationship between your source code and the expanded output. This means:
 - Errors in generated code point back to your source
 - Debugging works correctly
 - IDE features like "go to definition" work as expected
 > with @derive decorators   TypeScript → AST   Finds @derive decorators, runs macros, generates new AST nodes   AST → TypeScript   ready for normal compilation  ## Integration Points
 Macroforge integrates at two key points:
 ### IDE (TypeScript Plugin)
 The TypeScript plugin intercepts language server calls to provide:
 - Diagnostics that reference your source, not expanded code
 - Completions for generated methods
 - Hover information showing what macros generate
 ### Build (Vite Plugin)
 The Vite plugin runs macro expansion during the build process:
 - Transforms files before they reach the TypeScript compiler
 - Generates type declaration files (.d.ts)
 - Produces metadata for debugging
 ## Next Steps
 - [Learn about the derive system](../docs/concepts/derive-system)
 - [Explore the architecture](../docs/concepts/architecture)

---

# The Derive System
 *The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with `@derive`.*
 ## Syntax Reference
 Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.
 ### The @derive Statement
 The `@derive` decorator triggers macro expansion on a class or interface:

**Source:**
```

/\*_ @derive(Debug) _/
class MyClass {
value: string;
}

```Syntax rules:
 - Must be inside a JSDoc comment (`/** */`)
 - Must appear immediately before the class/interface declaration
 - Multiple macros can be comma-separated: `@derive(A, B, C)`
 - Multiple `@derive` statements can be stacked

**Source:**
```

/\*_ @derive(Debug, Clone) _/
class User {
name: string;
email: string;
}

```### The import macro Statement
 To use macros from external packages, you must declare them with `import macro`:
```

/\*_ import macro { MacroName } from "package-name"; _/

```Syntax rules:
 - Must be inside a JSDoc comment (`/** */`)
 - Can appear anywhere in the file (typically at the top)
 - Multiple macros can be imported: `import macro { A, B } from "pkg";`
 - Multiple import statements can be used for different packages
```

/** import macro { JSON, Validate } from "@my/macros"; \*/
/** import macro { Builder } from "@other/macros"; \*/

/\*_ @derive(JSON, Validate, Builder) _/
class User {
name: string;
email: string;
}

```**Built-in macros Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement. ### Field Attributes
 Macros can define field-level attributes to customize behavior per field:
 **
**Before:**
```

/** @derive(Debug, Serialize) \*/
class User {
/** @debug({ rename: "userId" }) _/
/\*\* @serde({ rename: "user_id" }) _/
id: number;

    name: string;

    /** @debug({ skip: true }) */
    /** @serde({ skip: true }) */
    password: string;

    /** @serde({ flatten: true }) */
    metadata: Record<string, unknown>;

}

```
**After:**
```

import { SerializeContext } from "macroforge/serde";

class User {

id: number;

name: string;

password: string;

metadata: Record<string, unknown>;

toString(): string {
const parts: string[] = [];
parts.push("userId: " + this.id);
parts.push("name: " + this.name);
parts.push("metadata: " + this.metadata);
return "User { " + parts.join(", ") + " }";
}

toStringifiedJSON(): string {
const ctx = SerializeContext.create();
return JSON.stringify(this.serializeWithContext(ctx));
}

toObject(): Record<string, unknown> {
const ctx = SerializeContext.create();
return this.serializeWithContext(ctx);
}

serializeWithContext(ctx: SerializeContext): Record<string, unknown> {
const existingId = ctx.getId(this);
if (existingId !== undefined) {
return {
**ref: existingId
};
}
const **id = ctx.register(this);
const result: Record<string, unknown> = {
**type: "User",
**id
};
result["user_id"] = this.id;
result["name"] = this.name;
{
const **flattened = record < string, unknown;
const { **type: \_, **id: **, ...rest } = \_\_flattened as any;
Object.assign(result, rest);
}
return result;
}
}

```Syntax rules:
 - Must be inside a JSDoc comment immediately before the field
 - Options use object literal syntax: `@attr({ key: value })`
 - Boolean options: `@attr({ skip: true })`
 - String options: `@attr({ rename: "newName" })`
 - Multiple attributes can be on separate lines or combined
 Common field attributes by macro:
 | Macro | Attribute | Options |
| --- | --- | --- |
| Debug | `@debug` | `skip`, `rename` |
| Clone | `@clone` | `skip`, `clone_with` |
| Serialize/Deserialize | `@serde` | `skip`, `rename`, `flatten`, `default` |
| Hash | `@hash` | `skip` |
| PartialEq/Ord | `@eq`, `@ord` | `skip` |
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
 Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the `import macro` statement.
 | Type | Import Required | Examples |
| --- | --- | --- |
| Built-in | No | Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize |
| Custom | Yes | Any macro from an external package |
 ## Next Steps
 - [Explore built-in macros](../../docs/builtin-macros)
 - [Create custom macros](../../docs/custom-macros)

---

# Architecture
 *Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.*
 ## Overview
    `macroforge_ts_syn`macroforge_ts_quote`macroforge_ts_macros `TypeScript parsing & codegen ## Core Components
 ### SWC Core
 The foundation layer provides:
 - Fast TypeScript/JavaScript parsing
 - AST representation
 - Code generation (AST → source code)
 ### macroforge_ts_syn
 A Rust crate that provides:
 - TypeScript-specific AST types
 - Parsing utilities for macro input
 - Derive input structures (class fields, decorators, etc.)
 ### macroforge_ts_quote
 Template-based code generation similar to Rust's `quote!`:
 - `ts_template!` - Generate TypeScript code from templates
 - `body!` - Generate class body members
 - Control flow: `{#for}`, `{#if}`, `{$let}`
 ### macroforge_ts_macros
 The procedural macro attribute for defining derive macros:
 - `#[ts_macro_derive(Name)]` attribute
 - Automatic registration with the macro system
 - Error handling and span tracking
 ### NAPI-RS Bindings
 Bridges Rust and Node.js:
 - Exposes `expandSync`, `transformSync`, etc.
 - Provides the `NativePlugin` class for caching
 - Handles data marshaling between Rust and JavaScript
 ## Data Flow
  TypeScript with @derive   receives JavaScript string   parses to AST   finds @derive decorators   extract data, run macro, generate AST nodes   generated nodes into AST   generates source code   to JavaScript with source mapping  ## Performance Characteristics
 - **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack
 - **Caching**: `NativePlugin` caches results by file version
 - **Binary search**: Position mapping uses O(log n) lookups
 - **Zero-copy**: SWC's arena allocator minimizes allocations
 ## Re-exported Crates
 For custom macro development, `macroforge_ts` re-exports everything you need:
```

// Convenient re-exports for macro development
use macroforge_ts::macros::{ts_macro_derive, body, ts_template, above, below, signature};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

// Also available: raw crate access and SWC modules
use macroforge_ts::swc_core;
use macroforge_ts::swc_common;
use macroforge_ts::swc_ecma_ast;

```## Next Steps
 - [Write custom macros](../../docs/custom-macros)
 - [Explore the API reference](../../docs/api)

---

# Built-in Macros

# Built-in Macros
 *Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*
 ## Overview
 | Macro | Generates | Description |
| --- | --- | --- |
| [`Debug`](../docs/builtin-macros/debug) | `toString(): string` | Human-readable string representation |
| [`Clone`](../docs/builtin-macros/clone) | `clone(): T` | Creates a deep copy of the object |
| [`Default`](../docs/builtin-macros/default) | `static default(): T` | Creates an instance with default values |
| [`Hash`](../docs/builtin-macros/hash) | `hashCode(): number` | Generates a hash code for the object |
| [`PartialEq`](../docs/builtin-macros/partial-eq) | `equals(other: T): boolean` | Value equality comparison |
| [`Ord`](../docs/builtin-macros/ord) | `compare(other: T): number` | Total ordering comparison (-1, 0, 1) |
| [`PartialOrd`](../docs/builtin-macros/partial-ord) | `partialCompare(other: T): number | null` | Partial ordering comparison |
| [`Serialize`](../docs/builtin-macros/serialize) | `toJSON(): Record<string, unknown>` | JSON serialization with type handling |
| [`Deserialize`](../docs/builtin-macros/deserialize) | `static fromJSON(data: unknown): T` | JSON deserialization with validation |
 ## Using Built-in Macros
 Built-in macros don't require imports. Just use them with `@derive`:
```

/\*_ @derive(Debug, Clone, PartialEq) _/
class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}
}

```## Interface Support
 All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using `self` as the first parameter:
```

/\*_ @derive(Debug, Clone, PartialEq) _/
interface Point {
x: number;
y: number;
}

// Generated namespace:
// namespace Point {
// export function toString(self: Point): string { ... }
// export function clone(self: Point): Point { ... }
// export function equals(self: Point, other: Point): boolean { ... }
// export function hashCode(self: Point): number { ... }
// }

const point: Point = { x: 10, y: 20 };

// Use the namespace functions
console.log(Point.toString(point)); // "Point { x: 10, y: 20 }"
const copy = Point.clone(point); // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true

```## Enum Support
 All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:
```

/\*_ @derive(Debug, Clone, PartialEq, Serialize, Deserialize) _/
enum Status {
Active = "active",
Inactive = "inactive",
Pending = "pending",
}

// Generated namespace:
// namespace Status {
// export function toString(value: Status): string { ... }
// export function clone(value: Status): Status { ... }
// export function equals(a: Status, b: Status): boolean { ... }
// export function hashCode(value: Status): number { ... }
// export function toJSON(value: Status): string | number { ... }
// export function fromJSON(data: unknown): Status { ... }
// }

// Use the namespace functions
console.log(Status.toString(Status.Active)); // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending); // "pending"
const parsed = Status.fromJSON("active"); // Status.Active

```## Type Alias Support
 All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:
```

/\*_ @derive(Debug, Clone, PartialEq, Serialize, Deserialize) _/
type Point = {
x: number;
y: number;
};

// Generated namespace:
// namespace Point {
// export function toString(value: Point): string { ... }
// export function clone(value: Point): Point { ... }
// export function equals(a: Point, b: Point): boolean { ... }
// export function hashCode(value: Point): number { ... }
// export function toJSON(value: Point): Record<string, unknown> { ... }
// export function fromJSON(data: unknown): Point { ... }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point)); // "Point { x: 10, y: 20 }"
const copy = Point.clone(point); // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true

```Union type aliases also work, using JSON-based implementations:

```

/\*_ @derive(Debug, PartialEq) _/
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\"success\")"
console.log(ApiStatus.equals("success", "success")); // true

```## Combining Macros
 All macros can be used together. They don't conflict and each generates independent methods:
```

const user = new User("Alice", 30);

// Debug
console.log(user.toString());
// "User { name: Alice, age: 30 }"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true

````## Detailed Documentation
 Each macro has its own options and behaviors:
 - [**Debug**](../docs/builtin-macros/debug) - Customizable field renaming and skipping
 - [**Clone**](../docs/builtin-macros/clone) - Deep copying for all field types
 - [**Default**](../docs/builtin-macros/default) - Default value generation with field attributes
 - [**Hash**](../docs/builtin-macros/hash) - Hash code generation for use in maps and sets
 - [**PartialEq**](../docs/builtin-macros/partial-eq) - Value-based equality comparison
 - [**Ord**](../docs/builtin-macros/ord) - Total ordering for sorting
 - [**PartialOrd**](../docs/builtin-macros/partial-ord) - Partial ordering comparison
 - [**Serialize**](../docs/builtin-macros/serialize) - JSON serialization with serde-style options
 - [**Deserialize**](../docs/builtin-macros/deserialize) - JSON deserialization with validation

---

# Debug

The `Debug` macro generates a human-readable `toString()` method for
TypeScript classes, interfaces, enums, and type aliases.

## Generated Output

**Classes**: Generates an instance method returning a string
like `"ClassName { field1: value1, field2: value2 }"`.

**Enums**: Generates a standalone function `toStringEnumName(value)` that performs
reverse lookup on numeric enums.

**Interfaces**: Generates a standalone function `toStringInterfaceName(value)`.

**Type Aliases**: Generates a standalone function using JSON.stringify for
complex types, or field enumeration for object types.

## Field-Level Options

The `@debug` decorator supports:

- `skip` - Exclude the field from debug output
- `rename = "label"` - Use a custom label instead of the field name

## Example

```typescript before
/** @derive(Debug) */
class User {
    /** @debug({ rename: "id" }) */
    userId: number;

    /** @debug({ skip: true }) */
    password: string;

    email: string;
}
````

```typescript after
class User {
    userId: number;

    password: string;

    email: string;

    toString(): string {
        const parts: string[] = [];
        parts.push("id: " + this.userId);
        parts.push("email: " + this.email);
        return "User { " + parts.join(", ") + " }";
    }
}
```

Generated output:

```typescript
class User {
    userId: number;

    password: string;

    email: string;

    toString(): string {
        const parts: string[] = [];
        parts.push("id: " + this.userId);
        parts.push("email: " + this.email);
        return "User { " + parts.join(", ") + " }";
    }
}
```

---

# Clone

The `Clone` macro generates a `clone()` method for deep copying objects.
This is analogous to Rust's `Clone` trait, providing a way to create
independent copies of values.

## Generated Output

| Type       | Generated Code                                            | Description                                                     |
| ---------- | --------------------------------------------------------- | --------------------------------------------------------------- |
| Class      | `clone(): ClassName`                                      | Instance method creating a new instance with copied fields      |
| Enum       | `cloneEnumName(value: EnumName): EnumName`                | Standalone function (enums are primitives, returns value as-is) |
| Interface  | `cloneInterfaceName(value: InterfaceName): InterfaceName` | Standalone function creating a new object literal               |
| Type Alias | `cloneTypeName(value: TypeName): TypeName`                | Standalone function with spread copy for objects                |

## Cloning Strategy

The generated clone performs a **shallow copy** of all fields:

- **Primitives** (`string`, `number`, `boolean`): Copied by value
- **Objects**: Reference is copied (not deep cloned)
- **Arrays**: Reference is copied (not deep cloned)

For deep cloning of nested objects, those objects should also derive `Clone`
and the caller should clone them explicitly.

## Example

```typescript before
/** @derive(Clone) */
class Point {
    x: number;
    y: number;
}

const p1 = new Point();
const p2 = p1.clone(); // Creates a new Point with same values
```

```typescript after
class Point {
    x: number;
    y: number;

    clone(): Point {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.x = this.x;
        cloned.y = this.y;
        return cloned;
    }
}

const p1 = new Point();
const p2 = p1.clone(); // Creates a new Point with same values
```

Generated output:

```typescript
class Point {
    x: number;
    y: number;

    clone(): Point {
        const cloned = Object.create(Object.getPrototypeOf(this));
        cloned.x = this.x;
        cloned.y = this.y;
        return cloned;
    }
}

const p1 = new Point();
const p2 = p1.clone(); // Creates a new Point with same values
```

## Implementation Notes

- **Classes**: Uses `Object.create(Object.getPrototypeOf(this))` to preserve
  the prototype chain, ensuring `instanceof` checks work correctly
- **Enums**: Simply returns the value (enums are primitives in TypeScript)
- **Interfaces/Type Aliases**: Creates new object literals with spread operator
  for union/tuple types, or field-by-field copy for object types

---

# Default

The `Default` macro generates a static `defaultValue()` factory method that creates
instances with default values. This is analogous to Rust's `Default` trait, providing
a standard way to create "zero" or "empty" instances of types.

## Generated Output

| Type       | Generated Code                               | Description                                       |
| ---------- | -------------------------------------------- | ------------------------------------------------- |
| Class      | `static defaultValue(): ClassName`           | Static factory method                             |
| Enum       | `defaultValueEnumName(): EnumName`           | Standalone function returning marked variant      |
| Interface  | `defaultValueInterfaceName(): InterfaceName` | Standalone function returning object literal      |
| Type Alias | `defaultValueTypeName(): TypeName`           | Standalone function with type-appropriate default |

## Default Values by Type

The macro uses Rust-like default semantics:

| Type         | Default Value                           |
| ------------ | --------------------------------------- |
| `string`     | `""` (empty string)                     |
| `number`     | `0`                                     |
| `boolean`    | `false`                                 |
| `bigint`     | `0n`                                    |
| `T[]`        | `[]` (empty array)                      |
| `Array<T>`   | `[]` (empty array)                      |
| `Map<K,V>`   | `new Map()`                             |
| `Set<T>`     | `new Set()`                             |
| `Date`       | `new Date()` (current time)             |
| `T \| null`  | `null`                                  |
| `CustomType` | `CustomType.defaultValue()` (recursive) |

## Field-Level Options

The `@default` decorator allows specifying explicit default values:

- `@default(42)` - Use 42 as the default
- `@default("hello")` - Use "hello" as the default
- `@default([])` - Use empty array as the default
- `@default({ value: "test" })` - Named form for complex values

## Example

```typescript before
/** @derive(Default) */
class UserSettings {
    /** @default({ "light" }) */
    theme: string;

    /** @default({ 10: true }) */
    pageSize: number;

    notifications: boolean; // Uses type default: false
}
```

```typescript after
class UserSettings {

    theme: string;


    pageSize: number;

    notifications: boolean;  // Uses type default: false

    static defaultValue(): UserSettings {
    const instance = new UserSettings();
    instance.theme = {
        "light": <invalid>
    };
    instance.pageSize = {
        10: true
    };
    instance.notifications = false;
    return instance;
}
}
```

Generated output:

```typescript
class UserSettings {

    theme: string;


    pageSize: number;

    notifications: boolean;  // Uses type default: false

    static defaultValue(): UserSettings {
    const instance = new UserSettings();
    instance.theme = {
        "light": <invalid>
    };
    instance.pageSize = {
        10: true
    };
    instance.notifications = false;
    return instance;
}
}
```

## Enum Defaults

For enums, mark one variant with `@default`:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

Generated output:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

Generated output:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

Generated output:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

Generated output:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

Generated output:

```typescript before
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

```typescript after
/** @derive(Default) */
enum Status {
    @default
    Pending,
    Active,
    Completed
}
```

## Error Handling

The macro will return an error if:

- A non-primitive field lacks `@default` and has no known default
- An enum has no variant marked with `@default`
- A union type has no `@default` on a variant

---

# Hash

The `Hash` macro generates a `hashCode()` method for computing numeric hash codes.
This is analogous to Rust's `Hash` trait and Java's `hashCode()` method, enabling
objects to be used as keys in hash-based collections.

## Generated Output

| Type       | Generated Code                                        | Description                                    |
| ---------- | ----------------------------------------------------- | ---------------------------------------------- |
| Class      | `hashCode(): number`                                  | Instance method computing hash from all fields |
| Enum       | `hashCodeEnumName(value: EnumName): number`           | Standalone function hashing by enum value      |
| Interface  | `hashCodeInterfaceName(value: InterfaceName): number` | Standalone function computing hash             |
| Type Alias | `hashCodeTypeName(value: TypeName): number`           | Standalone function computing hash             |

## Hash Algorithm

Uses the standard polynomial rolling hash algorithm:

```text
hash = 17  // Initial seed
for each field:
    hash = (hash * 31 + fieldHash) | 0  // Bitwise OR keeps it 32-bit integer
```

This algorithm is consistent with Java's `Objects.hash()` implementation.

## Type-Specific Hashing

| Type      | Hash Strategy                                          |
| --------- | ------------------------------------------------------ |
| `number`  | Integer: direct value; Float: string hash of decimal   |
| `bigint`  | String hash of decimal representation                  |
| `string`  | Character-by-character polynomial hash                 |
| `boolean` | 1231 for true, 1237 for false (Java convention)        |
| `Date`    | `getTime()` timestamp                                  |
| Arrays    | Element-by-element hash combination                    |
| `Map`     | Entry-by-entry key+value hash                          |
| `Set`     | Element-by-element hash                                |
| Objects   | Calls `hashCode()` if available, else JSON string hash |

## Field-Level Options

The `@hash` decorator supports:

- `skip` - Exclude the field from hash calculation

## Example

```typescript before
/** @derive(Hash, PartialEq) */
class User {
    id: number;
    name: string;

    @hash(skip) // Cached value shouldn't affect hash
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Cached value shouldn't affect hash
    cachedScore: number;

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        return hash;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return (
            this.id === typedOther.id &&
            this.name === typedOther.name &&
            this.cachedScore === typedOther.cachedScore
        );
    }
}
```

Generated output:

```typescript
class User {
    id: number;
    name: string;

    // Cached value shouldn't affect hash
    cachedScore: number;

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        return hash;
    }

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return (
            this.id === typedOther.id &&
            this.name === typedOther.name &&
            this.cachedScore === typedOther.cachedScore
        );
    }
}
```

## Hash Contract

Objects that are equal (`PartialEq`) should produce the same hash code.
When using `@hash(skip)`, ensure the same fields are skipped in both
`Hash` and `PartialEq` to maintain this contract.

---

# Ord

The `Ord` macro generates a `compareTo()` method for **total ordering** comparison.
This is analogous to Rust's `Ord` trait, enabling objects to be sorted and
compared with a guaranteed ordering relationship.

## Generated Output

| Type       | Generated Code                                                     | Description                                          |
| ---------- | ------------------------------------------------------------------ | ---------------------------------------------------- |
| Class      | `compareTo(other): number`                                         | Instance method returning -1, 0, or 1                |
| Enum       | `compareEnumName(a: EnumName, b: EnumName): number`                | Standalone function comparing enum values            |
| Interface  | `compareInterfaceName(a: InterfaceName, b: InterfaceName): number` | Standalone function comparing fields                 |
| Type Alias | `compareTypeName(a: TypeName, b: TypeName): number`                | Standalone function with type-appropriate comparison |

## Return Values

Unlike `PartialOrd`, `Ord` provides **total ordering** - every pair of values
can be compared:

- **-1**: `this` is less than `other`
- **0**: `this` is equal to `other`
- **1**: `this` is greater than `other`

The method **never returns null** - all values must be comparable.

## Comparison Strategy

Fields are compared **lexicographically** in declaration order:

1. Compare first field
2. If not equal, return that result
3. Otherwise, compare next field
4. Continue until a difference is found or all fields are equal

## Type-Specific Comparisons

| Type              | Comparison Method                        |
| ----------------- | ---------------------------------------- |
| `number`/`bigint` | Direct `<` and `>` comparison            |
| `string`          | `localeCompare()` (clamped to -1, 0, 1)  |
| `boolean`         | false &lt; true                          |
| Arrays            | Lexicographic element-by-element         |
| `Date`            | `getTime()` timestamp comparison         |
| Objects           | Calls `compareTo()` if available, else 0 |

## Field-Level Options

The `@ord` decorator supports:

- `skip` - Exclude the field from ordering comparison

## Example

```typescript before
/** @derive(Ord) */
class Version {
    major: number;
    minor: number;
    patch: number;
}

// Usage:
versions.sort((a, b) => a.compareTo(b));
```

```typescript after
class Version {
    major: number;
    minor: number;
    patch: number;

    compareTo(other: Version): number {
        if (this === other) return 0;
        const typedOther = other;
        const cmp0 =
            this.major < typedOther.major
                ? -1
                : this.major > typedOther.major
                  ? 1
                  : 0;
        if (cmp0 !== 0) return cmp0;
        const cmp1 =
            this.minor < typedOther.minor
                ? -1
                : this.minor > typedOther.minor
                  ? 1
                  : 0;
        if (cmp1 !== 0) return cmp1;
        const cmp2 =
            this.patch < typedOther.patch
                ? -1
                : this.patch > typedOther.patch
                  ? 1
                  : 0;
        if (cmp2 !== 0) return cmp2;
        return 0;
    }
}

// Usage:
versions.sort((a, b) => a.compareTo(b));
```

Generated output:

```typescript
class Version {
    major: number;
    minor: number;
    patch: number;

    compareTo(other: Version): number {
        if (this === other) return 0;
        const typedOther = other;
        const cmp0 =
            this.major < typedOther.major
                ? -1
                : this.major > typedOther.major
                  ? 1
                  : 0;
        if (cmp0 !== 0) return cmp0;
        const cmp1 =
            this.minor < typedOther.minor
                ? -1
                : this.minor > typedOther.minor
                  ? 1
                  : 0;
        if (cmp1 !== 0) return cmp1;
        const cmp2 =
            this.patch < typedOther.patch
                ? -1
                : this.patch > typedOther.patch
                  ? 1
                  : 0;
        if (cmp2 !== 0) return cmp2;
        return 0;
    }
}

// Usage:
versions.sort((a, b) => a.compareTo(b));
```

## Ord vs PartialOrd

- Use **Ord** when all values are comparable (total ordering)
- Use **PartialOrd** when some values may be incomparable (returns `Option<number>`)

---

# PartialEq

The `PartialEq` macro generates an `equals()` method for field-by-field
structural equality comparison. This is analogous to Rust's `PartialEq` trait,
enabling value-based equality semantics instead of reference equality.

## Generated Output

| Type       | Generated Code                                                     | Description                                          |
| ---------- | ------------------------------------------------------------------ | ---------------------------------------------------- |
| Class      | `equals(other: unknown): boolean`                                  | Instance method with instanceof check                |
| Enum       | `equalsEnumName(a: EnumName, b: EnumName): boolean`                | Standalone function using strict equality            |
| Interface  | `equalsInterfaceName(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields                 |
| Type Alias | `equalsTypeName(a: TypeName, b: TypeName): boolean`                | Standalone function with type-appropriate comparison |

## Comparison Strategy

The generated equality check:

1. **Identity check**: `this === other` returns true immediately
2. **Type check**: For classes, uses `instanceof`; returns false if wrong type
3. **Field comparison**: Compares each non-skipped field

## Type-Specific Comparisons

| Type       | Comparison Method                         |
| ---------- | ----------------------------------------- |
| Primitives | Strict equality (`===`)                   |
| Arrays     | Length + element-by-element (recursive)   |
| `Date`     | `getTime()` comparison                    |
| `Map`      | Size + entry-by-entry comparison          |
| `Set`      | Size + membership check                   |
| Objects    | Calls `equals()` if available, else `===` |

## Field-Level Options

The `@partialEq` decorator supports:

- `skip` - Exclude the field from equality comparison

## Example

```typescript before
/** @derive(PartialEq, Hash) */
class User {
    id: number;
    name: string;

    @partialEq(skip) // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

Generated output:

```typescript before
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

```typescript after
class User {
    id: number;
    name: string;

    // Don't compare cached values
    /** @hash({ skip: true }) */
    cachedScore: number;

    equals(other: unknown): boolean {
        if (this === other) return true;
        if (!(other instanceof User)) return false;
        const typedOther = other as User;
        return this.id === typedOther.id && this.name === typedOther.name;
    }

    hashCode(): number {
        let hash = 17;
        hash =
            (hash * 31 +
                (Number.isInteger(this.id)
                    ? this.id | 0
                    : this.id
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        hash =
            (hash * 31 +
                (this.name ?? "")
                    .split("")
                    .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
            0;
        hash =
            (hash * 31 +
                (Number.isInteger(this.cachedScore)
                    ? this.cachedScore | 0
                    : this.cachedScore
                          .toString()
                          .split("")
                          .reduce(
                              (h, c) => (h * 31 + c.charCodeAt(0)) | 0,
                              0,
                          ))) |
            0;
        return hash;
    }
}
```

## Equality Contract

When implementing `PartialEq`, consider also implementing `Hash`:

- **Reflexivity**: `a.equals(a)` is always true
- **Symmetry**: `a.equals(b)` implies `b.equals(a)`
- **Hash consistency**: Equal objects must have equal hash codes

---

# PartialOrd

The `PartialOrd` macro generates a `compareTo()` method for **partial ordering**
comparison. This is analogous to Rust's `PartialOrd` trait, enabling comparison
between values where some pairs may be incomparable.

## Generated Output

| Type       | Generated Code                                                                    | Description                          |
| ---------- | --------------------------------------------------------------------------------- | ------------------------------------ |
| Class      | `compareTo(other): Option<number>`                                                | Instance method with optional result |
| Enum       | `partialCompareEnumName(a: EnumName, b: EnumName): Option<number>`                | Standalone function returning Option |
| Interface  | `partialCompareInterfaceName(a: InterfaceName, b: InterfaceName): Option<number>` | Standalone function with Option      |
| Type Alias | `partialCompareTypeName(a: TypeName, b: TypeName): Option<number>`                | Standalone function with Option      |

## Return Values

Unlike `Ord`, `PartialOrd` returns an `Option<number>` to handle incomparable values:

- **Option.some(-1)**: `this` is less than `other`
- **Option.some(0)**: `this` is equal to `other`
- **Option.some(1)**: `this` is greater than `other`
- **Option.none()**: Values are incomparable

## When to Use PartialOrd vs Ord

- **PartialOrd**: When some values may not be comparable
    - Example: Floating-point NaN values
    - Example: Mixed-type unions
    - Example: Type mismatches between objects

- **Ord**: When all values are guaranteed comparable (total ordering)

## Comparison Strategy

Fields are compared **lexicographically** in declaration order:

1. Compare first field
2. If incomparable, return `Option.none()`
3. If not equal, return that result wrapped in `Option.some()`
4. Otherwise, compare next field
5. Continue until a difference is found or all fields are equal

## Type-Specific Comparisons

| Type              | Comparison Method                                         |
| ----------------- | --------------------------------------------------------- |
| `number`/`bigint` | Direct comparison, returns some()                         |
| `string`          | `localeCompare()` wrapped in some()                       |
| `boolean`         | false &lt; true, wrapped in some()                        |
| null/undefined    | Returns none() for mismatched nullability                 |
| Arrays            | Lexicographic, propagates none() on incomparable elements |
| `Date`            | Timestamp comparison, none() if invalid                   |
| Objects           | Unwraps nested Option from compareTo()                    |

## Field-Level Options

The `@ord` decorator supports:

- `skip` - Exclude the field from ordering comparison

## Example

```typescript before
/** @derive(PartialOrd) */
class Temperature {
    value: number | null; // null represents "unknown"
    unit: string;
}
```

```typescript after
import { Option } from "macroforge/utils";

class Temperature {
    value: number | null; // null represents "unknown"
    unit: string;

    compareTo(other: unknown): Option<number> {
        if (this === other) return Option.some(0);
        if (!(other instanceof Temperature)) return Option.none();
        const typedOther = other as Temperature;
        const cmp0 = (() => {
            if (typeof (this.value as any)?.compareTo === "function") {
                const optResult = (this.value as any).compareTo(
                    typedOther.value,
                );
                return Option.isNone(optResult) ? null : optResult.value;
            }
            return this.value === typedOther.value ? 0 : null;
        })();
        if (cmp0 === null) return Option.none();
        if (cmp0 !== 0) return Option.some(cmp0);
        const cmp1 = this.unit.localeCompare(typedOther.unit);
        if (cmp1 === null) return Option.none();
        if (cmp1 !== 0) return Option.some(cmp1);
        return Option.some(0);
    }
}
```

Generated output:

```typescript
class Temperature {
    value: number | null; // null represents "unknown"
    unit: string;

    compareTo(other: unknown): Option<number> {
        if (this === other) return Option.some(0);
        if (!(other instanceof Temperature)) return Option.none();
        const typedOther = other as Temperature;
        const cmp0 = (() => {
            if (typeof (this.value as any)?.compareTo === "function") {
                const optResult = (this.value as any).compareTo(
                    typedOther.value,
                );
                return Option.isNone(optResult) ? null : optResult.value;
            }
            return this.value === typedOther.value ? 0 : null;
        })();
        if (cmp0 === null) return Option.none();
        if (cmp0 !== 0) return Option.some(cmp0);
        const cmp1 = this.unit.localeCompare(typedOther.unit);
        if (cmp1 === null) return Option.none();
        if (cmp1 !== 0) return Option.some(cmp1);
        return Option.some(0);
    }
}
```

## Required Import

The generated code automatically adds an import for `Option` from `macroforge/utils`.

---

# Serialize

The `Serialize` macro generates JSON serialization methods with **cycle detection**
and object identity tracking. This enables serialization of complex object graphs
including circular references.

## Generated Methods

| Type       | Generated Code                                | Description          |
| ---------- | --------------------------------------------- | -------------------- |
| Class      | `serialize()`, `SerializeWithContext(ctx)`             | Instance methods     |
| Enum       | `myEnumSerialize(value)`, `myEnumSerializeWithContext` | Standalone functions |
| Interface  | `myInterfaceSerialize(value)`, etc.           | Standalone functions |
| Type Alias | `myTypeSerialize(value)`, etc.                | Standalone functions |

## Cycle Detection Protocol

The generated code handles circular references using `__id` and `__ref` markers:

```json
{
    "__type": "User",
    "__id": 1,
    "name": "Alice",
    "friend": { "__ref": 2 } // Reference to object with __id: 2
}
```

When an object is serialized:

1. Check if it's already been serialized (has an `__id`)
2. If so, return `{ "__ref": existingId }` instead
3. Otherwise, register the object and serialize its fields

## Type-Specific Serialization

| Type       | Serialization Strategy                                                                                            |
| ---------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Primitives | Direct value                                                                                                      |
| `Date`     | `toISOString()`                                                                                                   |
| Arrays     | For primitive-like element types, pass through; for `Date`/`Date                                                  | null`, map to ISO strings; otherwise map and call `\_\_serialize(ctx)` when available       |
| `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date                                  | null`, convert to ISO strings; otherwise call `\_\_serialize(ctx)` per value when available |
| `Set<T>`   | Convert to array; element handling matches `Array<T>`                                                             |
| Nullable   | Include `null` explicitly; for primitive-like and `Date` unions the generator avoids runtime `SerializeWithContext` checks |
| Objects    | Call `SerializeWithContext(ctx)` if available (to support user-defined implementations)                                    |

Note: the generator specializes some code paths based on the declared TypeScript type to
avoid runtime feature detection on primitives and literal unions.

## Field-Level Options

The `@serde` decorator supports:

- `skip` / `skipSerializing` - Exclude field from serialization
- `rename = "jsonKey"` - Use different JSON property name
- `flatten` - Merge nested object's fields into parent

## Example

```typescript before
/** @derive(Serialize) */
class User {
    id: number;

    /** @serde({ rename: "userName" }) */
    name: string;

    /** @serde({ skipSerializing: true }) */
    password: string;

    /** @serde({ flatten: true }) */
    metadata: UserMetadata;
}

// Usage:
const user = new User();
const json = user.serialize();
// => '{"__type":"User","__id":1,"id":1,"userName":"Alice",...}'
```

```typescript after
import { SerializeContext } from "macroforge/serde";

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;

    toStringifiedJSON(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.serializeWithContext(ctx));
    }

    toObject(): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return this.serializeWithContext(ctx);
    }

    serializeWithContext(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId,
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: "User",
            __id,
        };
        result["id"] = this.id;
        result["userName"] = this.name;
        result["password"] = this.password;
        {
            const __flattened = userMetadataSerializeWithContext(this.metadata, ctx);
            const { __type: _, __id: __, ...rest } = __flattened as any;
            Object.assign(result, rest);
        }
        return result;
    }
}

// Usage:
const user = new User();
const json = user.serialize();
// => '{"__type":"User","__id":1,"id":1,"userName":"Alice",...}'
```

Generated output:

```typescript
import { SerializeContext } from "macroforge/serde";

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;

    /**
     * Serializes this instance to a JSON string.
     * @returns JSON string representation with cycle detection metadata
     */
    serialize(): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(this.serializeWithContext(ctx));
    }

    /** @internal */
    serializeWithContext(ctx: SerializeContext): Record<string, unknown> {
        const existingId = ctx.getId(this);
        if (existingId !== undefined) {
            return {
                __ref: existingId,
            };
        }
        const __id = ctx.register(this);
        const result: Record<string, unknown> = {
            __type: "User",
            __id,
        };
        result["id"] = this.id;
        result["userName"] = this.name;
        {
            const __flattened = userMetadataSerializeWithContext(this.metadata, ctx);
            const { __type: _, __id: __, ...rest } = __flattened as any;
            Object.assign(result, rest);
        }
        return result;
    }
}

// Usage:
const user = new User();
const json = user.serialize();
// => '{"__type":"User","__id":1,"id":1,"userName":"Alice",...}'
```

## Required Import

The generated code automatically imports `SerializeContext` from `macroforge/serde`.

---

# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and
forward-reference support**, plus comprehensive runtime validation. This enables
safe parsing of complex JSON structures including circular references.

## Generated Output

| Type       | Generated Code                                                                      | Description            |
| ---------- | ----------------------------------------------------------------------------------- | ---------------------- |
| Class      | `static deserialize()`, `static deserializeWithContext()`                           | Static factory methods |
| Enum       | `myEnumDeserialize(input)`, `myEnumDeserializeWithContext(data)`, `myEnumIs(value)` | Standalone functions   |
| Interface  | `myInterfaceDeserialize(input)`, etc.                                               | Standalone functions   |
| Type Alias | `myTypeDeserialize(input)`, etc.                                                    | Standalone functions   |

## Return Type

All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:

- `Result.ok(value)` - Successfully deserialized value
- `Result.err(errors)` - Array of validation errors with field names and messages

## Cycle/Forward-Reference Support

Uses deferred patching to handle references:

1. When encountering `{ "__ref": id }`, returns a `PendingRef` marker
2. Continues deserializing other fields
3. After all objects are created, `ctx.applyPatches()` resolves all pending references

References only apply to object-shaped, serializable values. The generator avoids probing for
`__ref` on primitive-like fields (including literal unions and `T | null` where `T` is primitive-like),
and it parses `Date` / `Date | null` from ISO strings without treating them as references.

## Validation

The macro supports 30+ validators via `@serde(validate(...))`:

### String Validators

- `email`, `url`, `uuid` - Format validation
- `minLength(n)`, `maxLength(n)`, `length(n)` - Length constraints
- `pattern("regex")` - Regular expression matching
- `nonEmpty`, `trimmed`, `lowercase`, `uppercase` - String properties

### Number Validators

- `gt(n)`, `gte(n)`, `lt(n)`, `lte(n)`, `between(min, max)` - Range checks
- `int`, `positive`, `nonNegative`, `finite` - Number properties

### Array Validators

- `minItems(n)`, `maxItems(n)`, `itemsCount(n)` - Collection size

### Date Validators

- `validDate`, `afterDate("ISO")`, `beforeDate("ISO")` - Date validation

## Field-Level Options

The `@serde` decorator supports:

- `skip` / `skipDeserializing` - Exclude field from deserialization
- `rename = "jsonKey"` - Read from different JSON property
- `default` / `default = expr` - Use default value if missing
- `flatten` - Read fields from parent object level
- `validate(...)` - Apply validators

## Container-Level Options

- `denyUnknownFields` - Error on unrecognized JSON properties
- `renameAll = "camelCase"` - Apply naming convention to all fields

## Union Type Deserialization

Union types are deserialized based on their member types:

### Literal Unions

For unions of literal values (`"A" | "B" | 123`), the value is validated against
the allowed literals directly.

### Primitive Unions

For unions containing primitive types (`string | number`), the deserializer uses
`typeof` checks to validate the value type. No `__type` discriminator is needed.

### Class/Interface Unions

For unions of serializable types (`User | Admin`), the deserializer requires a
`__type` field in the JSON to dispatch to the correct type's `DeserializeWithContext` method.

### Generic Type Parameters

For generic unions like `type Result<T> = T | Error`, the generic type parameter `T`
is passed through as-is since its concrete type is only known at the call site.

### Mixed Unions

Mixed unions (e.g., `string | Date | User`) check in order:

1. Literal values
2. Primitives (via `typeof`)
3. Date (via `instanceof` or ISO string parsing)
4. Serializable types (via `__type` dispatch)
5. Generic type parameters (pass-through)

## Example

```typescript before
/** @derive(Deserialize) */
/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    @serde(validate(email, maxLength(255)))
    email: string;

    /** @serde({ default: "guest" }) */
    name: string;

    @serde(validate(positive))
    age?: number;
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return User.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.fromObject: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.fromObject(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript before
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript before
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript before
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript before
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

Generated output:

```typescript before
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

```typescript after
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";

/** @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    email: string;

    name: string;

    age?: number;

    constructor(props: {
        id: number;
        email: string;
        name?: string;
        age?: number;
    }) {
        this.id = props.id;
        this.email = props.email;
        this.name = props.name as string;
        this.age = props.age as number;
    }

    /**
     * Deserializes input to an instance of this class.
     * Automatically detects whether input is a JSON string or object.
     * @param input - JSON string or object to deserialize
     * @param opts - Optional deserialization options
     * @returns Result containing the deserialized instance or validation errors
     */
    static deserialize(
        input: unknown,
        opts?: DeserializeOptions,
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === "string" ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: "_root",
                        message:
                            "User.deserialize: root cannot be a forward reference",
                    },
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return Result.ok(resultOrRef);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: "_root",
                    message,
                },
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(
        value: any,
        ctx: DeserializeContext,
    ): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (
            typeof value !== "object" ||
            value === null ||
            Array.isArray(value)
        ) {
            throw new DeserializeError([
                {
                    field: "_root",
                    message: "User.deserializeWithContext: expected an object",
                },
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set([
            "__type",
            "__id",
            "__ref",
            "id",
            "email",
            "name",
            "age",
        ]);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: "unknown field",
                });
            }
        }
        if (!("id" in obj)) {
            errors.push({
                field: "id",
                message: "missing required field",
            });
        }
        if (!("email" in obj)) {
            errors.push({
                field: "email",
                message: "missing required field",
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(User.prototype) as User;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj["id"] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj["email"] as string;
            instance.email = __raw_email;
        }
        if ("name" in obj && obj["name"] !== undefined) {
            const __raw_name = obj["name"] as string;
            instance.name = __raw_name;
        } else {
            instance.name = guest;
        }
        if ("age" in obj && obj["age"] !== undefined) {
            const __raw_age = obj["age"] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K],
    ): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static validateFields(partial: Partial<User>): Array<{
        field: string;
        message: string;
    }> {
        return [];
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return "id" in o && "email" in o;
    }

    static is(obj: unknown): obj is User {
        if (obj instanceof User) {
            return true;
        }
        if (!User.hasShape(obj)) {
            return false;
        }
        const result = User.deserialize(obj);
        return Result.isOk(result);
    }
}

// Usage:
const result = User.deserialize('{"id":1,"email":"test@example.com"}');
if (Result.isOk(result)) {
    const user = result.value;
} else {
    console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
}
```

## Required Imports

The generated code automatically imports:

- `Result` from `macroforge/utils`
- `DeserializeContext`, `DeserializeError`, `PendingRef` from `macroforge/serde`

---

# Custom Macros

# Custom Macros

_Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code._

## Overview

Custom macros are written in Rust and compiled to native Node.js addons. The process involves:

1.  Creating a Rust crate with NAPI bindings
2.  Defining macro functions with `#[ts_macro_derive]`
3.  Using `macroforge_ts_quote` to generate TypeScript code
4.  Building and publishing as an npm package

## Quick Example

````
use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
   JSON,
   description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
   let input = parse_ts_macro_input!(input as DeriveInput);

   match &input.data {
       Data::Class(class) => {
           Ok(body! {
               toJSON(): Record<string, unknown> {
                   return {
                       {#for field in class.field_names()}
                           @{field}: this.@{field},
                       {/for}
                   };
               }
           })
       }
       _ => Err(MacroforgeError::new(
           input.decorator_span(),
           "@derive(JSON) only works on classes",
       )),
   }
}
``` ## Using Custom Macros
Once your macro package is published, users can import and use it:
````

/\*_ import macro { JSON } from "@my/macros"; _/

/\*_ @derive(JSON) _/
class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}
}

const user = new User("Alice", 30);
console.log(user.toJSON()); // { name: "Alice", age: 30 }

```> **Note:** The import macro comment tells Macroforge which package provides the macro. ## Getting Started
 Follow these guides to create your own macros:
 - [Set up a Rust macro crate](../docs/custom-macros/rust-setup)
 - [Learn the #[ts_macro_derive] attribute](../docs/custom-macros/ts-macro-derive)
 - [Learn the template syntax](../docs/custom-macros/ts-quote)

---

# Rust Setup
 *Create a new Rust crate that will contain your custom macros. This crate compiles to a native Node.js addon.*
 ## Prerequisites
 - Rust toolchain (1.88 or later)
 - Node.js 24 or later
 - NAPI-RS CLI: `cargo install macroforge_ts`
 ## Create the Project
```

# Create a new directory

mkdir my-macros
cd my-macros

# Initialize with NAPI-RS

napi new --platform --name my-macros

```## Configure Cargo.toml
 Update your `Cargo.toml` with the required dependencies:
```

[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
macroforge_ts = "0.1"
napi = { version = "3", features = ["napi8", "compat-mode"] }
napi-derive = "3"

[build-dependencies]
napi-build = "2"

[profile.release]
lto = true
strip = true

```## Create build.rs

```

fn main() {
napi_build::setup();
}

```## Create src/lib.rs

```

use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{
Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

#[ts_macro_derive(
JSON,
description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            Ok(body! {
                toJSON(): Record<string, unknown> {
                    return {
                        {#for field in class.field_names()}
                            @{field}: this.@{field},
                        {/for}
                    };
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    }

}

```## Create package.json

```

{
"name": "@my-org/macros",
"version": "0.1.0",
"main": "index.js",
"types": "index.d.ts",
"napi": {
"name": "my-macros",
"triples": {
"defaults": true
}
},
"files": [
"index.js",
"index.d.ts",
"*.node"
],
"scripts": {
"build": "napi build --release",
"prepublishOnly": "napi build --release"
},
"devDependencies": {
"@napi-rs/cli": "^3.0.0-alpha.0"
}
}

```## Build the Package

```

# Build the native addon

npm run build

# This creates:

# - index.js (JavaScript bindings)

# - index.d.ts (TypeScript types)

# - \*.node (native binary)

```**Tip For cross-platform builds, use GitHub Actions with the NAPI-RS CI template. ## Next Steps
 - [Learn the #[ts_macro_derive] attribute](../../docs/custom-macros/ts-macro-derive)
 - [Master the template syntax](../../docs/custom-macros/ts-quote)
**

---

# ts_macro_derive
 *The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*
 ## Basic Syntax
```

use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
// Macro implementation
}

```## Attribute Options
 ### Name (Required)
 The first argument is the macro name that users will reference in `@derive()`:
```

#[ts_macro_derive(JSON)] // Users write: @derive(JSON)
pub fn derive_json(...)

```### Description
 Provides documentation for the macro:
```

#[ts_macro_derive(
JSON,
description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)

```### Attributes
 Declare which field-level decorators your macro accepts:
```

#[ts_macro_derive(
Debug,
description = "Generates toString()",
attributes(debug) // Allows @debug({ ... }) on fields
)]
pub fn derive_debug(...)

```> **Note:** Declared attributes become available as @attributeName({ options }) decorators in TypeScript. ## Function Signature

```

pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>

```| Parameter | Description |
| --- | --- |
| `input: TsStream` | Token stream containing the class/interface AST |
| `Result<TsStream, MacroforgeError>` | Returns generated code or an error with source location |
 ## Parsing Input
 Use `parse_ts_macro_input!` to convert the token stream:
```

use macroforge_ts::ts_syn::{Data, DeriveInput, parse_ts_macro_input};

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
let input = parse_ts_macro_input!(input as DeriveInput);

    // Access class data
    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.fields();
            // ...
        }
        Data::Interface(interface) => {
            // Handle interfaces
        }
        Data::Enum(_) => {
            // Handle enums (if supported)
        }
    }

}

```## DeriveInput Structure

```

struct DeriveInput {
pub ident: Ident, // The type name
pub span: SpanIR, // Span of the type definition
pub attrs: Vec<Attribute>, // Decorators (excluding @derive)
pub data: Data, // The parsed type data
pub context: MacroContextIR, // Macro context with spans

    // Helper methods
    fn name(&self) -> &str;              // Get the type name
    fn decorator_span(&self) -> SpanIR;  // Span of @derive decorator
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;

}

enum Data {
Class(DataClass),
Interface(DataInterface),
Enum(DataEnum),
TypeAlias(DataTypeAlias),
}

impl DataClass {
fn fields(&self) -> &[FieldIR];
fn methods(&self) -> &[MethodSigIR];
fn field_names(&self) -> impl Iterator<Item = &str>;
fn field(&self, name: &str) -> Option<&FieldIR>;
fn body_span(&self) -> SpanIR; // For inserting code into class body
fn type_params(&self) -> &[String]; // Generic type parameters
fn heritage(&self) -> &[String]; // extends/implements clauses
fn is_abstract(&self) -> bool;
}

impl DataInterface {
fn fields(&self) -> &[InterfaceFieldIR];
fn methods(&self) -> &[InterfaceMethodIR];
fn field_names(&self) -> impl Iterator<Item = &str>;
fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
fn body_span(&self) -> SpanIR;
fn type_params(&self) -> &[String];
fn heritage(&self) -> &[String]; // extends clauses
}

impl DataEnum {
fn variants(&self) -> &[EnumVariantIR];
fn variant_names(&self) -> impl Iterator<Item = &str>;
fn variant(&self, name: &str) -> Option<&EnumVariantIR>;
}

impl DataTypeAlias {
fn body(&self) -> &TypeBody;
fn type_params(&self) -> &[String];
fn is_union(&self) -> bool;
fn is_object(&self) -> bool;
fn as_union(&self) -> Option<&[TypeMember]>;
fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
}

```## Accessing Field Data
 ### Class Fields (FieldIR)
```

struct FieldIR {
pub name: String, // Field name
pub span: SpanIR, // Field span
pub ts_type: String, // TypeScript type annotation
pub optional: bool, // Whether field has ?
pub readonly: bool, // Whether field is readonly
pub visibility: Visibility, // Public, Protected, Private
pub decorators: Vec<DecoratorIR>, // Field decorators
}

```### Interface Fields (InterfaceFieldIR)

```

struct InterfaceFieldIR {
pub name: String,
pub span: SpanIR,
pub ts_type: String,
pub optional: bool,
pub readonly: bool,
pub decorators: Vec<DecoratorIR>,
// Note: No visibility field (interfaces are always public)
}

```### Enum Variants (EnumVariantIR)

```

struct EnumVariantIR {
pub name: String,
pub span: SpanIR,
pub value: EnumValue, // Auto, String(String), or Number(f64)
pub decorators: Vec<DecoratorIR>,
}

```### Decorator Structure

```

struct DecoratorIR {
pub name: String, // e.g., "serde"
pub args_src: String, // Raw args text, e.g., "skip, rename: 'id'"
pub span: SpanIR,
}

```> **Note:** To check for decorators, iterate through field.decorators and check decorator.name. For parsing options, you can write helper functions like the built-in macros do. ## Adding Imports
 If your macro generates code that requires imports, use the `add_import` method on `TsStream`:
```

// Add an import to be inserted at the top of the file
let mut output = body! {
validate(): ValidationResult {
return validateFields(this);
}
};

// This will add: import { validateFields, ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_import("ValidationResult", "my-validation-lib");

Ok(output)

```> **Note:** Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again. ## Returning Errors
 Use `MacroforgeError` to report errors with source locations:
```

#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(body! { /* ... */ })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }

}

```## Complete Example

```

use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{
Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
};

// Helper function to check if a field has a decorator
fn has_decorator(field: &FieldIR, name: &str) -> bool {
field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
Validate,
description = "Generates a validate() method",
attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(body! {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Validate) only works on classes",
        )),
    }

}

```## Next Steps
 - [Learn the template syntax](../../docs/custom-macros/ts-quote)

---

# Template Syntax
 *The `macroforge_ts_quote` crate provides template-based code generation for TypeScript. The `ts_template!` macro uses Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*
 ## Available Macros
 | Macro | Output | Use Case |
| --- | --- | --- |
| `ts_template!` | Any TypeScript code | General code generation |
| `body!` | Class body members | Methods and properties |
 ## Quick Reference
 | Syntax | Description |
| --- | --- |
| `@{expr}` | Interpolate a Rust expression (adds space after) |
| `{| content |}` | Ident block: concatenates without spaces (e.g., `{|get@{name}|}` → `getUser`) |
| `{> "comment" <}` | Block comment: outputs `/* comment */` (string preserves whitespace) |
| `{>> "doc" <<}` | Doc comment: outputs `/** doc */` (string preserves whitespace) |
| `@@{` | Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`) |
| `"text @{expr}"` | String interpolation (auto-detected) |
| `"'^template ${js}^'"` | JS backtick template literal (outputs ``template ${js}``) |
| `{#if cond}...{/if}` | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| {#if a}...{:else if b}...{:else}...{/if} | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| {#match expr}{:case pattern}...{/match} | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| `{#while cond}...{/while}` | While loop |
| `{#while let pattern = expr}...{/while}` | While-let pattern matching loop |
| `{$let name = expr}` | Define a local constant |
| `{$let mut name = expr}` | Define a mutable local variable |
| `{$do expr}` | Execute a side-effectful expression |
| `{$typescript stream}` | Inject a TsStream, preserving its source and runtime_patches (imports) |
 **Note:** A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com` works as expected).
 ## Interpolation: `@{expr}`
 Insert Rust expressions into the generated TypeScript:
```

let class_name = "User";
let method = "toString";

let code = ts_template! {
@{class_name}.prototype.@{method} = function() {
return "User instance";
};
};

```**Generates:**

```

User.prototype.toString = function () {
return "User instance";
};
```## Identifier Concatenation:`{| content |}` When you need to build identifiers dynamically (like`getUser`, `setName`), use the ident block syntax. Everything inside `{| |}` is concatenated without spaces:

````
let field_name = "User";

let code = ts_template! {
   function {|get@{field_name}|}() {
       return this.@{field_name.to_lowercase()};
   }
};
``` **Generates:**
````

function getUser() {
return this.user;
}
```Without ident blocks,`@{}`always adds a space after for readability. Use`{| |}` when you explicitly want concatenation:

````
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
``` Multiple interpolations can be combined:
````

let entity = "user";
let action = "create";

ts*template! { {|@{entity}*@{action}|} } // → "user_create"
```## Comments:`{> "..." <}`and`{>> "..." <<}`
Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:

### Block Comments

Use `{> "comment" <}` for block comments:

````
let code = ts_template! {
   {> "This is a block comment" <}
   const x = 42;
};
``` **Generates:**
````

/_ This is a block comment _/
const x = 42;

```### Doc Comments (JSDoc)
 Use `{>> "doc" <<}` for JSDoc comments:
```

let code = ts_template! {
{>> "@param {string} name - The user's name" <<}
{>> "@returns {string} A greeting message" <<}
function greet(name: string): string {
return "Hello, " + name;
}
};

```**Generates:**

```

/** @param {string} name - The user's name \*/
/** @returns {string} A greeting message \*/
function greet(name: string): string {
return "Hello, " + name;
}

```### Comments with Interpolation
 Use `format!()` or similar to build dynamic comment strings:
```

let param_name = "userId";
let param_type = "number";
let comment = format!("@param {{{}}} {} - The user ID", param_type, param_name);

let code = ts_template! {
{>> @{comment} <<}
function getUser(userId: number) {}
};

```**Generates:**

```

/\*_ @param {number} userId - The user ID _/
function getUser(userId: number) {}
```## String Interpolation:`"text @{expr}"` Interpolation works automatically inside string literals - no`format!()` needed:

````
let name = "World";
let count = 42;

let code = ts_template! {
   console.log("Hello @{name}!");
   console.log("Count: @{count}, doubled: @{count * 2}");
};
``` **Generates:**
````

console.log("Hello World!");
console.log("Count: 42, doubled: 84");

```This also works with method calls and complex expressions:

```

let field = "username";

let code = ts_template! {
throw new Error("Invalid @{field.to_uppercase()}");
};
```## Backtick Template Literals:`"'^...^'"` For JavaScript template literals (backtick strings), use the`'^...^'`syntax. This outputs actual backticks and passes through`$${}` for JS interpolation:

````
let tag_name = "div";

let code = ts_template! {
   const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
``` **Generates:**
````

const html = `${content}`;
```You can mix Rust`@{}`interpolation (evaluated at macro expansion time) with JS`$${}` interpolation (evaluated at runtime):

````
let class_name = "User";

let code = ts_template! {
   "'^Hello ${this.name}, you are a @{class_name}^'"
};
``` **Generates:**
````

`Hello ${this.name}, you are a User`
```## Conditionals:`{#if}...{/if}`
Basic conditional:

````
let needs_validation = true;

let code = ts_template! {
   function save() {
       {#if needs_validation}
           if (!this.isValid()) return false;
       {/if}
       return this.doSave();
   }
};
``` ### If-Else
````

let has_default = true;

let code = ts_template! {
{#if has_default}
return defaultValue;
{:else}
throw new Error("No default");
{/if}
};

```### If-Else-If Chains

```

let level = 2;

let code = ts_template! {
{#if level == 1}
console.log("Level 1");
{:else if level == 2}
console.log("Level 2");
{:else}
console.log("Other level");
{/if}
};
```## Pattern Matching:`{#if let}` Use`if let`for pattern matching on`Option`, `Result`, or other Rust enums:

````
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
   {#if let Some(name) = maybe_name}
       console.log("Hello, @{name}!");
   {:else}
       console.log("Hello, anonymous!");
   {/if}
};
``` **Generates:**
````

console.log("Hello, Alice!");

```This is useful when working with optional values from your IR:

```

let code = ts_template! {
{#if let Some(default_val) = field.default_value}
this.@{field.name} = @{default_val};
{:else}
this.@{field.name} = undefined;
{/if}
};
```## Match Expressions:`{#match}` Use`match` for exhaustive pattern matching:

````
enum Visibility { Public, Private, Protected }
let visibility = Visibility::Public;

let code = ts_template! {
   {#match visibility}
       {:case Visibility::Public}
           public
       {:case Visibility::Private}
           private
       {:case Visibility::Protected}
           protected
   {/match}
   field: string;
};
``` **Generates:**
````

public field: string;

```### Match with Value Extraction

```

let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
const value = {#match result}
{:case Ok(val)}
@{val}
{:case Err(msg)}
throw new Error("@{msg}")
{/match};
};

```### Match with Wildcard

```

let count = 5;

let code = ts*template! {
{#match count}
{:case 0}
console.log("none");
{:case 1}
console.log("one");
{:case *}
console.log("many");
{/match}
};
```## Iteration:`{#for}`

````
let fields = vec!["name", "email", "age"];

let code = ts_template! {
   function toJSON() {
       const result = {};
       {#for field in fields}
           result.@{field} = this.@{field};
       {/for}
       return result;
   }
};
``` **Generates:**
````

function toJSON() {
const result = {};
result.name = this.name;
result.email = this.email;
result.age = this.age;
return result;
}

```### Tuple Destructuring in Loops

```

let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
{#for (key, class_name) in items}
const @{key} = new @{class_name}();
{/for}
};

```### Nested Iterations

```

let classes = vec![
("User", vec!["name", "email"]),
("Post", vec!["title", "content"]),
];

ts_template! {
{#for (class_name, fields) in classes}
@{class_name}.prototype.toJSON = function() {
return {
{#for field in fields}
@{field}: this.@{field},
{/for}
};
};
{/for}
}
```## While Loops:`{#while}` Use`while` for loops that need to continue until a condition is false:

````
let items = get_items();
let mut idx = 0;

let code = ts_template! {
   {$let mut i = 0}
   {#while i < items.len()}
       console.log("Item @{i}");
       {$do i += 1}
   {/while}
};
``` ### While-Let Pattern Matching
Use `while let` for iterating with pattern matching, similar to `if let`:
````

let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
{#while let Some(item) = items.next()}
console.log("@{item}");
{/while}
};

```**Generates:**

```

console.log("a");
console.log("b");
console.log("c");

```This is especially useful when working with iterators or consuming optional values:

```

let code = ts_template! {
{#while let Some(next_field) = remaining_fields.pop()}
result.@{next_field.name} = this.@{next_field.name};
{/while}
};
```## Local Constants:`{$let}`
Define local variables within the template scope:

````
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
   {#for (key, class_name) in items}
       {$let upper = class_name.to_uppercase()}
       console.log("Processing @{upper}");
       const @{key} = new @{class_name}();
   {/for}
};
``` This is useful for computing derived values inside loops without cluttering the Rust code.
## Mutable Variables: `{$let mut}`
When you need to modify a variable within the template (e.g., in a `while` loop), use `{$let mut}`:
````

let code = ts_template! {
{$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
{/for}
console.log("Total: @{count}");
};
```## Side Effects:`{$do}`
Execute an expression for its side effects without producing output. This is commonly used with mutable variables:

````
let code = ts_template! {
   {$let mut results: Vec<String> = Vec::new()}
   {#for field in fields}
       {$do results.push(format!("this.{}", field))}
   {/for}
   return [@{results.join(", ")}];
};
``` Common uses for `{$do}`:
- Incrementing counters: `{$do i += 1}`
- Building collections: `{$do vec.push(item)}`
- Setting flags: `{$do found = true}`
- Any mutating operation
## TsStream Injection: `{$typescript}`
Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):
````

// Create a helper method with its own import
let mut helper = body! {
validateEmail(email: string): boolean {
return Result.ok(true);
}
};
helper.add_import("Result", "macroforge/utils");

// Inject the helper into the main template
let result = body! {
{$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }

};
// result now includes helper's source AND its Result import

```This is essential for composing multiple macro outputs while preserving imports and patches:

```

let extra_methods = if include_validation {
Some(body! {
validate(): boolean { return true; }
})
} else {
None
};

body! {
mainMethod(): void {}

    {#if let Some(methods) = extra_methods}
        {$typescript methods}
    {/if}

}

```## Escape Syntax
 If you need a literal `@{` in your output (not interpolation), use `@@{`:
```

ts_template! {
// This outputs a literal @{foo}
const example = "Use @@{foo} for templates";
}

```**Generates:**

```

// This outputs a literal @{foo}
const example = "Use @{foo} for templates";

```## Complete Example: JSON Derive Macro
 Here's a comparison showing how `ts_template!` simplifies code generation:
 ### Before (Manual AST Building)
```

pub fn derive_json_macro(input: TsStream) -> MacroResult {
let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            let mut body_stmts = vec![ts_quote!( const result = {}; as Stmt )];

            for field_name in class.field_names() {
                body_stmts.push(ts_quote!(
                    result.$(ident!("{}", field_name)) = this.$(ident!("{}", field_name));
                    as Stmt
                ));
            }

            body_stmts.push(ts_quote!( return result; as Stmt ));

            let runtime_code = fn_assign!(
                member_expr!(Expr::Ident(ident!(class_name)), "prototype"),
                "toJSON",
                body_stmts
            );

            // ...
        }
    }

}

```### After (With ts_template!)

```

pub fn derive_json_macro(input: TsStream) -> MacroResult {
let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! {
                @{class_name}.prototype.toJSON = function() {
                    const result = {};
                    {#for field in fields}
                        result.@{field} = this.@{field};
                    {/for}
                    return result;
                };
            };

            // ...
        }
    }

}

```## How It Works
 1. **Compile-Time:** The template is parsed during macro expansion
 2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
 3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST
 4. **Result:** Returns `Stmt` that can be used in `MacroResult` patches
 ## Return Type
 `ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
```

Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
return {};
}

```This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with `#`, `$`, `:`, or `/`
 - **Regular TypeScript blocks** otherwise
```

ts_template! {
const config = {
{#if use_strict}
strict: true,
{:else}
strict: false,
{/if}
timeout: 5000
};
}

```## Comparison with Alternatives
 | Approach | Pros | Cons |
| --- | --- | --- |
| `ts_quote!` | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| `parse_ts_str()` | Maximum flexibility | Runtime parsing, less readable |
| `ts_template!` | Readable, handles loops/conditions | Small runtime parsing overhead |
 ## Best Practices
 1. Use `ts_template!` for complex code generation with loops/conditions
 2. Use `ts_quote!` for simple, static statements
 3. Keep templates readable - extract complex logic into variables
 4. Don't nest templates too deeply - split into helper functions

---

# Integration

# Integration
 *Macroforge integrates with your development workflow through IDE plugins and build tool integration.*
 ## Overview
 | Integration | Purpose | Package |
| --- | --- | --- |
| TypeScript Plugin | IDE support (errors, completions) | `@macroforge/typescript-plugin` |
| Vite Plugin | Build-time macro expansion | `@macroforge/vite-plugin` |
 ## Recommended Setup
 For the best development experience, use both integrations:
 1. **TypeScript Plugin**: Provides real-time feedback in your IDE
 2. **Vite Plugin**: Expands macros during development and production builds
```

# Install both plugins

npm install -D @macroforge/typescript-plugin @macroforge/vite-plugin

```## How They Work Together
 <div class="flex justify-center"><div class="border-2 border-primary bg-primary/10 rounded-lg px-6 py-3 text-center">Your Code TypeScript with @derive decorators  <div class="absolute top-0 h-px bg-border" style="width: 50%; left: 25%;">

---

# Command Line Interface
  *This binary provides command-line utilities for working with Macroforge TypeScript macros. It is designed for development workflows, enabling macro expansion and type checking without requiring Node.js integration.*
 ## Installation
 The CLI is a Rust binary. You can install it using Cargo:
```

cargo install macroforge_ts

```Or build from source:

```

git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge

```## Commands
 ### macroforge expand
 Expands macros in a TypeScript file and outputs the transformed code.
```

macroforge expand <input> [options]

```#### Arguments
 | Argument | Description |
| --- | --- |
| `<input>` | Path to the TypeScript or TSX file to expand |
 #### Options
 | Option | Description |
| --- | --- |
| `--out <path>` | Write the expanded JavaScript/TypeScript to a file |
| `--types-out <path>` | Write the generated `.d.ts` declarations to a file |
| `--print` | Print output to stdout even when `--out` is specified |
| `--builtin-only` | Use only built-in Rust macros (faster, but no external macro support) |
 #### Examples
 Expand a file and print to stdout:
```

macroforge expand src/user.ts

```Expand and write to a file:

```

macroforge expand src/user.ts --out dist/user.js

```Expand with both runtime output and type declarations:

```

macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts

```Use fast built-in macros only (no external macro support):

```

macroforge expand src/user.ts --builtin-only

```> **Note:** By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where macroforge and any external macro packages are installed in node_modules. ### macroforge tsc
 Runs TypeScript type checking with macro expansion. This wraps `tsc --noEmit` and expands macros before type checking, so your generated methods are properly type-checked.
```

macroforge tsc [options]

```#### Options
 | Option | Description |
| --- | --- |
| `-p, --project <path>` | Path to `tsconfig.json` (defaults to `tsconfig.json` in current directory) |
 #### Examples
 Type check with default tsconfig.json:
```

macroforge tsc

```Type check with a specific config:

```

macroforge tsc -p tsconfig.build.json

```## Output Format
 ### Expanded Code
 When expanding a file like this:
```

/\*_ @derive(Debug) _/
class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}
}

```The CLI outputs the expanded code with the generated methods:

```

class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}

[Symbol.for("nodejs.util.inspect.custom")](): string {
return `User { name: ${this.name}, age: ${this.age} }`;
}
}

```### Diagnostics
 Errors and warnings are printed to stderr in a readable format:
```

[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used

```## Use Cases
 ### CI/CD Type Checking
 Use `macroforge tsc` in your CI pipeline to type-check with macro expansion:
```

# package.json

{
"scripts": {
"typecheck": "macroforge tsc"
}
}

```### Debugging Macro Output
 Use `macroforge expand` to inspect what code your macros generate:
```

macroforge expand src/models/user.ts | less

```### Build Pipeline
 Generate expanded files as part of a custom build:
```

#!/bin/bash
for file in src/\*_/_.ts; do
outfile="dist/$(basename "$file" .ts).js"
macroforge expand "$file" --out "$outfile"
done

```## Built-in vs Full Mode
 By default, the CLI uses Node.js for full macro support including external macros. Use `--builtin-only` for faster expansion when you only need built-in macros:
 | Feature | Default (Node.js) | `--builtin-only` (Rust) |
| --- | --- | --- |
| Built-in macros | Yes | Yes |
| External macros | Yes | No |
| Performance | Standard | Faster |
| Dependencies | Requires `macroforge` in node_modules | None |

---

# TypeScript Plugin
 *The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.*
 ## Installation
```

npm install -D @macroforge/typescript-plugin

```## Configuration
 Add the plugin to your `tsconfig.json`:
```

{
"compilerOptions": {
"plugins": [
{
"name": "@macroforge/typescript-plugin"
}
]
}
}

````## VS Code Setup
 VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):
 1. Open the Command Palette (`Cmd/Ctrl + Shift + P`)
 2. Search for "TypeScript: Select TypeScript Version"
 3. Choose "Use Workspace Version"
  **Tip Add this setting to your `.vscode/settings.json` to make it permanent: ```
{
  "typescript.tsdk": "node_modules/typescript/lib"
}
``` ## Features
 ### Error Reporting
 Errors in macro-generated code are reported at the `@derive` decorator position:
````

/** @derive(Debug) \*/ // **<- Errors appear here
class User {
name: string;
}

```### Completions
 The plugin provides completions for generated methods:
```

const user = new User("Alice");
user.to // Suggests: toString(), toJSON(), etc.

```### Type Information
 Hover over generated methods to see their types:
```

// Hover over 'clone' shows:
// (method) User.clone(): User
const copy = user.clone();

```## Troubleshooting
 ### Plugin Not Loading
 1. Ensure you're using the workspace TypeScript version
 2. Restart the TypeScript server (Command Palette → "TypeScript: Restart TS Server")
 3. Check that the plugin is listed in `tsconfig.json`
 ### Errors Not Showing
 If errors from macros aren't appearing:
 1. Make sure the Vite plugin is also installed (for source file watching)
 2. Check that your file is saved (plugins process on save)

---

# Vite Plugin
 *The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.*
 ## Installation
```

npm install -D @macroforge/vite-plugin

```## Configuration
 Add the plugin to your `vite.config.ts`:
```

import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
plugins: [
macroforge()
]
});

```## Options

```

macroforge({
// Generate .d.ts files for expanded code
generateTypes: true,

// Output directory for generated types
typesOutputDir: ".macroforge/types",

// Emit metadata files for debugging
emitMetadata: false,

// Keep @derive decorators in output (for debugging)
keepDecorators: false,

// File patterns to process
include: ["**/*.ts", "**/*.tsx"],
exclude: ["node_modules/**"]
})

```### Option Reference
 | Option | Type | Default | Description |
| --- | --- | --- | --- |
| `generateTypes` | `boolean` | `true` | Generate .d.ts files |
| `typesOutputDir` | `string` | `.macroforge/types` | Where to write type files |
| `emitMetadata` | `boolean` | `false` | Emit macro metadata files |
| `keepDecorators` | `boolean` | `false` | Keep decorators in output |
 ## Framework Integration
 ### React (Vite)
```

import macroforge from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
plugins: [
macroforge(), // Before React plugin
react()
]
});

```### SvelteKit

```

import macroforge from "@macroforge/vite-plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
plugins: [
macroforge(), // Before SvelteKit
sveltekit()
]
});

```> **Note:** Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first. ## Development Server
 During development, the plugin:
 - Watches for file changes
 - Expands macros on save
 - Provides HMR support for expanded code
 ## Production Build
 During production builds, the plugin:
 - Expands all macros in the source files
 - Generates type declaration files
 - Strips `@derive` decorators from output

---

# Configuration
 *Macroforge can be configured with a `macroforge.json` file in your project root.*
 ## Configuration File
 Create a `macroforge.json` file:
```

{
"allowNativeMacros": true,
"macroPackages": [],
"keepDecorators": false,
"limits": {
"maxExecutionTimeMs": 5000,
"maxMemoryBytes": 104857600,
"maxOutputSize": 10485760,
"maxDiagnostics": 100
}
}

```## Options Reference
 ### allowNativeMacros
 | Type | `boolean` |
| Default | `true` |
 Enable or disable native (Rust) macro packages. Set to `false` to only allow built-in macros.
 ### macroPackages
 | Type | `string[]` |
| Default | `[]` |
 List of npm packages that provide macros. Macroforge will look for macros in these packages.
```

{
"macroPackages": [
"@my-org/custom-macros",
"community-macros"
]
}

```### keepDecorators
 | Type | `boolean` |
| Default | `false` |
 Keep `@derive` decorators in the output. Useful for debugging.
 ### limits
 Configure resource limits for macro expansion:
```

{
"limits": {
// Maximum time for a single macro expansion (ms)
"maxExecutionTimeMs": 5000,

    // Maximum memory usage (bytes)
    "maxMemoryBytes": 104857600,  // 100MB

    // Maximum size of generated code (bytes)
    "maxOutputSize": 10485760,    // 10MB

    // Maximum number of diagnostics per file
    "maxDiagnostics": 100

}
}

```## Macro Runtime Overrides
 Override settings for specific macros:
```

{
"macroRuntimeOverrides": {
"@my-org/macros": {
"maxExecutionTimeMs": 10000
}
}
}

```**Warning Be careful when increasing limits, as this could allow malicious macros to consume excessive resources. ## Environment Variables
 Some settings can be overridden with environment variables:
 | Variable | Description |
| --- | --- |
| `MACROFORGE_DEBUG` | Enable debug logging |
| `MACROFORGE_LOG_FILE` | Write logs to a file |
```

MACROFORGE_DEBUG=1 npm run dev

```**

---

# Language Servers

# Language Servers
 *Macroforge provides language server integrations for enhanced IDE support beyond the TypeScript plugin.*
  **Work in Progress Language server integrations are currently experimental. They work in the repository but are not yet published as official extensions. You'll need to fork the repo and install them as developer extensions. ## Overview
 While the [TypeScript Plugin](../docs/integration/typescript-plugin) provides macro support in any TypeScript-aware editor, dedicated language servers offer deeper integration for specific frameworks and editors.
 | Integration | Purpose | Status |
| --- | --- | --- |
| [Svelte Language Server](../docs/language-servers/svelte) | Full Svelte support with macroforge | Working (dev install) |
| [Zed Extensions](../docs/language-servers/zed) | VTSLS and Svelte for Zed editor | Working (dev install) |
 ## Current Status
 The language servers are functional and used during development of macroforge itself. However, they require manual installation:
 1. Fork or clone the [macroforge-ts repository](https://github.com/rymskip/macroforge-ts)
 2. Build the extension you need
 3. Install it as a developer extension in your editor
 See the individual pages for detailed installation instructions.
 ## Roadmap
 We're working on official extension releases for:
 - VS Code (via VTSLS)
 - Zed (native extensions)
 - Other editors with LSP support
 ## Detailed Guides
 - [Svelte Language Server](../docs/language-servers/svelte) - Full Svelte IDE support
 - [Zed Extensions](../docs/language-servers/zed) - VTSLS and Svelte for Zed
**

---

# Svelte Language Server
 *`@macroforge/svelte-language-server` provides full Svelte IDE support with macroforge integration.*
  **Developer Installation Required This package is not yet published as an official extension. You'll need to build and install it manually. ## Features
 - **Svelte syntax diagnostics** - Errors and warnings in .svelte files
 - **HTML support** - Hover info, autocompletions, Emmet, outline symbols
 - **CSS/SCSS/LESS** - Diagnostics, hover, completions, formatting, Emmet, color picking
 - **TypeScript/JavaScript** - Full language features with macroforge macro expansion
 - **Go-to-definition** - Navigate to macro-generated code
 - **Code actions** - Quick fixes and refactorings
 ## Installation
 ### 1. Clone the Repository
```

git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts

```### 2. Build the Language Server

```

# Install dependencies

npm install

# Build the Svelte language server

cd packages/svelte-language-server
npm run build

```### 3. Configure Your Editor
 The language server exposes a `svelteserver` binary that implements the Language Server Protocol (LSP). Configure your editor to use it:
```

# The binary is located at:

./packages/svelte-language-server/bin/server.js

```## Package Info
 | Package | `@macroforge/svelte-language-server` |
| Version | 0.1.7 |
| CLI Command | `svelteserver` |
| Node Version | >= 18.0.0 |
 ## How It Works
 The Svelte language server extends the standard Svelte language tooling with macroforge integration:
 1. Parses `.svelte` files and extracts TypeScript/JavaScript blocks
 2. Expands macros using the `@macroforge/typescript-plugin`
 3. Maps diagnostics back to original source positions
 4. Provides completions for macro-generated methods
 ## Using with Zed
 For Zed editor, see the [Zed Extensions](../../docs/language-servers/zed) page for the dedicated `svelte-macroforge` extension.
**

---

# Zed Extensions
 *Macroforge provides two extensions for the [Zed editor](https://zed.dev): one for TypeScript via VTSLS, and one for Svelte.*
  **Developer Installation Required These extensions are not yet in the Zed extension registry. You'll need to install them as developer extensions. ## Available Extensions
 | Extension | Description | Location |
| --- | --- | --- |
| `vtsls-macroforge` | VTSLS with macroforge support for TypeScript | `crates/extensions/vtsls-macroforge` |
| `svelte-macroforge` | Svelte language support with macroforge | `crates/extensions/svelte-macroforge` |
 ## Installation
 ### 1. Clone the Repository
```

git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts

```### 2. Build the Extension
 Build the extension you want to use:
```

# For VTSLS (TypeScript)

cd crates/extensions/vtsls-macroforge

# Or for Svelte

cd crates/extensions/svelte-macroforge

```### 3. Install as Dev Extension in Zed
 In Zed, open the command palette and run **zed: install dev extension**, then select the extension directory.
 Alternatively, symlink the extension to your Zed extensions directory:
```

# macOS

ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/Library/Application\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux

ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/.config/zed/extensions/installed/vtsls-macroforge

```## vtsls-macroforge
 This extension wraps [VTSLS](https://github.com/yioneko/vtsls) (a TypeScript language server) with macroforge integration. It provides:
 - Full TypeScript language features
 - Macro expansion at edit time
 - Accurate error positions in original source
 - Completions for macro-generated methods
 ## svelte-macroforge
 This extension provides Svelte support using the `@macroforge/svelte-language-server`. It includes:
 - Svelte component syntax support
 - HTML, CSS, and TypeScript features
 - Macroforge integration in script blocks
 ## Troubleshooting
 ### Extension not loading
 Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error messages.
 ### Macros not expanding
 Ensure your project has the `macroforge` package installed and a valid `tsconfig.json` with the TypeScript plugin configured.
**

---

# API Reference

# API Reference
 *Macroforge provides a programmatic API for expanding macros in TypeScript code.*
 ## Overview
```

import {
expandSync,
transformSync,
checkSyntax,
parseImportSources,
NativePlugin,
PositionMapper
} from "macroforge";

```## Core Functions
 | Function | Description |
| --- | --- |
| [`expandSync()`](../docs/api/expand-sync) | Expand macros synchronously |
| [`transformSync()`](../docs/api/transform-sync) | Transform code with additional metadata |
| `checkSyntax()` | Validate TypeScript syntax |
| `parseImportSources()` | Extract import information |
 ## Classes
 | Class | Description |
| --- | --- |
| [`NativePlugin`](../docs/api/native-plugin) | Stateful plugin with caching |
| [`PositionMapper`](../docs/api/position-mapper) | Maps positions between original and expanded code |
 ## Quick Example
```

import { expandSync } from "macroforge";

const sourceCode = `/** @derive(Debug) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}`;

const result = expandSync(sourceCode, "user.ts", {
keepDecorators: false
});

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) {
console.error("Errors:", result.diagnostics);
}

```## Detailed Reference
 - [`expandSync()`](../docs/api/expand-sync) - Full options and return types
 - [`transformSync()`](../docs/api/transform-sync) - Transform with source maps
 - [`NativePlugin`](../docs/api/native-plugin) - Caching for language servers
 - [`PositionMapper`](../docs/api/position-mapper) - Position mapping utilities

---

# expandSync()
  *Synchronously expands macros in TypeScript code. This is the standalone macro expansion function that doesn't use caching. For cached expansion, use [`NativePlugin::process_file`] instead.*
 ## Signature
```

function expandSync(
code: string,
filepath: string,
options?: ExpandOptions
): ExpandResult

```## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| `code` | `string` | TypeScript source code to transform |
| `filepath` | `string` | File path (used for error reporting) |
| `options` | `ExpandOptions` | Optional configuration |
 ## ExpandOptions
```

interface ExpandOptions {
// Keep @derive decorators in output (default: false)
keepDecorators?: boolean;
}

```## ExpandResult

```

interface ExpandResult {
// Transformed TypeScript code
code: string;

// Generated type declarations (.d.ts content)
types?: string;

// Macro expansion metadata (JSON string)
metadata?: string;

// Warnings and errors from macro expansion
diagnostics: MacroDiagnostic[];

// Position mapping data for source maps
sourceMapping?: SourceMappingResult;
}

```## MacroDiagnostic

```

interface MacroDiagnostic {
message: string;
severity: "error" | "warning" | "info";
span: {
start: number;
end: number;
};
}

```## Example

```

import { expandSync } from "macroforge";

const sourceCode = `
/\*_ @derive(Debug) _/
class User {
name: string;
age: number;

constructor(name: string, age: number) {
this.name = name;
this.age = age;
}
}
`;

const result = expandSync(sourceCode, "user.ts");

console.log("Transformed code:");
console.log(result.code);

if (result.types) {
console.log("Type declarations:");
console.log(result.types);
}

if (result.diagnostics.length > 0) {
for (const diag of result.diagnostics) {
console.log(`[${diag.severity}] ${diag.message}`);
}
}

```## Error Handling
 Syntax errors and macro errors are returned in the `diagnostics` array, not thrown as exceptions:
```

const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
if (diag.severity === "error") {
console.error(`Error at ${diag.span.start}: ${diag.message}`);
}
}

```

---

# transformSync()
  *Synchronously transforms TypeScript code through the macro expansion system. This is similar to [`expand_sync`] but returns a [`TransformResult`] which includes source map information (when available).*
 ## Signature
```

function transformSync(
code: string,
filepath: string
): TransformResult

```## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| `code` | `string` | TypeScript source code to transform |
| `filepath` | `string` | File path (used for error reporting) |
 ## TransformResult
```

interface TransformResult {
// Transformed TypeScript code
code: string;

// Source map (JSON string, not yet implemented)
map?: string;

// Generated type declarations
types?: string;

// Macro expansion metadata
metadata?: string;
}

```## Comparison with expandSync()
 | Feature | `expandSync` | `transformSync` |
| --- | --- | --- |
| Options | Yes | No |
| Diagnostics | Yes | No |
| Source Mapping | Yes | Limited |
| Use Case | General purpose | Build tools |
 ## Example
```

import { transformSync } from "macroforge";

const sourceCode = `/** @derive(Debug) */
class User {
  name: string;
}`;

const result = transformSync(sourceCode, "user.ts");

console.log(result.code);

if (result.types) {
// Write to .d.ts file
fs.writeFileSync("user.d.ts", result.types);
}

if (result.metadata) {
// Parse and use metadata
const meta = JSON.parse(result.metadata);
console.log("Macros expanded:", meta);
}

```## When to Use
 Use `transformSync` when:
 - Building custom integrations
 - You need raw output without diagnostics
 - You're implementing a build tool plugin
 Use `expandSync` for most other use cases, as it provides better error handling.

---

# NativePlugin
  *The main plugin class for macro expansion with caching support. `NativePlugin` is designed to be instantiated once and reused across multiple file processing operations. It maintains a cache of expansion results keyed by filepath and version, enabling efficient incremental processing.*
 ## Constructor
```

const plugin = new NativePlugin();

```## Methods
 ### processFile()
 Process a file with version-based caching:
```

processFile(
filepath: string,
code: string,
options?: ProcessFileOptions
): ExpandResult
` `
interface ProcessFileOptions {
// Cache key - if unchanged, returns cached result
version?: string;
}

```### getMapper()
 Get the position mapper for a previously processed file:
```

getMapper(filepath: string): NativeMapper | null

```### mapDiagnostics()
 Map diagnostics from expanded positions to original positions:
```

mapDiagnostics(
filepath: string,
diagnostics: JsDiagnostic[]
): JsDiagnostic[]

```### log() / setLogFile()
 Logging utilities for debugging:
```

log(message: string): void
setLogFile(path: string): void

```## Caching Behavior
 The plugin caches expansion results by file path and version:
```

const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });

```## Example: Language Server Integration

```

import { NativePlugin } from "macroforge";

class MacroforgeLanguageService {
private plugin = new NativePlugin();

processDocument(uri: string, content: string, version: number) {
// Process with version-based caching
const result = this.plugin.processFile(uri, content, {
version: String(version)
});

    // Get mapper for position translation
    const mapper = this.plugin.getMapper(uri);

    return { result, mapper };

}

getSemanticDiagnostics(uri: string, diagnostics: Diagnostic[]) {
// Map positions from expanded to original
return this.plugin.mapDiagnostics(uri, diagnostics);
}
}

```## Thread Safety
 The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.

---

# PositionMapper
  *Bidirectional position mapper for translating between original and expanded source positions. This mapper enables IDE features like error reporting, go-to-definition, and hover to work correctly with macro-expanded code by translating positions between the original source (what the user wrote) and the expanded source (what the compiler sees).*
 ## Getting a Mapper
```

import { NativePlugin, PositionMapper } from "macroforge";

const plugin = new NativePlugin();
const result = plugin.processFile("user.ts", code, { version: "1" });

// Get the mapper for this file
const mapper = plugin.getMapper("user.ts");
if (mapper) {
// Use the mapper...
}

```## Methods
 ### isEmpty()
 Check if the mapper has any mappings:
```

isEmpty(): boolean

```### originalToExpanded()
 Map a position from original to expanded code:
```

originalToExpanded(pos: number): number

```### expandedToOriginal()
 Map a position from expanded to original code:
```

expandedToOriginal(pos: number): number | null
```Returns`null` if the position is in generated code.

### isInGenerated()

Check if a position is in macro-generated code:

````
isInGenerated(pos: number): boolean
``` ### generatedBy()
Get the name of the macro that generated code at a position:
````

generatedBy(pos: number): string | null

```### mapSpanToOriginal()
 Map a span (range) from expanded to original code:
```

mapSpanToOriginal(start: number, length: number): SpanResult | null

interface SpanResult {
start: number;
length: number;
}

```### mapSpanToExpanded()
 Map a span from original to expanded code:
```

mapSpanToExpanded(start: number, length: number): SpanResult

```## Example: Error Position Mapping

```

import { NativePlugin } from "macroforge";

const plugin = new NativePlugin();

function mapError(filepath: string, expandedPos: number, message: string) {
const mapper = plugin.getMapper(filepath);
if (!mapper) return null;

// Check if the error is in generated code
if (mapper.isInGenerated(expandedPos)) {
const macroName = mapper.generatedBy(expandedPos);
return {
message: `Error in code generated by @derive(${macroName}): ${message}`,
// Find the @derive decorator position
position: findDecoratorPosition(filepath)
};
}

// Map to original position
const originalPos = mapper.expandedToOriginal(expandedPos);
if (originalPos !== null) {
return {
message,
position: originalPos
};
}

return null;
}

```## Performance
 Position mapping uses binary search with O(log n) complexity:
 - Fast lookups even for large files
 - Minimal memory overhead
 - Thread-safe access

---
```
