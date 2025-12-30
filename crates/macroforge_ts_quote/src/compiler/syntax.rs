//! Syntax kinds for the template language.
//!
//! This module defines all token and node types in the template grammar.
//! These are used by both the lexer (for tokens) and parser (for nodes).

/// All syntax kinds in the template language.
///
/// Tokens are prefixed with their category:
/// - Literals: Text, StringLit, etc.
/// - Punctuation: At, LBrace, RBrace, etc.
/// - Keywords: IfKw, ForKw, etc.
/// - Special: Error, Whitespace, etc.
///
/// Nodes are suffixed with their category:
/// - Expr: Expression nodes
/// - Stmt: Statement nodes
/// - Block: Block nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // ============================================
    // Tokens - Atoms produced by the lexer
    // ============================================

    // Literals and text
    /// Plain text content (TypeScript code, etc.)
    Text = 0,
    /// Whitespace (spaces, tabs, newlines)
    Whitespace,
    /// Error token (invalid input)
    Error,

    // Punctuation
    /// `@` - interpolation prefix
    At,
    /// `@@` - escaped @ (produces literal @)
    AtAt,
    /// `{` - left brace
    LBrace,
    /// `}` - right brace
    RBrace,
    /// `{#` - control flow open
    HashOpen,
    /// `{/` - control flow close
    SlashOpen,
    /// `{:` - control flow continuation (else, case)
    ColonOpen,
    /// `{$` - directive open (let, do, typescript)
    DollarOpen,
    /// `` - ident block open
    PipeOpen,
    /// `` - ident block close
    PipeClose,
    /// `{>` - line comment open
    CommentLineOpen,
    /// `<}` - line comment close
    CommentLineClose,
    /// `{>>` - block comment open
    CommentBlockOpen,
    /// `<<}` - block comment close
    CommentBlockClose,
    /// `"` - double quote
    DoubleQuote,
    /// `'` - single quote
    SingleQuote,
    /// `` ` `` - backtick
    Backtick,
    /// `///` - doc comment prefix
    DocCommentPrefix,
    /// `/**` - jsdoc block open
    JsDocOpen,
    /// `*/` - jsdoc block close
    JsDocClose,
    /// `#[doc = "..."]` - Rust doc attribute (converted from `/** ... */` by Rust tokenizer)
    RustDocAttr,
    /// `:` - colon (for type annotations, object properties)
    Colon,
    /// `;` - semicolon
    Semicolon,
    /// `(` - left paren
    LParen,
    /// `)` - right paren
    RParen,
    /// `[` - left bracket
    LBracket,
    /// `]` - right bracket
    RBracket,
    /// `<` - less than (for generics)
    Lt,
    /// `>` - greater than (for generics)
    Gt,
    /// `,` - comma
    Comma,
    /// `=` - equals
    Eq,
    /// `?` - question mark (for optional)
    Question,
    /// `.` - dot
    Dot,

    // Keywords (inside control blocks)
    /// `if` keyword
    IfKw,
    /// `else` keyword
    ElseKw,
    /// `for` keyword
    ForKw,
    /// `while` keyword
    WhileKw,
    /// `match` keyword
    MatchKw,
    /// `case` keyword
    CaseKw,
    /// `let` keyword
    LetKw,
    /// `do` keyword
    DoKw,
    /// `in` keyword
    InKw,
    /// `typescript` keyword
    TypeScriptKw,
    /// `mut` keyword
    MutKw,

    // TypeScript-specific (for type position detection)
    /// `as` keyword (type assertion)
    AsKw,
    /// `function` keyword
    FunctionKw,
    /// `const` keyword
    ConstKw,
    /// `var` keyword
    VarKw,
    /// `class` keyword
    ClassKw,
    /// `interface` keyword
    InterfaceKw,
    /// `type` keyword
    TypeKw,
    /// `export` keyword
    ExportKw,
    /// `import` keyword
    ImportKw,
    /// `return` keyword
    ReturnKw,
    /// `new` keyword
    NewKw,
    /// `keyof` keyword (type operator)
    KeyofKw,
    /// `typeof` keyword (type operator)
    TypeofKw,
    /// `extends` keyword
    ExtendsKw,
    /// `implements` keyword
    ImplementsKw,
    /// `readonly` keyword
    ReadonlyKw,
    /// `private` keyword
    PrivateKw,
    /// `public` keyword
    PublicKw,
    /// `protected` keyword
    ProtectedKw,
    /// `static` keyword
    StaticKw,
    /// `abstract` keyword
    AbstractKw,
    /// `declare` keyword
    DeclareKw,
    /// `async` keyword
    AsyncKw,
    /// `await` keyword
    AwaitKw,
    /// `yield` keyword
    YieldKw,
    /// `throw` keyword
    ThrowKw,
    /// `try` keyword
    TryKw,
    /// `catch` keyword
    CatchKw,
    /// `finally` keyword
    FinallyKw,
    /// `default` keyword
    DefaultKw,
    /// `from` keyword
    FromKw,
    /// `of` keyword
    OfKw,
    /// `get` keyword
    GetKw,
    /// `set` keyword
    SetKw,
    /// `satisfies` keyword
    SatisfiesKw,
    /// `infer` keyword
    InferKw,
    /// `is` keyword (type guard)
    IsKw,

    // Identifiers and expressions
    /// An identifier (variable name, type name, etc.)
    Ident,
    /// A Rust token stream (inside @{...})
    RustTokens,

    // ============================================
    // Composite nodes - Built by the parser
    // ============================================
    /// Root node of the template
    Root,

    // Interpolation nodes
    /// `@{expr}` - basic interpolation
    Interpolation,
    /// ` parts ` - ident block (concatenated identifier)
    IdentBlock,

    // String/template nodes
    /// `"string with @{expr}"` - string interpolation
    StringInterp,
    /// `` `template ${expr}` `` - template literal
    TemplateLiteral,
    /// Part of a string (text or interpolation)
    StringPart,

    // Comment nodes
    /// `{> "comment" <}` - line comment
    LineComment,
    /// `{>> "comment" <<}` - block comment
    BlockComment,
    /// `/// doc` or `/** doc */` - doc comment
    DocComment,

    // Control flow nodes
    /// `{#if cond}...{/if}` - if block
    IfBlock,
    /// `{#if let pat = expr}...{/if}` - if-let block
    IfLetBlock,
    /// `{:else}...` - else clause
    ElseClause,
    /// `{:else if cond}...` - else-if clause
    ElseIfClause,
    /// `{#for pat in iter}...{/for}` - for loop
    ForBlock,
    /// `{#while cond}...{/while}` - while loop
    WhileBlock,
    /// `{#while let pat = expr}...{/while}` - while-let loop
    WhileLetBlock,
    /// `{#match expr}...{/match}` - match block
    MatchBlock,
    /// `{:case pat}...` - match case
    MatchCase,

    // Directive nodes
    /// `{$let name = expr}` - let binding
    LetDirective,
    /// `{$do expr}` - do expression
    DoDirective,
    /// `{$typescript stream}` - typescript injection
    TypeScriptDirective,

    // TypeScript structural nodes (for type position detection)
    /// A TypeScript statement
    TsStmt,
    /// A TypeScript expression
    TsExpr,
    /// A TypeScript type annotation (`: Type`)
    TypeAnnotation,
    /// A TypeScript type assertion (`as Type` or `<Type>`)
    TypeAssertion,
    /// A TypeScript function declaration/expression
    TsFunction,
    /// A TypeScript parameter (with optional type)
    TsParam,
    /// A TypeScript generic type parameters (`<T, U>`)
    TsTypeParams,
    /// A TypeScript object literal
    TsObject,
    /// A TypeScript object property
    TsObjectProp,
    /// A TypeScript class declaration
    TsClass,
    /// A TypeScript class member
    TsClassMember,

    // Brace block (preserves structure)
    /// `{ ... }` - brace-delimited block
    BraceBlock,

    // Object property loop
    /// `{ {#for (k,v) in items} @{k}: @{v}, {/for} }` - object from loop
    ObjectPropLoop,

    // Placeholder markers (after semantic analysis)
    /// Placeholder in expression position
    ExprPlaceholder,
    /// Placeholder in type position
    TypePlaceholder,
    /// Placeholder in identifier position
    IdentPlaceholder,
    /// Placeholder in identifier name position (string → ident)
    IdentNamePlaceholder,
    /// Placeholder in statement position
    StmtPlaceholder,

    // Sentinel - must be last
    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    /// Returns true if this is a token (vs a composite node).
    pub fn is_token(self) -> bool {
        (self as u16) < (Self::Root as u16)
    }

    /// Returns true if this is a trivia token (whitespace, comments).
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }

    /// Returns true if this is a control flow keyword.
    pub fn is_control_keyword(self) -> bool {
        matches!(
            self,
            Self::IfKw | Self::ElseKw | Self::ForKw | Self::WhileKw | Self::MatchKw | Self::CaseKw
        )
    }

    /// Returns true if this is a directive keyword.
    pub fn is_directive_keyword(self) -> bool {
        matches!(
            self,
            Self::LetKw | Self::DoKw | Self::TypeScriptKw | Self::MutKw
        )
    }

    /// Returns true if this is a TypeScript keyword.
    pub fn is_ts_keyword(self) -> bool {
        matches!(
            self,
            Self::AsKw
                | Self::FunctionKw
                | Self::ConstKw
                | Self::VarKw
                | Self::ClassKw
                | Self::InterfaceKw
                | Self::TypeKw
                | Self::ExportKw
                | Self::ImportKw
                | Self::ReturnKw
                | Self::NewKw
                | Self::KeyofKw
                | Self::TypeofKw
                | Self::ExtendsKw
                | Self::ImplementsKw
                | Self::ReadonlyKw
                | Self::PrivateKw
                | Self::PublicKw
                | Self::ProtectedKw
                | Self::StaticKw
                | Self::AbstractKw
                | Self::DeclareKw
                | Self::AsyncKw
                | Self::AwaitKw
                | Self::YieldKw
                | Self::ThrowKw
                | Self::TryKw
                | Self::CatchKw
                | Self::FinallyKw
                | Self::DefaultKw
                | Self::FromKw
                | Self::OfKw
                | Self::GetKw
                | Self::SetKw
                | Self::SatisfiesKw
                | Self::InferKw
                | Self::IsKw
        )
    }

    /// Returns true if this keyword introduces an identifier context.
    /// After these keywords, the next token is typically an identifier being declared.
    pub fn starts_identifier_context(self) -> bool {
        matches!(
            self,
            Self::FunctionKw
                | Self::ClassKw
                | Self::InterfaceKw
                | Self::TypeKw
                | Self::ConstKw
                | Self::LetKw
                | Self::VarKw
        )
    }

    /// Returns true if this keyword introduces a type context.
    /// After these keywords, the following tokens are types.
    pub fn starts_type_context(self) -> bool {
        matches!(
            self,
            Self::AsKw
                | Self::KeyofKw
                | Self::TypeofKw
                | Self::ExtendsKw
                | Self::ImplementsKw
                | Self::SatisfiesKw
                | Self::InferKw
                | Self::IsKw
        )
    }

    /// Returns true if this keyword introduces an expression context.
    pub fn starts_expression_context(self) -> bool {
        matches!(
            self,
            Self::ReturnKw
                | Self::ThrowKw
                | Self::YieldKw
                | Self::AwaitKw
                | Self::NewKw
        )
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// The language definition for Rowan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TemplateLanguage {}

impl rowan::Language for TemplateLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < SyntaxKind::__LAST as u16);
        // SAFETY: We just checked that raw.0 is in range
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// Type alias for the syntax node.
pub type SyntaxNode = rowan::SyntaxNode<TemplateLanguage>;
/// Type alias for the syntax token.
pub type SyntaxToken = rowan::SyntaxToken<TemplateLanguage>;

#[cfg(test)]
mod tests {
    use super::*;
    use rowan::Language;

    #[test]
    fn test_syntax_kind_is_token() {
        assert!(SyntaxKind::Text.is_token());
        assert!(SyntaxKind::At.is_token());
        assert!(SyntaxKind::IfKw.is_token());
        assert!(!SyntaxKind::Root.is_token());
        assert!(!SyntaxKind::Interpolation.is_token());
    }

    #[test]
    fn test_syntax_kind_conversions() {
        let kind = SyntaxKind::Text;
        let raw: rowan::SyntaxKind = kind.into();
        let back = TemplateLanguage::kind_from_raw(raw);
        assert_eq!(kind, back);
    }

    #[test]
    fn test_is_control_keyword() {
        assert!(SyntaxKind::IfKw.is_control_keyword());
        assert!(SyntaxKind::ForKw.is_control_keyword());
        assert!(!SyntaxKind::LetKw.is_control_keyword());
        assert!(!SyntaxKind::Text.is_control_keyword());
    }
}
