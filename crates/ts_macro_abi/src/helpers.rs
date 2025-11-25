//! Helper utilities for working with patches and spans.

use crate::*;

/// Insert code inside a class body.
///
/// This is a convenience helper that creates a `Patch::Insert` positioned
/// just before the closing brace of a class body.
///
/// # Example
/// ```ignore
/// let patch = insert_into_class(class.body_span, "myMethod() { }");
/// ```
pub fn insert_into_class(class_span: SpanIR, code: impl Into<String>) -> Patch {
    let insert_at = class_span.end.saturating_sub(1);
    Patch::Insert {
        at: SpanIR::new(insert_at, insert_at),
        code: code.into(),
    }
}
