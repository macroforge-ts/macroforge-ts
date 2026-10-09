mod options;
mod type_category;
mod validator_errors;
mod validators;

use crate::ts_syn::abi::SpanIR;

pub(super) fn span() -> SpanIR {
    SpanIR::new(0, 0)
}

pub(super) fn make_decorator(args: &str) -> crate::ts_syn::abi::DecoratorIR {
    crate::ts_syn::abi::DecoratorIR {
        name: "endec".into(),
        args_src: args.into(),
        span: span(),
    }
}
