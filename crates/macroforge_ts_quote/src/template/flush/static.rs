use crate::template::{Segment, tokens_to_ts_string};
use proc_macro2::TokenStream as TokenStream2;

/// Flushes buffered static tokens into a static segment.
pub fn flush_static(segments: &mut Vec<Segment>, static_tokens: &mut TokenStream2) {
    if static_tokens.is_empty() {
        return;
    }
    let tokens = std::mem::take(static_tokens);
    let s = tokens_to_ts_string(tokens);
    if !s.trim().is_empty() {
        segments.push(Segment::Static(s));
    }
}
