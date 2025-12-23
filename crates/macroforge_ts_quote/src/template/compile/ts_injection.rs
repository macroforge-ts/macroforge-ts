use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Emits Rust code that injects a TsStream into the statement list.
pub fn compile_ts_injection(
    expr: &TokenStream2,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
) -> TokenStream2 {
    quote! {
        {
            let mut __mf_stream = #expr;
            __patches.extend(__mf_stream.runtime_patches.drain(..));
            let __mf_source = __mf_stream.source().to_string();
            let mut __mf_parser = macroforge_ts::ts_syn::TsStream::from_string(__mf_source);
            let __mf_module: swc_core::ecma::ast::Module = __mf_parser.parse().expect("Failed to parse injected TsStream");
            let mut __mf_first = true;
            for __mf_item in __mf_module.body {
                if let swc_core::ecma::ast::ModuleItem::Stmt(stmt) = __mf_item {
                    if __mf_first {
                        __mf_first = false;
                        if !#pending_ident.is_empty() {
                            use swc_core::common::comments::Comments;
                            use swc_core::common::Spanned;
                            let __mf_pos = stmt.span().lo();
                            for __mf_comment in #pending_ident.drain(..) {
                                #comments_ident.add_leading(__mf_pos, __mf_comment);
                            }
                        }
                    }
                    #out_ident.push(stmt);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    fn make_idents() -> (proc_macro2::Ident, proc_macro2::Ident, proc_macro2::Ident) {
        (
            proc_macro2::Ident::new("__mf_out", Span::call_site()),
            proc_macro2::Ident::new("__mf_comments", Span::call_site()),
            proc_macro2::Ident::new("__mf_pending", Span::call_site()),
        )
    }

    #[test]
    fn test_compile_ts_injection_basic() {
        let expr = quote! { my_stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        // Should contain the stream expression
        assert!(result_str.contains("my_stream"));
    }

    #[test]
    fn test_compile_ts_injection_contains_stream_assignment() {
        let expr = quote! { stream_var };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("let mut __mf_stream = stream_var"));
    }

    #[test]
    fn test_compile_ts_injection_extends_patches() {
        let expr = quote! { ts_stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("__patches . extend"));
        assert!(result_str.contains("__mf_stream . runtime_patches . drain"));
    }

    #[test]
    fn test_compile_ts_injection_parses_source() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("__mf_stream . source () . to_string ()"));
        assert!(result_str.contains("macroforge_ts :: ts_syn :: TsStream :: from_string"));
        assert!(result_str.contains("__mf_parser . parse ()"));
    }

    #[test]
    fn test_compile_ts_injection_handles_module_items() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("for __mf_item in __mf_module . body"));
        assert!(result_str.contains("swc_core :: ecma :: ast :: ModuleItem :: Stmt"));
    }

    #[test]
    fn test_compile_ts_injection_pushes_to_output() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("__mf_out . push (stmt)"));
    }

    #[test]
    fn test_compile_ts_injection_handles_pending_comments() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("if ! __mf_pending . is_empty ()"));
        assert!(result_str.contains("__mf_pending . drain"));
        assert!(result_str.contains("__mf_comments . add_leading"));
    }

    #[test]
    fn test_compile_ts_injection_uses_first_flag() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("let mut __mf_first = true"));
        assert!(result_str.contains("if __mf_first"));
        assert!(result_str.contains("__mf_first = false"));
    }

    #[test]
    fn test_compile_ts_injection_imports_swc_traits() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("use swc_core :: common :: comments :: Comments"));
        assert!(result_str.contains("use swc_core :: common :: Spanned"));
    }

    #[test]
    fn test_compile_ts_injection_gets_span_position() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("stmt . span () . lo ()"));
    }

    #[test]
    fn test_compile_ts_injection_with_complex_expr() {
        let expr = quote! { get_stream().unwrap() };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("get_stream () . unwrap ()"));
    }

    #[test]
    fn test_compile_ts_injection_custom_idents() {
        let expr = quote! { stream };
        let out = proc_macro2::Ident::new("custom_out", Span::call_site());
        let comments = proc_macro2::Ident::new("custom_comments", Span::call_site());
        let pending = proc_macro2::Ident::new("custom_pending", Span::call_site());

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        assert!(result_str.contains("custom_out . push"));
        assert!(result_str.contains("custom_comments . add_leading"));
        assert!(result_str.contains("custom_pending . is_empty ()"));
        assert!(result_str.contains("custom_pending . drain"));
    }

    #[test]
    fn test_compile_ts_injection_wraps_in_block() {
        let expr = quote! { stream };
        let (out, comments, pending) = make_idents();

        let result = compile_ts_injection(&expr, &out, &comments, &pending);
        let result_str = result.to_string();

        // Should be wrapped in a block
        assert!(result_str.starts_with("{"));
        assert!(result_str.ends_with("}"));
    }
}
