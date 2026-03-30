use super::*;

#[test]
fn test_source_mapping_produced() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "Expansion should report changes");

        // Source mapping should be produced
        let mapping = result
            .source_mapping
            .expect("Source mapping should be produced");

        // Should have segments for unchanged regions
        assert!(!mapping.segments.is_empty(), "Should have mapping segments");

        // Should have a generated region for the toString implementation
        assert!(
            !mapping.generated_regions.is_empty(),
            "Should have generated regions"
        );
    });
}

#[test]
fn parse_import_sources_handles_aliases_and_defaults() {
    let code = r#"
import { Derive, Debug as Dbg } from "@macro/derive";
import DefaultMacro from "@macro/default";
import * as Everything from "@macro/all";
"#;

    let imports = parse_import_sources(code.to_string(), "test.ts".to_string())
        .expect("should parse imports");

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
