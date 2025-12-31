use proc_macro2::Span;
use swc_core::atoms::{Atom, Wtf8Atom};
use swc_core::ecma::ast::*;
use syn::{ExprLit, LitBool, LitByteStr, LitFloat, parse_quote};

use super::ToCode;
use crate::{builder::Builder, ctxt::Ctx};

fail_todo!(BigInt);
fail_todo!(JSXText);

impl ToCode for Str {
    fn to_code(&self, cx: &crate::ctxt::Ctx) -> syn::Expr {
        if let Some(var_name) = self.value.as_str()
            && let Some(var_name) = var_name.strip_prefix('$')
            && let Some(var) = cx.var(crate::ctxt::VarPos::Str, var_name)
        {
            return var.get_expr();
        }

        let mut builder = Builder::new("Str");
        builder.add("span", ToCode::to_code(&self.span, cx));
        builder.add("value", ToCode::to_code(&self.value, cx));
        builder.add("raw", ToCode::to_code(&self.raw, cx));
        syn::Expr::Struct(builder.build())
    }
}
impl_struct!(Bool, [span, value]);
impl_struct!(Null, [span]);
impl_struct!(Number, [span, value, raw]);
impl_struct!(Regex, [span, exp, flags]);

impl ToCode for Atom {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let val = &**self;
        parse_quote!(macroforge_ts::swc_core::atoms::atom!(#val))
    }
}

impl ToCode for Wtf8Atom {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let bytes_literal = LitByteStr::new(self.as_bytes(), Span::call_site());
        parse_quote!(unsafe { macroforge_ts::swc_core::atoms::Wtf8Atom::from_bytes_unchecked(#bytes_literal) })
    }
}

impl ToCode for bool {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        syn::Expr::Lit(ExprLit {
            attrs: Default::default(),
            lit: syn::Lit::Bool(LitBool::new(*self, Span::call_site())),
        })
    }
}

impl ToCode for f64 {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        syn::Expr::Lit(ExprLit {
            attrs: Default::default(),
            lit: syn::Lit::Float(LitFloat::new(&format!("{self}f64"), Span::call_site())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;
    use swc_core::atoms::Atom;

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== Str Tests ====================

    #[test]
    fn test_str_to_code_simple() {
        let cx = empty_ctx();
        let str_lit = Str {
            span: swc_core::common::Span::default(),
            value: Atom::from("hello").into(),
            raw: None,
        };
        let code = str_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Str"));
        assert!(code_str.contains("span"));
        assert!(code_str.contains("value"));
    }

    #[test]
    fn test_str_to_code_with_raw() {
        let cx = empty_ctx();
        let str_lit = Str {
            span: swc_core::common::Span::default(),
            value: Atom::from("hello").into(),
            raw: Some(Atom::from("\"hello\"")),
        };
        let code = str_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("raw"));
    }

    #[test]
    fn test_str_to_code_with_dollar_prefix() {
        // Test Str with $-prefix without actual var mapping
        let cx = empty_ctx();
        let str_lit = Str {
            span: swc_core::common::Span::default(),
            value: Atom::from("$myStr").into(),
            raw: None,
        };
        let code = str_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Should generate code even without mapping
        assert!(!code_str.is_empty());
    }

    // ==================== Bool Tests ====================

    #[test]
    fn test_bool_to_code_true() {
        let cx = empty_ctx();
        let bool_lit = Bool {
            span: swc_core::common::Span::default(),
            value: true,
        };
        let code = bool_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Bool"));
        assert!(code_str.contains("value"));
    }

    #[test]
    fn test_bool_to_code_false() {
        let cx = empty_ctx();
        let bool_lit = Bool {
            span: swc_core::common::Span::default(),
            value: false,
        };
        let code = bool_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Bool"));
    }

    // ==================== Null Tests ====================

    #[test]
    fn test_null_to_code() {
        let cx = empty_ctx();
        let null_lit = Null {
            span: swc_core::common::Span::default(),
        };
        let code = null_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Null"));
        assert!(code_str.contains("span"));
    }

    // ==================== Number Tests ====================

    #[test]
    fn test_number_to_code_integer() {
        let cx = empty_ctx();
        let num_lit = Number {
            span: swc_core::common::Span::default(),
            value: 42.0,
            raw: Some(Atom::from("42")),
        };
        let code = num_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Number"));
        assert!(code_str.contains("value"));
    }

    #[test]
    fn test_number_to_code_float() {
        let cx = empty_ctx();
        let num_lit = Number {
            span: swc_core::common::Span::default(),
            value: 3.14159,
            raw: Some(Atom::from("3.14159")),
        };
        let code = num_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Number"));
    }

    // ==================== Regex Tests ====================

    #[test]
    fn test_regex_to_code() {
        let cx = empty_ctx();
        let regex_lit = Regex {
            span: swc_core::common::Span::default(),
            exp: Atom::from("[a-z]+"),
            flags: Atom::from("gi"),
        };
        let code = regex_lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Regex"));
        assert!(code_str.contains("exp"));
        assert!(code_str.contains("flags"));
    }

    // ==================== Atom Tests ====================

    #[test]
    fn test_atom_to_code() {
        let cx = empty_ctx();
        let atom = Atom::from("test");
        let code = atom.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "atom !"
        assert!(code_str.contains("atom") && code_str.contains("!"));
    }

    #[test]
    fn test_atom_to_code_empty() {
        let cx = empty_ctx();
        let atom = Atom::from("");
        let code = atom.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "atom !"
        assert!(code_str.contains("atom") && code_str.contains("!"));
    }

    #[test]
    fn test_atom_to_code_special_chars() {
        let cx = empty_ctx();
        let atom = Atom::from("hello world!");
        let code = atom.to_code(&cx);
        // Should not panic
        let _code_str = code.to_token_stream().to_string();
    }

    // ==================== Primitive ToCode Tests ====================

    #[test]
    fn test_bool_primitive_true() {
        let cx = empty_ctx();
        let val = true;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert_eq!(code_str.trim(), "true");
    }

    #[test]
    fn test_bool_primitive_false() {
        let cx = empty_ctx();
        let val = false;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert_eq!(code_str.trim(), "false");
    }

    #[test]
    fn test_f64_to_code_integer() {
        let cx = empty_ctx();
        let val: f64 = 42.0;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("f64"));
    }

    #[test]
    fn test_f64_to_code_decimal() {
        let cx = empty_ctx();
        let val: f64 = 3.14159;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("f64"));
        assert!(code_str.contains("3.14159"));
    }

    #[test]
    fn test_f64_to_code_negative() {
        let cx = empty_ctx();
        let val: f64 = -42.5;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("-"));
    }

    #[test]
    fn test_f64_to_code_zero() {
        let cx = empty_ctx();
        let val: f64 = 0.0;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("0"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_str_empty() {
        let cx = empty_ctx();
        let str_lit = Str {
            span: swc_core::common::Span::default(),
            value: Atom::from("").into(),
            raw: None,
        };
        let code = str_lit.to_code(&cx);
        // Should not panic
        let _code_str = code.to_token_stream().to_string();
    }

    #[test]
    fn test_str_with_escape_chars() {
        let cx = empty_ctx();
        let str_lit = Str {
            span: swc_core::common::Span::default(),
            value: Atom::from("hello\nworld").into(),
            raw: None,
        };
        let code = str_lit.to_code(&cx);
        // Should not panic
        let _code_str = code.to_token_stream().to_string();
    }
}
