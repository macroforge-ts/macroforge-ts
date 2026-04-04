#![cfg(feature = "swc")]

use super::applicator::PatchApplicator;
use super::collector::PatchCollector;
use super::helpers::{dedupe_imports, dedupe_patches, parse_import_patch};
use crate::ts_syn::abi::{Patch, PatchCode, SpanIR};

#[test]
fn test_insert_patch() {
    let source = "class Foo {}";
    // Inserting at position 12 (1-based, just before the closing brace at index 11)
    let patch = Patch::Insert {
        at: SpanIR { start: 12, end: 12 },
        code: " bar: string; ".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply().unwrap();
    assert_eq!(result, "class Foo { bar: string; }");
}

#[test]
fn test_replace_patch() {
    let source = "class Foo { old: number; }";
    // Replace "old: number;" with "new: string;" (1-based spans)
    let patch = Patch::Replace {
        span: SpanIR { start: 13, end: 26 },
        code: "new: string;".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply().unwrap();
    assert_eq!(result, "class Foo { new: string;}");
}

#[test]
fn test_delete_patch() {
    let source = "class Foo { unnecessary: any; }";
    // Delete "unnecessary: any;" (1-based spans)
    let patch = Patch::Delete {
        span: SpanIR { start: 13, end: 31 },
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply().unwrap();
    assert_eq!(result, "class Foo { }");
}

#[test]
fn test_multiple_patches() {
    let source = "class Foo {}";
    let patches = vec![
        Patch::Insert {
            at: SpanIR { start: 12, end: 12 },
            code: " bar: string;".to_string().into(),
            source_macro: None,
        },
        Patch::Insert {
            at: SpanIR { start: 12, end: 12 },
            code: " baz: number;".to_string().into(),
            source_macro: None,
        },
    ];

    let applicator = PatchApplicator::new(source, patches);
    let result = applicator.apply().unwrap();
    assert!(result.contains("bar: string"));
    assert!(result.contains("baz: number"));
}

#[test]
fn test_replace_multiline_block_with_single_line() {
    let source = "class C { constructor() { /* body */ } }";
    let constructor_start = source.find("constructor").unwrap();
    let constructor_end = source.find("} }").unwrap() + 1;

    // Convert 0-based indices to 1-based spans
    let patch = Patch::Replace {
        span: SpanIR {
            start: constructor_start as u32 + 1,
            end: constructor_end as u32 + 1,
        },
        code: "constructor();".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply().unwrap();

    let expected = "class C { constructor(); }";
    assert_eq!(result, expected);
}

#[test]
fn test_detect_indentation_spaces() {
    let source = r#"class User {
  id: number;
  name: string;
}"#;
    // Position at closing brace
    let closing_brace_pos = source.rfind('}').unwrap();
    let applicator = PatchApplicator::new(source, vec![]);
    let indent = applicator.detect_indentation(closing_brace_pos);
    // Should detect 2 spaces from the class members
    assert_eq!(indent, "  ");
}

#[test]
fn test_detect_indentation_tabs() {
    let source = "class User {\n\tid: number;\n}";
    let closing_brace_pos = source.rfind('}').unwrap();
    let applicator = PatchApplicator::new(source, vec![]);
    let indent = applicator.detect_indentation(closing_brace_pos);
    // Should detect tab from the class member
    assert_eq!(indent, "\t");
}

#[test]
fn test_format_insertion_adds_newline_and_indent() {
    let source = r#"class User {
  id: number;
}"#;
    let closing_brace_pos = source.rfind('}').unwrap();
    let applicator = PatchApplicator::new(source, vec![]);

    // Simulate a class member insertion
    use swc_core::ecma::ast::{ClassMember, EmptyStmt};
    let code = PatchCode::ClassMember(ClassMember::Empty(EmptyStmt {
        span: swc_core::common::DUMMY_SP,
    }));
    let formatted = applicator.format_insertion("toString(): string;", closing_brace_pos, &code);

    // Should start with newline and have proper indentation
    assert!(formatted.starts_with('\n'));
    assert!(formatted.contains("toString(): string;"));
}

#[test]
fn test_insert_class_member_with_proper_formatting() {
    let source = r#"class User {
  id: number;
  name: string;
}"#;
    // Find position just before closing brace (0-based index)
    let closing_brace_pos = source.rfind('}').unwrap();

    // Create a text patch that simulates what emit_node would produce
    // Convert to 1-based span
    let patch = Patch::Insert {
        at: SpanIR {
            start: closing_brace_pos as u32 + 1,
            end: closing_brace_pos as u32 + 1,
        },
        code: "toString(): string;".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply().unwrap();

    // The result should have the method on its own line with proper indentation
    // Note: Text patches won't get formatted, only ClassMember patches
    // This test verifies the basic insertion works
    assert!(result.contains("toString(): string;"));
}

#[test]
fn test_multiple_class_member_insertions() {
    let source = r#"class User {
  id: number;
}"#;
    let closing_brace_pos = source.rfind('}').unwrap();

    // Convert to 1-based spans
    let patches = vec![
        Patch::Insert {
            at: SpanIR {
                start: closing_brace_pos as u32 + 1,
                end: closing_brace_pos as u32 + 1,
            },
            code: "toString(): string;".to_string().into(),
            source_macro: None,
        },
        Patch::Insert {
            at: SpanIR {
                start: closing_brace_pos as u32 + 1,
                end: closing_brace_pos as u32 + 1,
            },
            code: "toJSON(): Record<string, unknown>;".to_string().into(),
            source_macro: None,
        },
    ];

    let applicator = PatchApplicator::new(source, patches);
    let result = applicator.apply().unwrap();

    assert!(result.contains("toString(): string;"));
    assert!(result.contains("toJSON(): Record<string, unknown>;"));
}

#[test]
fn test_indentation_preserved_in_nested_class() {
    let source = r#"export namespace Models {
  class User {
    id: number;
  }
}"#;
    let closing_brace_pos = source.find("  }").unwrap() + 2; // Find the class closing brace
    let applicator = PatchApplicator::new(source, vec![]);
    let indent = applicator.detect_indentation(closing_brace_pos);
    // Should detect the indentation from the class members (4 spaces)
    assert_eq!(indent, "    ");
}

#[test]
fn test_no_formatting_for_text_patches() {
    let source = "class User {}";
    let pos = 11; // 0-based index for format_insertion (internal use)
    let applicator = PatchApplicator::new(source, vec![]);
    let formatted = applicator.format_insertion("test", pos, &PatchCode::Text("test".to_string()));
    // Text patches should not get extra formatting
    assert_eq!(formatted, "test");
}

#[test]
fn test_dedupe_patches_removes_identical_inserts() {
    // Using 1-based spans
    let mut patches = vec![
        Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: "console.log('a');".to_string().into(),
            source_macro: None,
        },
        Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: "console.log('a');".to_string().into(),
            source_macro: None,
        },
        Patch::Insert {
            at: SpanIR { start: 21, end: 21 },
            code: "console.log('b');".to_string().into(),
            source_macro: None,
        },
    ];

    dedupe_patches(&mut patches).expect("dedupe should succeed");
    assert_eq!(
        patches.len(),
        2,
        "duplicate inserts should collapse to a single patch"
    );
    assert!(
        patches
            .iter()
            .any(|patch| matches!(patch, Patch::Insert { at, .. } if at.start == 21)),
        "dedupe should retain distinct spans"
    );
}

// =========================================================================
// Source Mapping Tests
// =========================================================================

#[test]
fn test_apply_with_mapping_no_patches() {
    let source = "class Foo {}";
    let applicator = PatchApplicator::new(source, vec![]);
    let result = applicator.apply_with_mapping(None).unwrap();

    assert_eq!(result.code, source);
    assert_eq!(result.mapping.segments.len(), 1);
    assert!(result.mapping.generated_regions.is_empty());

    // Identity mapping
    assert_eq!(result.mapping.original_to_expanded(0), 0);
    assert_eq!(result.mapping.original_to_expanded(5), 5);
    assert_eq!(result.mapping.expanded_to_original(5), Some(5));
}

#[test]
fn test_apply_with_mapping_simple_insert() {
    let source = "class Foo {}";
    // Insert at position 12 (1-based span, just before closing brace at index 11)
    let patch = Patch::Insert {
        at: SpanIR { start: 12, end: 12 },
        code: " bar;".to_string().into(),
        source_macro: Some("Test".to_string()),
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply_with_mapping(None).unwrap();

    // Original: "class Foo {}" (12 chars)
    // Expanded: "class Foo { bar;}" (17 chars)
    assert_eq!(result.code, "class Foo { bar;}");
    assert_eq!(result.code.len(), 17);

    // Should have 2 segments and 1 generated region
    assert_eq!(result.mapping.segments.len(), 2);
    assert_eq!(result.mapping.generated_regions.len(), 1);

    // First segment: "class Foo {" (0-based: 0-11)
    let seg1 = &result.mapping.segments[0];
    assert_eq!(seg1.original_start, 0);
    assert_eq!(seg1.original_end, 11);
    assert_eq!(seg1.expanded_start, 0);
    assert_eq!(seg1.expanded_end, 11);

    // Generated region: " bar;" (0-based: 11-16 in expanded)
    let generated = &result.mapping.generated_regions[0];
    assert_eq!(generated.start, 11);
    assert_eq!(generated.end, 16);
    assert_eq!(generated.source_macro, "Test");

    // Second segment: "}" (0-based: 11-12 original -> 16-17 expanded)
    let seg2 = &result.mapping.segments[1];
    assert_eq!(seg2.original_start, 11);
    assert_eq!(seg2.original_end, 12);
    assert_eq!(seg2.expanded_start, 16);
    assert_eq!(seg2.expanded_end, 17);

    // Test position mappings (0-based positions)
    assert_eq!(result.mapping.original_to_expanded(0), 0);
    assert_eq!(result.mapping.original_to_expanded(10), 10);
    assert_eq!(result.mapping.original_to_expanded(11), 16); // After insert

    assert_eq!(result.mapping.expanded_to_original(5), Some(5));
    assert_eq!(result.mapping.expanded_to_original(12), None); // In generated
    assert_eq!(result.mapping.expanded_to_original(16), Some(11));
}

#[test]
fn test_apply_with_mapping_replace() {
    let source = "let x = old;";
    // Replace "old" (1-based span: 9-12) with "new"
    let patch = Patch::Replace {
        span: SpanIR { start: 9, end: 12 },
        code: "new".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply_with_mapping(None).unwrap();

    assert_eq!(result.code, "let x = new;");

    // 2 segments, 1 generated
    assert_eq!(result.mapping.segments.len(), 2);
    assert_eq!(result.mapping.generated_regions.len(), 1);

    // "let x = " unchanged (0-based: 0-8)
    let seg1 = &result.mapping.segments[0];
    assert_eq!(seg1.original_start, 0);
    assert_eq!(seg1.original_end, 8);

    // "new" is generated (0-based: 8-11 in expanded)
    let generated = &result.mapping.generated_regions[0];
    assert_eq!(generated.start, 8);
    assert_eq!(generated.end, 11);

    // ";" unchanged (0-based: 11-12 original -> 11-12 expanded, same length replacement)
    let seg2 = &result.mapping.segments[1];
    assert_eq!(seg2.original_start, 11);
    assert_eq!(seg2.original_end, 12);
    assert_eq!(seg2.expanded_start, 11);
    assert_eq!(seg2.expanded_end, 12);

    // In replaced region
    assert_eq!(result.mapping.expanded_to_original(9), None);
}

#[test]
fn test_apply_with_mapping_delete() {
    let source = "let x = 1; let y = 2;";
    // Delete " let y = 2" (1-based: 11-21)
    let patch = Patch::Delete {
        span: SpanIR { start: 11, end: 21 },
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply_with_mapping(None).unwrap();

    assert_eq!(result.code, "let x = 1;;");

    // 2 segments, no generated regions
    assert_eq!(result.mapping.segments.len(), 2);
    assert_eq!(result.mapping.generated_regions.len(), 0);

    // Position after deletion maps correctly (0-based for SourceMapping API)
    // Original position 20 (final ";") -> expanded position 10
    assert_eq!(result.mapping.original_to_expanded(20), 10);
    assert_eq!(result.mapping.expanded_to_original(10), Some(20));
}

#[test]
fn test_apply_with_mapping_multiple_inserts() {
    let source = "a;b;c;";
    // Insert "X" after "a;" (1-based: 3) and "Y" after "b;" (1-based: 5)
    let patches = vec![
        Patch::Insert {
            at: SpanIR { start: 3, end: 3 },
            code: "X".to_string().into(),
            source_macro: Some("multi".to_string()),
        },
        Patch::Insert {
            at: SpanIR { start: 5, end: 5 },
            code: "Y".to_string().into(),
            source_macro: Some("multi".to_string()),
        },
    ];

    let applicator = PatchApplicator::new(source, patches);
    let result = applicator.apply_with_mapping(None).unwrap();

    // "a;Xb;Yc;"
    assert_eq!(result.code, "a;Xb;Yc;");

    // 3 segments, 2 generated regions
    assert_eq!(result.mapping.segments.len(), 3);
    assert_eq!(result.mapping.generated_regions.len(), 2);

    // Verify position mappings (0-based for SourceMapping API)
    // Original: "a;b;c;" -> Expanded: "a;Xb;Yc;"
    assert_eq!(result.mapping.original_to_expanded(0), 0); // 'a' at 0 -> 0
    assert_eq!(result.mapping.original_to_expanded(2), 3); // 'b' at 2 -> 3 (shifted by 1)
    assert_eq!(result.mapping.original_to_expanded(4), 6); // 'c' at 4 -> 6 (shifted by 2)

    // Verify generated regions (0-based for SourceMapping API)
    // "a;Xb;Yc;" - X is at position 2, Y is at position 5
    assert!(result.mapping.is_in_generated(2)); // 'X'
    assert!(result.mapping.is_in_generated(5)); // 'Y'
    assert!(!result.mapping.is_in_generated(0)); // 'a'
    assert!(!result.mapping.is_in_generated(3)); // 'b'
}

#[test]
fn test_apply_with_mapping_span_mapping() {
    let source = "class Foo {}";
    let patch = Patch::Insert {
        at: SpanIR { start: 12, end: 12 },
        code: " bar();".to_string().into(),
        source_macro: None,
    };

    let applicator = PatchApplicator::new(source, vec![patch]);
    let result = applicator.apply_with_mapping(None).unwrap();

    // Map span from original to expanded (0-based for SourceMapping API)
    let (exp_start, exp_len) = result.mapping.map_span_to_expanded(0, 5);
    assert_eq!(exp_start, 0);
    assert_eq!(exp_len, 5);

    // Map span from expanded to original (in unchanged region, 0-based)
    let orig = result.mapping.map_span_to_original(0, 5);
    assert_eq!(orig, Some((0, 5)));

    // Map span in generated region returns None (0-based)
    // Generated region is at positions 11-18 in expanded (7 chars: " bar();")
    let gen_span = result.mapping.map_span_to_original(12, 3);
    assert_eq!(gen_span, None);
}

#[test]
fn test_patch_collector_with_mapping() {
    let source = "class Foo {}";

    let mut collector = PatchCollector::new();
    collector.add_runtime_patches(vec![Patch::Insert {
        at: SpanIR { start: 12, end: 12 },
        code: " toString() {}".to_string().into(),
        source_macro: Some("Debug".to_string()),
    }]);

    let result = collector
        .apply_runtime_patches_with_mapping(source, None)
        .unwrap();

    assert!(result.code.contains("toString()"));
    assert_eq!(result.mapping.generated_regions.len(), 1);
    assert_eq!(result.mapping.generated_regions[0].source_macro, "Debug");
}

// =========================================================================
// Import Deduplication Tests
// =========================================================================

#[test]
fn test_parse_import_patch_value() {
    let code = "import { Option } from \"effect\";\n";
    let result = parse_import_patch(code);
    assert_eq!(
        result,
        Some(("Option".to_string(), "effect".to_string(), false))
    );
}

#[test]
fn test_parse_import_patch_type_only() {
    let code = "import type { Exit } from \"effect\";\n";
    let result = parse_import_patch(code);
    assert_eq!(
        result,
        Some(("Exit".to_string(), "effect".to_string(), true))
    );
}

#[test]
fn test_parse_import_patch_aliased() {
    let code = "import { Option as __gigaform_reexport_Option } from \"effect\";\n";
    let result = parse_import_patch(code);
    assert_eq!(
        result,
        Some(("Option".to_string(), "effect".to_string(), false))
    );
}

#[test]
fn test_dedupe_imports_value_subsumes_type() {
    let mut patches = vec![
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import type { Exit } from \"effect\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import { Exit } from \"effect\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
    ];

    dedupe_imports(&mut patches);
    assert_eq!(
        patches.len(),
        1,
        "type-only import should be removed when value import exists"
    );
    assert!(patches[0].source_macro().is_none());
    if let Patch::InsertRaw { code, .. } = &patches[0] {
        assert!(
            !code.contains("import type"),
            "remaining import should be the value import"
        );
        assert!(code.contains("import { Exit }"));
    } else {
        panic!("expected InsertRaw");
    }
}

#[test]
fn test_dedupe_imports_keeps_unrelated() {
    let mut patches = vec![
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import type { FieldController } from \"$lib/types/gigaform\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import { Option } from \"effect\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
    ];

    dedupe_imports(&mut patches);
    assert_eq!(patches.len(), 2, "unrelated imports should both be kept");
}

#[test]
fn test_dedupe_imports_different_modules_kept() {
    let mut patches = vec![
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import type { Option } from \"effect/Option\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
        Patch::InsertRaw {
            at: SpanIR { start: 1, end: 1 },
            code: "import { Option } from \"effect\";\n".to_string(),
            context: Some("import".to_string()),
            source_macro: None,
        },
    ];

    dedupe_imports(&mut patches);
    // Different modules -- type import from "effect/Option" is NOT subsumed by value import from "effect"
    assert_eq!(
        patches.len(),
        2,
        "imports from different modules should both be kept"
    );
}
