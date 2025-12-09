# Macroforge Documentation

*TypeScript Macros - Rust-Powered Code Generation*

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

*Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros.*

## Requirements

- Node.js 24.0 or later

- TypeScript 5.9 or later

## Install the Package

Install Macroforge using your preferred package manager:

`npm`
```bash
npm install macroforge
```

`bun`
```bash
bun add macroforge
```

`pnpm`
```bash
pnpm add macroforge
```

> **Note:**
> Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64).

## Basic Usage

The simplest way to use Macroforge is with the built-in derive macros. Add a `@derive` comment decorator to your class:

`user.ts`
```typescript
/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

// After macro expansion, User has:
// - toString(): string         (from Debug)
// - clone(): User              (from Clone)
// - equals(other: User): boolean  (from Eq)
```

## IDE Integration

For the best development experience, add the TypeScript plugin to your `tsconfig.json`:

`tsconfig.json`
```json
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}
```

This enables features like:

- Accurate error positions in your source code

- Autocompletion for generated methods

- Type checking for expanded code

## Build Integration (Vite)

If you're using Vite, add the plugin to your config for automatic macro expansion during build:

`vite.config.ts`
```typescript
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
```

## Next Steps

Now that you have Macroforge installed, learn how to use it:

- [Create your first macro]({base}/docs/getting-started/first-macro)

- [Understand how macros work]({base}/docs/concepts)

- [Explore built-in macros]({base}/docs/builtin-macros)

---

# Your First Macro

*Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*

## Creating a Class with Derive Macros

Start by creating a simple `User` class. We'll use the `@derive` decorator to automatically generate methods.

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

## Using the Generated Methods

```typescript
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
```

## Customizing Behavior

You can customize how macros work using field-level decorators. For example, with the Debug macro:

<MacroExample before={data.examples.customizing.before} after={data.examples.customizing.after} />

```typescript
const user = new User(42, "Alice", "secret123");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
// Note: 'id' is renamed to 'userId', 'password' is skipped
```

<Alert type="tip" title="Field-level decorators">
Field-level decorators let you control exactly how each field is handled by the macro.
</Alert>

## Next Steps

- [Learn how macros work under the hood]({base}/docs/concepts)

- [Explore all Debug options]({base}/docs/builtin-macros/debug)

- [Create your own custom macros]({base}/docs/custom-macros)

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

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

## Zero Runtime Overhead

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

<Alert type="info" title="Error positioning">
The TypeScript plugin uses source mapping to show errors at the `@derive` decorator position, not in the generated code.
</Alert>

## Execution Flow

<Flowchart steps={[
{ title: "Your Source Code", description: "with @derive decorators" },
{ title: "SWC Parser", description: "TypeScript → AST" },
{ title: "Macro Expansion Engine", description: "Finds @derive decorators, runs macros, generates new AST nodes" },
{ title: "Code Generator", description: "AST → TypeScript" },
{ title: "Expanded TypeScript", description: "ready for normal compilation" }
]} />

## Integration Points

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

- [Learn about the derive system]({base}/docs/concepts/derive-system)

- [Explore the architecture]({base}/docs/concepts/architecture)

---

# The Derive System

*The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with `@derive`.*

## Syntax Reference

Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.

### The @derive Statement

The `@derive` decorator triggers macro expansion on a class or interface:

<InteractiveMacro code={`/** @derive(Debug) */
class MyClass {
  value: string;
}`} />

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)

- Must appear immediately before the class/interface declaration

- Multiple macros can be comma-separated: `@derive(A, B, C)`

- Multiple `@derive` statements can be stacked

<InteractiveMacro code={`/** @derive(Debug, Clone) */
class User {
  name: string;
  email: string;
}`} />

### The import macro Statement

To use macros from external packages, you must declare them with `import macro`:

```typescript
/** import macro { MacroName } from "package-name"; */
```

Syntax rules:

- Must be inside a JSDoc comment (`/** */`)

- Can appear anywhere in the file (typically at the top)

- Multiple macros can be imported: `import macro &#123; A, B &#125; from "pkg";`

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

<Alert type="note" title="Built-in macros">
Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement.
</Alert>

### Field Attributes

Macros can define field-level attributes to customize behavior per field:

<MacroExample before={data.examples.fieldAttributes.before} after={data.examples.fieldAttributes.after} />

Syntax rules:

- Must be inside a JSDoc comment immediately before the field

- Options use object literal syntax: `@attr(&#123; key: value &#125;)`

- Boolean options: `@attr(&#123; skip: true &#125;)`

- String options: `@attr(&#123; rename: "newName" &#125;)`

- Multiple attributes can be on separate lines or combined

Common field attributes by macro:

<table>
<thead>
<tr>
<th>Macro</th>
<th>Attribute</th>
<th>Options</th>
</tr>
</thead>
<tbody>
<tr>
<td>Debug</td>
<td>`@debug`</td>
<td>`skip`, `rename`</td>
</tr>
<tr>
<td>Clone</td>
<td>`@clone`</td>
<td>`skip`, `clone_with`</td>
</tr>
<tr>
<td>Serialize/Deserialize</td>
<td>`@serde`</td>
<td>`skip`, `rename`, `flatten`, `default`</td>
</tr>
<tr>
<td>Hash</td>
<td>`@hash`</td>
<td>`skip`</td>
</tr>
<tr>
<td>PartialEq/Ord</td>
<td>`@eq`, `@ord`</td>
<td>`skip`</td>
</tr>
</tbody>
</table>

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

<table>
<thead>
<tr>
<th>Type</th>
<th>Import Required</th>
<th>Examples</th>
</tr>
</thead>
<tbody>
<tr>
<td>Built-in</td>
<td>No</td>
<td>Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize</td>
</tr>
<tr>
<td>Custom</td>
<td>Yes</td>
<td>Any macro from an external package</td>
</tr>
</tbody>
</table>

## Next Steps

- [Explore built-in macros]({base}/docs/builtin-macros)

- [Create custom macros]({base}/docs/custom-macros)

---

# Architecture

*Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.*

## Overview

<ArchitectureDiagram layers={[
{ title: "Node.js / Vite" },
{ title: "NAPI-RS Bindings" },
{ title: "Macro Crates", items: ["macroforge_ts_syn", "macroforge_ts_quote", "macroforge_ts_macros"] },
{ title: "SWC Core", items: ["TypeScript parsing & codegen"] }
]} />

## Core Components

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

- Control flow: `{"{#for}"}`, `{"{#if}"}`, `{"{$let}"}`

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

<Flowchart steps={[
{ title: "1. Source Code", description: "TypeScript with @derive" },
{ title: "2. NAPI-RS", description: "receives JavaScript string" },
{ title: "3. SWC Parser", description: "parses to AST" },
{ title: "4. Macro Expander", description: "finds @derive decorators" },
{ title: "5. For Each Macro", description: "extract data, run macro, generate AST nodes" },
{ title: "6. Merge", description: "generated nodes into AST" },
{ title: "7. SWC Codegen", description: "generates source code" },
{ title: "8. Return", description: "to JavaScript with source mapping" }
]} />

## Performance Characteristics

- **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack

- **Caching**: `NativePlugin` caches results by file version

- **Binary search**: Position mapping uses O(log n) lookups

- **Zero-copy**: SWC's arena allocator minimizes allocations

## Re-exported Crates

For custom macro development, `macroforge_ts` re-exports everything you need:

```rust
// Convenient re-exports for macro development
use macroforge_ts::macros::{ts_macro_derive, body, ts_template, above, below, signature};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

// Also available: raw crate access and SWC modules
use macroforge_ts::swc_core;
use macroforge_ts::swc_common;
use macroforge_ts::swc_ecma_ast;
```

## Next Steps

- [Write custom macros]({base}/docs/custom-macros)

- [Explore the API reference]({base}/docs/api)

---

# Built-in Macros

# Built-in Macros

*Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*

## Overview

<table>
<thead>
<tr>
<th>Macro</th>
<th>Generates</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>[`Debug`]({base}/docs/builtin-macros/debug)</td>
<td>`toString(): string`</td>
<td>Human-readable string representation</td>
</tr>
<tr>
<td>[`Clone`]({base}/docs/builtin-macros/clone)</td>
<td>`clone(): T`</td>
<td>Creates a deep copy of the object</td>
</tr>
<tr>
<td>[`Default`]({base}/docs/builtin-macros/default)</td>
<td>`static default(): T`</td>
<td>Creates an instance with default values</td>
</tr>
<tr>
<td>[`Hash`]({base}/docs/builtin-macros/hash)</td>
<td>`hashCode(): number`</td>
<td>Generates a hash code for the object</td>
</tr>
<tr>
<td>[`PartialEq`]({base}/docs/builtin-macros/partial-eq)</td>
<td>`equals(other: T): boolean`</td>
<td>Value equality comparison</td>
</tr>
<tr>
<td>[`Ord`]({base}/docs/builtin-macros/ord)</td>
<td>`compare(other: T): number`</td>
<td>Total ordering comparison (-1, 0, 1)</td>
</tr>
<tr>
<td>[`PartialOrd`]({base}/docs/builtin-macros/partial-ord)</td>
<td>`partialCompare(other: T): number | null`</td>
<td>Partial ordering comparison</td>
</tr>
<tr>
<td>[`Serialize`]({base}/docs/builtin-macros/serialize)</td>
<td>`toJSON(): Record<string, unknown>`</td>
<td>JSON serialization with type handling</td>
</tr>
<tr>
<td>[`Deserialize`]({base}/docs/builtin-macros/deserialize)</td>
<td>`static fromJSON(data: unknown): T`</td>
<td>JSON deserialization with validation</td>
</tr>
</tbody>
</table>

## Using Built-in Macros

Built-in macros don't require imports. Just use them with `@derive`:

```typescript
/** @derive(Debug, Clone, PartialEq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```

## Interface Support

All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using `self` as the first parameter:

```typescript
/** @derive(Debug, Clone, PartialEq) */
interface Point {
  x: number;
  y: number;
}

// Generated namespace:
// namespace Point {
//   export function toString(self: Point): string { ... }
//   export function clone(self: Point): Point { ... }
//   export function equals(self: Point, other: Point): boolean { ... }
//   export function hashCode(self: Point): number { ... }
// }

const point: Point = { x: 10, y: 20 };

// Use the namespace functions
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
```

## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:

```typescript
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated namespace:
// namespace Status {
//   export function toString(value: Status): string { ... }
//   export function clone(value: Status): Status { ... }
//   export function equals(a: Status, b: Status): boolean { ... }
//   export function hashCode(value: Status): number { ... }
//   export function toJSON(value: Status): string | number { ... }
//   export function fromJSON(data: unknown): Status { ... }
// }

// Use the namespace functions
console.log(Status.toString(Status.Active));     // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending);      // "pending"
const parsed = Status.fromJSON("active");        // Status.Active
```

## Type Alias Support

All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:

```typescript
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = {
  x: number;
  y: number;
};

// Generated namespace:
// namespace Point {
//   export function toString(value: Point): string { ... }
//   export function clone(value: Point): Point { ... }
//   export function equals(a: Point, b: Point): boolean { ... }
//   export function hashCode(value: Point): number { ... }
//   export function toJSON(value: Point): Record<string, unknown> { ... }
//   export function fromJSON(data: unknown): Point { ... }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));     // "Point { x: 10, y: 20 }"
const copy = Point.clone(point);        // { x: 10, y: 20 }
console.log(Point.equals(point, copy)); // true
```

Union type aliases also work, using JSON-based implementations:

```typescript
/** @derive(Debug, PartialEq) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success", "success")); // true
```

## Combining Macros

All macros can be used together. They don't conflict and each generates independent methods:

```typescript
const user = new User("Alice", 30);

// Debug
console.log(user.toString());
// "User { name: Alice, age: 30 }"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true
```

## Detailed Documentation

Each macro has its own options and behaviors:

- [**Debug**]({base}/docs/builtin-macros/debug) - Customizable field renaming and skipping

- [**Clone**]({base}/docs/builtin-macros/clone) - Deep copying for all field types

- [**Default**]({base}/docs/builtin-macros/default) - Default value generation with field attributes

- [**Hash**]({base}/docs/builtin-macros/hash) - Hash code generation for use in maps and sets

- [**PartialEq**]({base}/docs/builtin-macros/partial-eq) - Value-based equality comparison

- [**Ord**]({base}/docs/builtin-macros/ord) - Total ordering for sorting

- [**PartialOrd**]({base}/docs/builtin-macros/partial-ord) - Partial ordering comparison

- [**Serialize**]({base}/docs/builtin-macros/serialize) - JSON serialization with serde-style options

- [**Deserialize**]({base}/docs/builtin-macros/deserialize) - JSON deserialization with validation

---

# Debug

*The `Debug` macro generates a `toString()` method that produces a human-readable string representation of your class.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }
```

## Field Options

Use the `@debug` field decorator to customize behavior:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
```

### Skipping Fields

Use `skip: true` to exclude sensitive fields from the output:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included
```

<Alert type="tip" title="Security">
Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging.
</Alert>

## Combining Options

<InteractiveMacro code={`/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}`} />

## All Options

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`rename`</td>
<td>`string`</td>
<td>Display a different name in the output</td>
</tr>
<tr>
<td>`skip`</td>
<td>`boolean`</td>
<td>Exclude this field from the output</td>
</tr>
</tbody>
</table>

## Interface Support

Debug also works with interfaces. For interfaces, a namespace is generated with a `toString` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }
```

## Enum Support

Debug also works with enums. For enums, a namespace is generated with a `toString` function that displays the enum name and variant:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Priority.toString(Priority.High));
// Output: Priority.High

console.log(Priority.toString(Priority.Low));
// Output: Priority.Low
```

Works with both numeric and string enums:

<InteractiveMacro code={`/** @derive(Debug) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}`} />

## Type Alias Support

Debug works with type aliases. For object types, fields are displayed similar to interfaces:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));
// Output: Point { x: 10, y: 20 }
```

For union types, the value is displayed using JSON.stringify:

<InteractiveMacro code={`/** @derive(Debug) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.toString("success"));
// Output: ApiStatus("success")
```

---

# Clone

*The `Clone` macro generates a `clone()` method that creates a copy of the object.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const original = new Point(10, 20);
const copy = original.clone();

console.log(copy.x, copy.y); // 10, 20
console.log(original === copy); // false (different instances)
```

## How It Works

The Clone macro:

1. Creates a new instance of the class

2. Passes all field values to the constructor

3. Returns the new instance

This creates a **shallow clone** - primitive values are copied, but object references remain the same.

## With Nested Objects

<MacroExample before={data.examples.nested.before} after={data.examples.nested.after} />

```typescript
const original = new User("Alice", { city: "NYC", zip: "10001" });
const copy = original.clone();

// The address object is the same reference
console.log(original.address === copy.address); // true

// Modifying the copy's address affects the original
copy.address.city = "LA";
console.log(original.address.city); // "LA"
```

For deep cloning of nested objects, you would need to implement custom clone methods or use a deep clone utility.

## Combining with PartialEq

Clone works well with PartialEq for creating independent copies that compare as equal:

<InteractiveMacro code={`/** @derive(Clone, PartialEq) */
class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}`} />

```typescript
const original = new Point(10, 20);
const copy = original.clone();

console.log(original === copy);       // false (different instances)
console.log(original.equals(copy));   // true (same values)
```

## Interface Support

Clone also works with interfaces. For interfaces, a namespace is generated with a `clone` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);        // 10, 20
console.log(original === copy);     // false (different objects)
```

## Enum Support

Clone also works with enums. For enums, the clone function simply returns the value as-is, since enum values are primitives and don't need cloning:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const original = Status.Active;
const copy = Status.clone(original);

console.log(copy);               // "active"
console.log(original === copy);  // true (same primitive value)
```

## Type Alias Support

Clone works with type aliases. For object types, a shallow copy is created using spread:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const original: Point = { x: 10, y: 20 };
const copy = Point.clone(original);

console.log(copy.x, copy.y);     // 10, 20
console.log(original === copy);  // false (different objects)
```

For union types, the value is returned as-is (unions of primitives don't need cloning):

<InteractiveMacro code={`/** @derive(Clone) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
const status: ApiStatus = "success";
const copy = ApiStatus.clone(status);
console.log(copy); // "success"
```

---

# Default

*The `Default` macro generates a static `default()` factory method that creates instances with default field values.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const config = Config.default();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false
```

## Automatic Default Values

The Default macro automatically determines default values based on field types:

- `string` → `""` (empty string)

- `number` → `0`

- `boolean` → `false`

- `bigint` → `0n`

- `Array<T>` or `T[]` → `[]` (empty array)

- `Map<K, V>` → `new Map()`

- `Set<T>` → `new Set()`

- `Date` → `new Date()`

- Custom types → `null as any`

## Custom Default Values

Use the `@defaultValue()` decorator to specify custom default values for fields:

<MacroExample before={data.examples.custom.before} after={data.examples.custom.after} />

```typescript
const config = ServerConfig.default();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]
```

## Interface Support

Default also works with interfaces. For interfaces, a namespace is generated with a `default_()` function (note the underscore to avoid conflicts with the reserved word):

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const origin = Point.default_();
console.log(origin); // { x: 0, y: 0 }
```

## Enum Support

Default works with enums. For enums, it returns the first variant as the default value:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const defaultStatus = Status.default_();
console.log(defaultStatus); // "pending"
```

## Type Alias Support

Default works with type aliases. For object types, it creates an object with default field values:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const dims = Dimensions.default_();
console.log(dims); // { width: 0, height: 0 }
```

## Combining with Other Macros

<InteractiveMacro code={`/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @defaultValue("Anonymous") */
  name: string;

  /** @defaultValue(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} />

```typescript
const user1 = User.default();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true
```

---

# Hash

*The `Hash` macro generates a `hashCode()` method that computes a numeric hash value based on field values.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.hashCode()); // Same hash
console.log(p2.hashCode()); // Same hash (equal values = equal hash)
console.log(p3.hashCode()); // Different hash
```

## Hash Algorithm

The generated hash function uses the following algorithm for different types:

- `number` → Integers use bitwise OR (`| 0`), floats are stringified and hashed

- `string` → Character-by-character hash: `(h * 31 + charCode) | 0`

- `boolean` → `1231` for true, `1237` for false (Java convention)

- `bigint` → Converted to string and hashed character-by-character

- `Date` → Uses `getTime() | 0` for timestamp hash

- `Array` → Combines element hashes with `h * 31 + elementHash`

- `Map/Set` → Combines all entry hashes

- `Object` → Calls `hashCode()` if available, otherwise JSON stringifies and hashes

- `null` → Returns 0

- `undefined` → Returns 1

## Field Options

### @hash(skip)

Use `@hash(skip)` to exclude a field from hash computation:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.hashCode() === user2.hashCode()); // true (lastLogin is skipped)
```

## Use with PartialEq

Hash is often used together with PartialEq. Objects that are equal should have the same hash code:

<InteractiveMacro code={`/** @derive(Hash, PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

```typescript
const p1 = new Product("ABC123", "Widget");
const p2 = new Product("ABC123", "Widget");

// Equal objects have equal hash codes
console.log(p1.equals(p2));                       // true
console.log(p1.hashCode() === p2.hashCode());     // true
```

## Interface Support

Hash also works with interfaces. For interfaces, a namespace is generated with a `hashCode` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const p: Point = { x: 10, y: 20 };
console.log(Point.hashCode(p)); // numeric hash value
```

## Enum Support

Hash works with enums. For string enums, it hashes the string value; for numeric enums, it uses the numeric value directly:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.hashCode(Status.Active));   // consistent hash
console.log(Status.hashCode(Status.Inactive)); // different hash
```

## Type Alias Support

Hash works with type aliases. For object types, it hashes each field:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const loc: Coordinates = { lat: 40.7128, lng: -74.0060 };
console.log(Coordinates.hashCode(loc));
```

For union types, it uses JSON stringification as a fallback:

<InteractiveMacro code={`/** @derive(Hash) */
type Result = "success" | "error" | "pending";`} />

```typescript
console.log(Result.hashCode("success")); // hash of "success" string
console.log(Result.hashCode("error"));   // hash of "error" string
```

---

# Ord

*The `Ord` macro generates a `compareTo()` method that implements total ordering, always returning `-1`, `0`, or `1`.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const v1 = new Version(1, 0, 0);
const v2 = new Version(1, 2, 0);
const v3 = new Version(1, 2, 0);

console.log(v1.compareTo(v2)); // -1 (v1 < v2)
console.log(v2.compareTo(v1)); // 1  (v2 > v1)
console.log(v2.compareTo(v3)); // 0  (v2 == v3)
```

## Comparison Logic

The Ord macro compares fields in declaration order (lexicographic ordering). For each type:

- `number` / `bigint` → Direct numeric comparison

- `string` → Uses `localeCompare()` clamped to -1/0/1

- `boolean` → `false < true`

- `Date` → Compares timestamps via `getTime()`

- `Array` → Lexicographic: compares element-by-element, then length

- `Map/Set` → Size and content comparison

- `Object` → Calls `compareTo()` if available, otherwise 0

- `null/undefined` → Treated as equal (returns 0)

## Return Values

The `compareTo()` method always returns:

- `-1` → `this` is less than `other`

- `0` → `this` equals `other`

- `1` → `this` is greater than `other`

Unlike `PartialOrd`, the `Ord` macro never returns `null` - it provides total ordering.

## Field Options

### @ord(skip)

Use `@ord(skip)` to exclude a field from ordering comparison:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const t1 = new Task(1, "Bug fix", new Date("2024-01-01"));
const t2 = new Task(1, "Bug fix", new Date("2024-12-01"));

console.log(t1.compareTo(t2)); // 0 (createdAt is skipped)
```

## Sorting Arrays

The generated `compareTo()` method works directly with `Array.sort()`:

<InteractiveMacro code={`/** @derive(Ord) */
class Score {
  points: number;
  name: string;

  constructor(points: number, name: string) {
    this.points = points;
    this.name = name;
  }
}`} />

```typescript
const scores = [
  new Score(100, "Alice"),
  new Score(50, "Bob"),
  new Score(150, "Charlie"),
  new Score(50, "Alice")  // Same points, different name
];

// Sort ascending
scores.sort((a, b) => a.compareTo(b));
// Result: [Bob(50), Alice(50), Alice(100), Charlie(150)]

// Sort descending
scores.sort((a, b) => b.compareTo(a));
// Result: [Charlie(150), Alice(100), Alice(50), Bob(50)]
```

## Interface Support

Ord works with interfaces. For interfaces, a namespace is generated with a `compareTo` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const points: Point[] = [
  { x: 5, y: 10 },
  { x: 1, y: 20 },
  { x: 5, y: 5 }
];

points.sort((a, b) => Point.compareTo(a, b));
// Result: [{ x: 1, y: 20 }, { x: 5, y: 5 }, { x: 5, y: 10 }]
```

## Enum Support

Ord works with enums. For numeric enums, it compares the numeric values; for string enums, it uses string comparison:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Priority.compareTo(Priority.Low, Priority.High));      // -1
console.log(Priority.compareTo(Priority.Critical, Priority.Low));  // 1
console.log(Priority.compareTo(Priority.Medium, Priority.Medium)); // 0
```

## Type Alias Support

Ord works with type aliases. For object types, it uses lexicographic field comparison:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const c1: Coordinate = { x: 10, y: 20 };
const c2: Coordinate = { x: 10, y: 30 };

console.log(Coordinate.compareTo(c1, c2)); // -1 (c1 < c2)
```

## Ord vs PartialOrd

Use `Ord` when all values of a type are comparable. Use `PartialOrd` when some values might be incomparable (e.g., different types at runtime).

<InteractiveMacro code={`// Ord: Total ordering - never returns null
/** @derive(Ord) */
class Version {
  major: number;
  minor: number;
  constructor(major: number, minor: number) {
    this.major = major;
    this.minor = minor;
  }
}`} />

```typescript
const v1 = new Version(1, 0);
const v2 = new Version(2, 0);
console.log(v1.compareTo(v2)); // Always -1, 0, or 1
```

---

# PartialEq

*The `PartialEq` macro generates an `equals()` method for value-based equality comparison between objects.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const p1 = new Point(10, 20);
const p2 = new Point(10, 20);
const p3 = new Point(5, 5);

console.log(p1.equals(p2)); // true (same values)
console.log(p1.equals(p3)); // false (different values)
console.log(p1 === p2);     // false (different references)
```

## How It Works

The PartialEq macro performs field-by-field comparison using these strategies:

- **Primitives** (string, number, boolean, null, undefined) → Strict equality (`===`)

- **Date** → Compares timestamps via `getTime()`

- **Array** → Length check + element-by-element comparison

- **Map** → Size check + entry comparison

- **Set** → Size check + membership verification

- **Objects** → Calls `equals()` if available, otherwise `===`

## Field Options

### @partialEq(skip)

Use `@partialEq(skip)` to exclude a field from equality comparison:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user1 = new User(1, "Alice", new Date("2024-01-01"));
const user2 = new User(1, "Alice", new Date("2024-12-01"));

console.log(user1.equals(user2)); // true (createdAt is skipped)
```

## Type Safety

The generated `equals()` method accepts `unknown` and performs runtime type checking:

<InteractiveMacro code={`/** @derive(PartialEq) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}`} />

```typescript
const user = new User("Alice");

console.log(user.equals(new User("Alice"))); // true
console.log(user.equals("Alice")); // false (not a User instance)
```

## With Nested Objects

For objects with nested fields, PartialEq recursively calls `equals()` if available:

<InteractiveMacro code={`/** @derive(PartialEq) */
class Address {
  city: string;
  zip: string;

  constructor(city: string, zip: string) {
    this.city = city;
    this.zip = zip;
  }
}

/** @derive(PartialEq) */
class Person {
  name: string;
  address: Address;

  constructor(name: string, address: Address) {
    this.name = name;
    this.address = address;
  }
}`} />

```typescript
const addr1 = new Address("NYC", "10001");
const addr2 = new Address("NYC", "10001");

const p1 = new Person("Alice", addr1);
const p2 = new Person("Alice", addr2);

console.log(p1.equals(p2)); // true (deep equality via Address.equals)
```

## Interface Support

PartialEq works with interfaces. For interfaces, a namespace is generated with an `equals` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };
const p3: Point = { x: 5, y: 5 };

console.log(Point.equals(p1, p2)); // true
console.log(Point.equals(p1, p3)); // false
```

## Enum Support

PartialEq works with enums. For enums, strict equality comparison is used:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.equals(Status.Active, Status.Active));   // true
console.log(Status.equals(Status.Active, Status.Inactive)); // false
```

## Type Alias Support

PartialEq works with type aliases. For object types, field-by-field comparison is used:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const p1: Point = { x: 10, y: 20 };
const p2: Point = { x: 10, y: 20 };

console.log(Point.equals(p1, p2)); // true
```

For union types, strict equality is used:

<InteractiveMacro code={`/** @derive(PartialEq) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.equals("success", "success")); // true
console.log(ApiStatus.equals("success", "error"));   // false
```

## Common Patterns

### Finding Items in Arrays

<InteractiveMacro code={`/** @derive(PartialEq) */
class Product {
  sku: string;
  name: string;

  constructor(sku: string, name: string) {
    this.sku = sku;
    this.name = name;
  }
}`} />

```typescript
const products = [
  new Product("A1", "Widget"),
  new Product("B2", "Gadget"),
  new Product("C3", "Gizmo")
];

const target = new Product("B2", "Gadget");
const found = products.find(p => p.equals(target));

console.log(found); // Product { sku: "B2", name: "Gadget" }
```

### Use with Hash

When using objects as keys in Map-like structures, combine PartialEq with Hash:

<InteractiveMacro code={`/** @derive(PartialEq, Hash) */
class Key {
  id: number;
  type: string;

  constructor(id: number, type: string) {
    this.id = id;
    this.type = type;
  }
}`} />

```typescript
const k1 = new Key(1, "user");
const k2 = new Key(1, "user");

// Equal objects should have equal hash codes
console.log(k1.equals(k2));                   // true
console.log(k1.hashCode() === k2.hashCode()); // true
```

---

# PartialOrd

*The `PartialOrd` macro generates a `compareTo()` method that implements partial ordering, returning `-1`, `0`, `1`, or `null` for incomparable values.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const t1 = new Temperature(20);
const t2 = new Temperature(30);
const t3 = new Temperature(20);

console.log(t1.compareTo(t2)); // -1 (t1 < t2)
console.log(t2.compareTo(t1)); // 1  (t2 > t1)
console.log(t1.compareTo(t3)); // 0  (t1 == t3)

// Returns null for incomparable types
console.log(t1.compareTo("not a Temperature")); // null
```

## Return Values

The `compareTo()` method returns:

- `-1` → `this` is less than `other`

- `0` → `this` equals `other`

- `1` → `this` is greater than `other`

- `null` → Values are incomparable (e.g., different types)

## Comparison Logic

The PartialOrd macro compares fields in declaration order with type checking:

- `number` / `bigint` → Direct numeric comparison

- `string` → Uses `localeCompare()`

- `boolean` → `false < true`

- `Date` → Compares timestamps; returns `null` if not both Date instances

- `Array` → Lexicographic comparison; returns `null` if not both arrays

- `Object` → Calls `compareTo()` if available

- **Type mismatch** → Returns `null`

## Field Options

### @ord(skip)

Use `@ord(skip)` to exclude a field from ordering comparison:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const i1 = new Item(10, "Widget", "A useful widget");
const i2 = new Item(10, "Widget", "Different description");

console.log(i1.compareTo(i2)); // 0 (description is skipped)
```

## Handling Null Results

When using PartialOrd, always handle the `null` case:

<InteractiveMacro code={`/** @derive(PartialOrd) */
class Value {
  amount: number;

  constructor(amount: number) {
    this.amount = amount;
  }
}`} />

```typescript
function safeCompare(a: Value, b: unknown): string {
  const result = a.compareTo(b);
  if (result === null) {
    return "incomparable";
  } else if (result < 0) {
    return "less than";
  } else if (result > 0) {
    return "greater than";
  } else {
    return "equal";
  }
}

const v = new Value(100);
console.log(safeCompare(v, new Value(50)));  // "greater than"
console.log(safeCompare(v, "string"));       // "incomparable"
```

## Sorting with PartialOrd

When sorting, handle `null` values appropriately:

<InteractiveMacro code={`/** @derive(PartialOrd) */
class Score {
  value: number;

  constructor(value: number) {
    this.value = value;
  }
}`} />

```typescript
const scores = [
  new Score(100),
  new Score(50),
  new Score(75)
];

// Safe sort that handles null (treats null as equal)
scores.sort((a, b) => a.compareTo(b) ?? 0);
// Result: [Score(50), Score(75), Score(100)]
```

## Interface Support

PartialOrd works with interfaces. For interfaces, a namespace is generated with a `compareTo` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const m1: Measurement = { value: 10, unit: "kg" };
const m2: Measurement = { value: 10, unit: "lb" };

console.log(Measurement.compareTo(m1, m2)); // 1 (kg > lb alphabetically)
```

## Enum Support

PartialOrd works with enums:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Size.compareTo(Size.Small, Size.Large)); // -1
console.log(Size.compareTo(Size.Large, Size.Small)); // 1
```

## Type Alias Support

PartialOrd works with type aliases:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const i1: Interval = { start: 0, end: 10 };
const i2: Interval = { start: 0, end: 20 };

console.log(Interval.compareTo(i1, i2)); // -1
```

## PartialOrd vs Ord

Choose between `Ord` and `PartialOrd` based on your use case:

- **Ord** → Use when all values are always comparable (never returns null)

- **PartialOrd** → Use when comparing with `unknown` types or when some values might be incomparable

<InteractiveMacro code={`// PartialOrd is safer for public APIs that accept unknown input
/** @derive(PartialOrd) */
class SafeValue {
  data: number;
  constructor(data: number) {
    this.data = data;
  }

  // Can safely compare with any value
  isGreaterThan(other: unknown): boolean {
    const result = this.compareTo(other);
    return result !== null && result > 0;
  }
}`} />

```typescript
const safe = new SafeValue(100);
console.log(safe.isGreaterThan(new SafeValue(50)));  // true
console.log(safe.isGreaterThan("invalid"));          // false
```

---

# Serialize

*The `Serialize` macro generates a `toJSON()` method that converts your object to a JSON-compatible format with automatic handling of complex types like Date, Map, Set, and nested objects.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const user = new User("Alice", 30);
console.log(JSON.stringify(user));
// {"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}
```

## Automatic Type Handling

Serialize automatically handles various TypeScript types:

<table>
<thead>
<tr>
<th>Type</th>
<th>Serialization</th>
</tr>
</thead>
<tbody>
<tr>
<td>`string`, `number`, `boolean`</td>
<td>Direct copy</td>
</tr>
<tr>
<td>`Date`</td>
<td>`.toISOString()`</td>
</tr>
<tr>
<td>`T[]`</td>
<td>Maps items, calling `toJSON()` if available</td>
</tr>
<tr>
<td>`Map<K, V>`</td>
<td>`Object.fromEntries()`</td>
</tr>
<tr>
<td>`Set<T>`</td>
<td>`Array.from()`</td>
</tr>
<tr>
<td>Nested objects</td>
<td>Calls `toJSON()` if available</td>
</tr>
</tbody>
</table>

## Serde Options

Use the `@serde` decorator for fine-grained control over serialization:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = new User();
user.id = "123";
user.name = "Alice";
console.log(JSON.stringify(user));
// {"user_id":"123","full_name":"Alice"}
```

### Skipping Fields

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<Alert type="tip" title="skip vs skip_serializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_serializing: true` to only skip during serialization.
</Alert>

### Rename All Fields

Apply a naming convention to all fields at the container level:

<InteractiveMacro code={`/** @derive(Serialize) */
/** @serde({ rename_all: "camelCase" }) */
class ApiResponse {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

Supported conventions:

- `camelCase`

- `snake_case`

- `PascalCase`

- `SCREAMING_SNAKE_CASE`

- `kebab-case`

### Flattening Nested Objects

<InteractiveMacro code={`/** @derive(Serialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Serialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

```typescript
const user = new User();
user.name = "Alice";
user.address = { city: "NYC", zip: "10001" };
console.log(JSON.stringify(user));
// {"name":"Alice","city":"NYC","zip":"10001"}
```

## All Options

### Container Options (on class/interface)

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`rename_all`</td>
<td>`string`</td>
<td>Apply naming convention to all fields</td>
</tr>
</tbody>
</table>

### Field Options (on properties)

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`rename`</td>
<td>`string`</td>
<td>Use a different JSON key</td>
</tr>
<tr>
<td>`skip`</td>
<td>`boolean`</td>
<td>Exclude from serialization and deserialization</td>
</tr>
<tr>
<td>`skip_serializing`</td>
<td>`boolean`</td>
<td>Exclude from serialization only</td>
</tr>
<tr>
<td>`flatten`</td>
<td>`boolean`</td>
<td>Merge nested object fields into parent</td>
</tr>
</tbody>
</table>

## Interface Support

Serialize also works with interfaces. For interfaces, a namespace is generated with a `toJSON` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const response: ApiResponse = {
  status: 200,
  message: "OK",
  timestamp: new Date()
};

console.log(JSON.stringify(ApiResponse.toJSON(response)));
// {"status":200,"message":"OK","timestamp":"2024-01-15T10:30:00.000Z"}
```

## Enum Support

Serialize also works with enums. The `toJSON` function returns the underlying enum value (string or number):

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.toJSON(Status.Active));  // "active"
console.log(Status.toJSON(Status.Pending)); // "pending"
```

Works with numeric enums too:

<InteractiveMacro code={`/** @derive(Serialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

```typescript
console.log(Priority.toJSON(Priority.High)); // 3
```

## Type Alias Support

Serialize works with type aliases. For object types, fields are serialized with full type handling:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const profile: UserProfile = {
  id: "123",
  name: "Alice",
  createdAt: new Date("2024-01-15")
};

console.log(JSON.stringify(UserProfile.toJSON(profile)));
// {"id":"123","name":"Alice","createdAt":"2024-01-15T00:00:00.000Z"}
```

For union types, the value is returned directly:

<InteractiveMacro code={`/** @derive(Serialize) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.toJSON("success")); // "success"
```

## Combining with Deserialize

Use both Serialize and Deserialize for complete JSON round-trip support:

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
class User {
  name: string;
  createdAt: Date;
}`} />

```typescript
// Serialize
const user = new User();
user.name = "Alice";
user.createdAt = new Date();
const json = JSON.stringify(user);

// Deserialize
const parsed = User.fromJSON(JSON.parse(json));
console.log(parsed.createdAt instanceof Date); // true
```

---

# Deserialize

*The `Deserialize` macro generates a static `fromJSON()` method that parses JSON data into your class with runtime validation and automatic type conversion.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true
```

## Runtime Validation

Deserialize validates the input data and throws descriptive errors:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
// Missing required field
User.fromJSON({ name: "Alice" });
// Error: User.fromJSON: missing required field "email"

// Wrong type
User.fromJSON("not an object");
// Error: User.fromJSON: expected an object, got string

// Array instead of object
User.fromJSON([1, 2, 3]);
// Error: User.fromJSON: expected an object, got array
```

## Automatic Type Conversion

Deserialize automatically converts JSON types to their TypeScript equivalents:

<table>
<thead>
<tr>
<th>JSON Type</th>
<th>TypeScript Type</th>
<th>Conversion</th>
</tr>
</thead>
<tbody>
<tr>
<td>string/number/boolean</td>
<td>`string`/`number`/`boolean`</td>
<td>Direct assignment</td>
</tr>
<tr>
<td>ISO string</td>
<td>`Date`</td>
<td>`new Date(string)`</td>
</tr>
<tr>
<td>array</td>
<td>`T[]`</td>
<td>Maps items with auto-detection</td>
</tr>
<tr>
<td>object</td>
<td>`Map<K, V>`</td>
<td>`new Map(Object.entries())`</td>
</tr>
<tr>
<td>array</td>
<td>`Set<T>`</td>
<td>`new Set(array)`</td>
</tr>
<tr>
<td>object</td>
<td>Nested class</td>
<td>Calls `fromJSON()` if available</td>
</tr>
</tbody>
</table>

## Serde Options

Use the `@serde` decorator to customize deserialization:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"
```

### Default Values

<MacroExample before={data.examples.default.before} after={data.examples.default.after} />

```typescript
const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false
```

### Skipping Fields

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}`} />

<Alert type="tip" title="skip vs skip_deserializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_deserializing: true` to only skip during deserialization.
</Alert>

### Deny Unknown Fields

<InteractiveMacro code={`/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}`} />

```typescript
// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"
```

### Flatten Nested Objects

<InteractiveMacro code={`/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

```typescript
// Flat JSON structure
const user = User.fromJSON({
  name: "Alice",
  city: "NYC",
  zip: "10001"
});
console.log(user.address.city); // "NYC"
```

## All Options

### Container Options (on class/interface)

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`rename_all`</td>
<td>`string`</td>
<td>Apply naming convention to all fields</td>
</tr>
<tr>
<td>`deny_unknown_fields`</td>
<td>`boolean`</td>
<td>Throw error if JSON has unknown keys</td>
</tr>
</tbody>
</table>

### Field Options (on properties)

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`rename`</td>
<td>`string`</td>
<td>Use a different JSON key</td>
</tr>
<tr>
<td>`skip`</td>
<td>`boolean`</td>
<td>Exclude from serialization and deserialization</td>
</tr>
<tr>
<td>`skip_deserializing`</td>
<td>`boolean`</td>
<td>Exclude from deserialization only</td>
</tr>
<tr>
<td>`default`</td>
<td>`boolean | string`</td>
<td>Use TypeScript default or custom expression if missing</td>
</tr>
<tr>
<td>`flatten`</td>
<td>`boolean`</td>
<td>Merge nested object fields from parent</td>
</tr>
</tbody>
</table>

## Interface Support

Deserialize also works with interfaces. For interfaces, a namespace is generated with `is` (type guard) and `fromJSON` functions:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const json = { status: 200, message: "OK", timestamp: "2024-01-15T10:30:00.000Z" };

// Type guard
if (ApiResponse.is(json)) {
  console.log(json.status); // TypeScript knows this is ApiResponse
}

// Deserialize with validation
const response = ApiResponse.fromJSON(json);
console.log(response.timestamp instanceof Date); // true
```

## Enum Support

Deserialize also works with enums. The `fromJSON` function validates that the input matches one of the enum values:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const status = Status.fromJSON("active");
console.log(status); // Status.Active

// Invalid values throw an error
try {
  Status.fromJSON("invalid");
} catch (e) {
  console.log(e.message); // "Invalid Status value: invalid"
}
```

Works with numeric enums too:

<InteractiveMacro code={`/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

```typescript
const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High
```

## Type Alias Support

Deserialize works with type aliases. For object types, validation and type conversion is applied:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true
```

For union types, basic validation is applied:

<InteractiveMacro code={`/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
const status = ApiStatus.fromJSON("success");
console.log(status); // "success"
```

## Combining with Serialize

Use both Serialize and Deserialize for complete JSON round-trip support:

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

```typescript
// Create and serialize
const profile = new UserProfile();
profile.user_name = "Alice";
profile.created_at = new Date();
profile.is_active = true;

const json = JSON.stringify(profile);
// {"userName":"Alice","createdAt":"2024-...","isActive":true}

// Deserialize back
const restored = UserProfile.fromJSON(JSON.parse(json));
console.log(restored.user_name);              // "Alice"
console.log(restored.created_at instanceof Date); // true
```

## Error Handling

Handle deserialization errors gracefully:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
function parseUser(json: unknown): User | null {
  try {
    return User.fromJSON(json);
  } catch (error) {
    console.error("Failed to parse user:", error.message);
    return null;
  }
}

const user = parseUser({ name: "Alice" });
// Logs: Failed to parse user: User.fromJSON: missing required field "email"
// Returns: null
```

---

# Custom Macros

# Custom Macros

*Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.*

## Overview

Custom macros are written in Rust and compiled to native Node.js addons. The process involves:

1. Creating a Rust crate with NAPI bindings

2. Defining macro functions with `#[ts_macro_derive]`

3. Using `macroforge_ts_quote` to generate TypeScript code

4. Building and publishing as an npm package

## Quick Example

```rust
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
```

## Using Custom Macros

Once your macro package is published, users can import and use it:

```typescript
/** import macro { JSON } from "@my/macros"; */

/** @derive(JSON) */
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
```

>
> The `import macro` comment tells Macroforge which package provides the macro.

## Getting Started

Follow these guides to create your own macros:

- [Set up a Rust macro crate]({base}/docs/custom-macros/rust-setup)

- [Learn the #[ts_macro_derive] attribute]({base}/docs/custom-macros/ts-macro-derive)

- [Learn the template syntax]({base}/docs/custom-macros/ts-quote)

---

# Rust Setup

*Create a new Rust crate that will contain your custom macros. This crate compiles to a native Node.js addon.*

## Prerequisites

- Rust toolchain (1.70 or later)

- Node.js 18 or later

- NAPI-RS CLI: `npm install -g @napi-rs/cli`

## Create the Project

```bash
# Create a new directory
mkdir my-macros
cd my-macros

# Initialize with NAPI-RS
napi new --platform --name my-macros
```

## Configure Cargo.toml

Update your `Cargo.toml` with the required dependencies:

`Cargo.toml`
```toml
[package]
name = "my-macros"
version = "0.1.0"
edition = "2021"

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
```

## Create build.rs

`build.rs`
```rust
fn main() {
    napi_build::setup();
}
```

## Create src/lib.rs

`src/lib.rs`
```rust
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
```

## Create package.json

`package.json`
```json
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
```

## Build the Package

```bash
# Build the native addon
npm run build

# This creates:
# - index.js (JavaScript bindings)
# - index.d.ts (TypeScript types)
# - *.node (native binary)
```

>
> For cross-platform builds, use GitHub Actions with the NAPI-RS CI template.

## Next Steps

- [Learn the #[ts_macro_derive] attribute]({base}/docs/custom-macros/ts-macro-derive)

- [Master the template syntax]({base}/docs/custom-macros/ts-quote)

---

# ts_macro_derive

*The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*

## Basic Syntax

```rust
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}
```

## Attribute Options

### Name (Required)

The first argument is the macro name that users will reference in `@derive()`:

```rust
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
```

### Description

Provides documentation for the macro:

```rust
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
```

### Attributes

Declare which field-level decorators your macro accepts:

```rust
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(debug)  // Allows @debug({ ... }) on fields
)]
pub fn derive_debug(...)
```

>
> Declared attributes become available as `@attributeName(&#123; options &#125;)` decorators in TypeScript.

## Function Signature

```rust
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>
```

<table>
<thead>
<tr>
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`input: TsStream`</td>
<td>Token stream containing the class/interface AST</td>
</tr>
<tr>
<td>`Result<TsStream, MacroforgeError>`</td>
<td>Returns generated code or an error with source location</td>
</tr>
</tbody>
</table>

## Parsing Input

Use `parse_ts_macro_input!` to convert the token stream:

```rust
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
```

## DeriveInput Structure

```rust
struct DeriveInput {
    pub ident: Ident,           // The type name
    pub span: SpanIR,           // Span of the type definition
    pub attrs: Vec<Attribute>,  // Decorators (excluding @derive)
    pub data: Data,             // The parsed type data
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
    fn body_span(&self) -> SpanIR;      // For inserting code into class body
    fn type_params(&self) -> &[String]; // Generic type parameters
    fn heritage(&self) -> &[String];    // extends/implements clauses
    fn is_abstract(&self) -> bool;
}

impl DataInterface {
    fn fields(&self) -> &[InterfaceFieldIR];
    fn methods(&self) -> &[InterfaceMethodIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
    fn body_span(&self) -> SpanIR;
    fn type_params(&self) -> &[String];
    fn heritage(&self) -> &[String];    // extends clauses
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
```

## Accessing Field Data

### Class Fields (FieldIR)

```rust
struct FieldIR {
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
}
```

### Interface Fields (InterfaceFieldIR)

```rust
struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}
```

### Enum Variants (EnumVariantIR)

```rust
struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // Auto, String(String), or Number(f64)
    pub decorators: Vec<DecoratorIR>,
}
```

### Decorator Structure

```rust
struct DecoratorIR {
    pub name: String,      // e.g., "serde"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}
```

>
> To check for decorators, iterate through `field.decorators` and check `decorator.name`. For parsing options, you can write helper functions like the built-in macros do.

## Adding Imports

If your macro generates code that requires imports, use the `add_import` method on `TsStream`:

```rust
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
```

>
> Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again.

## Returning Errors

Use `MacroforgeError` to report errors with source locations:

```rust
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
```

## Complete Example

```rust
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
```

## Next Steps

- [Learn the template syntax]({base}/docs/custom-macros/ts-quote)

---

# Template Syntax

*The `macroforge_ts_quote` crate provides template-based code generation for TypeScript. The `ts_template!` macro uses Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*

## Available Macros

<table>
<thead>
<tr>
<th>Macro</th>
<th>Output</th>
<th>Use Case</th>
</tr>
</thead>
<tbody>
<tr>
<td>`ts_template!`</td>
<td>Any TypeScript code</td>
<td>General code generation</td>
</tr>
<tr>
<td>`body!`</td>
<td>Class body members</td>
<td>Methods and properties</td>
</tr>
</tbody>
</table>

## Quick Reference

<table>
<thead>
<tr>
<th>Syntax</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`@&#123;expr&#125;`</td>
<td>Interpolate a Rust expression (adds space after)</td>
</tr>
<tr>
<td>`&#123;| content |&#125;`</td>
<td>Ident block: concatenates without spaces (e.g., `&#123;|get@&#123;name&#125;|&#125;` → `getUser`)</td>
</tr>
<tr>
<td>`@@&#123;`</td>
<td>Escape for literal `@&#123;` (e.g., `"@@&#123;foo&#125;"` → `@&#123;foo&#125;`)</td>
</tr>
<tr>
<td>`"text @&#123;expr&#125;"`</td>
<td>String interpolation (auto-detected)</td>
</tr>
<tr>
<td>`"'^template $&#123;js&#125;^'"`</td>
<td>JS backtick template literal (outputs ``template $&#123;js&#125;``)</td>
</tr>
<tr>
<td>`&#123;#if cond&#125;...&#123;/if&#125;`</td>
<td>Conditional block</td>
</tr>
<tr>
<td>`&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;`</td>
<td>Conditional with else</td>
</tr>
<tr>
<td>`&#123;#if a&#125;...&#123;:else if b&#125;...&#123;:else&#125;...&#123;/if&#125;`</td>
<td>Full if/else-if/else chain</td>
</tr>
<tr>
<td>`&#123;#if let pattern = expr&#125;...&#123;/if&#125;`</td>
<td>Pattern matching if-let</td>
</tr>
<tr>
<td>`&#123;#match expr&#125;&#123;:case pattern&#125;...&#123;/match&#125;`</td>
<td>Match expression with case arms</td>
</tr>
<tr>
<td>`&#123;#for item in list&#125;...&#123;/for&#125;`</td>
<td>Iterate over a collection</td>
</tr>
<tr>
<td>`&#123;#while cond&#125;...&#123;/while&#125;`</td>
<td>While loop</td>
</tr>
<tr>
<td>`&#123;#while let pattern = expr&#125;...&#123;/while&#125;`</td>
<td>While-let pattern matching loop</td>
</tr>
<tr>
<td>`&#123;$let name = expr&#125;`</td>
<td>Define a local constant</td>
</tr>
<tr>
<td>`&#123;$let mut name = expr&#125;`</td>
<td>Define a mutable local variable</td>
</tr>
<tr>
<td>`&#123;$do expr&#125;`</td>
<td>Execute a side-effectful expression</td>
</tr>
<tr>
<td>`&#123;$typescript stream&#125;`</td>
<td>Inject a TsStream, preserving its source and runtime_patches (imports)</td>
</tr>
</tbody>
</table>

**Note:** A single `@` not followed by `&#123;` passes through unchanged (e.g., `email@domain.com` works as expected).

## Interpolation: `@&#123;expr&#125;`

Insert Rust expressions into the generated TypeScript:

```rust
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};
```

**Generates:**

```typescript
User.prototype.toString = function () {
  return "User instance";
};
```

## Identifier Concatenation: `&#123;| content |&#125;`

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `&#123;| |&#125;` is concatenated without spaces:

```rust
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
```

**Generates:**

```typescript
function getUser() {
  return this.user;
}
```

Without ident blocks, `@&#123;&#125;` always adds a space after for readability. Use `&#123;| |&#125;` when you explicitly want concatenation:

```rust
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
```

Multiple interpolations can be combined:

```rust
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
```

## String Interpolation: `"text @&#123;expr&#125;"`

Interpolation works automatically inside string literals - no `format!()` needed:

```rust
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
```

**Generates:**

```typescript
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
```

This also works with method calls and complex expressions:

```rust
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
```

## Backtick Template Literals: `"'^...^'"`

For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through `${"${}"}` for JS interpolation:

```rust
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>\${content}</@{tag_name}>^'";
};
```

**Generates:**

<CodeBlock code={'const html = `${content}`;'} lang="typescript" />

You can mix Rust `@&#123;&#125;` interpolation (evaluated at macro expansion time) with JS `${"${}"}` interpolation (evaluated at runtime):

```rust
let class_name = "User";

let code = ts_template! {
    "'^Hello \${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

<CodeBlock code={'`Hello ${this.name}, you are a User`'} lang="typescript" />

## Conditionals: `&#123;#if&#125;...&#123;/if&#125;`

Basic conditional:

```rust
let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};
```

### If-Else

```rust
let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};
```

### If-Else-If Chains

```rust
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
```

## Pattern Matching: `&#123;#if let&#125;`

Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:

```rust
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
```

**Generates:**

```typescript
console.log("Hello, Alice!");
```

This is useful when working with optional values from your IR:

```rust
let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};
```

## Match Expressions: `&#123;#match&#125;`

Use `match` for exhaustive pattern matching:

```rust
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
```

**Generates:**

```typescript
public field: string;
```

### Match with Value Extraction

```rust
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
```

### Match with Wildcard

```rust
let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};
```

## Iteration: `&#123;#for&#125;`

```rust
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
```

**Generates:**

```typescript
function toJSON() {
  const result = {};
  result.name = this.name;
  result.email = this.email;
  result.age = this.age;
  return result;
}
```

### Tuple Destructuring in Loops

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        const @{key} = new @{class_name}();
    {/for}
};
```

### Nested Iterations

```rust
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
```

## While Loops: `&#123;#while&#125;`

Use `while` for loops that need to continue until a condition is false:

```rust
let items = get_items();
let mut idx = 0;

let code = ts_template! {
    {$let mut i = 0}
    {#while i < items.len()}
        console.log("Item @{i}");
        {$do i += 1}
    {/while}
};
```

### While-Let Pattern Matching

Use `while let` for iterating with pattern matching, similar to `if let`:

```rust
let mut items = vec!["a", "b", "c"].into_iter();

let code = ts_template! {
    {#while let Some(item) = items.next()}
        console.log("@{item}");
    {/while}
};
```

**Generates:**

```typescript
console.log("a");
console.log("b");
console.log("c");
```

This is especially useful when working with iterators or consuming optional values:

```rust
let code = ts_template! {
    {#while let Some(next_field) = remaining_fields.pop()}
        result.@{next_field.name} = this.@{next_field.name};
    {/while}
};
```

## Local Constants: `&#123;$let&#125;`

Define local variables within the template scope:

```rust
let items = vec![("user", "User"), ("post", "Post")];

let code = ts_template! {
    {#for (key, class_name) in items}
        {$let upper = class_name.to_uppercase()}
        console.log("Processing @{upper}");
        const @{key} = new @{class_name}();
    {/for}
};
```

This is useful for computing derived values inside loops without cluttering the Rust code.

## Mutable Variables: `&#123;$let mut&#125;`

When you need to modify a variable within the template (e.g., in a `while` loop), use `&#123;$let mut&#125;`:

```rust
let code = ts_template! {
    {$let mut count = 0}
    {#for item in items}
        console.log("Item @{count}: @{item}");
        {$do count += 1}
    {/for}
    console.log("Total: @{count}");
};
```

## Side Effects: `&#123;$do&#125;`

Execute an expression for its side effects without producing output. This is commonly used with mutable variables:

```rust
let code = ts_template! {
    {$let mut results: Vec<String> = Vec::new()}
    {#for field in fields}
        {$do results.push(format!("this.{}", field))}
    {/for}
    return [@{results.join(", ")}];
};
```

Common uses for `&#123;$do&#125;`:

- Incrementing counters: `&#123;$do i += 1&#125;`

- Building collections: `&#123;$do vec.push(item)&#125;`

- Setting flags: `&#123;$do found = true&#125;`

- Any mutating operation

## TsStream Injection: `&#123;$typescript&#125;`

Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via `add_import()`):

```rust
// Create a helper method with its own import
let mut helper = body! {
    validateEmail(email: string): boolean {
        return Result.ok(true);
    }
};
helper.add_import("Result", "macroforge/result");

// Inject the helper into the main template
let result = body! {
    {$typescript helper}

    process(data: Record<string, unknown>): void {
        // ...
    }
};
// result now includes helper's source AND its Result import
```

This is essential for composing multiple macro outputs while preserving imports and patches:

```rust
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
```

## Escape Syntax

If you need a literal `@&#123;` in your output (not interpolation), use `@@&#123;`:

```rust
ts_template! {
    // This outputs a literal @{foo}
    const example = "Use @@{foo} for templates";
}
```

**Generates:**

```typescript
// This outputs a literal @{foo}
const example = "Use @{foo} for templates";
```

## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual AST Building)

```rust
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
```

### After (With ts_template!)

```rust
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
```

## How It Works

1. **Compile-Time:** The template is parsed during macro expansion

2. **String Building:** Generates Rust code that builds a TypeScript string at runtime

3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST

4. **Result:** Returns `Stmt` that can be used in `MacroResult` patches

## Return Type

`ts_template!` returns a `Result<Stmt, TsSynError>` by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:

```text
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `&#123;&#125;` are recognized as either:

- **Template tags** if they start with `#`, `$`, `:`, or `/`

- **Regular TypeScript blocks** otherwise

```rust
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
```

## Comparison with Alternatives

<table>
<thead>
<tr>
<th>Approach</th>
<th>Pros</th>
<th>Cons</th>
</tr>
</thead>
<tbody>
<tr>
<td>`ts_quote!`</td>
<td>Compile-time validation, type-safe</td>
<td>Can't handle Vec<Stmt>, verbose</td>
</tr>
<tr>
<td>`parse_ts_str()`</td>
<td>Maximum flexibility</td>
<td>Runtime parsing, less readable</td>
</tr>
<tr>
<td>`ts_template!`</td>
<td>Readable, handles loops/conditions</td>
<td>Small runtime parsing overhead</td>
</tr>
</tbody>
</table>

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

<table>
<thead>
<tr>
<th>Integration</th>
<th>Purpose</th>
<th>Package</th>
</tr>
</thead>
<tbody>
<tr>
<td>TypeScript Plugin</td>
<td>IDE support (errors, completions)</td>
<td>`@macroforge/typescript-plugin`</td>
</tr>
<tr>
<td>Vite Plugin</td>
<td>Build-time macro expansion</td>
<td>`@macroforge/vite-plugin`</td>
</tr>
</tbody>
</table>

## Recommended Setup

For the best development experience, use both integrations:

1. **TypeScript Plugin**: Provides real-time feedback in your IDE

2. **Vite Plugin**: Expands macros during development and production builds

```bash
# Install both plugins
npm install -D @macroforge/typescript-plugin @macroforge/vite-plugin
```

## How They Work Together

<IntegrationFlow
source="Your Code"
sourceDesc="TypeScript with @derive decorators"
branches={[
{
plugin: 'TypeScript Plugin',
pluginDesc: 'Language service integration',
outputs: [
{ label: 'IDE Feedback', desc: 'Errors & completions' }
]
},
{
plugin: 'Vite Plugin',
pluginDesc: 'Build-time transformation',
outputs: [
{ label: 'Dev Server', desc: 'Hot reload' },
{ label: 'Production Build', desc: 'Optimized output' }
]
}
]}
/>

## Detailed Guides

- [TypeScript Plugin setup]({base}/docs/integration/typescript-plugin)

- [Vite Plugin configuration]({base}/docs/integration/vite-plugin)

- [Configuration options]({base}/docs/integration/configuration)

---

# Command Line Interface

*The `macroforge` CLI provides commands for expanding macros and running type checks with macro support.*

## Installation

The CLI is a Rust binary. You can install it using Cargo:

```bash
cargo install macroforge_ts
```

Or build from source:

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge
```

## Commands

### macroforge expand

Expands macros in a TypeScript file and outputs the transformed code.

```bash
macroforge expand <input> [options]
```

#### Arguments

<table>
<thead>
<tr>
<th>Argument</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`<input>`</td>
<td>Path to the TypeScript or TSX file to expand</td>
</tr>
</tbody>
</table>

#### Options

<table>
<thead>
<tr>
<th>Option</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`--out <path>`</td>
<td>Write the expanded JavaScript/TypeScript to a file</td>
</tr>
<tr>
<td>`--types-out <path>`</td>
<td>Write the generated `.d.ts` declarations to a file</td>
</tr>
<tr>
<td>`--print`</td>
<td>Print output to stdout even when `--out` is specified</td>
</tr>
<tr>
<td>`--builtin-only`</td>
<td>Use only built-in Rust macros (faster, but no external macro support)</td>
</tr>
</tbody>
</table>

#### Examples

Expand a file and print to stdout:

```bash
macroforge expand src/user.ts
```

Expand and write to a file:

```bash
macroforge expand src/user.ts --out dist/user.js
```

Expand with both runtime output and type declarations:

```bash
macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts
```

Use fast built-in macros only (no external macro support):

```bash
macroforge expand src/user.ts --builtin-only
```

>
> By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where `macroforge` and any external macro packages are installed in `node_modules`.

### macroforge tsc

Runs TypeScript type checking with macro expansion. This wraps `tsc --noEmit` and expands macros before type checking, so your generated methods are properly type-checked.

```bash
macroforge tsc [options]
```

#### Options

<table>
<thead>
<tr>
<th>Option</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`-p, --project <path>`</td>
<td>Path to `tsconfig.json` (defaults to `tsconfig.json` in current directory)</td>
</tr>
</tbody>
</table>

#### Examples

Type check with default tsconfig.json:

```bash
macroforge tsc
```

Type check with a specific config:

```bash
macroforge tsc -p tsconfig.build.json
```

## Output Format

### Expanded Code

When expanding a file like this:

```typescript
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```

The CLI outputs the expanded code with the generated methods:

```typescript
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  [Symbol.for("nodejs.util.inspect.custom")](): string {
    return \`User { name: \${this.name}, age: \${this.age} }\`;
  }
}
```

### Diagnostics

Errors and warnings are printed to stderr in a readable format:

```text
[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used
```

## Use Cases

### CI/CD Type Checking

Use `macroforge tsc` in your CI pipeline to type-check with macro expansion:

```json
# package.json
{
  "scripts": {
    "typecheck": "macroforge tsc"
  }
}
```

### Debugging Macro Output

Use `macroforge expand` to inspect what code your macros generate:

```bash
macroforge expand src/models/user.ts | less
```

### Build Pipeline

Generate expanded files as part of a custom build:

```bash
#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done
```

## Built-in vs Full Mode

By default, the CLI uses Node.js for full macro support including external macros. Use `--builtin-only` for faster expansion when you only need built-in macros:

<table>
<thead>
<tr>
<th>Feature</th>
<th>Default (Node.js)</th>
<th>`--builtin-only` (Rust)</th>
</tr>
</thead>
<tbody>
<tr>
<td>Built-in macros</td>
<td>Yes</td>
<td>Yes</td>
</tr>
<tr>
<td>External macros</td>
<td>Yes</td>
<td>No</td>
</tr>
<tr>
<td>Performance</td>
<td>Standard</td>
<td>Faster</td>
</tr>
<tr>
<td>Dependencies</td>
<td>Requires `macroforge` in node_modules</td>
<td>None</td>
</tr>
</tbody>
</table>

---

# TypeScript Plugin

*The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.*

## Installation

```bash
npm install -D @macroforge/typescript-plugin
```

## Configuration

Add the plugin to your `tsconfig.json`:

`tsconfig.json`
```json
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}
```

## VS Code Setup

VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):

1. Open the Command Palette (`Cmd/Ctrl + Shift + P`)

2. Search for "TypeScript: Select TypeScript Version"

3. Choose "Use Workspace Version"

>
> Add this setting to your `.vscode/settings.json` to make it permanent:

`.vscode/settings.json`
```json
{
  "typescript.tsdk": "node_modules/typescript/lib"
}
```

## Features

### Error Reporting

Errors in macro-generated code are reported at the `@derive` decorator position:

```typescript
/** @derive(Debug) */  // <- Errors appear here
class User {
  name: string;
}
```

### Completions

The plugin provides completions for generated methods:

```typescript
const user = new User("Alice");
user.to  // Suggests: toString(), toJSON(), etc.
```

### Type Information

Hover over generated methods to see their types:

```typescript
// Hover over 'clone' shows:
// (method) User.clone(): User
const copy = user.clone();
```

## Troubleshooting

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

```bash
npm install -D @macroforge/vite-plugin
```

## Configuration

Add the plugin to your `vite.config.ts`:

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge()
  ]
});
```

## Options

```typescript
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
```

### Option Reference

<table>
<thead>
<tr>
<th>Option</th>
<th>Type</th>
<th>Default</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`generateTypes`</td>
<td>`boolean`</td>
<td>`true`</td>
<td>Generate .d.ts files</td>
</tr>
<tr>
<td>`typesOutputDir`</td>
<td>`string`</td>
<td>`.macroforge/types`</td>
<td>Where to write type files</td>
</tr>
<tr>
<td>`emitMetadata`</td>
<td>`boolean`</td>
<td>`false`</td>
<td>Emit macro metadata files</td>
</tr>
<tr>
<td>`keepDecorators`</td>
<td>`boolean`</td>
<td>`false`</td>
<td>Keep decorators in output</td>
</tr>
</tbody>
</table>

## Framework Integration

### React (Vite)

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before React plugin
    react()
  ]
});
```

### SvelteKit

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge(),  // Before SvelteKit
    sveltekit()
  ]
});
```

>
> Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first.

## Development Server

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

`macroforge.json`
```json
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
```

## Options Reference

### allowNativeMacros

<table>
<tbody>
<tr>
<td>Type</td>
<td>`boolean`</td>
</tr>
<tr>
<td>Default</td>
<td>`true`</td>
</tr>
</tbody>
</table>

Enable or disable native (Rust) macro packages. Set to `false` to only allow built-in macros.

### macroPackages

<table>
<tbody>
<tr>
<td>Type</td>
<td>`string[]`</td>
</tr>
<tr>
<td>Default</td>
<td>`[]`</td>
</tr>
</tbody>
</table>

List of npm packages that provide macros. Macroforge will look for macros in these packages.

```json
{
  "macroPackages": [
    "@my-org/custom-macros",
    "community-macros"
  ]
}
```

### keepDecorators

<table>
<tbody>
<tr>
<td>Type</td>
<td>`boolean`</td>
</tr>
<tr>
<td>Default</td>
<td>`false`</td>
</tr>
</tbody>
</table>

Keep `@derive` decorators in the output. Useful for debugging.

### limits

Configure resource limits for macro expansion:

```json
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
```

## Macro Runtime Overrides

Override settings for specific macros:

```json
{
  "macroRuntimeOverrides": {
    "@my-org/macros": {
      "maxExecutionTimeMs": 10000
    }
  }
}
```

> **Warning:**
> Be careful when increasing limits, as this could allow malicious macros to consume excessive resources.

## Environment Variables

Some settings can be overridden with environment variables:

<table>
<thead>
<tr>
<th>Variable</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`MACROFORGE_DEBUG`</td>
<td>Enable debug logging</td>
</tr>
<tr>
<td>`MACROFORGE_LOG_FILE`</td>
<td>Write logs to a file</td>
</tr>
</tbody>
</table>

```bash
MACROFORGE_DEBUG=1 npm run dev
```

---

# Language Servers

# Language Servers

*Macroforge provides language server integrations for enhanced IDE support beyond the TypeScript plugin.*

<Alert type="warning" title="Work in Progress">
Language server integrations are currently experimental. They work in the repository but are not yet published as official extensions. You'll need to fork the repo and install them as developer extensions.
</Alert>

## Overview

While the [TypeScript Plugin]({base}/docs/integration/typescript-plugin) provides macro support in any TypeScript-aware editor, dedicated language servers offer deeper integration for specific frameworks and editors.

<table>
<thead>
<tr>
<th>Integration</th>
<th>Purpose</th>
<th>Status</th>
</tr>
</thead>
<tbody>
<tr>
<td>[Svelte Language Server]({base}/docs/language-servers/svelte)</td>
<td>Full Svelte support with macroforge</td>
<td>Working (dev install)</td>
</tr>
<tr>
<td>[Zed Extensions]({base}/docs/language-servers/zed)</td>
<td>VTSLS and Svelte for Zed editor</td>
<td>Working (dev install)</td>
</tr>
</tbody>
</table>

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

- [Svelte Language Server]({base}/docs/language-servers/svelte) - Full Svelte IDE support

- [Zed Extensions]({base}/docs/language-servers/zed) - VTSLS and Svelte for Zed

---

# Svelte Language Server

*`@macroforge/svelte-language-server` provides full Svelte IDE support with macroforge integration.*

<Alert type="warning" title="Developer Installation Required">
This package is not yet published as an official extension. You'll need to build and install it manually.
</Alert>

## Features

- **Svelte syntax diagnostics** - Errors and warnings in .svelte files

- **HTML support** - Hover info, autocompletions, Emmet, outline symbols

- **CSS/SCSS/LESS** - Diagnostics, hover, completions, formatting, Emmet, color picking

- **TypeScript/JavaScript** - Full language features with macroforge macro expansion

- **Go-to-definition** - Navigate to macro-generated code

- **Code actions** - Quick fixes and refactorings

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts
```

### 2. Build the Language Server

```bash
# Install dependencies
npm install

# Build the Svelte language server
cd packages/svelte-language-server
npm run build
```

### 3. Configure Your Editor

The language server exposes a `svelteserver` binary that implements the Language Server Protocol (LSP). Configure your editor to use it:

```bash
# The binary is located at:
./packages/svelte-language-server/bin/server.js
```

## Package Info

<table>
<tbody>
<tr>
<td>Package</td>
<td>`@macroforge/svelte-language-server`</td>
</tr>
<tr>
<td>Version</td>
<td>0.1.7</td>
</tr>
<tr>
<td>CLI Command</td>
<td>`svelteserver`</td>
</tr>
<tr>
<td>Node Version</td>
<td>>= 18.0.0</td>
</tr>
</tbody>
</table>

## How It Works

The Svelte language server extends the standard Svelte language tooling with macroforge integration:

1. Parses `.svelte` files and extracts TypeScript/JavaScript blocks

2. Expands macros using the `@macroforge/typescript-plugin`

3. Maps diagnostics back to original source positions

4. Provides completions for macro-generated methods

## Using with Zed

For Zed editor, see the [Zed Extensions]({base}/docs/language-servers/zed) page for the dedicated `svelte-macroforge` extension.

---

# Zed Extensions

*Macroforge provides two extensions for the [Zed editor](https://zed.dev): one for TypeScript via VTSLS, and one for Svelte.*

<Alert type="warning" title="Developer Installation Required">
These extensions are not yet in the Zed extension registry. You'll need to install them as developer extensions.
</Alert>

## Available Extensions

<table>
<thead>
<tr>
<th>Extension</th>
<th>Description</th>
<th>Location</th>
</tr>
</thead>
<tbody>
<tr>
<td>`vtsls-macroforge`</td>
<td>VTSLS with macroforge support for TypeScript</td>
<td>`crates/extensions/vtsls-macroforge`</td>
</tr>
<tr>
<td>`svelte-macroforge`</td>
<td>Svelte language support with macroforge</td>
<td>`crates/extensions/svelte-macroforge`</td>
</tr>
</tbody>
</table>

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts
```

### 2. Build the Extension

Build the extension you want to use:

```bash
# For VTSLS (TypeScript)
cd crates/extensions/vtsls-macroforge

# Or for Svelte
cd crates/extensions/svelte-macroforge
```

### 3. Install as Dev Extension in Zed

In Zed, open the command palette and run **zed: install dev extension**, then select the extension directory.

Alternatively, symlink the extension to your Zed extensions directory:

```bash
# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/.config/zed/extensions/installed/vtsls-macroforge
```

## vtsls-macroforge

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

---

# API Reference

# API Reference

*Macroforge provides a programmatic API for expanding macros in TypeScript code.*

## Overview

```typescript
import {
  expandSync,
  transformSync,
  checkSyntax,
  parseImportSources,
  NativePlugin,
  PositionMapper
} from "macroforge";
```

## Core Functions

<table>
<thead>
<tr>
<th>Function</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>[`expandSync()`]({base}/docs/api/expand-sync)</td>
<td>Expand macros synchronously</td>
</tr>
<tr>
<td>[`transformSync()`]({base}/docs/api/transform-sync)</td>
<td>Transform code with additional metadata</td>
</tr>
<tr>
<td>`checkSyntax()`</td>
<td>Validate TypeScript syntax</td>
</tr>
<tr>
<td>`parseImportSources()`</td>
<td>Extract import information</td>
</tr>
</tbody>
</table>

## Classes

<table>
<thead>
<tr>
<th>Class</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>[`NativePlugin`]({base}/docs/api/native-plugin)</td>
<td>Stateful plugin with caching</td>
</tr>
<tr>
<td>[`PositionMapper`]({base}/docs/api/position-mapper)</td>
<td>Maps positions between original and expanded code</td>
</tr>
</tbody>
</table>

## Quick Example

```typescript
import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts", {
  keepDecorators: false
});

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) {
  console.error("Errors:", result.diagnostics);
}
```

## Detailed Reference

- [`expandSync()`]({base}/docs/api/expand-sync) - Full options and return types

- [`transformSync()`]({base}/docs/api/transform-sync) - Transform with source maps

- [`NativePlugin`]({base}/docs/api/native-plugin) - Caching for language servers

- [`PositionMapper`]({base}/docs/api/position-mapper) - Position mapping utilities

---

# expandSync()

*Expands macros in TypeScript code synchronously and returns the transformed output.*

## Signature

```typescript
function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult
```

## Parameters

<table>
<thead>
<tr>
<th>Parameter</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`code`</td>
<td>`string`</td>
<td>TypeScript source code to transform</td>
</tr>
<tr>
<td>`filepath`</td>
<td>`string`</td>
<td>File path (used for error reporting)</td>
</tr>
<tr>
<td>`options`</td>
<td>`ExpandOptions`</td>
<td>Optional configuration</td>
</tr>
</tbody>
</table>

## ExpandOptions

```typescript
interface ExpandOptions {
  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;
}
```

## ExpandResult

```typescript
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
```

## MacroDiagnostic

```typescript
interface MacroDiagnostic {
  message: string;
  severity: "error" | "warning" | "info";
  span: {
    start: number;
    end: number;
  };
}
```

## Example

```typescript
import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts");

console.log("Transformed code:");
console.log(result.code);

if (result.types) {
  console.log("Type declarations:");
  console.log(result.types);
}

if (result.diagnostics.length > 0) {
  for (const diag of result.diagnostics) {
    console.log(\`[\${diag.severity}] \${diag.message}\`);
  }
}
```

## Error Handling

Syntax errors and macro errors are returned in the `diagnostics` array, not thrown as exceptions:

```typescript
const result = expandSync(invalidCode, "file.ts");

for (const diag of result.diagnostics) {
  if (diag.severity === "error") {
    console.error(\`Error at \${diag.span.start}: \${diag.message}\`);
  }
}
```

---

# transformSync()

*A lower-level transform function that returns additional metadata alongside the transformed code.*

## Signature

```typescript
function transformSync(
  code: string,
  filepath: string
): TransformResult
```

## Parameters

<table>
<thead>
<tr>
<th>Parameter</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>`code`</td>
<td>`string`</td>
<td>TypeScript source code to transform</td>
</tr>
<tr>
<td>`filepath`</td>
<td>`string`</td>
<td>File path (used for error reporting)</td>
</tr>
</tbody>
</table>

## TransformResult

```typescript
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
```

## Comparison with expandSync()

<table>
<thead>
<tr>
<th>Feature</th>
<th>`expandSync`</th>
<th>`transformSync`</th>
</tr>
</thead>
<tbody>
<tr>
<td>Options</td>
<td>Yes</td>
<td>No</td>
</tr>
<tr>
<td>Diagnostics</td>
<td>Yes</td>
<td>No</td>
</tr>
<tr>
<td>Source Mapping</td>
<td>Yes</td>
<td>Limited</td>
</tr>
<tr>
<td>Use Case</td>
<td>General purpose</td>
<td>Build tools</td>
</tr>
</tbody>
</table>

## Example

```typescript
import { transformSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
}
\`;

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
```

## When to Use

Use `transformSync` when:

- Building custom integrations

- You need raw output without diagnostics

- You're implementing a build tool plugin

Use `expandSync` for most other use cases, as it provides better error handling.

---

# NativePlugin

*A stateful plugin class with version-based caching, designed for integration with language servers and IDEs.*

## Constructor

```typescript
const plugin = new NativePlugin();
```

## Methods

### processFile()

Process a file with version-based caching:

```typescript
processFile(
  filepath: string,
  code: string,
  options?: ProcessFileOptions
): ExpandResult
```

```typescript
interface ProcessFileOptions {
  // Cache key - if unchanged, returns cached result
  version?: string;
}
```

### getMapper()

Get the position mapper for a previously processed file:

```typescript
getMapper(filepath: string): NativeMapper | null
```

### mapDiagnostics()

Map diagnostics from expanded positions to original positions:

```typescript
mapDiagnostics(
  filepath: string,
  diagnostics: JsDiagnostic[]
): JsDiagnostic[]
```

### log() / setLogFile()

Logging utilities for debugging:

```typescript
log(message: string): void
setLogFile(path: string): void
```

## Caching Behavior

The plugin caches expansion results by file path and version:

```typescript
const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });
```

## Example: Language Server Integration

```typescript
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
```

## Thread Safety

The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.

---

# PositionMapper

*Maps positions between original source code and macro-expanded code. Essential for accurate error reporting and debugging.*

## Getting a Mapper

```typescript
import { NativePlugin, PositionMapper } from "macroforge";

const plugin = new NativePlugin();
const result = plugin.processFile("user.ts", code, { version: "1" });

// Get the mapper for this file
const mapper = plugin.getMapper("user.ts");
if (mapper) {
  // Use the mapper...
}
```

## Methods

### isEmpty()

Check if the mapper has any mappings:

```typescript
isEmpty(): boolean
```

### originalToExpanded()

Map a position from original to expanded code:

```typescript
originalToExpanded(pos: number): number
```

### expandedToOriginal()

Map a position from expanded to original code:

```typescript
expandedToOriginal(pos: number): number | null
```

Returns `null` if the position is in generated code.

### isInGenerated()

Check if a position is in macro-generated code:

```typescript
isInGenerated(pos: number): boolean
```

### generatedBy()

Get the name of the macro that generated code at a position:

```typescript
generatedBy(pos: number): string | null
```

### mapSpanToOriginal()

Map a span (range) from expanded to original code:

```typescript
mapSpanToOriginal(start: number, length: number): SpanResult | null

interface SpanResult {
  start: number;
  length: number;
}
```

### mapSpanToExpanded()

Map a span from original to expanded code:

```typescript
mapSpanToExpanded(start: number, length: number): SpanResult
```

## Example: Error Position Mapping

```typescript
import { NativePlugin } from "macroforge";

const plugin = new NativePlugin();

function mapError(filepath: string, expandedPos: number, message: string) {
  const mapper = plugin.getMapper(filepath);
  if (!mapper) return null;

  // Check if the error is in generated code
  if (mapper.isInGenerated(expandedPos)) {
    const macroName = mapper.generatedBy(expandedPos);
    return {
      message: \`Error in code generated by @derive(\${macroName}): \${message}\`,
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
```

## Performance

Position mapping uses binary search with O(log n) complexity:

- Fast lookups even for large files

- Minimal memory overhead

- Thread-safe access

---
