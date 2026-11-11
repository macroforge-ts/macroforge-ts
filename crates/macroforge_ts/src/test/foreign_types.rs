use super::*;
use crate::host::import_registry::{clear_foreign_types, set_foreign_types};
use crate::ts_syn::abi::DiagnosticLevel;

// ============================================================================
// Foreign Types in Default macro -- foreign expression body references a
// namespace beyond the surface type (e.g. DateTime.Utc default body calling
// Option.match). The target file imports only DateTime, NOT Option, so the
// engine must auto-import + alias-rewrite Option for the inlined expression
// to be runnable.
// ============================================================================

#[test]
fn test_a_foreign_default_is_called_through_its_export() {
    // A handler whose body reads names the target file never imports (here
    // `Option`) is not copied into generated code: generated code imports its
    // export from the expanded config, where the body keeps the config's own
    // imports.
    let config_source = r#"
import { DateTime, Option } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v: DateTime.Utc) => DateTime.formatIso(v),
            decode: (raw: unknown) =>
                Option.match(DateTime.make(raw as string), {
                    onSome: (dt) => dt,
                    onNone: () => Option.getOrElse(DateTime.make(0), () => null as never),
                }),
            default: () =>
                Option.match(DateTime.make(new Date()), {
                    onSome: (dt) => dt,
                    onNone: () => Option.getOrElse(DateTime.make(0), () => null as never),
                }),
            hasShape: (v: unknown) => typeof v === 'string',
        },
    },
};
"#;
    let config_path = "/test/cross-ns-default/macroforge.config.ts";

    let source = r#"
import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Foo {
    /** @default("place:holder") */
    id: string;
    createdAt: DateTime.Utc;
}
"#;

    {
        crate::host::config::CONFIG_CACHE.remove(config_path);
        clear_foreign_types();
        crate::host::import_registry::clear_registry();
        crate::host::config::MacroforgeConfigLoader::load_and_cache(config_source, config_path)
            .expect("config should parse");

        let options = crate::ExpandOptions {
            keep_decorators: None,
            external_decorator_modules: None,
            config_path: Some(config_path.to_string()),
            type_registry_json: None,
            declarative_registry_json: None,
            type_registry_id: None,
            declarative_registry_id: None,
            build_mode: None,
            emit_metadata: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::CONFIG_CACHE.remove(config_path);

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == "error")
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        assert!(
            result.code.lines().any(|line| line.starts_with("import {")
                && line.contains("__foreign__dateTimeUtcDefault")
                && line.ends_with(r##"} from "#macroforge/config";"##)),
            "the default is imported by name from the expanded config. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("__foreign__dateTimeUtcDefault()"),
            "the default is called through its export. Got:\n{}",
            result.code
        );
        assert!(
            !result.code.contains("Option.match") && !result.code.contains("__mf_Option"),
            "the handler body is not copied into generated code. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_a_foreign_default_reading_another_foreign_namespace_is_called_by_export() {
    // The default reads `Option`, which neither the target nor the config
    // imports. Nothing needs importing in generated code: the default is
    // called through its export, and its body stays in the expanded config.
    let config_source = r#"
// Note: NO top-level `Option` import in the config: the namespace is
// only known to the engine via the `Option` foreign type's `from` list.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v) => DateTime.formatIso(v),
            decode: (raw) => DateTime.make(raw),
            default: () =>
                Option.match(DateTime.make(new Date()), {
                    onSome: (dt) => dt,
                    onNone: () => DateTime.make(0),
                }),
        },
        'Option': {
            from: ['effect'],
            encode: (v) => v,
            decode: (raw) => raw,
            default: () => null,
        },
    },
};
"#;
    let config_path = "/test/cross-ns-only-ft-from/macroforge.config.ts";

    let source = r#"
import { DateTime } from 'effect';

/** @derive(Default) */
export interface Foo {
    createdAt: DateTime.Utc;
}
"#;

    {
        crate::host::config::CONFIG_CACHE.remove(config_path);
        clear_foreign_types();
        crate::host::import_registry::clear_registry();
        crate::host::config::MacroforgeConfigLoader::load_and_cache(config_source, config_path)
            .expect("config should parse");

        let options = crate::ExpandOptions {
            keep_decorators: None,
            external_decorator_modules: None,
            config_path: Some(config_path.to_string()),
            type_registry_json: None,
            declarative_registry_json: None,
            type_registry_id: None,
            declarative_registry_id: None,
            build_mode: None,
            emit_metadata: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::CONFIG_CACHE.remove(config_path);

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == "error")
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        assert!(
            result.code.lines().any(|line| line.starts_with("import {")
                && line.contains("__foreign__dateTimeUtcDefault")
                && line.ends_with(r##"} from "#macroforge/config";"##)),
            "the default is imported by name from the expanded config. Got:\n{}",
            result.code
        );
        assert!(
            !result.code.contains("Option.match") && !result.code.contains("__mf_Option"),
            "the handler body is not copied into generated code. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_foreign_handlers_reading_js_globals_import_only_their_exports() {
    // Regression: a foreign-type expression body that references a JS
    // global (`console`, `Math`, `Array`, `Object`, `BigInt`, `JSON`, …)
    // must NOT cause the engine to synthesise an
    // `import { <global> as __mf_<global> } from "<ft.from>"` line. Globals
    // resolve at runtime; treating them as importable namespaces produces a
    // broken cache (e.g. `import { console as __mf_console } from "effect"`).
    let config_source = r#"
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            encode: (v) => DateTime.formatIso(v),
            decode: (raw) => {
                if (!Array.isArray(raw) && typeof raw !== 'string') {
                    console.error('bad DateTime.Utc payload', raw);
                    return DateTime.make(0);
                }
                return DateTime.make(raw);
            },
            default: () => {
                const now = Math.floor(Date.now() / 1000);
                return DateTime.make(now);
            },
        },
    },
};
"#;
    let config_path = "/test/no-import-for-globals/macroforge.config.ts";

    let source = r#"
import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Foo {
    createdAt: DateTime.Utc;
}
"#;

    {
        crate::host::config::CONFIG_CACHE.remove(config_path);
        clear_foreign_types();
        crate::host::import_registry::clear_registry();
        crate::host::config::MacroforgeConfigLoader::load_and_cache(config_source, config_path)
            .expect("config should parse");

        let options = crate::ExpandOptions {
            keep_decorators: None,
            external_decorator_modules: None,
            config_path: Some(config_path.to_string()),
            type_registry_json: None,
            declarative_registry_json: None,
            type_registry_id: None,
            declarative_registry_id: None,
            build_mode: None,
            emit_metadata: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::CONFIG_CACHE.remove(config_path);

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == "error")
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // None of these JS globals must appear as `__mf_X` aliases or as
        // generated imports. They resolve at runtime as part of the JS spec.
        for global in [
            "console", "Math", "Array", "Object", "BigInt", "JSON", "Date",
        ] {
            let alias = format!("__mf_{}", global);
            assert!(
                !result.code.contains(&alias),
                "Should NOT alias JS global `{}`. Got:\n{}",
                global,
                result.code
            );
            let bad_import = format!("import {{ {} as __mf_{} }}", global, global);
            assert!(
                !result.code.contains(&bad_import),
                "Should NOT emit synthetic import for JS global `{}`. Got:\n{}",
                global,
                result.code
            );
        }

        // The bodies that read the globals stay in the expanded config;
        // generated code only imports and calls the exports.
        let handler_imports: Vec<&str> = result
            .code
            .lines()
            .filter(|line| line.ends_with(r##"} from "#macroforge/config";"##))
            .collect();
        assert!(
            handler_imports.len() == 1
                && [
                    "__foreign__dateTimeUtcDecode",
                    "__foreign__dateTimeUtcDefault"
                ]
                .iter()
                .all(|name| handler_imports[0].contains(name)),
            "the handlers are imported from the expanded config in one declaration. Got:\n{}",
            result.code
        );
        assert!(
            !result.code.contains("console.error") && !result.code.contains("Math.floor"),
            "the handler bodies are not copied into generated code. Got:\n{}",
            result.code
        );
    }
}

// ============================================================================
// Foreign Types in Union Type Alias -- Decode
// ============================================================================

#[test]
fn test_derive_decode_union_with_foreign_type_uses_has_shape() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Decode) */
interface RegularType {
    id: string;
}

/** @derive(Decode) */
type FlexibleValue = DateTime.DateTime | RegularType;
"#;

    {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            Some("(v) => typeof v === \"string\""),
        )]);

        let result = expand_test(source);

        clear_foreign_types();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should use the configured decode expression, not broken camelCase helpers
        assert!(
            result.code.contains("__foreign__dateTimeDateTimeDecode("),
            "Should use foreign type decode expression. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result.code.contains("dateTime.dateTimeDecodeWithContext"),
            "Should NOT generate broken dotted decode fn. Got:\n{}",
            result.code
        );

        // Should use the hasShape expression for shape matching
        assert!(
            result.code.contains("__foreign__dateTimeDateTimeHasShape("),
            "Should use foreign type hasShape expression. Got:\n{}",
            result.code
        );
    }
}

// A string input to a union with a foreign member is that member's value when
// its hasShape accepts the string, and JSON text otherwise. The handler is
// imported, so the check runs at runtime rather than reading its source.
#[test]
fn test_derive_decode_union_takes_a_value_and_from_string_takes_text() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Decode) */
interface RegularType {
    id: string;
}

/** @derive(Decode) */
type FlexibleValue = DateTime.DateTime | RegularType;
"#;
    set_foreign_types(vec![make_foreign_type(
        "DateTime.DateTime",
        vec!["effect"],
        None,
        Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
        None,
        Some("isIsoString"),
    )]);
    let result = expand_test(source);
    clear_foreign_types();

    assert!(
        result
            .code
            .contains("const resultOrRef = flexibleValueDecodeWithContext(input, ctx);"),
        "decode hands its input on as a value. Got:\n{}",
        result.code
    );
    assert!(
        !result.code.contains("__mf_parsedText(input)"),
        "decode never reads a string as JSON text. Got:\n{}",
        result.code
    );
    assert!(
        result.code.contains(
            "export function flexibleValueFromString(text: string, opts?: __mf_DecodeOptions)"
        ) && result.code.contains(
            "return __mf_decodedText(text, (value) => flexibleValueDecode(value, opts));"
        ),
        "fromString decodes JSON text through decode. Got:\n{}",
        result.code
    );
}

#[test]
fn test_derive_decode_union_foreign_only_types() {
    let source = r#"
import type { DateTime, BigDecimal } from 'effect';

/** @derive(Decode) */
type FlexValue = DateTime.DateTime | BigDecimal.BigDecimal;
"#;

    {
        set_foreign_types(vec![
            make_foreign_type(
                "DateTime.DateTime",
                vec!["effect"],
                Some("(v) => DateTime.formatIso(v)"),
                Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
                Some("() => DateTime.unsafeNow()"),
                Some("(v) => typeof v === \"string\""),
            ),
            make_foreign_type(
                "BigDecimal.BigDecimal",
                vec!["effect"],
                Some("(v) => BigDecimal.format(v)"),
                Some("(raw) => BigDecimal.fromString(String(raw))"),
                Some("() => BigDecimal.unsafeFromNumber(0)"),
                Some("(v) => typeof v === \"string\" || typeof v === \"number\""),
            ),
        ]);

        let result = expand_test(source);

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Both foreign decode expressions should be present
        assert!(
            result.code.contains("__foreign__dateTimeDateTimeDecode("),
            "Should have DateTime foreign decode. Got:\n{}",
            result.code
        );
        assert!(
            result
                .code
                .contains("__foreign__bigDecimalBigDecimalDecode("),
            "Should have BigDecimal foreign decode. Got:\n{}",
            result.code
        );

        // hasShape functions should be used
        assert!(
            result.code.contains("flexValueHasShape"),
            "Should generate hasShape for union. Got:\n{}",
            result.code
        );

        // Should NOT generate broken camelCase helper calls
        assert!(
            !result.code.contains("dateTime.dateTimeDecodeWithContext"),
            "Should NOT generate broken DateTime dotted identifier. Got:\n{}",
            result.code
        );
        assert!(
            !result
                .code
                .contains("bigDecimal.bigDecimalDecodeWithContext"),
            "Should NOT generate broken BigDecimal dotted identifier. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_derive_decode_union_foreign_without_has_shape() {
    // Foreign type without hasShape should still use foreign decode for __type dispatch
    // but won't participate in shape matching
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Decode) */
interface RegularType {
    id: string;
}

/** @derive(Decode) */
type Value = DateTime.DateTime | RegularType;
"#;

    {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            None, // No hasShape
        )]);

        let result = expand_test(source);

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should still use foreign decode for __type-based dispatch
        assert!(
            result.code.contains("__foreign__dateTimeDateTimeDecode("),
            "Should use foreign decode even without hasShape. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result.code.contains("dateTime.dateTimeDecodeWithContext"),
            "Should NOT generate broken dotted identifier. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_derive_decode_union_mixed_foreign_and_primitives() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Decode) */
type MaybeDate = DateTime.DateTime | string | number;
"#;

    {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            Some("(v) => typeof v === \"string\""),
        )]);

        let result = expand_test(source);

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should use foreign type decode
        assert!(
            result.code.contains("__foreign__dateTimeDateTimeDecode("),
            "Should use foreign type decode in mixed union. Got:\n{}",
            result.code
        );

        // Should have primitive checks too
        assert!(
            result.code.contains("typeof value === \"string\""),
            "Should have primitive string check. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("typeof value === \"number\""),
            "Should have primitive number check. Got:\n{}",
            result.code
        );
    }
}

// ============================================================================
// Foreign Types in Union Type Alias -- Encode
// ============================================================================

const LINK_SOURCE: &str = r#"
import type { DateTime } from 'effect';

/** @derive(Encode) */
type Link<Linked> = DateTime.DateTime | Linked;
"#;

// A foreign member is told by its `is` and encoded by its handler, ahead of
// the type parameter that takes whatever no other member is.
#[test]
fn test_derive_encode_generic_union_tells_a_foreign_member_by_its_is() {
    let mut date_time = make_foreign_type(
        "DateTime.DateTime",
        vec!["effect"],
        Some("(v) => DateTime.formatIso(v)"),
        Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
        Some("() => DateTime.unsafeNow()"),
        Some("(v) => typeof v === \"string\""),
    );
    date_time.is_expr =
        Some("(v: unknown): v is DateTime.DateTime => DateTime.isDateTime(v)".to_string());
    set_foreign_types(vec![date_time]);
    let result = expand_test(LINK_SOURCE);
    clear_foreign_types();

    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.level == DiagnosticLevel::Error)
        .collect();
    assert!(errors.is_empty(), "no errors expected, got {errors:?}");
    let code = &result.code;
    let foreign = code
        .find("__foreign__dateTimeDateTimeIs(member), (member) => __foreign__dateTimeDateTimeEncode(member))")
        .unwrap_or_else(|| panic!("the foreign member needs a coder of its own:\n{code}"));
    let param = code
        .find("__encodeLinked !== undefined")
        .unwrap_or_else(|| panic!("the type parameter keeps its coder:\n{code}"));
    assert!(
        foreign < param,
        "the foreign member is tried before the type parameter:\n{code}"
    );
}

#[test]
fn test_derive_encode_union_refuses_a_foreign_member_without_is() {
    set_foreign_types(vec![make_foreign_type(
        "DateTime.DateTime",
        vec!["effect"],
        Some("(v) => DateTime.formatIso(v)"),
        Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
        Some("() => DateTime.unsafeNow()"),
        Some("(v) => typeof v === \"string\""),
    )]);
    let result = expand_test(LINK_SOURCE);
    clear_foreign_types();

    assert!(
        result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.level == DiagnosticLevel::Error
                && diagnostic.message.contains("configures no `is` handler")),
        "a foreign member nothing can tell apart is an error, got {:?}",
        result.diagnostics
    );
}

// ============================================================================
// Foreign Types in the value traits
// ============================================================================

const VALUE_TRAITS_SOURCE: &str = r#"
import type { DateTime } from 'effect';

/** @derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord) */
interface Meeting {
    at: DateTime.DateTime;
    moved: DateTime.DateTime | null;
}
"#;

fn date_time_with_every_handler() -> ForeignTypeConfig {
    let mut date_time = make_foreign_type(
        "DateTime.DateTime",
        vec!["effect"],
        Some("(v) => DateTime.formatIso(v)"),
        Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
        Some("() => DateTime.unsafeNow()"),
        Some("(v) => typeof v === \"string\""),
    );
    date_time.is_expr = Some("(v) => DateTime.isDateTime(v)".to_string());
    date_time.clone_expr = Some("(v) => v".to_string());
    date_time.to_string_expr = Some("(v) => DateTime.formatIso(v)".to_string());
    date_time.equals_expr = Some("(a, b) => DateTime.Equivalence(a, b)".to_string());
    date_time.hash_code_expr = Some("(v) => DateTime.toEpochMillis(v) | 0".to_string());
    date_time.partial_compare_expr = Some("(a, b) => DateTime.Order(a, b)".to_string());
    date_time.compare_expr = Some("(a, b) => DateTime.Order(a, b)".to_string());
    date_time
}

// Each value trait runs the handler the foreign type configures for it, on a
// field of the type and on a nullable one.
#[test]
fn test_value_traits_call_a_foreign_types_handlers() {
    set_foreign_types(vec![date_time_with_every_handler()]);
    let result = expand_test(VALUE_TRAITS_SOURCE);
    clear_foreign_types();

    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.level == DiagnosticLevel::Error)
        .collect();
    assert!(errors.is_empty(), "no errors expected, got {errors:?}");
    let code = &result.code;
    for call in [
        "__foreign__dateTimeDateTimeClone(value.at)",
        "__foreign__dateTimeDateTimeEquals(a.at, b.at)",
        "__foreign__dateTimeDateTimeHashCode(value.at)",
        "__foreign__dateTimeDateTimeToString(value.at)",
        "__foreign__dateTimeDateTimePartialCompare(a.at, b.at)",
        "__foreign__dateTimeDateTimeCompare(a.at, b.at)",
        "__foreign__dateTimeDateTimeEquals(a.moved, b.moved)",
    ] {
        assert!(code.contains(call), "expected `{call}` in:\n{code}");
    }
    assert!(
        !code.contains("__mf_structuralClone(value.at)"),
        "a foreign field is not copied structurally:\n{code}"
    );
}

// A value trait on a foreign field whose type configures no handler for it is
// an error naming the handler, not a structural fallback.
#[test]
fn test_value_traits_refuse_a_foreign_type_without_their_handler() {
    set_foreign_types(vec![make_foreign_type(
        "DateTime.DateTime",
        vec!["effect"],
        Some("(v) => DateTime.formatIso(v)"),
        Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
        Some("() => DateTime.unsafeNow()"),
        Some("(v) => typeof v === \"string\""),
    )]);
    let result = expand_test(VALUE_TRAITS_SOURCE);
    clear_foreign_types();

    for (derive, handler) in [
        ("Clone", "clone"),
        ("PartialEq", "equals"),
        ("Hash", "hashCode"),
        ("Debug", "toString"),
        ("PartialOrd", "partialCompare"),
        ("Ord", "compare"),
    ] {
        let expected = format!(
            "@derive({derive}): the foreign type 'DateTime.DateTime' configures no `{handler}` handler"
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.level == DiagnosticLevel::Error
                    && diagnostic.message.contains(&expected)),
            "expected `{expected}`, got {:?}",
            result.diagnostics
        );
    }
}
