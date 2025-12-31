use swc_core::common::{Span, SyntaxContext};
use swc_core::ecma::ast::*;
use syn::{ExprBlock, parse_quote};

use super::ctxt::Ctx;

macro_rules! fail_todo {
    ($T:ty) => {
        impl crate::ast::ToCode for $T {
            fn to_code(&self, _: &crate::ctxt::Ctx) -> syn::Expr {
                todo!("ToCode for {}", stringify!($T))
            }
        }
    };
}

macro_rules! impl_enum_body {
    ($E:ident, $s:expr, $cx:expr,[ $($v:ident),* ]) => {
        match $s {
            $(
                $E::$v(inner) => {
                    let val = crate::ast::ToCode::to_code(inner, $cx);
                    syn::parse_quote!(
                        macroforge_ts::swc_core::ecma::ast::$E::$v(#val)
                    )
                },
            )*
            #[cfg(swc_ast_unknown)]
            _ => panic!("unable to access unknown nodes"),
        }
    };
}

macro_rules! impl_enum {
    ($E:ident, [ $($v:ident),* ]) => {
        impl crate::ast::ToCode for $E {
            fn to_code(&self, cx: &crate::ctxt::Ctx) -> syn::Expr {
                impl_enum_body!($E, self, cx, [ $($v),* ])
            }
        }
    };


    ($E:ident, [ $($v:ident),* ], true) => {
        impl crate::ast::ToCode for $E {
            fn to_code(&self, cx: &crate::ctxt::Ctx) -> syn::Expr {
                if let Some(i) = self.as_ident() {
                    if let Some(var_name) = i.sym.strip_prefix('$') {
                        if let Some(var) = cx.var(crate::ctxt::VarPos::$E, var_name) {
                            return var.get_expr();
                        }
                    }
                }

                impl_enum_body!($E, self, cx, [ $($v),* ])
            }
        }
    };
}

macro_rules! impl_struct {
    (
        $name:ident,
        [ $($v:ident),* ]
    ) => {
        impl crate::ast::ToCode for $name {
            fn to_code(&self, cx: &crate::ctxt::Ctx) -> syn::Expr {
                let mut builder = crate::builder::Builder::new(stringify!($name));

                let Self { $($v,)* } = self;

                $(
                    builder.add(
                        stringify!($v),
                        crate::ast::ToCode::to_code($v, cx),
                    );
                )*

                syn::Expr::Struct(builder.build())
            }
        }
    };
}

mod class;
mod decl;
mod enums;
mod expr;
mod id;
mod lit;
mod module_decl;
mod pat;
mod prop;
mod stmt;
mod typescript;

pub(crate) trait ToCode: 'static {
    fn to_code(&self, cx: &Ctx) -> syn::Expr;
}

impl<T> ToCode for Box<T>
where
    T: ?Sized + ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let inner = (**self).to_code(cx);
        parse_quote!(Box::new(#inner))
    }
}

/// TODO: Optimize
impl<T> ToCode for Option<T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            Some(inner) => {
                let inner = inner.to_code(cx);

                parse_quote!(Some(#inner))
            }
            None => parse_quote!(None),
        }
    }
}

impl_struct!(Invalid, [span]);

impl ToCode for Span {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote!(macroforge_ts::swc_core::common::DUMMY_SP)
    }
}

impl ToCode for SyntaxContext {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote!(macroforge_ts::swc_core::common::SyntaxContext::empty())
    }
}

impl_enum!(ModuleItem, [ModuleDecl, Stmt]);

impl_enum!(
    Pat,
    [Ident, Array, Rest, Object, Assign, Invalid, Expr],
    true
);
impl_enum!(Lit, [Str, Bool, Null, Num, BigInt, Regex, JSXText]);
impl_enum!(
    ClassMember,
    [
        Constructor,
        Method,
        PrivateMethod,
        ClassProp,
        PrivateProp,
        TsIndexSignature,
        Empty,
        StaticBlock,
        AutoAccessor
    ]
);
impl_enum!(ObjectPatProp, [KeyValue, Assign, Rest]);
impl_enum!(PropName, [Ident, Str, Num, Computed, BigInt]);
impl_enum!(ParamOrTsParamProp, [TsParamProp, Param]);
impl_enum!(PropOrSpread, [Spread, Prop]);
impl_enum!(BlockStmtOrExpr, [BlockStmt, Expr]);
impl_enum!(MemberProp, [Ident, PrivateName, Computed]);
impl_enum!(SuperProp, [Ident, Computed]);
impl_enum!(JSXObject, [Ident, JSXMemberExpr]);
impl_enum!(
    JSXElementChild,
    [
        JSXText,
        JSXElement,
        JSXExprContainer,
        JSXFragment,
        JSXSpreadChild
    ]
);
impl_enum!(OptChainBase, [Member, Call]);
impl_enum!(JSXElementName, [Ident, JSXMemberExpr, JSXNamespacedName]);
impl_enum!(JSXAttrOrSpread, [JSXAttr, SpreadElement]);

impl<T> ToCode for Vec<T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let len = self.len();
        let var_stmt: syn::Stmt = parse_quote!(let mut items = Vec::with_capacity(#len););
        let mut stmts = vec![var_stmt];

        for item in self {
            let item = item.to_code(cx);
            stmts.push(syn::Stmt::Expr(
                parse_quote!(items.push(#item)),
                Some(Default::default()),
            ));
        }

        stmts.push(syn::Stmt::Expr(parse_quote!(items), None));

        syn::Expr::Block(ExprBlock {
            attrs: Default::default(),
            label: Default::default(),
            block: syn::Block {
                brace_token: Default::default(),
                stmts,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== Span Tests ====================

    #[test]
    fn test_span_to_code() {
        let cx = empty_ctx();
        let span = Span::default();
        let code = span.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("DUMMY_SP"));
    }

    // ==================== SyntaxContext Tests ====================

    #[test]
    fn test_syntax_context_to_code() {
        let cx = empty_ctx();
        let ctx = SyntaxContext::empty();
        let code = ctx.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("SyntaxContext"));
        assert!(code_str.contains("empty"));
    }

    // ==================== Box<T> Tests ====================

    #[test]
    fn test_box_to_code() {
        let cx = empty_ctx();
        let boxed = Box::new(Span::default());
        let code = boxed.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Box :: new"
        assert!(code_str.contains("Box") && code_str.contains("new"));
    }

    // ==================== Option<T> Tests ====================

    #[test]
    fn test_option_some_to_code() {
        let cx = empty_ctx();
        let opt = Some(Span::default());
        let code = opt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Some"));
    }

    #[test]
    fn test_option_none_to_code() {
        let cx = empty_ctx();
        let opt: Option<Span> = None;
        let code = opt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert_eq!(code_str.trim(), "None");
    }

    // ==================== Vec<T> Tests ====================

    #[test]
    fn test_vec_empty_to_code() {
        let cx = empty_ctx();
        let vec: Vec<Span> = vec![];
        let code = vec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Vec :: with_capacity"
        assert!(code_str.contains("Vec") && code_str.contains("with_capacity"));
        assert!(code_str.contains("0"));
    }

    #[test]
    fn test_vec_single_item_to_code() {
        let cx = empty_ctx();
        let vec = vec![Span::default()];
        let code = vec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Vec :: with_capacity"
        assert!(code_str.contains("Vec") && code_str.contains("with_capacity"));
        assert!(code_str.contains("push"));
    }

    #[test]
    fn test_vec_multiple_items_to_code() {
        let cx = empty_ctx();
        let vec = vec![Span::default(), Span::default(), Span::default()];
        let code = vec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Vec :: with_capacity"
        assert!(code_str.contains("Vec") && code_str.contains("with_capacity"));
        assert!(code_str.contains("3"));
    }

    // ==================== Invalid Tests ====================

    #[test]
    fn test_invalid_to_code() {
        let cx = empty_ctx();
        let invalid = Invalid {
            span: Span::default(),
        };
        let code = invalid.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Invalid"));
        assert!(code_str.contains("span"));
    }

    // ==================== ModuleItem Enum Tests ====================

    #[test]
    fn test_module_item_stmt_to_code() {
        let cx = empty_ctx();
        let stmt = Stmt::Empty(EmptyStmt {
            span: Span::default(),
        });
        let module_item = ModuleItem::Stmt(stmt);
        let code = module_item.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ModuleItem"));
        assert!(code_str.contains("Stmt"));
    }

    // ==================== Lit Enum Tests ====================

    #[test]
    fn test_lit_bool_to_code() {
        let cx = empty_ctx();
        let lit = Lit::Bool(Bool {
            span: Span::default(),
            value: true,
        });
        let code = lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Lit"));
        assert!(code_str.contains("Bool"));
    }

    #[test]
    fn test_lit_null_to_code() {
        let cx = empty_ctx();
        let lit = Lit::Null(Null {
            span: Span::default(),
        });
        let code = lit.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Lit"));
        assert!(code_str.contains("Null"));
    }

    // ==================== Pat Enum Tests ====================

    #[test]
    fn test_pat_invalid_to_code() {
        let cx = empty_ctx();
        let pat = Pat::Invalid(Invalid {
            span: Span::default(),
        });
        let code = pat.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Pat"));
        assert!(code_str.contains("Invalid"));
    }

    // ==================== PropOrSpread Tests ====================

    #[test]
    fn test_prop_or_spread_spread_to_code() {
        let cx = empty_ctx();
        let spread = SpreadElement {
            dot3_token: Span::default(),
            expr: Box::new(Expr::Invalid(Invalid {
                span: Span::default(),
            })),
        };
        let prop = PropOrSpread::Spread(spread);
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("PropOrSpread"));
        assert!(code_str.contains("Spread"));
    }

    // ==================== BlockStmtOrExpr Tests ====================

    #[test]
    fn test_block_stmt_or_expr_block_to_code() {
        let cx = empty_ctx();
        let block = BlockStmt {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            stmts: vec![],
        };
        let block_or_expr = BlockStmtOrExpr::BlockStmt(block);
        let code = block_or_expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BlockStmtOrExpr"));
        assert!(code_str.contains("BlockStmt"));
    }

    // ==================== Nested Type Tests ====================

    #[test]
    fn test_box_option_to_code() {
        let cx = empty_ctx();
        let boxed: Box<Option<Span>> = Box::new(Some(Span::default()));
        let code = boxed.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Box :: new"
        assert!(code_str.contains("Box") && code_str.contains("new"));
        assert!(code_str.contains("Some"));
    }

    #[test]
    fn test_option_box_to_code() {
        let cx = empty_ctx();
        let opt: Option<Box<Span>> = Some(Box::new(Span::default()));
        let code = opt.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Some"));
        // Token stream adds spaces: "Box :: new"
        assert!(code_str.contains("Box") && code_str.contains("new"));
    }

    #[test]
    fn test_vec_of_options_to_code() {
        let cx = empty_ctx();
        let vec: Vec<Option<Span>> = vec![Some(Span::default()), None, Some(Span::default())];
        let code = vec.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        // Token stream adds spaces: "Vec :: with_capacity"
        assert!(code_str.contains("Vec") && code_str.contains("with_capacity"));
        assert!(code_str.contains("3"));
    }
}
