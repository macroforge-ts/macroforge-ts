use super::*;

// ============================================================================
// Foreign Types in Default macro -- foreign expression body references a
// namespace beyond the surface type (e.g. DateTime.Utc default body calling
// Option.match). The target file imports only DateTime, NOT Option, so the
// engine must auto-import + alias-rewrite Option for the inlined expression
// to be runnable.
// ============================================================================

#[test]
fn test_default_inlines_foreign_expression_with_cross_namespace_reference() {
    // User repro: foreign type DateTime.Utc whose default body uses Option.
    // Target file imports only DateTime; Option appears only inside the
    // inlined default expression. Without aliasing + import generation, the
    // emitted file references `Option.match` against an undefined symbol.
    //
    // Drive the full production flow: load a config file via
    // `MacroforgeConfigLoader::load_and_cache` (which extracts
    // `expression_namespaces` and `config_imports` from a real source string),
    // then call the public expand entry point with `config_path` so the
    // CONFIG_CACHE -> registry hand-off mirrors what the CLI / WASM bindings
    // do at runtime.
    let config_source = r#"
import { DateTime, Option } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v: DateTime.Utc) => DateTime.formatIso(v),
            deserialize: (raw: unknown) =>
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

/** @derive(Default, Serialize, Deserialize) */
export interface Foo {
    /** @default("place:holder") */
    id: string;
    createdAt: DateTime.Utc;
}
"#;

    {
        crate::host::config::clear_config_cache();
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
            build_mode: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::clear_config_cache();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == "error")
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Option appears only in the inlined default body. The engine must
        // emit a value import for it (aliased) so the IIFE actually runs.
        assert!(
            result
                .code
                .contains("import { Option as __mf_Option } from"),
            "Expected `import {{ Option as __mf_Option }}` to be emitted. Got:\n{}",
            result.code
        );

        // The inlined default must use the aliased namespace, not bare Option.
        assert!(
            result.code.contains("__mf_Option.match"),
            "Default IIFE should use __mf_Option.match. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("__mf_Option.getOrElse"),
            "Default IIFE should use __mf_Option.getOrElse. Got:\n{}",
            result.code
        );

        // DateTime is already a value import in the target — should NOT be
        // re-imported under an __mf_ alias, and the body should reference it
        // directly.
        assert!(
            !result.code.contains("__mf_DateTime"),
            "DateTime is already value-imported; should not be aliased. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("DateTime.make"),
            "Default IIFE should call DateTime.make directly. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_default_inlines_foreign_expression_when_namespace_only_in_ft_from() {
    // Variant of the user repro: the namespace referenced in the default body
    // (here `Option`) is the foreign type's *own* surface type, but at the
    // point we're processing a *different* foreign type (`DateTime.Utc`)
    // whose default body also calls `Option.match`, neither
    //   - the target file's source imports
    //   - nor the config file's top-level imports
    // contain `Option`. The engine must still emit a value import for it,
    // resolving the module from the foreign type's `from` list.
    //
    // Pre-fix this skips silently and the inlined IIFE references undefined
    // `Option`. After the fix, the alias and import are emitted.
    let config_source = r#"
// Note: NO top-level `Option` import in the config — the namespace is
// only known to the engine via the `Option` foreign type's `from` list.
import { DateTime } from 'effect';

export default {
    foreignTypes: {
        'DateTime.Utc': {
            from: ['effect'],
            serialize: (v) => DateTime.formatIso(v),
            deserialize: (raw) => DateTime.make(raw),
            default: () =>
                Option.match(DateTime.make(new Date()), {
                    onSome: (dt) => dt,
                    onNone: () => DateTime.make(0),
                }),
        },
        'Option': {
            from: ['effect'],
            serialize: (v) => v,
            deserialize: (raw) => raw,
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
        crate::host::config::clear_config_cache();
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
            build_mode: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::clear_config_cache();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == "error")
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // The default body references Option.match. The engine must emit a
        // value import for Option even though it's not in source_imports
        // (target only imports DateTime) and not in config_imports (config
        // only imports DateTime). It IS configured as a foreign type, so its
        // module is known via the Option foreign type's `from = ['effect']`.
        assert!(
            result
                .code
                .contains("import { Option as __mf_Option } from"),
            "Expected `import {{ Option as __mf_Option }}` to be emitted. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("__mf_Option.match"),
            "Default IIFE should use __mf_Option.match. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_default_does_not_import_js_globals_referenced_in_foreign_body() {
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
            serialize: (v) => DateTime.formatIso(v),
            deserialize: (raw) => {
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

/** @derive(Default, Serialize, Deserialize) */
export interface Foo {
    createdAt: DateTime.Utc;
}
"#;

    {
        crate::host::config::clear_config_cache();
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
            build_mode: None,
        };
        let result = crate::api::CoreEngine::expand_sync(
            source.to_string(),
            "test.ts".to_string(),
            Some(options),
        )
        .expect("expand_sync should succeed");

        clear_foreign_types();
        crate::host::config::clear_config_cache();

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

        // Globals stay unrewritten in the inlined bodies.
        assert!(
            result.code.contains("Array.isArray"),
            "Default/deserialize should call Array.isArray directly. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("console.error"),
            "Deserialize should call console.error directly. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("Math.floor"),
            "Default should call Math.floor directly. Got:\n{}",
            result.code
        );
    }
}

// ============================================================================
// Foreign Types in Union Type Alias -- Deserialize
// ============================================================================

#[test]
fn test_derive_deserialize_union_with_foreign_type_uses_has_shape() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
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
            vec!["DateTime"],
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

        // Should use the configured deserialize expression, not broken camelCase helpers
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign type deserialize expression. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken dotted deserialize fn. Got:\n{}",
            result.code
        );

        // Should use the hasShape expression for shape matching
        assert!(
            result.code.contains("typeof v === \"string\""),
            "Should use foreign type hasShape expression. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_derive_deserialize_union_foreign_only_types() {
    let source = r#"
import type { DateTime, BigDecimal } from 'effect';

/** @derive(Deserialize) */
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
                vec!["DateTime"],
            ),
            make_foreign_type(
                "BigDecimal.BigDecimal",
                vec!["effect"],
                Some("(v) => BigDecimal.format(v)"),
                Some("(raw) => BigDecimal.fromString(String(raw))"),
                Some("() => BigDecimal.unsafeFromNumber(0)"),
                Some("(v) => typeof v === \"string\" || typeof v === \"number\""),
                vec!["BigDecimal"],
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

        // Both foreign deserialize expressions should be present
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should have DateTime foreign deserialize. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("BigDecimal.fromString")
                || result.code.contains("__mf_BigDecimal.fromString"),
            "Should have BigDecimal foreign deserialize. Got:\n{}",
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
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken DateTime dotted identifier. Got:\n{}",
            result.code
        );
        assert!(
            !result
                .code
                .contains("bigDecimal.bigDecimalDeserializeWithContext"),
            "Should NOT generate broken BigDecimal dotted identifier. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_derive_deserialize_union_foreign_without_has_shape() {
    // Foreign type without hasShape should still use foreign deserialize for __type dispatch
    // but won't participate in shape matching
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
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
            vec!["DateTime"],
        )]);

        let result = expand_test(source);

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should still use foreign deserialize for __type-based dispatch
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign deserialize even without hasShape. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken dotted identifier. Got:\n{}",
            result.code
        );
    }
}

#[test]
fn test_derive_deserialize_union_mixed_foreign_and_primitives() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
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
            vec!["DateTime"],
        )]);

        let result = expand_test(source);

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should use foreign type deserialize
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign type deserialize in mixed union. Got:\n{}",
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
