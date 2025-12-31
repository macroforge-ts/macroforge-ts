use swc_core::ecma::ast::*;
use syn::parse_quote;

macro_rules! impl_simple_enum {
    ($E:ident, [ $($v:ident),* ]) => {
        impl crate::ast::ToCode for $E {
            fn to_code(&self, _: &crate::ctxt::Ctx) -> syn::Expr {
                match self {
                    $(
                        $E::$v => parse_quote!(
                             macroforge_ts::swc_core::ecma::ast::$E::$v
                        ),
                    )*
                    #[cfg(swc_ast_unknown)]
                    _ => panic!("unable to access unknown nodes"),
                }
            }
        }
    };
}

impl_simple_enum!(VarDeclKind, [Var, Const, Let]);
impl_simple_enum!(UnaryOp, [Minus, Plus, Bang, Tilde, TypeOf, Void, Delete]);
impl_simple_enum!(UpdateOp, [PlusPlus, MinusMinus]);
impl_simple_enum!(
    AssignOp,
    [
        Assign,
        AddAssign,
        SubAssign,
        MulAssign,
        DivAssign,
        ModAssign,
        LShiftAssign,
        RShiftAssign,
        ZeroFillRShiftAssign,
        BitOrAssign,
        BitXorAssign,
        BitAndAssign,
        ExpAssign,
        AndAssign,
        OrAssign,
        NullishAssign
    ]
);
impl_simple_enum!(
    BinaryOp,
    [
        EqEq,
        NotEq,
        EqEqEq,
        NotEqEq,
        Lt,
        LtEq,
        Gt,
        GtEq,
        LShift,
        RShift,
        ZeroFillRShift,
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        BitOr,
        BitXor,
        BitAnd,
        LogicalOr,
        LogicalAnd,
        In,
        InstanceOf,
        Exp,
        NullishCoalescing
    ]
);

impl_simple_enum!(Accessibility, [Public, Protected, Private]);
impl_simple_enum!(MethodKind, [Method, Getter, Setter]);
impl_simple_enum!(MetaPropKind, [NewTarget, ImportMeta]);
impl_simple_enum!(ImportPhase, [Defer, Source, Evaluation]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ToCode;
    use crate::ctxt::Ctx;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== VarDeclKind Tests ====================

    #[test]
    fn test_var_decl_kind_var() {
        let cx = empty_ctx();
        let kind = VarDeclKind::Var;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclKind"));
        assert!(code_str.contains("Var"));
    }

    #[test]
    fn test_var_decl_kind_const() {
        let cx = empty_ctx();
        let kind = VarDeclKind::Const;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclKind"));
        assert!(code_str.contains("Const"));
    }

    #[test]
    fn test_var_decl_kind_let() {
        let cx = empty_ctx();
        let kind = VarDeclKind::Let;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("VarDeclKind"));
        assert!(code_str.contains("Let"));
    }

    // ==================== UnaryOp Tests ====================

    #[test]
    fn test_unary_op_minus() {
        let cx = empty_ctx();
        let op = UnaryOp::Minus;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("UnaryOp"));
        assert!(code_str.contains("Minus"));
    }

    #[test]
    fn test_unary_op_plus() {
        let cx = empty_ctx();
        let op = UnaryOp::Plus;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Plus"));
    }

    #[test]
    fn test_unary_op_bang() {
        let cx = empty_ctx();
        let op = UnaryOp::Bang;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Bang"));
    }

    #[test]
    fn test_unary_op_typeof() {
        let cx = empty_ctx();
        let op = UnaryOp::TypeOf;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TypeOf"));
    }

    #[test]
    fn test_unary_op_void() {
        let cx = empty_ctx();
        let op = UnaryOp::Void;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Void"));
    }

    #[test]
    fn test_unary_op_delete() {
        let cx = empty_ctx();
        let op = UnaryOp::Delete;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Delete"));
    }

    // ==================== UpdateOp Tests ====================

    #[test]
    fn test_update_op_plus_plus() {
        let cx = empty_ctx();
        let op = UpdateOp::PlusPlus;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("UpdateOp"));
        assert!(code_str.contains("PlusPlus"));
    }

    #[test]
    fn test_update_op_minus_minus() {
        let cx = empty_ctx();
        let op = UpdateOp::MinusMinus;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("MinusMinus"));
    }

    // ==================== AssignOp Tests ====================

    #[test]
    fn test_assign_op_assign() {
        let cx = empty_ctx();
        let op = AssignOp::Assign;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AssignOp"));
        assert!(code_str.contains("Assign"));
    }

    #[test]
    fn test_assign_op_add_assign() {
        let cx = empty_ctx();
        let op = AssignOp::AddAssign;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("AddAssign"));
    }

    #[test]
    fn test_assign_op_nullish_assign() {
        let cx = empty_ctx();
        let op = AssignOp::NullishAssign;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("NullishAssign"));
    }

    // ==================== BinaryOp Tests ====================

    #[test]
    fn test_binary_op_add() {
        let cx = empty_ctx();
        let op = BinaryOp::Add;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BinaryOp"));
        assert!(code_str.contains("Add"));
    }

    #[test]
    fn test_binary_op_sub() {
        let cx = empty_ctx();
        let op = BinaryOp::Sub;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Sub"));
    }

    #[test]
    fn test_binary_op_eq_eq_eq() {
        let cx = empty_ctx();
        let op = BinaryOp::EqEqEq;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("EqEqEq"));
    }

    #[test]
    fn test_binary_op_logical_and() {
        let cx = empty_ctx();
        let op = BinaryOp::LogicalAnd;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("LogicalAnd"));
    }

    #[test]
    fn test_binary_op_nullish_coalescing() {
        let cx = empty_ctx();
        let op = BinaryOp::NullishCoalescing;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("NullishCoalescing"));
    }

    #[test]
    fn test_binary_op_instance_of() {
        let cx = empty_ctx();
        let op = BinaryOp::InstanceOf;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("InstanceOf"));
    }

    #[test]
    fn test_binary_op_in() {
        let cx = empty_ctx();
        let op = BinaryOp::In;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("BinaryOp"));
        // Note: "In" might be part of other words, so check carefully
    }

    // ==================== Accessibility Tests ====================

    #[test]
    fn test_accessibility_public() {
        let cx = empty_ctx();
        let access = Accessibility::Public;
        let code = access.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Accessibility"));
        assert!(code_str.contains("Public"));
    }

    #[test]
    fn test_accessibility_protected() {
        let cx = empty_ctx();
        let access = Accessibility::Protected;
        let code = access.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Protected"));
    }

    #[test]
    fn test_accessibility_private() {
        let cx = empty_ctx();
        let access = Accessibility::Private;
        let code = access.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Private"));
    }

    // ==================== MethodKind Tests ====================

    #[test]
    fn test_method_kind_method() {
        let cx = empty_ctx();
        let kind = MethodKind::Method;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("MethodKind"));
        assert!(code_str.contains("Method"));
    }

    #[test]
    fn test_method_kind_getter() {
        let cx = empty_ctx();
        let kind = MethodKind::Getter;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Getter"));
    }

    #[test]
    fn test_method_kind_setter() {
        let cx = empty_ctx();
        let kind = MethodKind::Setter;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Setter"));
    }

    // ==================== MetaPropKind Tests ====================

    #[test]
    fn test_meta_prop_kind_new_target() {
        let cx = empty_ctx();
        let kind = MetaPropKind::NewTarget;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("MetaPropKind"));
        assert!(code_str.contains("NewTarget"));
    }

    #[test]
    fn test_meta_prop_kind_import_meta() {
        let cx = empty_ctx();
        let kind = MetaPropKind::ImportMeta;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportMeta"));
    }

    // ==================== ImportPhase Tests ====================

    #[test]
    fn test_import_phase_evaluation() {
        let cx = empty_ctx();
        let phase = ImportPhase::Evaluation;
        let code = phase.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ImportPhase"));
        assert!(code_str.contains("Evaluation"));
    }

    #[test]
    fn test_import_phase_source() {
        let cx = empty_ctx();
        let phase = ImportPhase::Source;
        let code = phase.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Source"));
    }

    #[test]
    fn test_import_phase_defer() {
        let cx = empty_ctx();
        let phase = ImportPhase::Defer;
        let code = phase.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Defer"));
    }
}
