use swc_core::ecma::ast::*;
use syn::parse_quote;

use super::ToCode;
use crate::ctxt::{Ctx, VarPos};

impl ToCode for swc_core::ecma::ast::Ident {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        if let Some(var_name) = self.sym.strip_prefix('$') {
            if let Some(var) = cx.var(VarPos::Ident, var_name) {
                return var.get_expr();
            }
            panic!(
                "Unresolved placeholder identifier: ${} - no matching variable in context",
                var_name
            );
        }

        let sym_value = self.sym.to_code(cx);
        let optional_value = self.optional.to_code(cx);
        parse_quote!(macroforge_ts::swc_core::ecma::ast::Ident {
            span: macroforge_ts::swc_core::common::DUMMY_SP,
            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
            sym: #sym_value,
            optional: #optional_value,
        })
    }
}

impl ToCode for swc_core::ecma::ast::IdentName {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        if let Some(var_name) = self.sym.strip_prefix('$') {
            if let Some(var) = cx.var(VarPos::Ident, var_name) {
                let var_expr = var.get_expr();
                return parse_quote!(#var_expr.into());
            }
            panic!(
                "Unresolved placeholder IdentName: ${} - no matching variable in context",
                var_name
            );
        }

        let span_value = self.span.to_code(cx);
        let sym_value = self.sym.to_code(cx);
        parse_quote!(macroforge_ts::swc_core::ecma::ast::IdentName {
            span: #span_value,
            sym: #sym_value,
        })
    }
}

impl_struct!(PrivateName, [span, name]);

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;
    use swc_core::atoms::Atom;
    use swc_core::common::Span;

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== Ident Tests ====================

    #[test]
    fn test_ident_to_code_simple() {
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("foo"),
            optional: false,
        };
        let code = ident.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Ident"));
        assert!(code_str.contains("sym"));
        assert!(code_str.contains("optional"));
    }

    #[test]
    fn test_ident_to_code_optional() {
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("maybeNull"),
            optional: true,
        };
        let code = ident.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("optional"));
    }

    #[test]
    #[should_panic(expected = "Unresolved placeholder identifier")]
    fn test_ident_to_code_dollar_prefix_without_var() {
        // Test behavior when a $-prefixed identifier doesn't have a matching var
        // Should panic to surface the bug early rather than emitting broken code
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("$noSuchVar"),
            optional: false,
        };
        let _ = ident.to_code(&cx);
    }

    #[test]
    fn test_ident_to_code_no_substitution_without_dollar() {
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("regularIdent"),
            optional: false,
        };
        let code = ident.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Should generate a struct, not a variable reference
        assert!(code_str.contains("Ident"));
        assert!(code_str.contains("sym"));
    }

    // ==================== IdentName Tests ====================

    #[test]
    fn test_ident_name_to_code_simple() {
        let cx = empty_ctx();
        let ident_name = swc_core::ecma::ast::IdentName {
            span: Span::default(),
            sym: Atom::from("propertyName"),
        };
        let code = ident_name.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("IdentName"));
        assert!(code_str.contains("sym"));
    }

    #[test]
    #[should_panic(expected = "Unresolved placeholder IdentName")]
    fn test_ident_name_to_code_with_dollar_prefix() {
        // Test IdentName with $-prefix without actual var mapping
        // Should panic to surface the bug early rather than emitting broken code
        let cx = empty_ctx();
        let ident_name = swc_core::ecma::ast::IdentName {
            span: Span::default(),
            sym: Atom::from("$someProp"),
        };
        let _ = ident_name.to_code(&cx);
    }

    // ==================== PrivateName Tests ====================

    #[test]
    fn test_private_name_to_code() {
        let cx = empty_ctx();
        let private_name = PrivateName {
            span: Span::default(),
            name: Atom::from("privateField"),
        };
        let code = private_name.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("PrivateName"));
        assert!(code_str.contains("span"));
        assert!(code_str.contains("name"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_ident_with_special_characters() {
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("_underscore"),
            optional: false,
        };
        let code = ident.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Ident"));
    }

    #[test]
    fn test_ident_with_unicode() {
        let cx = empty_ctx();
        let ident = swc_core::ecma::ast::Ident {
            span: Span::default(),
            ctxt: swc_core::common::SyntaxContext::empty(),
            sym: Atom::from("αβγ"),
            optional: false,
        };
        let code = ident.to_code(&cx);
        // Should not panic
        let _code_str = code.to_token_stream().to_string();
    }
}
