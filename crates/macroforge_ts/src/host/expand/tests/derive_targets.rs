use super::super::derive_targets::find_macro_name_span;
use crate::ts_syn::abi::SpanIR;

fn whole(source: &str) -> SpanIR {
    SpanIR::new(
        1,
        u32::try_from(source.len()).map_or(u32::MAX, |len| len + 1),
    )
}

#[test]
fn finds_name_after_non_ascii_text() {
    let source = "@derive(/* größe */ Clone)";
    let span = find_macro_name_span(source, whole(source), "Clone");
    assert_eq!(
        span.and_then(|span| source.get(span.source_range())),
        Some("Clone")
    );
}

#[test]
fn skips_name_inside_a_longer_identifier() {
    let source = "@derive(CloneMe, $Clone, Clone)";
    let span = find_macro_name_span(source, whole(source), "Clone");
    assert_eq!(span.map(|span| span.start), Some(26));
}

#[test]
fn rejects_span_off_a_char_boundary() {
    let source = "@derive(é)";
    let span = SpanIR::new(1, 10);
    assert_eq!(find_macro_name_span(source, span, "Clone"), None);
}
