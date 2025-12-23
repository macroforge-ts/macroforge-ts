use crate::template::{ControlNode, Segment};
use proc_macro2::TokenStream as TokenStream2;

/// Finds a for loop within segments, ensuring it's the only dynamic control node.
///
/// Returns `Some((pat, iter, body))` if exactly one for loop is found with no other
/// dynamic content, `None` otherwise.
pub(super) fn find_for_loop_in_segments(
    inner_segments: &[Segment],
) -> Option<(TokenStream2, TokenStream2, Vec<Segment>)> {
    let mut for_node = None;
    let mut other_dynamic = false;

    for seg in inner_segments {
        match seg {
            Segment::Control {
                node: ControlNode::For { pat, iter, body },
                ..
            } => {
                if for_node.is_some() {
                    // Multiple for loops - not a simple object prop loop
                    return None;
                }
                for_node = Some((pat.clone(), iter.clone(), body.clone()));
            }
            Segment::Static(_) => {
                // Static content (whitespace, commas) is OK
            }
            _ => {
                // Other dynamic segments (interpolations outside the loop, etc.)
                other_dynamic = true;
            }
        }
    }

    // If there's other dynamic content, fall back to normal handling
    if other_dynamic {
        return None;
    }

    for_node
}

/// Validates that the loop body matches the key-value property pattern.
///
/// Expected pattern: `@{key}: @{val},` (with potential whitespace variations)
///
/// Returns `Some((key_expr, value_expr))` if the pattern matches, `None` otherwise.
pub(super) fn validate_key_value_body(
    body: &[Segment],
) -> Option<(TokenStream2, TokenStream2)> {
    let mut key_expr = None;
    let mut value_expr = None;
    let mut found_colon = false;

    for seg in body {
        match seg {
            Segment::Static(s) => {
                let trimmed = s.trim();
                if trimmed.contains(':') {
                    found_colon = true;
                }
                // Allow whitespace, colons, commas
            }
            Segment::Interpolation { expr, .. } => {
                if !found_colon {
                    // This is before the colon, so it's the key
                    if key_expr.is_some() {
                        // Multiple keys - not a simple pattern
                        return None;
                    }
                    key_expr = Some(expr.clone());
                } else {
                    // This is after the colon, so it's the value
                    if value_expr.is_some() {
                        // Multiple values - not a simple pattern
                        return None;
                    }
                    value_expr = Some(expr.clone());
                }
            }
            _ => {
                // Other segment types in the body - not a simple pattern
                return None;
            }
        }
    }

    // Must have both key and value
    let key_expr = key_expr?;
    let value_expr = value_expr?;

    if !found_colon {
        return None;
    }

    Some((key_expr, value_expr))
}
