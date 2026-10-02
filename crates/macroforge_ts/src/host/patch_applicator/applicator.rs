use crate::host::error::{MacroError, Result};
use crate::ts_syn::abi::{GeneratedRegion, MappingSegment, Patch, SourceMapping, SpanIR};

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
        self.sort_patches();
        self.validate_no_overlaps()?;
        self.validate_in_bounds()?;

        // Sorted, disjoint and in bounds, so one forward pass copies each
        // unchanged stretch once instead of shifting the tail at every patch.
        let generated: usize = self
            .patches
            .iter()
            .map(|patch| patch_code(patch).len())
            .sum();
        let mut result = String::with_capacity(self.source.len() + generated);
        let mut cursor = 0;
        for patch in &self.patches {
            let (start, end) = match patch {
                Patch::Insert { at, .. } | Patch::InsertRaw { at, .. } => {
                    let at = at.start.saturating_sub(1) as usize;
                    (at, at)
                }
                Patch::Replace { span, .. }
                | Patch::ReplaceRaw { span, .. }
                | Patch::Delete { span } => (
                    span.start.saturating_sub(1) as usize,
                    span.end.saturating_sub(1) as usize,
                ),
            };
            result.push_str(&self.source[cursor..start]);
            result.push_str(patch_code(patch));
            cursor = end;
        }
        result.push_str(&self.source[cursor..]);

        Ok(result)
    }

    /// Apply all patches and return both the modified source code and source mapping.
    ///
    /// The `fallback_macro_name` is used when a patch doesn't have its own `source_macro` set.
    pub fn apply_with_mapping(mut self, fallback_macro_name: Option<&str>) -> Result<ApplyResult> {
        // Sort patches by position (forward order for mapping generation)
        self.sort_patches();

        // Validate patches don't overlap and stay inside the source
        self.validate_no_overlaps()?;
        self.validate_in_bounds()?;

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

        // Track positions: internally use 1-based (the patch span convention),
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
                Patch::Insert { at, code, .. } | Patch::InsertRaw { at, code, .. } => {
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
                Patch::Delete { span } => {
                    copy_unchanged(span.start);
                    // Skip content
                    original_pos = span.end;
                }
                Patch::Replace { span, code, .. } | Patch::ReplaceRaw { span, code, .. } => {
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

    fn sort_patches(&mut self) {
        self.patches.sort_by_key(|patch| match patch {
            Patch::Insert { at, .. } => at.start,
            Patch::InsertRaw { at, .. } => at.start,
            Patch::Replace { span, .. } => span.start,
            Patch::ReplaceRaw { span, .. } => span.start,
            Patch::Delete { span } => span.start,
        });
    }

    /// Check that no two patches cover overlapping regions of the
    /// source. This assumes [`Self::sort_patches`] has already been
    /// called, so patches appear in ascending `span.start` order; a
    /// linear sweep of adjacent pairs is sufficient (if two patches
    /// overlap, the earlier one's `end` will be greater than the
    /// next one's `start`, and that pair will be adjacent in the
    /// sorted list because any patch between them would have to
    /// start somewhere in `[earlier.start, earlier.end]`, which
    /// means it overlaps earlier too and we'd catch it on the
    /// previous iteration).
    ///
    /// Pre-PR 15 this was `O(n²)` (a nested loop). The linear sweep
    /// is a pure perf fix: same diagnostics, same public API.
    fn validate_no_overlaps(&self) -> Result<()> {
        for pair in self.patches.windows(2) {
            let a = self.get_patch_span(&pair[0]);
            let b = self.get_patch_span(&pair[1]);
            if a.end > b.start {
                return Err(MacroError::Other(anyhow::anyhow!(
                    "Overlapping patches detected: patches cannot modify the same region"
                )));
            }
        }
        Ok(())
    }

    /// Fails on a patch that reaches outside the source, which would
    /// otherwise lose the macro's output or the source around it.
    fn validate_in_bounds(&self) -> Result<()> {
        let end_of_source = self.source.len() as u32 + 1;
        for patch in &self.patches {
            let span = self.get_patch_span(patch);
            let end = match patch {
                Patch::Insert { .. } | Patch::InsertRaw { .. } => span.start,
                Patch::Replace { .. } | Patch::ReplaceRaw { .. } | Patch::Delete { .. } => span.end,
            };
            if span.start > end || end > end_of_source {
                return Err(MacroError::Patch(format!(
                    "{} produced a patch at positions {}..{}, outside the {}-byte source",
                    patch.source_macro().unwrap_or("a macro"),
                    span.start,
                    end,
                    self.source.len()
                )));
            }
        }
        Ok(())
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

/// The code a patch writes; empty for a deletion.
fn patch_code(patch: &Patch) -> &str {
    match patch {
        Patch::Insert { code, .. }
        | Patch::InsertRaw { code, .. }
        | Patch::Replace { code, .. }
        | Patch::ReplaceRaw { code, .. } => code,
        Patch::Delete { .. } => "",
    }
}

#[cfg(test)]
mod overlap_tests {
    //! Overlap-detection regression guards. These cover the linear
    //! sweep introduced in PR 15 of the production-hardening plan
    //! (replacing the prior `O(n²)` nested loop).

    use super::*;
    use crate::ts_syn::abi::{Patch, SpanIR};

    fn delete_patch(start: u32, end: u32) -> Patch {
        Patch::Delete {
            span: SpanIR::new(start, end),
        }
    }

    #[test]
    fn non_overlapping_patches_pass_validation() {
        let source = "x".repeat(100);
        let patches = vec![
            delete_patch(1, 10),
            delete_patch(10, 20),
            delete_patch(20, 30),
        ];
        let applicator = PatchApplicator::new(&source, patches);
        assert!(applicator.apply().is_ok());
    }

    #[test]
    fn a_patch_outside_the_source_is_rejected() {
        let source = "x".repeat(10);
        let too_far = Patch::Insert {
            at: SpanIR::new(20, 20),
            code: "lost".to_string(),
            source_macro: Some("Probe".to_string()),
        };
        for patches in [vec![delete_patch(5, 30)], vec![too_far]] {
            let error = PatchApplicator::new(&source, patches.clone())
                .apply()
                .expect_err("an out-of-range patch must fail");
            assert!(
                error.to_string().contains("outside the 10-byte source"),
                "{error}"
            );
            assert!(
                PatchApplicator::new(&source, patches)
                    .apply_with_mapping(None)
                    .is_err()
            );
        }
    }

    #[test]
    fn a_patch_at_the_end_of_the_source_applies() {
        let insert = Patch::Insert {
            at: SpanIR::new(4, 4),
            code: "!".to_string(),
            source_macro: None,
        };
        let applied = PatchApplicator::new("abc", vec![insert])
            .apply()
            .expect("inserting after the last byte is in bounds");
        assert_eq!(applied, "abc!");
    }

    #[test]
    fn overlapping_patches_are_rejected() {
        let source = "x".repeat(100);
        let patches = vec![
            delete_patch(1, 15),
            // Overlaps the first patch: the second span starts before
            // the first ends.
            delete_patch(10, 20),
        ];
        let applicator = PatchApplicator::new(&source, patches);
        let err = applicator.apply().unwrap_err();
        assert!(
            err.to_string().contains("Overlapping patches"),
            "got: {}",
            err
        );
    }

    #[test]
    fn overlap_detected_across_non_adjacent_pairs_after_sort() {
        // Regression guard for the linear sweep's correctness: if
        // three patches are in an order where a non-adjacent overlap
        // exists before sorting, sorting must bring the overlapping
        // pair next to each other so the adjacent-pair check catches
        // them.
        let source = "x".repeat(100);
        let patches = vec![
            delete_patch(1, 5),   // Sorts first.
            delete_patch(20, 30), // Sorts third.
            delete_patch(3, 25),  // Overlaps both; sorts second.
        ];
        let applicator = PatchApplicator::new(&source, patches);
        assert!(applicator.apply().is_err());
    }

    #[test]
    fn ten_thousand_non_overlapping_patches_validate_quickly() {
        // Stress test: with 10k disjoint patches the old `O(n²)`
        // check would do ~50 million comparisons. The linear sweep
        // does ~10k. The bound is deliberately generous (2 seconds)
        // to catch orders-of-magnitude regressions without flaking
        // on slow / heavily-parallel test runs. In practice the
        // sweep itself is sub-millisecond; most of the elapsed time
        // is the patch application work (deleting 60k bytes from a
        // 200k source).
        let source = "x".repeat(200_000);
        let patches: Vec<Patch> = (0..10_000)
            .map(|i| {
                let start = (i * 10 + 1) as u32;
                delete_patch(start, start + 5)
            })
            .collect();
        let applicator = PatchApplicator::new(&source, patches);
        let start = std::time::Instant::now();
        let result = applicator.apply();
        let elapsed = start.elapsed();
        assert!(result.is_ok(), "apply failed: {:?}", result);
        assert!(
            elapsed.as_millis() < 2000,
            "10k-patch validate+apply took {} ms, an orders-of-magnitude regression?",
            elapsed.as_millis()
        );
    }
}
