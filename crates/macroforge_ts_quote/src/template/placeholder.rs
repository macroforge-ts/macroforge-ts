use crate::template::PlaceholderUse;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::collections::HashMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use super::utils::use_rank;

pub(crate) struct PlaceholderFinder {
    ids: HashMap<String, usize>,
    uses: HashMap<usize, PlaceholderUse>,
}

impl Visit for PlaceholderFinder {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        if let Stmt::Expr(expr_stmt) = stmt
            && let Expr::Ident(ident) = &*expr_stmt.expr {
                self.record(ident.sym.as_ref(), PlaceholderUse::Stmt);
                return;
            }
        stmt.visit_children_with(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        if let Expr::Ident(ident) = expr {
            self.record(ident.sym.as_ref(), PlaceholderUse::Expr);
        }
        expr.visit_children_with(self);
    }

    fn visit_member_prop(&mut self, prop: &MemberProp) {
        if let MemberProp::Ident(ident) = prop {
            self.record(ident.sym.as_ref(), PlaceholderUse::IdentName);
        }
        prop.visit_children_with(self);
    }

    fn visit_prop_name(&mut self, prop: &PropName) {
        if let PropName::Ident(ident) = prop {
            self.record(ident.sym.as_ref(), PlaceholderUse::IdentName);
        }
        prop.visit_children_with(self);
    }

    fn visit_labeled_stmt(&mut self, stmt: &LabeledStmt) {
        self.record(stmt.label.sym.as_ref(), PlaceholderUse::Ident);
        stmt.visit_children_with(self);
    }

    fn visit_pat(&mut self, pat: &Pat) {
        if let Pat::Ident(binding) = pat {
            self.record(binding.id.sym.as_ref(), PlaceholderUse::Ident);
        }
        pat.visit_children_with(self);
    }

    fn visit_ts_type_ref(&mut self, ty: &TsTypeRef) {
        if let TsEntityName::Ident(ident) = &ty.type_name {
            self.record(ident.sym.as_ref(), PlaceholderUse::Type);
        }
        ty.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, decl: &FnDecl) {
        self.record(decl.ident.sym.as_ref(), PlaceholderUse::Ident);
        decl.visit_children_with(self);
    }

    fn visit_class_decl(&mut self, decl: &ClassDecl) {
        self.record(decl.ident.sym.as_ref(), PlaceholderUse::Ident);
        decl.visit_children_with(self);
    }
}

impl PlaceholderFinder {
    /// Creates a placeholder finder from a placeholder name map.
    pub(crate) fn new(ids: HashMap<String, usize>) -> Self {
        Self {
            ids,
            uses: HashMap::new(),
        }
    }

    /// Records a placeholder usage, keeping the most specific kind.
    fn record(&mut self, name: &str, use_kind: PlaceholderUse) {
        if let Some(id) = self.ids.get(name).copied() {
            let entry = self.uses.entry(id).or_insert(use_kind.clone());
            if use_rank(&use_kind) > use_rank(entry) {
                *entry = use_kind;
            }
        }
    }

    /// Returns the collected placeholder usage map.
    pub(crate) fn into_map(self) -> HashMap<usize, PlaceholderUse> {
        self.uses
    }
}

/// Type placeholder that needs post-processing via visitor (since SWC quote! doesn't support $ in types)
pub(crate) struct TypePlaceholder {
    pub(crate) id: usize,
    pub(crate) expr: TokenStream2,
}

/// Rewrites placeholder `IdentName`s (e.g. `$__mf_hole_0`) into real identifiers.
pub(crate) fn ident_name_fix_block(
    target_ident: &proc_macro2::Ident,
    placeholder_ids: &[usize],
) -> TokenStream2 {
    if placeholder_ids.is_empty() {
        return TokenStream2::new();
    }

    let fields: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! { #field_ident: swc_core::ecma::ast::IdentName }
        })
        .collect();
    let inits: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! { #field_ident: swc_core::ecma::ast::IdentName::from(#field_ident.clone()) }
        })
        .collect();
    let arms: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let marker = format!("${}", placeholder_name(*id));
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! {
                #marker => {
                    *ident = self.#field_ident.clone();
                }
            }
        })
        .collect();
    let member_arms = arms.clone();
    let prop_arms = arms;

    quote! {
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfIdentNameFix {
                #(#fields,)*
            }
            impl VisitMut for __MfIdentNameFix {
                fn visit_mut_member_prop(&mut self, prop: &mut swc_core::ecma::ast::MemberProp) {
                    if let swc_core::ecma::ast::MemberProp::Ident(ident) = prop {
                        match ident.sym.as_ref() {
                            #(#member_arms)*
                            _ => {}
                        }
                    }
                    prop.visit_mut_children_with(self);
                }

                fn visit_mut_prop_name(&mut self, prop: &mut swc_core::ecma::ast::PropName) {
                    if let swc_core::ecma::ast::PropName::Ident(ident) = prop {
                        match ident.sym.as_ref() {
                            #(#prop_arms)*
                            _ => {}
                        }
                    }
                    prop.visit_mut_children_with(self);
                }
            }
            let mut __mf_ident_fix = __MfIdentNameFix {
                #(#inits,)*
            };
            #target_ident.visit_mut_with(&mut __mf_ident_fix);
        }
    }
}

/// Generates code to replace type placeholders like __MfType0 with actual type references.
///
/// SWC's quote! macro doesn't support $ placeholders in type positions, so we use
/// regular identifiers and replace them via a visitor after quoting.
pub(crate) fn generate_type_placeholder_fix(type_placeholders: &[TypePlaceholder]) -> TokenStream2 {
    if type_placeholders.is_empty() {
        return TokenStream2::new();
    }

    let fields: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            quote! { #field_name: String }
        })
        .collect();

    let inits: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            let expr = &tp.expr;
            // Convert the expression to a String for the type name
            quote! { #field_name: (#expr).to_string() }
        })
        .collect();

    let arms: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let marker = format!("__MfTypeMarker{}", tp.id);
            let field_name = format_ident!("__mf_type_{}", tp.id);
            quote! {
                #marker => {
                    *ty = swc_core::ecma::ast::TsType::TsTypeRef(swc_core::ecma::ast::TsTypeRef {
                        span: swc_core::common::DUMMY_SP,
                        type_name: swc_core::ecma::ast::TsEntityName::Ident(
                            swc_core::ecma::ast::Ident::new(
                                self.#field_name.clone().into(),
                                swc_core::common::DUMMY_SP,
                                Default::default(),
                            )
                        ),
                        type_params: None,
                    });
                }
            }
        })
        .collect();

    quote! {
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfTypeFix {
                #(#fields,)*
            }
            impl VisitMut for __MfTypeFix {
                fn visit_mut_ts_type(&mut self, ty: &mut swc_core::ecma::ast::TsType) {
                    if let swc_core::ecma::ast::TsType::TsTypeRef(type_ref) = ty {
                        if let swc_core::ecma::ast::TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }
            let mut __mf_type_fix = __MfTypeFix {
                #(#inits,)*
            };
            __mf_stmt.visit_mut_with(&mut __mf_type_fix);
        }
    }
}

/// Maps placeholder usage to the SWC AST type used for binding in quote!.
///
/// NOTE: SWC's quote! macro only supports Ident, Expr, and Pat for variable
/// substitution. For Type placeholders, we use Ident and later convert them
/// to TsType via a post-processing visitor.
pub(crate) fn placeholder_type_tokens(use_kind: &PlaceholderUse) -> TokenStream2 {
    match use_kind {
        PlaceholderUse::Expr | PlaceholderUse::Stmt => quote!(Expr),
        PlaceholderUse::Ident => quote!(Ident),
        PlaceholderUse::IdentName => quote!(Ident),
        // Use Ident for types too - SWC quote! doesn't support TsType
        // The Ident will be placed in type position and needs no conversion
        // because TypeScript type references are just identifiers
        PlaceholderUse::Type => quote!(Ident),
    }
}
/// Formats the placeholder identifier for a given segment ID.
pub(crate) fn placeholder_name(id: usize) -> String {
    format!("__mf_hole_{id}")
}
