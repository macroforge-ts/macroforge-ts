//! Intermediate Representation for the template language.
//!
//! The IR is produced by the parser with inline placeholder classification
//! and consumed by codegen to produce Rust code.

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

/// A node in the IR.
#[derive(Debug, Clone)]
pub enum IrNode {
    /// Literal text to emit verbatim.
    Text(String),

    /// A placeholder that will be substituted.
    Placeholder {
        /// Classification (expr, type, ident, stmt).
        kind: PlaceholderKind,
        /// The Rust expression tokens.
        rust_expr: String,
    },

    /// Identifier block: concatenated parts forming a single identifier.
    IdentBlock {
        /// Parts (text or placeholders) to concatenate.
        parts: Vec<IrNode>,
    },

    /// String interpolation: a string with embedded expressions.
    StringInterp {
        /// Quote style: '"', '\'', or '`'.
        quote: char,
        /// Parts of the string.
        parts: Vec<IrNode>,
    },

    /// If/else-if/else control flow.
    If {
        /// The condition (Rust expression).
        condition: String,
        /// Body when condition is true.
        then_body: Vec<IrNode>,
        /// Else-if branches: (condition, body).
        else_if_branches: Vec<(String, Vec<IrNode>)>,
        /// Else branch body.
        else_body: Option<Vec<IrNode>>,
    },

    /// For loop.
    For {
        /// Loop variable pattern.
        pattern: String,
        /// Iterator expression.
        iterator: String,
        /// Loop body.
        body: Vec<IrNode>,
    },

    /// While loop.
    While {
        /// Loop condition.
        condition: String,
        /// Loop body.
        body: Vec<IrNode>,
    },

    /// Match expression.
    Match {
        /// Expression to match.
        expr: String,
        /// Match arms: (pattern, guard, body).
        arms: Vec<(String, Option<String>, Vec<IrNode>)>,
    },

    /// Let binding directive.
    Let {
        /// Variable name.
        name: String,
        /// Whether it's mutable.
        mutable: bool,
        /// Optional type annotation (e.g., "Expr" in `name: Expr`).
        type_hint: Option<String>,
        /// Initial value expression.
        value: String,
    },

    /// Do directive (execute Rust code).
    Do {
        /// Rust code to execute.
        code: String,
    },

    /// TypeScript injection directive.
    TypeScript {
        /// TokenStream to inject.
        stream: String,
    },

    /// A comment (preserved for debugging).
    Comment {
        /// Comment text.
        text: String,
    },
}

/// The complete IR for a template.
#[derive(Debug)]
pub struct Ir {
    /// Root nodes of the template.
    pub nodes: Vec<IrNode>,
}

impl Ir {
    /// Creates a new empty IR.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Creates an IR with the given nodes.
    pub fn with_nodes(nodes: Vec<IrNode>) -> Self {
        Self { nodes }
    }
}

impl Default for Ir {
    fn default() -> Self {
        Self::new()
    }
}
