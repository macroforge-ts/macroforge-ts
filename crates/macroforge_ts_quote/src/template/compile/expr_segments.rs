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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_expr_segments_simple_static() {
        let segments = vec![Segment::Static("x + y".to_string())];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        // Should contain the basic quote_ts structure
        assert!(tokens_str.contains("x + y"));
    }

    #[test]
    fn test_compile_expr_segments_empty() {
        let segments: Vec<Segment> = vec![];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_expr_segments_multiple_static() {
        let segments = vec![
            Segment::Static("foo".to_string()),
            Segment::Static(" + ".to_string()),
            Segment::Static("bar".to_string()),
        ];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("foo"));
        assert!(tokens_str.contains("bar"));
    }

    #[test]
    fn test_compile_expr_segments_with_interpolation() {
        // Use valid TypeScript: foo(__mf_hole_0) is a function call
        let segments = vec![
            Segment::Static("foo(".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: quote! { arg },
            },
            Segment::Static(")".to_string()),
        ];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_expr_segments_with_control_flow() {
        // Use valid TypeScript numeric literals for branches
        let segments = vec![Segment::Control {
            id: 0,
            node: crate::template::ControlNode::If {
                cond: quote! { true },
                then_branch: vec![Segment::Static("1".to_string())],
                else_branch: Some(vec![Segment::Static("0".to_string())]),
            },
        }];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_expr_segments_generates_bindings_block() {
        // Use valid TypeScript identifier
        let segments = vec![Segment::Static("x".to_string())];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        // Should be wrapped in a block
        assert!(tokens_str.starts_with("{"));
        assert!(tokens_str.ends_with("}"));
    }

    #[test]
    fn test_compile_expr_segments_preserves_whitespace() {
        let segments = vec![Segment::Static("  x  +  y  ".to_string())];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_expr_segments_handles_special_chars() {
        let segments = vec![Segment::Static(r#""string with \"quotes\"""#.to_string())];
        let result = compile_expr_segments(&segments);
        assert!(result.is_ok());
    }
}
