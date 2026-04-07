use crate::host::error::Result;
use crate::ts_syn::abi::{Patch, PatchCode};
use std::collections::HashSet;
#[cfg(feature = "swc")]
use swc_core::{
    common::{SourceMap, sync::Lrc},
    ecma::codegen::{Config, Emitter, Node, text_writer::JsWriter},
};

/// Fixed dedupe logic: Separate key generation from filtering.
/// Also performs import-aware deduplication: when both `import type { X }` and
/// `import { X }` exist for the same specifier and module, the value import
/// subsumes the type-only import (since a value import is usable in both
/// value and type positions).
pub(crate) fn dedupe_patches(patches: &mut Vec<Patch>) -> Result<()> {
    // Phase 1: Collect import-aware dedup info.
    // For InsertRaw patches with context == "import", parse the specifier and module,
    // then drop type-only imports when a matching value import exists.
    dedupe_imports(patches);

    // Phase 2: Standard exact-match deduplication.
    let mut seen: HashSet<(u8, u32, u32, Option<String>)> = HashSet::new();
    let mut indices_to_keep = Vec::new();

    for (i, patch) in patches.iter().enumerate() {
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

    let old_patches = std::mem::take(patches);
    *patches = indices_to_keep
        .into_iter()
        .map(|i| old_patches[i].clone())
        .collect();

    Ok(())
}

/// Parses an import patch's code string into (specifier, module, is_type_only).
///
/// Handles formats like:
/// - `import { Foo } from "bar";\n`          -> ("Foo", "bar", false)
/// - `import type { Foo } from "bar";\n`     -> ("Foo", "bar", true)
/// - `import { Foo as Bar } from "bar";\n`   -> ("Foo", "bar", false)  (base name = "Foo")
pub(crate) fn parse_import_patch(code: &str) -> Option<(String, String, bool)> {
    let trimmed = code.trim();

    let is_type = trimmed.starts_with("import type ");
    let rest = if is_type {
        trimmed.strip_prefix("import type ")?
    } else {
        trimmed.strip_prefix("import ")?
    };

    // Extract specifier between { and }
    let brace_start = rest.find('{')?;
    let brace_end = rest.find('}')?;
    let specifier_raw = rest[brace_start + 1..brace_end].trim();

    // Get the base specifier name (before " as " if aliased)
    let base_specifier = if let Some(pos) = specifier_raw.find(" as ") {
        specifier_raw[..pos].trim().to_string()
    } else {
        specifier_raw.to_string()
    };

    // Extract module between quotes
    let after_brace = &rest[brace_end + 1..];
    let quote_char = if after_brace.contains('"') { '"' } else { '\'' };
    let first_quote = after_brace.find(quote_char)?;
    let second_quote = after_brace[first_quote + 1..].find(quote_char)?;
    let module = after_brace[first_quote + 1..first_quote + 1 + second_quote].to_string();

    Some((base_specifier, module, is_type))
}

/// Removes type-only import patches when a value import for the same
/// (specifier, module) pair exists. A value import subsumes a type-only import.
pub(crate) fn dedupe_imports(patches: &mut Vec<Patch>) {
    // Collect all (specifier, module) pairs that have value imports
    let mut value_imports: HashSet<(String, String)> = HashSet::new();

    for patch in patches.iter() {
        if let Patch::InsertRaw {
            context: Some(ctx),
            code,
            ..
        } = patch
            && ctx == "import"
            && let Some((specifier, module, is_type)) = parse_import_patch(code)
            && !is_type
        {
            value_imports.insert((specifier, module));
        }
    }

    // If no value imports exist, nothing to dedupe
    if value_imports.is_empty() {
        return;
    }

    // Remove type-only imports that are subsumed by a value import
    patches.retain(|patch| {
        if let Patch::InsertRaw {
            context: Some(ctx),
            code,
            ..
        } = patch
            && ctx == "import"
            && let Some((specifier, module, is_type)) = parse_import_patch(code)
            && is_type
            && value_imports.contains(&(specifier, module))
        {
            return false; // Drop this type-only import
        }
        true
    });
}

pub(crate) fn render_patch_code(code: &PatchCode) -> Result<String> {
    match code {
        PatchCode::Text(s) => Ok(s.clone()),
        #[cfg(feature = "swc")]
        PatchCode::ClassMember(member) => emit_node(member),
        #[cfg(feature = "swc")]
        PatchCode::Stmt(stmt) => emit_node(stmt),
        #[cfg(feature = "swc")]
        PatchCode::ModuleItem(item) => emit_node(item),
    }
}

#[cfg(feature = "swc")]
pub(crate) fn emit_node<N: Node>(node: &N) -> Result<String> {
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
