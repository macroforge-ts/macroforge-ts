pub mod ts_syn {
    use swc_core::common::DUMMY_SP;
    use swc_core::ecma::ast::{Expr, Ident, Lit, Str};

    pub mod abi {
        #[derive(Clone, Default, Debug)]
        pub struct Patch;
    }

    pub trait ToTsExpr {
        fn to_ts_expr(self) -> Expr;
    }

    impl ToTsExpr for Expr {
        fn to_ts_expr(self) -> Expr {
            self
        }
    }

    impl ToTsExpr for Ident {
        fn to_ts_expr(self) -> Expr {
            Expr::Ident(self)
        }
    }

    impl ToTsExpr for String {
        fn to_ts_expr(self) -> Expr {
            Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: self.into(),
                raw: None,
            }))
        }
    }

    impl ToTsExpr for &str {
        fn to_ts_expr(self) -> Expr {
            Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: self.into(),
                raw: None,
            }))
        }
    }

    pub fn to_ts_expr<T: ToTsExpr>(value: T) -> Expr {
        value.to_ts_expr()
    }

    #[derive(Clone, Default, Debug)]
    pub struct TsStream {
        source: String,
        pub runtime_patches: Vec<abi::Patch>,
    }

    impl TsStream {
        pub fn from_string(source: String) -> Self {
            Self {
                source,
                runtime_patches: Vec::new(),
            }
        }

        pub fn source(&self) -> &str {
            &self.source
        }
    }
}
