//! Token to IR operator mappings.
//!
//! This module provides functions to convert token text and syntax kinds
//! to the corresponding IR operator types.

use crate::compiler::ir::{AssignOp, BinaryOp, UnaryOp, UpdateOp};
use crate::compiler::syntax::SyntaxKind;

/// Maps token text to `BinaryOp`.
///
/// Returns `None` if the text doesn't represent a binary operator.
pub fn text_to_binary_op(text: &str) -> Option<BinaryOp> {
    match text {
        // Arithmetic
        "+" => Some(BinaryOp::Add),
        "-" => Some(BinaryOp::Sub),
        "*" => Some(BinaryOp::Mul),
        "/" => Some(BinaryOp::Div),
        "%" => Some(BinaryOp::Mod),
        "**" => Some(BinaryOp::Exp),

        // Comparison / Equality
        "==" => Some(BinaryOp::EqEq),
        "!=" => Some(BinaryOp::NotEq),
        "===" => Some(BinaryOp::EqEqEq),
        "!==" => Some(BinaryOp::NotEqEq),
        "<" => Some(BinaryOp::Lt),
        "<=" => Some(BinaryOp::Le),
        ">" => Some(BinaryOp::Gt),
        ">=" => Some(BinaryOp::Ge),

        // Logical
        "&&" => Some(BinaryOp::And),
        "||" => Some(BinaryOp::Or),
        "??" => Some(BinaryOp::NullishCoalesce),

        // Bitwise
        "&" => Some(BinaryOp::BitAnd),
        "|" => Some(BinaryOp::BitOr),
        "^" => Some(BinaryOp::BitXor),
        "<<" => Some(BinaryOp::Shl),
        ">>" => Some(BinaryOp::Shr),
        ">>>" => Some(BinaryOp::UShr),

        // Keywords as operators
        "in" => Some(BinaryOp::In),
        "instanceof" => Some(BinaryOp::InstanceOf),

        _ => None,
    }
}

/// Maps a `SyntaxKind` to `BinaryOp`.
///
/// This handles keyword-based binary operators like `in` and `instanceof`,
/// as well as punctuation-based operators that have their own SyntaxKind.
pub fn keyword_to_binary_op(kind: SyntaxKind) -> Option<BinaryOp> {
    match kind {
        SyntaxKind::InKw => Some(BinaryOp::In),
        SyntaxKind::InstanceofKw => Some(BinaryOp::InstanceOf),
        SyntaxKind::EqEq => Some(BinaryOp::EqEq),
        SyntaxKind::EqEqEq => Some(BinaryOp::EqEqEq),
        SyntaxKind::NotEq => Some(BinaryOp::NotEq),
        SyntaxKind::NotEqEq => Some(BinaryOp::NotEqEq),
        _ => None,
    }
}

/// Maps token text to `AssignOp`.
///
/// Returns `None` if the text doesn't represent an assignment operator.
pub fn text_to_assign_op(text: &str) -> Option<AssignOp> {
    match text {
        "=" => Some(AssignOp::Assign),
        "+=" => Some(AssignOp::AddAssign),
        "-=" => Some(AssignOp::SubAssign),
        "*=" => Some(AssignOp::MulAssign),
        "/=" => Some(AssignOp::DivAssign),
        "%=" => Some(AssignOp::ModAssign),
        "**=" => Some(AssignOp::ExpAssign),
        "<<=" => Some(AssignOp::ShlAssign),
        ">>=" => Some(AssignOp::ShrAssign),
        ">>>=" => Some(AssignOp::UShrAssign),
        "&=" => Some(AssignOp::BitAndAssign),
        "|=" => Some(AssignOp::BitOrAssign),
        "^=" => Some(AssignOp::BitXorAssign),
        "&&=" => Some(AssignOp::AndAssign),
        "||=" => Some(AssignOp::OrAssign),
        "??=" => Some(AssignOp::NullishAssign),
        _ => None,
    }
}

/// Maps a `SyntaxKind` and text to `UnaryOp`.
///
/// This handles both keyword-based and punctuation-based unary operators.
pub fn to_unary_op(kind: SyntaxKind, text: &str) -> Option<UnaryOp> {
    match kind {
        // Keyword operators
        SyntaxKind::TypeofKw => Some(UnaryOp::TypeOf),
        SyntaxKind::VoidKw => Some(UnaryOp::Void),
        SyntaxKind::DeleteKw => Some(UnaryOp::Delete),

        // Punctuation operators - check by text
        _ => match text {
            "-" => Some(UnaryOp::Minus),
            "+" => Some(UnaryOp::Plus),
            "!" => Some(UnaryOp::Not),
            "~" => Some(UnaryOp::BitNot),
            // Fallback for keywords that might come as text
            "typeof" => Some(UnaryOp::TypeOf),
            "void" => Some(UnaryOp::Void),
            "delete" => Some(UnaryOp::Delete),
            _ => None,
        },
    }
}

/// Maps a `SyntaxKind` to `UpdateOp`.
///
/// Returns `None` if the kind is not an update operator.
pub fn to_update_op(kind: SyntaxKind) -> Option<UpdateOp> {
    match kind {
        SyntaxKind::PlusPlus => Some(UpdateOp::Increment),
        SyntaxKind::MinusMinus => Some(UpdateOp::Decrement),
        _ => None,
    }
}

/// Maps text to `UpdateOp`.
///
/// Returns `None` if the text is not an update operator.
pub fn text_to_update_op(text: &str) -> Option<UpdateOp> {
    match text {
        "++" => Some(UpdateOp::Increment),
        "--" => Some(UpdateOp::Decrement),
        _ => None,
    }
}

/// Checks if a token text represents a binary operator.
pub fn is_binary_operator(text: &str) -> bool {
    text_to_binary_op(text).is_some()
}

/// Checks if a token text represents an assignment operator.
pub fn is_assignment_operator(text: &str) -> bool {
    text_to_assign_op(text).is_some()
}

/// Checks if a SyntaxKind/text combination represents a unary operator.
pub fn is_unary_operator(kind: SyntaxKind, text: &str) -> bool {
    to_unary_op(kind, text).is_some()
}

/// Checks if a SyntaxKind represents an update operator.
pub fn is_update_operator(kind: SyntaxKind) -> bool {
    to_update_op(kind).is_some()
}

/// Gets the string representation of a `BinaryOp`.
///
/// Useful for error messages and debugging.
pub fn binary_op_to_str(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::Exp => "**",
        BinaryOp::EqEq => "==",
        BinaryOp::NotEq => "!=",
        BinaryOp::EqEqEq => "===",
        BinaryOp::NotEqEq => "!==",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::NullishCoalesce => "??",
        BinaryOp::BitAnd => "&",
        BinaryOp::BitOr => "|",
        BinaryOp::BitXor => "^",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
        BinaryOp::UShr => ">>>",
        BinaryOp::In => "in",
        BinaryOp::InstanceOf => "instanceof",
    }
}

/// Gets the string representation of an `AssignOp`.
pub fn assign_op_to_str(op: AssignOp) -> &'static str {
    match op {
        AssignOp::Assign => "=",
        AssignOp::AddAssign => "+=",
        AssignOp::SubAssign => "-=",
        AssignOp::MulAssign => "*=",
        AssignOp::DivAssign => "/=",
        AssignOp::ModAssign => "%=",
        AssignOp::ExpAssign => "**=",
        AssignOp::ShlAssign => "<<=",
        AssignOp::ShrAssign => ">>=",
        AssignOp::UShrAssign => ">>>=",
        AssignOp::BitAndAssign => "&=",
        AssignOp::BitOrAssign => "|=",
        AssignOp::BitXorAssign => "^=",
        AssignOp::AndAssign => "&&=",
        AssignOp::OrAssign => "||=",
        AssignOp::NullishAssign => "??=",
    }
}

/// Gets the string representation of a `UnaryOp`.
pub fn unary_op_to_str(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Minus => "-",
        UnaryOp::Plus => "+",
        UnaryOp::Not => "!",
        UnaryOp::BitNot => "~",
        UnaryOp::TypeOf => "typeof",
        UnaryOp::Void => "void",
        UnaryOp::Delete => "delete",
    }
}

/// Gets the string representation of an `UpdateOp`.
pub fn update_op_to_str(op: UpdateOp) -> &'static str {
    match op {
        UpdateOp::Increment => "++",
        UpdateOp::Decrement => "--",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Binary Operator Tests
    // =========================================================================

    #[test]
    fn test_text_to_binary_op_arithmetic() {
        assert_eq!(text_to_binary_op("+"), Some(BinaryOp::Add));
        assert_eq!(text_to_binary_op("-"), Some(BinaryOp::Sub));
        assert_eq!(text_to_binary_op("*"), Some(BinaryOp::Mul));
        assert_eq!(text_to_binary_op("/"), Some(BinaryOp::Div));
        assert_eq!(text_to_binary_op("%"), Some(BinaryOp::Mod));
        assert_eq!(text_to_binary_op("**"), Some(BinaryOp::Exp));
    }

    #[test]
    fn test_text_to_binary_op_comparison() {
        assert_eq!(text_to_binary_op("=="), Some(BinaryOp::EqEq));
        assert_eq!(text_to_binary_op("!="), Some(BinaryOp::NotEq));
        assert_eq!(text_to_binary_op("==="), Some(BinaryOp::EqEqEq));
        assert_eq!(text_to_binary_op("!=="), Some(BinaryOp::NotEqEq));
        assert_eq!(text_to_binary_op("<"), Some(BinaryOp::Lt));
        assert_eq!(text_to_binary_op("<="), Some(BinaryOp::Le));
        assert_eq!(text_to_binary_op(">"), Some(BinaryOp::Gt));
        assert_eq!(text_to_binary_op(">="), Some(BinaryOp::Ge));
    }

    #[test]
    fn test_text_to_binary_op_logical() {
        assert_eq!(text_to_binary_op("&&"), Some(BinaryOp::And));
        assert_eq!(text_to_binary_op("||"), Some(BinaryOp::Or));
        assert_eq!(text_to_binary_op("??"), Some(BinaryOp::NullishCoalesce));
    }

    #[test]
    fn test_text_to_binary_op_bitwise() {
        assert_eq!(text_to_binary_op("&"), Some(BinaryOp::BitAnd));
        assert_eq!(text_to_binary_op("|"), Some(BinaryOp::BitOr));
        assert_eq!(text_to_binary_op("^"), Some(BinaryOp::BitXor));
        assert_eq!(text_to_binary_op("<<"), Some(BinaryOp::Shl));
        assert_eq!(text_to_binary_op(">>"), Some(BinaryOp::Shr));
        assert_eq!(text_to_binary_op(">>>"), Some(BinaryOp::UShr));
    }

    #[test]
    fn test_text_to_binary_op_keywords() {
        assert_eq!(text_to_binary_op("in"), Some(BinaryOp::In));
        assert_eq!(text_to_binary_op("instanceof"), Some(BinaryOp::InstanceOf));
    }

    #[test]
    fn test_text_to_binary_op_invalid() {
        assert_eq!(text_to_binary_op("="), None);
        assert_eq!(text_to_binary_op("+="), None);
        assert_eq!(text_to_binary_op("foo"), None);
        assert_eq!(text_to_binary_op(""), None);
    }

    #[test]
    fn test_keyword_to_binary_op() {
        assert_eq!(keyword_to_binary_op(SyntaxKind::InKw), Some(BinaryOp::In));
        assert_eq!(
            keyword_to_binary_op(SyntaxKind::InstanceofKw),
            Some(BinaryOp::InstanceOf)
        );
        assert_eq!(keyword_to_binary_op(SyntaxKind::EqEq), Some(BinaryOp::EqEq));
        assert_eq!(keyword_to_binary_op(SyntaxKind::EqEqEq), Some(BinaryOp::EqEqEq));
        assert_eq!(keyword_to_binary_op(SyntaxKind::NotEq), Some(BinaryOp::NotEq));
        assert_eq!(keyword_to_binary_op(SyntaxKind::NotEqEq), Some(BinaryOp::NotEqEq));
        assert_eq!(keyword_to_binary_op(SyntaxKind::Ident), None);
    }

    // =========================================================================
    // Assignment Operator Tests
    // =========================================================================

    #[test]
    fn test_text_to_assign_op_basic() {
        assert_eq!(text_to_assign_op("="), Some(AssignOp::Assign));
        assert_eq!(text_to_assign_op("+="), Some(AssignOp::AddAssign));
        assert_eq!(text_to_assign_op("-="), Some(AssignOp::SubAssign));
        assert_eq!(text_to_assign_op("*="), Some(AssignOp::MulAssign));
        assert_eq!(text_to_assign_op("/="), Some(AssignOp::DivAssign));
        assert_eq!(text_to_assign_op("%="), Some(AssignOp::ModAssign));
        assert_eq!(text_to_assign_op("**="), Some(AssignOp::ExpAssign));
    }

    #[test]
    fn test_text_to_assign_op_bitwise() {
        assert_eq!(text_to_assign_op("<<="), Some(AssignOp::ShlAssign));
        assert_eq!(text_to_assign_op(">>="), Some(AssignOp::ShrAssign));
        assert_eq!(text_to_assign_op(">>>="), Some(AssignOp::UShrAssign));
        assert_eq!(text_to_assign_op("&="), Some(AssignOp::BitAndAssign));
        assert_eq!(text_to_assign_op("|="), Some(AssignOp::BitOrAssign));
        assert_eq!(text_to_assign_op("^="), Some(AssignOp::BitXorAssign));
    }

    #[test]
    fn test_text_to_assign_op_logical() {
        assert_eq!(text_to_assign_op("&&="), Some(AssignOp::AndAssign));
        assert_eq!(text_to_assign_op("||="), Some(AssignOp::OrAssign));
        assert_eq!(text_to_assign_op("??="), Some(AssignOp::NullishAssign));
    }

    #[test]
    fn test_text_to_assign_op_invalid() {
        assert_eq!(text_to_assign_op("+"), None);
        assert_eq!(text_to_assign_op("=="), None);
        assert_eq!(text_to_assign_op("foo"), None);
    }

    // =========================================================================
    // Unary Operator Tests
    // =========================================================================

    #[test]
    fn test_to_unary_op_punctuation() {
        assert_eq!(to_unary_op(SyntaxKind::Text, "-"), Some(UnaryOp::Minus));
        assert_eq!(to_unary_op(SyntaxKind::Text, "+"), Some(UnaryOp::Plus));
        assert_eq!(to_unary_op(SyntaxKind::Text, "!"), Some(UnaryOp::Not));
        assert_eq!(to_unary_op(SyntaxKind::Text, "~"), Some(UnaryOp::BitNot));
    }

    #[test]
    fn test_to_unary_op_keywords() {
        assert_eq!(
            to_unary_op(SyntaxKind::TypeofKw, "typeof"),
            Some(UnaryOp::TypeOf)
        );
        assert_eq!(to_unary_op(SyntaxKind::VoidKw, "void"), Some(UnaryOp::Void));
        assert_eq!(
            to_unary_op(SyntaxKind::DeleteKw, "delete"),
            Some(UnaryOp::Delete)
        );
    }

    #[test]
    fn test_to_unary_op_text_keywords() {
        // Keywords might come as plain text in some contexts
        assert_eq!(to_unary_op(SyntaxKind::Text, "typeof"), Some(UnaryOp::TypeOf));
        assert_eq!(to_unary_op(SyntaxKind::Text, "void"), Some(UnaryOp::Void));
        assert_eq!(to_unary_op(SyntaxKind::Text, "delete"), Some(UnaryOp::Delete));
    }

    #[test]
    fn test_to_unary_op_invalid() {
        assert_eq!(to_unary_op(SyntaxKind::Text, "*"), None);
        assert_eq!(to_unary_op(SyntaxKind::Text, "/"), None);
        assert_eq!(to_unary_op(SyntaxKind::Text, "foo"), None);
    }

    // =========================================================================
    // Update Operator Tests
    // =========================================================================

    #[test]
    fn test_to_update_op() {
        assert_eq!(to_update_op(SyntaxKind::PlusPlus), Some(UpdateOp::Increment));
        assert_eq!(
            to_update_op(SyntaxKind::MinusMinus),
            Some(UpdateOp::Decrement)
        );
        assert_eq!(to_update_op(SyntaxKind::Text), None);
    }

    #[test]
    fn test_text_to_update_op() {
        assert_eq!(text_to_update_op("++"), Some(UpdateOp::Increment));
        assert_eq!(text_to_update_op("--"), Some(UpdateOp::Decrement));
        assert_eq!(text_to_update_op("+"), None);
        assert_eq!(text_to_update_op("-"), None);
    }

    // =========================================================================
    // Operator String Representation Tests
    // =========================================================================

    #[test]
    fn test_binary_op_roundtrip() {
        let ops = [
            BinaryOp::Add,
            BinaryOp::Sub,
            BinaryOp::Mul,
            BinaryOp::Div,
            BinaryOp::Mod,
            BinaryOp::Exp,
            BinaryOp::EqEq,
            BinaryOp::NotEq,
            BinaryOp::EqEqEq,
            BinaryOp::NotEqEq,
            BinaryOp::Lt,
            BinaryOp::Le,
            BinaryOp::Gt,
            BinaryOp::Ge,
            BinaryOp::And,
            BinaryOp::Or,
            BinaryOp::NullishCoalesce,
            BinaryOp::BitAnd,
            BinaryOp::BitOr,
            BinaryOp::BitXor,
            BinaryOp::Shl,
            BinaryOp::Shr,
            BinaryOp::UShr,
            BinaryOp::In,
            BinaryOp::InstanceOf,
        ];

        for op in ops {
            let s = binary_op_to_str(op);
            let roundtrip = text_to_binary_op(s);
            assert_eq!(
                roundtrip,
                Some(op),
                "Roundtrip failed for {:?} -> {} -> {:?}",
                op,
                s,
                roundtrip
            );
        }
    }

    #[test]
    fn test_assign_op_roundtrip() {
        let ops = [
            AssignOp::Assign,
            AssignOp::AddAssign,
            AssignOp::SubAssign,
            AssignOp::MulAssign,
            AssignOp::DivAssign,
            AssignOp::ModAssign,
            AssignOp::ExpAssign,
            AssignOp::ShlAssign,
            AssignOp::ShrAssign,
            AssignOp::UShrAssign,
            AssignOp::BitAndAssign,
            AssignOp::BitOrAssign,
            AssignOp::BitXorAssign,
            AssignOp::AndAssign,
            AssignOp::OrAssign,
            AssignOp::NullishAssign,
        ];

        for op in ops {
            let s = assign_op_to_str(op);
            let roundtrip = text_to_assign_op(s);
            assert_eq!(
                roundtrip,
                Some(op),
                "Roundtrip failed for {:?} -> {} -> {:?}",
                op,
                s,
                roundtrip
            );
        }
    }

    #[test]
    fn test_update_op_roundtrip() {
        let ops = [UpdateOp::Increment, UpdateOp::Decrement];

        for op in ops {
            let s = update_op_to_str(op);
            let roundtrip = text_to_update_op(s);
            assert_eq!(
                roundtrip,
                Some(op),
                "Roundtrip failed for {:?} -> {} -> {:?}",
                op,
                s,
                roundtrip
            );
        }
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_is_binary_operator() {
        assert!(is_binary_operator("+"));
        assert!(is_binary_operator("-"));
        assert!(is_binary_operator("&&"));
        assert!(is_binary_operator("||"));
        assert!(!is_binary_operator("="));
        assert!(!is_binary_operator("+="));
    }

    #[test]
    fn test_is_assignment_operator() {
        assert!(is_assignment_operator("="));
        assert!(is_assignment_operator("+="));
        assert!(is_assignment_operator("??="));
        assert!(!is_assignment_operator("+"));
        assert!(!is_assignment_operator("=="));
    }

    #[test]
    fn test_is_unary_operator() {
        assert!(is_unary_operator(SyntaxKind::Text, "-"));
        assert!(is_unary_operator(SyntaxKind::Text, "!"));
        assert!(is_unary_operator(SyntaxKind::TypeofKw, "typeof"));
        assert!(!is_unary_operator(SyntaxKind::Text, "*"));
    }

    #[test]
    fn test_is_update_operator() {
        assert!(is_update_operator(SyntaxKind::PlusPlus));
        assert!(is_update_operator(SyntaxKind::MinusMinus));
        assert!(!is_update_operator(SyntaxKind::Text));
    }
}
