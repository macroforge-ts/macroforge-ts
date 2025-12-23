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
