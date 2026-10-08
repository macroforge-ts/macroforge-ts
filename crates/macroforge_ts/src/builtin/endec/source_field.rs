//! The view of a declared field that encode and decode read.

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::abi::ir::{DecoratorIR, FieldIR, InterfaceFieldIR};

/// The parts of a class, interface or object-type field that the endec
/// derives read.
pub(super) struct SourceField<'a> {
    pub name: &'a str,
    pub ts_type: &'a str,
    pub decorators: &'a [DecoratorIR],
    pub optional: bool,
    pub span: SpanIR,
}

impl<'a> From<&'a FieldIR> for SourceField<'a> {
    fn from(field: &'a FieldIR) -> Self {
        Self {
            name: &field.name,
            ts_type: &field.ts_type,
            decorators: &field.decorators,
            optional: field.optional,
            span: field.span,
        }
    }
}

impl<'a> From<&'a InterfaceFieldIR> for SourceField<'a> {
    fn from(field: &'a InterfaceFieldIR) -> Self {
        Self {
            name: &field.name,
            ts_type: &field.ts_type,
            decorators: &field.decorators,
            optional: field.optional,
            span: field.span,
        }
    }
}
