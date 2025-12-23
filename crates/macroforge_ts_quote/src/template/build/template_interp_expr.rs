use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use crate::template::{
    escape_tpl_segment_allow_dollar, quote_ts, BindingSpec, QuoteTsResult, StringPart,
};

/// Builds an SWC expression from a backtick template literal.
pub fn build_template_interp_expr(parts: &[StringPart], id: usize) -> TokenStream2 {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut expr_index = 0usize;

    template.push('`');
    for part in parts {
        match part {
            StringPart::Text(text) => {
                template.push_str(&escape_tpl_segment_allow_dollar(text));
            }
            StringPart::Expr(expr) => {
                let name = format!("__mf_tpl_{id}_{expr_index}");
                expr_index += 1;
                template.push_str("${$");
                template.push_str(&name);
                template.push('}');
                let ident = proc_macro2::Ident::new(&name, Span::call_site());
                bindings.push(BindingSpec {
                    name: ident,
                    ty: quote!(Expr),
                    expr: quote! { macroforge_ts::ts_syn::to_ts_expr(#expr) },
                });
            }
        }
    }
    template.push('`');

    let quote_ts = quote_ts(&template, quote!(Expr), &bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;
    quote! {{
        #bindings
        #expr
    }}
}
