//! # Clone Macro Implementation
//!
//! The `Clone` macro generates a `clone()` method for deep copying objects.
//! This is analogous to Rust's `Clone` trait, providing a way to create
//! independent copies of values.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `{className}Clone(value)` + `static clone(value)` | Standalone function + static wrapper method |
//! | Enum | `{enumName}Clone(value): EnumName` | Standalone function (enums are primitives, returns value as-is) |
//! | Interface | `{ifaceName}Clone(value): InterfaceName` | Standalone function creating a new object literal |
//! | Type Alias | `{typeName}Clone(value): TypeName` | Standalone function cloning by the alias's shape |
//!
//! Names use **camelCase** conversion (e.g., `Point` -> `pointClone`).
//!
//!
//! ## Cloning Strategy
//!
//! The generated clone dispatches on each field's declared type, and a clone
//! always equals its source under the derived `PartialEq`:
//!
//! - **Primitives** (`string`, `number`, `boolean`, `bigint`): Copied by value
//! - **`Date`, `RegExp`, `URL`, typed arrays**: Copied by value
//! - **Arrays, `Map`, `Set`**: Rebuilt, each element cloned by its own type
//!   (a map's keys are shared, since lookups match them by identity)
//! - **Objects with `@derive(Clone)`**: Their standalone clone function
//! - **Optional fields**: `null`/`undefined` pass through, and an absent
//!   optional field stays absent
//! - **Anything else** (unions, object literals, unresolved types):
//!   `structuralClone` from `@macroforge/core/structural`
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Clone) */
//! class Point {
//!     x: number;
//!     y: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { instanceWith as __mf_instanceWith } from "@macroforge/core/structural";
//!
//! class Point {
//!     x: number;
//!     y: number;
//!
//!     static clone(value: Point): Point {
//!         return pointClone(value);
//!     }
//! }
//!
//! export function pointClone(value: Point): Point {
//!     return __mf_instanceWith(value, { x: value.x, y: value.y });
//! }
//! ```
//!
//! ## Implementation Notes
//!
//! - **Classes**: `instanceWith` from `@macroforge/core/structural` copies the
//!   fields onto a new object with the value's prototype, so `instanceof`
//!   checks work and the constructor does not run
//! - **Enums**: Simply returns the value (enums are primitives in TypeScript)
//! - **Interfaces/Type Aliases**: An object literal copying each field for
//!   object types, element by element for fixed tuples, and
//!   `structuralClone` for unions and other shapes

mod clone_generation;
mod core;

pub use core::derive_clone_macro;
