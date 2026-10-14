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
    for ts_type in ["unknown", "unknown[]", "Record<string, unknown>", "null"] {
        assert_eq!(coded(ts_type), (None, None), "{ts_type}");
    }
    // An object is its own encoding too, once it is seen to be one.
    assert_eq!(
        coded("object"),
        (
            None,
            Some(
                r#"(__mf_decodeChecked(raw,(typeofraw==="object"&&raw!==null),"object")asobject)"#
                    .to_string()
            )
        )
    );
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
            "(boxDecodePartialWithContext<T>(raw,ctx,undefined,(__raw0)=>(__decodeT===undefined?__raw0asT:__decodeT(__raw0,ctx)))asPartial<Box<T>>|__mf_PendingRef)"
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
    // A union of values that are their own coding is carried as it is, and
    // a date beside a string is that string.
    for ts_type in ["Date | string", "string | string[]", "number | 'auto'"] {
        assert_eq!(coded(ts_type).0, None, "{ts_type}");
    }
    // A function's `|` is its own, and a function has no coding at all.
    assert!(failure("() => User | Admin").contains("is a function, which JSON has no form for"));
    assert!(failure("Map<string, symbol>").contains("is a symbol, which JSON has no form for"));
    assert!(failure("(keyof User)[]").contains("only the type checker can resolve"));
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

    // A selection of a type's fields is decoded as the selection written.
    let (encoded, decoded) = coded("Readonly<User>");
    assert_eq!(encoded.as_deref(), Some("userEncodeWithContext(v,ctx)"));
    assert_eq!(
        decoded.as_deref(),
        Some("(userDecodeWithContext(raw,ctx)asReadonly<User>|__mf_PendingRef)")
    );
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
        Some(
            "__mf_decodeArray(raw,ctx,(__raw0)=>(userDecodePartialWithContext(__raw0,ctx)asPartial<User>|__mf_PendingRef))"
        )
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
        Some(
            r#"(raw===null?null:(userDecodePartialWithContext(raw,ctx,{except:["id"]})asOmit<User,'id'>|__mf_PendingRef))"#
        )
    );
}

#[test]
fn a_tuple_is_coded_element_by_element() {
    let (encoded, decoded) = coded("[User, Date?, ...string[]]");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "[userEncodeWithContext(v[0],ctx),...(1<v.length?[(v[1]===undefined?undefined:v[1].toISOString())]:[]),...(([,,...__rest0])=>__rest0)(v)]"
        )
    );
    assert_eq!(
        decoded.as_deref(),
        Some(
            "(__mf_decodeTuple(raw,ctx,[(__raw0)=>userDecodeWithContext(__raw0,ctx),(__raw0)=>{const__read0=__raw0??undefined;return(__read0===undefined?undefined:__mf_decodeDate(__read0));}],1,__mf_decodeString)as[User,Date?,...string[]])"
        )
    );
    // A tuple whose elements decode as themselves is still held to its length.
    let (encoded, decoded) = coded("[unknown, unknown]");
    assert_eq!(encoded, None);
    assert_eq!(
        decoded.as_deref(),
        Some(
            "(__mf_decodeTuple(raw,ctx,[(__raw0)=>__raw0asunknown,(__raw0)=>__raw0asunknown],2)as[unknown,unknown])"
        )
    );
    assert_eq!(
        coded("[]").1.as_deref(),
        Some("(__mf_decodeTuple(raw,ctx,[],0)as[])")
    );
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
            r#"__mf_decodeArray(raw,ctx,(__raw0)=>(__mf_decodeChecked(__raw0,(typeof__raw0==="number"&&globalThis.Number.isFinite(__raw0))||__raw0==='auto'||__raw0===null,"number|'auto'|null")asnumber|'auto'|null))"#
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

/// A registry holding `Owner`, an unexported `Hidden` and an `Item` with a
/// field of each in one file, for a scope in another.
fn declared_elsewhere() -> TypeRegistry {
    use crate::ts_syn::abi::ir::InterfaceFieldIR;
    use crate::ts_syn::abi::ir::class::MemberKey;
    use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistryEntry};
    use crate::ts_syn::abi::{InterfaceIR, SpanIR};

    let field = |name: &str, ts_type: &str| InterfaceFieldIR {
        name: name.to_string(),
        key: MemberKey::Named,
        span: SpanIR::default(),
        ts_type: ts_type.to_string(),
        optional: false,
        readonly: false,
        decorators: vec![],
    };
    let interface = |name: &str, fields: Vec<InterfaceFieldIR>| InterfaceIR {
        name: name.to_string(),
        span: SpanIR::default(),
        body_span: SpanIR::default(),
        type_params: vec![],
        heritage: vec![],
        decorators: vec![],
        fields,
        methods: vec![],
    };
    let mut registry = TypeRegistry::new();
    for declared in [
        interface("Owner", vec![field("name", "string")]),
        interface("Hidden", vec![field("name", "string")]),
        interface(
            "Item",
            vec![
                field("owner", "Owner"),
                field("label", "string"),
                field("secret", "Hidden"),
            ],
        ),
    ] {
        registry.insert(
            TypeRegistryEntry {
                is_exported: declared.name != "Hidden",
                exported_as: Vec::new(),
                re_exported: Vec::new(),
                name: declared.name.clone(),
                file_path: "/project/src/item.ts".to_string(),
                definition: TypeDefinitionIR::Interface(declared),
                file_imports: vec![],
            },
            "/project",
        );
    }
    registry
}

#[test]
fn an_operator_over_a_declared_type_is_read_as_what_it_yields() {
    use crate::ts_syn::abi::ir::type_registry::FileImportEntry;

    let registry = declared_elsewhere();
    let import = |name: &str| FileImportEntry {
        local_name: name.to_string(),
        module_specifier: "./item".to_string(),
        original_name: None,
        is_type_only: true,
    };
    let imports = [import("Item")];
    let scope = TypeScope {
        registry: &registry,
        file_path: "/project/src/holder.ts",
        file_imports: &imports,
        type_params: &[],
    };
    let decoded = |ts_type: &str| {
        decode_expr(ts_type, "raw", &scope, 0).map(|coded| coded.map(|source| squeezed(&source)))
    };
    assert_eq!(
        decoded("keyof Item"),
        Ok(Some(
            r#"(__mf_decodeChecked(raw,raw==="owner"||raw==="label"||raw==="secret","\"owner\"|\"label\"|\"secret\"")as"owner"|"label"|"secret")"#.to_string()
        ))
    );
    assert_eq!(
        decoded("Item['label']"),
        Ok(Some("__mf_decodeString(raw)".to_string()))
    );

    // `Owner` is named where `Item` is declared: it is imported from there.
    assert_eq!(
        decoded("Item['owner']"),
        Ok(Some("ownerDecodeWithContext(raw,ctx)".to_string()))
    );
    crate::host::import_registry::with_registry(|imports| {
        assert_eq!(imports.get_source("Owner"), Some("./item"));
        assert_eq!(
            imports.borrowed_modules().get("Owner").map(String::as_str),
            Some("./item")
        );
    });

    // A type its module keeps to itself cannot be.
    let hidden = decoded("Item['secret']").expect_err("Hidden is not exported");
    assert!(
        hidden.contains("names `Hidden`, a type its module does not export; export `Hidden`"),
        "{hidden}"
    );
}

/// A type is read once at each level it nests, and one at the limit of what
/// a derive reads fits a test thread's stack: read twice a level, or deeper
/// than the stack holds, it would not finish.
#[test]
fn a_type_nested_to_the_limit_is_coded_a_level_at_a_time() {
    let depth = crate::builtin::derive::common::MAX_TYPE_DEPTH;
    let nested =
        |open: &str, close: &str| format!("{}number{}", open.repeat(depth), close.repeat(depth));
    let decoded = |ts_type: String| coded(&ts_type).1.expect("the type decodes");
    assert_eq!(
        decoded(nested("", "[]")).matches("decodeArray").count(),
        depth
    );
    assert_eq!(
        decoded(nested("{ held: ", " }"))
            .matches("decodeObject")
            .count(),
        depth
    );
    assert_eq!(
        decoded(nested("[", "]")).matches("decodeTuple(").count(),
        depth
    );
    assert_eq!(
        decoded(nested("Record<string, ", ">"))
            .matches("decodeRecord")
            .count(),
        depth
    );
    assert!(decoded(nested("(", " | null)")).contains("decodeChecked"));
}

/// JSON has no `undefined`: it writes one in an array as `null` and drops
/// an entry that holds one.
#[test]
fn an_undefined_in_a_collection_is_coded_as_null() {
    let (encoded, decoded) = coded("(number | undefined)[]");
    assert_eq!(encoded, None);
    // The value is read once, whatever the decoder asks of it after.
    let decoded = decoded.expect("the array decodes");
    assert!(decoded.contains("const__read0=__raw0??undefined;"));
    assert_eq!(decoded.matches("??undefined").count(), 1);

    let (encoded, decoded) = coded("Record<string, number | undefined>");
    assert_eq!(
        encoded.as_deref(),
        Some(
            "Object.fromEntries((Object.entries(v)as[string,number|undefined][]).map(([__key0,__item0])=>[__key0,(__item0===undefined?null:__item0)]))"
        )
    );
    assert!(
        decoded
            .expect("the record decodes")
            .contains("const__read0=__raw0??undefined;")
    );

    let (encoded, _) = coded("Map<string, Date | undefined>");
    assert!(
        encoded
            .expect("the map encodes")
            .contains("(__item0===undefined?null:")
    );

    let (_, decoded) = coded("[number, string?]");
    assert!(
        decoded
            .expect("the tuple decodes")
            .contains("const__read0=__raw0??undefined;")
    );

    // A `null` that is a value of the type stays one.
    let (encoded, decoded) = coded("(number | null | undefined)[]");
    assert_eq!(encoded, None);
    assert!(!decoded.expect("the array decodes").contains("??undefined"));
    let (_, decoded) = coded("[number, (string | null)?]");
    assert!(!decoded.expect("the tuple decodes").contains("??undefined"));
}
