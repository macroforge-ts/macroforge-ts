use super::expand_test;
use crate::api::CoreEngine;
use crate::{
    GeneratedRegionResult, MappingSegmentResult, NativePositionMapper, SourceMappingResult,
};

#[test]
fn test_source_mapping_produced() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
    name: string;
}
"#;

    let result = expand_test(source);

    assert!(result.changed, "Expansion should report changes");

    let mapping = result
        .source_mapping
        .expect("Source mapping should be produced");

    // Unchanged regions map through segments.
    assert!(!mapping.segments.is_empty(), "Should have mapping segments");

    // The generated toString implementation is a generated region.
    assert!(
        !mapping.generated_regions.is_empty(),
        "Should have generated regions"
    );
}

#[test]
fn parse_import_sources_handles_aliases_and_defaults() {
    let code = r#"
import { Derive, Debug as Dbg } from "@macro/derive";
import DefaultMacro from "@macro/default";
import * as Everything from "@macro/all";
"#;

    let imports = CoreEngine::parse_import_sources(code, "test.ts").expect("should parse imports");

    let map: std::collections::HashMap<_, _> = imports
        .into_iter()
        .map(|entry| (entry.local, entry.module))
        .collect();

    assert_eq!(map.get("Derive").map(String::as_str), Some("@macro/derive"));
    assert_eq!(map.get("Dbg").map(String::as_str), Some("@macro/derive"));
    assert_eq!(
        map.get("DefaultMacro").map(String::as_str),
        Some("@macro/default")
    );
    assert_eq!(
        map.get("Everything").map(String::as_str),
        Some("@macro/all")
    );
}

#[test]
fn native_position_mapper_matches_js_logic() {
    let mapping = SourceMappingResult {
        segments: vec![
            MappingSegmentResult {
                original_start: 0,
                original_end: 10,
                expanded_start: 0,
                expanded_end: 10,
            },
            MappingSegmentResult {
                original_start: 10,
                original_end: 20,
                expanded_start: 12,
                expanded_end: 22,
            },
        ],
        generated_regions: vec![GeneratedRegionResult {
            start: 10,
            end: 12,
            source_macro: "demo".into(),
        }],
    };

    let mapper = NativePositionMapper::new(mapping);

    assert_eq!(mapper.original_to_expanded(5), 5);
    assert_eq!(mapper.original_to_expanded(15), 17);

    assert_eq!(mapper.expanded_to_original(5), Some(5));
    assert_eq!(mapper.expanded_to_original(17), Some(15));
    assert_eq!(mapper.expanded_to_original(10), None);

    assert!(mapper.is_in_generated(10));
    assert_eq!(mapper.generated_by(11).as_deref(), Some("demo"));
    assert!(mapper.generated_by(25).is_none());

    let span = mapper.map_span_to_original(12, 2).expect("span should map");
    assert_eq!(span.start, 10);
    assert_eq!(span.length, 2);

    let expanded_span = mapper.map_span_to_expanded(8, 4);
    assert_eq!(expanded_span.start, 8);
    assert_eq!(expanded_span.length, 6);

    assert!(!mapper.is_empty());
}

/// The UTF-16 position of `needle` in `text`, as JavaScript would report it.
fn utf16_position(text: &str, needle: &str) -> u32 {
    let byte = text.find(needle).expect("needle is in the text");
    text[..byte].encode_utf16().count() as u32
}

/// Asserts that `needle`, which appears after macro output in `source`, maps
/// from its expanded position back to where it was written.
fn assert_maps_back(source: &str, needle: &str) {
    let result = crate::expand_core::expand_inner(source, "probe.ts", None).unwrap();
    let mapping = result.source_mapping.expect("expansion has a mapping");
    let mapper = NativePositionMapper::new(mapping);
    assert_eq!(
        mapper.expanded_to_original(utf16_position(&result.code, needle)),
        Some(utf16_position(source, needle)),
        "`{needle}` maps back\nexpanded:\n{}",
        result.code
    );
}

#[test]
fn mapping_counts_positions_in_utf16_after_non_ascii_text() {
    assert_maps_back(
        "const café = \"naïve 🚀\";\n/** @derive(Debug) */\ninterface A { a: number }\nconst trailing = 1;\n",
        "const trailing",
    );
}

#[test]
fn mapping_reaches_through_the_pre_passes_and_generated_imports() {
    assert_maps_back(
        "/** @derive(Encode, Decode) */\nexport type Meters = $Newtype<number>;\n\nexport const trailing = 1;\n",
        "export const trailing",
    );
}

/// Asserts that the diagnostic whose message contains `message` spans exactly
/// `needle` in `source`, in 0-based UTF-16 positions.
fn assert_diagnostic_at(source: &str, message: &str, needle: &str) {
    let result = crate::expand_core::expand_inner(source, "probe.ts", None).unwrap();
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.message.contains(message))
        .unwrap_or_else(|| {
            panic!(
                "no diagnostic containing {message:?}: {:?}",
                result.diagnostics
            )
        });
    let start = utf16_position(source, needle);
    let end = start + needle.encode_utf16().count() as u32;
    assert_eq!((diagnostic.start, diagnostic.end), (Some(start), Some(end)));
}

#[test]
fn derive_diagnostics_point_at_the_derive_as_written() {
    assert_diagnostic_at(
        "const café = \"🚀\";\nexport type Id = $Newtype<string>;\n/** @derive(Decode, Default) */\n/** @endec(positive) */\ntype Meters = $Newtype<number>;\n",
        "requires @default",
        "@derive(Decode, Default)",
    );
}

#[test]
fn field_diagnostics_point_at_the_field_annotation_as_written() {
    assert_diagnostic_at(
        "const café = \"🚀\";\n/** @derive(Decode) */\nexport interface User {\n  /** @endec({ validate: [\"doesNotExist\"] }) */\n  name: string;\n}\n",
        "unknown validator",
        "/** @endec({ validate: [\"doesNotExist\"] }) */",
    );
}

#[test]
fn call_macro_diagnostics_point_at_the_call_as_written() {
    assert_diagnostic_at(
        "/** @cfg({ feature: 'never' }) */\nexport const gone = \"é\";\ntype Pair = $Newtype<string, number>;\n",
        "exactly one type argument",
        "$Newtype<string, number>",
    );
}

#[test]
fn import_warnings_point_at_the_import_as_written() {
    assert_diagnostic_at(
        "/** @cfg({ feature: 'never' }) */\nexport const gone = \"é\";\nimport { Debug } from '@macroforge/core';\n/** @derive(Debug) */\nexport class Point {}\n",
        "doesn't need to be imported",
        "Debug",
    );
}

#[test]
fn log_comments_keep_the_mapping_to_the_source() {
    let source = "/** @derive(Decode, Default) */\n/** @endec(positive) */\ntype Meters = $Newtype<number>;\nexport const trailing = 1;\n";
    let host = crate::host::MacroExpander::new().unwrap();
    let mut expansion = host.expand_source(source, "probe.ts").unwrap();
    crate::expand_core::inject_log_comments(&mut expansion, crate::expand_core::LogLevel::Error)
        .unwrap();
    assert!(
        expansion.code.contains("// "),
        "an error was logged into the code"
    );
    let mapping = expansion
        .source_mapping
        .expect("the expansion has a mapping");
    let expanded = expansion.code.find("export const trailing").unwrap() as u32;
    assert_eq!(
        mapping.expanded_to_original(expanded),
        source
            .find("export const trailing")
            .map(|offset| offset as u32)
    );
    let comment = expansion.code.find("// ").unwrap() as u32;
    assert!(
        mapping.is_in_generated(comment),
        "the log comment is generated code"
    );
}
