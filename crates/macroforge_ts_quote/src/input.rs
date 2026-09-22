use syn::{
    Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub(super) struct QuoteInput {
    /// The oxc arena the quote parses into: the first argument, or the
    /// caller's `arena` binding when omitted. The node lives as long as it does.
    #[cfg(feature = "oxc")]
    pub allocator: syn::Expr,
    pub src: syn::LitStr,
    pub output_type: syn::Type,

    pub vars: Option<(Token![,], Punctuated<QuoteVar, Token![,]>)>,
}

pub(super) struct QuoteVar {
    pub name: syn::Ident,
    /// Defaults to `swc_ecma_ast::Ident`
    pub ty: Option<syn::Type>,
    pub value: syn::Expr,
}

impl Parse for QuoteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Without an explicit arena, the quote uses the `arena` binding in
        // scope at the call site.
        #[cfg(feature = "oxc")]
        let allocator = if input.peek(syn::LitStr) {
            let arena = syn::Ident::new("arena", proc_macro2::Span::call_site());
            syn::parse_quote!(#arena)
        } else {
            let allocator = input.parse()?;
            input.parse::<Token![,]>()?;
            allocator
        };
        let src = input.parse()?;
        input.parse::<Token![as]>()?;
        let output_type = input.parse()?;
        let vars = if input.is_empty() {
            None
        } else {
            let comma_token = input.parse()?;
            let vars = Punctuated::parse_terminated(input)?;
            Some((comma_token, vars))
        };

        Ok(Self {
            #[cfg(feature = "oxc")]
            allocator,
            src,
            output_type,
            vars,
        })
    }
}

impl Parse for QuoteVar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let ty = if input.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        input.parse::<Token![=]>()?;
        Ok(Self {
            name,
            ty,
            value: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    // ==================== QuoteInput Tests ====================

    #[test]
    fn test_quote_input_defaults_to_the_arena_binding() {
        let input: QuoteInput = syn::parse_quote!("x + 1" as Expr);
        let arena: syn::Expr = syn::parse_quote!(arena);
        assert_eq!(
            input.allocator.to_token_stream().to_string(),
            arena.to_token_stream().to_string()
        );
        assert_eq!(input.src.value(), "x + 1");
    }

    #[test]
    fn test_quote_input_basic() {
        let input: QuoteInput = syn::parse_quote!(allocator, "x + 1" as Expr);
        assert_eq!(input.src.value(), "x + 1");
        assert!(input.vars.is_none());
    }

    #[test]
    fn test_quote_input_with_simple_type() {
        let input: QuoteInput = syn::parse_quote!(allocator, "let x = 1" as Stmt);
        assert_eq!(input.src.value(), "let x = 1");
    }

    #[test]
    fn test_quote_input_with_single_var() {
        let input: QuoteInput = syn::parse_quote!(allocator, "$name" as Expr, name = my_ident);
        assert_eq!(input.src.value(), "$name");
        assert!(input.vars.is_some());

        let (_, vars) = input.vars.unwrap();
        assert_eq!(vars.len(), 1);

        let var = vars.first().unwrap();
        assert_eq!(var.name.to_string(), "name");
        assert!(var.ty.is_none()); // No type specified
    }

    #[test]
    fn test_quote_input_with_typed_var() {
        let input: QuoteInput = syn::parse_quote!(allocator, "$val" as Expr, val: Expr = some_expr);
        assert!(input.vars.is_some());

        let (_, vars) = input.vars.unwrap();
        let var = vars.first().unwrap();
        assert_eq!(var.name.to_string(), "val");
        assert!(var.ty.is_some());
    }

    #[test]
    fn test_quote_input_with_multiple_vars() {
        let input: QuoteInput =
            syn::parse_quote!(allocator, "$a + $b" as Expr, a = expr_a, b: Expr = expr_b);
        assert!(input.vars.is_some());

        let (_, vars) = input.vars.unwrap();
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_quote_input_various_output_types() {
        // Test different output types
        let _: QuoteInput = syn::parse_quote!(allocator, "x" as Pat);
        let _: QuoteInput = syn::parse_quote!(allocator, "x = 1" as Stmt);
        let _: QuoteInput = syn::parse_quote!(allocator, "number" as TsType);
        let _: QuoteInput = syn::parse_quote!(allocator, "import x from 'y'" as ModuleItem);
    }

    #[test]
    fn test_quote_input_with_box_type() {
        let input: QuoteInput = syn::parse_quote!(allocator, "x" as Box<Expr>);
        // Just verify it parses
        assert_eq!(input.src.value(), "x");
    }

    #[test]
    fn test_quote_input_with_option_type() {
        let input: QuoteInput = syn::parse_quote!(allocator, "" as Option<Expr>);
        assert_eq!(input.src.value(), "");
    }

    #[test]
    fn test_quote_input_empty_string() {
        let input: QuoteInput = syn::parse_quote!(allocator, "" as Option<Expr>);
        assert_eq!(input.src.value(), "");
        assert!(input.vars.is_none());
    }

    #[test]
    fn test_quote_input_complex_typescript() {
        let input: QuoteInput = syn::parse_quote!(
            allocator,
            "async function foo<T>(x: T): Promise<T> { return x; }" as Stmt
        );
        assert!(input.src.value().contains("async"));
        assert!(input.src.value().contains("Promise"));
    }

    // ==================== QuoteVar Tests ====================

    #[test]
    fn test_quote_var_simple() {
        let var: QuoteVar = syn::parse_quote!(name = value);
        assert_eq!(var.name.to_string(), "name");
        assert!(var.ty.is_none());
    }

    #[test]
    fn test_quote_var_with_ident_type() {
        let var: QuoteVar = syn::parse_quote!(name: Ident = value);
        assert_eq!(var.name.to_string(), "name");
        assert!(var.ty.is_some());

        if let Some(syn::Type::Path(path)) = &var.ty {
            assert_eq!(
                path.path.segments.last().unwrap().ident.to_string(),
                "Ident"
            );
        } else {
            panic!("Expected Type::Path");
        }
    }

    #[test]
    fn test_quote_var_with_expr_type() {
        let var: QuoteVar = syn::parse_quote!(val: Expr = some_value);
        assert!(var.ty.is_some());

        if let Some(syn::Type::Path(path)) = &var.ty {
            assert_eq!(path.path.segments.last().unwrap().ident.to_string(), "Expr");
        }
    }

    #[test]
    fn test_quote_var_with_pat_type() {
        let var: QuoteVar = syn::parse_quote!(pattern: Pat = my_pat);
        assert!(var.ty.is_some());
    }

    #[test]
    fn test_quote_var_with_str_type() {
        let var: QuoteVar = syn::parse_quote!(s: Str = my_string);
        assert!(var.ty.is_some());
    }

    #[test]
    fn test_quote_var_with_ts_type() {
        let var: QuoteVar = syn::parse_quote!(t: TsType = type_node);
        assert!(var.ty.is_some());
    }

    #[test]
    fn test_quote_var_with_assign_target_type() {
        let var: QuoteVar = syn::parse_quote!(target: AssignTarget = lhs);
        assert!(var.ty.is_some());
    }

    #[test]
    fn test_quote_var_complex_value() {
        let var: QuoteVar = syn::parse_quote!(x = foo.bar().baz);
        assert_eq!(var.name.to_string(), "x");
        // Value should be parsed as an expression
    }

    #[test]
    fn test_quote_var_literal_value() {
        let var: QuoteVar = syn::parse_quote!(num = 42);
        assert_eq!(var.name.to_string(), "num");
    }

    #[test]
    fn test_quote_var_string_literal_value() {
        let var: QuoteVar = syn::parse_quote!(s = "hello");
        assert_eq!(var.name.to_string(), "s");
    }

    #[test]
    fn test_quote_var_closure_value() {
        let var: QuoteVar = syn::parse_quote!(f = |x| x + 1);
        assert_eq!(var.name.to_string(), "f");
    }

    #[test]
    fn test_quote_var_block_value() {
        let var: QuoteVar = syn::parse_quote!(
            result = {
                let x = 1;
                x
            }
        );
        assert_eq!(var.name.to_string(), "result");
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_quote_input_whitespace_in_source() {
        let input: QuoteInput = syn::parse_quote!(allocator, "  x  +  y  " as Expr);
        assert_eq!(input.src.value(), "  x  +  y  ");
    }

    #[test]
    fn test_quote_input_newlines_in_source() {
        let input: QuoteInput = syn::parse_quote!(allocator, "x\n+\ny" as Expr);
        assert!(input.src.value().contains('\n'));
    }

    #[test]
    fn test_quote_input_special_characters() {
        let input: QuoteInput = syn::parse_quote!(allocator, "x?.y ?? z" as Expr);
        assert!(input.src.value().contains("?."));
        assert!(input.src.value().contains("??"));
    }

    #[test]
    fn test_quote_var_underscore_name() {
        let var: QuoteVar = syn::parse_quote!(_unused = value);
        assert_eq!(var.name.to_string(), "_unused");
    }

    #[test]
    fn test_quote_var_long_name() {
        let var: QuoteVar = syn::parse_quote!(very_long_variable_name_here = value);
        assert_eq!(var.name.to_string(), "very_long_variable_name_here");
    }

    #[test]
    fn test_quote_input_trailing_comma_in_vars() {
        // Punctuated::parse_terminated handles trailing commas
        let input: QuoteInput = syn::parse_quote!(allocator, "$x" as Expr, x = val,);
        assert!(input.vars.is_some());
    }

    #[test]
    fn test_quote_var_method_call_value() {
        let var: QuoteVar = syn::parse_quote!(ident = Ident::new("foo", span));
        assert_eq!(var.name.to_string(), "ident");
    }
}
