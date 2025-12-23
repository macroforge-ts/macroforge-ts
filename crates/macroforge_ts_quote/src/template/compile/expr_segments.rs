use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use crate::template::{
    build_template_and_bindings, classify_placeholders_expr, collect_ident_name_ids,
    generate_type_placeholder_fix, ident_name_fix_block, quote_ts, QuoteTsResult, Segment,
};

/// Compiles expression-level segments into a single SWC expression.
pub fn compile_expr_segments(segments: &[Segment]) -> syn::Result<TokenStream2> {
    let context_map = classify_placeholders_expr(segments)?;
    let template_result = build_template_and_bindings(segments, &context_map)?;
    let ident_name_ids = collect_ident_name_ids(segments.iter(), &context_map);
    let quote_ts = quote_ts(
        &template_result.template,
        quote!(Expr),
        &template_result.bindings,
    );
    let QuoteTsResult { bindings, expr } = quote_ts;

    // Generate type placeholder fix if there are any
    let type_fix = generate_type_placeholder_fix(&template_result.type_placeholders);

    if ident_name_ids.is_empty() && type_fix.is_empty() {
        Ok(quote! {{
            #bindings
            #expr
        }})
    } else {
        let expr_ident = proc_macro2::Ident::new("__mf_expr", Span::call_site());
        let fix_block = ident_name_fix_block(&expr_ident, &ident_name_ids);
        Ok(quote! {{
            #bindings
            let mut #expr_ident = #expr;
            #fix_block
            #type_fix
            #expr_ident
        }})
    }
}
