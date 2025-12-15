# The Derive System
 *The derive system is inspired by Rust's derive macros. It allows you to automatically implement common patterns by annotating your classes with `@derive`.*
 ## Syntax Reference
 Macroforge uses JSDoc comments for all macro annotations. This ensures compatibility with standard TypeScript tooling.
 ### The @derive Statement
 The `@derive` decorator triggers macro expansion on a class or interface:
 **Source:**
```
/** @derive(Debug) */
class MyClass {
  value: string;
}
```  Syntax rules:
 - Must be inside a JSDoc comment (`/** */`)
 - Must appear immediately before the class/interface declaration
 - Multiple macros can be comma-separated: `@derive(A, B, C)`
 - Multiple `@derive` statements can be stacked
 **Source:**
```
/** @derive(Debug, Clone) */
class User {
  name: string;
  email: string;
}
```  ### The import macro Statement
 To use macros from external packages, you must declare them with `import macro`:
 ```
/** import macro { MacroName } from "package-name"; */
``` Syntax rules:
 - Must be inside a JSDoc comment (`/** */`)
 - Can appear anywhere in the file (typically at the top)
 - Multiple macros can be imported: `import macro { A, B } from "pkg";`
 - Multiple import statements can be used for different packages
 ```
/** import macro { JSON, Validate } from "@my/macros"; */
/** import macro { Builder } from "@other/macros"; */

/** @derive(JSON, Validate, Builder) */
class User {
  name: string;
  email: string;
}
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

  toString(): string {
    const parts: string[] = [];
    parts.push("userId: " + this.id);
    parts.push("name: " + this.name);
    parts.push("metadata: " + this.metadata);
    return "User { " + parts.join(", ") + " }";
}

  toStringifiedJSON(): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(this.__serialize(ctx));
}

  toObject(): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return this.__serialize(ctx);
}

  __serialize(ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(this);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(this);
    const result: Record<string, unknown> = {
        __type: "User",
        __id
    };
    result["user_id"] = this.id;
    result["name"] = this.name;
    {
        const __flattened = record < string, unknown;
        const { __type: _, __id: __, ...rest } = __flattened as any;
        Object.assign(result, rest);
    }
    return result;
}
}
``` Syntax rules:
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