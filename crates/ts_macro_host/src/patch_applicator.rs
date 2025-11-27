//! Patch application engine for applying macro-generated patches to source code

use crate::error::{MacroError, Result};
use std::collections::HashSet;
use swc_core::{
    common::{SourceMap, sync::Lrc},
    ecma::codegen::{Config, Emitter, Node, text_writer::JsWriter},
};
use ts_macro_abi::{Patch, PatchCode, SpanIR};

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
                Patch::Replace { span, code } => {
                    let rendered = render_patch_code(code)?;
                    result.replace_range(span.start as usize..span.end as usize, &rendered);
                }
                Patch::Delete { span } => {
                    result.replace_range(span.start as usize..span.end as usize, "");
                }
            }
        }

        Ok(result)
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
        format!("\n{}{}", indent, rendered.trim())
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
            Patch::Replace { span, .. } => span.start,
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
            Patch::Replace { span, .. } => *span,
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
            Patch::Replace { span, code } => match render_patch_code(code) {
                Ok(rendered) => (1, span.start, span.end, Some(rendered)),
                Err(_) => return true,
            },
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
        let formatted = applicator.format_insertion("test", pos, &PatchCode::Text("test".to_string()));
        // Text patches should not get extra formatting
        assert_eq!(formatted, "test");
    }
}
