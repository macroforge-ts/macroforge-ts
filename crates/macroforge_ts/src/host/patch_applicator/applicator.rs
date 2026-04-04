use super::helpers::render_patch_code;
use crate::host::error::{MacroError, Result};
use crate::ts_syn::abi::{
    GeneratedRegion, MappingSegment, Patch, PatchCode, SourceMapping, SpanIR,
};

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
    pub(crate) fn format_insertion(
        &self,
        rendered: &str,
        position: usize,
        code: &PatchCode,
    ) -> String {
        #[cfg(not(feature = "swc"))]
        {
            let _ = (position, code);
            return rendered.to_string();
        }

        #[cfg(feature = "swc")]
        if !matches!(code, PatchCode::ClassMember(_)) {
            return rendered.to_string();
        }

        #[cfg(feature = "swc")]
        let indent = self.detect_indentation(position);
        #[cfg(feature = "swc")]
        format!("\n{}{}\n", indent, rendered.trim())
    }

    /// Detect indentation level at a given position by looking backwards
    #[cfg(feature = "swc")]
    pub(crate) fn detect_indentation(&self, position: usize) -> String {
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
