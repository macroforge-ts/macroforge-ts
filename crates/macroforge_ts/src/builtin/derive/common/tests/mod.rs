mod intersection;
mod registry;
mod types;
mod value_type;

use crate::ts_syn::abi::SpanIR;

pub(super) fn span() -> SpanIR {
    SpanIR::new(0, 0)
}

pub(super) fn make_decorator(name: &str, args: &str) -> crate::ts_syn::abi::DecoratorIR {
    crate::ts_syn::abi::DecoratorIR {
        name: name.into(),
        args_src: args.into(),
        span: span(),
    }
}

pub(super) fn zero_span() -> SpanIR {
    SpanIR::new(0, 0)
}
