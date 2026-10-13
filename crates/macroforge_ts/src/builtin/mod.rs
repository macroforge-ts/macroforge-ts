//! # Built-in Derive Macros for Macroforge
//!
//! This module provides the standard derive macros that ship with macroforge-ts.
//! These macros generate common boilerplate methods for TypeScript classes,
//! interfaces, enums, and type aliases.
//!
//! ## Available Macros
//!
//! For **classes**, each macro generates a static method on the class, where the
//! code reaches the class's `private` and `protected` fields, plus a standalone
//! function (e.g. `userClone`, `userEncode`) that runs it.
//! **Enums, interfaces, and type aliases** get standalone functions only (e.g.
//! `statusDefaultValue`, `pointEquals`), since methods cannot be attached to them.
//! The tables below show the class-side static methods. One that a class
//! it extends has as well, by deriving the same macro or by declaring it, is
//! marked `override`, so the class passes `noImplicitOverride`; so is the
//! `toString` of any class with a superclass, which has one as a function.
//!
//! A derive works over every field an instance holds. For a class that is its
//! `private`, `protected` and `readonly` fields, its constructor parameter
//! properties and, when the project was scanned, the fields of its
//! superclasses; for an interface, those of the interfaces it extends. A field
//! keyed by a string or a number, such as `'content-type'`, is a field like any
//! other. What no derive can reach is an expansion error: a `#private` field,
//! a `private` field of a superclass, a superclass or extended interface the
//! project does not declare, and a generic one. `Clone`, `Default` and `Decode`
//! build an instance without running its constructor, so they also reject a
//! class with a field set up by a Svelte rune such as `$state`.
//!
//! ### Equality & Hashing
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialEq` | `static equals(a, b): boolean` | Field-by-field equality comparison |
//! | `Eq` | None | Marks `PartialEq` equality as total, as Rust's `Eq`; requires `PartialEq` |
//! | `Hash` | `static hashCode(value): number` | Hash code generation for collections |
//!
//! ### Ordering
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialOrd` | `static partialCompare(a, b): number \| null` | Partial ordering (can return null); requires `PartialEq` |
//! | `Ord` | `static compare(a, b): number` | Total ordering (never null); requires `Eq` and `PartialOrd` |
//!
//! ### Cloning & Debugging
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Clone` | `static clone(value): T` | Deep copy of the object |
//! | `Debug` | `static toString(value): string` | Human-readable debug representation |
//!
//! ### Initialization
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Default` | `static defaultValue(): T` | Factory method with default values |
//!
//! ### Encoding (Endec)
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Encode` | `static encode(value, keepMetadata?): string` | JSON encoding with cycle detection |
//! | `Decode` | `static decode(input, opts?): { success: true; value: T } \| { success: false; errors }` | JSON decoding with validation |
//!
//! `Decode` additionally generates `static is(value)` / `static hasShape(obj)` type
//! guards and `validateField`/`validateFields` helpers; see [`endec`] for the full surface.
//!
//! ## Values That Hold Themselves
//!
//! Every derive knows a value it has met before in the same walk. `Encode`
//! writes a value held in several places once, and `Decode` reads it back as
//! one. `Clone` gives the copy the shape of the original: a value held twice
//! is one value in the copy, and a cycle closes on the copy. `PartialEq`,
//! `PartialOrd` and `Ord` take a pair they are already comparing for equal,
//! `Hash` lets a value it is already hashing add nothing, and `Debug` prints
//! it as `[Circular]`.
//!
//! ## Names A Derive Reads
//!
//! A derive reads a field's type by its name, so `Map<K, V>`, `Date` and
//! `Record<K, V>` are taken for the global ones, and the code it generates
//! builds and tests values through those globals. A module that declares or
//! imports something of its own under such a name is an expansion error on
//! any built-in derive in it.
//!
//! A member under a computed key (`[KEY]: T`) or an index signature
//! (`[key: string]: T`) has no name to be read by, and is an expansion error
//! on the type deriving: name the member, or hold the entries in a `Record`
//! field.
//!
//! Two derived types whose names differ only in case, such as `HTTPResponse`
//! and `HttpResponse`, would generate the same function names, as would a
//! type and a function its module already has. Either is an expansion error.
//! So is a field named `__proto__`, which reaches an object's prototype, and
//! a derive on a `declare enum`, which names an enum the module does not hold.
//!
//! A class field without an annotation is read by the type its initializer
//! makes plain: a literal, an assertion, a `new` with type arguments, or the
//! value a `$state` rune is given. An `accessor` field is read like any
//! other, and is an error under a derive that builds an instance (`Clone`,
//! `Default`, `Decode`), which runs no constructor to set it up. A method an
//! interface declares is a member that holds a function.
//!
//! A type a derive reads nests sixteen levels deep at most, counting each
//! bracket open at its deepest point and each `[]` of an array of arrays. A
//! field or an alias that nests deeper is an expansion error: name its inner
//! types with aliases, and derive on those.
//!
//! A type held from another module is found by the name its module exports
//! it under: imported under another name (`import type { User as Member }`),
//! exported under an alias (`export type { User as Member }`) or as the
//! default, it is coded, defaulted and compared through the functions of
//! `User`, by whatever path the module is imported. A type named through a
//! namespace import (`models.User`) is an expansion error under `Encode`
//! and `Decode`: import it by name.
//!
//! ## Type-Position Macros
//!
//! | Macro | Expands To | Description |
//! |-------|------------|-------------|
//! | `$Newtype<T>` | `T & { readonly [brand]: true }` | Nominal brand backed by a per-site `unique symbol` |
//!
//! A `$Newtype` over a primitive gets checked `decode` / `is` from `Decode`,
//! including validators written on the alias itself; see [`newtype`].
//!
//! ## Field-Level Decorators
//!
//! Most macros support field-level decorators to customize behavior:
//!
//! ```typescript
//! /** @derive(Debug, PartialEq, Encode) */
//! class User {
//!     /** @debug({ rename: "identifier" }) */
//!     id: number;
//!
//!     /** @partialEq({ skip: true }) @endec({ skipEncoding: true }) */
//!     password: string;
//!
//!     /** @endec({ rename: "emailAddress" }) */
//!     email: string;
//! }
//! ```
//!
//! A decorator's arguments are read as written: an option it does not take,
//! a value of the wrong kind (`skip: "no"`, `rename: 5`) and an option given
//! twice are expansion errors, and text in a string is only text, whatever
//! brackets, commas or option names it holds.
//!
//! ## Where `@derive` Goes
//!
//! An `@derive` stands in a JSDoc directly above a class, an interface, an
//! enum or a type alias at the top level of its module, with only other
//! JSDoc comments in between. A declaration may spread its derives over
//! several directives, which add up. One that stands above anything else, a
//! `const`, a namespace, a member, a line comment or a type declared inside
//! a namespace or a function, is an expansion error, as it would generate
//! nothing.
//! So is a derive on a class, an interface or an enum its module declares
//! more than once: TypeScript merges the declarations, and a derive reads
//! only the one it is written on.
//!
//! A class or an interface that is its module's default export is derived
//! like any other, given a name: a class without one has none for its
//! functions to take. The code a derive generates is TypeScript with
//! bodies, so a derive in a declaration file (`.d.ts`), in a JavaScript
//! file, or on a `declare class` is an expansion error.
//!
//! ## Example Usage
//!
//! ```typescript
//! /** @derive(Debug, Clone, PartialEq, Hash) */
//! class Point {
//!     x: number;
//!     y: number;
//!
//!     constructor(x: number, y: number) {
//!         this.x = x;
//!         this.y = y;
//!     }
//! }
//! ```

/// The derive macros: equality, ordering, hashing, cloning, debugging,
/// defaults and encoding.
pub mod derive;

/// `$Newtype<T>` type-position macro (nominal symbol brands).
pub mod newtype;

/// Return type code generation helpers for Decode and PartialOrd macros.
pub mod return_types;
