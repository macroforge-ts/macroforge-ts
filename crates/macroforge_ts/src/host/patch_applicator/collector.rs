use super::applicator::{ApplyResult, PatchApplicator};
use super::helpers::dedupe_patches;
use crate::host::error::{MacroError, Result};
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

    /// Applies the collected patches to `source`, deduplicated: the runtime
    /// output with its source mapping, and the type output when any type
    /// patches were collected.
    ///
    /// `macro_name` attributes generated code whose patch names no macro.
    pub fn apply(
        self,
        source: &str,
        macro_name: Option<&str>,
    ) -> Result<(ApplyResult, Option<String>)> {
        let runtime = if self.runtime_patches.is_empty() {
            let source_len = source.len() as u32;
            let mut mapping = SourceMapping::new();
            if source_len > 0 {
                mapping.add_segment(MappingSegment::new(0, source_len, 0, source_len));
            }
            ApplyResult {
                code: source.to_string(),
                mapping,
            }
        } else {
            let mut patches = self.runtime_patches;
            dedupe_patches(&mut patches);
            PatchApplicator::new(source, patches)
                .apply_with_mapping(macro_name)
                .map_err(|error| MacroError::Patch(format!("Patch error: {error:?}")))?
        };
        let types = if self.type_patches.is_empty() {
            None
        } else {
            let mut patches = self.type_patches;
            dedupe_patches(&mut patches);
            Some(
                PatchApplicator::new(source, patches)
                    .apply()
                    .map_err(|error| MacroError::Patch(format!("Type patch error: {error:?}")))?,
            )
        };
        Ok((runtime, types))
    }
}

impl Default for PatchCollector {
    fn default() -> Self {
        Self::new()
    }
}
