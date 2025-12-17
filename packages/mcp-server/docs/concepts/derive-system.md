# The Derive System
 *The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code>.*
 ## Syntax Reference
 Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.
 ### The @derive Statement
 The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> decorator triggers macro expansion on a class or interface:
 **Source:**
```
/** @derive(Debug) */
class MyClass {
  value: string;
}
```  Syntax rules:
 - Must be inside a JSDoc comment (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#6A737D;--shiki-light:#6A737D">/** */</code>)
 - Must appear immediately before the class/interface declaration
 - Multiple macros can be comma-separated: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">A<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">,<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">B<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">,<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">C<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">)</code>
 - Multiple <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code> statements can be stacked
 **Source:**
```
/** @derive(Debug, Clone) */
class User {
  name: string;
  email: string;
}
```  ### The import macro Statement
 To use macros from external packages, you must declare them with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macro</code>:
 ```
/** import macro &#123; MacroName &#125; from "package-name"; */
``` Syntax rules:
 - Must be inside a JSDoc comment (<code class="shiki-inline"><span class="line"><span style="--shiki-dark:#6A737D;--shiki-light:#6A737D">/** */</code>)
 - Can appear anywhere in the file (typically at the top)
 - Multiple macros can be imported: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macro &#123; A, B &#125;<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">from<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"pkg"<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;</code>
 - Multiple import statements can be used for different packages
 ```
/** import macro &#123; JSON, Validate &#125; from "@my/macros"; */
/** import macro &#123; Builder &#125; from "@other/macros"; */

/** @derive(JSON, Validate, Builder) */
class User &#123;
  name: string;
  email: string;
&#125;
```  **Built-in macros Built-in macros (Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize) do not require an import statement. ### Field Attributes
 Macros can define field-level attributes to customize behavior per field:
 ****Before:**
```
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
**After:**
```
import { SerializeContext } from "macroforge/serde";

class User {
  
  
  id: number;

  name: string;

  
  
  password: string;

  
  metadata: Record<string, unknown>;

  static toString(value: User): string {
    return userToString(value);
}
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

export function userToString(value: User): string {const parts: string[]= []; parts.push("userId: " + value.id); parts.push("name: " + value.name); parts.push("metadata: " + value.metadata); return "User { " + parts.join(", " )+ " }" ; }

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */export function userSerialize(value: User): string {const ctx = SerializeContext.create(); return JSON.stringify(userSerializeWithContext(value, ctx));}/** @internal Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */export function userSerializeWithContext(value: User, ctx: SerializeContext): Record<string, unknown>{const existingId = ctx.getId(value); if(existingId!== undefined){return {__ref: existingId};}const __id = ctx.register(value); const result: Record<string, unknown>= {__type: "User" , __id,}; result["user_id" ]= value.id; result["name" ]= value.name; {const __flattened = record<string, unknown>SerializeWithContext(value.metadata, ctx); const {__type: _, __id: __,...rest}= __flattened as any; Object.assign(result, rest);}return result;}
``` Syntax rules:
 - Must be inside a JSDoc comment immediately before the field
 - Options use object literal syntax: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">; key: value<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;)</code>
 - Boolean options: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">; skip:<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">true<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;)</code>
 - String options: <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">attr<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">(<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">123<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">; rename:<span style="--shiki-dark:#9ECBFF;--shiki-light:#032F62">"newName"<span style="--shiki-dark:#F97583;--shiki-light:#D73A49">&<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#<span style="--shiki-dark:#79B8FF;--shiki-light:#005CC5">125<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">;)</code>
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
 Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#F97583;--shiki-light:#D73A49">import<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macro</code> statement.
 | Type | Import Required | Examples |
| --- | --- | --- |
| Built-in | No | Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize |
| Custom | Yes | Any macro from an external package |
 ## Next Steps
 - [Explore built-in macros](../../docs/builtin-macros)
 - [Create custom macros](../../docs/custom-macros)