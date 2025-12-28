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
