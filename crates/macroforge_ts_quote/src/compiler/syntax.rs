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
    /// `@` followed by identifier - TypeScript decorator marker
    DecoratorAt,
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
    /// `*` - star/asterisk (for namespace imports, multiplication)
    Star,
    /// `#` - hash (for private names like #field)
    Hash,
    /// `!` - exclamation mark (for non-null assertions)
    Exclaim,
    /// `++` - increment operator
    PlusPlus,
    /// `--` - decrement operator
    MinusMinus,
    /// `?.` - optional chaining
    QuestionDot,
    /// `&` - ampersand (for intersection types)
    Ampersand,
    /// `...` - spread/rest operator
    DotDotDot,

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
    /// `enum` keyword
    EnumKw,

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

    // ==================== is_token Tests ====================

    #[test]
    fn test_syntax_kind_is_token() {
        assert!(SyntaxKind::Text.is_token());
        assert!(SyntaxKind::At.is_token());
        assert!(SyntaxKind::IfKw.is_token());
        assert!(!SyntaxKind::Root.is_token());
        assert!(!SyntaxKind::Interpolation.is_token());
    }

    #[test]
    fn test_all_tokens_are_tokens() {
        let tokens = [
            SyntaxKind::Text,
            SyntaxKind::Whitespace,
            SyntaxKind::Error,
            SyntaxKind::At,
            SyntaxKind::AtAt,
            SyntaxKind::LBrace,
            SyntaxKind::RBrace,
            SyntaxKind::HashOpen,
            SyntaxKind::SlashOpen,
            SyntaxKind::ColonOpen,
            SyntaxKind::DollarOpen,
            SyntaxKind::Colon,
            SyntaxKind::Semicolon,
            SyntaxKind::LParen,
            SyntaxKind::RParen,
            SyntaxKind::LBracket,
            SyntaxKind::RBracket,
            SyntaxKind::Lt,
            SyntaxKind::Gt,
            SyntaxKind::Comma,
            SyntaxKind::Eq,
            SyntaxKind::Question,
            SyntaxKind::Dot,
            SyntaxKind::IfKw,
            SyntaxKind::ElseKw,
            SyntaxKind::ForKw,
            SyntaxKind::WhileKw,
            SyntaxKind::MatchKw,
            SyntaxKind::CaseKw,
            SyntaxKind::LetKw,
            SyntaxKind::DoKw,
            SyntaxKind::Ident,
            SyntaxKind::RustTokens,
        ];
        for token in tokens {
            assert!(token.is_token(), "{:?} should be a token", token);
        }
    }

    #[test]
    fn test_composite_nodes_are_not_tokens() {
        let nodes = [
            SyntaxKind::Root,
            SyntaxKind::Interpolation,
            SyntaxKind::IdentBlock,
            SyntaxKind::StringInterp,
            SyntaxKind::TemplateLiteral,
            SyntaxKind::LineComment,
            SyntaxKind::BlockComment,
            SyntaxKind::DocComment,
            SyntaxKind::IfBlock,
            SyntaxKind::ForBlock,
            SyntaxKind::WhileBlock,
            SyntaxKind::MatchBlock,
            SyntaxKind::LetDirective,
            SyntaxKind::DoDirective,
        ];
        for node in nodes {
            assert!(!node.is_token(), "{:?} should not be a token", node);
        }
    }

    // ==================== Conversion Tests ====================

    #[test]
    fn test_syntax_kind_conversions() {
        let kind = SyntaxKind::Text;
        let raw: rowan::SyntaxKind = kind.into();
        let back = TemplateLanguage::kind_from_raw(raw);
        assert_eq!(kind, back);
    }

    #[test]
    fn test_syntax_kind_round_trip_all_kinds() {
        // Test a sampling of kinds at different positions
        let kinds = [
            SyntaxKind::Text,
            SyntaxKind::At,
            SyntaxKind::IfKw,
            SyntaxKind::Ident,
            SyntaxKind::Root,
            SyntaxKind::IfBlock,
            SyntaxKind::ExprPlaceholder,
        ];
        for kind in kinds {
            let raw: rowan::SyntaxKind = kind.into();
            let back = TemplateLanguage::kind_from_raw(raw);
            assert_eq!(kind, back, "Round trip failed for {:?}", kind);
        }
    }

    // ==================== is_control_keyword Tests ====================

    #[test]
    fn test_is_control_keyword() {
        assert!(SyntaxKind::IfKw.is_control_keyword());
        assert!(SyntaxKind::ForKw.is_control_keyword());
        assert!(!SyntaxKind::LetKw.is_control_keyword());
        assert!(!SyntaxKind::Text.is_control_keyword());
    }

    #[test]
    fn test_all_control_keywords() {
        let control_keywords = [
            SyntaxKind::IfKw,
            SyntaxKind::ElseKw,
            SyntaxKind::ForKw,
            SyntaxKind::WhileKw,
            SyntaxKind::MatchKw,
            SyntaxKind::CaseKw,
        ];
        for kw in control_keywords {
            assert!(kw.is_control_keyword(), "{:?} should be a control keyword", kw);
        }
    }

    // ==================== is_directive_keyword Tests ====================

    #[test]
    fn test_is_directive_keyword() {
        assert!(SyntaxKind::LetKw.is_directive_keyword());
        assert!(SyntaxKind::DoKw.is_directive_keyword());
        assert!(SyntaxKind::TypeScriptKw.is_directive_keyword());
        assert!(SyntaxKind::MutKw.is_directive_keyword());
        assert!(!SyntaxKind::IfKw.is_directive_keyword());
        assert!(!SyntaxKind::Text.is_directive_keyword());
    }

    #[test]
    fn test_all_directive_keywords() {
        let directive_keywords = [
            SyntaxKind::LetKw,
            SyntaxKind::DoKw,
            SyntaxKind::TypeScriptKw,
            SyntaxKind::MutKw,
        ];
        for kw in directive_keywords {
            assert!(kw.is_directive_keyword(), "{:?} should be a directive keyword", kw);
        }
    }

    // ==================== is_ts_keyword Tests ====================

    #[test]
    fn test_is_ts_keyword() {
        assert!(SyntaxKind::AsKw.is_ts_keyword());
        assert!(SyntaxKind::FunctionKw.is_ts_keyword());
        assert!(SyntaxKind::ClassKw.is_ts_keyword());
        assert!(SyntaxKind::InterfaceKw.is_ts_keyword());
        assert!(!SyntaxKind::IfKw.is_ts_keyword());
        assert!(!SyntaxKind::ForKw.is_ts_keyword());
        assert!(!SyntaxKind::Text.is_ts_keyword());
    }

    #[test]
    fn test_ts_keyword_sample() {
        let ts_keywords = [
            SyntaxKind::AsKw,
            SyntaxKind::FunctionKw,
            SyntaxKind::ConstKw,
            SyntaxKind::VarKw,
            SyntaxKind::ClassKw,
            SyntaxKind::InterfaceKw,
            SyntaxKind::TypeKw,
            SyntaxKind::ExportKw,
            SyntaxKind::ImportKw,
            SyntaxKind::ReturnKw,
            SyntaxKind::NewKw,
            SyntaxKind::AsyncKw,
            SyntaxKind::AwaitKw,
        ];
        for kw in ts_keywords {
            assert!(kw.is_ts_keyword(), "{:?} should be a TS keyword", kw);
        }
    }

    // ==================== is_trivia Tests ====================

    #[test]
    fn test_is_trivia() {
        assert!(SyntaxKind::Whitespace.is_trivia());
        assert!(!SyntaxKind::Text.is_trivia());
        assert!(!SyntaxKind::Ident.is_trivia());
    }

    // ==================== starts_identifier_context Tests ====================

    #[test]
    fn test_starts_identifier_context() {
        assert!(SyntaxKind::FunctionKw.starts_identifier_context());
        assert!(SyntaxKind::ClassKw.starts_identifier_context());
        assert!(SyntaxKind::InterfaceKw.starts_identifier_context());
        assert!(SyntaxKind::TypeKw.starts_identifier_context());
        assert!(SyntaxKind::ConstKw.starts_identifier_context());
        assert!(SyntaxKind::LetKw.starts_identifier_context());
        assert!(SyntaxKind::VarKw.starts_identifier_context());
        assert!(!SyntaxKind::ReturnKw.starts_identifier_context());
        assert!(!SyntaxKind::Text.starts_identifier_context());
    }

    // ==================== starts_type_context Tests ====================

    #[test]
    fn test_starts_type_context() {
        assert!(SyntaxKind::AsKw.starts_type_context());
        assert!(SyntaxKind::KeyofKw.starts_type_context());
        assert!(SyntaxKind::TypeofKw.starts_type_context());
        assert!(SyntaxKind::ExtendsKw.starts_type_context());
        assert!(SyntaxKind::ImplementsKw.starts_type_context());
        assert!(SyntaxKind::SatisfiesKw.starts_type_context());
        assert!(SyntaxKind::InferKw.starts_type_context());
        assert!(SyntaxKind::IsKw.starts_type_context());
        assert!(!SyntaxKind::ReturnKw.starts_type_context());
        assert!(!SyntaxKind::FunctionKw.starts_type_context());
    }

    // ==================== starts_expression_context Tests ====================

    #[test]
    fn test_starts_expression_context() {
        assert!(SyntaxKind::ReturnKw.starts_expression_context());
        assert!(SyntaxKind::ThrowKw.starts_expression_context());
        assert!(SyntaxKind::YieldKw.starts_expression_context());
        assert!(SyntaxKind::AwaitKw.starts_expression_context());
        assert!(SyntaxKind::NewKw.starts_expression_context());
        assert!(!SyntaxKind::FunctionKw.starts_expression_context());
        assert!(!SyntaxKind::ClassKw.starts_expression_context());
        assert!(!SyntaxKind::AsKw.starts_expression_context());
    }

    // ==================== Debug and Clone Tests ====================

    #[test]
    fn test_syntax_kind_debug() {
        let kind = SyntaxKind::Text;
        let debug_str = format!("{:?}", kind);
        assert_eq!(debug_str, "Text");
    }

    #[test]
    fn test_syntax_kind_clone() {
        let kind = SyntaxKind::IfKw;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_syntax_kind_copy() {
        let kind = SyntaxKind::ForKw;
        let copied: SyntaxKind = kind;
        assert_eq!(kind, copied);
    }

    // ==================== PartialOrd and Ord Tests ====================

    #[test]
    fn test_syntax_kind_ordering() {
        // Text is 0, Whitespace is 1, etc.
        assert!(SyntaxKind::Text < SyntaxKind::Whitespace);
        assert!(SyntaxKind::Text < SyntaxKind::Root);
    }

    // ==================== Hash Tests ====================

    #[test]
    fn test_syntax_kind_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SyntaxKind::Text);
        set.insert(SyntaxKind::At);
        set.insert(SyntaxKind::IfKw);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&SyntaxKind::Text));
        assert!(!set.contains(&SyntaxKind::ForKw));
    }

    // ==================== TemplateLanguage Tests ====================

    #[test]
    fn test_template_language_kind_to_raw() {
        let kind = SyntaxKind::Interpolation;
        let raw = TemplateLanguage::kind_to_raw(kind);
        assert_eq!(raw.0, SyntaxKind::Interpolation as u16);
    }

    #[test]
    fn test_template_language_kind_from_raw() {
        let raw = rowan::SyntaxKind(SyntaxKind::Text as u16);
        let kind = TemplateLanguage::kind_from_raw(raw);
        assert_eq!(kind, SyntaxKind::Text);
    }

    #[test]
    fn test_template_language_roundtrip() {
        let original = SyntaxKind::IfKw;
        let raw = TemplateLanguage::kind_to_raw(original);
        let recovered = TemplateLanguage::kind_from_raw(raw);
        assert_eq!(original, recovered);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_first_token_is_token() {
        assert!(SyntaxKind::Text.is_token());
    }

    #[test]
    fn test_last_token_before_root() {
        // RustTokens is the last token before composite nodes
        assert!(SyntaxKind::RustTokens.is_token());
    }

    #[test]
    fn test_keyword_categories_are_mutually_exclusive() {
        // Control keywords should not be directive keywords
        let control_kws = [
            SyntaxKind::IfKw,
            SyntaxKind::ElseKw,
            SyntaxKind::ForKw,
            SyntaxKind::WhileKw,
            SyntaxKind::MatchKw,
            SyntaxKind::CaseKw,
        ];
        for kw in control_kws {
            assert!(kw.is_control_keyword());
            assert!(!kw.is_directive_keyword());
        }

        // Directive keywords should not be control keywords
        let directive_kws = [
            SyntaxKind::LetKw,
            SyntaxKind::DoKw,
            SyntaxKind::TypeScriptKw,
            SyntaxKind::MutKw,
        ];
        for kw in directive_kws {
            assert!(kw.is_directive_keyword());
            assert!(!kw.is_control_keyword());
        }
    }
}
