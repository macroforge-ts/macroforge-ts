#![allow(unused)]

use std::cell::RefCell;

use rustc_hash::FxHashMap;
use swc_macros_common::call_site;
use syn::{ExprPath, ExprReference, Ident, Token, parse_quote, punctuated::Punctuated};

use super::{ast::ToCode, input::QuoteVar};

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
    pos: VarPos,
    is_counting: bool,

    /// How many times this variable should be cloned. 0 for variables used only
    /// once.
    clone: RefCell<usize>,

    ident: syn::Ident,
}

impl VarData {
    pub fn get_expr(&self) -> syn::Expr {
        if self.is_counting {
            *self.clone.borrow_mut() += 1;
            return self.expr_for_var_ref();
        }

        let use_clone = {
            let mut b = self.clone.borrow_mut();
            let val = *b;
            if val > 0 {
                *b -= 1;
                val != 1
            } else {
                false
            }
        };

        if use_clone {
            let var_ref_expr = self.expr_for_var_ref();

            parse_quote!(macroforge_ts::swc_core::quote::ImplicitClone::clone_quote_var(&#var_ref_expr))
        } else {
            self.expr_for_var_ref()
        }
    }

    fn expr_for_var_ref(&self) -> syn::Expr {
        syn::Expr::Path(ExprPath {
            attrs: Default::default(),
            qself: Default::default(),
            path: self.ident.clone().into(),
        })
    }
}

pub type Vars = FxHashMap<String, VarData>;

/// Prepares variables for use in quote macro expansion.
/// This function:
/// 1. Parses variable declarations and their types
/// 2. Creates variable bindings with unique names
/// 3. Counts how many times each variable is used (for clone optimization)
/// 4. Returns the prepared statements and variable map
pub(super) fn prepare_vars(
    src: &dyn ToCode,
    vars: Punctuated<QuoteVar, Token![,]>,
) -> syn::Result<(Vec<syn::Stmt>, FxHashMap<VarPos, Vars>)> {
    let mut stmts = Vec::new();
    let mut init_map = FxHashMap::<_, Vars>::default();

    for var in vars {
        let value = var.value;

        let ident = var.name.clone();
        let ident_str = ident.to_string();

        let pos = match &var.ty {
            Some(syn::Type::Path(syn::TypePath {
                qself: None,
                path:
                    syn::Path {
                        leading_colon: None,
                        segments,
                    },
            })) => {
                let segment = segments.first().unwrap();
                match segment.ident.to_string().as_str() {
                    "Ident" => VarPos::Ident,
                    "Expr" => VarPos::Expr,
                    "Pat" => VarPos::Pat,
                    "Str" => VarPos::Str,
                    "AssignTarget" => VarPos::AssignTarget,
                    "TsType" => VarPos::TsType,
                    ty => {
                        return Err(syn::Error::new_spanned(
                            &segment.ident,
                            format!(
                                "invalid variable type `{ty}`, expected one of: \
                                Ident, Expr, Pat, Str, AssignTarget, TsType"
                            ),
                        ));
                    }
                }
            }
            None => VarPos::Ident,
            Some(ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    "variable type must be a simple identifier like Ident, Expr, Pat, etc.",
                ));
            }
        };

        let var_ident = syn::Ident::new(&format!("quote_var_{ident}"), ident.span());

        let old = init_map.entry(pos).or_default().insert(
            ident_str.clone(),
            VarData {
                pos,
                is_counting: true,
                clone: Default::default(),
                ident: var_ident.clone(),
            },
        );

        if old.is_some() {
            return Err(syn::Error::new_spanned(
                &ident,
                format!("duplicate variable name: `{ident_str}`"),
            ));
        }

        let type_name = Ident::new(
            match pos {
                VarPos::Ident => "Ident",
                VarPos::Expr => "Expr",
                VarPos::Pat => "Pat",
                VarPos::AssignTarget => "AssignTarget",
                VarPos::Str => "Str",
                VarPos::TsType => "TsType",
            },
            call_site(),
        );
        stmts.push(parse_quote! {
            let #var_ident: macroforge_ts::swc_core::ecma::ast::#type_name = #value;
        });
    }

    // Use `ToCode` to count how many times each variable is used.
    let mut cx = Ctx { vars: init_map };

    src.to_code(&cx);

    // We are done
    cx.vars
        .iter_mut()
        .for_each(|(_, v)| v.iter_mut().for_each(|(_, v)| v.is_counting = false));

    Ok((stmts, cx.vars))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    // ==================== VarPos Tests ====================

    #[test]
    fn test_var_pos_equality() {
        assert_eq!(VarPos::Ident, VarPos::Ident);
        assert_eq!(VarPos::Expr, VarPos::Expr);
        assert_eq!(VarPos::Pat, VarPos::Pat);
        assert_eq!(VarPos::AssignTarget, VarPos::AssignTarget);
        assert_eq!(VarPos::Str, VarPos::Str);
        assert_eq!(VarPos::TsType, VarPos::TsType);
    }

    #[test]
    fn test_var_pos_inequality() {
        assert_ne!(VarPos::Ident, VarPos::Expr);
        assert_ne!(VarPos::Pat, VarPos::TsType);
        assert_ne!(VarPos::Str, VarPos::AssignTarget);
    }

    #[test]
    fn test_var_pos_ordering() {
        // VarPos should be orderable for use in BTreeMap etc.
        let mut positions = vec![VarPos::TsType, VarPos::Ident, VarPos::Expr];
        positions.sort();
        // Should not panic - ordering is defined
    }

    #[test]
    fn test_var_pos_hash() {
        // VarPos should be usable as a hash key
        let mut map = FxHashMap::default();
        map.insert(VarPos::Ident, "ident");
        map.insert(VarPos::Expr, "expr");

        assert_eq!(map.get(&VarPos::Ident), Some(&"ident"));
        assert_eq!(map.get(&VarPos::Expr), Some(&"expr"));
        assert_eq!(map.get(&VarPos::Pat), None);
    }

    #[test]
    fn test_var_pos_clone() {
        let pos = VarPos::Ident;
        let cloned = pos;
        assert_eq!(pos, cloned);
    }

    #[test]
    fn test_var_pos_debug() {
        // VarPos should be debuggable
        let debug_str = format!("{:?}", VarPos::Ident);
        assert!(debug_str.contains("Ident"));
    }

    // ==================== Ctx Tests ====================

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
        inner_map.insert(
            "foo".to_string(),
            VarData {
                pos: VarPos::Ident,
                is_counting: false,
                clone: RefCell::new(0),
                ident: syn::Ident::new("quote_var_foo", proc_macro2::Span::call_site()),
            },
        );
        vars.insert(VarPos::Ident, inner_map);

        let ctx = Ctx { vars };
        assert!(ctx.var(VarPos::Ident, "foo").is_some());
        assert!(ctx.var(VarPos::Ident, "bar").is_none());
        assert!(ctx.var(VarPos::Expr, "foo").is_none());
    }

    #[test]
    fn test_ctx_multiple_var_positions() {
        let mut vars = FxHashMap::default();

        // Add Ident var
        let mut ident_map: Vars = FxHashMap::default();
        ident_map.insert(
            "x".to_string(),
            VarData {
                pos: VarPos::Ident,
                is_counting: false,
                clone: RefCell::new(0),
                ident: syn::Ident::new("quote_var_x", proc_macro2::Span::call_site()),
            },
        );
        vars.insert(VarPos::Ident, ident_map);

        // Add Expr var
        let mut expr_map: Vars = FxHashMap::default();
        expr_map.insert(
            "y".to_string(),
            VarData {
                pos: VarPos::Expr,
                is_counting: false,
                clone: RefCell::new(0),
                ident: syn::Ident::new("quote_var_y", proc_macro2::Span::call_site()),
            },
        );
        vars.insert(VarPos::Expr, expr_map);

        let ctx = Ctx { vars };
        assert!(ctx.var(VarPos::Ident, "x").is_some());
        assert!(ctx.var(VarPos::Expr, "y").is_some());
        assert!(ctx.var(VarPos::Ident, "y").is_none());
        assert!(ctx.var(VarPos::Expr, "x").is_none());
    }

    // ==================== VarData Tests ====================

    #[test]
    fn test_var_data_get_expr_first_use_no_clone() {
        let var_data = VarData {
            pos: VarPos::Ident,
            is_counting: false,
            clone: RefCell::new(0),
            ident: syn::Ident::new("quote_var_test", proc_macro2::Span::call_site()),
        };

        let expr = var_data.get_expr();
        let expr_str = expr.to_token_stream().to_string();
        assert_eq!(expr_str, "quote_var_test");
    }

    #[test]
    fn test_var_data_get_expr_with_clone() {
        let var_data = VarData {
            pos: VarPos::Ident,
            is_counting: false,
            clone: RefCell::new(2), // Will need cloning
            ident: syn::Ident::new("quote_var_test", proc_macro2::Span::call_site()),
        };

        // First call - val=2, should clone (val != 1), count becomes 1
        let expr1 = var_data.get_expr();
        let expr1_str = expr1.to_token_stream().to_string();
        assert!(expr1_str.contains("clone_quote_var"));

        // Second call - val=1, should NOT clone (val == 1 is last use), count becomes 0
        let expr2 = var_data.get_expr();
        let expr2_str = expr2.to_token_stream().to_string();
        assert_eq!(expr2_str, "quote_var_test");

        // Third call - val=0, should NOT clone (val > 0 is false)
        let expr3 = var_data.get_expr();
        let expr3_str = expr3.to_token_stream().to_string();
        assert_eq!(expr3_str, "quote_var_test");
    }

    #[test]
    fn test_var_data_counting_mode() {
        let var_data = VarData {
            pos: VarPos::Ident,
            is_counting: true,
            clone: RefCell::new(0),
            ident: syn::Ident::new("quote_var_test", proc_macro2::Span::call_site()),
        };

        // Each call should increment the clone counter
        var_data.get_expr();
        assert_eq!(*var_data.clone.borrow(), 1);

        var_data.get_expr();
        assert_eq!(*var_data.clone.borrow(), 2);

        var_data.get_expr();
        assert_eq!(*var_data.clone.borrow(), 3);
    }

    #[test]
    fn test_var_data_expr_for_var_ref() {
        let var_data = VarData {
            pos: VarPos::Expr,
            is_counting: false,
            clone: RefCell::new(0),
            ident: syn::Ident::new("my_var", proc_macro2::Span::call_site()),
        };

        let expr = var_data.expr_for_var_ref();
        if let syn::Expr::Path(path) = expr {
            assert_eq!(path.path.segments.len(), 1);
            assert_eq!(path.path.segments[0].ident.to_string(), "my_var");
        } else {
            panic!("Expected ExprPath");
        }
    }

    #[test]
    fn test_var_data_debug() {
        let var_data = VarData {
            pos: VarPos::Ident,
            is_counting: false,
            clone: RefCell::new(0),
            ident: syn::Ident::new("test", proc_macro2::Span::call_site()),
        };

        let debug_str = format!("{:?}", var_data);
        assert!(debug_str.contains("VarData"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_var_data_clone_count_boundary() {
        // Test when clone count is exactly 1
        let var_data = VarData {
            pos: VarPos::Ident,
            is_counting: false,
            clone: RefCell::new(1),
            ident: syn::Ident::new("test", proc_macro2::Span::call_site()),
        };

        // First call - should use clone (count = 1, not last use)
        let expr1 = var_data.get_expr();
        // After first call, count becomes 0, but since val was 1 and val != 1 is false,
        // it should NOT clone
        let expr1_str = expr1.to_token_stream().to_string();
        // When clone count is exactly 1, the condition val != 1 is false, so no clone
        assert_eq!(expr1_str, "test");
    }

    #[test]
    fn test_empty_vars_map() {
        let vars: Vars = FxHashMap::default();
        assert!(vars.is_empty());
        assert!(vars.get("anything").is_none());
    }

    #[test]
    fn test_vars_map_multiple_entries() {
        let mut vars: Vars = FxHashMap::default();
        vars.insert(
            "a".to_string(),
            VarData {
                pos: VarPos::Ident,
                is_counting: false,
                clone: RefCell::new(0),
                ident: syn::Ident::new("a", proc_macro2::Span::call_site()),
            },
        );
        vars.insert(
            "b".to_string(),
            VarData {
                pos: VarPos::Ident,
                is_counting: false,
                clone: RefCell::new(0),
                ident: syn::Ident::new("b", proc_macro2::Span::call_site()),
            },
        );

        assert_eq!(vars.len(), 2);
        assert!(vars.get("a").is_some());
        assert!(vars.get("b").is_some());
        assert!(vars.get("c").is_none());
    }
}
