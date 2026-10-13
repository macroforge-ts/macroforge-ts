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
//! | Class | `static encode(value, keepMetadata?)`, `static encodeWithContext`, `static encodePartialWithContext` + `classNameEncode(value, keepMetadata?)` etc. | Static methods + standalone functions |
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
//! `encode(value)` leaves `__type` and `__id` out of its text unless
//! `keepMetadata` is set. Nothing is then left for a reference to name, so a
//! value held in several places is written out at each of them, and a value
//! that holds itself, which only a reference can write, throws a `TypeError`.
//!
//! ## Type-Specific Encoding
//!
//! A field is encoded by the whole structure of its type, to any depth:
//!
//! | Type | Encoding Strategy |
//! |------|------------------------|
//! | Primitives, literals and their unions | The value as it is |
//! | `Date` | `toISOString()` |
//! | Arrays, readonly or not | An array of its elements, each encoded by the element type; left as it is when the elements are |
//! | `Set<T>`, `ReadonlySet<T>` | An array of its elements, each encoded by `T` |
//! | `Map<K,V>`, `ReadonlyMap<K,V>` | An object of its entries, each value encoded by `V`. A key is its text: a string, number, bigint, boolean, an enum or newtype of one, or a `Date`, written as its ISO string. Any other key type is an expansion error |
//! | `Record<K,V>` | An object of its members, each value encoded by `V` |
//! | Tuples | An array of its elements, each encoded by its own type, optional and rest elements included |
//! | An object type written in place, `{ at: Date }` | The object, each member encoded by its own type |
//! | `T \| null`, `T \| undefined` | `null` and `undefined` as they are, anything else as `T` |
//! | `Required<T>`, `Readonly<T>`, `NonNullable<T>` | As the inner type `T` |
//! | `Partial<T>`, `Pick<T,K>`, `Omit<T,K>` | `{name}EncodePartialWithContext`, which encodes the fields the value has and, for a `Pick` or `Omit`, only those the type keeps. Such a value is written out wherever it is held and never becomes a `__ref`; one that holds itself throws a `TypeError` |
//! | A type deriving `Encode` | Its `{name}EncodeWithContext` function |
//! | A configured foreign type | Its `encode` handler |
//! | A type parameter | The encoder handed in for it, or the value as it is without one |
//! | A type named through a namespace import, `models.User` | An expansion error: import the type by name |
//! | `string \| User`, a primitive beside one type deriving `Encode` | The primitive as it is, anything else through the type |
//! | Any other union with a member that is encoded, `Circle \| Square`, `Date \| number` | An expansion error: nothing in a field says which member a value is. Name the union with an alias and derive on that |
//! | `keyof T`, `T['field']` over a type the project declares | As what the operator yields |
//! | A function, a symbol, `void`, `never`, a mapped, conditional or `typeof` type | An expansion error: code the field with `encodeWith`, or leave it out with `skip` |
//! | `unknown`, `any`, a type the project does not declare | The value as it is |
//!
//! A number has to be finite: `encode` throws a `TypeError` for `NaN` and the
//! infinities, which JSON would write as `null`. A class field whose
//! declaration leaves its type unsaid (`items = []`) is an expansion error:
//! annotate it, or code it with `encodeWith`.
//!
//! JSON has no `undefined`. A field that holds one is left out. In an array,
//! a tuple or a `Set`, and as the value of a `Record` or `Map` entry, one is
//! written as `null`, which `decode` reads back as `undefined` where the type
//! there takes `undefined` and not `null`.
//!
//! Note: which strategy applies is resolved **statically** from the field's declared
//! TypeScript type at expansion time: there is no runtime feature detection.
//!
//! ## Generic Types
//!
//! A type parameter says nothing of how its values are encoded, so
//! `{name}EncodeWithContext` and `{name}EncodePartialWithContext` of a generic
//! type take one optional `Encoder` for each parameter, after their other
//! parameters. A field written with the type's arguments hands them in:
//!
//! ```typescript
//! /** @derive(Encode) */
//! interface Box<T> { held: T; list: T[] }
//!
//! /** @derive(Encode) */
//! interface Shelf { dated: Box<Date> }
//! // shelfEncodeWithContext calls
//! // boxEncodeWithContext<Date>(value.dated, ctx, (date) => date.toISOString())
//! ```
//!
//! A union alias hands the encoders on to a member written with type
//! arguments. The top-level `{name}Encode` and a class's static `encode` take
//! the same encoders after `keepMetadata`,
//! `boxEncode(box, false, (date) => date.toISOString())`. Without an encoder
//! a value of the parameter is written as it is. A type argument a field
//! leaves to its default, `Box` for `Box<T = Date>`, is encoded as that
//! default.
//!
//! ## Field-Level Options
//!
//! The `@endec` decorator supports:
//!
//! - `skip` / `skipEncoding` - Exclude field from encoding
//! - `rename = "jsonKey"` - Use different JSON property name
//! - `flatten` - Merge nested object's fields into parent. The flattened
//!   value is written out by its fields, with no `__id` of its own
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
//! import {
//!     EncodeContext as __mf_EncodeContext,
//!     encodedText as __mf_encodedText,
//!     encodedTextWithoutMetadata as __mf_encodedTextWithoutMetadata,
//!     encodeSelection as __mf_encodeSelection,
//!     fieldRequirement as __mf_fieldRequirement
//! } from '@macroforge/core/endec';
//!
//! class User {
//!     id: number;
//!     name: string;
//!     password: string;
//!     metadata: UserMetadata;
//!
//!     static encode(value: User, keepMetadata?: boolean): string {
//!         return userEncode(value, keepMetadata);
//!     }
//!
//!     static encodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
//!         const existingId = ctx.getId(value);
//!         if (existingId !== undefined) {
//!             return { __ref: existingId };
//!         }
//!         const result: Record<string, unknown> = { __type: 'User', __id: ctx.register(value) };
//!         User.#encodeFields(value, ctx, result, undefined);
//!         return result;
//!     }
//!
//!     static encodePartialWithContext(value: Partial<User>, ctx: __mf_EncodeContext, fields?): Record<string, unknown> {
//!         return __mf_encodeSelection(ctx, value, () => {
//!             const result: Record<string, unknown> = { __type: 'User' };
//!             User.#encodeFields(value as User, ctx, result, fields === undefined ? undefined : __mf_fieldRequirement(fields));
//!             return result;
//!         });
//!     }
//!
//!     // Writes each field the value has and the selection keeps.
//!     static #encodeFields(value: User, ctx: __mf_EncodeContext, result: Record<string, unknown>, __fields): void {
//!         if (__fields === undefined || __fields.keeps('id')) {
//!             if (value.id !== undefined) {
//!                 result["id"] = value.id;
//!             }
//!         }
//!         if (__fields === undefined || __fields.keeps('name')) {
//!             if (value.name !== undefined) {
//!                 result["userName"] = value.name;
//!             }
//!         }
//!         if (__fields === undefined || __fields.keeps('metadata')) {
//!             if (value.metadata !== undefined) {
//!                 const __flattened = userMetadataEncodePartialWithContext(value.metadata, ctx);
//!                 const { ['__type']: _, __id: __, ...rest } = __flattened as Record<string, unknown>;
//!                 Object.assign(result, rest);
//!             }
//!         }
//!     }
//! }
//!
//! export function userEncode(value: User, keepMetadata?: boolean): string {
//!     const ctx = __mf_EncodeContext.create();
//!     const __raw = userEncodeWithContext(value, ctx);
//!     return keepMetadata ? __mf_encodedText(__raw) : __mf_encodedTextWithoutMetadata(__raw, "__type");
//! }
//!
//! export function userEncodeWithContext(value: User, ctx: __mf_EncodeContext): Record<string, unknown> {
//!     return User.encodeWithContext(value, ctx);
//! }
//!
//! export function userEncodePartialWithContext(value: Partial<User>, ctx: __mf_EncodeContext, fields?): Record<string, unknown> {
//!     return User.encodePartialWithContext(value, ctx, fields);
//! }
//! ```
//!
//! A class's encoders are static methods, where they reach its `private` and
//! `protected` fields, and the standalone functions run them. An interface or a
//! type alias has the same three functions beside it, with a module-level
//! `{name}EncodeFields`. A subclass encodes the fields of its superclasses too.
//!
//! ## Required Imports
//!
//! The generated code automatically imports `EncodeContext` and the helpers
//! that give an encoded value's JSON text from `@macroforge/core/endec`.
//! `encode` throws a `TypeError` for a value JSON cannot represent, such as a
//! top-level `undefined`.

mod class_handler;
mod enum_handler;
mod field;
mod interface_handler;
mod object_encoder;
mod text;
mod type_alias;
pub(crate) mod types;

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
