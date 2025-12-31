//! Code generation from IR to Rust TokenStream.
//!
//! Generates Rust code that builds SWC AST nodes directly.
//! Each IR node type maps to corresponding SWC AST construction code.

mod class;
mod expr;
mod ident;
mod interface;
mod module_item;
mod node;
mod op;
mod param;
mod pat;
mod prop;
mod signature;
mod stmt;
mod target;
mod r#type;

pub use super::ir::{
    Accessibility, AssignOp, BinaryOp, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind,
    TsKeyword, UnaryOp, UpdateOp, VarDeclarator, VarKind,
};
pub use proc_macro2::TokenStream;
pub use quote::{format_ident, quote};

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub output_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__stmts".to_string(),
        }
    }
}

pub struct Codegen {
    config: CodegenConfig,
}

impl Codegen {
    pub fn new() -> Self {
        Self::with_config(CodegenConfig::default())
    }

    pub fn with_config(config: CodegenConfig) -> Self {
        Self { config }
    }

    /// Generate the complete output for an IR tree.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_module_items(&ir.nodes);

        let result = quote! {
            {
                let mut #output_var: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        };

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!("[MF_DEBUG_CODEGEN] Generated code:\n{}", result);
        }

        result
    }

    fn generate_var_declarators(&self, decls: &[VarDeclarator]) -> TokenStream {
        let decls_code: Vec<TokenStream> = decls
            .iter()
            .map(|d| {
                let name_code = self.generate_pat(&d.name);
                let init_code = d
                    .init
                    .as_ref()
                    .map(|i| {
                        let ic = self.generate_expr(i);
                        quote! { Some(Box::new(#ic)) }
                    })
                    .unwrap_or(quote! { None });

                quote! {
                    macroforge_ts::swc_core::ecma::ast::VarDeclarator {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: #name_code,
                        init: #init_code,
                        definite: false,
                    }
                }
            })
            .collect();

        quote! { vec![#(#decls_code),*] }
    }

    fn generate_entity_name(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                }
            }
            _ => quote! {
                macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        "".into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            },
        }
    }

    fn generate_implements(&self, _implements: &[IrNode]) -> TokenStream {
        quote! { vec![] }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

fn _unused_variants() {
    let _ = UnaryOp::Minus;
    let _ = UpdateOp::Increment;
    let _ = Accessibility::Public;
    let _ = MethodKind::Method;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CodegenConfig Tests ====================

    #[test]
    fn test_codegen_config_default() {
        let config = CodegenConfig::default();
        assert_eq!(config.output_var, "__stmts");
    }

    #[test]
    fn test_codegen_config_custom() {
        let config = CodegenConfig {
            output_var: "my_output".to_string(),
        };
        assert_eq!(config.output_var, "my_output");
    }

    #[test]
    fn test_codegen_config_clone() {
        let config = CodegenConfig {
            output_var: "test_var".to_string(),
        };
        let cloned = config.clone();
        assert_eq!(cloned.output_var, "test_var");
    }

    #[test]
    fn test_codegen_config_debug() {
        let config = CodegenConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("CodegenConfig"));
        assert!(debug_str.contains("output_var"));
        assert!(debug_str.contains("__stmts"));
    }

    // ==================== Codegen Construction Tests ====================

    #[test]
    fn test_codegen_new() {
        let codegen = Codegen::new();
        assert_eq!(codegen.config.output_var, "__stmts");
    }

    #[test]
    fn test_codegen_default() {
        let codegen = Codegen::default();
        assert_eq!(codegen.config.output_var, "__stmts");
    }

    #[test]
    fn test_codegen_with_config() {
        let config = CodegenConfig {
            output_var: "custom_output".to_string(),
        };
        let codegen = Codegen::with_config(config);
        assert_eq!(codegen.config.output_var, "custom_output");
    }

    // ==================== Generate Tests ====================

    #[test]
    fn test_generate_empty_ir() {
        let codegen = Codegen::new();
        let ir = Ir::new();
        let output = codegen.generate(&ir);
        let output_str = output.to_string();

        // Should contain the output variable and vec creation
        assert!(output_str.contains("__stmts"));
        assert!(output_str.contains("Vec"));
        assert!(output_str.contains("ModuleItem"));
    }

    #[test]
    fn test_generate_with_custom_output_var() {
        let config = CodegenConfig {
            output_var: "my_items".to_string(),
        };
        let codegen = Codegen::with_config(config);
        let ir = Ir::new();
        let output = codegen.generate(&ir);
        let output_str = output.to_string();

        assert!(output_str.contains("my_items"));
    }

    #[test]
    fn test_generate_with_ident_node() {
        let codegen = Codegen::new();
        let ir = Ir::with_nodes(vec![IrNode::Ident("test".to_string())]);
        let output = codegen.generate(&ir);
        let output_str = output.to_string();

        // Should contain Ident related code
        assert!(output_str.contains("__stmts"));
    }

    #[test]
    fn test_generate_with_string_literal() {
        let codegen = Codegen::new();
        let ir = Ir::with_nodes(vec![IrNode::StrLit("hello".to_string())]);
        let output = codegen.generate(&ir);
        let output_str = output.to_string();

        assert!(output_str.contains("__stmts"));
    }

    // ==================== Helper Method Tests ====================

    #[test]
    fn test_generate_var_declarators_empty() {
        let codegen = Codegen::new();
        let decls: Vec<VarDeclarator> = vec![];
        let output = codegen.generate_var_declarators(&decls);
        let output_str = output.to_string();

        assert!(output_str.contains("vec"));
    }

    #[test]
    fn test_generate_var_declarators_single() {
        let codegen = Codegen::new();
        let decls = vec![VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            init: None,
            definite: false,
        }];
        let output = codegen.generate_var_declarators(&decls);
        let output_str = output.to_string();

        assert!(output_str.contains("VarDeclarator"));
    }

    #[test]
    fn test_generate_var_declarators_with_init() {
        let codegen = Codegen::new();
        let decls = vec![VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            init: Some(Box::new(IrNode::NumLit("42".to_string()))),
            definite: false,
        }];
        let output = codegen.generate_var_declarators(&decls);
        let output_str = output.to_string();

        assert!(output_str.contains("VarDeclarator"));
        assert!(output_str.contains("init"));
    }

    #[test]
    fn test_generate_entity_name_ident() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("MyType".to_string());
        let output = codegen.generate_entity_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsEntityName"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_entity_name_placeholder() {
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_var },
        };
        let output = codegen.generate_entity_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsEntityName"));
        assert!(output_str.contains("ToTsIdent"));
    }

    #[test]
    fn test_generate_entity_name_fallback() {
        let codegen = Codegen::new();
        let node = IrNode::NumLit("123".to_string()); // Not a valid entity name
        let output = codegen.generate_entity_name(&node);
        let output_str = output.to_string();

        // Should still produce valid entity name (empty string fallback)
        assert!(output_str.contains("TsEntityName"));
    }

    #[test]
    fn test_generate_implements_empty() {
        let codegen = Codegen::new();
        let implements: Vec<IrNode> = vec![];
        let output = codegen.generate_implements(&implements);
        let output_str = output.to_string();

        assert!(output_str.contains("vec"));
    }

    // ==================== Binary Operator Tests ====================

    #[test]
    fn test_generate_binary_op_add() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Add);
        let output_str = output.to_string();
        assert!(output_str.contains("BinaryOp"));
        assert!(output_str.contains("Add"));
    }

    #[test]
    fn test_generate_binary_op_sub() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Sub);
        let output_str = output.to_string();
        assert!(output_str.contains("Sub"));
    }

    #[test]
    fn test_generate_binary_op_mul() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Mul);
        let output_str = output.to_string();
        assert!(output_str.contains("Mul"));
    }

    #[test]
    fn test_generate_binary_op_div() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Div);
        let output_str = output.to_string();
        assert!(output_str.contains("Div"));
    }

    #[test]
    fn test_generate_binary_op_mod() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Mod);
        let output_str = output.to_string();
        assert!(output_str.contains("Mod"));
    }

    #[test]
    fn test_generate_binary_op_exp() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Exp);
        let output_str = output.to_string();
        assert!(output_str.contains("Exp"));
    }

    #[test]
    fn test_generate_binary_op_eq_eq() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::EqEq);
        let output_str = output.to_string();
        assert!(output_str.contains("EqEq"));
    }

    #[test]
    fn test_generate_binary_op_eq_eq_eq() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::EqEqEq);
        let output_str = output.to_string();
        assert!(output_str.contains("EqEqEq"));
    }

    #[test]
    fn test_generate_binary_op_not_eq() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::NotEq);
        let output_str = output.to_string();
        assert!(output_str.contains("NotEq"));
    }

    #[test]
    fn test_generate_binary_op_not_eq_eq() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::NotEqEq);
        let output_str = output.to_string();
        assert!(output_str.contains("NotEqEq"));
    }

    #[test]
    fn test_generate_binary_op_lt() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Lt);
        let output_str = output.to_string();
        assert!(output_str.contains("Lt"));
    }

    #[test]
    fn test_generate_binary_op_le() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Le);
        let output_str = output.to_string();
        assert!(output_str.contains("LtEq"));
    }

    #[test]
    fn test_generate_binary_op_gt() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Gt);
        let output_str = output.to_string();
        assert!(output_str.contains("Gt"));
    }

    #[test]
    fn test_generate_binary_op_ge() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Ge);
        let output_str = output.to_string();
        assert!(output_str.contains("GtEq"));
    }

    #[test]
    fn test_generate_binary_op_and() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::And);
        let output_str = output.to_string();
        assert!(output_str.contains("LogicalAnd"));
    }

    #[test]
    fn test_generate_binary_op_or() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Or);
        let output_str = output.to_string();
        assert!(output_str.contains("LogicalOr"));
    }

    #[test]
    fn test_generate_binary_op_nullish_coalesce() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::NullishCoalesce);
        let output_str = output.to_string();
        assert!(output_str.contains("NullishCoalescing"));
    }

    #[test]
    fn test_generate_binary_op_bit_and() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::BitAnd);
        let output_str = output.to_string();
        assert!(output_str.contains("BitAnd"));
    }

    #[test]
    fn test_generate_binary_op_bit_or() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::BitOr);
        let output_str = output.to_string();
        assert!(output_str.contains("BitOr"));
    }

    #[test]
    fn test_generate_binary_op_bit_xor() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::BitXor);
        let output_str = output.to_string();
        assert!(output_str.contains("BitXor"));
    }

    #[test]
    fn test_generate_binary_op_shl() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Shl);
        let output_str = output.to_string();
        assert!(output_str.contains("LShift"));
    }

    #[test]
    fn test_generate_binary_op_shr() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::Shr);
        let output_str = output.to_string();
        assert!(output_str.contains("RShift"));
    }

    #[test]
    fn test_generate_binary_op_ushr() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::UShr);
        let output_str = output.to_string();
        assert!(output_str.contains("ZeroFillRShift"));
    }

    #[test]
    fn test_generate_binary_op_in() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::In);
        let output_str = output.to_string();
        assert!(output_str.contains("In"));
    }

    #[test]
    fn test_generate_binary_op_instanceof() {
        let codegen = Codegen::new();
        let output = codegen.generate_binary_op(&BinaryOp::InstanceOf);
        let output_str = output.to_string();
        assert!(output_str.contains("InstanceOf"));
    }

    // ==================== Assign Operator Tests ====================

    #[test]
    fn test_generate_assign_op_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::Assign);
        let output_str = output.to_string();
        assert!(output_str.contains("AssignOp"));
        assert!(output_str.contains("Assign"));
    }

    #[test]
    fn test_generate_assign_op_add_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::AddAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("AddAssign"));
    }

    #[test]
    fn test_generate_assign_op_sub_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::SubAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("SubAssign"));
    }

    #[test]
    fn test_generate_assign_op_mul_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::MulAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("MulAssign"));
    }

    #[test]
    fn test_generate_assign_op_div_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::DivAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("DivAssign"));
    }

    #[test]
    fn test_generate_assign_op_mod_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::ModAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("ModAssign"));
    }

    #[test]
    fn test_generate_assign_op_exp_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::ExpAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("ExpAssign"));
    }

    #[test]
    fn test_generate_assign_op_shl_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::ShlAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("LShiftAssign"));
    }

    #[test]
    fn test_generate_assign_op_shr_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::ShrAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("RShiftAssign"));
    }

    #[test]
    fn test_generate_assign_op_ushr_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::UShrAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("ZeroFillRShiftAssign"));
    }

    #[test]
    fn test_generate_assign_op_bit_and_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::BitAndAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("BitAndAssign"));
    }

    #[test]
    fn test_generate_assign_op_bit_or_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::BitOrAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("BitOrAssign"));
    }

    #[test]
    fn test_generate_assign_op_bit_xor_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::BitXorAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("BitXorAssign"));
    }

    #[test]
    fn test_generate_assign_op_and_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::AndAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("AndAssign"));
    }

    #[test]
    fn test_generate_assign_op_or_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::OrAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("OrAssign"));
    }

    #[test]
    fn test_generate_assign_op_nullish_assign() {
        let codegen = Codegen::new();
        let output = codegen.generate_assign_op(&AssignOp::NullishAssign);
        let output_str = output.to_string();
        assert!(output_str.contains("NullishAssign"));
    }

    // ==================== Expression Generation Tests ====================

    #[test]
    fn test_generate_expr_ident() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("myVar".to_string());
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_expr_str_lit() {
        let codegen = Codegen::new();
        let node = IrNode::StrLit("hello world".to_string());
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Lit"));
        assert!(output_str.contains("Str"));
    }

    #[test]
    fn test_generate_expr_num_lit() {
        let codegen = Codegen::new();
        let node = IrNode::NumLit("42".to_string());
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Lit"));
        assert!(output_str.contains("Num"));
    }

    #[test]
    fn test_generate_expr_bool_lit_true() {
        let codegen = Codegen::new();
        let node = IrNode::BoolLit(true);
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Bool"));
        assert!(output_str.contains("true"));
    }

    #[test]
    fn test_generate_expr_bool_lit_false() {
        let codegen = Codegen::new();
        let node = IrNode::BoolLit(false);
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Bool"));
        assert!(output_str.contains("false"));
    }

    #[test]
    fn test_generate_expr_null_lit() {
        let codegen = Codegen::new();
        let node = IrNode::NullLit;
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Null"));
    }

    #[test]
    fn test_generate_expr_this() {
        let codegen = Codegen::new();
        let node = IrNode::ThisExpr;
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("This"));
    }

    #[test]
    fn test_generate_expr_call() {
        let codegen = Codegen::new();
        let node = IrNode::CallExpr {
            callee: Box::new(IrNode::Ident("foo".to_string())),
            type_args: None,
            args: vec![IrNode::NumLit("1".to_string())],
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Call"));
        assert!(output_str.contains("Callee"));
    }

    #[test]
    fn test_generate_expr_member() {
        let codegen = Codegen::new();
        let node = IrNode::MemberExpr {
            obj: Box::new(IrNode::Ident("obj".to_string())),
            prop: Box::new(IrNode::Ident("prop".to_string())),
            computed: false,
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Member"));
        assert!(output_str.contains("MemberProp"));
    }

    #[test]
    fn test_generate_expr_member_computed() {
        let codegen = Codegen::new();
        let node = IrNode::MemberExpr {
            obj: Box::new(IrNode::Ident("arr".to_string())),
            prop: Box::new(IrNode::NumLit("0".to_string())),
            computed: true,
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Member"));
        assert!(output_str.contains("Computed"));
    }

    #[test]
    fn test_generate_expr_object_lit() {
        let codegen = Codegen::new();
        let node = IrNode::ObjectLit {
            props: vec![IrNode::KeyValueProp {
                key: Box::new(IrNode::Ident("x".to_string())),
                value: Box::new(IrNode::NumLit("1".to_string())),
            }],
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Object"));
        assert!(output_str.contains("ObjectLit"));
    }

    #[test]
    fn test_generate_expr_array_lit() {
        let codegen = Codegen::new();
        let node = IrNode::ArrayLit {
            elems: vec![
                IrNode::NumLit("1".to_string()),
                IrNode::NumLit("2".to_string()),
            ],
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Array"));
        assert!(output_str.contains("ArrayLit"));
    }

    #[test]
    fn test_generate_expr_bin() {
        let codegen = Codegen::new();
        let node = IrNode::BinExpr {
            left: Box::new(IrNode::NumLit("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(IrNode::NumLit("2".to_string())),
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Bin"));
        assert!(output_str.contains("BinExpr"));
    }

    #[test]
    fn test_generate_expr_assign() {
        let codegen = Codegen::new();
        let node = IrNode::AssignExpr {
            left: Box::new(IrNode::Ident("x".to_string())),
            op: AssignOp::Assign,
            right: Box::new(IrNode::NumLit("5".to_string())),
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Assign"));
        assert!(output_str.contains("AssignExpr"));
    }

    #[test]
    fn test_generate_expr_cond() {
        let codegen = Codegen::new();
        let node = IrNode::CondExpr {
            test: Box::new(IrNode::BoolLit(true)),
            cons: Box::new(IrNode::NumLit("1".to_string())),
            alt: Box::new(IrNode::NumLit("0".to_string())),
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Cond"));
        assert!(output_str.contains("CondExpr"));
    }

    #[test]
    fn test_generate_expr_new() {
        let codegen = Codegen::new();
        let node = IrNode::NewExpr {
            callee: Box::new(IrNode::Ident("Date".to_string())),
            type_args: None,
            args: vec![],
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("New"));
        assert!(output_str.contains("NewExpr"));
    }

    #[test]
    fn test_generate_expr_arrow() {
        let codegen = Codegen::new();
        let node = IrNode::ArrowExpr {
            async_: false,
            type_params: None,
            params: vec![IrNode::Ident("x".to_string())],
            return_type: None,
            body: Box::new(IrNode::Ident("x".to_string())),
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Arrow"));
        assert!(output_str.contains("ArrowExpr"));
    }

    #[test]
    fn test_generate_expr_arrow_async() {
        let codegen = Codegen::new();
        let node = IrNode::ArrowExpr {
            async_: true,
            type_params: None,
            params: vec![],
            return_type: None,
            body: Box::new(IrNode::NullLit),
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("is_async"));
        assert!(output_str.contains("true"));
    }

    #[test]
    fn test_generate_expr_tpl_lit() {
        let codegen = Codegen::new();
        let node = IrNode::TplLit {
            quasis: vec!["hello ".to_string(), "!".to_string()],
            exprs: vec![IrNode::Ident("name".to_string())],
        };
        let output = codegen.generate_expr(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Tpl"));
        assert!(output_str.contains("TplElement"));
    }

    // ==================== Pattern Generation Tests ====================

    #[test]
    fn test_generate_pat_ident() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("x".to_string());
        let output = codegen.generate_pat(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_pat_binding_ident() {
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            optional: false,
        };
        let output = codegen.generate_pat(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("BindingIdent"));
    }

    #[test]
    fn test_generate_pat_binding_ident_with_type() {
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: Some(Box::new(IrNode::KeywordType(TsKeyword::String))),
            optional: false,
        };
        let output = codegen.generate_pat(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("type_ann"));
    }

    #[test]
    fn test_generate_pat_placeholder() {
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_name },
        };
        let output = codegen.generate_pat(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("ToTsIdent"));
    }

    // ==================== Type Generation Tests ====================

    #[test]
    fn test_generate_type_keyword_string() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::String);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsKeywordType"));
        assert!(output_str.contains("TsStringKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_number() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Number);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsNumberKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_boolean() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Boolean);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsBooleanKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_any() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Any);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsAnyKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_unknown() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Unknown);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsUnknownKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_void() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Void);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsVoidKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_null() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Null);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsNullKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_undefined() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Undefined);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsUndefinedKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_never() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Never);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsNeverKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_object() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Object);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsObjectKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_bigint() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::BigInt);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsBigIntKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_symbol() {
        let codegen = Codegen::new();
        let node = IrNode::KeywordType(TsKeyword::Symbol);
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsSymbolKeyword"));
    }

    #[test]
    fn test_generate_type_ref() {
        let codegen = Codegen::new();
        let node = IrNode::TypeRef {
            name: Box::new(IrNode::Ident("MyType".to_string())),
            type_params: None,
        };
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsTypeRef"));
    }

    #[test]
    fn test_generate_type_union() {
        let codegen = Codegen::new();
        let node = IrNode::UnionType {
            types: vec![
                IrNode::KeywordType(TsKeyword::String),
                IrNode::KeywordType(TsKeyword::Number),
            ],
        };
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsUnionType"));
    }

    #[test]
    fn test_generate_type_array() {
        let codegen = Codegen::new();
        let node = IrNode::ArrayType {
            elem: Box::new(IrNode::KeywordType(TsKeyword::String)),
        };
        let output = codegen.generate_type(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("TsArrayType"));
    }

    // ==================== Statement Generation Tests ====================

    #[test]
    fn test_generate_stmt_expr() {
        let codegen = Codegen::new();
        let node = IrNode::ExprStmt {
            expr: Box::new(IrNode::Ident("foo".to_string())),
        };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Stmt"));
        assert!(output_str.contains("Expr"));
    }

    #[test]
    fn test_generate_stmt_return() {
        let codegen = Codegen::new();
        let node = IrNode::ReturnStmt {
            arg: Some(Box::new(IrNode::NumLit("42".to_string()))),
        };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Return"));
    }

    #[test]
    fn test_generate_stmt_return_void() {
        let codegen = Codegen::new();
        let node = IrNode::ReturnStmt { arg: None };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Return"));
        assert!(output_str.contains("None"));
    }

    #[test]
    fn test_generate_stmt_block() {
        let codegen = Codegen::new();
        let node = IrNode::BlockStmt { stmts: vec![] };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Block"));
        assert!(output_str.contains("BlockStmt"));
    }

    #[test]
    fn test_generate_stmt_var_decl() {
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![VarDeclarator {
                name: Box::new(IrNode::Ident("x".to_string())),
                type_ann: None,
                init: Some(Box::new(IrNode::NumLit("1".to_string()))),
                definite: false,
            }],
        };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Decl"));
        assert!(output_str.contains("VarDecl"));
        assert!(output_str.contains("Const"));
    }

    #[test]
    fn test_generate_stmt_var_decl_let() {
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            exported: false,
            declare: false,
            kind: VarKind::Let,
            decls: vec![],
        };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Let"));
    }

    #[test]
    fn test_generate_stmt_var_decl_var() {
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            exported: false,
            declare: false,
            kind: VarKind::Var,
            decls: vec![],
        };
        let output = codegen.generate_stmt(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Var"));
    }

    // ==================== Class Member Tests ====================

    #[test]
    fn test_generate_class_member_constructor() {
        let codegen = Codegen::new();
        let node = IrNode::Constructor {
            accessibility: None,
            params: vec![],
            body: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Constructor"));
    }

    #[test]
    fn test_generate_class_member_constructor_public() {
        let codegen = Codegen::new();
        let node = IrNode::Constructor {
            accessibility: Some(Accessibility::Public),
            params: vec![],
            body: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Public"));
    }

    #[test]
    fn test_generate_class_member_method() {
        let codegen = Codegen::new();
        let node = IrNode::Method {
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Method,
            name: Box::new(IrNode::Ident("myMethod".to_string())),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("ClassMethod"));
    }

    #[test]
    fn test_generate_class_member_method_getter() {
        let codegen = Codegen::new();
        let node = IrNode::Method {
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Getter,
            name: Box::new(IrNode::Ident("value".to_string())),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Getter"));
    }

    #[test]
    fn test_generate_class_member_method_setter() {
        let codegen = Codegen::new();
        let node = IrNode::Method {
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Setter,
            name: Box::new(IrNode::Ident("value".to_string())),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Setter"));
    }

    #[test]
    fn test_generate_class_member_prop() {
        let codegen = Codegen::new();
        let node = IrNode::ClassProp {
            static_: false,
            accessibility: None,
            readonly: false,
            declare: false,
            optional: false,
            definite: false,
            name: Box::new(IrNode::Ident("myProp".to_string())),
            type_ann: None,
            value: None,
        };
        let output = codegen.generate_class_member(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("ClassProp"));
    }

    // ==================== Node Detection Tests ====================

    #[test]
    fn test_is_fragment_node_raw() {
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::Raw("text".to_string())));
    }

    #[test]
    fn test_is_fragment_node_ident() {
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::Ident("name".to_string())));
    }

    #[test]
    fn test_is_fragment_node_str_lit() {
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::StrLit("hello".to_string())));
    }

    #[test]
    fn test_is_fragment_node_placeholder() {
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            kind: PlaceholderKind::Expr,
            expr: syn::parse_quote! { x },
        };
        assert!(codegen.is_fragment_node(&node));
    }

    #[test]
    fn test_is_fragment_node_non_fragment() {
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![],
        };
        assert!(!codegen.is_fragment_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_for() {
        let codegen = Codegen::new();
        let node = IrNode::For {
            pattern: syn::parse_quote! { x },
            iterator: syn::parse_quote! { items },
            body: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_if() {
        let codegen = Codegen::new();
        let node = IrNode::If {
            condition: syn::parse_quote! { true },
            then_body: vec![],
            else_if_branches: vec![],
            else_body: None,
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_while() {
        let codegen = Codegen::new();
        let node = IrNode::While {
            condition: syn::parse_quote! { true },
            body: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_match() {
        let codegen = Codegen::new();
        let node = IrNode::Match {
            expr: syn::parse_quote! { x },
            arms: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_let() {
        let codegen = Codegen::new();
        let node = IrNode::Let {
            pattern: syn::parse_quote! { x },
            mutable: false,
            type_hint: None,
            value: syn::parse_quote! { 1 },
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_do() {
        let codegen = Codegen::new();
        let node = IrNode::Do {
            code: syn::parse_quote! { println!("hi") },
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_non_control_flow() {
        let codegen = Codegen::new();
        assert!(!codegen.is_control_flow_node(&IrNode::Ident("x".to_string())));
    }

    // ==================== Identifier Generation Tests ====================

    #[test]
    fn test_generate_ident_basic() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("myVar".to_string());
        let output = codegen.generate_ident(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Ident"));
        assert!(output_str.contains("new_no_ctxt"));
    }

    #[test]
    fn test_generate_ident_placeholder() {
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_var },
        };
        let output = codegen.generate_ident(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("ToTsIdent"));
    }

    #[test]
    fn test_generate_ident_name_basic() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("propName".to_string());
        let output = codegen.generate_ident_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("IdentName"));
    }

    // ==================== Property Generation Tests ====================

    #[test]
    fn test_generate_prop_key_value() {
        let codegen = Codegen::new();
        let node = IrNode::KeyValueProp {
            key: Box::new(IrNode::Ident("x".to_string())),
            value: Box::new(IrNode::NumLit("1".to_string())),
        };
        let output = codegen.generate_prop(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("KeyValue"));
    }

    #[test]
    fn test_generate_prop_shorthand() {
        let codegen = Codegen::new();
        let node = IrNode::ShorthandProp {
            key: Box::new(IrNode::Ident("x".to_string())),
        };
        let output = codegen.generate_prop(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Shorthand"));
    }

    #[test]
    fn test_generate_prop_spread() {
        let codegen = Codegen::new();
        let node = IrNode::SpreadElement {
            expr: Box::new(IrNode::Ident("obj".to_string())),
        };
        let output = codegen.generate_prop(&node);

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Spread"));
    }

    #[test]
    fn test_generate_prop_name_ident() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("propName".to_string());
        let output = codegen.generate_prop_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_prop_name_str() {
        let codegen = Codegen::new();
        let node = IrNode::StrLit("prop-name".to_string());
        let output = codegen.generate_prop_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Str"));
    }

    #[test]
    fn test_generate_prop_name_computed() {
        let codegen = Codegen::new();
        let node = IrNode::ComputedPropName {
            expr: Box::new(IrNode::Ident("key".to_string())),
        };
        let output = codegen.generate_prop_name(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Computed"));
    }

    // ==================== Parameter Generation Tests ====================

    #[test]
    fn test_generate_param_ident() {
        let codegen = Codegen::new();
        let node = IrNode::Ident("x".to_string());
        let output = codegen.generate_param(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
        assert!(output_str.contains("Pat"));
    }

    #[test]
    fn test_generate_param_binding_ident() {
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: Some(Box::new(IrNode::KeywordType(TsKeyword::String))),
            optional: false,
        };
        let output = codegen.generate_param(&node);
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
        assert!(output_str.contains("type_ann"));
    }

    #[test]
    fn test_generate_params_empty() {
        let codegen = Codegen::new();
        let params: Vec<IrNode> = vec![];
        let output = codegen.generate_params(&params);
        let output_str = output.to_string();

        assert!(output_str.contains("vec"));
    }

    #[test]
    fn test_generate_params_multiple() {
        let codegen = Codegen::new();
        let params = vec![
            IrNode::Ident("a".to_string()),
            IrNode::Ident("b".to_string()),
        ];
        let output = codegen.generate_params(&params);
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
    }
}
