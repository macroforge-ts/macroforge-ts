use rustc_hash::FxHashMap;
use syn::{ExprPath, Token, parse_quote, punctuated::Punctuated};

use super::input::QuoteVar;

#[derive(Debug)]
pub(crate) struct Ctx {
    pub(crate) vars: FxHashMap<VarPos, Vars>,
}

impl Ctx {
    pub fn var(&self, ty: VarPos, var_name: &str) -> Option<&VarData> {
        self.vars.get(&ty)?.get(var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarPos {
    Ident,
    Expr,
    Pat,
    AssignTarget,
    Str,
    TsType,
}

#[derive(Debug)]
pub struct VarData {
    ident: syn::Ident,
}

impl VarData {
    pub fn get_expr(&self) -> syn::Expr {
        syn::Expr::Path(ExprPath {
            attrs: Default::default(),
            qself: Default::default(),
            path: self.ident.clone().into(),
        })
    }
}

pub type Vars = FxHashMap<String, VarData>;

/// Binds each quote variable to a uniquely named local and records which
/// placeholder position (`Ident`, `Expr`, ...) it may fill.
pub(super) fn prepare_vars(
    vars: Punctuated<QuoteVar, Token![,]>,
) -> syn::Result<(Vec<syn::Stmt>, FxHashMap<VarPos, Vars>)> {
    let mut stmts = Vec::new();
    let mut var_map = FxHashMap::<_, Vars>::default();

    for var in vars {
        let value = var.value;

        let ident = var.name;
        let ident_str = ident.to_string();

        let pos = match &var.ty {
            None => VarPos::Ident,
            Some(syn::Type::Path(type_path)) if type_path.qself.is_none() => {
                let Some(type_ident) = type_path.path.get_ident() else {
                    return Err(syn::Error::new_spanned(
                        type_path,
                        "variable type must be a simple identifier like Ident, Expr, Pat, etc.",
                    ));
                };
                match type_ident.to_string().as_str() {
                    "Ident" => VarPos::Ident,
                    "Expr" => VarPos::Expr,
                    "Pat" => VarPos::Pat,
                    "Str" => VarPos::Str,
                    "AssignTarget" => VarPos::AssignTarget,
                    "TsType" => VarPos::TsType,
                    ty => {
                        return Err(syn::Error::new_spanned(
                            type_ident,
                            format!(
                                "invalid variable type `{ty}`, expected one of: \
                                Ident, Expr, Pat, Str, AssignTarget, TsType"
                            ),
                        ));
                    }
                }
            }
            Some(ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "variable type must be a simple identifier like Ident, Expr, Pat, etc.",
                ));
            }
        };

        let var_ident = syn::Ident::new(&format!("quote_var_{ident}"), ident.span());

        let old = var_map.entry(pos).or_default().insert(
            ident_str.clone(),
            VarData {
                ident: var_ident.clone(),
            },
        );

        if old.is_some() {
            return Err(syn::Error::new_spanned(
                &ident,
                format!("duplicate variable name: `{ident_str}`"),
            ));
        }

        stmts.push(parse_quote! {
            let #var_ident = #value;
        });
    }

    Ok((stmts, var_map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn var_data(name: &str) -> VarData {
        VarData {
            ident: syn::Ident::new(name, proc_macro2::Span::call_site()),
        }
    }

    #[test]
    fn test_ctx_empty() {
        let ctx = Ctx {
            vars: FxHashMap::default(),
        };
        assert!(ctx.var(VarPos::Ident, "foo").is_none());
    }

    #[test]
    fn test_ctx_var_lookup() {
        let mut vars = FxHashMap::default();
        let mut inner_map: Vars = FxHashMap::default();
        inner_map.insert("foo".to_string(), var_data("quote_var_foo"));
        vars.insert(VarPos::Ident, inner_map);

        let ctx = Ctx { vars };
        assert!(ctx.var(VarPos::Ident, "foo").is_some());
        assert!(ctx.var(VarPos::Ident, "bar").is_none());
        assert!(ctx.var(VarPos::Expr, "foo").is_none());
    }

    #[test]
    fn test_ctx_multiple_var_positions() {
        let mut vars = FxHashMap::default();

        let mut ident_map: Vars = FxHashMap::default();
        ident_map.insert("x".to_string(), var_data("quote_var_x"));
        vars.insert(VarPos::Ident, ident_map);

        let mut expr_map: Vars = FxHashMap::default();
        expr_map.insert("y".to_string(), var_data("quote_var_y"));
        vars.insert(VarPos::Expr, expr_map);

        let ctx = Ctx { vars };
        assert!(ctx.var(VarPos::Ident, "x").is_some());
        assert!(ctx.var(VarPos::Expr, "y").is_some());
        assert!(ctx.var(VarPos::Ident, "y").is_none());
        assert!(ctx.var(VarPos::Expr, "x").is_none());
    }

    #[test]
    fn test_var_data_get_expr_is_plain_reference() {
        let var_data = var_data("quote_var_test");

        for _ in 0..3 {
            let expr = var_data.get_expr();
            assert_eq!(expr.to_token_stream().to_string(), "quote_var_test");
        }
    }

    fn parse_vars(tokens: proc_macro2::TokenStream) -> Punctuated<QuoteVar, Token![,]> {
        syn::parse::Parser::parse2(Punctuated::parse_terminated, tokens)
            .expect("quote vars should parse")
    }

    #[test]
    fn test_prepare_vars_binds_each_var_by_position() {
        let (stmts, vars) = prepare_vars(parse_vars(quote::quote! {
            name = "count", rhs: Expr = rhs_expr
        }))
        .expect("vars should prepare");

        assert_eq!(stmts.len(), 2);
        assert_eq!(
            stmts[0].to_token_stream().to_string(),
            "let quote_var_name = \"count\" ;"
        );
        let ctx = Ctx { vars };
        assert!(ctx.var(VarPos::Ident, "name").is_some());
        assert!(ctx.var(VarPos::Expr, "rhs").is_some());
    }

    #[test]
    fn test_prepare_vars_rejects_duplicates() {
        let err = prepare_vars(parse_vars(quote::quote! { name = a, name = b }))
            .expect_err("duplicate names must be rejected");
        assert!(err.to_string().contains("duplicate variable name"));
    }

    #[test]
    fn test_prepare_vars_rejects_unknown_and_qualified_types() {
        let unknown = prepare_vars(parse_vars(quote::quote! { value: Number = a }))
            .expect_err("unknown type must be rejected");
        assert!(
            unknown
                .to_string()
                .contains("invalid variable type `Number`")
        );

        let qualified = prepare_vars(parse_vars(quote::quote! { value: ast::Expr = a }))
            .expect_err("qualified type must be rejected");
        assert!(qualified.to_string().contains("simple identifier"));
    }
}
