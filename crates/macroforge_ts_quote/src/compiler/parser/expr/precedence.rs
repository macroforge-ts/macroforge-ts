//! Operator precedence and binding power for the Pratt parser.
//!
//! This module defines the precedence levels for all TypeScript/JavaScript operators
//! following the ECMAScript specification. The Pratt parser uses binding powers to
//! determine operator associativity and precedence.

use crate::compiler::syntax::SyntaxKind;

/// Binding power for operators in Pratt parser style.
///
/// Using (left, right) pairs enables both left and right associativity:
/// - Left-associative: `left < right` (e.g., `a + b + c` = `(a + b) + c`)
/// - Right-associative: `left > right` (e.g., `a = b = c` = `a = (b = c)`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingPower {
    /// Left binding power - compared against previous operator's right bp.
    pub left: u8,
    /// Right binding power - compared against next operator's left bp.
    pub right: u8,
}

impl BindingPower {
    /// Creates a left-associative binding power.
    ///
    /// For left-associative operators like `+`, `*`, `==`:
    /// `a + b + c` parses as `(a + b) + c`
    #[inline]
    pub const fn left(power: u8) -> Self {
        Self {
            left: power,
            right: power + 1,
        }
    }

    /// Creates a right-associative binding power.
    ///
    /// For right-associative operators like `=`, `**`, `?:`:
    /// `a = b = c` parses as `a = (b = c)`
    #[inline]
    pub const fn right(power: u8) -> Self {
        Self {
            left: power + 1,
            right: power,
        }
    }

    /// Creates a non-associative binding power.
    ///
    /// For operators that cannot chain without parentheses.
    #[inline]
    pub const fn none(power: u8) -> Self {
        Self {
            left: power,
            right: power,
        }
    }
}

/// Precedence levels following ECMAScript specification.
///
/// Higher numbers mean tighter binding (higher precedence).
/// Gaps between levels allow for future additions.
pub mod prec {
    use super::BindingPower;

    // =========================================================================
    // Standard JavaScript/TypeScript Operators
    // =========================================================================

    /// Level 1: Comma operator (sequence expression) - lowest precedence
    /// `a, b, c` evaluates all expressions, returns last
    pub const COMMA: BindingPower = BindingPower::left(2);

    /// Level 2: Assignment operators (=, +=, -=, etc.) - right associative
    /// `a = b = c` assigns c to b, then result to a
    pub const ASSIGN: BindingPower = BindingPower::right(4);

    /// Level 3: Conditional/ternary operator (?:) - right associative
    /// `a ? b : c ? d : e` groups as `a ? b : (c ? d : e)`
    pub const CONDITIONAL: BindingPower = BindingPower::right(6);

    /// Level 4: Nullish coalescing operator (??)
    /// `a ?? b ?? c` groups left-to-right
    pub const NULLISH: BindingPower = BindingPower::left(8);

    /// Level 5: Logical OR (||)
    /// Short-circuit: if left is truthy, right is not evaluated
    pub const LOGICAL_OR: BindingPower = BindingPower::left(10);

    /// Level 6: Logical AND (&&)
    /// Short-circuit: if left is falsy, right is not evaluated
    pub const LOGICAL_AND: BindingPower = BindingPower::left(12);

    /// Level 7: Bitwise OR (|)
    pub const BITWISE_OR: BindingPower = BindingPower::left(14);

    /// Level 8: Bitwise XOR (^)
    pub const BITWISE_XOR: BindingPower = BindingPower::left(16);

    /// Level 9: Bitwise AND (&)
    pub const BITWISE_AND: BindingPower = BindingPower::left(18);

    /// Level 10: Equality operators (==, !=, ===, !==)
    pub const EQUALITY: BindingPower = BindingPower::left(20);

    /// Level 11: Relational operators (<, >, <=, >=, in, instanceof)
    pub const RELATIONAL: BindingPower = BindingPower::left(22);

    /// Level 12: Bitwise shift operators (<<, >>, >>>)
    pub const SHIFT: BindingPower = BindingPower::left(24);

    /// Level 13: Additive operators (+, -)
    pub const ADDITIVE: BindingPower = BindingPower::left(26);

    /// Level 14: Multiplicative operators (*, /, %)
    pub const MULTIPLICATIVE: BindingPower = BindingPower::left(28);

    /// Level 15: Exponentiation operator (**) - right associative
    /// `2 ** 3 ** 2` = `2 ** 9` = 512
    pub const EXPONENT: BindingPower = BindingPower::right(30);

    /// Level 16: Prefix unary operators (!, ~, +, -, typeof, void, delete, await)
    /// These bind to the operand immediately to their right.
    pub const PREFIX: u8 = 32;

    /// Level 17: Postfix operators (++, --)
    /// These bind to the operand immediately to their left.
    pub const POSTFIX: u8 = 34;

    /// Level 18: Call, member access, optional chain - highest infix precedence
    /// `a.b.c()` chains naturally from left to right
    pub const CALL: BindingPower = BindingPower::left(36);

    /// Level 19: new with arguments - highest precedence
    /// `new Foo()` binds tighter than call
    pub const NEW_WITH_ARGS: u8 = 38;

    // =========================================================================
    // TypeScript-specific Operators
    // =========================================================================

    /// TypeScript 'as' and 'satisfies' type assertions
    /// Placed between relational (22) and equality (20) at 21
    /// `a as T === b` parses as `(a as T) === b`
    pub const TS_AS: BindingPower = BindingPower::left(21);

    /// TypeScript non-null assertion (!) - postfix, high precedence
    /// `a!.b` parses as `(a!).b`
    pub const TS_NON_NULL: u8 = 35;

    /// TypeScript type instantiation `<T>` - treated like call
    /// `fn<T>()` binds together
    pub const TS_TYPE_INSTANTIATION: BindingPower = BindingPower::left(36);
}

/// Gets the infix binding power for a token.
///
/// Returns `None` if the token is not an infix operator at the given position,
/// or if it requires special handling (like `?` for ternary).
pub fn infix_binding_power(kind: SyntaxKind, text: &str) -> Option<BindingPower> {
    // First check by SyntaxKind for known tokens
    match kind {
        // Comma
        SyntaxKind::Comma => return Some(prec::COMMA),

        // Known keywords that are binary operators
        SyntaxKind::InKw => return Some(prec::RELATIONAL),
        SyntaxKind::AsKw => return Some(prec::TS_AS),
        SyntaxKind::SatisfiesKw => return Some(prec::TS_AS),

        // Comparison operators that have their own SyntaxKind
        SyntaxKind::Lt => return Some(prec::RELATIONAL),
        SyntaxKind::Gt => return Some(prec::RELATIONAL),

        // Equality operators
        SyntaxKind::EqEq => return Some(prec::EQUALITY),
        SyntaxKind::EqEqEq => return Some(prec::EQUALITY),
        SyntaxKind::NotEq => return Some(prec::EQUALITY),
        SyntaxKind::NotEqEq => return Some(prec::EQUALITY),

        // Known punctuation with specific meaning
        SyntaxKind::AmpersandAmpersand => return Some(prec::LOGICAL_AND),
        SyntaxKind::Ampersand => return Some(prec::BITWISE_AND),

        SyntaxKind::Star => {
            // Could be * or ** - need to check text
            if text == "**" {
                return Some(prec::EXPONENT);
            }
            return Some(prec::MULTIPLICATIVE);
        }

        // Question mark is special - ternary needs custom handling
        SyntaxKind::Question => return None,

        // Equals is special - could be = or == or ===
        SyntaxKind::Eq => {
            // Plain = is assignment, handled separately
            return None;
        }

        _ => {}
    }

    // Then check by text for multi-character operators
    match text {
        // Assignment operators (handled separately, but return None here)
        "=" => None,

        // Compound assignments (handled separately)
        "+=" | "-=" | "*=" | "/=" | "%=" | "**=" | "<<=" | ">>=" | ">>>=" | "&=" | "|="
        | "^=" | "&&=" | "||=" | "??=" => None,

        // Equality
        "==" | "===" => Some(prec::EQUALITY),
        "!=" | "!==" => Some(prec::EQUALITY),

        // Relational
        "<=" | ">=" => Some(prec::RELATIONAL),

        // Logical
        "&&" => Some(prec::LOGICAL_AND),
        "||" => Some(prec::LOGICAL_OR),
        "??" => Some(prec::NULLISH),

        // Bitwise
        "|" => Some(prec::BITWISE_OR),
        "^" => Some(prec::BITWISE_XOR),
        "&" => Some(prec::BITWISE_AND),

        // Shift
        "<<" => Some(prec::SHIFT),
        ">>" | ">>>" => Some(prec::SHIFT),

        // Arithmetic
        "+" | "-" => Some(prec::ADDITIVE),
        "*" | "/" | "%" => Some(prec::MULTIPLICATIVE),
        "**" => Some(prec::EXPONENT),

        // instanceof keyword (may come as text)
        "instanceof" => Some(prec::RELATIONAL),

        _ => None,
    }
}

/// Checks if a token is an assignment operator.
pub fn is_assignment_operator(text: &str) -> bool {
    matches!(
        text,
        "="
            | "+="
            | "-="
            | "*="
            | "/="
            | "%="
            | "**="
            | "<<="
            | ">>="
            | ">>>="
            | "&="
            | "|="
            | "^="
            | "&&="
            | "||="
            | "??="
    )
}

/// Checks if a token could start a prefix expression.
pub fn is_prefix_operator(kind: SyntaxKind, text: &str) -> bool {
    match kind {
        // Update operators
        SyntaxKind::PlusPlus | SyntaxKind::MinusMinus => true,

        // Unary keyword operators
        SyntaxKind::TypeofKw => true,
        SyntaxKind::VoidKw => true,
        SyntaxKind::DeleteKw => true,
        SyntaxKind::AwaitKw => true,
        SyntaxKind::YieldKw => true,
        SyntaxKind::NewKw => true,

        // Check text for punctuation-based operators
        _ => matches!(text, "-" | "+" | "!" | "~"),
    }
}

/// Checks if a token could be a postfix operator.
pub fn is_postfix_operator(kind: SyntaxKind) -> bool {
    matches!(kind, SyntaxKind::PlusPlus | SyntaxKind::MinusMinus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_power_left_associative() {
        let bp = BindingPower::left(10);
        assert_eq!(bp.left, 10);
        assert_eq!(bp.right, 11);
        // Left binding power < right, so same-precedence operators bind left-to-right
    }

    #[test]
    fn test_binding_power_right_associative() {
        let bp = BindingPower::right(10);
        assert_eq!(bp.left, 11);
        assert_eq!(bp.right, 10);
        // Left binding power > right, so same-precedence operators bind right-to-left
    }

    #[test]
    fn test_precedence_ordering() {
        // Lower precedence operators should have lower binding powers
        assert!(prec::COMMA.left < prec::ASSIGN.right);
        assert!(prec::ASSIGN.right < prec::CONDITIONAL.right);
        assert!(prec::LOGICAL_OR.left < prec::LOGICAL_AND.left);
        assert!(prec::LOGICAL_AND.left < prec::BITWISE_OR.left);
        assert!(prec::EQUALITY.left < prec::RELATIONAL.left);
        assert!(prec::ADDITIVE.left < prec::MULTIPLICATIVE.left);
        assert!(prec::MULTIPLICATIVE.left < prec::EXPONENT.left);
    }

    #[test]
    fn test_infix_binding_power_arithmetic() {
        assert_eq!(infix_binding_power(SyntaxKind::Text, "+"), Some(prec::ADDITIVE));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "-"), Some(prec::ADDITIVE));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "*"), Some(prec::MULTIPLICATIVE));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "/"), Some(prec::MULTIPLICATIVE));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "%"), Some(prec::MULTIPLICATIVE));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "**"), Some(prec::EXPONENT));
    }

    #[test]
    fn test_infix_binding_power_comparison() {
        assert_eq!(infix_binding_power(SyntaxKind::Text, "=="), Some(prec::EQUALITY));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "==="), Some(prec::EQUALITY));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "!="), Some(prec::EQUALITY));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "!=="), Some(prec::EQUALITY));
        assert_eq!(infix_binding_power(SyntaxKind::Lt, "<"), Some(prec::RELATIONAL));
        assert_eq!(infix_binding_power(SyntaxKind::Gt, ">"), Some(prec::RELATIONAL));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "<="), Some(prec::RELATIONAL));
        assert_eq!(infix_binding_power(SyntaxKind::Text, ">="), Some(prec::RELATIONAL));
    }

    #[test]
    fn test_infix_binding_power_logical() {
        assert_eq!(infix_binding_power(SyntaxKind::Text, "&&"), Some(prec::LOGICAL_AND));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "||"), Some(prec::LOGICAL_OR));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "??"), Some(prec::NULLISH));
    }

    #[test]
    fn test_infix_binding_power_bitwise() {
        assert_eq!(infix_binding_power(SyntaxKind::Text, "&"), Some(prec::BITWISE_AND));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "|"), Some(prec::BITWISE_OR));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "^"), Some(prec::BITWISE_XOR));
        assert_eq!(infix_binding_power(SyntaxKind::Text, "<<"), Some(prec::SHIFT));
        assert_eq!(infix_binding_power(SyntaxKind::Text, ">>"), Some(prec::SHIFT));
        assert_eq!(infix_binding_power(SyntaxKind::Text, ">>>"), Some(prec::SHIFT));
    }

    #[test]
    fn test_is_assignment_operator() {
        assert!(is_assignment_operator("="));
        assert!(is_assignment_operator("+="));
        assert!(is_assignment_operator("-="));
        assert!(is_assignment_operator("*="));
        assert!(is_assignment_operator("/="));
        assert!(is_assignment_operator("**="));
        assert!(is_assignment_operator("&&="));
        assert!(is_assignment_operator("||="));
        assert!(is_assignment_operator("??="));

        assert!(!is_assignment_operator("+"));
        assert!(!is_assignment_operator("=="));
        assert!(!is_assignment_operator("==="));
    }

    #[test]
    fn test_is_prefix_operator() {
        assert!(is_prefix_operator(SyntaxKind::PlusPlus, "++"));
        assert!(is_prefix_operator(SyntaxKind::MinusMinus, "--"));
        assert!(is_prefix_operator(SyntaxKind::TypeofKw, "typeof"));
        assert!(is_prefix_operator(SyntaxKind::Text, "-"));
        assert!(is_prefix_operator(SyntaxKind::Text, "+"));
        assert!(is_prefix_operator(SyntaxKind::Text, "!"));
        assert!(is_prefix_operator(SyntaxKind::Text, "~"));

        assert!(!is_prefix_operator(SyntaxKind::Text, "*"));
        assert!(!is_prefix_operator(SyntaxKind::Text, "/"));
    }

    #[test]
    fn test_typescript_as_precedence() {
        // 'as' should bind tighter than equality but looser than relational
        let as_bp = prec::TS_AS;
        assert!(as_bp.left > prec::EQUALITY.left);
        assert!(as_bp.left < prec::RELATIONAL.left);
    }
}
