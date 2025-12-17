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
 ```
npm&nbsp;install&nbsp;macroforge
``` ```
bun&nbsp;add&nbsp;macroforge
``` ```
pnpm&nbsp;add&nbsp;macroforge
```  **Info Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64). ## Basic Usage
 The simplest way to use Macroforge is with the built-in derive macros. Add a **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> comment decorator to your class:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;

//&nbsp;After&nbsp;macro&nbsp;expansion,&nbsp;User&nbsp;has:
//&nbsp;-&nbsp;toString():&nbsp;string&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(from&nbsp;Debug)
//&nbsp;-&nbsp;clone():&nbsp;User&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(from&nbsp;Clone)
//&nbsp;-&nbsp;equals(other:&nbsp;unknown):&nbsp;boolean&nbsp;(from&nbsp;PartialEq)
``` ## IDE Integration
 For the best development experience, add the TypeScript plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>:
 ```
&#123;
&nbsp;&nbsp;"compilerOptions":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"plugins":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"name":&nbsp;"@macroforge/typescript-plugin"
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;]
&nbsp;&nbsp;&#125;
&#125;
``` This enables features like:
 - Accurate error positions in your source code
 - Autocompletion for generated methods
 - Type checking for expanded code
 ## Build Integration (Vite)
 If you're using Vite, add the plugin to your config for automatic macro expansion during build:
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;generateTypes:&nbsp;true,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;typesOutputDir:&nbsp;".macroforge/types"
&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;]
&#125;);
``` ## Next Steps
 Now that you have Macroforge installed, learn how to use it:
 - [Create your first macro](../docs/getting-started/first-macro)
 - [Understand how macros work](../docs/concepts)
 - [Explore built-in macros](../docs/builtin-macros)

---

# Your First Macro
 *Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*
 ## Creating a Class with Derive Macros
 Start by creating a simple <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">User</code> class. We'll use the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator to automatically generate methods.
 **Before:**
```
/** @derive(Debug, Clone, PartialEq) */
export class User &#123;
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) &#123;
        this.name = name;
        this.age = age;
        this.email = email;
    &#125;
&#125;
``` 
**After:**
```
export class User &#123;
    name: string;
    age: number;
    email: string;

    constructor(name: string, age: number, email: string) &#123;
        this.name = name;
        this.age = age;
        this.email = email;
    &#125;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;

    static clone(value: User): User &#123;
        return userClone(value);
    &#125;

    static equals(a: User, b: User): boolean &#123;
        return userEquals(a, b);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('name: ' + value.name);
    parts.push('age: ' + value.age);
    parts.push('email: ' + value.email);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;

export function userClone(value: User): User &#123;
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.name = value.name;
    cloned.age = value.age;
    cloned.email = value.email;
    return cloned;
&#125;

export function userEquals(a: User, b: User): boolean &#123;
    if (a === b) return true;
    return a.name === b.name &#x26;&#x26; a.age === b.age &#x26;&#x26; a.email === b.email;
&#125;
``` ## Using the Generated Methods
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30,&nbsp;"alice@example.com");

//&nbsp;Debug:&nbsp;toString()
console.log(user.toString());
//&nbsp;Output:&nbsp;User&nbsp;&#123;&nbsp;name:&nbsp;Alice,&nbsp;age:&nbsp;30,&nbsp;email:&nbsp;alice@example.com&nbsp;&#125;

//&nbsp;Clone:&nbsp;clone()
const&nbsp;copy&nbsp;=&nbsp;user.clone();
console.log(copy.name);&nbsp;//&nbsp;"Alice"

//&nbsp;Eq:&nbsp;equals()
console.log(user.equals(copy));&nbsp;//&nbsp;true

const&nbsp;different&nbsp;=&nbsp;new&nbsp;User("Bob",&nbsp;25,&nbsp;"bob@example.com");
console.log(user.equals(different));&nbsp;//&nbsp;false
``` ## Customizing Behavior
 You can customize how macros work using field-level decorators. For example, with the Debug macro:
 **Before:**
```
/** @derive(Debug) */
export class User &#123;
    /** @debug(&#123; rename: "userId" &#125;) */
    id: number;

    name: string;

    /** @debug(&#123; skip: true &#125;) */
    password: string;

    constructor(id: number, name: string, password: string) &#123;
        this.id = id;
        this.name = name;
        this.password = password;
    &#125;
&#125;
``` 
**After:**
```
export class User &#123;
    id: number;

    name: string;

    password: string;

    constructor(id: number, name: string, password: string) &#123;
        this.id = id;
        this.name = name;
        this.password = password;
    &#125;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('userId: ' + value.id);
    parts.push('name: ' + value.name);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;
``` ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User(42,&nbsp;"Alice",&nbsp;"secret123");
console.log(user.toString());
//&nbsp;Output:&nbsp;User&nbsp;&#123;&nbsp;userId:&nbsp;42,&nbsp;name:&nbsp;Alice&nbsp;&#125;
//&nbsp;Note:&nbsp;'id'&nbsp;is&nbsp;renamed&nbsp;to&nbsp;'userId',&nbsp;'password'&nbsp;is&nbsp;skipped
```  **Field-level decorators Field-level decorators let you control exactly how each field is handled by the macro. ## Next Steps
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
 2. **Find**: Macroforge finds <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators and their associated items
 3. **Expand**: Each macro generates new code based on the class structure
 4. **Output**: The transformed TypeScript is written out, ready for normal compilation
 **Before:**
```
/** @derive(Debug) */
class User &#123;
    name: string;
&#125;
``` 
**After:**
```
class User &#123;
    name: string;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('name: ' + value.name);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;
``` ## Zero Runtime Overhead
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
 *The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code>.*
 ## Syntax Reference
 Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.
 ### The @derive Statement
 The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator triggers macro expansion on a class or interface:
 
**Source:**
```
/** @derive(Debug) */
class MyClass &#123;
    value: string;
&#125;
```  Syntax rules:
 - Must be inside a JSDoc comment (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#6A737D;--shiki-light:#6A737D">/** */</code>)
 - Must appear immediately before the class/interface declaration
 - Multiple macros can be comma-separated: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">A<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">, <span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">B<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">, <span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">C<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">)</code>
 - Multiple <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> statements can be stacked
 
**Source:**
```
/** @derive(Debug, Clone) */
class User &#123;
    name: string;
    email: string;
&#125;
```  ### The import macro Statement
 To use macros from external packages, you must declare them with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> macro</code>:
 ```
/**&nbsp;import&nbsp;macro&nbsp;&#123;&nbsp;MacroName&nbsp;&#125;&nbsp;from&nbsp;"package-name";&nbsp;*/
``` Syntax rules:
 - Must be inside a JSDoc comment (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#6A737D;--shiki-light:#6A737D">/** */</code>)
 - Can appear anywhere in the file (typically at the top)
 - Multiple macros can be imported: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> macro { A, B } <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">from<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "pkg"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 - Multiple import statements can be used for different packages
 ```
/**&nbsp;import&nbsp;macro&nbsp;&#123;&nbsp;JSON,&nbsp;Validate&nbsp;&#125;&nbsp;from&nbsp;"@my/macros";&nbsp;*/
/**&nbsp;import&nbsp;macro&nbsp;&#123;&nbsp;Builder&nbsp;&#125;&nbsp;from&nbsp;"@other/macros";&nbsp;*/

/**&nbsp;@derive(JSON,&nbsp;Validate,&nbsp;Builder)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;email:&nbsp;string;
&#125;
```  **Built-in macros Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement. ### Field Attributes
 Macros can define field-level attributes to customize behavior per field:
 ****Before:**
```
/** @derive(Debug, Serialize) */
class User &#123;
    /** @debug(&#123; rename: "userId" &#125;) */
    /** @serde(&#123; rename: "user_id" &#125;) */
    id: number;

    name: string;

    /** @debug(&#123; skip: true &#125;) */
    /** @serde(&#123; skip: true &#125;) */
    password: string;

    metadata: Record&#x3C;string, unknown>;
&#125;
``` 
**After:**
```
import &#123; SerializeContext &#125; from 'macroforge/serde';

class User &#123;
    id: number;

    name: string;

    password: string;

    metadata: Record&#x3C;string, unknown>;

    static toString(value: User): string &#123;
        return userToString(value);
    &#125;
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: User): string &#123;
        return userSerialize(value);
    &#125;
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: User, ctx: SerializeContext): Record&#x3C;string, unknown> &#123;
        return userSerializeWithContext(value, ctx);
    &#125;
&#125;

export function userToString(value: User): string &#123;
    const parts: string[] = [];
    parts.push('userId: ' + value.id);
    parts.push('name: ' + value.name);
    parts.push('metadata: ' + value.metadata);
    return 'User &#123; ' + parts.join(', ') + ' &#125;';
&#125;

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string &#123;
    const ctx = SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
&#125; /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: SerializeContext
): Record&#x3C;string, unknown> &#123;
    const existingId = ctx.getId(value);
    if (existingId !== undefined) &#123;
        return &#123; __ref: existingId &#125;;
    &#125;
    const __id = ctx.register(value);
    const result: Record&#x3C;string, unknown> = &#123; __type: 'User', __id &#125;;
    result['user_id'] = value.id;
    result['name'] = value.name;
    result['metadata'] = value.metadata;
    return result;
&#125;
``` Syntax rules:
 - Must be inside a JSDoc comment immediately before the field
 - Options use object literal syntax: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">({ key: value })</code>
 - Boolean options: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">({ skip: <span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">true<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> })</code>
 - String options: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">({ rename: <span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"newName"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> })</code>
 - Multiple attributes can be on separate lines or combined
 Common field attributes by macro:
 | Macro | Attribute | Options |
| --- | --- | --- |
| Debug | @debug | skip, rename |
| Clone | @clone | skip, clone_with |
| Serialize/Deserialize | @serde | skip, rename, flatten, default |
| Hash | @hash | skip |
| PartialEq/Ord | @eq, @ord | skip |
 ## How It Works
 1. **Declaration**: You write <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(MacroName)</code> before a class
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
 Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> macro</code> statement.
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
 Template-based code generation similar to Rust's <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">quote!</code>:
 - <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> - Generate TypeScript code from templates
 - <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">body!</code> - Generate class body members
 - Control flow: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"{#for}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"{#if}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"{$let}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 ### macroforge_ts_macros
 The procedural macro attribute for defining derive macros:
 - <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_macro_derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(Name)]</code> attribute
 - Automatic registration with the macro system
 - Error handling and span tracking
 ### NAPI-RS Bindings
 Bridges Rust and Node.js:
 - Exposes <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">expandSync</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">transformSync</code>, etc.
 - Provides the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code> class for caching
 - Handles data marshaling between Rust and JavaScript
 ## Data Flow
  TypeScript with @derive   receives JavaScript string   parses to AST   finds @derive decorators   extract data, run macro, generate AST nodes   generated nodes into AST   generates source code   to JavaScript with source mapping  ## Performance Characteristics
 - **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack
 - **Caching**: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code> caches results by file version
 - **Binary search**: Position mapping uses O(log n) lookups
 - **Zero-copy**: SWC's arena allocator minimizes allocations
 ## Re-exported Crates
 For custom macro development, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts</code> re-exports everything you need:
 ```
//&nbsp;Convenient&nbsp;re-exports&nbsp;for&nbsp;macro&nbsp;development
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body,&nbsp;ts_template,&nbsp;above,&nbsp;below,&nbsp;signature&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;Data,&nbsp;DeriveInput,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input&#125;;

//&nbsp;Also&nbsp;available:&nbsp;raw&nbsp;crate&nbsp;access&nbsp;and&nbsp;SWC&nbsp;modules
use&nbsp;macroforge_ts::swc_core;
use&nbsp;macroforge_ts::swc_common;
use&nbsp;macroforge_ts::swc_ecma_ast;
``` ## Next Steps
 - [Write custom macros](../../docs/custom-macros)
 - [Explore the API reference](../../docs/api)

---

# Built-in Macros

# Built-in Macros
 *Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*
 ## Overview
 | Macro | Generates | Description |
| --- | --- | --- |
| [Debug](../docs/builtin-macros/debug) | toString(): string | Human-readable string representation |
| [Clone](../docs/builtin-macros/clone) | clone(): T | Creates a deep copy of the object |
| [Default](../docs/builtin-macros/default) | static default(): T | Creates an instance with default values |
| [Hash](../docs/builtin-macros/hash) | hashCode(): number | Generates a hash code for the object |
| [PartialEq](../docs/builtin-macros/partial-eq) | equals(other: T): boolean | Value equality comparison |
| [Ord](../docs/builtin-macros/ord) | compare(other: T): number | Total ordering comparison (-1, 0, 1) |
| [PartialOrd](../docs/builtin-macros/partial-ord) | partialCompare(other: T): number | null | Partial ordering comparison |
| [Serialize](../docs/builtin-macros/serialize) | toJSON(): Record<string, unknown> | JSON serialization with type handling |
| [Deserialize](../docs/builtin-macros/deserialize) | static fromJSON(data: unknown): T | JSON deserialization with validation |
 ## Using Built-in Macros
 Built-in macros don't require imports. Just use them with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code>:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;
``` ## Interface Support
 All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">self</code> as the first parameter:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
interface&nbsp;Point&nbsp;&#123;
&nbsp;&nbsp;x:&nbsp;number;
&nbsp;&nbsp;y:&nbsp;number;
&#125;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Point&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(self:&nbsp;Point):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(self:&nbsp;Point):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(self:&nbsp;Point,&nbsp;other:&nbsp;Point):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(self:&nbsp;Point):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

const&nbsp;point:&nbsp;Point&nbsp;=&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;;

//&nbsp;Use&nbsp;the&nbsp;namespace&nbsp;functions
console.log(Point.toString(point));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Point&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;"
const&nbsp;copy&nbsp;=&nbsp;Point.clone(point);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;
console.log(Point.equals(point,&nbsp;copy));&nbsp;//&nbsp;true
``` ## Enum Support
 All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq,&nbsp;Serialize,&nbsp;Deserialize)&nbsp;*/
enum&nbsp;Status&nbsp;&#123;
&nbsp;&nbsp;Active&nbsp;=&nbsp;"active",
&nbsp;&nbsp;Inactive&nbsp;=&nbsp;"inactive",
&nbsp;&nbsp;Pending&nbsp;=&nbsp;"pending",
&#125;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Status&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(value:&nbsp;Status):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(value:&nbsp;Status):&nbsp;Status&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(a:&nbsp;Status,&nbsp;b:&nbsp;Status):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(value:&nbsp;Status):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toJSON(value:&nbsp;Status):&nbsp;string&nbsp;|&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;fromJSON(data:&nbsp;unknown):&nbsp;Status&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

//&nbsp;Use&nbsp;the&nbsp;namespace&nbsp;functions
console.log(Status.toString(Status.Active));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Status.Active"
console.log(Status.equals(Status.Active,&nbsp;Status.Active));&nbsp;//&nbsp;true
const&nbsp;json&nbsp;=&nbsp;Status.toJSON(Status.Pending);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"pending"
const&nbsp;parsed&nbsp;=&nbsp;Status.fromJSON("active");&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Status.Active
``` ## Type Alias Support
 All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq,&nbsp;Serialize,&nbsp;Deserialize)&nbsp;*/
type&nbsp;Point&nbsp;=&nbsp;&#123;
&nbsp;&nbsp;x:&nbsp;number;
&nbsp;&nbsp;y:&nbsp;number;
&#125;;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Point&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(value:&nbsp;Point):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(value:&nbsp;Point):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(a:&nbsp;Point,&nbsp;b:&nbsp;Point):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(value:&nbsp;Point):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toJSON(value:&nbsp;Point):&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;fromJSON(data:&nbsp;unknown):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

const&nbsp;point:&nbsp;Point&nbsp;=&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;;
console.log(Point.toString(point));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Point&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;"
const&nbsp;copy&nbsp;=&nbsp;Point.clone(point);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;
console.log(Point.equals(point,&nbsp;copy));&nbsp;//&nbsp;true
``` Union type aliases also work, using JSON-based implementations:
 ```
/**&nbsp;@derive(Debug,&nbsp;PartialEq)&nbsp;*/
type&nbsp;ApiStatus&nbsp;=&nbsp;"loading"&nbsp;|&nbsp;"success"&nbsp;|&nbsp;"error";

const&nbsp;status:&nbsp;ApiStatus&nbsp;=&nbsp;"success";
console.log(ApiStatus.toString(status));&nbsp;//&nbsp;"ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success",&nbsp;"success"));&nbsp;//&nbsp;true
``` ## Combining Macros
 All macros can be used together. They don't conflict and each generates independent methods:
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30);

//&nbsp;Debug
console.log(user.toString());
//&nbsp;"User&nbsp;&#123;&nbsp;name:&nbsp;Alice,&nbsp;age:&nbsp;30&nbsp;&#125;"

//&nbsp;Clone
const&nbsp;copy&nbsp;=&nbsp;user.clone();
console.log(copy.name);&nbsp;//&nbsp;"Alice"

//&nbsp;Eq
console.log(user.equals(copy));&nbsp;//&nbsp;true
``` ## Detailed Documentation
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

**Classes**: Generates a standalone function `classNameToString(value)` and a static wrapper
method `static toString(value)` returning a string like `"ClassName { field1: value1, field2: value2 }"`.

**Enums**: Generates a standalone function `enumNameToString(value)` that performs
reverse lookup on numeric enums.

**Interfaces**: Generates a standalone function `interfaceNameToString(value)`.

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
```

```typescript after
class User {
    userId: number;

    password: string;

    email: string;

    static toString(value: User): string {
        return userToString(value);
    }
}

export function userToString(value: User): string {
    const parts: string[] = [];
    parts.push('id: ' + value.userId);
    parts.push('email: ' + value.email);
    return 'User { ' + parts.join(', ') + ' }';
}
```

Generated output:

```typescript
class User {
    userId: number;

    password: string;

    email: string;

    static toString(value: User): string {
        return userToString(value);
    }
}

export function userToString(value: User): string {
    const parts: string[] = [];
    parts.push('id: ' + value.userId);
    parts.push('email: ' + value.email);
    return 'User { ' + parts.join(', ') + ' }';
}
```


---

# Clone

The `Clone` macro generates a `clone()` method for deep copying objects.
This is analogous to Rust's `Clone` trait, providing a way to create
independent copies of values.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameClone(value)` + `static clone(value)` | Standalone function + static wrapper method |
| Enum | `enumNameClone(value: EnumName): EnumName` | Standalone function (enums are primitives, returns value as-is) |
| Interface | `interfaceNameClone(value: InterfaceName): InterfaceName` | Standalone function creating a new object literal |
| Type Alias | `typeNameClone(value: TypeName): TypeName` | Standalone function with spread copy for objects |


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
```

```typescript after
class Point {
    x: number;
    y: number;

    static clone(value: Point): Point {
        return pointClone(value);
    }
}

export function pointClone(value: Point): Point {
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.x = value.x;
    cloned.y = value.y;
    return cloned;
}
```

Generated output:

```typescript
class Point {
    x: number;
    y: number;

    static clone(value: Point): Point {
        return pointClone(value);
    }
}

export function pointClone(value: Point): Point {
    const cloned = Object.create(Object.getPrototypeOf(value));
    cloned.x = value.x;
    cloned.y = value.y;
    return cloned;
}
```

## Implementation Notes

- **Classes**: Uses `Object.create(Object.getPrototypeOf(value))` to preserve
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

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `static defaultValue(): ClassName` | Static factory method |
| Enum | `defaultValueEnumName(): EnumName` | Standalone function returning marked variant |
| Interface | `defaultValueInterfaceName(): InterfaceName` | Standalone function returning object literal |
| Type Alias | `defaultValueTypeName(): TypeName` | Standalone function with type-appropriate default |


## Default Values by Type

The macro uses Rust-like default semantics:

| Type | Default Value |
|------|---------------|
| `string` | `""` (empty string) |
| `number` | `0` |
| `boolean` | `false` |
| `bigint` | `0n` |
| `T[]` | `[]` (empty array) |
| `Array<T>` | `[]` (empty array) |
| `Map<K,V>` | `new Map()` |
| `Set<T>` | `new Set()` |
| `Date` | `new Date()` (current time) |
| `T \| null` | `null` |
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
    /** @default("light") */
    theme: string;

    /** @default(10) */
    pageSize: number;

    notifications: boolean; // Uses type default: false
}
```

```typescript after
class UserSettings {
    theme: string;

    pageSize: number;

    notifications: boolean; // Uses type default: false

    static defaultValue(): UserSettings {
        const instance = new UserSettings();
        instance.theme = 'light';
        instance.pageSize = 10;
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

    notifications: boolean; // Uses type default: false

    static defaultValue(): UserSettings {
        const instance = new UserSettings();
        instance.theme = 'light';
        instance.pageSize = 10;
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
    /** @default */
    Pending,
    Active,
    Completed
}
```

```typescript after
enum Status {
    /** @default */
    Pending,
    Active,
    Completed
}

export function statusDefaultValue(): Status {
    return Status.Pending;
}

namespace Status {
    export const defaultValue = statusDefaultValue;
}
```

Generated output:

```typescript
enum Status {
    /** @default */
    Pending,
    Active,
    Completed
}

export function statusDefaultValue(): Status {
    return Status.Pending;
}

namespace Status {
    export const defaultValue = statusDefaultValue;
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

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameHashCode(value)` + `static hashCode(value)` | Standalone function + static wrapper method |
| Enum | `enumNameHashCode(value: EnumName): number` | Standalone function hashing by enum value |
| Interface | `interfaceNameHashCode(value: InterfaceName): number` | Standalone function computing hash |
| Type Alias | `typeNameHashCode(value: TypeName): number` | Standalone function computing hash |


## Hash Algorithm

Uses the standard polynomial rolling hash algorithm:

```text
hash = 17  // Initial seed
for each field:
    hash = (hash * 31 + fieldHash) | 0  // Bitwise OR keeps it 32-bit integer
```

This algorithm is consistent with Java's `Objects.hash()` implementation.

## Type-Specific Hashing

| Type | Hash Strategy |
|------|---------------|
| `number` | Integer: direct value; Float: string hash of decimal |
| `bigint` | String hash of decimal representation |
| `string` | Character-by-character polynomial hash |
| `boolean` | 1231 for true, 1237 for false (Java convention) |
| `Date` | `getTime()` timestamp |
| Arrays | Element-by-element hash combination |
| `Map` | Entry-by-entry key+value hash |
| `Set` | Element-by-element hash |
| Objects | Calls `hashCode()` if available, else JSON string hash |

## Field-Level Options

The `@hash` decorator supports:

- `skip` - Exclude the field from hash calculation

## Example

```typescript before
/** @derive(Hash, PartialEq) */
class User {
    id: number;
    name: string;

    /** @hash({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    cachedScore: number;

    static hashCode(value: User): number {
        return userHashCode(value);
    }

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name && a.cachedScore === b.cachedScore;
}
```

Generated output:

```typescript
class User {
    id: number;
    name: string;

    cachedScore: number;

    static hashCode(value: User): number {
        return userHashCode(value);
    }

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name && a.cachedScore === b.cachedScore;
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

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameCompare(a, b)` + `static compareTo(a, b)` | Standalone function + static wrapper method |
| Enum | `enumNameCompare(a: EnumName, b: EnumName): number` | Standalone function comparing enum values |
| Interface | `interfaceNameCompare(a: InterfaceName, b: InterfaceName): number` | Standalone function comparing fields |
| Type Alias | `typeNameCompare(a: TypeName, b: TypeName): number` | Standalone function with type-appropriate comparison |


## Return Values

Unlike `PartialOrd`, `Ord` provides **total ordering** - every pair of values
can be compared:

- **-1**: `a` is less than `b`
- **0**: `a` is equal to `b`
- **1**: `a` is greater than `b`

The function **never returns null** - all values must be comparable.

## Comparison Strategy

Fields are compared **lexicographically** in declaration order:

1. Compare first field
2. If not equal, return that result
3. Otherwise, compare next field
4. Continue until a difference is found or all fields are equal

## Type-Specific Comparisons

| Type | Comparison Method |
|------|-------------------|
| `number`/`bigint` | Direct `<` and `>` comparison |
| `string` | `localeCompare()` (clamped to -1, 0, 1) |
| `boolean` | false &lt; true |
| Arrays | Lexicographic element-by-element |
| `Date` | `getTime()` timestamp comparison |
| Objects | Calls `compareTo()` if available, else 0 |

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
```

```typescript after
class Version {
    major: number;
    minor: number;
    patch: number;

    static compareTo(a: Version, b: Version): number {
        return versionCompare(a, b);
    }
}

export function versionCompare(a: Version, b: Version): number {
    if (a === b) return 0;
    const cmp0 = a.major < b.major ? -1 : a.major > b.major ? 1 : 0;
    if (cmp0 !== 0) return cmp0;
    const cmp1 = a.minor < b.minor ? -1 : a.minor > b.minor ? 1 : 0;
    if (cmp1 !== 0) return cmp1;
    const cmp2 = a.patch < b.patch ? -1 : a.patch > b.patch ? 1 : 0;
    if (cmp2 !== 0) return cmp2;
    return 0;
}
```

Generated output:

```typescript
class Version {
    major: number;
    minor: number;
    patch: number;

    static compareTo(a: Version, b: Version): number {
        return versionCompare(a, b);
    }
}

export function versionCompare(a: Version, b: Version): number {
    if (a === b) return 0;
    const cmp0 = a.major < b.major ? -1 : a.major > b.major ? 1 : 0;
    if (cmp0 !== 0) return cmp0;
    const cmp1 = a.minor < b.minor ? -1 : a.minor > b.minor ? 1 : 0;
    if (cmp1 !== 0) return cmp1;
    const cmp2 = a.patch < b.patch ? -1 : a.patch > b.patch ? 1 : 0;
    if (cmp2 !== 0) return cmp2;
    return 0;
}
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

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameEquals(a, b)` + `static equals(a, b)` | Standalone function + static wrapper method |
| Enum | `enumNameEquals(a: EnumName, b: EnumName): boolean` | Standalone function using strict equality |
| Interface | `interfaceNameEquals(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields |
| Type Alias | `typeNameEquals(a: TypeName, b: TypeName): boolean` | Standalone function with type-appropriate comparison |

## Comparison Strategy

The generated equality check:

1. **Identity check**: `a === b` returns true immediately
2. **Field comparison**: Compares each non-skipped field

## Type-Specific Comparisons

| Type | Comparison Method |
|------|-------------------|
| Primitives | Strict equality (`===`) |
| Arrays | Length + element-by-element (recursive) |
| `Date` | `getTime()` comparison |
| `Map` | Size + entry-by-entry comparison |
| `Set` | Size + membership check |
| Objects | Calls `equals()` if available, else `===` |

## Field-Level Options

The `@partialEq` decorator supports:

- `skip` - Exclude the field from equality comparison

## Example

```typescript before
/** @derive(PartialEq, Hash) */
class User {
    id: number;
    name: string;

    /** @partialEq({ skip: true }) @hash({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    cachedScore: number;

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }

    static hashCode(value: User): number {
        return userHashCode(value);
    }
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name;
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}
```

Generated output:

```typescript
class User {
    id: number;
    name: string;

    cachedScore: number;

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }

    static hashCode(value: User): number {
        return userHashCode(value);
    }
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name;
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}
```

## Equality Contract

When implementing `PartialEq`, consider also implementing `Hash`:

- **Reflexivity**: `a.equals(a)` is always true
- **Symmetry**: `a.equals(b)` implies `b.equals(a)`
- **Hash consistency**: Equal objects must have equal hash codes

To maintain the hash contract, skip the same fields in both `PartialEq` and `Hash`:

```typescript before
/** @derive(PartialEq, Hash) */
class User {
    id: number;
    name: string;

    /** @partialEq({ skip: true }) @hash({ skip: true }) */
    cachedScore: number;
}
```

```typescript after
class User {
    id: number;
    name: string;

    cachedScore: number;

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }

    static hashCode(value: User): number {
        return userHashCode(value);
    }
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name;
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}
```

Generated output:

```typescript
class User {
    id: number;
    name: string;

    cachedScore: number;

    static equals(a: User, b: User): boolean {
        return userEquals(a, b);
    }

    static hashCode(value: User): number {
        return userHashCode(value);
    }
}

export function userEquals(a: User, b: User): boolean {
    if (a === b) return true;
    return a.id === b.id && a.name === b.name;
}

export function userHashCode(value: User): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.id)
                ? value.id | 0
                : value.id
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    return hash;
}
```


---

# PartialOrd

The `PartialOrd` macro generates a `compareTo()` method for **partial ordering**
comparison. This is analogous to Rust's `PartialOrd` trait, enabling comparison
between values where some pairs may be incomparable.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNamePartialCompare(a, b)` + `static compareTo(a, b)` | Standalone function + static wrapper method |
| Enum | `enumNamePartialCompare(a: EnumName, b: EnumName): Option<number>` | Standalone function returning Option |
| Interface | `interfaceNamePartialCompare(a: InterfaceName, b: InterfaceName): Option<number>` | Standalone function with Option |
| Type Alias | `typeNamePartialCompare(a: TypeName, b: TypeName): Option<number>` | Standalone function with Option |

## Return Values

Unlike `Ord`, `PartialOrd` returns an `Option<number>` to handle incomparable values:

- **Option.some(-1)**: `a` is less than `b`
- **Option.some(0)**: `a` is equal to `b`
- **Option.some(1)**: `a` is greater than `b`
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

| Type | Comparison Method |
|------|-------------------|
| `number`/`bigint` | Direct comparison, returns some() |
| `string` | `localeCompare()` wrapped in some() |
| `boolean` | false &lt; true, wrapped in some() |
| null/undefined | Returns none() for mismatched nullability |
| Arrays | Lexicographic, propagates none() on incomparable elements |
| `Date` | Timestamp comparison, none() if invalid |
| Objects | Unwraps nested Option from compareTo() |

## Field-Level Options

The `@ord` decorator supports:

- `skip` - Exclude the field from ordering comparison

## Example

```typescript before
/** @derive(PartialOrd) */
class Temperature {
    value: number | null;
    unit: string;
}
```

```typescript after
class Temperature {
    value: number | null;
    unit: string;

    static compareTo(a: Temperature, b: Temperature): number | null {
        return temperaturePartialCompare(a, b);
    }
}

export function temperaturePartialCompare(a: Temperature, b: Temperature): number | null {
    if (a === b) return 0;
    const cmp0 = (() => {
        if (typeof (a.value as any)?.compareTo === 'function') {
            const optResult = (a.value as any).compareTo(b.value);
            return optResult === null ? null : optResult;
        }
        return a.value === b.value ? 0 : null;
    })();
    if (cmp0 === null) return null;
    if (cmp0 !== 0) return cmp0;
    const cmp1 = a.unit.localeCompare(b.unit);
    if (cmp1 === null) return null;
    if (cmp1 !== 0) return cmp1;
    return 0;
}
```

Generated output:

```typescript
class Temperature {
    value: number | null;
    unit: string;

    static compareTo(a: Temperature, b: Temperature): number | null {
        return temperaturePartialCompare(a, b);
    }
}

export function temperaturePartialCompare(a: Temperature, b: Temperature): number | null {
    if (a === b) return 0;
    const cmp0 = (() => {
        if (typeof (a.value as any)?.compareTo === 'function') {
            const optResult = (a.value as any).compareTo(b.value);
            return optResult === null ? null : optResult;
        }
        return a.value === b.value ? 0 : null;
    })();
    if (cmp0 === null) return null;
    if (cmp0 !== 0) return cmp0;
    const cmp1 = a.unit.localeCompare(b.unit);
    if (cmp1 === null) return null;
    if (cmp1 !== 0) return cmp1;
    return 0;
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

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameSerialize(value)` + `static serialize(value)` | Standalone function + static wrapper method |
| Enum | `enumNameSerialize(value)`, `enumNameSerializeWithContext` | Standalone functions |
| Interface | `interfaceNameSerialize(value)`, etc. | Standalone functions |
| Type Alias | `typeNameSerialize(value)`, etc. | Standalone functions |

## Cycle Detection Protocol

The generated code handles circular references using `__id` and `__ref` markers:

```json
{
    "__type": "User",
    "__id": 1,
    "name": "Alice",
    "friend": { "__ref": 2 }  // Reference to object with __id: 2
}
```

When an object is serialized:
1. Check if it's already been serialized (has an `__id`)
2. If so, return `{ "__ref": existingId }` instead
3. Otherwise, register the object and serialize its fields

## Type-Specific Serialization

| Type | Serialization Strategy |
|------|------------------------|
| Primitives | Direct value |
| `Date` | `toISOString()` |
| Arrays | For primitive-like element types, pass through; for `Date`/`Date | null`, map to ISO strings; otherwise map and call `SerializeWithContext(ctx)` when available |
| `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date | null`, convert to ISO strings; otherwise call `SerializeWithContext(ctx)` per value when available |
| `Set<T>` | Convert to array; element handling matches `Array<T>` |
| Nullable | Include `null` explicitly; for primitive-like and `Date` unions the generator avoids runtime `SerializeWithContext` checks |
| Objects | Call `SerializeWithContext(ctx)` if available (to support user-defined implementations) |

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
```

```typescript after
import { SerializeContext } from 'macroforge/serde';

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: User): string {
        return userSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: User, ctx: SerializeContext): Record<string, unknown> {
        return userSerializeWithContext(value, ctx);
    }
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['userName'] = value.name;
    {
        const __flattened = userMetadataSerializeWithContext(value.metadata, ctx);
        const { __type: _, __id: __, ...rest } = __flattened as any;
        Object.assign(result, rest);
    }
    return result;
}
```

Generated output:

```typescript
import { SerializeContext } from 'macroforge/serde';

class User {
    id: number;

    name: string;

    password: string;

    metadata: UserMetadata;
    /** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata  */

    static serialize(value: User): string {
        return userSerialize(value);
    }
    /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context  */

    static serializeWithContext(value: User, ctx: SerializeContext): Record<string, unknown> {
        return userSerializeWithContext(value, ctx);
    }
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
} /** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['userName'] = value.name;
    {
        const __flattened = userMetadataSerializeWithContext(value.metadata, ctx);
        const { __type: _, __id: __, ...rest } = __flattened as any;
        Object.assign(result, rest);
    }
    return result;
}
```

## Required Import

The generated code automatically imports `SerializeContext` from `macroforge/serde`.


---

# Deserialize

The `Deserialize` macro generates JSON deserialization methods with **cycle and
forward-reference support**, plus comprehensive runtime validation. This enables
safe parsing of complex JSON structures including circular references.

## Generated Output

| Type | Generated Code | Description |
|------|----------------|-------------|
| Class | `classNameDeserialize(input)` + `static deserialize(input)` | Standalone function + static factory method |
| Enum | `enumNameDeserialize(input)`, `enumNameDeserializeWithContext(data)`, `enumNameIs(value)` | Standalone functions |
| Interface | `interfaceNameDeserialize(input)`, etc. | Standalone functions |
| Type Alias | `typeNameDeserialize(input)`, etc. | Standalone functions |

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
`__type` field in the JSON to dispatch to the correct type's `deserializeWithContext` method.

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
/** @derive(Deserialize) @serde({ denyUnknownFields: true }) */
class User {
    id: number;

    /** @serde({ validate: { email: true, maxLength: 255 } }) */
    email: string;

    /** @serde({ default: "guest" }) */
    name: string;

    /** @serde({ validate: { positive: true } }) */
    age?: number;
}
```

```typescript after
class User {
    id: number;

    email: string;

    name: string;

    age?: number;
}
```

Generated output:

```typescript
class User {
    id: number;

    email: string;

    name: string;

    age?: number;
}
```

Generated output:

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = "guest";
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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

```typescript
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';

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
        opts?: DeserializeOptions
    ): Result<
        User,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            // Auto-detect: if string, parse as JSON first
            const data = typeof input === 'string' ? JSON.parse(input) : input;

            const ctx = DeserializeContext.create();
            const resultOrRef = User.deserializeWithContext(data, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'User.deserialize: root cannot be a forward reference'
                    }
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
                    field: '_root',
                    message
                }
            ]);
        }
    }

    /** @internal */
    static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'User.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
        for (const key of Object.keys(obj)) {
            if (!knownKeys.has(key)) {
                errors.push({
                    field: key,
                    message: 'unknown field'
                });
            }
        }
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
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
            const __raw_id = obj['id'] as number;
            instance.id = __raw_id;
        }
        {
            const __raw_email = obj['email'] as string;
            instance.email = __raw_email;
        }
        if ('name' in obj && obj['name'] !== undefined) {
            const __raw_name = obj['name'] as string;
            instance.name = __raw_name;
        } else {
            instance.name = 'guest';
        }
        if ('age' in obj && obj['age'] !== undefined) {
            const __raw_age = obj['age'] as number;
            instance.age = __raw_age;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof User>(
        field: K,
        value: User[K]
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
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'id' in o && 'email' in o;
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
 *Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.*
 ## Overview
 Custom macros are written in Rust and compiled to native Node.js addons. The process involves:
 1. Creating a Rust crate with NAPI bindings
 2. Defining macro functions with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[ts_macro_derive]</code>
 3. Using <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts_quote</code> to generate TypeScript code
 4. Building and publishing as an npm package
 ## Quick Example
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;Data,&nbsp;DeriveInput,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input&#125;;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;toJSON():&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;class.field_names()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(JSON)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Using Custom Macros
 Once your macro package is published, users can import and use it:
 ```
/**&nbsp;import&nbsp;macro&nbsp;&#123;&nbsp;JSON&nbsp;&#125;&nbsp;from&nbsp;"@my/macros";&nbsp;*/

/**&nbsp;@derive(JSON)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;

const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30);
console.log(user.toJSON());&nbsp;//&nbsp;&#123;&nbsp;name:&nbsp;"Alice",&nbsp;age:&nbsp;30&nbsp;&#125;
``` > **Note:** The import macro comment tells Macroforge which package provides the macro. ## Getting Started
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
 - NAPI-RS CLI: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">cargo<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> install<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> macroforge_ts</code>
 ## Create the Project
 ```
#&nbsp;Create&nbsp;a&nbsp;new&nbsp;directory
mkdir&nbsp;my-macros
cd&nbsp;my-macros

#&nbsp;Initialize&nbsp;with&nbsp;NAPI-RS
napi&nbsp;new&nbsp;--platform&nbsp;--name&nbsp;my-macros
``` ## Configure Cargo.toml
 Update your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#FDAEB7;--shiki-dark-font-style:italic;--shiki-light:#B31D28;--shiki-light-font-style:italic">Cargo.toml</code> with the required dependencies:
 ```
[package]
name&nbsp;=&nbsp;"my-macros"
version&nbsp;=&nbsp;"0.1.0"
edition&nbsp;=&nbsp;"2024"

[lib]
crate-type&nbsp;=&nbsp;["cdylib"]

[dependencies]
macroforge_ts&nbsp;=&nbsp;"0.1"
napi&nbsp;=&nbsp;&#123;&nbsp;version&nbsp;=&nbsp;"3",&nbsp;features&nbsp;=&nbsp;["napi8",&nbsp;"compat-mode"]&nbsp;&#125;
napi-derive&nbsp;=&nbsp;"3"

[build-dependencies]
napi-build&nbsp;=&nbsp;"2"

[profile.release]
lto&nbsp;=&nbsp;true
strip&nbsp;=&nbsp;true
``` ## Create build.rs
 ```
fn&nbsp;main()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;napi_build::setup();
&#125;
``` ## Create src/lib.rs
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Data,&nbsp;DeriveInput,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input,
&#125;;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;toJSON():&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;class.field_names()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(JSON)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Create package.json
 ```
&#123;
&nbsp;&nbsp;"name":&nbsp;"@my-org/macros",
&nbsp;&nbsp;"version":&nbsp;"0.1.0",
&nbsp;&nbsp;"main":&nbsp;"index.js",
&nbsp;&nbsp;"types":&nbsp;"index.d.ts",
&nbsp;&nbsp;"napi":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"name":&nbsp;"my-macros",
&nbsp;&nbsp;&nbsp;&nbsp;"triples":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"defaults":&nbsp;true
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;,
&nbsp;&nbsp;"files":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;"index.js",
&nbsp;&nbsp;&nbsp;&nbsp;"index.d.ts",
&nbsp;&nbsp;&nbsp;&nbsp;"*.node"
&nbsp;&nbsp;],
&nbsp;&nbsp;"scripts":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"build":&nbsp;"napi&nbsp;build&nbsp;--release",
&nbsp;&nbsp;&nbsp;&nbsp;"prepublishOnly":&nbsp;"napi&nbsp;build&nbsp;--release"
&nbsp;&nbsp;&#125;,
&nbsp;&nbsp;"devDependencies":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"@napi-rs/cli":&nbsp;"^3.0.0-alpha.0"
&nbsp;&nbsp;&#125;
&#125;
``` ## Build the Package
 ```
#&nbsp;Build&nbsp;the&nbsp;native&nbsp;addon
npm&nbsp;run&nbsp;build

#&nbsp;This&nbsp;creates:
#&nbsp;-&nbsp;index.js&nbsp;(JavaScript&nbsp;bindings)
#&nbsp;-&nbsp;index.d.ts&nbsp;(TypeScript&nbsp;types)
#&nbsp;-&nbsp;*.node&nbsp;(native&nbsp;binary)
```  **Tip For cross-platform builds, use GitHub Actions with the NAPI-RS CI template. ## Next Steps
 - [Learn the #[ts_macro_derive] attribute](../../docs/custom-macros/ts-macro-derive)
 - [Master the template syntax](../../docs/custom-macros/ts-quote)
**

---

# ts_macro_derive
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[ts_macro_derive]</code> attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*
 ## Basic Syntax
 ```
use&nbsp;macroforge_ts::macros::ts_macro_derive;
use&nbsp;macroforge_ts::ts_syn::&#123;TsStream,&nbsp;MacroforgeError&#125;;

#[ts_macro_derive(MacroName)]
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Macro&nbsp;implementation
&#125;
``` ## Attribute Options
 ### Name (Required)
 The first argument is the macro name that users will reference in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>:
 ```
#[ts_macro_derive(JSON)]&nbsp;&nbsp;//&nbsp;Users&nbsp;write:&nbsp;@derive(JSON)
pub&nbsp;fn&nbsp;derive_json(...)
``` ### Description
 Provides documentation for the macro:
 ```
#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(...)
``` ### Attributes
 Declare which field-level decorators your macro accepts:
 ```
#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;Debug,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toString()",
&nbsp;&nbsp;&nbsp;&nbsp;attributes(debug)&nbsp;&nbsp;//&nbsp;Allows&nbsp;@debug(&#123;&nbsp;...&nbsp;&#125;)&nbsp;on&nbsp;fields
)]
pub&nbsp;fn&nbsp;derive_debug(...)
``` > **Note:** Declared attributes become available as @attributeName({ options }) decorators in TypeScript. ## Function Signature
 ```
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>
``` | Parameter | Description |
| --- | --- |
| input: TsStream | Token stream containing the class/interface AST |
| Result<TsStream, MacroforgeError> | Returns generated code or an error with source location |
 ## Parsing Input
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">parse_ts_macro_input!</code> to convert the token stream:
 ```
use&nbsp;macroforge_ts::ts_syn::&#123;Data,&nbsp;DeriveInput,&nbsp;parse_ts_macro_input&#125;;

#[ts_macro_derive(MyMacro)]
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Access&nbsp;class&nbsp;data
&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;fields&nbsp;=&nbsp;class.fields();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Interface(interface)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Handle&nbsp;interfaces
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Enum(_)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Handle&nbsp;enums&nbsp;(if&nbsp;supported)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## DeriveInput Structure
 ```
struct&nbsp;DeriveInput&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ident:&nbsp;Ident,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;The&nbsp;type&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Span&nbsp;of&nbsp;the&nbsp;type&nbsp;definition
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;attrs:&nbsp;Vec&#x3C;Attribute>,&nbsp;&nbsp;//&nbsp;Decorators&nbsp;(excluding&nbsp;@derive)
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;data:&nbsp;Data,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;The&nbsp;parsed&nbsp;type&nbsp;data
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;context:&nbsp;MacroContextIR,&nbsp;//&nbsp;Macro&nbsp;context&nbsp;with&nbsp;spans

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Helper&nbsp;methods
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;name(&#x26;self)&nbsp;->&nbsp;&#x26;str;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Get&nbsp;the&nbsp;type&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;decorator_span(&#x26;self)&nbsp;->&nbsp;SpanIR;&nbsp;&nbsp;//&nbsp;Span&nbsp;of&nbsp;@derive&nbsp;decorator
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_class(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataClass>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_interface(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataInterface>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_enum(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataEnum>;
&#125;

enum&nbsp;Data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Class(DataClass),
&nbsp;&nbsp;&nbsp;&nbsp;Interface(DataInterface),
&nbsp;&nbsp;&nbsp;&nbsp;Enum(DataEnum),
&nbsp;&nbsp;&nbsp;&nbsp;TypeAlias(DataTypeAlias),
&#125;

impl&nbsp;DataClass&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;fields(&#x26;self)&nbsp;->&nbsp;&#x26;[FieldIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;methods(&#x26;self)&nbsp;->&nbsp;&#x26;[MethodSigIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;FieldIR>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body_span(&#x26;self)&nbsp;->&nbsp;SpanIR;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;For&nbsp;inserting&nbsp;code&nbsp;into&nbsp;class&nbsp;body
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;//&nbsp;Generic&nbsp;type&nbsp;parameters
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;heritage(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;extends/implements&nbsp;clauses
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_abstract(&#x26;self)&nbsp;->&nbsp;bool;
&#125;

impl&nbsp;DataInterface&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;fields(&#x26;self)&nbsp;->&nbsp;&#x26;[InterfaceFieldIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;methods(&#x26;self)&nbsp;->&nbsp;&#x26;[InterfaceMethodIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;InterfaceFieldIR>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body_span(&#x26;self)&nbsp;->&nbsp;SpanIR;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;heritage(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;extends&nbsp;clauses
&#125;

impl&nbsp;DataEnum&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variants(&#x26;self)&nbsp;->&nbsp;&#x26;[EnumVariantIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variant_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variant(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;EnumVariantIR>;
&#125;

impl&nbsp;DataTypeAlias&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body(&#x26;self)&nbsp;->&nbsp;&#x26;TypeBody;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_union(&#x26;self)&nbsp;->&nbsp;bool;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_object(&#x26;self)&nbsp;->&nbsp;bool;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_union(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;[TypeMember]>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_object(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;[InterfaceFieldIR]>;
&#125;
``` ## Accessing Field Data
 ### Class Fields (FieldIR)
 ```
struct&nbsp;FieldIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Field&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Field&nbsp;span
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ts_type:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;TypeScript&nbsp;type&nbsp;annotation
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;optional:&nbsp;bool,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Whether&nbsp;field&nbsp;has&nbsp;?
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;readonly:&nbsp;bool,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Whether&nbsp;field&nbsp;is&nbsp;readonly
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;visibility:&nbsp;Visibility,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Public,&nbsp;Protected,&nbsp;Private
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,&nbsp;//&nbsp;Field&nbsp;decorators
&#125;
``` ### Interface Fields (InterfaceFieldIR)
 ```
struct&nbsp;InterfaceFieldIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ts_type:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;optional:&nbsp;bool,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;readonly:&nbsp;bool,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Note:&nbsp;No&nbsp;visibility&nbsp;field&nbsp;(interfaces&nbsp;are&nbsp;always&nbsp;public)
&#125;
``` ### Enum Variants (EnumVariantIR)
 ```
struct&nbsp;EnumVariantIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;value:&nbsp;EnumValue,&nbsp;&nbsp;//&nbsp;Auto,&nbsp;String(String),&nbsp;or&nbsp;Number(f64)
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,
&#125;
``` ### Decorator Structure
 ```
struct&nbsp;DecoratorIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;e.g.,&nbsp;"serde"
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;args_src:&nbsp;String,&nbsp;&nbsp;//&nbsp;Raw&nbsp;args&nbsp;text,&nbsp;e.g.,&nbsp;"skip,&nbsp;rename:&nbsp;'id'"
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&#125;
``` > **Note:** To check for decorators, iterate through field.decorators and check decorator.name. For parsing options, you can write helper functions like the built-in macros do. ## Adding Imports
 If your macro generates code that requires imports, use the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">add_import</code> method on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">TsStream</code>:
 ```
//&nbsp;Add&nbsp;an&nbsp;import&nbsp;to&nbsp;be&nbsp;inserted&nbsp;at&nbsp;the&nbsp;top&nbsp;of&nbsp;the&nbsp;file
let&nbsp;mut&nbsp;output&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;ValidationResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;validateFields(this);
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;

//&nbsp;This&nbsp;will&nbsp;add:&nbsp;import&nbsp;&#123;&nbsp;validateFields,&nbsp;ValidationResult&nbsp;&#125;&nbsp;from&nbsp;"my-validation-lib";
output.add_import("validateFields",&nbsp;"my-validation-lib");
output.add_import("ValidationResult",&nbsp;"my-validation-lib");

Ok(output)
``` > **Note:** Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again. ## Returning Errors
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroforgeError</code> to report errors with source locations:
 ```
#[ts_macro_derive(ClassOnly)]
pub&nbsp;fn&nbsp;class_only(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(_)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Generate&nbsp;code...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;&nbsp;/*&nbsp;...&nbsp;*/&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(ClassOnly)&nbsp;can&nbsp;only&nbsp;be&nbsp;used&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Complete Example
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Data,&nbsp;DeriveInput,&nbsp;FieldIR,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input,
&#125;;

//&nbsp;Helper&nbsp;function&nbsp;to&nbsp;check&nbsp;if&nbsp;a&nbsp;field&nbsp;has&nbsp;a&nbsp;decorator
fn&nbsp;has_decorator(field:&nbsp;&#x26;FieldIR,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;bool&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;field.decorators.iter().any(|d|&nbsp;d.name.eq_ignore_ascii_case(name))
&#125;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;Validate,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;a&nbsp;validate()&nbsp;method",
&nbsp;&nbsp;&nbsp;&nbsp;attributes(validate)
)]
pub&nbsp;fn&nbsp;derive_validate(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;validations:&nbsp;Vec&#x3C;_>&nbsp;=&nbsp;class.fields()
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.iter()
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.filter(|f|&nbsp;has_decorator(f,&nbsp;"validate"))
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.collect();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;string[]&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;errors:&nbsp;string[]&nbsp;=&nbsp;[];
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;validations&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;if&nbsp;(!this.@&#123;field.name&#125;)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;errors.push("@&#123;field.name&#125;&nbsp;is&nbsp;required");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;errors;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(Validate)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Next Steps
 - [Learn the template syntax](../../docs/custom-macros/ts-quote)

---

# Template Syntax
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts_quote</code> crate provides template-based code generation for TypeScript. The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> macro uses Svelte + Rust-inspired syntax for control flow and interpolation, making it easy to generate complex TypeScript code.*
 ## Available Macros
 | Macro | Output | Use Case |
| --- | --- | --- |
| ts_template! | Any TypeScript code | General code generation |
| body! | Class body members | Methods and properties |
 ## Quick Reference
 | Syntax | Description |
| --- | --- |
| @{expr} | Interpolate a Rust expression (adds space after) |
| {| content |} | Ident block: concatenates without spaces (e.g., `{|get@{name}|}` → getUser) |
| {> "comment" <} | Block comment: outputs /* comment */ (string preserves whitespace) |
| {>> "doc" <<} | Doc comment: outputs /** doc */ (string preserves whitespace) |
| @@{ | Escape for literal @{ (e.g., "@@{foo}" → @{foo}) |
| "text @{expr}" | String interpolation (auto-detected) |
| "'^template ${js}^'" | JS backtick template literal (outputs ``template ${js}``) |
| {#if cond}...{/if} | Conditional block |
| `{#if cond}...{:else}...{/if}` | Conditional with else |
| {#if a}...{:else if b}...{:else}...{/if} | Full if/else-if/else chain |
| `{#if let pattern = expr}...{/if}` | Pattern matching if-let |
| {#match expr}{:case pattern}...{/match} | Match expression with case arms |
| `{#for item in list}...{/for}` | Iterate over a collection |
| {#while cond}...{/while} | While loop |
| `{#while let pattern = expr}...{/while}` | While-let pattern matching loop |
| {$let name = expr} | Define a local constant |
| {$let mut name = expr} | Define a mutable local variable |
| {$do expr} | Execute a side-effectful expression |
| {$typescript stream} | Inject a TsStream, preserving its source and runtime_patches (imports) |
 **Note:** A single <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@</code> not followed by <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{</code> passes through unchanged (e.g., <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">email@domain.com</code> works as expected).
 ## Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{expr}</code>
 Insert Rust expressions into the generated TypeScript:
 ```
let&nbsp;class_name&nbsp;=&nbsp;"User";
let&nbsp;method&nbsp;=&nbsp;"toString";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.@&#123;method&#125;&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"User&nbsp;instance";
&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&#125;;
``` **Generates:**
 ```
User.prototype.toString&nbsp;=&nbsp;function&nbsp;()&nbsp;&#123;
&nbsp;&nbsp;return&nbsp;"User&nbsp;instance";
&#125;;
``` ## Identifier Concatenation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> content <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 When you need to build identifiers dynamically (like <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">getUser</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">setName</code>), use the ident block syntax. Everything inside <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> |<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> is concatenated without spaces:
 ```
let&nbsp;field_name&nbsp;=&nbsp;"User";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;&#123;|get@&#123;field_name&#125;|&#125;()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.@&#123;field_name.to_lowercase()&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
function&nbsp;getUser()&nbsp;&#123;
&nbsp;&nbsp;return&nbsp;this.user;
&#125;
``` Without ident blocks, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{}</code> always adds a space after for readability. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">|<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> |<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> when you explicitly want concatenation:
 ```
let&nbsp;name&nbsp;=&nbsp;"Status";

//&nbsp;With&nbsp;space&nbsp;(default&nbsp;behavior)
ts_template!&nbsp;&#123;&nbsp;namespace&nbsp;@&#123;name&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"namespace&nbsp;Status"

//&nbsp;Without&nbsp;space&nbsp;(ident&nbsp;block)
ts_template!&nbsp;&#123;&nbsp;&#123;|namespace@&#123;name&#125;|&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"namespaceStatus"
``` Multiple interpolations can be combined:
 ```
let&nbsp;entity&nbsp;=&nbsp;"user";
let&nbsp;action&nbsp;=&nbsp;"create";

ts_template!&nbsp;&#123;&nbsp;&#123;|@&#123;entity&#125;_@&#123;action&#125;|&#125;&nbsp;&#125;&nbsp;&nbsp;//&nbsp;→&nbsp;"user_create"
``` ## Comments: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> and <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">>><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "..."<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:
 ### Block Comments
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "comment"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for block comments:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>&nbsp;"This&nbsp;is&nbsp;a&nbsp;block&nbsp;comment"&nbsp;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;x&nbsp;=&nbsp;42;
&#125;;
``` **Generates:**
 ```
/*&nbsp;This&nbsp;is&nbsp;a&nbsp;block&nbsp;comment&nbsp;*/
const&nbsp;x&nbsp;=&nbsp;42;
``` ### Doc Comments (JSDoc)
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">>><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> "doc"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> <<<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for JSDoc comments:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;"@param&nbsp;&#123;string&#125;&nbsp;name&nbsp;-&nbsp;The&nbsp;user's&nbsp;name"&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;"@returns&nbsp;&#123;string&#125;&nbsp;A&nbsp;greeting&nbsp;message"&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;greet(name:&nbsp;string):&nbsp;string&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"Hello,&nbsp;"&nbsp;+&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
/**&nbsp;@param&nbsp;&#123;string&#125;&nbsp;name&nbsp;-&nbsp;The&nbsp;user's&nbsp;name&nbsp;*/
/**&nbsp;@returns&nbsp;&#123;string&#125;&nbsp;A&nbsp;greeting&nbsp;message&nbsp;*/
function&nbsp;greet(name:&nbsp;string):&nbsp;string&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;"Hello,&nbsp;"&nbsp;+&nbsp;name;
&#125;
``` ### Comments with Interpolation
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">format<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">!<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code> or similar to build dynamic comment strings:
 ```
let&nbsp;param_name&nbsp;=&nbsp;"userId";
let&nbsp;param_type&nbsp;=&nbsp;"number";
let&nbsp;comment&nbsp;=&nbsp;format!("@param&nbsp;&#123;&#123;&#123;&#125;&#125;&#125;&nbsp;&#123;&#125;&nbsp;-&nbsp;The&nbsp;user&nbsp;ID",&nbsp;param_type,&nbsp;param_name);

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;>>&nbsp;@&#123;comment&#125;&nbsp;&#x3C;&#x3C;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;getUser(userId:&nbsp;number)&nbsp;&#123;&#125;
&#125;;
``` **Generates:**
 ```
/**&nbsp;@param&nbsp;&#123;number&#125;&nbsp;userId&nbsp;-&nbsp;The&nbsp;user&nbsp;ID&nbsp;*/
function&nbsp;getUser(userId:&nbsp;number)&nbsp;&#123;&#125;
``` ## String Interpolation: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"text @{expr}"</code>
 Interpolation works automatically inside string literals - no `format!()` needed:
 ```
let&nbsp;name&nbsp;=&nbsp;"World";
let&nbsp;count&nbsp;=&nbsp;42;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello&nbsp;@&#123;name&#125;!");
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Count:&nbsp;@&#123;count&#125;,&nbsp;doubled:&nbsp;@&#123;count&nbsp;*&nbsp;2&#125;");
&#125;;
``` **Generates:**
 ```
console.log("Hello&nbsp;World!");
console.log("Count:&nbsp;42,&nbsp;doubled:&nbsp;84");
``` This also works with method calls and complex expressions:
 ```
let&nbsp;field&nbsp;=&nbsp;"username";

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("Invalid&nbsp;@&#123;field.to_uppercase()&#125;");
&#125;;
``` ## Backtick Template Literals: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"'^...^'"</code>
 For JavaScript template literals (backtick strings), use the `'^...^'` syntax. This outputs actual backticks and passes through <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">${<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"${}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> for JS interpolation:
 ```
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>${content}</@{tag_name}>^'";
};
``` **Generates:**
 ```
const html = `${content}`;
``` You can mix Rust <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{}</code> interpolation (evaluated at macro expansion time) with JS <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">${<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"${}"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code> interpolation (evaluated at runtime):
 ```
let class_name = "User";

let code = ts_template! {
    "'^Hello ${this.name}, you are a @{class_name}^'"
};
``` **Generates:**
 ```
`Hello&nbsp;$&#123;this.name&#125;,&nbsp;you&nbsp;are&nbsp;a&nbsp;User`
``` ## Conditionals: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">...<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/if<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Basic conditional:
 ```
let&nbsp;needs_validation&nbsp;=&nbsp;true;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;save()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;needs_validation&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;if&nbsp;(!this.isValid())&nbsp;return&nbsp;false;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.doSave();
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` ### If-Else
 ```
let&nbsp;has_default&nbsp;=&nbsp;true;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;has_default&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;defaultValue;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("No&nbsp;default");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ### If-Else-If Chains
 ```
let&nbsp;level&nbsp;=&nbsp;2;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;level&nbsp;==&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Level&nbsp;1");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&nbsp;if&nbsp;level&nbsp;==&nbsp;2&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Level&nbsp;2");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Other&nbsp;level");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ## Pattern Matching: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code> for pattern matching on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Option</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result</code>, or other Rust enums:
 ```
let&nbsp;maybe_name:&nbsp;Option&#x3C;&#x26;str>&nbsp;=&nbsp;Some("Alice");

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(name)&nbsp;=&nbsp;maybe_name&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello,&nbsp;@&#123;name&#125;!");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Hello,&nbsp;anonymous!");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` **Generates:**
 ```
console.log("Hello,&nbsp;Alice!");
``` This is useful when working with optional values from your IR:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(default_val)&nbsp;=&nbsp;field.default_value&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.@&#123;field.name&#125;&nbsp;=&nbsp;@&#123;default_val&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;this.@&#123;field.name&#125;&nbsp;=&nbsp;undefined;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;;
``` ## Match Expressions: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#match}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">match</code> for exhaustive pattern matching:
 ```
enum&nbsp;Visibility&nbsp;&#123;&nbsp;Public,&nbsp;Private,&nbsp;Protected&nbsp;&#125;
let&nbsp;visibility&nbsp;=&nbsp;Visibility::Public;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#match&nbsp;visibility&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Public&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;public
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Private&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;private
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Visibility::Protected&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;protected
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;
&nbsp;&nbsp;&nbsp;&nbsp;field:&nbsp;string;
&#125;;
``` **Generates:**
 ```
public&nbsp;field:&nbsp;string;
``` ### Match with Value Extraction
 ```
let&nbsp;result:&nbsp;Result&#x3C;i32,&nbsp;&#x26;str>&nbsp;=&nbsp;Ok(42);

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;value&nbsp;=&nbsp;&#123;#match&nbsp;result&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Ok(val)&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;val&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;Err(msg)&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;throw&nbsp;new&nbsp;Error("@&#123;msg&#125;")
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;;
&#125;;
``` ### Match with Wildcard
 ```
let&nbsp;count&nbsp;=&nbsp;5;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#match&nbsp;count&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("none");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("one");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:case&nbsp;_&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("many");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/match&#125;
&#125;;
``` ## Iteration: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#for}</code>
 ```
let&nbsp;fields&nbsp;=&nbsp;vec!["name",&nbsp;"email",&nbsp;"age"];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;function&nbsp;toJSON()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;field&#125;&nbsp;=&nbsp;this.@&#123;field&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;result;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
``` **Generates:**
 ```
function&nbsp;toJSON()&nbsp;&#123;
&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;result.name&nbsp;=&nbsp;this.name;
&nbsp;&nbsp;result.email&nbsp;=&nbsp;this.email;
&nbsp;&nbsp;result.age&nbsp;=&nbsp;this.age;
&nbsp;&nbsp;return&nbsp;result;
&#125;
``` ### Tuple Destructuring in Loops
 ```
let&nbsp;items&nbsp;=&nbsp;vec![("user",&nbsp;"User"),&nbsp;("post",&nbsp;"Post")];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(key,&nbsp;class_name)&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;@&#123;key&#125;&nbsp;=&nbsp;new&nbsp;@&#123;class_name&#125;();
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;;
``` ### Nested Iterations
 ```
let&nbsp;classes&nbsp;=&nbsp;vec![
&nbsp;&nbsp;&nbsp;&nbsp;("User",&nbsp;vec!["name",&nbsp;"email"]),
&nbsp;&nbsp;&nbsp;&nbsp;("Post",&nbsp;vec!["title",&nbsp;"content"]),
];

ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(class_name,&nbsp;fields)&nbsp;in&nbsp;classes&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.toJSON&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;
``` ## While Loops: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{#<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while</code> for loops that need to continue until a condition is false:
 ```
let&nbsp;items&nbsp;=&nbsp;get_items();
let&nbsp;mut&nbsp;idx&nbsp;=&nbsp;0;

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;i&nbsp;=&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;i&nbsp;&#x3C;&nbsp;items.len()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Item&nbsp;@&#123;i&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;i&nbsp;+=&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` ### While-Let Pattern Matching
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">while<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code> for iterating with pattern matching, similar to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">if<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"> let</code>:
 ```
let&nbsp;mut&nbsp;items&nbsp;=&nbsp;vec!["a",&nbsp;"b",&nbsp;"c"].into_iter();

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;let&nbsp;Some(item)&nbsp;=&nbsp;items.next()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("@&#123;item&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` **Generates:**
 ```
console.log("a");
console.log("b");
console.log("c");
``` This is especially useful when working with iterators or consuming optional values:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#while&nbsp;let&nbsp;Some(next_field)&nbsp;=&nbsp;remaining_fields.pop()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;next_field.name&#125;&nbsp;=&nbsp;this.@&#123;next_field.name&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/while&#125;
&#125;;
``` ## Local Constants: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let}</code>
 Define local variables within the template scope:
 ```
let&nbsp;items&nbsp;=&nbsp;vec![("user",&nbsp;"User"),&nbsp;("post",&nbsp;"Post")];

let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;(key,&nbsp;class_name)&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;upper&nbsp;=&nbsp;class_name.to_uppercase()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Processing&nbsp;@&#123;upper&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;@&#123;key&#125;&nbsp;=&nbsp;new&nbsp;@&#123;class_name&#125;();
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&#125;;
``` This is useful for computing derived values inside loops without cluttering the Rust code.
 ## Mutable Variables: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let mut}</code>
 When you need to modify a variable within the template (e.g., in a `while` loop), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$let mut}</code>:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;count&nbsp;=&nbsp;0&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;item&nbsp;in&nbsp;items&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;console.log("Item&nbsp;@&#123;count&#125;:&nbsp;@&#123;item&#125;");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;count&nbsp;+=&nbsp;1&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;console.log("Total:&nbsp;@&#123;count&#125;");
&#125;;
``` ## Side Effects: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do}</code>
 Execute an expression for its side effects without producing output. This is commonly used with mutable variables:
 ```
let&nbsp;code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$let&nbsp;mut&nbsp;results:&nbsp;Vec&#x3C;String>&nbsp;=&nbsp;Vec::new()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$do&nbsp;results.push(format!("this.&#123;&#125;",&nbsp;field))&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;[@&#123;results.join(",&nbsp;")&#125;];
&#125;;
``` Common uses for <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do}</code>:
 - Incrementing counters: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do i <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> 1<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 - Building collections: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do vec.<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">push<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(item)}</code>
 - Setting flags: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$do found <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">=<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> true<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">}</code>
 - Any mutating operation
 ## TsStream Injection: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">{$typescript}</code>
 Inject another TsStream into your template, preserving both its source code and runtime patches (like imports added via <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">add_import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>):
 ```
//&nbsp;Create&nbsp;a&nbsp;helper&nbsp;method&nbsp;with&nbsp;its&nbsp;own&nbsp;import
let&nbsp;mut&nbsp;helper&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;validateEmail(email:&nbsp;string):&nbsp;boolean&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;Result.ok(true);
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
helper.add_import("Result",&nbsp;"macroforge/utils");

//&nbsp;Inject&nbsp;the&nbsp;helper&nbsp;into&nbsp;the&nbsp;main&nbsp;template
let&nbsp;result&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;$typescript&nbsp;helper&#125;

&nbsp;&nbsp;&nbsp;&nbsp;process(data:&nbsp;Record&#x3C;string,&nbsp;unknown>):&nbsp;void&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;
//&nbsp;result&nbsp;now&nbsp;includes&nbsp;helper's&nbsp;source&nbsp;AND&nbsp;its&nbsp;Result&nbsp;import
``` This is essential for composing multiple macro outputs while preserving imports and patches:
 ```
let&nbsp;extra_methods&nbsp;=&nbsp;if&nbsp;include_validation&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Some(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;boolean&nbsp;&#123;&nbsp;return&nbsp;true;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&#125;&nbsp;else&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;None
&#125;;

body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;mainMethod():&nbsp;void&nbsp;&#123;&#125;

&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;let&nbsp;Some(methods)&nbsp;=&nbsp;extra_methods&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;$typescript&nbsp;methods&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&#125;
``` ## Escape Syntax
 If you need a literal <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@{</code> in your output (not interpolation), use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@@{</code>:
 ```
ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;This&nbsp;outputs&nbsp;a&nbsp;literal&nbsp;@&#123;foo&#125;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;example&nbsp;=&nbsp;"Use&nbsp;@@&#123;foo&#125;&nbsp;for&nbsp;templates";
&#125;
``` **Generates:**
 ```
//&nbsp;This&nbsp;outputs&nbsp;a&nbsp;literal&nbsp;@&#123;foo&#125;
const&nbsp;example&nbsp;=&nbsp;"Use&nbsp;@&#123;foo&#125;&nbsp;for&nbsp;templates";
``` ## Complete Example: JSON Derive Macro
 Here's a comparison showing how <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> simplifies code generation:
 ### Before (Manual AST Building)
 ```
pub&nbsp;fn&nbsp;derive_json_macro(input:&nbsp;TsStream)&nbsp;->&nbsp;MacroResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;mut&nbsp;body_stmts&nbsp;=&nbsp;vec![ts_quote!(&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;&nbsp;as&nbsp;Stmt&nbsp;)];

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;for&nbsp;field_name&nbsp;in&nbsp;class.field_names()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts.push(ts_quote!(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.$(ident!("&#123;&#125;",&nbsp;field_name))&nbsp;=&nbsp;this.$(ident!("&#123;&#125;",&nbsp;field_name));
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;as&nbsp;Stmt
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;));
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts.push(ts_quote!(&nbsp;return&nbsp;result;&nbsp;as&nbsp;Stmt&nbsp;));

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;runtime_code&nbsp;=&nbsp;fn_assign!(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;member_expr!(Expr::Ident(ident!(class_name)),&nbsp;"prototype"),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"toJSON",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;body_stmts
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;);

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ### After (With ts_template!)
 ```
pub&nbsp;fn&nbsp;derive_json_macro(input:&nbsp;TsStream)&nbsp;->&nbsp;MacroResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;fields&nbsp;=&nbsp;class.field_names();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;runtime_code&nbsp;=&nbsp;ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;class_name&#125;.prototype.toJSON&nbsp;=&nbsp;function()&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;&#123;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;fields&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;result.@&#123;field&#125;&nbsp;=&nbsp;this.@&#123;field&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;result;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## How It Works
 1. **Compile-Time:** The template is parsed during macro expansion
 2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
 3. **SWC Parsing:** The generated string is parsed with SWC to produce a typed AST
 4. **Result:** Returns <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Stmt</code> that can be used in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroResult</code> patches
 ## Return Type
 <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> returns a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Result<span style="--shiki-dark:#F97583;--shiki-light:#D73A49"><<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Stmt, TsSynError<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">></code> by default. The macro automatically unwraps and provides helpful error messages showing the generated TypeScript code if parsing fails:
 ```
Failed&nbsp;to&nbsp;parse&nbsp;generated&nbsp;TypeScript:
User.prototype.toJSON&nbsp;=&nbsp;function(&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;&#125;;
&#125;
``` This shows you exactly what was generated, making debugging easy!
 ## Nesting and Regular TypeScript
 You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:
 - **Template tags** if they start with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">$</code>, <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">:</code>, or <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/</code>
 - **Regular TypeScript blocks** otherwise
 ```
ts_template!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;config&nbsp;=&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#if&nbsp;use_strict&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;strict:&nbsp;true,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;:else&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;strict:&nbsp;false,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/if&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;timeout:&nbsp;5000
&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&#125;
``` ## Comparison with Alternatives
 | Approach | Pros | Cons |
| --- | --- | --- |
| ts_quote! | Compile-time validation, type-safe | Can't handle Vec<Stmt>, verbose |
| parse_ts_str() | Maximum flexibility | Runtime parsing, less readable |
| ts_template! | Readable, handles loops/conditions | Small runtime parsing overhead |
 ## Best Practices
 1. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_template!</code> for complex code generation with loops/conditions
 2. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">ts_quote!</code> for simple, static statements
 3. Keep templates readable - extract complex logic into variables
 4. Don't nest templates too deeply - split into helper functions

---

# Integration

# Integration
 *Macroforge integrates with your development workflow through IDE plugins and build tool integration.*
 ## Overview
 | Integration | Purpose | Package |
| --- | --- | --- |
| TypeScript Plugin | IDE support (errors, completions) | @macroforge/typescript-plugin |
| Vite Plugin | Build-time macro expansion | @macroforge/vite-plugin |
 ## Recommended Setup
 For the best development experience, use both integrations:
 1. **TypeScript Plugin**: Provides real-time feedback in your IDE
 2. **Vite Plugin**: Expands macros during development and production builds
 ```
#&nbsp;Install&nbsp;both&nbsp;plugins
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/typescript-plugin&nbsp;@macroforge/vite-plugin
``` ## How They Work Together
 <div class="flex justify-center"><div class="border-2 border-primary bg-primary/10 rounded-lg px-6 py-3 text-center">Your Code TypeScript with @derive decorators  <div class="absolute top-0 h-px bg-border" style="width: 50%; left: 25%;">

---

# Command Line Interface
  *This binary provides command-line utilities for working with Macroforge TypeScript macros. It is designed for development workflows, enabling macro expansion and type checking without requiring Node.js integration.*
 ## Installation
 The CLI is a Rust binary. You can install it using Cargo:
 ```
cargo&nbsp;install&nbsp;macroforge_ts
``` Or build from source:
 ```
git&nbsp;clone&nbsp;https://github.com/rymskip/macroforge-ts.git
cd&nbsp;macroforge-ts/crates
cargo&nbsp;build&nbsp;--release&nbsp;--bin&nbsp;macroforge

#&nbsp;The&nbsp;binary&nbsp;is&nbsp;at&nbsp;target/release/macroforge
``` ## Commands
 ### macroforge expand
 Expands macros in a TypeScript file and outputs the transformed code.
 ```
macroforge&nbsp;expand&nbsp;&#x3C;input>&nbsp;[options]
``` #### Arguments
 | Argument | Description |
| --- | --- |
| <input> | Path to the TypeScript or TSX file to expand |
 #### Options
 | Option | Description |
| --- | --- |
| --out <path> | Write the expanded JavaScript/TypeScript to a file |
| --types-out <path> | Write the generated .d.ts declarations to a file |
| --print | Print output to stdout even when --out is specified |
| --builtin-only | Use only built-in Rust macros (faster, but no external macro support) |
 #### Examples
 Expand a file and print to stdout:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts
``` Expand and write to a file:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--out&nbsp;dist/user.js
``` Expand with both runtime output and type declarations:
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--out&nbsp;dist/user.js&nbsp;--types-out&nbsp;dist/user.d.ts
``` Use fast built-in macros only (no external macro support):
 ```
macroforge&nbsp;expand&nbsp;src/user.ts&nbsp;--builtin-only
``` > **Note:** By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where macroforge and any external macro packages are installed in node_modules. ### macroforge tsc
 Runs TypeScript type checking with macro expansion. This wraps <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsc <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">noEmit</code> and expands macros before type checking, so your generated methods are properly type-checked.
 ```
macroforge&nbsp;tsc&nbsp;[options]
``` #### Options
 | Option | Description |
| --- | --- |
| -p, --project <path> | Path to tsconfig.json (defaults to tsconfig.json in current directory) |
 #### Examples
 Type check with default tsconfig.json:
 ```
macroforge&nbsp;tsc
``` Type check with a specific config:
 ```
macroforge&nbsp;tsc&nbsp;-p&nbsp;tsconfig.build.json
``` ## Output Format
 ### Expanded Code
 When expanding a file like this:
 ```
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;
``` The CLI outputs the expanded code with the generated methods:
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
``` ### Diagnostics
 Errors and warnings are printed to stderr in a readable format:
 ```
[macroforge]&nbsp;error&nbsp;at&nbsp;src/user.ts:5:1:&nbsp;Unknown&nbsp;derive&nbsp;macro:&nbsp;InvalidMacro
[macroforge]&nbsp;warning&nbsp;at&nbsp;src/user.ts:10:3:&nbsp;Field&nbsp;'unused'&nbsp;is&nbsp;never&nbsp;used
``` ## Use Cases
 ### CI/CD Type Checking
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> tsc</code> in your CI pipeline to type-check with macro expansion:
 ```
#&nbsp;package.json
&#123;
&nbsp;&nbsp;"scripts":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"typecheck":&nbsp;"macroforge&nbsp;tsc"
&nbsp;&nbsp;&#125;
&#125;
``` ### Debugging Macro Output
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">macroforge<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62"> expand</code> to inspect what code your macros generate:
 ```
macroforge&nbsp;expand&nbsp;src/models/user.ts&nbsp;|&nbsp;less
``` ### Build Pipeline
 Generate expanded files as part of a custom build:
 ```
#!/bin/bash
for&nbsp;file&nbsp;in&nbsp;src/**/*.ts;&nbsp;do
&nbsp;&nbsp;outfile="dist/$(basename&nbsp;"$file"&nbsp;.ts).js"
&nbsp;&nbsp;macroforge&nbsp;expand&nbsp;"$file"&nbsp;--out&nbsp;"$outfile"
done
``` ## Built-in vs Full Mode
 By default, the CLI uses Node.js for full macro support including external macros. Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">--<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">builtin<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">only</code> for faster expansion when you only need built-in macros:
 | Feature | Default (Node.js) | --builtin-only (Rust) |
| --- | --- | --- |
| Built-in macros | Yes | Yes |
| External macros | Yes | No |
| Performance | Standard | Faster |
| Dependencies | Requires macroforge in node_modules | None |

---

# TypeScript Plugin
 *The TypeScript plugin provides IDE integration for Macroforge, including error reporting, completions, and type checking for generated code.*
 ## Installation
 ```
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/typescript-plugin
``` ## Configuration
 Add the plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>:
 ```
&#123;
&nbsp;&nbsp;"compilerOptions":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"plugins":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"name":&nbsp;"@macroforge/typescript-plugin"
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;]
&nbsp;&nbsp;&#125;
&#125;
``` ## VS Code Setup
 VS Code uses its own TypeScript version by default. To use the workspace version (which includes plugins):
 1. Open the Command Palette (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Cmd<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">Ctrl <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E"> Shift <span style="--shiki-dark:#F97583;--shiki-light:#D73A49">+<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5"> P</code>)
 2. Search for "TypeScript: Select TypeScript Version"
 3. Choose "Use Workspace Version"
  **Tip Add this setting to your **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.vscode/settings.json</code> to make it permanent: ```
&#123;
&nbsp;&nbsp;"typescript.tsdk":&nbsp;"node_modules/typescript/lib"
&#125;
``` ## Features
 ### Error Reporting
 Errors in macro-generated code are reported at the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator position:
 ```
/**&nbsp;@derive(Debug)&nbsp;*/&nbsp;&nbsp;//&nbsp;&#x3C;-&nbsp;Errors&nbsp;appear&nbsp;here
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&#125;
``` ### Completions
 The plugin provides completions for generated methods:
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice");
user.to&nbsp;&nbsp;//&nbsp;Suggests:&nbsp;toString(),&nbsp;toJSON(),&nbsp;etc.
``` ### Type Information
 Hover over generated methods to see their types:
 ```
//&nbsp;Hover&nbsp;over&nbsp;'clone'&nbsp;shows:
//&nbsp;(method)&nbsp;User.clone():&nbsp;User
const&nbsp;copy&nbsp;=&nbsp;user.clone();
``` ## Troubleshooting
 ### Plugin Not Loading
 1. Ensure you're using the workspace TypeScript version
 2. Restart the TypeScript server (Command Palette → "TypeScript: Restart TS Server")
 3. Check that the plugin is listed in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code>
 ### Errors Not Showing
 If errors from macros aren't appearing:
 1. Make sure the Vite plugin is also installed (for source file watching)
 2. Check that your file is saved (plugins process on save)

---

# Vite Plugin
 *The Vite plugin provides build-time macro expansion, transforming your code during development and production builds.*
 ## Installation
 ```
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/vite-plugin
``` ## Configuration
 Add the plugin to your <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">vite.config.ts</code>:
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge()
&nbsp;&nbsp;]
&#125;);
``` ## Options
 ```
macroforge(&#123;
&nbsp;&nbsp;//&nbsp;Generate&nbsp;.d.ts&nbsp;files&nbsp;for&nbsp;expanded&nbsp;code
&nbsp;&nbsp;generateTypes:&nbsp;true,

&nbsp;&nbsp;//&nbsp;Output&nbsp;directory&nbsp;for&nbsp;generated&nbsp;types
&nbsp;&nbsp;typesOutputDir:&nbsp;".macroforge/types",

&nbsp;&nbsp;//&nbsp;Emit&nbsp;metadata&nbsp;files&nbsp;for&nbsp;debugging
&nbsp;&nbsp;emitMetadata:&nbsp;false,

&nbsp;&nbsp;//&nbsp;Keep&nbsp;@derive&nbsp;decorators&nbsp;in&nbsp;output&nbsp;(for&nbsp;debugging)
&nbsp;&nbsp;keepDecorators:&nbsp;false,

&nbsp;&nbsp;//&nbsp;File&nbsp;patterns&nbsp;to&nbsp;process
&nbsp;&nbsp;include:&nbsp;["**/*.ts",&nbsp;"**/*.tsx"],
&nbsp;&nbsp;exclude:&nbsp;["node_modules/**"]
&#125;)
``` ### Option Reference
 | Option | Type | Default | Description |
| --- | --- | --- | --- |
| generateTypes | boolean | true | Generate .d.ts files |
| typesOutputDir | string | .macroforge/types | Where to write type files |
| emitMetadata | boolean | false | Emit macro metadata files |
| keepDecorators | boolean | false | Keep decorators in output |
 ## Framework Integration
 ### React (Vite)
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;react&nbsp;from&nbsp;"@vitejs/plugin-react";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),&nbsp;&nbsp;//&nbsp;Before&nbsp;React&nbsp;plugin
&nbsp;&nbsp;&nbsp;&nbsp;react()
&nbsp;&nbsp;]
&#125;);
``` ### SvelteKit
 ```
import&nbsp;macroforge&nbsp;from&nbsp;"@macroforge/vite-plugin";
import&nbsp;&#123;&nbsp;sveltekit&nbsp;&#125;&nbsp;from&nbsp;"@sveltejs/kit/vite";
import&nbsp;&#123;&nbsp;defineConfig&nbsp;&#125;&nbsp;from&nbsp;"vite";

export&nbsp;default&nbsp;defineConfig(&#123;
&nbsp;&nbsp;plugins:&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;macroforge(),&nbsp;&nbsp;//&nbsp;Before&nbsp;SvelteKit
&nbsp;&nbsp;&nbsp;&nbsp;sveltekit()
&nbsp;&nbsp;]
&#125;);
``` > **Note:** Always place the Macroforge plugin before other framework plugins to ensure macros are expanded first. ## Development Server
 During development, the plugin:
 - Watches for file changes
 - Expands macros on save
 - Provides HMR support for expanded code
 ## Production Build
 During production builds, the plugin:
 - Expands all macros in the source files
 - Generates type declaration files
 - Strips <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators from output

---

# Configuration
 *Macroforge can be configured with a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge.json</code> file in your project root.*
 ## Configuration File
 Create a <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge.json</code> file:
 ```
&#123;
&nbsp;&nbsp;"allowNativeMacros":&nbsp;true,
&nbsp;&nbsp;"macroPackages":&nbsp;[],
&nbsp;&nbsp;"keepDecorators":&nbsp;false,
&nbsp;&nbsp;"limits":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;5000,
&nbsp;&nbsp;&nbsp;&nbsp;"maxMemoryBytes":&nbsp;104857600,
&nbsp;&nbsp;&nbsp;&nbsp;"maxOutputSize":&nbsp;10485760,
&nbsp;&nbsp;&nbsp;&nbsp;"maxDiagnostics":&nbsp;100
&nbsp;&nbsp;&#125;
&#125;
``` ## Options Reference
 ### allowNativeMacros
 | Type | boolean |
| Default | true |
 Enable or disable native (Rust) macro packages. Set to <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">false</code> to only allow built-in macros.
 ### macroPackages
 | Type | string[] |
| Default | [] |
 List of npm packages that provide macros. Macroforge will look for macros in these packages.
 ```
&#123;
&nbsp;&nbsp;"macroPackages":&nbsp;[
&nbsp;&nbsp;&nbsp;&nbsp;"@my-org/custom-macros",
&nbsp;&nbsp;&nbsp;&nbsp;"community-macros"
&nbsp;&nbsp;]
&#125;
``` ### keepDecorators
 | Type | boolean |
| Default | false |
 Keep <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorators in the output. Useful for debugging.
 ### limits
 Configure resource limits for macro expansion:
 ```
&#123;
&nbsp;&nbsp;"limits":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;time&nbsp;for&nbsp;a&nbsp;single&nbsp;macro&nbsp;expansion&nbsp;(ms)
&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;5000,

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;memory&nbsp;usage&nbsp;(bytes)
&nbsp;&nbsp;&nbsp;&nbsp;"maxMemoryBytes":&nbsp;104857600,&nbsp;&nbsp;//&nbsp;100MB

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;size&nbsp;of&nbsp;generated&nbsp;code&nbsp;(bytes)
&nbsp;&nbsp;&nbsp;&nbsp;"maxOutputSize":&nbsp;10485760,&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;10MB

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Maximum&nbsp;number&nbsp;of&nbsp;diagnostics&nbsp;per&nbsp;file
&nbsp;&nbsp;&nbsp;&nbsp;"maxDiagnostics":&nbsp;100
&nbsp;&nbsp;&#125;
&#125;
``` ## Macro Runtime Overrides
 Override settings for specific macros:
 ```
&#123;
&nbsp;&nbsp;"macroRuntimeOverrides":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;"@my-org/macros":&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"maxExecutionTimeMs":&nbsp;10000
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&#125;
&#125;
```  **Warning Be careful when increasing limits, as this could allow malicious macros to consume excessive resources. ## Environment Variables
 Some settings can be overridden with environment variables:
 | Variable | Description |
| --- | --- |
| MACROFORGE_DEBUG | Enable debug logging |
| MACROFORGE_LOG_FILE | Write logs to a file |
 ```
MACROFORGE_DEBUG=1&nbsp;npm&nbsp;run&nbsp;dev
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
 *<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">language<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">server</code> provides full Svelte IDE support with macroforge integration.*
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
git&nbsp;clone&nbsp;https://github.com/rymskip/macroforge-ts.git
cd&nbsp;macroforge-ts
``` ### 2. Build the Language Server
 ```
#&nbsp;Install&nbsp;dependencies
npm&nbsp;install

#&nbsp;Build&nbsp;the&nbsp;Svelte&nbsp;language&nbsp;server
cd&nbsp;packages/svelte-language-server
npm&nbsp;run&nbsp;build
``` ### 3. Configure Your Editor
 The language server exposes a **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelteserver</code> binary that implements the Language Server Protocol (LSP). Configure your editor to use it:
 ```
#&nbsp;The&nbsp;binary&nbsp;is&nbsp;located&nbsp;at:
./packages/svelte-language-server/bin/server.js
``` ## Package Info
 | Package | @macroforge/svelte-language-server |
| Version | 0.1.7 |
| CLI Command | svelteserver |
| Node Version | >= 18.0.0 |
 ## How It Works
 The Svelte language server extends the standard Svelte language tooling with macroforge integration:
 1. Parses <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">.svelte</code> files and extracts TypeScript/JavaScript blocks
 2. Expands macros using the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">typescript<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">plugin</code>
 3. Maps diagnostics back to original source positions
 4. Provides completions for macro-generated methods
 ## Using with Zed
 For Zed editor, see the [Zed Extensions](../../docs/language-servers/zed) page for the dedicated <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge</code> extension.

---

# Zed Extensions
 *Macroforge provides two extensions for the [Zed editor](https://zed.dev): one for TypeScript via VTSLS, and one for Svelte.*
  **Developer Installation Required These extensions are not yet in the Zed extension registry. You'll need to install them as developer extensions. ## Available Extensions
 | Extension | Description | Location |
| --- | --- | --- |
| vtsls-macroforge | VTSLS with macroforge support for TypeScript | crates/extensions/vtsls-macroforge |
| svelte-macroforge | Svelte language support with macroforge | crates/extensions/svelte-macroforge |
 ## Installation
 ### 1. Clone the Repository
 ```
git&nbsp;clone&nbsp;https://github.com/rymskip/macroforge-ts.git
cd&nbsp;macroforge-ts
``` ### 2. Build the Extension
 Build the extension you want to use:
 ```
#&nbsp;For&nbsp;VTSLS&nbsp;(TypeScript)
cd&nbsp;crates/extensions/vtsls-macroforge

#&nbsp;Or&nbsp;for&nbsp;Svelte
cd&nbsp;crates/extensions/svelte-macroforge
``` ### 3. Install as Dev Extension in Zed
 In Zed, open the command palette and run **zed: install dev extension**, then select the extension directory.
 Alternatively, symlink the extension to your Zed extensions directory:
 ```
#&nbsp;macOS
ln&nbsp;-s&nbsp;/path/to/macroforge-ts/crates/extensions/vtsls-macroforge&nbsp;~/Library/Application\\&nbsp;Support/Zed/extensions/installed/vtsls-macroforge

#&nbsp;Linux
ln&nbsp;-s&nbsp;/path/to/macroforge-ts/crates/extensions/vtsls-macroforge&nbsp;~/.config/zed/extensions/installed/vtsls-macroforge
``` ## vtsls-macroforge
 This extension wraps [VTSLS](https://github.com/yioneko/vtsls) (a TypeScript language server) with macroforge integration. It provides:
 - Full TypeScript language features
 - Macro expansion at edit time
 - Accurate error positions in original source
 - Completions for macro-generated methods
 ## svelte-macroforge
 This extension provides Svelte support using the **<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@macroforge<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">/<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">svelte<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">language<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">-<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">server</code>. It includes:
 - Svelte component syntax support
 - HTML, CSS, and TypeScript features
 - Macroforge integration in script blocks
 ## Troubleshooting
 ### Extension not loading
 Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error messages.
 ### Macros not expanding
 Ensure your project has the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge</code> package installed and a valid <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">tsconfig.json</code> with the TypeScript plugin configured.

---

# API Reference

# API Reference
 *Macroforge provides a programmatic API for expanding macros in TypeScript code.*
 ## Overview
 ```
import&nbsp;&#123;
&nbsp;&nbsp;expandSync,
&nbsp;&nbsp;transformSync,
&nbsp;&nbsp;checkSyntax,
&nbsp;&nbsp;parseImportSources,
&nbsp;&nbsp;NativePlugin,
&nbsp;&nbsp;PositionMapper
&#125;&nbsp;from&nbsp;"macroforge";
``` ## Core Functions
 | Function | Description |
| --- | --- |
| [expandSync()](../docs/api/expand-sync) | Expand macros synchronously |
| [transformSync()](../docs/api/transform-sync) | Transform code with additional metadata |
| checkSyntax() | Validate TypeScript syntax |
| parseImportSources() | Extract import information |
 ## Classes
 | Class | Description |
| --- | --- |
| [NativePlugin](../docs/api/native-plugin) | Stateful plugin with caching |
| [PositionMapper](../docs/api/position-mapper) | Maps positions between original and expanded code |
 ## Quick Example
 ```
import&nbsp;&#123;&nbsp;expandSync&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

const&nbsp;sourceCode&nbsp;=&nbsp;\`
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;constructor(name:&nbsp;string)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&#125;
&#125;
\`;

const&nbsp;result&nbsp;=&nbsp;expandSync(sourceCode,&nbsp;"user.ts",&nbsp;&#123;
&nbsp;&nbsp;keepDecorators:&nbsp;false
&#125;);

console.log(result.code);
//&nbsp;Output:&nbsp;class&nbsp;with&nbsp;toString()&nbsp;method&nbsp;generated

if&nbsp;(result.diagnostics.length&nbsp;>&nbsp;0)&nbsp;&#123;
&nbsp;&nbsp;console.error("Errors:",&nbsp;result.diagnostics);
&#125;
``` ## Detailed Reference
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">expandSync<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>](../docs/api/expand-sync) - Full options and return types
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">transformSync<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>](../docs/api/transform-sync) - Transform with source maps
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code>](../docs/api/native-plugin) - Caching for language servers
 - [<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">PositionMapper</code>](../docs/api/position-mapper) - Position mapping utilities

---

# expandSync()
  *Synchronously expands macros in TypeScript code. This is the standalone macro expansion function that doesn't use caching. For cached expansion, use [`NativePlugin::process_file`] instead.*
 ## Signature
 ```
function&nbsp;expandSync(
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;options?:&nbsp;ExpandOptions
):&nbsp;ExpandResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| code | string | TypeScript source code to transform |
| filepath | string | File path (used for error reporting) |
| options | ExpandOptions | Optional configuration |
 ## ExpandOptions
 ```
interface&nbsp;ExpandOptions&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Keep&nbsp;@derive&nbsp;decorators&nbsp;in&nbsp;output&nbsp;(default:&nbsp;false)
&nbsp;&nbsp;keepDecorators?:&nbsp;boolean;
&#125;
``` ## ExpandResult
 ```
interface&nbsp;ExpandResult&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Transformed&nbsp;TypeScript&nbsp;code
&nbsp;&nbsp;code:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Generated&nbsp;type&nbsp;declarations&nbsp;(.d.ts&nbsp;content)
&nbsp;&nbsp;types?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Macro&nbsp;expansion&nbsp;metadata&nbsp;(JSON&nbsp;string)
&nbsp;&nbsp;metadata?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Warnings&nbsp;and&nbsp;errors&nbsp;from&nbsp;macro&nbsp;expansion
&nbsp;&nbsp;diagnostics:&nbsp;MacroDiagnostic[];

&nbsp;&nbsp;//&nbsp;Position&nbsp;mapping&nbsp;data&nbsp;for&nbsp;source&nbsp;maps
&nbsp;&nbsp;sourceMapping?:&nbsp;SourceMappingResult;
&#125;
``` ## MacroDiagnostic
 ```
interface&nbsp;MacroDiagnostic&nbsp;&#123;
&nbsp;&nbsp;message:&nbsp;string;
&nbsp;&nbsp;severity:&nbsp;"error"&nbsp;|&nbsp;"warning"&nbsp;|&nbsp;"info";
&nbsp;&nbsp;span:&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;start:&nbsp;number;
&nbsp;&nbsp;&nbsp;&nbsp;end:&nbsp;number;
&nbsp;&nbsp;&#125;;
&#125;
``` ## Example
 ```
import { expandSync } from "macroforge";

const sourceCode = `
/** @derive(Debug) */
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
``` ## Error Handling
 Syntax errors and macro errors are returned in the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">diagnostics</code> array, not thrown as exceptions:
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
function&nbsp;transformSync(
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;filepath:&nbsp;string
):&nbsp;TransformResult
``` ## Parameters
 | Parameter | Type | Description |
| --- | --- | --- |
| code | string | TypeScript source code to transform |
| filepath | string | File path (used for error reporting) |
 ## TransformResult
 ```
interface&nbsp;TransformResult&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Transformed&nbsp;TypeScript&nbsp;code
&nbsp;&nbsp;code:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Source&nbsp;map&nbsp;(JSON&nbsp;string,&nbsp;not&nbsp;yet&nbsp;implemented)
&nbsp;&nbsp;map?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Generated&nbsp;type&nbsp;declarations
&nbsp;&nbsp;types?:&nbsp;string;

&nbsp;&nbsp;//&nbsp;Macro&nbsp;expansion&nbsp;metadata
&nbsp;&nbsp;metadata?:&nbsp;string;
&#125;
``` ## Comparison with expandSync()
 | Feature | expandSync | transformSync |
| --- | --- | --- |
| Options | Yes | No |
| Diagnostics | Yes | No |
| Source Mapping | Yes | Limited |
| Use Case | General purpose | Build tools |
 ## Example
 ```
import&nbsp;&#123;&nbsp;transformSync&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

const&nbsp;sourceCode&nbsp;=&nbsp;\`
/**&nbsp;@derive(Debug)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&#125;
\`;

const&nbsp;result&nbsp;=&nbsp;transformSync(sourceCode,&nbsp;"user.ts");

console.log(result.code);

if&nbsp;(result.types)&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Write&nbsp;to&nbsp;.d.ts&nbsp;file
&nbsp;&nbsp;fs.writeFileSync("user.d.ts",&nbsp;result.types);
&#125;

if&nbsp;(result.metadata)&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Parse&nbsp;and&nbsp;use&nbsp;metadata
&nbsp;&nbsp;const&nbsp;meta&nbsp;=&nbsp;JSON.parse(result.metadata);
&nbsp;&nbsp;console.log("Macros&nbsp;expanded:",&nbsp;meta);
&#125;
``` ## When to Use
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">transformSync</code> when:
 - Building custom integrations
 - You need raw output without diagnostics
 - You're implementing a build tool plugin
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">expandSync</code> for most other use cases, as it provides better error handling.

---

# NativePlugin
  *The main plugin class for macro expansion with caching support. `NativePlugin` is designed to be instantiated once and reused across multiple file processing operations. It maintains a cache of expansion results keyed by filepath and version, enabling efficient incremental processing.*
 ## Constructor
 ```
const&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();
``` ## Methods
 ### processFile()
 Process a file with version-based caching:
 ```
processFile(
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;code:&nbsp;string,
&nbsp;&nbsp;options?:&nbsp;ProcessFileOptions
):&nbsp;ExpandResult
``` ```
interface&nbsp;ProcessFileOptions&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Cache&nbsp;key&nbsp;-&nbsp;if&nbsp;unchanged,&nbsp;returns&nbsp;cached&nbsp;result
&nbsp;&nbsp;version?:&nbsp;string;
&#125;
``` ### getMapper()
 Get the position mapper for a previously processed file:
 ```
getMapper(filepath:&nbsp;string):&nbsp;NativeMapper&nbsp;|&nbsp;null
``` ### mapDiagnostics()
 Map diagnostics from expanded positions to original positions:
 ```
mapDiagnostics(
&nbsp;&nbsp;filepath:&nbsp;string,
&nbsp;&nbsp;diagnostics:&nbsp;JsDiagnostic[]
):&nbsp;JsDiagnostic[]
``` ### log() / setLogFile()
 Logging utilities for debugging:
 ```
log(message:&nbsp;string):&nbsp;void
setLogFile(path:&nbsp;string):&nbsp;void
``` ## Caching Behavior
 The plugin caches expansion results by file path and version:
 ```
const&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();

//&nbsp;First&nbsp;call&nbsp;-&nbsp;performs&nbsp;expansion
const&nbsp;result1&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;code,&nbsp;&#123;&nbsp;version:&nbsp;"1"&nbsp;&#125;);

//&nbsp;Same&nbsp;version&nbsp;-&nbsp;returns&nbsp;cached&nbsp;result&nbsp;instantly
const&nbsp;result2&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;code,&nbsp;&#123;&nbsp;version:&nbsp;"1"&nbsp;&#125;);

//&nbsp;Different&nbsp;version&nbsp;-&nbsp;re-expands
const&nbsp;result3&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;newCode,&nbsp;&#123;&nbsp;version:&nbsp;"2"&nbsp;&#125;);
``` ## Example: Language Server Integration
 ```
import&nbsp;&#123;&nbsp;NativePlugin&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

class&nbsp;MacroforgeLanguageService&nbsp;&#123;
&nbsp;&nbsp;private&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();

&nbsp;&nbsp;processDocument(uri:&nbsp;string,&nbsp;content:&nbsp;string,&nbsp;version:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Process&nbsp;with&nbsp;version-based&nbsp;caching
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;result&nbsp;=&nbsp;this.plugin.processFile(uri,&nbsp;content,&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;version:&nbsp;String(version)
&nbsp;&nbsp;&nbsp;&nbsp;&#125;);

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Get&nbsp;mapper&nbsp;for&nbsp;position&nbsp;translation
&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;mapper&nbsp;=&nbsp;this.plugin.getMapper(uri);

&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;&nbsp;result,&nbsp;mapper&nbsp;&#125;;
&nbsp;&nbsp;&#125;

&nbsp;&nbsp;getSemanticDiagnostics(uri:&nbsp;string,&nbsp;diagnostics:&nbsp;Diagnostic[])&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Map&nbsp;positions&nbsp;from&nbsp;expanded&nbsp;to&nbsp;original
&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;this.plugin.mapDiagnostics(uri,&nbsp;diagnostics);
&nbsp;&nbsp;&#125;
&#125;
``` ## Thread Safety
 The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">NativePlugin</code> class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.

---

# PositionMapper
  *Bidirectional position mapper for translating between original and expanded source positions. This mapper enables IDE features like error reporting, go-to-definition, and hover to work correctly with macro-expanded code by translating positions between the original source (what the user wrote) and the expanded source (what the compiler sees).*
 ## Getting a Mapper
 ```
import&nbsp;&#123;&nbsp;NativePlugin,&nbsp;PositionMapper&nbsp;&#125;&nbsp;from&nbsp;"macroforge";

const&nbsp;plugin&nbsp;=&nbsp;new&nbsp;NativePlugin();
const&nbsp;result&nbsp;=&nbsp;plugin.processFile("user.ts",&nbsp;code,&nbsp;&#123;&nbsp;version:&nbsp;"1"&nbsp;&#125;);

//&nbsp;Get&nbsp;the&nbsp;mapper&nbsp;for&nbsp;this&nbsp;file
const&nbsp;mapper&nbsp;=&nbsp;plugin.getMapper("user.ts");
if&nbsp;(mapper)&nbsp;&#123;
&nbsp;&nbsp;//&nbsp;Use&nbsp;the&nbsp;mapper...
&#125;
``` ## Methods
 ### isEmpty()
 Check if the mapper has any mappings:
 ```
isEmpty():&nbsp;boolean
``` ### originalToExpanded()
 Map a position from original to expanded code:
 ```
originalToExpanded(pos:&nbsp;number):&nbsp;number
``` ### expandedToOriginal()
 Map a position from expanded to original code:
 ```
expandedToOriginal(pos:&nbsp;number):&nbsp;number&nbsp;|&nbsp;null
``` Returns <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">null</code> if the position is in generated code.
 ### isInGenerated()
 Check if a position is in macro-generated code:
 ```
isInGenerated(pos:&nbsp;number):&nbsp;boolean
``` ### generatedBy()
 Get the name of the macro that generated code at a position:
 ```
generatedBy(pos:&nbsp;number):&nbsp;string&nbsp;|&nbsp;null
``` ### mapSpanToOriginal()
 Map a span (range) from expanded to original code:
 ```
mapSpanToOriginal(start:&nbsp;number,&nbsp;length:&nbsp;number):&nbsp;SpanResult&nbsp;|&nbsp;null

interface&nbsp;SpanResult&nbsp;&#123;
&nbsp;&nbsp;start:&nbsp;number;
&nbsp;&nbsp;length:&nbsp;number;
&#125;
``` ### mapSpanToExpanded()
 Map a span from original to expanded code:
 ```
mapSpanToExpanded(start:&nbsp;number,&nbsp;length:&nbsp;number):&nbsp;SpanResult
``` ## Example: Error Position Mapping
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
``` ## Performance
 Position mapping uses binary search with O(log n) complexity:
 - Fast lookups even for large files
 - Minimal memory overhead
 - Thread-safe access

---
