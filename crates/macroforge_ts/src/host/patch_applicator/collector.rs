use super::applicator::{ApplyResult, PatchApplicator};
use super::helpers::dedupe_patches;
use crate::host::error::Result;
use crate::ts_syn::abi::{MappingSegment, Patch, SourceMapping};

/// Builder for collecting and applying patches from multiple macros.
///
/// Accumulates runtime patches (`.ts`/`.js` output) and type patches (`.d.ts` output)
/// separately, then applies them with deduplication.
pub struct PatchCollector {
    runtime_patches: Vec<Patch>,
    type_patches: Vec<Patch>,
}

impl PatchCollector {
    /// Create a new empty patch collector.
    pub fn new() -> Self {
        Self {
            runtime_patches: Vec::new(),
            type_patches: Vec::new(),
        }
    }

    /// Append runtime code patches (inserted into `.ts`/`.js` output).
    pub fn add_runtime_patches(&mut self, patches: Vec<Patch>) {
        self.runtime_patches.extend(patches);
    }

    /// Append type-level patches (inserted into `.d.ts` output).
    pub fn add_type_patches(&mut self, patches: Vec<Patch>) {
        self.type_patches.extend(patches);
    }

    /// Returns `true` if any type-level patches have been collected.
    pub fn has_type_patches(&self) -> bool {
        !self.type_patches.is_empty()
    }

    /// Returns `true` if any patches (runtime or type) have been collected.
    pub fn has_patches(&self) -> bool {
        !self.runtime_patches.is_empty() || !self.type_patches.is_empty()
    }

    /// Returns the number of runtime patches collected so far.
    pub fn runtime_patches_count(&self) -> usize {
        self.runtime_patches.len()
    }

    /// Returns a slice of runtime patches starting from the given index.
    pub fn runtime_patches_slice(&self, start: usize) -> &[Patch] {
        &self.runtime_patches[start..]
    }

    /// Apply all collected runtime patches to the source, returning the modified code.
    ///
    /// Deduplicates patches before applying. Returns the original source unchanged
    /// if no runtime patches have been collected.
    pub fn apply_runtime_patches(&self, source: &str) -> Result<String> {
        if self.runtime_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.runtime_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    /// Apply all collected type patches to the source, returning the modified code.
    ///
    /// Deduplicates patches before applying. Returns the original source unchanged
    /// if no type patches have been collected.
    pub fn apply_type_patches(&self, source: &str) -> Result<String> {
        if self.type_patches.is_empty() {
            return Ok(source.to_string());
        }
        let mut patches = self.type_patches.clone();
        dedupe_patches(&mut patches)?;
        let applicator = PatchApplicator::new(source, patches);
        applicator.apply()
    }

    /// Apply runtime patches and return both the modified code and source mapping.
    ///
    /// # Arguments
    ///
    /// * `source` - The original source code
    /// * `macro_name` - Fallback attribution for patches without `source_macro`
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

    /// Apply type patches and return both the modified code and source mapping.
    ///
    /// # Arguments
    ///
    /// * `source` - The original source code
    /// * `macro_name` - Fallback attribution for patches without `source_macro`
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

    /// Returns a reference to the collected type patches.
    pub fn get_type_patches(&self) -> &Vec<Patch> {
        &self.type_patches
    }
}

impl Default for PatchCollector {
    fn default() -> Self {
        Self::new()
    }
}
