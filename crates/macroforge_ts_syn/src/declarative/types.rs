//! Type definitions for parsed declarative macros.

use crate::abi::SpanIR;

/// Position where a declarative macro expands.
///
/// Value-position macros (the default) are invoked as `$name(args)` in
/// expression position and match their arms against OXC `Argument`
/// nodes. Type-position macros are invoked as `$name<T1, T2>` inside
/// TS type annotations and match their arms against `TSType` nodes.
///
/// A macro is either one or the other — never both. Invoking a
/// type-only macro from value position (or vice versa) is a hard
/// error at the call site.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MacroKind {
    /// Value-position macro. Called as `$name(args)` in expression
    /// position. The default for tag-form macros and for object-form
    /// macros that omit `kind`.
    #[default]
    Value,
    /// Type-position macro. Called as `$name<T1, T2>` in type
    /// position. Requires `kind: "type"` in the object form — tag
    /// form cannot declare type macros because TS grammar doesn't
    /// permit tagged templates in type position.
    Type,
}

/// A fully parsed declarative macro definition.
///
/// Produced by [`crate::declarative::parse_macro_def`] from the template
/// body of a `` const $name = macroRules`...` `` declaration, or by the
/// host discovery pass for the object-form
/// `const $name = macroRules({ expand, runtime, call, mode })`. Arms are
/// tried in source order at invocation time; first match wins.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MacroDef {
    /// The macro name, without the leading `$` (so `$vec` → `"vec"`).
    ///
    /// The host sets this field during discovery; the parser alone
    /// does not know the binding name, so it leaves this empty.
    pub name: String,

    /// Arms in source order. These are the dev-form / "expand" arms used
    /// when the macro expands inline at each call site. Populated from
    /// either the tag form's template body or the object form's `expand`
    /// field.
    pub arms: Vec<MacroArm>,

    /// Mode that controls when arms vs call_arms are used. Defaults to
    /// [`MacroMode::ExpandOnly`] when the tag form is used.
    pub mode: MacroMode,

    /// Whether this macro expands in value position or type position.
    /// Defaults to [`MacroKind::Value`] for both tag and object forms
    /// unless the object form explicitly passes `kind: "type"`.
    #[serde(default)]
    pub kind: MacroKind,

    /// Verbatim runtime source — a top-level helper function body that
    /// gets emitted once per file when the macro is in a sharing mode
    /// (`ShareOnly`, `ShareAnyway`, or `Auto` above the megamorphism
    /// threshold). `None` for tag-form macros and for `ExpandOnly` object
    /// forms.
    #[serde(default)]
    pub runtime: Option<String>,

    /// Call-site template arms used when the macro emits to share mode.
    /// Each call site expands `call_arms` (which typically just calls the
    /// runtime helper with per-call data) instead of the full `arms`.
    /// `None` for tag-form macros and for `ExpandOnly` object forms.
    #[serde(default)]
    pub call_arms: Option<Vec<MacroArm>>,

    /// The number of distinct call-site "shapes" above which an `Auto`
    /// macro's shared runtime is considered megamorphic, triggering
    /// clustering or forced expansion. Defaults to 4 (the value the
    /// design doc recommends). Only meaningful for `Auto` mode.
    #[serde(default = "default_megamorphism_threshold")]
    pub megamorphism_threshold: u8,

    /// Span of the template body (relative to the original source file),
    /// set by the host during discovery. The parser records spans inside
    /// each arm relative to the same coordinate system.
    pub span: SpanIR,
}

fn default_megamorphism_threshold() -> u8 {
    4
}

impl MacroDef {
    /// Construct a `MacroDef` from just the expand-mode arms, with the
    /// reverse-mono fields defaulted. Used by tests and by the tag-form
    /// path in [`crate::declarative::parse_macro_def`].
    pub fn from_arms(name: String, arms: Vec<MacroArm>, mode: MacroMode, span: SpanIR) -> Self {
        Self {
            name,
            arms,
            mode,
            kind: MacroKind::Value,
            runtime: None,
            call_arms: None,
            megamorphism_threshold: default_megamorphism_threshold(),
            span,
        }
    }
}

/// A single arm of a declarative macro.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MacroArm {
    /// Pattern matched against call arguments.
    pub pattern: Pattern,
    /// Template body spliced at the call site when the pattern matches.
    pub body: Body,
    /// Span of this arm within the original template body.
    pub span: SpanIR,
}

/// A pattern that matches a sequence of call arguments.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Pattern {
    /// `()` — matches a call with zero arguments.
    Empty,
    /// `(<elements>)` — matches a call argument-by-argument.
    Sequence(Vec<PatternElement>),
}

/// A single element within a [`Pattern::Sequence`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RepetitionKind {
    /// `*` — zero or more matches.
    ZeroOrMore,
    /// `+` — one or more matches.
    OneOrMore,
    /// `?` — zero or one match.
    ZeroOrOne,
}

/// A parsed macro body (the text after `=>`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Body(pub Vec<BodyToken>);

/// A single token in a body template.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BodyToken {
    /// Literal source text to emit as-is.
    Literal(String),

    /// `$<name>` — substitute the single binding named `<name>`.
    Substitution(String),

    /// `$<name>(<args>)` — call another declarative macro by name,
    /// passing the inner tokens as arguments. At expansion time the
    /// host renders the args, re-parses them as OXC `CallExpression`
    /// arguments, matches them against the callee's arms, and
    /// recursively expands the matched arm's body.
    ///
    /// Phase 12 — this variant is only produced when the body parser
    /// sees `$ident(` and consumes the balanced arg list. Outside of
    /// inter-macro composition, `$ident` remains a plain substitution.
    MacroCall {
        /// Callee name sans the leading `$`.
        name: String,
        /// Body tokens that make up the arg list, including the
        /// commas between args. The expander renders these into a
        /// source string that OXC then parses as a call's argument
        /// list.
        args: Vec<BodyToken>,
    },

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
/// Tag-form macros (`` const $x = macroRules`...` ``) always produce
/// [`MacroMode::ExpandOnly`]. Object-form macros
/// (`const $x = macroRules({ mode: "auto", ... })`) may pick any variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MacroMode {
    /// Always expand inline at every call site. The default for tag-form
    /// macros and the safe default when `mode` is omitted in the object
    /// form without specifying a shared runtime.
    #[default]
    ExpandOnly,
    /// Always emit one shared runtime helper per file and replace each
    /// call site with a call to it. User opts in manually; no analysis.
    ShareOnly,
    /// Like `ShareOnly`, but silences the megamorphism warning even when
    /// the shared runtime would become megamorphic. Used for cold-path
    /// macros where the author has weighed the perf trade-off.
    ShareAnyway,
    /// In dev: expand inline for precise diagnostics. In prod: the
    /// megamorphism analyzer picks `share`, `cluster`, or `force-expand`.
    Auto,
}

impl MacroMode {
    /// Parse a mode from the JS string value in a `macroRules({ mode: "..." })`
    /// call. Returns `None` for unknown strings.
    pub fn from_str_value(s: &str) -> Option<Self> {
        Some(match s {
            "expand-only" | "expandOnly" => MacroMode::ExpandOnly,
            "share-only" | "shareOnly" => MacroMode::ShareOnly,
            "share-anyway" | "shareAnyway" => MacroMode::ShareAnyway,
            "auto" => MacroMode::Auto,
            _ => return None,
        })
    }

    /// The comma-separated list of valid mode strings, used in error
    /// messages when the user passes an unknown mode.
    pub fn known_values() -> &'static str {
        "\"auto\", \"expand-only\", \"share-only\", \"share-anyway\""
    }

    /// Is this mode one of the sharing modes (i.e., does it require a
    /// `runtime` and `call` pair in the object form)?
    pub fn is_sharing(self) -> bool {
        matches!(
            self,
            MacroMode::ShareOnly | MacroMode::ShareAnyway | MacroMode::Auto
        )
    }
}
