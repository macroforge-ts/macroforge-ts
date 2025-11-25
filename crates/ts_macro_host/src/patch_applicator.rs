//! Patch application engine for applying macro-generated patches to source code

use crate::error::{MacroError, Result};
use std::collections::HashSet;
use ts_macro_abi::{Patch, SpanIR};

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
            result = self.apply_single_patch(result, patch)?;
        }

        Ok(result)
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

    /// Apply a single patch to the source
    fn apply_single_patch(&self, mut source: String, patch: &Patch) -> Result<String> {
        // Convert byte offsets to char indices for proper string manipulation
        let char_indices: Vec<(usize, char)> = source.char_indices().collect();

        match patch {
            Patch::Insert { at, code } => {
                let byte_pos = self.span_to_byte_offset(&char_indices, at.start as usize)?;
                source.insert_str(byte_pos, code);
            }
            Patch::Replace { span, code } => {
                let start = self.span_to_byte_offset(&char_indices, span.start as usize)?;
                let end = self.span_to_byte_offset(&char_indices, span.end as usize)?;

                // Replace the range with the new code
                source.replace_range(start..end, code);
            }
            Patch::Delete { span } => {
                let start = self.span_to_byte_offset(&char_indices, span.start as usize)?;
                let end = self.span_to_byte_offset(&char_indices, span.end as usize)?;

                // Delete the range
                source.replace_range(start..end, "");
            }
        }

        Ok(source)
    }

    /// Convert a span offset to a byte offset in the string
    fn span_to_byte_offset(&self, char_indices: &[(usize, char)], offset: usize) -> Result<usize> {
        if offset > char_indices.len() {
            return Err(MacroError::Other(anyhow::anyhow!(
                "Span offset {} exceeds source length {}",
                offset,
                char_indices.len()
            )));
        }

        if offset == char_indices.len() {
            // End of string
            Ok(self.source.len())
        } else {
            Ok(char_indices[offset].0)
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
        dedupe_patches(&mut patches);
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    /// Apply type patches to type declaration source
    pub fn apply_type_patches(&self, source: &str) -> Result<String> {
        if self.type_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.type_patches.clone();
        dedupe_patches(&mut patches);
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }
}

impl Default for PatchCollector {
    fn default() -> Self {
        Self::new()
    }
}

fn dedupe_patches(patches: &mut Vec<Patch>) {
    let mut seen: HashSet<(u8, u32, u32, Option<String>)> = HashSet::new();
    patches.retain(|patch| {
        let key = match patch {
            Patch::Insert { at, code } => (0, at.start, at.end, Some(code.clone())),
            Patch::Replace { span, code } => (1, span.start, span.end, Some(code.clone())),
            Patch::Delete { span } => (2, span.start, span.end, None),
        };
        seen.insert(key)
    });
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
            code: " bar: string; ".to_string(),
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
            code: "new: string;".to_string(),
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
                code: " bar: string;".to_string(),
            },
            Patch::Insert {
                at: SpanIR { start: 11, end: 11 },
                code: " baz: number;".to_string(),
            },
        ];

        let applicator = PatchApplicator::new(source, patches);
        let result = applicator.apply().unwrap();
        assert!(result.contains("bar: string"));
        assert!(result.contains("baz: number"));
    }
}
