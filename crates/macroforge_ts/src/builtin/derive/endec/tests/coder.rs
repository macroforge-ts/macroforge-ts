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
    for ts_type in [
        "unknown",
        "unknown[]",
        "Record<string, unknown>",
        "null",
        "object",
    ] {
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
fn a_value_of_a_type_parameter_is_coded_by_the_coder_handed_in_for_it() {
    assert_eq!(
        coded("T"),
        (
            Some("(__encodeT===undefined?v:__encodeT(v,ctx))".to_string()),
            Some("(__decodeT===undefined?rawasT:__decodeT(raw,ctx))".to_string())
        )
    );
    let (encoded, decoded) = coded("T[]");
    assert_eq!(
        encoded.as_deref(),
        Some("v.map((__item0)=>(__encodeT===undefined?__item0:__encodeT(__item0,ctx)))")
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "__mf_decodeArray(raw,ctx,(__raw0)=>(__decodeT===undefined?__raw0asT:__decodeT(__raw0,ctx)))"
        )
    );
}

#[test]
fn a_generic_type_is_handed_the_coders_of_its_arguments() {
    let (encoded, decoded) = coded("Box<Date>");
    assert_eq!(
        encoded.as_deref(),
        Some("boxEncodeWithContext<Date>(v,ctx,(__arg0)=>__arg0.toISOString())")
    );
    assert_eq!(
        decoded.as_deref(),
        Some("boxDecodeWithContext<Date>(raw,ctx,__mf_decodeDate)")
    );

    // An argument that is its own coding has no coder, and a later one that
    // has one keeps its place.
    let (encoded, decoded) = coded("Pair<unknown, User>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "pairEncodeWithContext<unknown,User>(v,ctx,undefined,(__arg0)=>userEncodeWithContext(__arg0,ctx))"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "pairDecodeWithContext<unknown,User>(raw,ctx,undefined,(__raw0)=>userDecodeWithContext(__raw0,ctx))"
        )
    );
    assert_eq!(
        coded("Box<unknown>"),
        (
            Some("boxEncodeWithContext(v,ctx)".to_string()),
            Some("boxDecodeWithContext(raw,ctx)".to_string())
        )
    );

    // A parameter in scope is handed on, and a selection keeps its place
    // ahead of the coders.
    let (encoded, decoded) = coded("Partial<Box<T>>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "boxEncodePartialWithContext<T>(v,ctx,undefined,(__arg0)=>(__encodeT===undefined?__arg0:__encodeT(__arg0,ctx)))"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "boxDecodePartialWithContext<T>(raw,ctx,undefined,(__raw0)=>(__decodeT===undefined?__raw0asT:__decodeT(__raw0,ctx)))"
        )
    );
    let (encoded, _) = coded("Pick<Box<Date>, 'held'>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            r#"boxEncodePartialWithContext<Date>(v,ctx,{only:["held"]},(__arg0)=>__arg0.toISOString())"#
        )
    );
}

#[test]
fn a_union_a_field_cannot_tell_apart_is_an_error() {
    for ts_type in [
        "User | Admin",
        "(User | Admin)[]",
        "User | Admin | null",
        "Date | number",
        "Set<string> | string[]",
        "Partial<User> | string[]",
    ] {
        assert!(
            failure(ts_type).contains("nothing in a field says which member"),
            "{ts_type}"
        );
    }
    // A union of values that are their own coding is carried as it is, a
    // date beside a string is that string, and a function's `|` is its own.
    for ts_type in [
        "Date | string",
        "string | string[]",
        "number | 'auto'",
        "() => User | Admin",
    ] {
        assert_eq!(coded(ts_type).0, None, "{ts_type}");
    }
    // One user type beside a primitive or a literal is told from it.
    let (encoded, decoded) = coded("string | User");
    assert_eq!(
        encoded.as_deref(),
        Some(r#"(typeofv==="string"?v:userEncodeWithContext(v,ctx))"#)
    );
    assert_eq!(
        decoded.as_deref(),
        Some(r#"(typeofraw==="string"?raw:userDecodeWithContext(raw,ctx))"#)
    );
    let (encoded, decoded) = coded("Box<Date> | 'none'");
    assert_eq!(
        encoded.as_deref(),
        Some("(v==='none'?v:boxEncodeWithContext<Date>(v,ctx,(__arg0)=>__arg0.toISOString()))")
    );
    assert_eq!(
        decoded.as_deref(),
        Some("(raw==='none'?raw:boxDecodeWithContext<Date>(raw,ctx,__mf_decodeDate))")
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
    assert_eq!(coded("[unknown, object]"), (None, None));
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
    // An index signature over strings alone is the `Record` it describes.
    let (encoded, decoded) = coded("{ [key: string]: Date }");
    assert!(encoded.is_some_and(|encoded| encoded.contains("toISOString()")));
    assert!(
        decoded
            .is_some_and(|decoded| decoded.contains("__mf_decodeRecord(raw,ctx,__mf_decodeDate)"))
    );
    assert_eq!(coded("{ [key: string]: unknown }"), (None, None));
    // Beside named members, or under a computed key, it names no member to code.
    assert!(failure("{ id: string; [key: string]: Date }").contains("which no name reads"));
    assert!(failure("{ id: string; [KEY]: Date }").contains("`[KEY]`"));
    assert!(failure("{ [key: number]: Date }").contains("which no name reads"));
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
            fields: WrapperFields::Any,
            ..NestedType::whole("User".to_string())
        }))
    );
    assert_eq!(
        NestedType::of("Required<Omit<User, 'id'>>", &no_param),
        Ok(Some(NestedType {
            fields: WrapperFields::Except(vec!["id".to_string()]),
            required: true,
            ..NestedType::whole("User".to_string())
        }))
    );
    // A `Required` gives a `Partial` its fields back, and a `Partial` takes
    // a `Required`'s away.
    assert_eq!(
        NestedType::of("Required<Partial<User>>", &no_param),
        Ok(Some(NestedType {
            required: true,
            ..NestedType::whole("User".to_string())
        }))
    );
    assert_eq!(
        NestedType::of("Partial<Required<User>>", &no_param),
        Ok(Some(NestedType {
            fields: WrapperFields::Any,
            ..NestedType::whole("User".to_string())
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
