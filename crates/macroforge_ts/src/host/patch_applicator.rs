//! Patch application engine for applying macro-generated patches to source code

use super::error::{MacroError, Result};
use std::collections::HashSet;
use swc_core::{
    common::{SourceMap, sync::Lrc},
    ecma::codegen::{Config, Emitter, Node, text_writer::JsWriter},
};
use ts_syn::abi::{GeneratedRegion, MappingSegment, Patch, PatchCode, SourceMapping, SpanIR};

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
                Patch::Insert { at, code, .. } => {
                    let rendered = render_patch_code(code)?;
                    let formatted =
                        self.format_insertion(&rendered, at.start.saturating_sub(1) as usize, code);
                    // Safety: ensure index is within bounds
                    let idx = at.start.saturating_sub(1) as usize;
                    if idx <= result.len() {
                        result.insert_str(idx, &formatted);
                    }
                }
                Patch::InsertRaw { at, code, .. } => {
                    let idx = at.start.saturating_sub(1) as usize;
                    if idx <= result.len() {
                        result.insert_str(idx, code);
                    }
                }
                Patch::Replace { span, code, .. } => {
                    let rendered = render_patch_code(code)?;
                    let start = span.start.saturating_sub(1) as usize;
                    let end = span.end.saturating_sub(1) as usize;
                    if start <= end && end <= result.len() {
                        result.replace_range(start..end, &rendered);
                    }
                }
                Patch::ReplaceRaw { span, code, .. } => {
                    let start = span.start.saturating_sub(1) as usize;
                    let end = span.end.saturating_sub(1) as usize;
                    if start <= end && end <= result.len() {
                        result.replace_range(start..end, code);
                    }
                }
                Patch::Delete { span } => {
                    let start = span.start.saturating_sub(1) as usize;
                    let end = span.end.saturating_sub(1) as usize;
                    if start <= end && end <= result.len() {
                        result.replace_range(start..end, "");
                    }
                }
            }
        }

        Ok(result)
    }

    /// Apply all patches and return both the modified source code and source mapping.
    ///
    /// The `fallback_macro_name` is used when a patch doesn't have its own `source_macro` set.
    pub fn apply_with_mapping(mut self, fallback_macro_name: Option<&str>) -> Result<ApplyResult> {
        // Sort patches by position (forward order for mapping generation)
        self.sort_patches();

        // Validate patches don't overlap
        self.validate_no_overlaps()?;

        // If no patches, return identity mapping (0-based positions for TS API)
        if self.patches.is_empty() {
            let source_len = self.source.len() as u32;
            let mut mapping = SourceMapping::new();
            if source_len > 0 {
                // 0-based: position 0 to source_len (exclusive end)
                mapping.add_segment(MappingSegment::new(0, source_len, 0, source_len));
            }
            return Ok(ApplyResult {
                code: self.source.to_string(),
                mapping,
            });
        }

        let mut result = String::new();
        let mut mapping = SourceMapping::with_capacity(self.patches.len() + 1, self.patches.len());

        // Track positions: internally use 1-based (matching SWC spans),
        // but convert to 0-based when creating MappingSegments (matching TS API)
        let mut original_pos: u32 = 1; // 1-based position (start of file)
        let mut expanded_pos: u32 = 1; // 1-based position
        let source_len = self.source.len() as u32;
        let source_end_pos = source_len + 1; // 1-based position after last char
        let default_macro_name = fallback_macro_name.unwrap_or("macro");

        for patch in &self.patches {
            // Helper closure to copy unchanged content
            let mut copy_unchanged = |upto: u32| {
                if upto > original_pos {
                    let len = upto - original_pos;
                    let start = original_pos.saturating_sub(1) as usize;
                    let end = upto.saturating_sub(1) as usize;

                    if end <= self.source.len() {
                        let unchanged = &self.source[start..end];
                        result.push_str(unchanged);

                        // Create 0-based segment for SourceMapping API
                        mapping.add_segment(MappingSegment::new(
                            original_pos - 1,       // Convert to 0-based
                            upto - 1,               // Convert to 0-based
                            expanded_pos - 1,       // Convert to 0-based
                            expanded_pos + len - 1, // Convert to 0-based
                        ));

                        expanded_pos += len;
                        original_pos = upto;
                    }
                }
            };

            // Get the macro name for this patch (use per-patch source_macro if available, else fallback)
            let macro_attribution = patch.source_macro().unwrap_or(default_macro_name);

            match patch {
                Patch::Insert { at, code, .. } => {
                    copy_unchanged(at.start);

                    let rendered = render_patch_code(code)?;
                    let formatted =
                        self.format_insertion(&rendered, at.start.saturating_sub(1) as usize, code);
                    let gen_len = formatted.len() as u32;

                    result.push_str(&formatted);
                    // Create 0-based generated region
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos - 1,
                        expanded_pos - 1 + gen_len,
                        macro_attribution,
                    ));
                    expanded_pos += gen_len;
                }
                Patch::InsertRaw { at, code, .. } => {
                    copy_unchanged(at.start);

                    let gen_len = code.len() as u32;
                    result.push_str(code);
                    // Create 0-based generated region
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos - 1,
                        expanded_pos - 1 + gen_len,
                        macro_attribution,
                    ));
                    expanded_pos += gen_len;
                }
                Patch::Replace { span, code, .. } => {
                    copy_unchanged(span.start);

                    let rendered = render_patch_code(code)?;
                    let gen_len = rendered.len() as u32;

                    result.push_str(&rendered);
                    // Create 0-based generated region
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos - 1,
                        expanded_pos - 1 + gen_len,
                        macro_attribution,
                    ));

                    expanded_pos += gen_len;
                    original_pos = span.end;
                }
                Patch::Delete { span } => {
                    copy_unchanged(span.start);
                    // Skip content
                    original_pos = span.end;
                }
                Patch::ReplaceRaw { span, code, .. } => {
                    copy_unchanged(span.start);

                    let gen_len = code.len() as u32;
                    result.push_str(code);
                    // Create 0-based generated region
                    mapping.add_generated(GeneratedRegion::new(
                        expanded_pos - 1,
                        expanded_pos - 1 + gen_len,
                        macro_attribution,
                    ));
                    expanded_pos += gen_len;
                    original_pos = span.end;
                }
            }
        }

        // Copy any remaining unchanged content after the last patch
        if original_pos < source_end_pos {
            let len = source_end_pos - original_pos;
            let start = original_pos.saturating_sub(1) as usize;
            let remaining = &self.source[start..]; // safe slice to end
            result.push_str(remaining);

            // Create 0-based segment
            mapping.add_segment(MappingSegment::new(
                original_pos - 1,
                source_end_pos - 1,
                expanded_pos - 1,
                expanded_pos - 1 + len,
            ));
        }

        Ok(ApplyResult {
            code: result,
            mapping,
        })
    }

    /// Format an insertion with proper indentation and newlines
    fn format_insertion(&self, rendered: &str, position: usize, code: &PatchCode) -> String {
        if !matches!(code, PatchCode::ClassMember(_)) {
            return rendered.to_string();
        }

        let indent = self.detect_indentation(position);
        format!("\n{}{}\n", indent, rendered.trim())
    }

    /// Detect indentation level at a given position by looking backwards
    fn detect_indentation(&self, position: usize) -> String {
        let bytes = self.source.as_bytes();
        let mut search_pos = position.saturating_sub(1);
        let mut found_indent: Option<String> = None;
        let search_limit = position.saturating_sub(500);

        while search_pos > search_limit && search_pos < bytes.len() {
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

            if line_start >= line_end {
                if line_start == 0 {
                    break;
                }
                search_pos = line_start - 1;
                continue;
            }

            let line = &self.source[line_start..line_end];
            let trimmed = line.trim();

            if !trimmed.is_empty()
                && !trimmed.starts_with('}')
                && !trimmed.starts_with('@')
                && (trimmed.contains(':')
                    || trimmed.contains('(')
                    || trimmed.starts_with("constructor"))
            {
                let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();
                if indent_count > 0 {
                    found_indent = Some(line.chars().take(indent_count).collect());
                    break;
                }
            }

            if line_start == 0 {
                break;
            }
            search_pos = line_start - 1;
        }

        found_indent.unwrap_or_else(|| "  ".to_string())
    }

    fn sort_patches(&mut self) {
        self.patches.sort_by_key(|patch| match patch {
            Patch::Insert { at, .. } => at.start,
            Patch::InsertRaw { at, .. } => at.start,
            Patch::Replace { span, .. } => span.start,
            Patch::ReplaceRaw { span, .. } => span.start,
            Patch::Delete { span } => span.start,
        });
    }

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

    fn patches_overlap(&self, a: &Patch, b: &Patch) -> bool {
        let a_span = self.get_patch_span(a);
        let b_span = self.get_patch_span(b);
        !(a_span.end <= b_span.start || b_span.end <= a_span.start)
    }

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

    pub fn add_runtime_patches(&mut self, patches: Vec<Patch>) {
        self.runtime_patches.extend(patches);
    }

    pub fn add_type_patches(&mut self, patches: Vec<Patch>) {
        self.type_patches.extend(patches);
    }

    pub fn has_type_patches(&self) -> bool {
        !self.type_patches.is_empty()
    }

    pub fn apply_runtime_patches(&self, source: &str) -> Result<String> {
        if self.runtime_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.runtime_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    pub fn apply_type_patches(&self, source: &str) -> Result<String> {
        if self.type_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.type_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    pub fn apply_runtime_patches_with_mapping(
        &self,
        source: &str,
        macro_name: Option<&str>,
    ) -> Result<ApplyResult> {
        if self.runtime_patches.is_empty() {
            // ... (Empty logic same as before)
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

/// Fixed dedupe logic: Separate key generation from filtering
fn dedupe_patches(patches: &mut Vec<Patch>) -> Result<()> {
    let mut seen: HashSet<(u8, u32, u32, Option<String>)> = HashSet::new();
    let mut indices_to_keep = Vec::new();

    for (i, patch) in patches.iter().enumerate() {
        // We calculate the key here. If rendering fails, we can either error out
        // or choose to keep the patch. Here we propagate the error.
        let key = match patch {
            Patch::Insert { at, code, .. } => (0, at.start, at.end, Some(render_patch_code(code)?)),
            Patch::InsertRaw { at, code, .. } => (3, at.start, at.end, Some(code.clone())),
            Patch::Replace { span, code, .. } => {
                (1, span.start, span.end, Some(render_patch_code(code)?))
            }
            Patch::ReplaceRaw { span, code, .. } => (4, span.start, span.end, Some(code.clone())),
            Patch::Delete { span } => (2, span.start, span.end, None),
        };

        if seen.insert(key) {
            indices_to_keep.push(i);
        }
    }

    // Reconstruct the vector using only the unique indices
    // This is more efficient than remove() in a loop for large vectors
    let old_patches = std::mem::take(patches);
    *patches = indices_to_keep
        .into_iter()
        .map(|i| old_patches[i].clone())
        .collect();

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
            .map_err(|err| anyhow::anyhow!(err))?;
    }
    let output = String::from_utf8(buf).map_err(|err| anyhow::anyhow!(err))?;
    // Trim trailing whitespace and newlines from the emitted code
    Ok(output.trim_end().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

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
        let formatted =
            applicator.format_insertion("test", pos, &PatchCode::Text("test".to_string()));
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
}
