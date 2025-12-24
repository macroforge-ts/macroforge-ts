//! Rust-style templating for TypeScript code generation (AST-based)
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions (calls `.to_string()`)
//! - `{| content |}` - Ident block: concatenates content without spaces (e.g., `{|get@{name}|}` → `getUser`)
//! - `{> "comment" <}` - Line comment: outputs `// comment` (string preserves whitespace)
//! - `{>> "comment" <<}` - Block comment: outputs `/* comment */` (string preserves whitespace)
//! - `///` or `/** */` - Rust doc comments in the template emit JSDoc blocks (`/** ... */`)
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{#while cond}...{/while}` - While loop
//! - `{#while let pattern = expr}...{/while}` - While-let pattern matching loop
//! - `{$let name = expr}` - Local constants
//! - `{$let mut name = expr}` - Mutable local binding
//! - `{$do expr}` - Execute side-effectful expression (discard result)
//! - `{$typescript stream}` - Inject a TsStream, preserving its source and runtime_patches (imports)
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

// Submodules
mod comment;
mod compile;
mod flush;
mod parse;
mod utils;

#[cfg(test)]
mod tests;

// Re-export the main public interface
pub use parse::parse_template;

// Re-export submodule items that are actually used
pub(crate) use comment::build_comment_expr;
#[cfg(test)]
pub(crate) use parse::parse_segments;
pub(crate) use utils::tokens_to_ts_string;

// External crate imports for type definitions
use proc_macro2::{Span, TokenStream as TokenStream2};

// Additional imports only needed for tests
#[cfg(test)]
use proc_macro2::{Delimiter, TokenTree};

/// Generates unique placeholder IDs for dynamic segments.
pub(crate) struct IdGen {
    next: usize,
}

impl IdGen {
    /// Creates a new placeholder ID generator.
    fn new() -> Self {
        Self { next: 0 }
    }

    /// Returns the next unique placeholder ID.
    fn next(&mut self) -> usize {
        let id = self.next;
        self.next += 1;
        id
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Segment {
    Static(String),
    Comment {
        style: CommentStyle,
        text: String,
    },
    Interpolation {
        id: usize,
        expr: TokenStream2,
    },
    StringInterp {
        parts: Vec<StringPart>,
    },
    TemplateInterp {
        parts: Vec<StringPart>,
    },
    IdentBlock {
        parts: Vec<IdentPart>,
    },
    Control {
        node: ControlNode,
    },
    Let {
        tokens: TokenStream2,
    },
    Do {
        expr: TokenStream2,
    },
    Typescript {
        expr: TokenStream2,
    },
    /// A nested brace block containing inner segments.
    /// Used to preserve the atomic structure of function bodies and object literals
    /// while still detecting interpolations and control flow inside.
    BraceBlock {
        inner: Vec<Segment>,
    },
    /// An object literal with properties generated from a for loop.
    /// This handles the pattern: `{ {#for (key, val) in items} @{key}: @{val}, {/for} }`
    /// The loop is compiled at Rust macro expansion time, not TypeScript runtime.
    ObjectPropLoop {
        /// The loop pattern (e.g., `(name_ident, value_expr)`)
        pat: TokenStream2,
        /// The iterator expression (e.g., `object_fields`)
        iter: TokenStream2,
        /// The key expression from the loop body (e.g., `name_ident`)
        key_expr: TokenStream2,
        /// The value expression from the loop body (e.g., `value_expr`)
        value_expr: TokenStream2,
    },
}

#[derive(Clone, Debug)]
pub(crate) enum IdentPart {
    Static(String),
    Interpolation { expr: TokenStream2 },
}

#[derive(Clone, Debug)]
pub(crate) enum StringPart {
    Text(String),
    Expr(TokenStream2),
}

#[derive(Clone, Debug)]
pub(crate) enum ControlNode {
    If {
        cond: TokenStream2,
        then_branch: Vec<Segment>,
        else_branch: Option<Vec<Segment>>,
    },
    IfLet {
        pattern: TokenStream2,
        expr: TokenStream2,
        then_branch: Vec<Segment>,
        else_branch: Option<Vec<Segment>>,
    },
    For {
        pat: TokenStream2,
        iter: TokenStream2,
        body: Vec<Segment>,
    },
    While {
        cond: TokenStream2,
        body: Vec<Segment>,
    },
    WhileLet {
        pattern: TokenStream2,
        expr: TokenStream2,
        body: Vec<Segment>,
    },
    Match {
        expr: TokenStream2,
        cases: Vec<MatchCase>,
    },
}

#[derive(Clone, Debug)]
pub(crate) enum CommentStyle {
    DocBlock,
    Block,
    Line,
}

#[derive(Clone, Debug)]
pub(crate) struct MatchCase {
    pub(crate) pattern: TokenStream2,
    pub(crate) body: Vec<Segment>,
}

#[derive(Debug, Clone)]
pub(crate) enum Terminator {
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    EndWhile,
    Case(TokenStream2),
    EndMatch,
}

#[derive(Debug)]
pub(crate) enum TagType {
    If(TokenStream2),
    IfLet(TokenStream2, TokenStream2),
    For(TokenStream2, TokenStream2),
    Match(TokenStream2),
    While(TokenStream2),
    WhileLet(TokenStream2, TokenStream2),
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    EndWhile,
    Case(TokenStream2),
    EndMatch,
    Let(TokenStream2),
    Do(TokenStream2),
    Typescript(TokenStream2),
    IdentBlock,
    DocComment(String),
    BlockComment(String),
    Block,
}

/// Builds a template error with optional context.
pub(crate) fn template_error(span: Span, message: &str, context: Option<&str>) -> syn::Error {
    let full_message = if let Some(ctx) = context {
        format!("{}\n  --> in: {}", message, ctx)
    } else {
        message.to_string()
    };
    syn::Error::new(span, full_message)
}
