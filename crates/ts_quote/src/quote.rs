/// Very small quasiquote for TS.
/// v0: just format strings safely.
/// v1: could parse into SWC expr/stmts if you want.
///
/// Usage:
///   let m = ts_quote!("toString(): string {{ return {} }}", body);
#[macro_export]
macro_rules! ts_quote {
    ($lit:expr $(, $($rest:tt)+)? ) => {{
        format!($lit $(, $($rest)+)? )
    }};
}
