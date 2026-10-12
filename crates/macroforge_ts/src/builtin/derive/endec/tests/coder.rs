//! How a value is encoded and decoded by the structure of its type.

use crate::builtin::derive::endec::coder::{TypeScope, decode_expr, encode_expr};
use crate::builtin::derive::endec::nested::NestedType;
use crate::builtin::derive::endec::{EndecFieldOptions, WrapperFields};
use crate::ts_syn::abi::ir::TypeRegistry;

use super::make_decorator;

/// `source` without its whitespace, so a comparison does not hang on spacing.
fn squeezed(source: &str) -> String {
    source.split_whitespace().collect()
}

/// The encoding of `v` and the decoding of `raw` as `ts_type`, with `T` a
/// type parameter in scope.
fn coded(ts_type: &str) -> (Option<String>, Option<String>) {
    let registry = TypeRegistry::default();
    let type_params = ["T".to_string()];
    let scope = TypeScope {
        registry: &registry,
        file_path: "",
        file_imports: &[],
        type_params: &type_params,
    };
    let squeeze = |coded: Option<String>| coded.map(|source| squeezed(&source));
    (
        squeeze(encode_expr(ts_type, "v", &scope, 0).expect("the type encodes")),
        squeeze(decode_expr(ts_type, "raw", &scope, 0).expect("the type decodes")),
    )
}

fn failure(ts_type: &str) -> String {
    let registry = TypeRegistry::default();
    let scope = TypeScope {
        registry: &registry,
        file_path: "",
        file_imports: &[],
        type_params: &[],
    };
    encode_expr(ts_type, "v", &scope, 0).expect_err("the type has no encoding")
}

#[test]
fn a_value_that_is_its_own_encoding_is_left_alone() {
    for ts_type in ["unknown", "T", "T[]", "Record<string, T>", "null", "object"] {
        assert_eq!(coded(ts_type), (None, None), "{ts_type}");
    }
    let (encoded, decoded) = coded("string[]");
    assert_eq!(encoded, None);
    assert_eq!(
        decoded.as_deref(),
        Some("__mf_decodeArray(raw,ctx,__mf_decodeString)")
    );
}

#[test]
fn a_user_type_is_coded_by_its_own_functions() {
    let (encoded, decoded) = coded("User");
    assert_eq!(encoded.as_deref(), Some("userEncodeWithContext(v,ctx)"));
    assert_eq!(decoded.as_deref(), Some("userDecodeWithContext(raw,ctx)"));

    let (encoded, decoded) = coded("Readonly<User>");
    assert_eq!(encoded.as_deref(), Some("userEncodeWithContext(v,ctx)"));
    assert_eq!(decoded.as_deref(), Some("userDecodeWithContext(raw,ctx)"));
}

#[test]
fn a_collection_is_coded_through_its_members_at_every_depth() {
    let (encoded, decoded) = coded("Map<string, Set<Date>>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "Object.fromEntries(Array.from(v,([__key0,__item0])=>[__key0,Array.from(__item0,(__item1)=>__item1.toISOString())]))"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "__mf_decodeMap(raw,ctx,(__key0)=>__key0,(__raw0)=>__mf_decodeSet(__raw0,ctx,__mf_decodeDate))"
        )
    );

    let (encoded, decoded) = coded("User[][]");
    assert_eq!(
        encoded.as_deref(),
        Some("v.map((__item0)=>__item0.map((__item1)=>userEncodeWithContext(__item1,ctx)))")
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "__mf_decodeArray(raw,ctx,(__raw0)=>__mf_decodeArray(__raw0,ctx,(__raw1)=>userDecodeWithContext(__raw1,ctx)))"
        )
    );
}

#[test]
fn null_and_undefined_members_are_passed_by() {
    let (encoded, decoded) = coded("(Date | null)[]");
    assert_eq!(
        encoded.as_deref(),
        Some("v.map((__item0)=>(__item0===null?null:__item0.toISOString()))")
    );
    assert_eq!(
        decoded.as_deref(),
        Some("__mf_decodeArray(raw,ctx,(__raw0)=>(__raw0===null?null:__mf_decodeDate(__raw0)))")
    );
}

#[test]
fn a_partial_pick_or_omit_is_coded_by_the_fields_it_has() {
    let (encoded, decoded) = coded("Partial<User>[]");
    assert_eq!(
        encoded.as_deref(),
        Some("v.map((__item0)=>userEncodePartialWithContext(__item0,ctx))")
    );
    assert_eq!(
        decoded.as_deref(),
        Some("__mf_decodeArray(raw,ctx,(__raw0)=>userDecodePartialWithContext(__raw0,ctx))")
    );

    let (encoded, _) = coded("Set<Pick<User, 'id' | 'name'>>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            r#"Array.from(v,(__item0)=>userEncodePartialWithContext(__item0,ctx,{only:["id","name"]}))"#
        )
    );
    let (_, decoded) = coded("Omit<User, 'id'> | null");
    assert_eq!(
        decoded.as_deref(),
        Some(r#"(raw===null?null:userDecodePartialWithContext(raw,ctx,{except:["id"]}))"#)
    );
}

#[test]
fn a_tuple_is_coded_element_by_element() {
    let (encoded, decoded) = coded("[User, Date?, ...string[]]");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "[userEncodeWithContext(v[0],ctx),(v[1]===undefined?undefined:v[1].toISOString()),...v.slice(2)]"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "(__mf_decodeTuple(raw,ctx,[(__raw0)=>userDecodeWithContext(__raw0,ctx),__mf_decodeDate],1,__mf_decodeString)as[User,Date?,...string[]])"
        )
    );
    assert_eq!(coded("[T, unknown]"), (None, None));
}

#[test]
fn a_literal_or_a_union_of_primitives_takes_only_its_own_values() {
    let (encoded, decoded) = coded("'on' | 'off'");
    assert_eq!(encoded, None);
    assert_eq!(
        decoded.as_deref(),
        Some(r#"(__mf_decodeChecked(raw,raw==='on'||raw==='off',"'on'|'off'")as'on'|'off')"#)
    );
    let (_, decoded) = coded("(number | 'auto' | null)[]");
    assert_eq!(
        decoded.as_deref(),
        Some(
            r#"__mf_decodeArray(raw,ctx,(__raw0)=>(__mf_decodeChecked(__raw0,typeof__raw0==="number"||__raw0==='auto'||__raw0===null,"number|'auto'|null")asnumber|'auto'|null))"#
        )
    );
}

#[test]
fn an_object_type_written_in_place_is_coded_member_by_member() {
    let (encoded, decoded) = coded("{ at: Date; note?: string; 'kebab-key': User }");
    assert_eq!(
        encoded.as_deref(),
        Some(
            r#"({...v,at:v.at.toISOString(),"kebab-key":userEncodeWithContext(v["kebab-key"],ctx)})"#
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            r#"(__mf_decodeObject(raw,ctx,{at:__mf_decodeDate,note:__mf_decodeString,"kebab-key":(__raw0)=>userDecodeWithContext(__raw0,ctx)},["note"])as{at:Date;note?:string;'kebab-key':User})"#
        )
    );
    // An index signature names no member, so its values pass through.
    assert_eq!(coded("{ [key: string]: Date }"), (None, None));
}

#[test]
fn a_map_key_is_spelled_as_text_and_read_back_as_its_type() {
    let (encoded, decoded) = coded("Map<Date, number>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "Object.fromEntries(Array.from(v,([__key0,__item0])=>[__key0.toISOString(),__item0]))"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some("__mf_decodeMap(raw,ctx,(__key0)=>__mf_decodeDate(__key0),__mf_decodeNumber)")
    );
    let (_, decoded) = coded("Map<number, boolean>");
    assert_eq!(
        decoded.as_deref(),
        Some("__mf_decodeMap(raw,ctx,(__key0)=>__mf_decodeNumberKey(__key0),__mf_decodeBoolean)")
    );
    let (_, decoded) = coded("Map<'a' | 'b', string>");
    assert_eq!(
        decoded.as_deref(),
        Some(
            r#"__mf_decodeMap(raw,ctx,(__key0)=>__mf_decodeChecked(__key0,__key0==='a'||__key0==='b',"'a'|'b'")as'a'|'b',__mf_decodeString)"#
        )
    );
}

#[test]
fn a_map_keyed_by_an_object_has_no_encoding() {
    assert!(failure("Map<{ id: string }, number>").contains("has no JSON encoding"));
    assert!(failure("Map<string | number, number>").contains("has no JSON encoding"));
    assert!(failure("Set<Map<string[], number>>").contains("has no JSON encoding"));
}

#[test]
fn a_utility_type_over_a_user_type_says_which_fields_a_value_has() {
    let no_param = |_: &str| false;
    assert_eq!(
        NestedType::of("User", &no_param),
        Ok(Some(NestedType::whole("User".to_string())))
    );
    assert_eq!(
        NestedType::of("Readonly<Partial<User>>", &no_param),
        Ok(Some(NestedType {
            name: "User".to_string(),
            fields: WrapperFields::Any,
        }))
    );
    assert_eq!(
        NestedType::of("Required<Omit<User, 'id'>>", &no_param),
        Ok(Some(NestedType {
            name: "User".to_string(),
            fields: WrapperFields::Except(vec!["id".to_string()]),
        }))
    );
    assert_eq!(NestedType::of("Partial<T>", &|name| name == "T"), Ok(None));
    assert_eq!(NestedType::of("string", &no_param), Ok(None));
    assert!(NestedType::of("Partial<Pick<User, 'id'>>", &no_param).is_err());
    assert!(NestedType::of("Pick<User, Keys>", &no_param).is_err());
}

#[test]
fn a_bare_true_default_is_the_flag() {
    let options =
        |args: &str| EndecFieldOptions::from_decorators(&[make_decorator(args)], "f").options;

    for args in [
        "default",
        "{ default: true }",
        "default: true",
        "{ default: true, flatten: true }",
    ] {
        let parsed = options(args);
        assert!(parsed.default, "{args}");
        assert_eq!(parsed.default_expr, None, "{args}");
    }
    assert_eq!(
        options(r#"{ default: "true" }"#).default_expr.as_deref(),
        Some("true")
    );
    assert_eq!(
        options("{ default: trueish }").default_expr.as_deref(),
        Some("trueish")
    );
    assert_eq!(options("{ default: 1 }").default_expr.as_deref(), Some("1"));
    assert!(!options("{ default: false }").default);
}
