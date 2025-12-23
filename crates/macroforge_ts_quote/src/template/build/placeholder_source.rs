use std::collections::HashMap;

use crate::template::{append_part, placeholder_name, Segment};

/// Placeholder source kind for parsing.
#[derive(Clone, Copy)]
pub enum PlaceholderSourceKind {
    Module,
    Expr,
}

/// Builds a placeholder-only source string and placeholder ID map.
pub fn build_placeholder_source(
    segments: &[Segment],
    kind: PlaceholderSourceKind,
) -> (String, HashMap<String, usize>) {
    let mut src = String::new();
    let mut map = HashMap::new();

    for seg in segments {
        match seg {
            Segment::Static(s) => append_part(&mut src, s),
            Segment::Comment { .. } => {}
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Control { id, .. }
            | Segment::Typescript { id, .. }
            | Segment::ObjectPropLoop { id, .. } => {
                let name = placeholder_name(*id);
                append_part(&mut src, &name);
                if matches!(kind, PlaceholderSourceKind::Module)
                    && matches!(seg, Segment::Control { .. } | Segment::Typescript { .. })
                {
                    src.push(';');
                }
                map.insert(name, *id);
            }
            Segment::Let { .. } | Segment::Do { .. } => {
                // Rust-only constructs are ignored for TS parsing.
            }
            Segment::BraceBlock { inner, .. } => {
                // Recursively process inner segments with braces
                append_part(&mut src, "{");
                let (inner_src, inner_map) = build_placeholder_source(inner, kind);
                append_part(&mut src, &inner_src);
                map.extend(inner_map);
                append_part(&mut src, "}");
            }
        }
    }

    (src, map)
}
