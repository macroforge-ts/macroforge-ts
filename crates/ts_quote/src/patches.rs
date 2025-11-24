use ts_macro_abi::*;

/// Insert code inside a class body.
/// v0: naive "insert before final }" using span.
/// Host can provide exact insertion spans later.
pub fn insert_into_class(class_span: SpanIR, code: impl Into<String>) -> Patch {
    let insert_at = class_span.end.saturating_sub(1);
    Patch::Insert {
        at: SpanIR::new(insert_at, insert_at),
        code: code.into(),
    }
}
