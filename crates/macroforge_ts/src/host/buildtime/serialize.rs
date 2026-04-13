//! Serialize [`SandboxValue`] back to TypeScript literal source.
//!
//! The pre-pass invokes the sandbox, gets a [`SandboxValue`], and must
//! splice an equivalent TS literal into the user's file. That conversion
//! happens here. The output is deliberately minimal — one line per value
//! when short, no pretty-printing, no trailing commas — because the
//! OXC codegen pass re-formats the file anyway.
//!
//! Object keys that aren't valid TS identifiers get quoted. Strings get
//! JSON-style escaping. Numbers that can't be expressed as decimal
//! literals (NaN, ±Infinity) cause a hard error — there's no valid TS
//! syntax for them, and the user would much rather see a build-time
//! failure than mysterious `NaN` values in their bundle.

use crate::host::buildtime::sandbox::SandboxValue;

/// Serializer failure modes.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SerializeError {
    #[error("value {0} has no valid TypeScript literal representation")]
    NotRepresentable(&'static str),
    #[error("number {0} cannot be serialized as a TypeScript numeric literal")]
    NumberNotRepresentable(f64),
}

/// Convert a [`SandboxValue`] to TypeScript source.
///
/// The returned string is a valid TS expression that, when parsed and
/// evaluated at runtime, produces a value equal to `value` (modulo the
/// BigInt/Number distinction, which is preserved by using `n` suffixes).
///
/// [`SandboxValue::SourceCode`] is emitted verbatim — the serializer
/// trusts the Tier 2 function to have returned valid TS. If it didn't,
/// the next OXC parse will catch it and produce a diagnostic on the
/// original `@buildtime function` declaration.
pub fn value_to_ts_source(value: &SandboxValue) -> Result<String, SerializeError> {
    let mut out = String::new();
    write_value(value, &mut out)?;
    Ok(out)
}

fn write_value(value: &SandboxValue, out: &mut String) -> Result<(), SerializeError> {
    match value {
        SandboxValue::Null => out.push_str("null"),
        SandboxValue::Undefined => out.push_str("undefined"),
        SandboxValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        SandboxValue::Number(n) => write_number(*n, out)?,
        SandboxValue::BigInt(i) => {
            out.push_str(&i.to_string());
            out.push('n');
        }
        SandboxValue::String(s) => write_string_literal(s, out),
        SandboxValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_value(item, out)?;
            }
            out.push(']');
        }
        SandboxValue::Object(map) => {
            out.push('{');
            for (i, (key, val)) in map.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_object_key(key, out);
                out.push_str(": ");
                write_value(val, out)?;
            }
            out.push('}');
        }
        SandboxValue::SourceCode(text) => out.push_str(text),
    }
    Ok(())
}

fn write_number(n: f64, out: &mut String) -> Result<(), SerializeError> {
    if n.is_nan() || n.is_infinite() {
        return Err(SerializeError::NumberNotRepresentable(n));
    }
    // Integer-valued floats go out as integers to avoid "42.0" in the
    // output. Negative zero is preserved by checking the bit pattern.
    if n == n.trunc() && n.abs() < 1e16 {
        if n == 0.0 && n.is_sign_negative() {
            out.push_str("-0");
        } else {
            out.push_str(&format!("{}", n as i64));
        }
    } else {
        out.push_str(&format!("{n}"));
    }
    Ok(())
}

fn write_string_literal(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_object_key(key: &str, out: &mut String) {
    if is_valid_ts_identifier(key) {
        out.push_str(key);
    } else {
        write_string_literal(key, out);
    }
}

/// True if `s` is a valid ES/TS identifier (and not a reserved word).
///
/// This is a conservative check — we reject anything that isn't an ASCII
/// identifier even though ES allows Unicode identifiers. Quoting an
/// otherwise-valid Unicode identifier is harmless, so the strict version
/// is fine.
fn is_valid_ts_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
        return false;
    }
    !is_reserved_word(s)
}

/// TS/JS reserved words that can't appear as bare property keys in
/// object literals without quoting.
///
/// Note: most JS engines accept reserved words as property keys, but TS
/// type-checking for the emitted code may flag some of them. Quoting is
/// always safe, so we err on the side of quoting.
fn is_reserved_word(s: &str) -> bool {
    matches!(
        s,
        "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "debugger"
            | "default"
            | "delete"
            | "do"
            | "else"
            | "enum"
            | "export"
            | "extends"
            | "false"
            | "finally"
            | "for"
            | "function"
            | "if"
            | "import"
            | "in"
            | "instanceof"
            | "new"
            | "null"
            | "return"
            | "super"
            | "switch"
            | "this"
            | "throw"
            | "true"
            | "try"
            | "typeof"
            | "var"
            | "void"
            | "while"
            | "with"
    )
}

#[cfg(test)]
mod serialize_unit {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn null_and_undefined() {
        assert_eq!(value_to_ts_source(&SandboxValue::Null).unwrap(), "null");
        assert_eq!(
            value_to_ts_source(&SandboxValue::Undefined).unwrap(),
            "undefined"
        );
    }

    #[test]
    fn booleans() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::Bool(true)).unwrap(),
            "true"
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::Bool(false)).unwrap(),
            "false"
        );
    }

    #[test]
    fn integer_numbers_go_out_as_integers() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::Number(42.0)).unwrap(),
            "42"
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::Number(-17.0)).unwrap(),
            "-17"
        );
        assert_eq!(value_to_ts_source(&SandboxValue::Number(0.0)).unwrap(), "0");
    }

    #[test]
    fn negative_zero_is_preserved() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::Number(-0.0)).unwrap(),
            "-0"
        );
    }

    #[test]
    fn fractional_numbers_round_trip() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::Number(3.14)).unwrap(),
            "3.14"
        );
    }

    #[test]
    fn nan_is_rejected() {
        let err = value_to_ts_source(&SandboxValue::Number(f64::NAN)).unwrap_err();
        assert!(matches!(err, SerializeError::NumberNotRepresentable(_)));
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(value_to_ts_source(&SandboxValue::Number(f64::INFINITY)).is_err());
        assert!(value_to_ts_source(&SandboxValue::Number(f64::NEG_INFINITY)).is_err());
    }

    #[test]
    fn bigint_gets_n_suffix() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::BigInt(42)).unwrap(),
            "42n"
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::BigInt(-9_999_999_999_999_999)).unwrap(),
            "-9999999999999999n"
        );
    }

    #[test]
    fn strings_are_json_escaped() {
        assert_eq!(
            value_to_ts_source(&SandboxValue::String("hi".to_string())).unwrap(),
            r#""hi""#
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::String("a\"b".to_string())).unwrap(),
            r#""a\"b""#
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::String("line\nnext".to_string())).unwrap(),
            r#""line\nnext""#
        );
        assert_eq!(
            value_to_ts_source(&SandboxValue::String("\x01".to_string())).unwrap(),
            r#""\u0001""#
        );
    }

    #[test]
    fn arrays() {
        let value = SandboxValue::Array(vec![
            SandboxValue::Number(1.0),
            SandboxValue::Number(2.0),
            SandboxValue::Number(3.0),
        ]);
        assert_eq!(value_to_ts_source(&value).unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn nested_arrays() {
        let value = SandboxValue::Array(vec![
            SandboxValue::Array(vec![SandboxValue::Number(1.0), SandboxValue::Number(2.0)]),
            SandboxValue::Array(vec![SandboxValue::Number(3.0)]),
        ]);
        assert_eq!(value_to_ts_source(&value).unwrap(), "[[1, 2], [3]]");
    }

    #[test]
    fn objects_with_valid_identifier_keys() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), SandboxValue::Number(1.0));
        map.insert("b".to_string(), SandboxValue::Number(2.0));
        let value = SandboxValue::Object(map);
        assert_eq!(value_to_ts_source(&value).unwrap(), "{a: 1, b: 2}");
    }

    #[test]
    fn objects_with_non_identifier_keys_are_quoted() {
        let mut map = BTreeMap::new();
        map.insert("has-dash".to_string(), SandboxValue::Number(1.0));
        map.insert("2leading".to_string(), SandboxValue::Number(2.0));
        let value = SandboxValue::Object(map);
        let out = value_to_ts_source(&value).unwrap();
        assert!(out.contains(r#""has-dash""#));
        assert!(out.contains(r#""2leading""#));
    }

    #[test]
    fn reserved_words_are_quoted() {
        let mut map = BTreeMap::new();
        map.insert("class".to_string(), SandboxValue::Number(1.0));
        let value = SandboxValue::Object(map);
        assert_eq!(value_to_ts_source(&value).unwrap(), r#"{"class": 1}"#);
    }

    #[test]
    fn source_code_splices_verbatim() {
        let value = SandboxValue::SourceCode("export const X = 42;".to_string());
        assert_eq!(value_to_ts_source(&value).unwrap(), "export const X = 42;");
    }

    #[test]
    fn dollar_sign_is_valid_identifier_start() {
        let mut map = BTreeMap::new();
        map.insert("$var".to_string(), SandboxValue::Number(1.0));
        let value = SandboxValue::Object(map);
        assert_eq!(value_to_ts_source(&value).unwrap(), "{$var: 1}");
    }
}
