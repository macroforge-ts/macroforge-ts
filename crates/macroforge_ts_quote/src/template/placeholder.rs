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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::PlaceholderUse;
    use quote::quote;
    use std::collections::HashMap;

    #[test]
    fn test_placeholder_finder_record_first_recording() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Expr);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_placeholder_finder_record_upgrade_expr_to_ident() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Expr);
        finder.record("foo", PlaceholderUse::Ident);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Ident));
    }

    #[test]
    fn test_placeholder_finder_record_upgrade_expr_to_ident_to_stmt() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Expr);
        finder.record("foo", PlaceholderUse::Ident);
        finder.record("foo", PlaceholderUse::Stmt);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Stmt));
    }

    #[test]
    fn test_placeholder_finder_record_no_downgrade_from_stmt() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Stmt);
        finder.record("foo", PlaceholderUse::Ident);
        finder.record("foo", PlaceholderUse::Expr);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Stmt));
    }

    #[test]
    fn test_placeholder_finder_record_no_downgrade_from_type() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Type);
        finder.record("foo", PlaceholderUse::Ident);
        finder.record("foo", PlaceholderUse::Expr);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Type));
    }

    #[test]
    fn test_placeholder_finder_record_upgrade_ident_name_to_ident() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::IdentName);
        finder.record("foo", PlaceholderUse::Ident);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 1);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Ident));
    }

    #[test]
    fn test_placeholder_finder_record_unknown_placeholder() {
        let ids = HashMap::new();

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("unknown", PlaceholderUse::Expr);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 0);
    }

    #[test]
    fn test_placeholder_finder_record_multiple_placeholders() {
        let mut ids = HashMap::new();
        ids.insert("foo".to_string(), 0);
        ids.insert("bar".to_string(), 1);
        ids.insert("baz".to_string(), 2);

        let mut finder = PlaceholderFinder::new(ids);
        finder.record("foo", PlaceholderUse::Expr);
        finder.record("bar", PlaceholderUse::Ident);
        finder.record("baz", PlaceholderUse::Stmt);

        let uses = finder.into_map();
        assert_eq!(uses.len(), 3);
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(uses.get(&1), Some(&PlaceholderUse::Ident));
        assert_eq!(uses.get(&2), Some(&PlaceholderUse::Stmt));
    }

    #[test]
    fn test_ident_name_fix_block_empty_list() {
        let target_ident = quote::format_ident!("target");
        let placeholder_ids: Vec<usize> = vec![];

        let result = ident_name_fix_block(&target_ident, &placeholder_ids);
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn test_ident_name_fix_block_single_placeholder() {
        let target_ident = quote::format_ident!("target");
        let placeholder_ids = vec![0];

        let result = ident_name_fix_block(&target_ident, &placeholder_ids);
        let result_str = result.to_string();

        // Check that the output contains expected components
        assert!(result_str.contains("__MfIdentNameFix"));
        assert!(result_str.contains("__mf_hole_0"));
        assert!(result_str.contains("IdentName"));
        assert!(result_str.contains("$__mf_hole_0"));
    }

    #[test]
    fn test_ident_name_fix_block_multiple_placeholders() {
        let target_ident = quote::format_ident!("target");
        let placeholder_ids = vec![0, 1, 2];

        let result = ident_name_fix_block(&target_ident, &placeholder_ids);
        let result_str = result.to_string();

        // Check that all placeholders are present
        assert!(result_str.contains("__mf_hole_0"));
        assert!(result_str.contains("__mf_hole_1"));
        assert!(result_str.contains("__mf_hole_2"));
        assert!(result_str.contains("$__mf_hole_0"));
        assert!(result_str.contains("$__mf_hole_1"));
        assert!(result_str.contains("$__mf_hole_2"));
    }

    #[test]
    fn test_generate_type_placeholder_fix_empty_list() {
        let type_placeholders: Vec<TypePlaceholder> = vec![];

        let result = generate_type_placeholder_fix(&type_placeholders);
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn test_generate_type_placeholder_fix_single_type() {
        let type_placeholders = vec![TypePlaceholder {
            id: 0,
            expr: quote! { "string" },
        }];

        let result = generate_type_placeholder_fix(&type_placeholders);
        let result_str = result.to_string();

        // Check that the output contains expected components
        assert!(result_str.contains("__MfTypeFix"));
        assert!(result_str.contains("__mf_type_0"));
        assert!(result_str.contains("__MfTypeMarker0"));
        assert!(result_str.contains("TsType"));
        assert!(result_str.contains("TsTypeRef"));
    }

    #[test]
    fn test_generate_type_placeholder_fix_multiple_types() {
        let type_placeholders = vec![
            TypePlaceholder {
                id: 0,
                expr: quote! { "string" },
            },
            TypePlaceholder {
                id: 1,
                expr: quote! { "number" },
            },
            TypePlaceholder {
                id: 2,
                expr: quote! { "boolean" },
            },
        ];

        let result = generate_type_placeholder_fix(&type_placeholders);
        let result_str = result.to_string();

        // Check that all type placeholders are present
        assert!(result_str.contains("__mf_type_0"));
        assert!(result_str.contains("__mf_type_1"));
        assert!(result_str.contains("__mf_type_2"));
        assert!(result_str.contains("__MfTypeMarker0"));
        assert!(result_str.contains("__MfTypeMarker1"));
        assert!(result_str.contains("__MfTypeMarker2"));
    }

    #[test]
    fn test_placeholder_type_tokens_expr() {
        let result = placeholder_type_tokens(&PlaceholderUse::Expr);
        assert_eq!(result.to_string(), "Expr");
    }

    #[test]
    fn test_placeholder_type_tokens_stmt() {
        let result = placeholder_type_tokens(&PlaceholderUse::Stmt);
        assert_eq!(result.to_string(), "Expr");
    }

    #[test]
    fn test_placeholder_type_tokens_ident() {
        let result = placeholder_type_tokens(&PlaceholderUse::Ident);
        assert_eq!(result.to_string(), "Ident");
    }

    #[test]
    fn test_placeholder_type_tokens_ident_name() {
        let result = placeholder_type_tokens(&PlaceholderUse::IdentName);
        assert_eq!(result.to_string(), "Ident");
    }

    #[test]
    fn test_placeholder_type_tokens_type() {
        let result = placeholder_type_tokens(&PlaceholderUse::Type);
        assert_eq!(result.to_string(), "Ident");
    }

    #[test]
    fn test_placeholder_name_zero() {
        assert_eq!(placeholder_name(0), "__mf_hole_0");
    }

    #[test]
    fn test_placeholder_name_small() {
        assert_eq!(placeholder_name(5), "__mf_hole_5");
    }

    #[test]
    fn test_placeholder_name_large() {
        assert_eq!(placeholder_name(123), "__mf_hole_123");
    }

    #[test]
    fn test_placeholder_name_very_large() {
        assert_eq!(placeholder_name(999999), "__mf_hole_999999");
    }
}
