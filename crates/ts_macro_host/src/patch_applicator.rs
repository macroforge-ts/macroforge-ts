//! Patch application engine for applying macro-generated patches to source code

use crate::error::{MacroError, Result};
use std::collections::HashSet;
use swc_core::{
    common::{SourceMap, sync::Lrc},
    ecma::codegen::{Config, Emitter, Node, text_writer::JsWriter},
};
use ts_macro_abi::{GeneratedRegion, MappingSegment, Patch, PatchCode, SourceMapping, SpanIR};

/// Result of applying patches with source mapping
#[derive(Clone, Debug)]
pub struct ApplyResult {
    /// The transformed source code
    pub code: String,
    /// Bidirectional source mapping between original and expanded positions
    pub mapping: SourceMapping,
}

/// Applies patches to source code
pub struct PatchApplicator<'a> {
    source: &'a str,
    patches: Vec<Patch>,
}

impl<'a> PatchApplicator<'a> {
    /// Create a new patch applicator
    pub fn new(source: &'a str, patches: Vec<Patch>) -> Self {
        Self { source, patches }
    }

    /// Apply all patches and return the modified source code
    pub fn apply(mut self) -> Result<String> {
        // Sort patches by position (reverse order for proper application)
        self.sort_patches();

        // Validate patches don't overlap
        self.validate_no_overlaps()?;

        // Apply patches in reverse order (from end to start)
        let mut result = self.source.to_string();

        for patch in self.patches.iter().rev() {
            match patch {
                Patch::Insert { at, code } => {
                    let rendered = render_patch_code(code)?;
                    let formatted = self.format_insertion(&rendered, at.start as usize, code);
                    result.insert_str(at.start as usize, &formatted);
                }
                Patch::InsertRaw { at, code, .. } => {
                    result.insert_str(at.start as usize, code);
                }
                Patch::Replace { span, code } => {
                    let rendered = render_patch_code(code)?;
                    result.replace_range(span.start as usize..span.end as usize, &rendered);
                }
                Patch::ReplaceRaw { span, code, .. } => {
                    result.replace_range(span.start as usize..span.end as usize, code);
                }
                Patch::Delete { span } => {
                    result.replace_range(span.start as usize..span.end as usize, "");
                }
            }
        }

        Ok(result)
    }

    /// Apply all patches and return both the modified source code and source mapping.
    ///
    /// The source mapping enables bidirectional position translation between
    /// original source positions and expanded source positions.
    ///
    /// # Arguments
    /// * `macro_name` - Optional name of the macro generating the patches (for attribution)
    pub fn apply_with_mapping(mut self, macro_name: Option<&str>) -> Result<ApplyResult> {
        // Sort patches by position (forward order for mapping generation)
        self.sort_patches();

        // Validate patches don't overlap
        self.validate_no_overlaps()?;

        // If no patches, return identity mapping
        if self.patches.is_empty() {
            let source_len = self.source.len() as u32;
            let mut mapping = SourceMapping::new();
            if source_len > 0 {
                mapping.add_segment(MappingSegment::new(0, source_len, 0, source_len));
            }
            return Ok(ApplyResult {
                code: self.source.to_string(),
                mapping,
            });
        }

        let mut result = String::new();
        let mut mapping = SourceMapping::with_capacity(self.patches.len() + 1, self.patches.len());

        let mut original_pos: u32 = 0;
        let mut expanded_pos: u32 = 0;
        let source_len = self.source.len() as u32;
        let macro_attribution = macro_name.unwrap_or("macro");

        for patch in &self.patches {
            match patch {
                Patch::Insert { at, code } => {
                    // Copy unchanged content before insertion point
                    if at.start > original_pos {
                        let len = at.start - original_pos;
                        let unchanged = &self.source[original_pos as usize..at.start as usize];
                        result.push_str(unchanged);

                        mapping.add_segment(MappingSegment::new(
                            original_pos,
                            at.start,
                            expanded_pos,
                            expanded_pos + len,
                        ));

                        expanded_pos += len;
                        original_pos = at.start;
                    }

                    // Insert generated code
                    let rendered = render_patch_code(code)?;
                    let formatted = self.format_insertion(&rendered, at.start as usize, code);
                    let gen_len = formatted.len() as u32;

                    result.push_str(&formatted);

                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos,
                        expanded_pos + gen_len,
                        macro_attribution,
                    ));

                    expanded_pos += gen_len;
                    // original_pos stays the same (we didn't consume any original content)
                }
                Patch::InsertRaw { at, code, context: _ } => {
                    if at.start > original_pos {
                        let len = at.start - original_pos;
                        let unchanged = &self.source[original_pos as usize..at.start as usize];
                        result.push_str(unchanged);

                        mapping.add_segment(MappingSegment::new(
                            original_pos,
                            at.start,
                            expanded_pos,
                            expanded_pos + len,
                        ));

                        expanded_pos += len;
                        original_pos = at.start;
                    }

                    let gen_len = code.len() as u32;
                    result.push_str(code);
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos,
                        expanded_pos + gen_len,
                        macro_attribution,
                    ));
                    expanded_pos += gen_len;
                }
                Patch::Replace { span, code } => {
                    // Copy unchanged content before replacement
                    if span.start > original_pos {
                        let len = span.start - original_pos;
                        let unchanged = &self.source[original_pos as usize..span.start as usize];
                        result.push_str(unchanged);

                        mapping.add_segment(MappingSegment::new(
                            original_pos,
                            span.start,
                            expanded_pos,
                            expanded_pos + len,
                        ));

                        expanded_pos += len;
                    }

                    // Insert replacement code (this is generated)
                    let rendered = render_patch_code(code)?;
                    let gen_len = rendered.len() as u32;

                    result.push_str(&rendered);

                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos,
                        expanded_pos + gen_len,
                        macro_attribution,
                    ));

                    expanded_pos += gen_len;
                    original_pos = span.end; // Skip the replaced content
                }
                Patch::Delete { span } => {
                    // Copy unchanged content before deletion
                    if span.start > original_pos {
                        let len = span.start - original_pos;
                        let unchanged = &self.source[original_pos as usize..span.start as usize];
                        result.push_str(unchanged);

                        mapping.add_segment(MappingSegment::new(
                            original_pos,
                            span.start,
                            expanded_pos,
                            expanded_pos + len,
                        ));

                        expanded_pos += len;
                    }

                    // Skip deleted content (no output, no generated region)
                    original_pos = span.end;
                }
                Patch::ReplaceRaw { span, code, context: _ } => {
                    if span.start > original_pos {
                        let len = span.start - original_pos;
                        let unchanged = &self.source[original_pos as usize..span.start as usize];
                        result.push_str(unchanged);

                        mapping.add_segment(MappingSegment::new(
                            original_pos,
                            span.start,
                            expanded_pos,
                            expanded_pos + len,
                        ));

                        expanded_pos += len;
                    }

                    let gen_len = code.len() as u32;
                    result.push_str(code);
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos,
                        expanded_pos + gen_len,
                        macro_attribution,
                    ));
                    expanded_pos += gen_len;
                    original_pos = span.end;
                }
            }
        }

        // Copy any remaining unchanged content after the last patch
        if original_pos < source_len {
            let len = source_len - original_pos;
            let remaining = &self.source[original_pos as usize..];
            result.push_str(remaining);

            mapping.add_segment(MappingSegment::new(
                original_pos,
                source_len,
                expanded_pos,
                expanded_pos + len,
            ));
        }

        Ok(ApplyResult {
            code: result,
            mapping,
        })
    }

    /// Format an insertion with proper indentation and newlines
    fn format_insertion(&self, rendered: &str, position: usize, code: &PatchCode) -> String {
        // Only add formatting for ClassMembers (methods, constructors, etc.)
        if !matches!(code, PatchCode::ClassMember(_)) {
            return rendered.to_string();
        }

        // Detect indentation from the surrounding context
        let indent = self.detect_indentation(position);

        // For class members, we want:
        // 1. A newline before the member
        // 2. Proper indentation
        // 3. The rendered member
        // 4. A newline after (the closing brace should be on its own line)
        format!("\n{}{}\n", indent, rendered.trim())
    }

    /// Detect indentation level at a given position by looking backwards
    fn detect_indentation(&self, position: usize) -> String {
        let bytes = self.source.as_bytes();

        // Look backwards to find a class member or property to match its indentation
        // We search for lines that contain class members (like "  name:" or "  method()")
        let mut search_pos = position.saturating_sub(1);
        let mut found_indent: Option<String> = None;

        // Search backwards up to 500 chars for a properly indented line
        let search_limit = position.saturating_sub(500);

        while search_pos > search_limit {
            // Find the start of this line
            let mut line_start = search_pos;
            while line_start > 0 && bytes[line_start - 1] != b'\n' {
                line_start -= 1;
            }

            // Find the end of this line
            let mut line_end = search_pos;
            while line_end < bytes.len() && bytes[line_end] != b'\n' {
                line_end += 1;
            }

            // Get the line content
            let line = &self.source[line_start..line_end];

            // Look for lines that are class members (have some content after whitespace)
            // Skip empty lines and lines that are just braces
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && !trimmed.starts_with('}')
                && !trimmed.starts_with('@')  // Skip decorators
                && (trimmed.contains(':') || trimmed.contains('(') || trimmed.starts_with("constructor"))
            {
                // Count leading whitespace
                let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();
                if indent_count > 0 {
                    found_indent = Some(line.chars().take(indent_count).collect());
                    break;
                }
            }

            // Move to previous line
            if line_start == 0 {
                break;
            }
            search_pos = line_start - 1;
        }

        // Return the found indentation, or default to 2 spaces
        found_indent.unwrap_or_else(|| "  ".to_string())
    }

    /// Sort patches by their position (start offset)
    fn sort_patches(&mut self) {
        self.patches.sort_by_key(|patch| match patch {
            Patch::Insert { at, .. } => at.start,
            Patch::InsertRaw { at, .. } => at.start,
            Patch::Replace { span, .. } => span.start,
            Patch::ReplaceRaw { span, .. } => span.start,
            Patch::Delete { span } => span.start,
        });
    }

    /// Validate that patches don't overlap
    fn validate_no_overlaps(&self) -> Result<()> {
        for i in 0..self.patches.len() {
            for j in i + 1..self.patches.len() {
                if self.patches_overlap(&self.patches[i], &self.patches[j]) {
                    return Err(MacroError::Other(anyhow::anyhow!(
                        "Overlapping patches detected: patches cannot modify the same region"
                    )));
                }
            }
        }
        Ok(())
    }

    /// Check if two patches overlap
    fn patches_overlap(&self, a: &Patch, b: &Patch) -> bool {
        let a_span = self.get_patch_span(a);
        let b_span = self.get_patch_span(b);

        // Check if spans overlap
        !(a_span.end <= b_span.start || b_span.end <= a_span.start)
    }

    /// Get the span affected by a patch
    fn get_patch_span(&self, patch: &Patch) -> SpanIR {
        match patch {
            Patch::Insert { at, .. } => *at,
            Patch::InsertRaw { at, .. } => *at,
            Patch::Replace { span, .. } => *span,
            Patch::ReplaceRaw { span, .. } => *span,
            Patch::Delete { span } => *span,
        }
    }
}

/// Builder for collecting and applying patches from multiple macros
pub struct PatchCollector {
    runtime_patches: Vec<Patch>,
    type_patches: Vec<Patch>,
}

impl PatchCollector {
    pub fn new() -> Self {
        Self {
            runtime_patches: Vec::new(),
            type_patches: Vec::new(),
        }
    }

    /// Add runtime patches from a macro result
    pub fn add_runtime_patches(&mut self, patches: Vec<Patch>) {
        self.runtime_patches.extend(patches);
    }

    /// Add type patches from a macro result
    pub fn add_type_patches(&mut self, patches: Vec<Patch>) {
        self.type_patches.extend(patches);
    }

    pub fn has_type_patches(&self) -> bool {
        !self.type_patches.is_empty()
    }

    /// Apply runtime patches to source code
    pub fn apply_runtime_patches(&self, source: &str) -> Result<String> {
        if self.runtime_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.runtime_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    /// Apply type patches to type declaration source
    pub fn apply_type_patches(&self, source: &str) -> Result<String> {
        if self.type_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.type_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    /// Apply runtime patches and return result with source mapping
    pub fn apply_runtime_patches_with_mapping(
        &self,
        source: &str,
        macro_name: Option<&str>,
    ) -> Result<ApplyResult> {
        if self.runtime_patches.is_empty() {
            let source_len = source.len() as u32;
            let mut mapping = SourceMapping::new();
            if source_len > 0 {
                mapping.add_segment(MappingSegment::new(0, source_len, 0, source_len));
            }
            return Ok(ApplyResult {
                code: source.to_string(),
                mapping,
            });
        }
        let mut patches = self.runtime_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply_with_mapping(macro_name)
    }

    /// Apply type patches and return result with source mapping
    pub fn apply_type_patches_with_mapping(
        &self,
        source: &str,
        macro_name: Option<&str>,
    ) -> Result<ApplyResult> {
        if self.type_patches.is_empty() {
            let source_len = source.len() as u32;
            let mut mapping = SourceMapping::new();
            if source_len > 0 {
                mapping.add_segment(MappingSegment::new(0, source_len, 0, source_len));
            }
            return Ok(ApplyResult {
                code: source.to_string(),
                mapping,
            });
        }
        let mut patches = self.type_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply_with_mapping(macro_name)
    }

    pub fn get_type_patches(&self) -> &Vec<Patch> {
        &self.type_patches
    }
}

impl Default for PatchCollector {
    fn default() -> Self {
        Self::new()
    }
}

fn dedupe_patches(patches: &mut Vec<Patch>) -> Result<()> {
    let mut seen: HashSet<(u8, u32, u32, Option<String>)> = HashSet::new();
    patches.retain(|patch| {
        let key = match patch {
            Patch::Insert { at, code } => match render_patch_code(code) {
                Ok(rendered) => (0, at.start, at.end, Some(rendered)),
                Err(_) => return true,
            },
            Patch::InsertRaw { at, code, .. } => (3, at.start, at.end, Some(code.clone())),
            Patch::Replace { span, code } => match render_patch_code(code) {
                Ok(rendered) => (1, span.start, span.end, Some(rendered)),
                Err(_) => return true,
            },
            Patch::ReplaceRaw { span, code, .. } => (4, span.start, span.end, Some(code.clone())),
            Patch::Delete { span } => (2, span.start, span.end, None),
        };
        seen.insert(key)
    });
    Ok(())
}

fn render_patch_code(code: &PatchCode) -> Result<String> {
    match code {
        PatchCode::Text(s) => Ok(s.clone()),
        PatchCode::ClassMember(member) => emit_node(member),
        PatchCode::Stmt(stmt) => emit_node(stmt),
        PatchCode::ModuleItem(item) => emit_node(item),
    }
}

fn emit_node<N: Node>(node: &N) -> Result<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    {
        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };
        node.emit_with(&mut emitter)
            .map_err(|err| MacroError::Other(anyhow::anyhow!(err)))?;
    }
    let output = String::from_utf8(buf).map_err(|err| MacroError::Other(anyhow::anyhow!(err)))?;
    // Trim trailing whitespace and newlines from the emitted code
    Ok(output.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_patch() {
        let source = "class Foo {}";
        // Inserting at position 11 (just before the closing brace)
        let patch = Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: " bar: string; ".to_string().into(),
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply().unwrap();
        assert_eq!(result, "class Foo { bar: string; }");
    }

    #[test]
    fn test_replace_patch() {
        let source = "class Foo { old: number; }";
        // Replace "old: number;" with "new: string;"
        let patch = Patch::Replace {
            span: SpanIR { start: 12, end: 25 },
            code: "new: string;".to_string().into(),
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply().unwrap();
        assert_eq!(result, "class Foo { new: string;}");
    }

    #[test]
    fn test_delete_patch() {
        let source = "class Foo { unnecessary: any; }";
        // Delete "unnecessary: any;"
        let patch = Patch::Delete {
            span: SpanIR { start: 12, end: 30 },
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
                at: SpanIR { start: 11, end: 11 },
                code: " bar: string;".to_string().into(),
            },
            Patch::Insert {
                at: SpanIR { start: 11, end: 11 },
                code: " baz: number;".to_string().into(),
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

        let patch = Patch::Replace {
            span: SpanIR {
                start: constructor_start as u32,
                end: constructor_end as u32,
            },
            code: "constructor();".to_string().into(),
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
        let formatted =
            applicator.format_insertion("toString(): string;", closing_brace_pos, &code);

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
        // Find position just before closing brace
        let closing_brace_pos = source.rfind('}').unwrap();

        // Create a text patch that simulates what emit_node would produce
        let patch = Patch::Insert {
            at: SpanIR {
                start: closing_brace_pos as u32,
                end: closing_brace_pos as u32,
            },
            code: "toString(): string;".to_string().into(),
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

        let patches = vec![
            Patch::Insert {
                at: SpanIR {
                    start: closing_brace_pos as u32,
                    end: closing_brace_pos as u32,
                },
                code: "toString(): string;".to_string().into(),
            },
            Patch::Insert {
                at: SpanIR {
                    start: closing_brace_pos as u32,
                    end: closing_brace_pos as u32,
                },
                code: "toJSON(): Record<string, unknown>;".to_string().into(),
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
        let pos = 11;
        let applicator = PatchApplicator::new(source, vec![]);
        let formatted =
            applicator.format_insertion("test", pos, &PatchCode::Text("test".to_string()));
        // Text patches should not get extra formatting
        assert_eq!(formatted, "test");
    }

    #[test]
    fn test_dedupe_patches_removes_identical_inserts() {
        let mut patches = vec![
            Patch::Insert {
                at: SpanIR { start: 10, end: 10 },
                code: "console.log('a');".to_string().into(),
            },
            Patch::Insert {
                at: SpanIR { start: 10, end: 10 },
                code: "console.log('a');".to_string().into(),
            },
            Patch::Insert {
                at: SpanIR { start: 20, end: 20 },
                code: "console.log('b');".to_string().into(),
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
                .any(|patch| matches!(patch, Patch::Insert { at, .. } if at.start == 20)),
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
        // Insert at position 11 (just before closing brace)
        let patch = Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: " bar;".to_string().into(),
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply_with_mapping(Some("Test")).unwrap();

        // Original: "class Foo {}" (12 chars)
        // Expanded: "class Foo { bar;}" (17 chars)
        assert_eq!(result.code, "class Foo { bar;}");
        assert_eq!(result.code.len(), 17);

        // Should have 2 segments and 1 generated region
        assert_eq!(result.mapping.segments.len(), 2);
        assert_eq!(result.mapping.generated_regions.len(), 1);

        // First segment: "class Foo {" (0-11)
        let seg1 = &result.mapping.segments[0];
        assert_eq!(seg1.original_start, 0);
        assert_eq!(seg1.original_end, 11);
        assert_eq!(seg1.expanded_start, 0);
        assert_eq!(seg1.expanded_end, 11);

        // Generated region: " bar;" (11-16 in expanded)
        let generated = &result.mapping.generated_regions[0];
        assert_eq!(generated.start, 11);
        assert_eq!(generated.end, 16);
        assert_eq!(generated.source_macro, "Test");

        // Second segment: "}" (11-12 original -> 16-17 expanded)
        let seg2 = &result.mapping.segments[1];
        assert_eq!(seg2.original_start, 11);
        assert_eq!(seg2.original_end, 12);
        assert_eq!(seg2.expanded_start, 16);
        assert_eq!(seg2.expanded_end, 17);

        // Test position mappings
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
        // Replace "old" (8-11) with "new"
        let patch = Patch::Replace {
            span: SpanIR { start: 8, end: 11 },
            code: "new".to_string().into(),
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply_with_mapping(None).unwrap();

        assert_eq!(result.code, "let x = new;");

        // 2 segments, 1 generated
        assert_eq!(result.mapping.segments.len(), 2);
        assert_eq!(result.mapping.generated_regions.len(), 1);

        // "let x = " unchanged (0-8)
        let seg1 = &result.mapping.segments[0];
        assert_eq!(seg1.original_start, 0);
        assert_eq!(seg1.original_end, 8);

        // "new" is generated (8-11 in expanded)
        let generated = &result.mapping.generated_regions[0];
        assert_eq!(generated.start, 8);
        assert_eq!(generated.end, 11);

        // ";" unchanged (11-12 original -> 11-12 expanded, same length replacement)
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
        // Delete " let y = 2" (10-20)
        let patch = Patch::Delete {
            span: SpanIR { start: 10, end: 20 },
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply_with_mapping(None).unwrap();

        assert_eq!(result.code, "let x = 1;;");

        // 2 segments, no generated regions
        assert_eq!(result.mapping.segments.len(), 2);
        assert_eq!(result.mapping.generated_regions.len(), 0);

        // Position after deletion maps correctly
        assert_eq!(result.mapping.original_to_expanded(20), 10);
        assert_eq!(result.mapping.expanded_to_original(10), Some(20));
    }

    #[test]
    fn test_apply_with_mapping_multiple_inserts() {
        let source = "a;b;c;";
        // Insert "X" after "a;" (position 2) and "Y" after "b;" (position 4)
        let patches = vec![
            Patch::Insert {
                at: SpanIR { start: 2, end: 2 },
                code: "X".to_string().into(),
            },
            Patch::Insert {
                at: SpanIR { start: 4, end: 4 },
                code: "Y".to_string().into(),
            },
        ];

        let applicator = PatchApplicator::new(source, patches);
        let result = applicator.apply_with_mapping(Some("multi")).unwrap();

        // "a;Xb;Yc;"
        assert_eq!(result.code, "a;Xb;Yc;");

        // 3 segments, 2 generated regions
        assert_eq!(result.mapping.segments.len(), 3);
        assert_eq!(result.mapping.generated_regions.len(), 2);

        // Verify position mappings
        assert_eq!(result.mapping.original_to_expanded(0), 0); // 'a'
        assert_eq!(result.mapping.original_to_expanded(2), 3); // 'b' (shifted by 1)
        assert_eq!(result.mapping.original_to_expanded(4), 6); // 'c' (shifted by 2)

        // Verify generated regions
        assert!(result.mapping.is_in_generated(2)); // 'X'
        assert!(result.mapping.is_in_generated(5)); // 'Y'
        assert!(!result.mapping.is_in_generated(0)); // 'a'
        assert!(!result.mapping.is_in_generated(3)); // 'b'
    }

    #[test]
    fn test_apply_with_mapping_span_mapping() {
        let source = "class Foo {}";
        let patch = Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: " bar();".to_string().into(),
        };

        let applicator = PatchApplicator::new(source, vec![patch]);
        let result = applicator.apply_with_mapping(None).unwrap();

        // Map span from original to expanded
        let (exp_start, exp_len) = result.mapping.map_span_to_expanded(0, 5);
        assert_eq!(exp_start, 0);
        assert_eq!(exp_len, 5);

        // Map span from expanded to original (in unchanged region)
        let orig = result.mapping.map_span_to_original(0, 5);
        assert_eq!(orig, Some((0, 5)));

        // Map span in generated region returns None
        let gen_span = result.mapping.map_span_to_original(12, 3);
        assert_eq!(gen_span, None);
    }

    #[test]
    fn test_patch_collector_with_mapping() {
        let source = "class Foo {}";

        let mut collector = PatchCollector::new();
        collector.add_runtime_patches(vec![Patch::Insert {
            at: SpanIR { start: 11, end: 11 },
            code: " toString() {}".to_string().into(),
        }]);

        let result = collector
            .apply_runtime_patches_with_mapping(source, Some("Debug"))
            .unwrap();

        assert!(result.code.contains("toString()"));
        assert_eq!(result.mapping.generated_regions.len(), 1);
        assert_eq!(result.mapping.generated_regions[0].source_macro, "Debug");
    }
}
