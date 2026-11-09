//! `{camelName}FromString`: decodes JSON text, as `encode` writes it. A type's
//! `decode` takes a value and never reads a string as text, so text decodes
//! here, the way serde keeps `from_str` apart from building a value.

use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::endec::coder::endec_helper;
use crate::builtin::derive::endec::type_arguments::{declared_param_coders, passed_param_coders};
use crate::builtin::derive::endec::union_guard::Direction;
use crate::builtin::return_types::{DECODE_OPTIONS, decode_return_type};

/// What a `fromString` hands the value its text spells on to.
pub(super) struct TextDecoder<'a> {
    /// The function's name with its type parameters: `boxFromString<T>`.
    pub name: &'a str,
    /// The decoder the value goes to, with its type arguments: `boxDecode<T>`.
    pub decode: &'a str,
    /// The coder parameters `decode` declares after its options.
    pub coders: &'a str,
    /// The same coders as a call hands them on.
    pub passed_coders: &'a str,
    /// What `decode` returns.
    pub return_type: &'a str,
    /// Whether `decode` takes decoding options.
    pub takes_options: bool,
}

/// The standalone `fromString` of a type whose `decode` returns a result.
pub(super) fn from_string_function(decoder: &TextDecoder<'_>) -> TsStream {
    let name = ts_ident!(decoder.name);
    let decode = ts_ident!(decoder.decode);
    let decoded_text = ts_ident!(endec_helper("decodedText"));
    let return_type = ts_ident!(decoder.return_type);
    let coders = decoder.coders;
    let passed_coders = decoder.passed_coders;
    if decoder.takes_options {
        let decode_options = ts_ident!(DECODE_OPTIONS);
        ts_template! {
            /** Decodes JSON text, as `encode` writes it, to this type. @param text - The JSON text @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
            export function @{name}(text: string, opts?: @{decode_options}@{coders}): @{return_type} {
                return @{decoded_text}(text, (value) => @{decode}(value, opts@{passed_coders}));
            }
        }
    } else {
        ts_template! {
            /** Decodes JSON text, as `encode` writes it, to this type. @param text - The JSON text @returns Result containing the decoded value or validation errors */
            export function @{name}(text: string@{coders}): @{return_type} {
                return @{decoded_text}(text, (value) => @{decode}(value@{passed_coders}));
            }
        }
    }
}

/// The `fromString` of a type named by `names`, whose `decode` takes options.
pub(super) fn from_string_of(names: &TypeNames) -> TsStream {
    let name = format!("{}{}", names.function("FromString"), names.generic_decl);
    let decode = format!("{}{}", names.function("Decode"), names.generic_args);
    let coders = declared_param_coders(Direction::Decode, &names.params);
    let passed_coders = passed_param_coders(Direction::Decode, &names.params);
    let return_type = decode_return_type(&names.full_type);
    from_string_function(&TextDecoder {
        name: &name,
        decode: &decode,
        coders: &coders,
        passed_coders: &passed_coders,
        return_type: &return_type,
        takes_options: true,
    })
}
