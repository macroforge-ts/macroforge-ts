//! # PartialOrd Macro Implementation
//!
//! The `PartialOrd` macro generates a `partialCompare()` method for **partial ordering**
//! comparison. This is analogous to Rust's `PartialOrd` trait, enabling comparison
//! between values where some pairs may be incomparable. Like `partial_cmp` and
//! `cmp` in Rust, it sits beside `Ord`'s `compare()` without colliding.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `{className}PartialCompare(a, b)` + `static partialCompare(a, b)` | Standalone function + static wrapper method |
//! | Enum | `{enumName}PartialCompare(a, b): number \| null` | Standalone function returning `number \| null` |
//! | Interface | `{ifaceName}PartialCompare(a, b): number \| null` | Standalone function returning `number \| null` |
//! | Type Alias | `{typeName}PartialCompare(a, b): number \| null` | Standalone function returning `number \| null` |
//!
//! Names use **camelCase** conversion (e.g., `Temperature` → `temperaturePartialCompare`).
//!
//! ## Return Values
//!
//! Unlike `Ord`, `PartialOrd` returns `number | null` to handle incomparable values:
//!
//! - **negative**: `a` is less than `b`
//! - **0**: `a` is equal to `b`
//! - **positive**: `a` is greater than `b`
//! - **null**: Values are incomparable
//!
//! A comparison is `0` exactly when the derived `PartialEq` holds. Strings
//! therefore order by UTF-16 code units, not `localeCompare`, which can call
//! distinct strings equal.
//!
//! ## When to Use PartialOrd vs Ord
//!
//! - **PartialOrd**: When some values may not be comparable
//!   - Example: Floating-point NaN values
//!   - Example: Mixed-type unions
//!   - Example: Type mismatches between objects
//!
//! - **Ord**: When all values are guaranteed comparable (total ordering)
//!
//! ## Comparison Strategy
//!
//! Fields are compared **lexicographically** in declaration order:
//!
//! 1. Compare first field
//! 2. If incomparable, return `null`
//! 3. If not equal, return that result
//! 4. Otherwise, compare next field
//! 5. Continue until a difference is found or all fields are equal
//!
//! ## Type-Specific Comparisons
//!
//! | Type | Comparison Method |
//! |------|-------------------|
//! | `number` | `<` and `>`; `null` when either is `NaN` |
//! | `bigint`, `string` | `<` and `>`, strings by UTF-16 code units |
//! | `boolean` | `false < true` |
//! | Optional and nullable | An absent value orders first |
//! | Arrays | Lexicographic, each element by its own type; `null` on an unordered element |
//! | `Date` | Timestamp; `null` if invalid |
//! | Types deriving `PartialOrd` | Their `partialCompare` function |
//! | Anything else | `structuralCompare` from `@macroforge/core/structural` |
//!
//! ## Requirements
//!
//! `PartialOrd` requires `PartialEq`, as in Rust. Deriving it without
//! `PartialEq` is an expansion error.
//!
//! ## Field-Level Options
//!
//! The `@ord` decorator supports:
//!
//! - `skip` - Exclude the field from ordering comparison
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(PartialEq, PartialOrd) */
//! class Temperature {
//!     value: number;
//!     unit: string;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Temperature {
//!     value: number;
//!     unit: string;
//!
//!     static partialCompare(a: Temperature, b: Temperature): number | null {
//!         return temperaturePartialCompare(a, b);
//!     }
//! }
//!
//! export function temperaturePartialCompare(a: Temperature, b: Temperature): number | null {
//!     if (a === b) return 0;
//!     const cmp0 = a.value < b.value ? -1 : a.value > b.value ? 1 : a.value === b.value ? 0 : null;
//!     if (cmp0 === null) return null;
//!     if (cmp0 !== 0) return cmp0;
//!     const cmp1 = a.unit < b.unit ? -1 : a.unit > b.unit ? 1 : 0;
//!     if (cmp1 === null) return null;
//!     if (cmp1 !== 0) return cmp1;
//!     return 0;
//! }
//! ```
//!
//! ## Return Type
//!
//! The generated functions return `number | null` where `null` indicates incomparable values.

mod core;
#[cfg(test)]
mod tests;
