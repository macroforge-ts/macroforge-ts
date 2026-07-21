//! # Serde (Serialization/Deserialization) Module
//!
//! This module provides the `Serialize` and `Deserialize` macros for JSON
//! serialization with cycle detection and validation support.
//!
//! ## Generated Methods
//!
//! For classes, methods are generated as `static` members plus standalone functions
//! (`{name}Serialize`, `{name}Deserialize`, ...) that delegate to them; interfaces and
//! type aliases get the standalone functions only.
//!
//! ### Serialize
//!
//! - `static serialize(value, keepMetadata?): string` - Serialize to JSON string;
//!   `__type`/`__id` metadata is stripped unless `keepMetadata` is true
//! - `static serializeWithContext(value, ctx): Record<string, unknown>` - Internal method with cycle detection
//!
//! ### Deserialize
//!
//! - `static deserialize(input: unknown, opts?: { freeze?: boolean }):
//!   { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
//!   Parse and validate (auto-detects string vs object); never throws
//! - `static deserializeWithContext(value, ctx): T | PendingRef` - Internal method with cycle resolution
//! - `static hasShape(obj): boolean` - Structural check that all required JSON keys are present
//! - `static is(value): value is T` - Type guard (instanceof, shape check, then a full deserialize)
//! - `static validateField(field, value)` / `static validateFields(partial)` - Run field validators standalone
//! - Classes also get a synthesized `constructor(props)` that assigns all deserialized fields
//! - **Enums** get standalone functions only (`{name}Deserialize`, `{name}Is`); the enum
//!   `deserialize` function **throws** on invalid values instead of returning a result union
//!
//! ## Cycle Detection
//!
//! Both macros support cycle detection for object graphs with circular references:
//!
//! ```typescript
//! /** @derive(Serialize, Deserialize) */
//! class Node {
//!     value: number;
//!     next: Node | null;
//! }
//! // Creates: { __type: "Node", __id: 1, value: 1, next: { __ref: 1 } }
//! ```
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports many options:
//!
//! | Option | Description |
//! |--------|-------------|
//! | `skip` | Skip both serialization and deserialization |
//! | `skipSerializing` | Skip only during serialization |
//! | `skipDeserializing` | Skip only during deserialization |
//! | `rename = "name"` | Use a different JSON key |
//! | `default` | Use type's default if missing |
//! | `default = "expr"` | Use specific expression if missing |
//! | `flatten` | Flatten nested object fields into parent |
//! | `serializeWith = "fn"` | Use custom function for serialization |
//! | `deserializeWith = "fn"` | Use custom function for deserialization |
//! | `format = "decimal"` | Serialize a number field as a string |
//! | `validate: [...]` | Run validators during deserialization (see below) |
//!
//! ## Container-Level Options
//!
//! | Option | Description |
//! |--------|-------------|
//! | `renameAll = "camelCase"` | Apply naming convention to all fields |
//! | `denyUnknownFields` | Reject JSON with extra fields |
//! | `tag = "fieldName"` | Custom type discriminator field name (default: `"__type"`) |
//! | `content = "fieldName"` | With `tag`, selects adjacently-tagged union representation |
//! | `untagged` | Untagged union representation (raw value, no discriminator) |
//! | `externallyTagged` | Externally-tagged union representation (`{ "TypeName": {...} }`) |
//!
//! The tagging options combine into four modes mirroring Rust serde: internally tagged
//! (the default, `tag` only), adjacently tagged (`tag` + `content`), externally tagged,
//! and untagged.
//!
//! ## Naming Conventions
//!
//! Supported values for `renameAll`:
//! - `camelCase` - `user_name` → `userName`
//! - `snake_case` - `userName` → `user_name`
//! - `SCREAMING_SNAKE_CASE` - `userName` → `USER_NAME`
//! - `kebab-case` - `userName` → `user-name`
//! - `PascalCase` - `user_name` → `UserName`
//!
//! ## Validation
//!
//! The Deserialize macro supports 40+ validators for runtime validation:
//!
//! ### String Validators
//! - `email` - Valid email format
//! - `url` - Valid URL format
//! - `uuid` - Valid UUID format
//! - `pattern("regex")` - Match regex pattern
//! - `minLength(n)`, `maxLength(n)`, `length(n)`, `length(min, max)` - Length constraints
//! - `nonEmpty`, `trimmed` - Content requirements
//! - `lowercase`, `uppercase`, `capitalized`, `uncapitalized` - Case requirements
//! - `startsWith("prefix")`, `endsWith("suffix")`, `includes("text")`
//!
//! ### Number Validators
//! - `int`, `nonNegativeInt` - Must be an integer (optionally also >= 0)
//! - `positive`, `negative`, `nonNegative`, `nonPositive`
//! - `greaterThan(n)`, `greaterThanOrEqualTo(n)`, `lessThan(n)`, `lessThanOrEqualTo(n)`, `between(min, max)`
//! - `multipleOf(n)`, `uint8`, `finite`, `nonNaN`
//!
//! ### BigInt Validators
//! - `positiveBigInt`, `negativeBigInt`, `nonNegativeBigInt`, `nonPositiveBigInt`
//! - `greaterThanBigInt(n)`, `greaterThanOrEqualToBigInt(n)`, `lessThanBigInt(n)`,
//!   `lessThanOrEqualToBigInt(n)`, `betweenBigInt(min, max)`
//!
//! ### Array Validators
//! - `minItems(n)`, `maxItems(n)`, `itemsCount(n)`
//!
//! ### Date Validators
//! - `validDate` - Must be valid Date
//! - `greaterThanDate("2020-01-01")`, `greaterThanOrEqualToDate("2020-01-01")`
//! - `lessThanDate("2030-01-01")`, `lessThanOrEqualToDate("2030-01-01")`
//! - `betweenDate("start", "end")`
//!
//! ### Custom Validators
//! - `custom(functionName)` - Call custom validation function
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Serialize, Deserialize) @serde({ renameAll: "camelCase" }) */
//! class User {
//!     /** @serde({ validate: { email: true } }) */
//!     emailAddress: string;
//!
//!     /** @serde({ validate: { minLength: 3, maxLength: 50 } }) */
//!     username: string;
//!
//!     /** @serde({ skipSerializing: true }) */
//!     password: string;
//!
//!     /** @serde({ default: true }) */
//!     role: string;
//!
//!     /** @serde({ flatten: true }) */
//!     metadata: UserMetadata;
//! }
//! ```
//!
//! ## Custom Serialization Functions
//!
//! For foreign types that can't be automatically serialized, use custom functions:
//!
//! ```typescript
//! import { ZonedDateTime } from "@internationalized/date";
//!
//! // Custom serializer/deserializer functions
//! function serializeZoned(value: ZonedDateTime): unknown {
//!     return value.toAbsoluteString();
//! }
//! function deserializeZoned(raw: unknown): ZonedDateTime {
//!     return parseZonedDateTime(raw as string);
//! }
//!
//! /** @derive(Serialize, Deserialize) */
//! interface Event {
//!     name: string;
//!     /** @serde({ serializeWith: "serializeZoned", deserializeWith: "deserializeZoned" }) */
//!     startTime: ZonedDateTime;
//! }
//! ```
//!
//! ## Foreign Types (Global Configuration)
//!
//! Instead of adding `serializeWith`/`deserializeWith` to every field, you can configure
//! foreign type handlers globally in `macroforge.config.js`:
//!
//! ```javascript
//! // macroforge.config.js
//! import { DateTime } from "effect";
//!
//! export default {
//!   foreignTypes: {
//!     "DateTime.DateTime": {
//!       from: ["effect"],
//!       aliases: [
//!         { name: "DateTime", from: "effect/DateTime" }
//!       ],
//!       serialize: (v) => DateTime.formatIso(v),
//!       deserialize: (raw) => DateTime.unsafeFromDate(new Date(raw)),
//!       default: () => DateTime.unsafeNow()
//!     }
//!   }
//! }
//! ```
//!
//! With this configuration, any field typed as `DateTime.DateTime` (imported from `effect`)
//! or `DateTime` (imported from `effect/DateTime`) will automatically use the configured
//! handlers without needing per-field decorators.
//!
//! ### Foreign Type Options
//!
//! | Option | Description |
//! |--------|-------------|
//! | `from` | Array of module paths this type can be imported from |
//! | `aliases` | Array of `{ name, from }` objects for alternative type-package pairs |
//! | `serialize` | Function `(value) => unknown` for serialization |
//! | `deserialize` | Function `(raw) => T` for deserialization |
//! | `default` | Function `() => T` for default value generation |
//!
//! ### Import Source Validation
//!
//! Foreign types are only matched when the type is imported from one of the configured
//! sources. Types with the same name from different packages are ignored and fall back
//! to generic handling (TypeScript will catch any issues downstream).
//!
//! See the [Configuration](crate::host::config) module for more details.

/// Deserialize macro implementation.
pub mod derive_deserialize;

/// Serialize macro implementation.
pub mod derive_serialize;

mod foreign_types;
mod helpers;
mod options;
mod type_category;
mod validators;

#[cfg(test)]
mod tests;

// Re-export the registry type and thread-local accessors from their canonical location.
pub use crate::host::import_registry::{
    ImportRegistry, clear_registry, install_registry, take_registry,
};

// Re-export submodule items so existing consumers (`super::*`, `crate::builtin::serde::*`) keep working.
pub use foreign_types::{ForeignTypeMatch, get_foreign_types, rewrite_expression_namespaces};
pub use helpers::{extract_named_string, has_flag};
pub(crate) use helpers::{find_top_level_comma, split_top_level_union};
pub use options::{
    RenameAll, SerdeContainerOptions, SerdeFieldOptions, SerdeFieldParseResult, TaggingMode,
};
pub use type_category::TypeCategory;
pub use validators::{Validator, ValidatorSpec, extract_validators};
