//! Code generation from IR to Rust TokenStream.
//!
//! Generates Rust code that builds SWC AST nodes directly.
//! Each IR node type maps to corresponding SWC AST construction code.
//! No silent fallbacks - all errors are explicit via GenResult.

mod class;
mod error;
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

pub use error::{GenError, GenErrorKind, GenResult};

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
    pub fn generate(&self, ir: &Ir) -> GenResult<TokenStream> {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_module_items(&ir.nodes)?;

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

        Ok(result)
    }

    fn generate_var_declarators(&self, decls: &[VarDeclarator]) -> GenResult<TokenStream> {
        let mut decls_code: Vec<TokenStream> = Vec::new();

        for d in decls {
            // Consume span field (we use DUMMY_SP for generated code)
            let _ = d.span;
            let definite = d.definite;

            let name_code = self.generate_pat(&d.name)?;
            let init_code = match &d.init {
                Some(i) => {
                    let ic = self.generate_expr(i)?;
                    quote! { Some(Box::new(#ic)) }
                }
                None => quote! { None },
            };

            decls_code.push(quote! {
                macroforge_ts::swc_core::ecma::ast::VarDeclarator {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    name: #name_code,
                    init: #init_code,
                    definite: #definite,
                }
            });
        }

        Ok(quote! { vec![#(#decls_code),*] })
    }

    fn generate_entity_name(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Ident { value: name, .. } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    )
                })
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
                ..
            } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                })
            }
            // Handle Placeholder with Expr kind - try to convert expression to entity name
            IrNode::Placeholder {
                kind: PlaceholderKind::Expr,
                expr,
                ..
            } => {
                // For expression placeholders, assume they evaluate to something that can be
                // converted to an identifier (string, &str, Ident, etc.)
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                })
            }
            // Handle Placeholder with Type kind - for type references used as entity names
            IrNode::Placeholder {
                kind: PlaceholderKind::Type,
                expr,
                ..
            } => {
                // For type placeholders in entity name position, convert to identifier
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                })
            }
            // Handle Placeholder with Stmt kind - shouldn't typically happen, but handle gracefully
            IrNode::Placeholder {
                kind: PlaceholderKind::Stmt,
                expr,
                ..
            } => {
                // For statement placeholders in entity name position, try to convert to identifier
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                })
            }
            // Handle TypeScript injection node - the stream produces an entity name directly
            IrNode::TypeScript { stream, .. } => {
                Ok(quote! { #stream })
            }
            other => Err(GenError::unexpected_node(
                "entity name",
                other,
                &["Ident", "Placeholder", "TypeScript"],
            )),
        }
    }

    fn generate_implements(&self, implements: &[IrNode]) -> GenResult<TokenStream> {
        if implements.is_empty() {
            return Ok(quote! { vec![] });
        }

        let mut impl_codes = Vec::new();
        for node in implements {
            let entity_name = self.generate_entity_name(node)?;
            impl_codes.push(quote! {
                macroforge_ts::swc_core::ecma::ast::TsExprWithTypeArgs {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                        match #entity_name {
                            macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(i) => i,
                            macroforge_ts::swc_core::ecma::ast::TsEntityName::TsQualifiedName(q) => {
                                // For qualified names, extract the rightmost identifier
                                q.right.into()
                            }
                        }
                    )),
                    type_args: None,
                }
            });
        }

        Ok(quote! { vec![#(#impl_codes),*] })
    }

    /// Generate a single decorator node.
    pub(crate) fn generate_decorator(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Decorator { expr, .. } => {
                let expr_code = self.generate_expr(expr)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Decorator {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                })
            }
            _ => Err(GenError::unexpected_node(
                "decorator",
                node,
                &["Decorator"],
            )),
        }
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
        let output = codegen.generate(&ir).unwrap();
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
        let output = codegen.generate(&ir).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("my_items"));
    }

    #[test]
    fn test_generate_with_ident_node() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let ir = Ir::with_nodes(vec![IrNode::Ident { span: IrSpan::empty(), value: "test".to_string() }]);
        let output = codegen.generate(&ir).unwrap();
        let output_str = output.to_string();

        // Should contain Ident related code
        assert!(output_str.contains("__stmts"));
    }

    #[test]
    fn test_generate_with_string_literal() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let ir = Ir::with_nodes(vec![IrNode::StrLit { span: IrSpan::empty(), value: "hello".to_string() }]);
        let output = codegen.generate(&ir).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("__stmts"));
    }

    // ==================== Helper Method Tests ====================

    #[test]
    fn test_generate_var_declarators_empty() {
        let codegen = Codegen::new();
        let decls: Vec<VarDeclarator> = vec![];
        let output = codegen.generate_var_declarators(&decls).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("vec"));
    }

    #[test]
    fn test_generate_var_declarators_single() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let decls = vec![VarDeclarator {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            type_ann: None,
            init: None,
            definite: false,
        }];
        let output = codegen.generate_var_declarators(&decls).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("VarDeclarator"));
    }

    #[test]
    fn test_generate_var_declarators_with_init() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let decls = vec![VarDeclarator {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            type_ann: None,
            init: Some(Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "42".to_string() })),
            definite: false,
        }];
        let output = codegen.generate_var_declarators(&decls).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("VarDeclarator"));
        assert!(output_str.contains("init"));
    }

    #[test]
    fn test_generate_entity_name_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "MyType".to_string() };
        let output = codegen.generate_entity_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsEntityName"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_entity_name_placeholder() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            span: IrSpan::empty(),
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_var },
        };
        let output = codegen.generate_entity_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsEntityName"));
        assert!(output_str.contains("ToTsIdent"));
    }

    #[test]
    fn test_generate_entity_name_invalid_returns_error() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::NumLit { span: IrSpan::empty(), value: "123".to_string() }; // Not a valid entity name
        let output = codegen.generate_entity_name(&node);

        // Should now return an error instead of silently producing invalid code
        assert!(output.is_err());
    }

    #[test]
    fn test_generate_implements_empty() {
        let codegen = Codegen::new();
        let implements: Vec<IrNode> = vec![];
        let output = codegen.generate_implements(&implements).unwrap();
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
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "myVar".to_string() };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_expr_str_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::StrLit { span: IrSpan::empty(), value: "hello world".to_string() };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Lit"));
        assert!(output_str.contains("Str"));
    }

    #[test]
    fn test_generate_expr_num_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::NumLit { span: IrSpan::empty(), value: "42".to_string() };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Expr"));
        assert!(output_str.contains("Lit"));
        assert!(output_str.contains("Num"));
    }

    #[test]
    fn test_generate_expr_bool_lit_true() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BoolLit { span: IrSpan::empty(), value: true };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Bool"));
        assert!(output_str.contains("true"));
    }

    #[test]
    fn test_generate_expr_bool_lit_false() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BoolLit { span: IrSpan::empty(), value: false };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Bool"));
        assert!(output_str.contains("false"));
    }

    #[test]
    fn test_generate_expr_null_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::NullLit { span: IrSpan::empty() };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Null"));
    }

    #[test]
    fn test_generate_expr_this() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ThisExpr { span: IrSpan::empty() };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("This"));
    }

    #[test]
    fn test_generate_expr_call() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::CallExpr {
            span: IrSpan::empty(),
            callee: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "foo".to_string() }),
            type_args: None,
            args: vec![IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() }],
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Call"));
        assert!(output_str.contains("Callee"));
    }

    #[test]
    fn test_generate_expr_member() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::MemberExpr {
            span: IrSpan::empty(),
            obj: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "obj".to_string() }),
            prop: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "prop".to_string() }),
            computed: false,
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Member"));
        assert!(output_str.contains("MemberProp"));
    }

    #[test]
    fn test_generate_expr_member_computed() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::MemberExpr {
            span: IrSpan::empty(),
            obj: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "arr".to_string() }),
            prop: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "0".to_string() }),
            computed: true,
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Member"));
        assert!(output_str.contains("Computed"));
    }

    #[test]
    fn test_generate_expr_object_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ObjectLit {
            span: IrSpan::empty(),
            props: vec![IrNode::KeyValueProp {
                span: IrSpan::empty(),
                key: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
                value: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() }),
            }],
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Object"));
        assert!(output_str.contains("ObjectLit"));
    }

    #[test]
    fn test_generate_expr_array_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ArrayLit {
            span: IrSpan::empty(),
            elems: vec![
                IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() },
                IrNode::NumLit { span: IrSpan::empty(), value: "2".to_string() },
            ],
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Array"));
        assert!(output_str.contains("ArrayLit"));
    }

    #[test]
    fn test_generate_expr_bin() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BinExpr {
            span: IrSpan::empty(),
            left: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() }),
            op: BinaryOp::Add,
            right: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "2".to_string() }),
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Bin"));
        assert!(output_str.contains("BinExpr"));
    }

    #[test]
    fn test_generate_expr_assign() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::AssignExpr {
            span: IrSpan::empty(),
            left: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            op: AssignOp::Assign,
            right: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "5".to_string() }),
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Assign"));
        assert!(output_str.contains("AssignExpr"));
    }

    #[test]
    fn test_generate_expr_cond() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::CondExpr {
            span: IrSpan::empty(),
            test: Box::new(IrNode::BoolLit { span: IrSpan::empty(), value: true }),
            consequent: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() }),
            alternate: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "0".to_string() }),
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Cond"));
        assert!(output_str.contains("CondExpr"));
    }

    #[test]
    fn test_generate_expr_new() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::NewExpr {
            span: IrSpan::empty(),
            callee: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "Date".to_string() }),
            type_args: None,
            args: vec![],
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("New"));
        assert!(output_str.contains("NewExpr"));
    }

    #[test]
    fn test_generate_expr_arrow() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ArrowExpr {
            span: IrSpan::empty(),
            async_: false,
            type_params: None,
            params: vec![IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }],
            return_type: None,
            body: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Arrow"));
        assert!(output_str.contains("ArrowExpr"));
    }

    #[test]
    fn test_generate_expr_arrow_async() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ArrowExpr {
            span: IrSpan::empty(),
            async_: true,
            type_params: None,
            params: vec![],
            return_type: None,
            body: Box::new(IrNode::NullLit { span: IrSpan::empty() }),
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("is_async"));
        assert!(output_str.contains("true"));
    }

    #[test]
    fn test_generate_expr_tpl_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::TplLit {
            span: IrSpan::empty(),
            quasis: vec!["hello ".to_string(), "!".to_string()],
            exprs: vec![IrNode::Ident { span: IrSpan::empty(), value: "name".to_string() }],
        };
        let output = codegen.generate_expr(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Tpl"));
        assert!(output_str.contains("TplElement"));
    }

    // ==================== Pattern Generation Tests ====================

    #[test]
    fn test_generate_pat_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() };
        let output = codegen.generate_pat(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_pat_binding_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            type_ann: None,
            optional: false,
        };
        let output = codegen.generate_pat(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("BindingIdent"));
    }

    #[test]
    fn test_generate_pat_binding_ident_with_type() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            type_ann: Some(Box::new(IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::String })),
            optional: false,
        };
        let output = codegen.generate_pat(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("type_ann"));
    }

    #[test]
    fn test_generate_pat_placeholder() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            span: IrSpan::empty(),
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_name },
        };
        let output = codegen.generate_pat(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Pat"));
        assert!(output_str.contains("ToTsIdent"));
    }

    // ==================== Type Generation Tests ====================

    #[test]
    fn test_generate_type_keyword_string() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::String };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsKeywordType"));
        assert!(output_str.contains("TsStringKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_number() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Number };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsNumberKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_boolean() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Boolean };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsBooleanKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_any() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Any };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsAnyKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_unknown() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Unknown };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsUnknownKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_void() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Void };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsVoidKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_null() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Null };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsNullKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_undefined() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Undefined };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsUndefinedKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_never() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Never };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsNeverKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_object() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Object };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsObjectKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_bigint() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::BigInt };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsBigIntKeyword"));
    }

    #[test]
    fn test_generate_type_keyword_symbol() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Symbol };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsSymbolKeyword"));
    }

    #[test]
    fn test_generate_type_ref() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::TypeRef {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "MyType".to_string() }),
            type_params: None,
        };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsTypeRef"));
    }

    #[test]
    fn test_generate_type_union() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::UnionType {
            span: IrSpan::empty(),
            types: vec![
                IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::String },
                IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::Number },
            ],
        };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsUnionType"));
    }

    #[test]
    fn test_generate_type_array() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ArrayType {
            span: IrSpan::empty(),
            elem: Box::new(IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::String }),
        };
        let output = codegen.generate_type(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("TsArrayType"));
    }

    // ==================== Statement Generation Tests ====================

    #[test]
    fn test_generate_stmt_expr() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ExprStmt {
            span: IrSpan::empty(),
            expr: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "foo".to_string() }),
        };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Stmt"));
        assert!(output_str.contains("Expr"));
    }

    #[test]
    fn test_generate_stmt_return() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ReturnStmt {
            span: IrSpan::empty(),
            arg: Some(Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "42".to_string() })),
        };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Return"));
    }

    #[test]
    fn test_generate_stmt_return_void() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ReturnStmt { span: IrSpan::empty(), arg: None };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Return"));
        assert!(output_str.contains("None"));
    }

    #[test]
    fn test_generate_stmt_block() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BlockStmt { span: IrSpan::empty(), stmts: vec![] };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Block"));
        assert!(output_str.contains("BlockStmt"));
    }

    #[test]
    fn test_generate_stmt_var_decl() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![VarDeclarator {
                span: IrSpan::empty(),
                name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
                type_ann: None,
                init: Some(Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() })),
                definite: false,
            }],
        };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Decl"));
        assert!(output_str.contains("VarDecl"));
        assert!(output_str.contains("Const"));
    }

    #[test]
    fn test_generate_stmt_var_decl_let() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            kind: VarKind::Let,
            decls: vec![],
        };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Let"));
    }

    #[test]
    fn test_generate_stmt_var_decl_var() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            kind: VarKind::Var,
            decls: vec![],
        };
        let output = codegen.generate_stmt(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Var"));
    }

    // ==================== Class Member Tests ====================

    #[test]
    fn test_generate_class_member_constructor() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Constructor {
            span: IrSpan::empty(),
            accessibility: None,
            params: vec![],
            body: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Constructor"));
    }

    #[test]
    fn test_generate_class_member_constructor_public() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Constructor {
            span: IrSpan::empty(),
            accessibility: Some(Accessibility::Public),
            params: vec![],
            body: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Public"));
    }

    #[test]
    fn test_generate_class_member_method() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Method {
            span: IrSpan::empty(),
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Method,
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "myMethod".to_string() }),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("ClassMethod"));
    }

    #[test]
    fn test_generate_class_member_method_getter() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Method {
            span: IrSpan::empty(),
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Getter,
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "value".to_string() }),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Getter"));
    }

    #[test]
    fn test_generate_class_member_method_setter() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Method {
            span: IrSpan::empty(),
            static_: false,
            accessibility: None,
            readonly: false,
            async_: false,
            generator: false,
            kind: MethodKind::Setter,
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "value".to_string() }),
            optional: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("Setter"));
    }

    #[test]
    fn test_generate_class_member_prop() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ClassProp {
            span: IrSpan::empty(),
            static_: false,
            accessibility: None,
            readonly: false,
            declare: false,
            optional: false,
            definite: false,
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "myProp".to_string() }),
            type_ann: None,
            value: None,
        };
        let output = codegen.generate_class_member(&node).unwrap();
        let output_str = output.to_string();
        assert!(output_str.contains("ClassProp"));
    }

    // ==================== Node Detection Tests ====================

    #[test]
    fn test_is_fragment_node_raw() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::Raw { span: IrSpan::empty(), value: "text".to_string() }));
    }

    #[test]
    fn test_is_fragment_node_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::Ident { span: IrSpan::empty(), value: "name".to_string() }));
    }

    #[test]
    fn test_is_fragment_node_str_lit() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        assert!(codegen.is_fragment_node(&IrNode::StrLit { span: IrSpan::empty(), value: "hello".to_string() }));
    }

    #[test]
    fn test_is_fragment_node_placeholder() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            span: IrSpan::empty(),
            kind: PlaceholderKind::Expr,
            expr: syn::parse_quote! { x },
        };
        assert!(codegen.is_fragment_node(&node));
    }

    #[test]
    fn test_is_fragment_node_non_fragment() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::VarDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![],
        };
        assert!(!codegen.is_fragment_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_for() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::For {
            span: IrSpan::empty(),
            pattern: syn::parse_quote! { x },
            iterator: syn::parse_quote! { items },
            body: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_if() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::If {
            span: IrSpan::empty(),
            condition: syn::parse_quote! { true },
            then_body: vec![],
            else_if_branches: vec![],
            else_body: None,
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_while() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::While {
            span: IrSpan::empty(),
            condition: syn::parse_quote! { true },
            body: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_match() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Match {
            span: IrSpan::empty(),
            expr: syn::parse_quote! { x },
            arms: vec![],
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_let() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Let {
            span: IrSpan::empty(),
            pattern: syn::parse_quote! { x },
            mutable: false,
            type_hint: None,
            value: syn::parse_quote! { 1 },
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_do() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Do {
            span: IrSpan::empty(),
            code: syn::parse_quote! { println!("hi") },
        };
        assert!(codegen.is_control_flow_node(&node));
    }

    #[test]
    fn test_is_control_flow_node_non_control_flow() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        assert!(!codegen.is_control_flow_node(&IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }));
    }

    // ==================== Identifier Generation Tests ====================

    #[test]
    fn test_generate_ident_basic() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "myVar".to_string() };
        let output = codegen.generate_ident(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Ident"));
        assert!(output_str.contains("new_no_ctxt"));
    }

    #[test]
    fn test_generate_ident_placeholder() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Placeholder {
            span: IrSpan::empty(),
            kind: PlaceholderKind::Ident,
            expr: syn::parse_quote! { my_var },
        };
        let output = codegen.generate_ident(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("ToTsIdent"));
    }

    #[test]
    fn test_generate_ident_name_basic() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "propName".to_string() };
        let output = codegen.generate_ident_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("IdentName"));
    }

    // ==================== Property Generation Tests ====================

    #[test]
    fn test_generate_prop_key_value() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::KeyValueProp {
            span: IrSpan::empty(),
            key: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            value: Box::new(IrNode::NumLit { span: IrSpan::empty(), value: "1".to_string() }),
        };
        let output = codegen.generate_prop(&node).unwrap();

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("KeyValue"));
    }

    #[test]
    fn test_generate_prop_shorthand() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ShorthandProp {
            span: IrSpan::empty(),
            key: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
        };
        let output = codegen.generate_prop(&node).unwrap();

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Shorthand"));
    }

    #[test]
    fn test_generate_prop_spread() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::SpreadElement {
            span: IrSpan::empty(),
            expr: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "obj".to_string() }),
        };
        let output = codegen.generate_prop(&node).unwrap();

        assert!(output.is_some());
        let output_str = output.unwrap().to_string();
        assert!(output_str.contains("Spread"));
    }

    #[test]
    fn test_generate_prop_name_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "propName".to_string() };
        let output = codegen.generate_prop_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Ident"));
    }

    #[test]
    fn test_generate_prop_name_str() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::StrLit { span: IrSpan::empty(), value: "prop-name".to_string() };
        let output = codegen.generate_prop_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Str"));
    }

    #[test]
    fn test_generate_prop_name_computed() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::ComputedPropName {
            span: IrSpan::empty(),
            expr: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "key".to_string() }),
        };
        let output = codegen.generate_prop_name(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("PropName"));
        assert!(output_str.contains("Computed"));
    }

    // ==================== Parameter Generation Tests ====================

    #[test]
    fn test_generate_param_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() };
        let output = codegen.generate_param(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
        assert!(output_str.contains("Pat"));
    }

    #[test]
    fn test_generate_param_binding_ident() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let node = IrNode::BindingIdent {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident { span: IrSpan::empty(), value: "x".to_string() }),
            type_ann: Some(Box::new(IrNode::KeywordType { span: IrSpan::empty(), keyword: TsKeyword::String })),
            optional: false,
        };
        let output = codegen.generate_param(&node).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
        assert!(output_str.contains("type_ann"));
    }

    #[test]
    fn test_generate_params_empty() {
        let codegen = Codegen::new();
        let params: Vec<IrNode> = vec![];
        let output = codegen.generate_params(&params).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("vec"));
    }

    #[test]
    fn test_generate_params_multiple() {
        use crate::compiler::ir::IrSpan;
        let codegen = Codegen::new();
        let params = vec![
            IrNode::Ident { span: IrSpan::empty(), value: "a".to_string() },
            IrNode::Ident { span: IrSpan::empty(), value: "b".to_string() },
        ];
        let output = codegen.generate_params(&params).unwrap();
        let output_str = output.to_string();

        assert!(output_str.contains("Param"));
    }
}
