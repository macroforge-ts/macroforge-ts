//! # Encode Macro Implementation
//!
//! The `Encode` macro generates JSON encoding methods with **cycle detection**
//! and object identity tracking. This enables encoding of complex object graphs
//! including circular references.
//!
//! ## Generated Methods
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameEncode(value, keepMetadata?)` + `static encode(value, keepMetadata?)` | Standalone function + static wrapper method |
//! | Enum | `enumNameEncode(value)`, `enumNameEncodeWithContext` | Standalone functions |
//! | Interface | `interfaceNameEncode(value)`, etc. | Standalone functions |
//! | Type Alias | `typeNameEncode(value)`, etc. | Standalone functions |
//!
//! ## Cycle Detection Protocol
//!
//! The generated code handles circular references using `__id` and `__ref` markers:
//!
//! ```json
//! {
//!     "__type": "User",
//!     "__id": 1,
//!     "name": "Alice",
//!     "friend": { "__ref": 2 }  // Reference to object with __id: 2
//! }
//! ```
//!
//! When an object is encoded:
//! 1. Check if it's already been encoded (has an `__id`)
//! 2. If so, return `{ "__ref": existingId }` instead
//! 3. Otherwise, register the object and encode its fields
//!
//! ## Type-Specific Encoding
//!
//! | Type | Encoding Strategy |
//! |------|------------------------|
//! | Primitives | Direct value |
//! | `Date` | `toISOString()` |
//! | Arrays | For primitive-like element types, pass through; for `Date`/`Date | null`, map to ISO strings; for encodable element types, map through the element's `{name}EncodeWithContext` function |
//! | `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date | null`, convert to ISO strings; for encodable values, call `{name}EncodeWithContext` per value |
//! | `Set<T>` | Convert to array; element handling matches `Array<T>` |
//! | `Record<K,V>` | For primitive-like values, pass through; for `Date`/`Date | null`, convert values to ISO strings; for encodable values, rebuild via `Object.fromEntries` calling `{name}EncodeWithContext` per value |
//! | Wrappers (`Partial<T>`, `Pick<T,K>`, ...) | Encode based on the inner type `T` |
//! | Nullable | Include `null` explicitly; non-null values follow the inner type's strategy |
//! | Objects | Call the type's `{name}EncodeWithContext` function |
//!
//! Note: which strategy applies is resolved **statically** from the field's declared
//! TypeScript type at expansion time: there is no runtime feature detection.
//!
//! ## Field-Level Options
//!
//! The `@endec` decorator supports:
//!
//! - `skip` / `skipEncoding` - Exclude field from encoding
//! - `rename = "jsonKey"` - Use different JSON property name
//! - `flatten` - Merge nested object's fields into parent
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Encode) */
//! class User {
//!     id: number;
//!
//!     /** @endec({ rename: "userName" }) */
//!     name: string;
//!
//!     /** @endec({ skipEncoding: true }) */
//!     password: string;
//!
//!     /** @endec({ flatten: true }) */
//!     metadata: UserMetadata;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { EncodeContext } from '@macroforge/core/endec';
//!
//! class User {
//!     id: number;
//!
//!     name: string;
//!
//!     password: string;
//!
//!     metadata: UserMetadata;
//!     /** Encodes a value to a JSON string.
//! @param value - The value to encode
//! @returns JSON string representation with cycle detection metadata  */
//!
//!     static encode(value: User, keepMetadata?: boolean): string {
//!         return userEncode(value, keepMetadata);
//!     }
//!     /** @internal Encodes with an existing context for nested/cyclic object graphs.
//! @param value - The value to encode
//! @param ctx - The encoding context  */
//!
//!     static encodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
//!         return userEncodeWithContext(value, ctx);
//!     }
//! }
//!
//! /** Encodes a value to a JSON string.
//! @param value - The value to encode
//! @returns JSON string representation with cycle detection metadata */ export function userEncode(
//!     value: User,
//!     keepMetadata?: boolean
//! ): string {
//!     const ctx = __mf_EncodeContext.create();
//!     const __raw = userEncodeWithContext(value, ctx);
//!     if (keepMetadata) return JSON.stringify(__raw);
//!     return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
//! } /** @internal Encodes with an existing context for nested/cyclic object graphs.
//! @param value - The value to encode
//! @param ctx - The encoding context */
//! export function userEncodeWithContext(
//!     value: User,
//!     ctx: EncodeContext
//! ): Record<string, unknown> {
//!     const existingId = ctx.getId(value);
//!     if (existingId !== undefined) {
//!         return { __ref: existingId };
//!     }
//!     const __id = ctx.register(value);
//!     const result: Record<string, unknown> = { __type: 'User', __id };
//!     result['id'] = value.id;
//!     result['userName'] = value.name;
//!     {
//!         const __flattened = userMetadataEncodeWithContext(value.metadata, ctx);
//!         const { __type: _, __id: __, ...rest } = __flattened as any; // tag field name is configurable via `tag` option
//!         Object.assign(result, rest);
//!     }
//!     return result;
//! }
//! ```
//!
//! ## Required Import
//!
//! The generated code automatically imports `EncodeContext` from `@macroforge/core/endec`.

mod class_handler;
mod enum_handler;
mod field;
mod foreign_types;
mod interface_handler;
mod object_encoder;
mod type_alias;
mod types;

use crate::macros::ts_macro_derive;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    Encode,
    description = "Generates encoding methods with cycle detection (encode, encodeWithContext)",
    attributes((endec, "Configure encoding for this field. Options: skip, rename, flatten"))
)]
pub fn derive_encode_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Class(class) => class_handler::handle_class(&input, class),
        Data::Enum(_) => enum_handler::handle_enum(&input),
        Data::Interface(interface) => interface_handler::handle_interface(&input, interface),
        Data::TypeAlias(type_alias) => type_alias::handle_type_alias(&input, type_alias),
    }
}
