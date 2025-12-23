use crate::template::{PlaceholderUse, Segment};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::{HashMap, HashSet};

use super::compile::compile_stmt_segments;

/// Checks if a segment list contains statement-level control flow or typescript injection.
fn has_stmt_level_control(
    segments: &[Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
) -> bool {
    for s in segments {
        match s {
            Segment::Control { id, .. } => {
                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt)) {
                    return true;
                }
            }
            // {$typescript} requires statement-level compilation
            Segment::Typescript { .. } => {
                return true;
            }
            Segment::BraceBlock { inner, .. } => {
                if has_stmt_level_control(inner, context_map) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Recursively traverses segments to find and compile BraceBlocks that need block substitution.
fn traverse_segments_recursively(
    seg: &Segment,
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
    compilations: &mut Vec<(usize, TokenStream2)>,
) -> syn::Result<()> {
    if let Segment::BraceBlock { id, inner, .. } = seg {
        // Check if this block has statement-level control flow or typescript injection
        if has_stmt_level_control(inner, context_map) {
            // Compile the inner segments
            let compiled = compile_stmt_segments(
                inner,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            compilations.push((*id, compiled));
        }

        // Recurse into inner segments to find nested BraceBlocks
        for inner_seg in inner {
            traverse_segments_recursively(
                inner_seg,
                context_map,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
                compilations,
            )?;
        }
    }
    Ok(())
}

/// Collects BraceBlocks that need block substitution and compiles their inner segments.
pub(crate) fn collect_block_compilations(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<Vec<(usize, TokenStream2)>> {
    let mut compilations = Vec::new();
    let out_ident = proc_macro2::Ident::new("__mf_block_stmts", Span::call_site());

    for seg in run {
        traverse_segments_recursively(
            seg,
            context_map,
            &out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
            &mut compilations,
        )?;
    }

    Ok(compilations)
}

/// Collects placeholder IDs used in `IdentName` positions.
pub(crate) fn collect_ident_name_ids<'a>(
    segments: impl IntoIterator<Item = &'a Segment>,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> Vec<usize> {
    let mut ids = HashSet::new();

    fn collect_from_segment(
        seg: &Segment,
        context_map: &HashMap<usize, PlaceholderUse>,
        ids: &mut HashSet<usize>,
    ) {
        let id = match seg {
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Control { id, .. }
            | Segment::Typescript { id, .. }
            | Segment::ObjectPropLoop { id, .. } => Some(*id),
            Segment::BraceBlock { inner, .. } => {
                for inner_seg in inner {
                    collect_from_segment(inner_seg, context_map, ids);
                }
                None
            }
            _ => None,
        };
        if let Some(id) = id
            && matches!(context_map.get(&id), Some(PlaceholderUse::IdentName))
        {
            ids.insert(id);
        }
    }

    for seg in segments {
        collect_from_segment(seg, context_map, &mut ids);
    }

    let mut ids: Vec<_> = ids.into_iter().collect();
    ids.sort_unstable();
    ids
}
