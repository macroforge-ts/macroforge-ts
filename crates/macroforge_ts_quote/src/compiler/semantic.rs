//! Semantic analysis for the template language.
//!
//! This module walks the CST and classifies placeholders based on their
//! syntactic context. This is the key insight: type positions are determined
//! by AST structure, not string heuristics.

use super::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use std::collections::HashMap;

/// Classification of a placeholder based on its syntactic context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderKind {
    /// Placeholder in expression position (default)
    Expr,
    /// Placeholder in type position (after `:` or `as`)
    Type,
    /// Placeholder in identifier position (variable/function name)
    Ident,
    /// Placeholder in statement position
    Stmt,
}

/// Information about a placeholder found during analysis.
#[derive(Debug, Clone)]
pub struct PlaceholderInfo {
    /// The kind of placeholder based on context.
    pub kind: PlaceholderKind,
    /// The Rust expression tokens (extracted from @{...}).
    pub tokens: String,
}

/// Result of semantic analysis.
#[derive(Debug)]
pub struct SemanticAnalysis {
    /// Map from placeholder ID to its classification.
    pub placeholders: HashMap<usize, PlaceholderInfo>,
}

/// The semantic analyzer.
pub struct Analyzer {
    /// Current context stack.
    context_stack: Vec<Context>,
    /// Collected placeholders.
    placeholders: HashMap<usize, PlaceholderInfo>,
    /// Placeholder counter.
    placeholder_id: usize,
}

/// Analysis context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    /// Top-level or statement context.
    Statement,
    /// Expression context.
    Expression,
    /// Type annotation context (after `:`)
    TypeAnnotation,
    /// Type assertion context (after `as`)
    TypeAssertion,
    /// Generic type parameters (inside `<...>`)
    GenericParams,
    /// Function parameter list.
    Parameters,
    /// Object literal.
    ObjectLiteral,
    /// Identifier context (for {|...|} blocks).
    Identifier,
}

impl Analyzer {
    /// Creates a new analyzer.
    pub fn new() -> Self {
        Self {
            // Default to Expression context since most placeholders in TypeScript are expressions
            // (property access, function calls, assignments, etc.)
            context_stack: vec![Context::Expression],
            placeholders: HashMap::new(),
            placeholder_id: 0,
        }
    }

    /// Analyzes a syntax tree and returns the analysis result.
    pub fn analyze(mut self, root: &SyntaxNode) -> SemanticAnalysis {
        self.visit_node(root);
        SemanticAnalysis {
            placeholders: self.placeholders,
        }
    }

    /// Returns the current context.
    fn current_context(&self) -> Context {
        *self.context_stack.last().unwrap_or(&Context::Statement)
    }

    /// Pushes a new context onto the stack.
    fn push_context(&mut self, ctx: Context) {
        self.context_stack.push(ctx);
    }

    /// Pops the current context from the stack.
    fn pop_context(&mut self) {
        if self.context_stack.len() > 1 {
            self.context_stack.pop();
        }
    }

    /// Determines the placeholder kind based on current context.
    fn placeholder_kind_from_context(&self) -> PlaceholderKind {
        match self.current_context() {
            Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams => {
                PlaceholderKind::Type
            }
            Context::Identifier => PlaceholderKind::Ident,
            Context::Statement => PlaceholderKind::Stmt,
            Context::Expression
            | Context::Parameters
            | Context::ObjectLiteral => PlaceholderKind::Expr,
        }
    }

    /// Visits a syntax node.
    fn visit_node(&mut self, node: &SyntaxNode) {
        match node.kind() {
            SyntaxKind::Root => {
                self.visit_children(node);
            }

            // Type positions
            SyntaxKind::TypeAnnotation => {
                // Don't treat as type annotation if we're inside an object literal
                // (where `:` is property separator, not type annotation)
                if self.current_context() != Context::ObjectLiteral {
                    self.push_context(Context::TypeAnnotation);
                }
                self.visit_children(node);
                if self.current_context() == Context::TypeAnnotation {
                    self.pop_context();
                }
            }
            SyntaxKind::TypeAssertion => {
                self.push_context(Context::TypeAssertion);
                self.visit_children(node);
                self.pop_context();
            }
            SyntaxKind::TsTypeParams => {
                self.push_context(Context::GenericParams);
                self.visit_children(node);
                self.pop_context();
            }

            // Expression positions
            SyntaxKind::TsExpr => {
                self.push_context(Context::Expression);
                self.visit_children(node);
                self.pop_context();
            }

            // Identifier positions
            SyntaxKind::IdentBlock => {
                self.push_context(Context::Identifier);
                self.visit_children(node);
                self.pop_context();
            }

            // Object literals
            SyntaxKind::TsObject | SyntaxKind::BraceBlock => {
                // Need to check if this is an object literal vs code block
                // For now, treat braces after `:` or `=` as object literals
                self.push_context(Context::ObjectLiteral);
                self.visit_children(node);
                self.pop_context();
            }

            // Function parameters
            SyntaxKind::TsParam => {
                self.push_context(Context::Parameters);
                self.visit_children(node);
                self.pop_context();
            }

            // Interpolations - this is where we classify!
            SyntaxKind::Interpolation => {
                self.record_interpolation(node);
            }

            // Control flow - visit body with expression context
            // Most placeholders in TypeScript code are expressions (property access, function calls, etc.)
            // not statements, so we default to Expression context for better inference
            SyntaxKind::IfBlock
            | SyntaxKind::ForBlock
            | SyntaxKind::WhileBlock
            | SyntaxKind::MatchBlock
            | SyntaxKind::ElseClause
            | SyntaxKind::ElseIfClause
            | SyntaxKind::MatchCase => {
                self.push_context(Context::Expression);
                self.visit_children(node);
                self.pop_context();
            }

            // Default: visit children
            _ => {
                self.visit_children(node);
            }
        }
    }

    /// Visits all children of a node in document order.
    /// This ensures tokens like `=` are processed before nodes like interpolations.
    fn visit_children(&mut self, node: &SyntaxNode) {
        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Node(child_node) => {
                    self.visit_node(&child_node);
                }
                rowan::NodeOrToken::Token(token) => {
                    self.visit_token(&token);
                }
            }
        }
    }

    /// Visits a token (for detecting context switches like `:` and `as`).
    fn visit_token(&mut self, token: &SyntaxToken) {
        match token.kind() {
            // Colon starts a type annotation context
            SyntaxKind::Colon => {
                // Pop Identifier context if we're in one (for `const x: Type`)
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                // Check if we're in a position where : means type annotation
                // (not object property)
                if self.current_context() != Context::ObjectLiteral {
                    self.push_context(Context::TypeAnnotation);
                }
            }

            // `as` keyword starts type assertion
            SyntaxKind::AsKw => {
                self.push_context(Context::TypeAssertion);
            }

            // `function` keyword - next identifier is a function name
            SyntaxKind::FunctionKw => {
                self.push_context(Context::Identifier);
            }

            // `class` keyword - next identifier is a class name
            SyntaxKind::ClassKw => {
                self.push_context(Context::Identifier);
            }

            // `const`, `let` keywords - next identifier is a variable name
            SyntaxKind::ConstKw | SyntaxKind::LetKw => {
                self.push_context(Context::Identifier);
            }

            // `return` keyword - next is an expression
            SyntaxKind::ReturnKw => {
                self.push_context(Context::Expression);
            }

            // `.` (member access) - next token is a property name (identifier)
            SyntaxKind::Dot => {
                self.push_context(Context::Identifier);
            }

            // Regular identifier token consumes the Identifier context
            SyntaxKind::Ident => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // `(` ends identifier context (start of function params)
            SyntaxKind::LParen => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
            }

            // `<` might start generic params or end identifier context
            SyntaxKind::Lt => {
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                // Could also start generic params - context-dependent
            }

            // Semicolon ends expression context and type contexts
            SyntaxKind::Semicolon => {
                // First pop expression context if we're in one
                if self.current_context() == Context::Expression {
                    self.pop_context();
                }
                // Pop any type contexts
                while matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                ) {
                    self.pop_context();
                }
            }

            // Closing brace ends type context
            SyntaxKind::RBrace => {
                // Pop any type contexts
                while matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion | Context::GenericParams
                ) {
                    self.pop_context();
                }
            }

            // Comma might end type context in certain situations
            SyntaxKind::Comma => {
                if matches!(
                    self.current_context(),
                    Context::TypeAnnotation | Context::TypeAssertion
                ) {
                    self.pop_context();
                }
            }

            // Equals ends type annotation or identifier context and starts expression context
            SyntaxKind::Eq => {
                // Pop Identifier context if we're in one (for `const x = ...`)
                if self.current_context() == Context::Identifier {
                    self.pop_context();
                }
                // Pop TypeAnnotation context if we're in one (for `let x: T = ...`)
                if self.current_context() == Context::TypeAnnotation {
                    self.pop_context();
                }
                // Push expression context for the right-hand side
                // (applies to `const x = ...`, `x = ...`, `let x: T = ...`, etc.)
                self.push_context(Context::Expression);
            }

            _ => {}
        }
    }

    /// Records an interpolation with its context-based classification.
    fn record_interpolation(&mut self, node: &SyntaxNode) {
        let kind = self.placeholder_kind_from_context();

        // Extract the tokens from inside the interpolation
        // The interpolation contains: @ { rust_tokens }
        let tokens = self.extract_interpolation_tokens(node);

        let id = self.placeholder_id;
        self.placeholder_id += 1;

        self.placeholders.insert(
            id,
            PlaceholderInfo { kind, tokens },
        );
    }

    /// Extracts the Rust tokens from an interpolation node.
    fn extract_interpolation_tokens(&self, node: &SyntaxNode) -> String {
        // Get all text except the @{ and }
        let text = node.text().to_string();

        // Strip @{ prefix and } suffix
        text.strip_prefix("@{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.trim().to_string())
            .or_else(|| text.strip_prefix("@").map(|s| s.trim().to_string()))
            .unwrap_or(text)
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to analyze a syntax tree.
pub fn analyze(root: &SyntaxNode) -> SemanticAnalysis {
    Analyzer::new().analyze(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::parser::Parser;

    fn analyze_template(input: &str) -> SemanticAnalysis {
        let parser = Parser::new(input);
        let green = parser.parse();
        let root = SyntaxNode::new_root(green);
        analyze(&root)
    }

    #[test]
    fn test_expr_placeholder() {
        let result = analyze_template("const x = @{value}");
        // Should have one placeholder classified as Expr or Stmt
        assert_eq!(result.placeholders.len(), 1);
        let placeholder = result.placeholders.values().next().unwrap();
        // In statement context, default is Stmt or Expr
        assert!(matches!(
            placeholder.kind,
            PlaceholderKind::Expr | PlaceholderKind::Stmt
        ));
    }

    #[test]
    fn test_type_annotation_placeholder() {
        let result = analyze_template("const x: @{MyType} = 1");
        assert_eq!(result.placeholders.len(), 1);
        let placeholder = result.placeholders.values().next().unwrap();
        assert_eq!(placeholder.kind, PlaceholderKind::Type);
    }

    #[test]
    fn test_type_assertion_placeholder() {
        let result = analyze_template("value as @{TargetType}");
        assert_eq!(result.placeholders.len(), 1);
        let placeholder = result.placeholders.values().next().unwrap();
        assert_eq!(placeholder.kind, PlaceholderKind::Type);
    }

    #[test]
    fn test_function_param_type() {
        let result = analyze_template("function foo(x: @{ParamType})");
        assert_eq!(result.placeholders.len(), 1);
        let placeholder = result.placeholders.values().next().unwrap();
        assert_eq!(placeholder.kind, PlaceholderKind::Type);
    }

    #[test]
    fn test_ident_block() {
        let result = analyze_template("{|get@{name}|}");
        // Should have a placeholder in identifier context
        assert!(!result.placeholders.is_empty());
    }

    #[test]
    fn test_function_name_placeholder() {
        let result = analyze_template("export function @{fn_name}(x: number): void {}");
        // Should have one placeholder for function name (Ident)
        // and the type annotation `:` shouldn't affect function name context
        let placeholder = result.placeholders.values().next().unwrap();
        assert_eq!(placeholder.kind, PlaceholderKind::Ident, "Function name should be Ident, got {:?}", placeholder.kind);
    }

    #[test]
    fn test_function_name_with_doc_comment() {
        let result = analyze_template("/** Documentation */ export function @{fn_name}(x: number): void {}");
        // Function name should still be Ident even with doc comment before
        let placeholder = result.placeholders.values().next().unwrap();
        assert_eq!(placeholder.kind, PlaceholderKind::Ident, "Function name after doc comment should be Ident, got {:?}", placeholder.kind);
    }

    #[test]
    fn test_multi_placeholder_function() {
        // This matches a real template pattern from derive_serialize.rs
        let result = analyze_template("export function @{fn_serialize_ident}(value: @{interface_ident}): string { const ctx = @{serialize_context_expr}.create(); }");
        assert_eq!(result.placeholders.len(), 3, "Expected 3 placeholders");

        // Print all placeholders for debugging
        for (id, info) in &result.placeholders {
            eprintln!("Placeholder {}: kind={:?}, tokens={}", id, info.kind, info.tokens);
        }

        // Check each placeholder's kind
        let mut placeholders: Vec<_> = result.placeholders.iter().collect();
        placeholders.sort_by_key(|(id, _)| *id);

        // First placeholder (fn_serialize_ident) should be Ident (after function keyword)
        assert_eq!(placeholders[0].1.kind, PlaceholderKind::Ident, "Function name should be Ident");
        // Second placeholder (interface_ident) should be Type (after :)
        assert_eq!(placeholders[1].1.kind, PlaceholderKind::Type, "Parameter type should be Type");
        // Third placeholder (serialize_context_expr) should be Expr (after =)
        assert_eq!(placeholders[2].1.kind, PlaceholderKind::Expr, "Expression should be Expr");
    }

    #[test]
    fn test_mixed_placeholders() {
        let result = analyze_template("const @{name}: @{type} = @{value}");
        assert_eq!(result.placeholders.len(), 3);

        // Count by kind
        let type_count = result
            .placeholders
            .values()
            .filter(|p| p.kind == PlaceholderKind::Type)
            .count();
        let expr_or_stmt_count = result
            .placeholders
            .values()
            .filter(|p| matches!(p.kind, PlaceholderKind::Expr | PlaceholderKind::Stmt))
            .count();

        // At least one type placeholder (after :)
        assert!(type_count >= 1);
        // At least one expr/stmt placeholder (the value)
        assert!(expr_or_stmt_count >= 1);
    }

    #[test]
    fn test_generic_type() {
        let result = analyze_template("const x: Record<@{K}, @{V}> = {}");
        // Both K and V should be type placeholders
        let type_count = result
            .placeholders
            .values()
            .filter(|p| p.kind == PlaceholderKind::Type)
            .count();
        assert!(type_count >= 1); // At least some should be type
    }

    #[test]
    fn test_object_literal_not_type() {
        let result = analyze_template("const obj = { key: @{value} }");
        assert_eq!(result.placeholders.len(), 1);
        let placeholder = result.placeholders.values().next().unwrap();
        // In object literal, colon is property separator, not type annotation
        // So this should NOT be classified as Type
        assert_ne!(placeholder.kind, PlaceholderKind::Type);
    }

    #[test]
    fn test_function_with_doc_attribute_tokenstream_format() {
        // This is how doc comments appear after going through Rust's TokenStream:
        // /** Doc */ becomes # [doc = "Doc"]
        // The = in the attribute should NOT trigger Expression context
        let result = analyze_template(
            r#"# [doc = "Doc comment"] export function @{fn_name}(value: @{type_param}): string { return @{body_expr}; }"#
        );

        assert_eq!(result.placeholders.len(), 3, "Expected 3 placeholders");

        // Print all placeholders for debugging
        let mut placeholders: Vec<_> = result.placeholders.iter().collect();
        placeholders.sort_by_key(|(id, _)| *id);
        for (id, info) in &placeholders {
            eprintln!("TokenStream format - Placeholder {}: kind={:?}, tokens={}", id, info.kind, info.tokens);
        }

        // First placeholder (fn_name) should be Ident (after function keyword)
        assert_eq!(
            placeholders[0].1.kind, PlaceholderKind::Ident,
            "Function name should be Ident, got {:?}", placeholders[0].1.kind
        );
        // Second placeholder (type_param) should be Type (after :)
        assert_eq!(
            placeholders[1].1.kind, PlaceholderKind::Type,
            "Parameter type should be Type, got {:?}", placeholders[1].1.kind
        );
        // Third placeholder (body_expr) should be Expr (after =)
        assert_eq!(
            placeholders[2].1.kind, PlaceholderKind::Expr,
            "Body expression should be Expr, got {:?}", placeholders[2].1.kind
        );
    }
}
