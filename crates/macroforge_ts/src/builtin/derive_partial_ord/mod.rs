//! # PartialOrd Macro Implementation
//!
//! The `PartialOrd` macro generates a `compareTo()` method for **partial ordering**
//! comparison. This is analogous to Rust's `PartialOrd` trait, enabling comparison
//! between values where some pairs may be incomparable.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `{className}PartialCompare(a, b)` + `static compareTo(a, b)` | Standalone function + static wrapper method |
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
//! - **-1**: `a` is less than `b`
//! - **0**: `a` is equal to `b`
//! - **1**: `a` is greater than `b`
//! - **null**: Values are incomparable
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
//! | `number`/`bigint` | Direct subtraction (`a - b`) |
//! | `string` | `localeCompare()` |
//! | `boolean` | `false < true` (cast to number) |
//! | null/undefined | Returns `null` for mismatched nullability |
//! | Arrays | Lexicographic, propagates `null` on incomparable elements |
//! | `Date` | Timestamp comparison, `null` if invalid |
//! | Objects | Delegates to `compareTo()` if available |
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
//! /** @derive(PartialOrd) */
//! class Temperature {
//!     value: number | null;
//!     unit: string;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Temperature {
//!     value: number | null;
//!     unit: string;
//!
//!     static compareTo(a: Temperature, b: Temperature): number | null {
//!         return temperaturePartialCompare(a, b);
//!     }
//! }
//!
//! export function temperaturePartialCompare(a: Temperature, b: Temperature): number | null {
//!     if (a === b) return 0;
//!     const cmp0 = (() => {
//!         if (typeof (a.value as any)?.compareTo === 'function') {
//!             const optResult = (a.value as any).compareTo(b.value);
//!             return optResult === null ? null : optResult;
//!         }
//!         return a.value === b.value ? 0 : null;
//!     })();
//!     if (cmp0 === null) return null;
//!     if (cmp0 !== 0) return cmp0;
//!     const cmp1 = a.unit.localeCompare(b.unit);
//!     if (cmp1 === null) return null;
//!     if (cmp1 !== 0) return cmp1;
//!     return 0;
//! }
//! ```
//!
//! ## Return Type
//!
//! The generated functions return `number | null` where `null` indicates incomparable values.

pub(crate) mod comparison;
mod core;
#[cfg(test)]
mod tests;
pub(crate) mod types;
