//! Source mapping between original and expanded code.
//!
//! When macros expand code, byte positions shift. This module provides
//! bidirectional mapping to translate positions between original source
//! and macro-expanded output.

use serde::{Deserialize, Serialize};

/// A segment mapping a contiguous region from original to expanded positions.
///
/// Represents an unchanged region of code that exists in both original
/// and expanded source at different byte offsets.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MappingSegment {
    /// Start position in original source (byte offset)
    pub original_start: u32,
    /// End position in original source (byte offset, exclusive)
    pub original_end: u32,
    /// Start position in expanded source (byte offset)
    pub expanded_start: u32,
    /// End position in expanded source (byte offset, exclusive)
    pub expanded_end: u32,
}

impl MappingSegment {
    pub fn new(
        original_start: u32,
        original_end: u32,
        expanded_start: u32,
        expanded_end: u32,
    ) -> Self {
        Self {
            original_start,
            original_end,
            expanded_start,
            expanded_end,
        }
    }

    /// Length in original source
    pub fn original_len(&self) -> u32 {
        self.original_end.saturating_sub(self.original_start)
    }

    /// Length in expanded source
    pub fn expanded_len(&self) -> u32 {
        self.expanded_end.saturating_sub(self.expanded_start)
    }

    /// Check if an original position falls within this segment
    pub fn contains_original(&self, pos: u32) -> bool {
        pos >= self.original_start && pos < self.original_end
    }

    /// Check if an expanded position falls within this segment
    pub fn contains_expanded(&self, pos: u32) -> bool {
        pos >= self.expanded_start && pos < self.expanded_end
    }
}

/// A region of generated code that has no corresponding original source.
///
/// This is code that was inserted by a macro and does not map back
/// to any position in the original source file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRegion {
    /// Start position in expanded source (byte offset)
    pub start: u32,
    /// End position in expanded source (byte offset, exclusive)
    pub end: u32,
    /// Which macro generated this code (e.g., "Debug::toString", "JSON::toJSON")
    pub source_macro: String,
}

impl GeneratedRegion {
    pub fn new(start: u32, end: u32, source_macro: impl Into<String>) -> Self {
        Self {
            start,
            end,
            source_macro: source_macro.into(),
        }
    }

    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Check if an expanded position falls within this generated region
    pub fn contains(&self, expanded_pos: u32) -> bool {
        expanded_pos >= self.start && expanded_pos < self.end
    }
}

/// Complete bidirectional source mapping between original and expanded code.
///
/// Provides methods to translate positions in both directions:
/// - `original_to_expanded`: Map from original source position to expanded
/// - `expanded_to_original`: Map from expanded source position to original
///
/// # Example
///
/// ```text
/// Original:  "class Foo { id: string; }"
///                        ^-- position 12
/// Expanded:  "class Foo { id: string; toString() { ... } }"
///                        ^-- position 12 (same)
///                                        ^-- position 24 (generated, no original)
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SourceMapping {
    /// Ordered list of mapped segments (sorted by original_start)
    pub segments: Vec<MappingSegment>,
    /// Regions that have no original source (generated code)
    pub generated_regions: Vec<GeneratedRegion>,
}

impl SourceMapping {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a mapping with pre-allocated capacity
    pub fn with_capacity(segments: usize, generated: usize) -> Self {
        Self {
            segments: Vec::with_capacity(segments),
            generated_regions: Vec::with_capacity(generated),
        }
    }

    /// Add a mapping segment
    pub fn add_segment(&mut self, segment: MappingSegment) {
        self.segments.push(segment);
    }

    /// Add a generated region
    pub fn add_generated(&mut self, region: GeneratedRegion) {
        self.generated_regions.push(region);
    }

    /// Check if any mappings exist
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.generated_regions.is_empty()
    }

    /// Map a position from original source to expanded source.
    ///
    /// Returns the corresponding position in expanded code, or the
    /// original position if no mapping applies (identity mapping for
    /// positions outside all segments).
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        // Binary search could be used for large mappings, but linear is fine for typical sizes
        for seg in &self.segments {
            if seg.contains_original(pos) {
                let offset = pos - seg.original_start;
                return seg.expanded_start + offset;
            }
        }

        // Position after last segment - calculate delta from last known mapping
        if let Some(last) = self.segments.last()
            && pos >= last.original_end
        {
            let delta = pos - last.original_end;
            return last.expanded_end + delta;
        }

        // No mapping found, return as-is (identity)
        pos
    }

    /// Map a position from expanded source to original source.
    ///
    /// Returns `None` if the position is within a generated region
    /// (code that has no original source).
    /// Returns the original position if found, or `None` if the position
    /// cannot be mapped.
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        // Check if in a generated region first
        if self.is_in_generated(pos) {
            return None;
        }

        for seg in &self.segments {
            if seg.contains_expanded(pos) {
                let offset = pos - seg.expanded_start;
                return Some(seg.original_start + offset);
            }
        }

        // Position after last segment
        if let Some(last) = self.segments.last()
            && pos >= last.expanded_end
        {
            let delta = pos - last.expanded_end;
            return Some(last.original_end + delta);
        }

        None
    }

    /// Check if a position in expanded source is within generated code.
    pub fn is_in_generated(&self, expanded_pos: u32) -> bool {
        self.generated_regions
            .iter()
            .any(|r| r.contains(expanded_pos))
    }

    /// Find which macro generated code at a position, if any.
    pub fn generated_by(&self, expanded_pos: u32) -> Option<&str> {
        self.generated_regions
            .iter()
            .find(|r| r.contains(expanded_pos))
            .map(|r| r.source_macro.as_str())
    }

    /// Map a span (start, length) from expanded to original coordinates.
    ///
    /// Returns `None` if either endpoint is in generated code.
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Option<(u32, u32)> {
        let end = start + length;
        let original_start = self.expanded_to_original(start)?;
        let original_end = self.expanded_to_original(end)?;
        Some((original_start, original_end.saturating_sub(original_start)))
    }

    /// Map a span (start, length) from original to expanded coordinates.
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> (u32, u32) {
        let end = start + length;
        let expanded_start = self.original_to_expanded(start);
        let expanded_end = self.original_to_expanded(end);
        (expanded_start, expanded_end.saturating_sub(expanded_start))
    }

    /// Maps a 0-based expanded span `start..end` back to the original source. A
    /// span in generated code becomes an empty span where that code was
    /// inserted. An empty mapping is the identity.
    pub fn span_to_original(&self, start: u32, end: u32) -> (u32, u32) {
        if self.is_empty() {
            return (start, end);
        }
        let mapped_start = self.expanded_to_original(start);
        // The end is exclusive, so it maps through the last character it covers.
        let mapped_end = if end > start {
            self.expanded_to_original(end - 1).map(|last| last + 1)
        } else {
            mapped_start
        };
        match (mapped_start, mapped_end) {
            (Some(original_start), Some(original_end)) if original_start <= original_end => {
                (original_start, original_end)
            }
            _ => {
                let anchor = mapped_start.unwrap_or_else(|| {
                    self.segments
                        .iter()
                        .filter(|segment| segment.expanded_end <= start)
                        .map(|segment| segment.original_end)
                        .max()
                        .unwrap_or(0)
                });
                (anchor, anchor)
            }
        }
    }

    /// This mapping followed by `next`: `self` maps A to B and `next` maps B to
    /// C, and the result maps A to C. Code either step generated stays
    /// generated, attributed to the macro that generated it. An empty mapping
    /// is the identity.
    pub fn compose(&self, next: &SourceMapping) -> SourceMapping {
        if self.is_empty() {
            return next.clone();
        }
        if next.is_empty() {
            return self.clone();
        }
        let mut composed = SourceMapping::new();
        for step in &next.segments {
            // The part of `step`'s B range that `piece` (a B range) covers, as
            // (start in B, length).
            let overlap = |piece_start: u32, piece_end: u32| {
                let start = step.original_start.max(piece_start);
                let end = step.original_end.min(piece_end);
                (start < end).then(|| (start, end - start))
            };
            let to_c = |b: u32| step.expanded_start + (b - step.original_start);
            for earlier in &self.segments {
                if let Some((b, len)) = overlap(earlier.expanded_start, earlier.expanded_end) {
                    let a = earlier.original_start + (b - earlier.expanded_start);
                    composed.add_segment(MappingSegment::new(a, a + len, to_c(b), to_c(b) + len));
                }
            }
            for generated in &self.generated_regions {
                if let Some((b, len)) = overlap(generated.start, generated.end) {
                    composed.add_generated(GeneratedRegion::new(
                        to_c(b),
                        to_c(b) + len,
                        generated.source_macro.clone(),
                    ));
                }
            }
        }
        composed
            .generated_regions
            .extend(next.generated_regions.iter().cloned());
        composed
            .segments
            .sort_by_key(|segment| segment.expanded_start);
        composed
            .generated_regions
            .sort_by_key(|region| region.start);
        composed
    }

    /// This mapping with `prefix_len` bytes, generated by `source_macro`,
    /// prepended to the expanded code.
    pub fn with_prefix(&self, prefix_len: u32, source_macro: &str) -> SourceMapping {
        if prefix_len == 0 {
            return self.clone();
        }
        let mut shifted = SourceMapping::new();
        shifted.add_generated(GeneratedRegion::new(0, prefix_len, source_macro));
        for segment in &self.segments {
            shifted.add_segment(MappingSegment::new(
                segment.original_start,
                segment.original_end,
                segment.expanded_start + prefix_len,
                segment.expanded_end + prefix_len,
            ));
        }
        for region in &self.generated_regions {
            shifted.add_generated(GeneratedRegion::new(
                region.start + prefix_len,
                region.end + prefix_len,
                region.source_macro.clone(),
            ));
        }
        shifted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_mapping() {
        let mapping = SourceMapping::new();
        assert_eq!(mapping.original_to_expanded(0), 0);
        assert_eq!(mapping.original_to_expanded(100), 100);
        assert_eq!(mapping.expanded_to_original(50), None); // No segments
    }

    #[test]
    fn test_simple_insert() {
        // Original: "class Foo { }"  (13 chars)
        // Expanded: "class Foo { toString() {} }"  (27 chars)
        // Insert at position 12 (before closing brace)
        let mut mapping = SourceMapping::new();

        // First segment: "class Foo { " (0-12 in both)
        mapping.add_segment(MappingSegment::new(0, 12, 0, 12));

        // Generated region: "toString() {} " (12-26 in expanded)
        mapping.add_generated(GeneratedRegion::new(12, 26, "Debug::toString"));

        // Second segment: "}" (12-13 original -> 26-27 expanded)
        mapping.add_segment(MappingSegment::new(12, 13, 26, 27));

        // Test original -> expanded
        assert_eq!(mapping.original_to_expanded(0), 0);
        assert_eq!(mapping.original_to_expanded(5), 5);
        assert_eq!(mapping.original_to_expanded(12), 26); // After insert

        // Test expanded -> original
        assert_eq!(mapping.expanded_to_original(0), Some(0));
        assert_eq!(mapping.expanded_to_original(5), Some(5));
        assert_eq!(mapping.expanded_to_original(15), None); // In generated
        assert_eq!(mapping.expanded_to_original(26), Some(12));

        // Test generated detection
        assert!(!mapping.is_in_generated(11));
        assert!(mapping.is_in_generated(12));
        assert!(mapping.is_in_generated(20));
        assert!(!mapping.is_in_generated(26));

        // Test macro attribution
        assert_eq!(mapping.generated_by(15), Some("Debug::toString"));
        assert_eq!(mapping.generated_by(5), None);
    }

    #[test]
    fn test_multiple_inserts() {
        // Original: "a;b;c;"  (6 chars)
        // Expanded: "a;X;b;Y;c;Z;"  (12 chars)
        // Inserts X after position 2, Y after position 4, Z after position 6
        let mut mapping = SourceMapping::new();

        // "a;" at 0-2 -> 0-2
        mapping.add_segment(MappingSegment::new(0, 2, 0, 2));
        // "X;" generated at 2-4
        mapping.add_generated(GeneratedRegion::new(2, 4, "macro1"));
        // "b;" at 2-4 -> 4-6
        mapping.add_segment(MappingSegment::new(2, 4, 4, 6));
        // "Y;" generated at 6-8
        mapping.add_generated(GeneratedRegion::new(6, 8, "macro2"));
        // "c;" at 4-6 -> 8-10
        mapping.add_segment(MappingSegment::new(4, 6, 8, 10));
        // "Z;" generated at 10-12
        mapping.add_generated(GeneratedRegion::new(10, 12, "macro3"));

        // Test mappings
        assert_eq!(mapping.original_to_expanded(0), 0);
        assert_eq!(mapping.original_to_expanded(2), 4);
        assert_eq!(mapping.original_to_expanded(4), 8);

        assert_eq!(mapping.expanded_to_original(0), Some(0));
        assert_eq!(mapping.expanded_to_original(3), None); // In generated
        assert_eq!(mapping.expanded_to_original(4), Some(2));
        assert_eq!(mapping.expanded_to_original(7), None); // In generated
        assert_eq!(mapping.expanded_to_original(8), Some(4));
    }

    #[test]
    fn compose_maps_through_both_steps() {
        // A "ab" -> B "aXb" (X inserted by m1) -> C "aXYb" (Y inserted by m2).
        let mut first = SourceMapping::new();
        first.add_segment(MappingSegment::new(0, 1, 0, 1));
        first.add_generated(GeneratedRegion::new(1, 2, "m1"));
        first.add_segment(MappingSegment::new(1, 2, 2, 3));
        let mut second = SourceMapping::new();
        second.add_segment(MappingSegment::new(0, 2, 0, 2));
        second.add_generated(GeneratedRegion::new(2, 3, "m2"));
        second.add_segment(MappingSegment::new(2, 3, 3, 4));

        let composed = first.compose(&second);
        assert_eq!(composed.expanded_to_original(0), Some(0));
        assert_eq!(composed.generated_by(1), Some("m1"));
        assert_eq!(composed.generated_by(2), Some("m2"));
        assert_eq!(composed.expanded_to_original(3), Some(1));
        assert_eq!(composed.original_to_expanded(1), 3);
    }

    #[test]
    fn compose_with_an_empty_mapping_is_the_other_mapping() {
        let mut mapping = SourceMapping::new();
        mapping.add_segment(MappingSegment::new(0, 4, 2, 6));
        assert_eq!(SourceMapping::new().compose(&mapping), mapping);
        assert_eq!(mapping.compose(&SourceMapping::new()), mapping);
    }

    #[test]
    fn span_to_original_maps_kept_text_and_anchors_generated_text() {
        // "ab" -> "aXXb", XX generated.
        let mut mapping = SourceMapping::new();
        mapping.add_segment(MappingSegment::new(0, 1, 0, 1));
        mapping.add_generated(GeneratedRegion::new(1, 3, "m"));
        mapping.add_segment(MappingSegment::new(1, 2, 3, 4));
        assert_eq!(mapping.span_to_original(3, 4), (1, 2));
        assert_eq!(mapping.span_to_original(0, 1), (0, 1));
        assert_eq!(mapping.span_to_original(1, 3), (1, 1));
        assert_eq!(SourceMapping::new().span_to_original(5, 9), (5, 9));
    }

    #[test]
    fn with_prefix_shifts_everything_after_the_prefix() {
        let mut mapping = SourceMapping::new();
        mapping.add_segment(MappingSegment::new(0, 3, 0, 3));
        let prefixed = mapping.with_prefix(5, "imports");
        assert_eq!(prefixed.generated_by(2), Some("imports"));
        assert_eq!(prefixed.expanded_to_original(6), Some(1));
        assert_eq!(prefixed.original_to_expanded(1), 6);
    }

    #[test]
    fn test_span_mapping() {
        let mut mapping = SourceMapping::new();
        mapping.add_segment(MappingSegment::new(0, 10, 0, 10));
        mapping.add_generated(GeneratedRegion::new(10, 20, "gen"));
        mapping.add_segment(MappingSegment::new(10, 20, 20, 30));

        // Span in unchanged region
        assert_eq!(mapping.map_span_to_expanded(2, 5), (2, 5));
        assert_eq!(mapping.map_span_to_original(2, 5), Some((2, 5)));

        // Span after insert
        assert_eq!(mapping.map_span_to_expanded(12, 3), (22, 3));
        assert_eq!(mapping.map_span_to_original(22, 3), Some((12, 3)));

        // Span in generated region returns None
        assert_eq!(mapping.map_span_to_original(12, 5), None);
    }

    #[test]
    fn test_segment_helpers() {
        let seg = MappingSegment::new(10, 20, 30, 45);
        assert_eq!(seg.original_len(), 10);
        assert_eq!(seg.expanded_len(), 15);
        assert!(seg.contains_original(15));
        assert!(!seg.contains_original(25));
        assert!(seg.contains_expanded(35));
        assert!(!seg.contains_expanded(50));
    }

    #[test]
    fn test_generated_region_helpers() {
        let region = GeneratedRegion::new(10, 25, "Test::method");
        assert_eq!(region.len(), 15);
        assert!(!region.is_empty());
        assert!(region.contains(15));
        assert!(!region.contains(5));
        assert!(!region.contains(25));
    }
}
