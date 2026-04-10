//! Type definitions for parsed declarative macros.

use crate::abi::SpanIR;

/// A fully parsed declarative macro definition.
///
/// Produced by [`crate::declarative::parse_macro_def`] from the template
/// body of a `` const $name = macro`...` `` declaration. Arms are tried in
/// source order at invocation time; first match wins.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroDef {
    /// The macro name, without the leading `$` (so `$vec` → `"vec"`).
    ///
    /// The host sets this field during discovery; the parser alone
    /// does not know the binding name, so it leaves this empty.
    pub name: String,

    /// Arms in source order.
    pub arms: Vec<MacroArm>,

    /// Reserved for future reverse-monomorphization work. Always
    /// [`MacroMode::ExpandOnly`] in the current MVP.
    pub mode: MacroMode,

    /// Span of the template body (relative to the original source file),
    /// set by the host during discovery. The parser records spans inside
    /// each arm relative to the same coordinate system.
    pub span: SpanIR,
}

/// A single arm of a declarative macro.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroArm {
    /// Pattern matched against call arguments.
    pub pattern: Pattern,
    /// Template body spliced at the call site when the pattern matches.
    pub body: Body,
    /// Span of this arm within the original template body.
    pub span: SpanIR,
}

/// A pattern that matches a sequence of call arguments.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// `()` — matches a call with zero arguments.
    Empty,
    /// `(<elements>)` — matches a call argument-by-argument.
    Sequence(Vec<PatternElement>),
}

/// A single element within a [`Pattern::Sequence`].
#[derive(Debug, Clone, PartialEq)]
pub enum PatternElement {
    /// A literal token (e.g., a trailing comma inside `$(,)?`). Matched
    /// against the verbatim source slice of the corresponding call argument.
    Literal(String),

    /// `$<name>:<kind>` — binds a single call argument to `<name>` if the
    /// argument's AST shape matches `<kind>`.
    Fragment { name: String, kind: FragmentKind },

    /// `$(<inner>)<sep><kind>` — matches zero or more arguments against
    /// the inner pattern, optionally requiring `<sep>` between consecutive
    /// elements. The repetition kind (`*`, `+`, `?`) is enforced at match time.
    Repetition {
        pattern: Box<Pattern>,
        separator: Option<String>,
        kind: RepetitionKind,
    },
}

/// Which AST shape a fragment specifier accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentKind {
    /// Any expression (OXC `Expression`).
    Expr,
    /// Any statement (OXC `Statement`).
    Stmt,
    /// A brace-delimited block (OXC `BlockStatement`). Not commonly used
    /// in call-argument position; supported for parity with the design doc.
    Block,
    /// A bare identifier (`Expression::Identifier`).
    Ident,
    /// A TypeScript type (`TSType`). Not supported in call-argument position
    /// in the MVP; reserved for future type-position macros.
    Type,
    /// A destructuring pattern (`BindingPattern`). Reserved.
    Pat,
    /// A literal (string, number, boolean, template, regex).
    Lit,
    /// A qualified name (`a.b.c`).
    Path,
    /// A top-level item (class, function, interface, etc.). Reserved.
    Item,
    /// A decorator expression. Reserved.
    Decorator,
    /// Structural fallback — matches any expression.
    Tt,
}

impl FragmentKind {
    /// Parses the fragment kind name as it appears after `$ident:` in
    /// pattern source. Returns `None` for unknown kinds.
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Expr" | "expr" => FragmentKind::Expr,
            "Stmt" | "stmt" => FragmentKind::Stmt,
            "Block" | "block" => FragmentKind::Block,
            "Ident" | "ident" => FragmentKind::Ident,
            "Type" | "ty" => FragmentKind::Type,
            "Pat" | "pat" => FragmentKind::Pat,
            "Lit" | "literal" => FragmentKind::Lit,
            "Path" | "path" => FragmentKind::Path,
            "Item" | "item" => FragmentKind::Item,
            "Decorator" | "decorator" => FragmentKind::Decorator,
            "Tt" | "tt" => FragmentKind::Tt,
            _ => return None,
        })
    }

    /// Returns the comma-separated list of known fragment kind names,
    /// used for diagnostic messages.
    pub fn known_names() -> &'static str {
        "Expr, Stmt, Block, Ident, Type, Pat, Lit, Path, Item, Decorator, Tt"
    }
}

/// Count constraint for a repetition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepetitionKind {
    /// `*` — zero or more matches.
    ZeroOrMore,
    /// `+` — one or more matches.
    OneOrMore,
    /// `?` — zero or one match.
    ZeroOrOne,
}

/// A parsed macro body (the text after `=>`).
#[derive(Debug, Clone, PartialEq)]
pub struct Body(pub Vec<BodyToken>);

/// A single token in a body template.
#[derive(Debug, Clone, PartialEq)]
pub enum BodyToken {
    /// Literal source text to emit as-is.
    Literal(String),

    /// `$<name>` — substitute the single binding named `<name>`.
    Substitution(String),

    /// `$(<inner>)<sep><kind>` — iterate the named sequence bindings
    /// inside `<inner>`, expanding `<inner>` once per element and joining
    /// with `<sep>`.
    Repetition {
        body: Vec<BodyToken>,
        separator: Option<String>,
        kind: RepetitionKind,
    },
}

impl Body {
    /// Returns an empty body.
    pub fn empty() -> Self {
        Body(Vec::new())
    }
}

/// Controls when and how a macro's template is emitted at build time.
///
/// MVP only implements [`MacroMode::ExpandOnly`] — the other three modes
/// from the design doc (`auto`, `share-only`, `share-anyway`) are part of
/// the reverse-monomorphization follow-up.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MacroMode {
    /// Always expand inline at every call site. The MVP default.
    #[default]
    ExpandOnly,
}
