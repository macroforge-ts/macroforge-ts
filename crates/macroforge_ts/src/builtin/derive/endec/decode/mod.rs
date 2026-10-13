//! # Decode Macro Implementation
//!
//! The `Decode` macro generates JSON decoding methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Output
//!
//! For **classes** (`class_handler`), the macro generates static methods plus
//! standalone functions (`{name}Decode`, `{name}DecodeWithContext`,
//! `{name}Is`) that delegate to them:
//!
//! - `static decode(input: unknown, opts?: DecodeOptions):
//!   { success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }` -
//!   Auto-detects JSON string vs object; never throws. `opts.freeze` freezes all
//!   decoded objects, arrays, tuples and records; a `Map` and a `Set` have no
//!   frozen form.
//! - `static decodeWithContext(value, ctx): T | PendingRef` - Internal method used
//!   for nested/cyclic graphs; throws `DecodeError` on structural errors
//! - `static hasShape(obj): boolean` - Checks all required JSON keys are present
//! - `static is(value): value is T` - Type guard: instanceof check, then `hasShape`,
//!   then a full `decode`
//! - `static validateField(field, value)` / `static validateFields(partial)` - Run the
//!   field validators on input that has not been decoded, typed `unknown`
//! - A synthesized `constructor(props)` that assigns the fields, when the class
//!   has no constructor of its own. A subclass's hands them to its superclass's
//!   when that derives `Decode`
//!
//! `decode` takes JSON text or a value already parsed from it. Where the type
//! itself takes a string, as an alias of `string` or of `Date`, an enum of
//! strings and a union with a string or a string-literal member do, a string
//! is the JSON text of a value when it spells one the type takes, and the
//! value itself otherwise. So `decode(encode(value))` reads back what was
//! encoded, and `decode("small")` is still the member `small`.
//!
//! Decoding builds an instance without running its constructor, so a class
//! keeps whatever constructor it has. Its `private`, `protected` and `readonly`
//! fields, its constructor parameter properties and the fields of its
//! superclasses are decoded like any other.
//!
//! **Interfaces** and **type aliases** get the standalone-function forms of the same
//! surface (there is no class to attach statics to), except that only an object-shaped
//! alias has fields to validate, so no other alias gets `validateField`/`validateFields`.
//! **Enums** (`enum_handler`) get
//! `{name}Decode`/`{name}DecodeWithContext`/`{name}Is`; unlike the other
//! shapes, the enum `decode` function **throws** a `DecodeError` on invalid
//! values rather than returning a result union.
//!
//! A **type alias of a primitive**, plain (`type Port = number`) or branded
//! (`type Meters = $Newtype<number>`), decodes to that primitive: `decode` rejects
//! any other `typeof`, `hasShape` checks the `typeof`, and `is` additionally runs
//! the alias's validators. Validators go on the alias itself:
//!
//! ```typescript
//! /** @derive(Decode) */
//! /** @endec(nonNegative, finite) */
//! type Meters = $Newtype<number>;
//!
//! metersDecode(-1);   // { success: false, errors: [{ field: "_root", ... }] }
//! metersIs("12");     // false
//! ```
//!
//! Alias-level validators on any other alias shape are an expansion error; put
//! them on the fields instead.
//!
//! A class, an interface and an object-shaped alias also get
//! `{name}DecodePartialWithContext(value, ctx, fields?)`, which a field typed
//! `Partial<T>`, `Pick<T, K>` or `Omit<T, K>` decodes through. `fields` is the
//! shape: `{ only: [...] }` for a `Pick`, `{ except: [...] }` for an `Omit`,
//! and left out for a `Partial`. A field the shape lacks is not read, even
//! when the input has it; one it keeps is required as `T` requires it, except
//! under `Partial`, where none is. Such a value is not registered for
//! references and may not be one, since a reference stands for a whole `T`.
//! The keys of a `Pick` or `Omit` have to be string literals.
//!
//! ## Field Types
//!
//! A field is decoded by the whole structure of its type, to any depth, and
//! every member is checked on the way:
//!
//! | Type | Decoding |
//! |------|----------|
//! | `string`, `number`, `boolean`, `bigint`, literals and their unions | The value, when it is one of the type's |
//! | `Date` | A `Date` from its ISO string |
//! | Arrays, `Set<T>` | From an array, each element decoded by the element type |
//! | `Map<K,V>` | From an object, each value decoded by `V` and each key read back as `K`: a number, bigint, boolean, `Date`, enum or newtype from its text |
//! | `Record<K,V>` | From an object, each value decoded by `V` and each key checked to be a `K`. Over a finite set of keys, `Record<'a' \| 'b', V>` or an enum, every key has to be there |
//! | Tuples | From an array of the tuple's length, each element decoded by its own type |
//! | An object type written in place | An object of exactly its members, each decoded by its own type |
//! | `Partial<T>`, `Pick<T,K>`, `Omit<T,K>` | `{name}DecodePartialWithContext` |
//! | `Required<T>` | As `T`, with the fields `T` leaves optional required too, unless a default fills them |
//! | A type deriving `Decode` | Its `{name}DecodeWithContext` function |
//! | A configured foreign type | Its `decode` handler |
//! | A type parameter | The decoder handed in for it, or the value as it is, unchecked, without one |
//! | `string \| User`, a primitive beside one type deriving `Decode` | The primitive as it is, anything else through the type |
//! | Any other union with a member that is decoded, `Circle \| Square`, `Date \| number` | An expansion error: nothing in a field says which member a value is. Name the union with an alias and derive on that |
//! | A union of values that are their own encoding, `string \| string[]`, `Date \| string` | The value as it is, checked to be one of the members |
//! | `object` | The value, when it is an object |
//! | A template literal type, `` `id-${number}` `` | The text, when it spells the template: its fixed text in place, a `${number}` or `${bigint}` hole a number in decimal, a hole of literals one of them. A hole of any other type is an expansion error |
//! | `keyof T`, `T['field']` over a type the project declares | As what the operator yields |
//! | A function, a symbol, `void`, `never`, a mapped, conditional or `typeof` type | An expansion error: code the field with `decodeWith`, or leave it out with `skip` |
//! | A type named through a namespace import, `models.User` | An expansion error: import the type by name |
//! | `unknown`, `any`, a type the project does not declare | The value as it is, unchecked |
//!
//! A `null` in an array, a tuple or a `Set`, or as the value of a `Record`
//! or `Map` entry, is read as `undefined` where the type there takes
//! `undefined` and not `null`: it is what `encode` writes for one, as JSON
//! has no `undefined`.
//!
//! A number has to be finite. `NaN`, an infinity, and JSON text that spells a
//! number too large to hold (`1e999`) are refused: `encode` could not write
//! them back. Only the members an input holds itself are read, so one it
//! inherits is not there.
//!
//! A `$Newtype<T>` is decoded as the `T` it brands, whatever kind of type
//! that is, and a union alias tells a template literal member by its text.
//!
//! A member that fails is reported under its path: `grid[0][1]`,
//! `byName.key.id`. Members share the context of the value that holds them, so
//! a value held in several places, or a child pointing back at its parent
//! through a collection, decodes to one object.
//!
//! ## Generic Types
//!
//! `{name}DecodeWithContext` and `{name}DecodePartialWithContext` of a generic
//! type take one optional `Decoder` for each type parameter, after their
//! other parameters, and a field written with the type's arguments hands in a
//! decoder made from each: `Box<Date>` decodes the dates its `T` holds, and
//! `Page<User>` its users, checked and built like any other. A union alias
//! hands the decoders on to a member written with type arguments. The
//! top-level `{name}Decode` and a class's static `decode` take the same
//! decoders after their options, `boxDecode(text, undefined, decodeDate)`.
//! Without a decoder a value of the parameter is kept as it is. A type
//! argument a field leaves to its default, `Box` for `Box<T = Date>`, is
//! decoded and checked as that default.
//!
//! ## Reserved Keys
//!
//! `__id` and `__ref` mark a value held in more than one place, and the tag
//! key (`__type`, or the container's `tag`) holds the type's name. A field
//! read from or written under one of them is an expansion error: give it
//! another key with `@endec({ rename: "..." })`. So are two fields under one
//! key, whether by `rename`, `renameAll` or a flattened type's own fields,
//! and two flattened `Record`s, each of which holds every key nothing else
//! reads.
//!
//! ## References
//!
//! A `{ "__ref": n }` stands for the object registered as `n`, and has to be
//! read as the type that object was decoded as: input cannot stand an object
//! of one type in for another. A value decoded by `{name}DecodePartialWithContext`
//! or flattened into its parent is not registered at all.
//!
//! Validation (see `validation`) runs during decoding and reports failures
//! through the `errors` array of the result union. Field-level options are parsed by
//! `field`; see the parent endec module for the option and validator
//! reference.

mod class_handler;
mod enum_handler;
mod field;
pub(crate) mod helpers;
mod interface_handler;
mod object_decoder;
mod reference;
mod type_alias;
pub(crate) mod types;
pub(crate) mod validation;

#[cfg(test)]
mod tests;

use crate::macros::ts_macro_derive;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(
    Decode,
    description = "Generates decoding methods with cycle/forward-reference support (decode, decodeWithContext)",
    attributes((endec, "Configure decoding for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_decode_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => class_handler::handle_class(&input, class),
        Data::Enum(enum_data) => enum_handler::handle_enum(&input, enum_data),
        Data::Interface(interface) => interface_handler::handle_interface(&input, interface),
        Data::TypeAlias(type_alias) => type_alias::handle_type_alias(&input, type_alias),
    }
}
