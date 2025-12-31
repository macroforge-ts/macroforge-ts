//! Intermediate Representation for the template language.
//!
//! The IR is produced by the parser and consumed by codegen to produce Rust code
//! that constructs SWC AST nodes directly.
//!
//! ## Architecture
//!
//! The IR represents TypeScript syntax structurally rather than as text fragments.
//! Each node type corresponds to a TypeScript or template construct:
//!
//! - **Declarations**: Function, Class, Interface, TypeAlias, VarDecl
//! - **Statements**: Block, Expr, Return, If (TS), Throw
//! - **Expressions**: Ident, Lit, Call, Member, Object, Array, Binary, etc.
//! - **Types**: TypeRef, Keyword, Union, Array, Function, Object types
//! - **Template Constructs**: Placeholder, If, For, While, Match, Let, Do
//!
//! Rust expressions in template constructs are stored as `TokenStream` to avoid
//! re-parsing and preserve span information.

use proc_macro2::TokenStream;

// =============================================================================
// Placeholder Classification
// =============================================================================

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

// =============================================================================
// Supporting Enums
// =============================================================================

/// Variable declaration kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarKind {
    Const,
    Let,
    Var,
}

/// Access modifier for class members.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,
    Private,
    Protected,
}

/// Method kind for class methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    Method,
    Getter,
    Setter,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Exp,      // **

    // Comparison
    EqEq,     // ==
    NotEq,    // !=
    EqEqEq,   // ===
    NotEqEq,  // !==
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=

    // Logical
    And,      // &&
    Or,       // ||
    NullishCoalesce, // ??

    // Bitwise
    BitAnd,   // &
    BitOr,    // |
    BitXor,   // ^
    Shl,      // <<
    Shr,      // >>
    UShr,     // >>>

    // Other
    In,       // in
    InstanceOf, // instanceof
}

/// Assignment operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,   // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    ExpAssign, // **=
    ShlAssign, // <<=
    ShrAssign, // >>=
    UShrAssign, // >>>=
    BitAndAssign, // &=
    BitOrAssign, // |=
    BitXorAssign, // ^=
    AndAssign, // &&=
    OrAssign,  // ||=
    NullishAssign, // ??=
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,    // -
    Plus,     // +
    Not,      // !
    BitNot,   // ~
    TypeOf,   // typeof
    Void,     // void
    Delete,   // delete
}

/// Update operators (++/--)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOp {
    Increment, // ++
    Decrement, // --
}

/// TypeScript keyword types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsKeyword {
    Any,
    Unknown,
    String,
    Number,
    Boolean,
    Void,
    Null,
    Undefined,
    Never,
    Object,
    BigInt,
    Symbol,
}

// =============================================================================
// IR Node Definition
// =============================================================================

/// A node in the IR representing TypeScript syntax or template constructs.
#[derive(Debug, Clone)]
pub enum IrNode {
    // =========================================================================
    // Declarations
    // =========================================================================

    /// Function declaration: `function name(params): RetType { body }`
    FnDecl {
        exported: bool,
        declare: bool,
        async_: bool,
        generator: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Option<Box<IrNode>>,
    },

    /// Class declaration: `class Name extends Base implements I { body }`
    ClassDecl {
        exported: bool,
        declare: bool,
        abstract_: bool,
        decorators: Vec<IrNode>,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        extends: Option<Box<IrNode>>,
        implements: Vec<IrNode>,
        body: Vec<IrNode>,
    },

    /// Interface declaration: `interface Name extends I { body }`
    InterfaceDecl {
        exported: bool,
        declare: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        extends: Vec<IrNode>,
        body: Vec<IrNode>,
    },

    /// Type alias: `type Name<T> = Type`
    TypeAliasDecl {
        exported: bool,
        declare: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        type_ann: Box<IrNode>,
    },

    /// Variable declaration: `const/let/var name: Type = init`
    VarDecl {
        exported: bool,
        declare: bool,
        kind: VarKind,
        decls: Vec<VarDeclarator>,
    },

    /// Enum declaration: `enum Name { ... }`
    EnumDecl {
        exported: bool,
        declare: bool,
        const_: bool,
        name: Box<IrNode>,
        members: Vec<IrNode>,
    },

    // =========================================================================
    // Class Members
    // =========================================================================

    /// Constructor: `constructor(params) { body }`
    Constructor {
        accessibility: Option<Accessibility>,
        params: Vec<IrNode>,
        body: Option<Box<IrNode>>,
    },

    /// Method: `name(params): Type { body }`
    Method {
        static_: bool,
        accessibility: Option<Accessibility>,
        readonly: bool,
        async_: bool,
        generator: bool,
        kind: MethodKind,
        name: Box<IrNode>,
        optional: bool,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Option<Box<IrNode>>,
    },

    /// Class property: `name: Type = value`
    ClassProp {
        static_: bool,
        accessibility: Option<Accessibility>,
        readonly: bool,
        declare: bool,
        optional: bool,
        definite: bool,
        name: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
        value: Option<Box<IrNode>>,
    },

    /// Index signature: `[key: K]: V`
    IndexSignature {
        readonly: bool,
        params: Vec<IrNode>,
        type_ann: Box<IrNode>,
    },

    // =========================================================================
    // Interface Members
    // =========================================================================

    /// Property signature: `name?: Type`
    PropSignature {
        readonly: bool,
        name: Box<IrNode>,
        optional: bool,
        type_ann: Option<Box<IrNode>>,
    },

    /// Method signature: `name(params): Type`
    MethodSignature {
        name: Box<IrNode>,
        optional: bool,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
    },

    // =========================================================================
    // Statements
    // =========================================================================

    /// Block statement: `{ stmts }`
    BlockStmt {
        stmts: Vec<IrNode>,
    },

    /// Expression statement: `expr;`
    ExprStmt {
        expr: Box<IrNode>,
    },

    /// Return statement: `return expr;`
    ReturnStmt {
        arg: Option<Box<IrNode>>,
    },

    /// Throw statement: `throw expr;`
    ThrowStmt {
        arg: Box<IrNode>,
    },

    /// TypeScript if statement: `if (test) { cons } else { alt }`
    TsIfStmt {
        test: Box<IrNode>,
        cons: Box<IrNode>,
        alt: Option<Box<IrNode>>,
    },

    /// TypeScript for/while loop (parsed as raw text with placeholders)
    TsLoopStmt {
        parts: Vec<IrNode>,
    },

    /// For-in statement: `for (left in right) body`
    ForInStmt {
        left: Box<IrNode>,
        right: Box<IrNode>,
        body: Box<IrNode>,
    },

    /// For-of statement: `for (left of right) body`
    ForOfStmt {
        await_: bool,
        left: Box<IrNode>,
        right: Box<IrNode>,
        body: Box<IrNode>,
    },

    /// Empty statement: `;`
    EmptyStmt,

    // =========================================================================
    // Expressions
    // =========================================================================

    /// Identifier: `foo`
    Ident(String),

    /// Private name: `#foo`
    PrivateName(String),

    /// String literal: `"hello"`
    StrLit(String),

    /// Number literal: `42`, `3.14`
    NumLit(String),

    /// BigInt literal: `42n`
    BigIntLit(String),

    /// Boolean literal: `true`, `false`
    BoolLit(bool),

    /// Null literal: `null`
    NullLit,

    /// This expression: `this`
    ThisExpr,

    /// Super expression: `super`
    SuperExpr,

    /// Call expression: `callee(args)`
    CallExpr {
        callee: Box<IrNode>,
        type_args: Option<Box<IrNode>>,
        args: Vec<IrNode>,
    },

    /// New expression: `new Callee(args)`
    NewExpr {
        callee: Box<IrNode>,
        type_args: Option<Box<IrNode>>,
        args: Vec<IrNode>,
    },

    /// Member expression: `obj.prop` or `obj[prop]`
    MemberExpr {
        obj: Box<IrNode>,
        prop: Box<IrNode>,
        computed: bool,
    },

    /// Optional chain expression: `obj?.prop` or `obj?.[prop]` or `fn?.()`
    OptChainExpr {
        base: Box<IrNode>,
    },

    /// Object literal: `{ props }`
    ObjectLit {
        props: Vec<IrNode>,
    },

    /// Array literal: `[elems]`
    ArrayLit {
        elems: Vec<IrNode>,
    },

    /// Arrow function: `(params) => body`
    ArrowExpr {
        async_: bool,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Box<IrNode>, // BlockStmt or Expr
    },

    /// Function expression: `function(params) { body }`
    FnExpr {
        async_: bool,
        generator: bool,
        name: Option<Box<IrNode>>,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Option<Box<IrNode>>,
    },

    /// Class expression: `class { body }`
    ClassExpr {
        name: Option<Box<IrNode>>,
        type_params: Option<Box<IrNode>>,
        extends: Option<Box<IrNode>>,
        implements: Vec<IrNode>,
        body: Vec<IrNode>,
    },

    /// Binary expression: `left op right`
    BinExpr {
        left: Box<IrNode>,
        op: BinaryOp,
        right: Box<IrNode>,
    },

    /// Assignment expression: `left = right`
    AssignExpr {
        left: Box<IrNode>,
        op: AssignOp,
        right: Box<IrNode>,
    },

    /// Unary expression: `op arg`
    UnaryExpr {
        op: UnaryOp,
        arg: Box<IrNode>,
    },

    /// Update expression: `++x` or `x++`
    UpdateExpr {
        op: UpdateOp,
        prefix: bool,
        arg: Box<IrNode>,
    },

    /// Conditional expression: `test ? cons : alt`
    CondExpr {
        test: Box<IrNode>,
        cons: Box<IrNode>,
        alt: Box<IrNode>,
    },

    /// Sequence expression: `a, b, c`
    SeqExpr {
        exprs: Vec<IrNode>,
    },

    /// Template literal: `` `hello ${expr}` ``
    TplLit {
        quasis: Vec<String>,
        exprs: Vec<IrNode>,
    },

    /// Tagged template: `tag`hello ${expr}``
    TaggedTpl {
        tag: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        quasis: Vec<String>,
        exprs: Vec<IrNode>,
    },

    /// Parenthesized expression: `(expr)`
    ParenExpr {
        expr: Box<IrNode>,
    },

    /// Await expression: `await expr`
    AwaitExpr {
        arg: Box<IrNode>,
    },

    /// Yield expression: `yield expr` or `yield* expr`
    YieldExpr {
        delegate: bool,
        arg: Option<Box<IrNode>>,
    },

    /// Type assertion: `expr as Type`
    TsAsExpr {
        expr: Box<IrNode>,
        type_ann: Box<IrNode>,
    },

    /// Satisfies expression: `expr satisfies Type`
    TsSatisfiesExpr {
        expr: Box<IrNode>,
        type_ann: Box<IrNode>,
    },

    /// Non-null assertion: `expr!`
    TsNonNullExpr {
        expr: Box<IrNode>,
    },

    /// Type instantiation: `expr<Type>`
    TsInstantiation {
        expr: Box<IrNode>,
        type_args: Box<IrNode>,
    },

    // =========================================================================
    // Object Properties
    // =========================================================================

    /// Key-value property: `key: value`
    KeyValueProp {
        key: Box<IrNode>,
        value: Box<IrNode>,
    },

    /// Shorthand property: `key` (same as `key: key`)
    ShorthandProp {
        key: Box<IrNode>,
    },

    /// Spread property: `...expr`
    SpreadElement {
        expr: Box<IrNode>,
    },

    /// Computed property name: `[expr]`
    ComputedPropName {
        expr: Box<IrNode>,
    },

    /// Method property: `name() { }`
    MethodProp {
        async_: bool,
        generator: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Box<IrNode>,
    },

    /// Getter property: `get name() { }`
    GetterProp {
        name: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
        body: Box<IrNode>,
    },

    /// Setter property: `set name(param) { }`
    SetterProp {
        name: Box<IrNode>,
        param: Box<IrNode>,
        body: Box<IrNode>,
    },

    // =========================================================================
    // Parameters
    // =========================================================================

    /// Parameter: `name: Type = default`
    Param {
        decorators: Vec<IrNode>,
        pat: Box<IrNode>,
    },

    /// Binding identifier: `name`
    BindingIdent {
        name: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Rest pattern: `...rest`
    RestPat {
        arg: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
    },

    /// Assignment pattern: `name = default`
    AssignPat {
        left: Box<IrNode>,
        right: Box<IrNode>,
    },

    /// Array pattern: `[a, b, ...rest]`
    ArrayPat {
        elems: Vec<Option<IrNode>>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Object pattern: `{ a, b: c }`
    ObjectPat {
        props: Vec<IrNode>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Object pattern property: `a` or `a: b` or `a = default`
    ObjectPatProp {
        key: Box<IrNode>,
        value: Option<Box<IrNode>>,
    },

    // =========================================================================
    // Type Nodes
    // =========================================================================

    /// Type annotation wrapper: `: Type`
    TypeAnnotation {
        type_ann: Box<IrNode>,
    },

    /// Type reference: `TypeName<Args>`
    TypeRef {
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
    },

    /// Qualified type name: `NS.Type`
    QualifiedName {
        left: Box<IrNode>,
        right: Box<IrNode>,
    },

    /// Type parameters: `<T, U extends V>`
    TypeParams {
        params: Vec<IrNode>,
    },

    /// Type parameter: `T extends Constraint = Default`
    TypeParam {
        name: String,
        constraint: Option<Box<IrNode>>,
        default: Option<Box<IrNode>>,
    },

    /// Type arguments: `<T, U>`
    TypeArgs {
        args: Vec<IrNode>,
    },

    /// Keyword type: `string`, `number`, etc.
    KeywordType(TsKeyword),

    /// Literal type: `"hello"`, `42`, `true`
    LiteralType {
        lit: Box<IrNode>,
    },

    /// Union type: `A | B`
    UnionType {
        types: Vec<IrNode>,
    },

    /// Intersection type: `A & B`
    IntersectionType {
        types: Vec<IrNode>,
    },

    /// Array type: `T[]`
    ArrayType {
        elem: Box<IrNode>,
    },

    /// Tuple type: `[A, B, C]`
    TupleType {
        elems: Vec<IrNode>,
    },

    /// Optional type element: `T?` (in tuple)
    OptionalType {
        type_ann: Box<IrNode>,
    },

    /// Rest type element: `...T` (in tuple)
    RestType {
        type_ann: Box<IrNode>,
    },

    /// Function type: `(params) => RetType`
    FnType {
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Box<IrNode>,
    },

    /// Constructor type: `new (params) => Type`
    ConstructorType {
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Box<IrNode>,
    },

    /// Object type / type literal: `{ prop: Type }`
    ObjectType {
        members: Vec<IrNode>,
    },

    /// Parenthesized type: `(Type)`
    ParenType {
        type_ann: Box<IrNode>,
    },

    /// Typeof type: `typeof expr`
    TypeofType {
        expr: Box<IrNode>,
    },

    /// Keyof type: `keyof Type`
    KeyofType {
        type_ann: Box<IrNode>,
    },

    /// Indexed access type: `T[K]`
    IndexedAccessType {
        obj: Box<IrNode>,
        index: Box<IrNode>,
    },

    /// Conditional type: `T extends U ? X : Y`
    ConditionalType {
        check: Box<IrNode>,
        extends: Box<IrNode>,
        true_type: Box<IrNode>,
        false_type: Box<IrNode>,
    },

    /// Infer type: `infer T`
    InferType {
        type_param: Box<IrNode>,
    },

    /// Mapped type: `{ [K in Keys]: Type }`
    MappedType {
        readonly: Option<bool>, // true = +readonly, false = -readonly, None = no modifier
        type_param: Box<IrNode>,
        name_type: Option<Box<IrNode>>,
        optional: Option<bool>,
        type_ann: Option<Box<IrNode>>,
    },

    /// Import type: `import("module").Type`
    ImportType {
        arg: Box<IrNode>,
        qualifier: Option<Box<IrNode>>,
        type_args: Option<Box<IrNode>>,
    },

    /// This type: `this`
    ThisType,

    // =========================================================================
    // Template Constructs (Rust Integration)
    // =========================================================================

    /// Placeholder: `@{expr}`
    Placeholder {
        kind: PlaceholderKind,
        expr: TokenStream,
    },

    /// Template if/else-if/else: `{#if cond}...{:else}...{/if}`
    If {
        condition: TokenStream,
        then_body: Vec<IrNode>,
        else_if_branches: Vec<(TokenStream, Vec<IrNode>)>,
        else_body: Option<Vec<IrNode>>,
    },

    /// Template for loop: `{#for pat in iter}...{/for}`
    For {
        pattern: TokenStream,
        iterator: TokenStream,
        body: Vec<IrNode>,
    },

    /// Template while loop: `{#while cond}...{/while}`
    While {
        condition: TokenStream,
        body: Vec<IrNode>,
    },

    /// Template match: `{#match expr}{:case pat}...{/match}`
    Match {
        expr: TokenStream,
        arms: Vec<MatchArm>,
    },

    /// Let binding: `{$let name = expr}`
    Let {
        pattern: TokenStream,
        mutable: bool,
        type_hint: Option<TokenStream>,
        value: TokenStream,
    },

    /// Do directive: `{$do code}`
    Do {
        code: TokenStream,
    },

    /// TypeScript injection: `{$typescript stream}`
    TypeScript {
        stream: TokenStream,
    },

    // =========================================================================
    // Comments
    // =========================================================================

    /// Line comment: `{> text <}` -> `//`
    LineComment {
        text: String,
    },

    /// Block comment: `{>> text <<}` -> `/* */`
    BlockComment {
        text: String,
    },

    /// Doc comment: `/** text */`
    DocComment {
        text: String,
    },

    /// A node with an attached doc comment.
    Documented {
        doc: String,
        inner: Box<IrNode>,
    },

    // =========================================================================
    // Special Constructs
    // =========================================================================

    /// Identifier block: `{|prefix_@{name}_suffix|}`
    IdentBlock {
        parts: Vec<IrNode>,
    },

    /// String interpolation: `"text @{expr} more"`
    StringInterp {
        quote: char,
        parts: Vec<IrNode>,
    },

    /// Raw text (fallback for unparseable content - should be minimized)
    Raw(String),

    /// Enum member: `Name = value`
    EnumMember {
        name: Box<IrNode>,
        init: Option<Box<IrNode>>,
    },

    /// Decorator: `@expr`
    Decorator {
        expr: Box<IrNode>,
    },

    /// Import declaration
    ImportDecl {
        type_only: bool,
        specifiers: Vec<IrNode>,
        src: String,
    },

    /// Named import: `{ a, b as c }`
    NamedImport {
        local: Box<IrNode>,
        imported: Option<Box<IrNode>>,
    },

    /// Default import: `foo`
    DefaultImport {
        local: Box<IrNode>,
    },

    /// Namespace import: `* as foo`
    NamespaceImport {
        local: Box<IrNode>,
    },

    /// Export declaration wrapper
    ExportDecl {
        decl: Box<IrNode>,
    },

    /// Named export: `export { a, b as c }`
    NamedExport {
        specifiers: Vec<IrNode>,
        src: Option<String>,
        type_only: bool,
    },

    /// Export specifier: `a as b`
    ExportSpecifier {
        local: Box<IrNode>,
        exported: Option<Box<IrNode>>,
    },

    /// Default export: `export default expr`
    ExportDefaultExpr {
        expr: Box<IrNode>,
    },

    /// Re-export all: `export * from "module"`
    ExportAll {
        src: String,
        type_only: bool,
    },
}

// =============================================================================
// Supporting Structures
// =============================================================================

/// Variable declarator: `name: Type = init`
#[derive(Debug, Clone)]
pub struct VarDeclarator {
    pub name: Box<IrNode>,
    pub type_ann: Option<Box<IrNode>>,
    pub init: Option<Box<IrNode>>,
    pub definite: bool,
}

/// Match arm for template match construct.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: TokenStream,
    pub guard: Option<TokenStream>,
    pub body: Vec<IrNode>,
}

// =============================================================================
// IR Container
// =============================================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PlaceholderKind Tests ====================

    #[test]
    fn test_placeholder_kind_debug() {
        let kind = PlaceholderKind::Expr;
        assert_eq!(format!("{:?}", kind), "Expr");
    }

    #[test]
    fn test_placeholder_kind_clone() {
        let kind = PlaceholderKind::Type;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_placeholder_kind_equality() {
        assert_eq!(PlaceholderKind::Expr, PlaceholderKind::Expr);
        assert_eq!(PlaceholderKind::Type, PlaceholderKind::Type);
        assert_eq!(PlaceholderKind::Ident, PlaceholderKind::Ident);
        assert_eq!(PlaceholderKind::Stmt, PlaceholderKind::Stmt);
        assert_ne!(PlaceholderKind::Expr, PlaceholderKind::Type);
    }

    #[test]
    fn test_placeholder_kind_copy() {
        let kind = PlaceholderKind::Stmt;
        let copied: PlaceholderKind = kind; // Copy
        assert_eq!(kind, copied);
    }

    // ==================== VarKind Tests ====================

    #[test]
    fn test_var_kind_debug() {
        assert_eq!(format!("{:?}", VarKind::Const), "Const");
        assert_eq!(format!("{:?}", VarKind::Let), "Let");
        assert_eq!(format!("{:?}", VarKind::Var), "Var");
    }

    #[test]
    fn test_var_kind_equality() {
        assert_eq!(VarKind::Const, VarKind::Const);
        assert_ne!(VarKind::Const, VarKind::Let);
        assert_ne!(VarKind::Let, VarKind::Var);
    }

    // ==================== Accessibility Tests ====================

    #[test]
    fn test_accessibility_debug() {
        assert_eq!(format!("{:?}", Accessibility::Public), "Public");
        assert_eq!(format!("{:?}", Accessibility::Private), "Private");
        assert_eq!(format!("{:?}", Accessibility::Protected), "Protected");
    }

    #[test]
    fn test_accessibility_equality() {
        assert_eq!(Accessibility::Public, Accessibility::Public);
        assert_ne!(Accessibility::Public, Accessibility::Private);
    }

    // ==================== MethodKind Tests ====================

    #[test]
    fn test_method_kind_debug() {
        assert_eq!(format!("{:?}", MethodKind::Method), "Method");
        assert_eq!(format!("{:?}", MethodKind::Getter), "Getter");
        assert_eq!(format!("{:?}", MethodKind::Setter), "Setter");
    }

    #[test]
    fn test_method_kind_equality() {
        assert_eq!(MethodKind::Getter, MethodKind::Getter);
        assert_ne!(MethodKind::Getter, MethodKind::Setter);
    }

    // ==================== BinaryOp Tests ====================

    #[test]
    fn test_binary_op_debug() {
        assert_eq!(format!("{:?}", BinaryOp::Add), "Add");
        assert_eq!(format!("{:?}", BinaryOp::Sub), "Sub");
        assert_eq!(format!("{:?}", BinaryOp::EqEqEq), "EqEqEq");
    }

    #[test]
    fn test_binary_op_equality() {
        assert_eq!(BinaryOp::Add, BinaryOp::Add);
        assert_ne!(BinaryOp::Add, BinaryOp::Sub);
    }

    #[test]
    fn test_binary_op_all_variants() {
        // Test that all variants exist and are distinct
        let ops = vec![
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
        assert_eq!(ops.len(), 25);
    }

    // ==================== AssignOp Tests ====================

    #[test]
    fn test_assign_op_debug() {
        assert_eq!(format!("{:?}", AssignOp::Assign), "Assign");
        assert_eq!(format!("{:?}", AssignOp::AddAssign), "AddAssign");
    }

    #[test]
    fn test_assign_op_equality() {
        assert_eq!(AssignOp::Assign, AssignOp::Assign);
        assert_ne!(AssignOp::Assign, AssignOp::AddAssign);
    }

    // ==================== UnaryOp Tests ====================

    #[test]
    fn test_unary_op_debug() {
        assert_eq!(format!("{:?}", UnaryOp::Minus), "Minus");
        assert_eq!(format!("{:?}", UnaryOp::Not), "Not");
        assert_eq!(format!("{:?}", UnaryOp::TypeOf), "TypeOf");
    }

    #[test]
    fn test_unary_op_equality() {
        assert_eq!(UnaryOp::Minus, UnaryOp::Minus);
        assert_ne!(UnaryOp::Minus, UnaryOp::Plus);
    }

    // ==================== UpdateOp Tests ====================

    #[test]
    fn test_update_op_debug() {
        assert_eq!(format!("{:?}", UpdateOp::Increment), "Increment");
        assert_eq!(format!("{:?}", UpdateOp::Decrement), "Decrement");
    }

    #[test]
    fn test_update_op_equality() {
        assert_eq!(UpdateOp::Increment, UpdateOp::Increment);
        assert_ne!(UpdateOp::Increment, UpdateOp::Decrement);
    }

    // ==================== TsKeyword Tests ====================

    #[test]
    fn test_ts_keyword_debug() {
        assert_eq!(format!("{:?}", TsKeyword::String), "String");
        assert_eq!(format!("{:?}", TsKeyword::Number), "Number");
        assert_eq!(format!("{:?}", TsKeyword::Boolean), "Boolean");
    }

    #[test]
    fn test_ts_keyword_all_variants() {
        let keywords = vec![
            TsKeyword::Any,
            TsKeyword::Unknown,
            TsKeyword::String,
            TsKeyword::Number,
            TsKeyword::Boolean,
            TsKeyword::Void,
            TsKeyword::Null,
            TsKeyword::Undefined,
            TsKeyword::Never,
            TsKeyword::Object,
            TsKeyword::BigInt,
            TsKeyword::Symbol,
        ];
        assert_eq!(keywords.len(), 12);
    }

    // ==================== IrNode Tests ====================

    #[test]
    fn test_ir_node_ident() {
        let node = IrNode::Ident("foo".to_string());
        assert!(matches!(node, IrNode::Ident(s) if s == "foo"));
    }

    #[test]
    fn test_ir_node_str_lit() {
        let node = IrNode::StrLit("hello".to_string());
        assert!(matches!(node, IrNode::StrLit(s) if s == "hello"));
    }

    #[test]
    fn test_ir_node_num_lit() {
        let node = IrNode::NumLit("42".to_string());
        assert!(matches!(node, IrNode::NumLit(s) if s == "42"));
    }

    #[test]
    fn test_ir_node_bool_lit() {
        let true_node = IrNode::BoolLit(true);
        let false_node = IrNode::BoolLit(false);
        assert!(matches!(true_node, IrNode::BoolLit(true)));
        assert!(matches!(false_node, IrNode::BoolLit(false)));
    }

    #[test]
    fn test_ir_node_null_lit() {
        let node = IrNode::NullLit;
        assert!(matches!(node, IrNode::NullLit));
    }

    #[test]
    fn test_ir_node_this_expr() {
        let node = IrNode::ThisExpr;
        assert!(matches!(node, IrNode::ThisExpr));
    }

    #[test]
    fn test_ir_node_super_expr() {
        let node = IrNode::SuperExpr;
        assert!(matches!(node, IrNode::SuperExpr));
    }

    #[test]
    fn test_ir_node_empty_stmt() {
        let node = IrNode::EmptyStmt;
        assert!(matches!(node, IrNode::EmptyStmt));
    }

    #[test]
    fn test_ir_node_block_stmt() {
        let node = IrNode::BlockStmt { stmts: vec![] };
        assert!(matches!(node, IrNode::BlockStmt { stmts } if stmts.is_empty()));
    }

    #[test]
    fn test_ir_node_expr_stmt() {
        let node = IrNode::ExprStmt {
            expr: Box::new(IrNode::Ident("x".to_string())),
        };
        assert!(matches!(node, IrNode::ExprStmt { .. }));
    }

    #[test]
    fn test_ir_node_return_stmt_with_arg() {
        let node = IrNode::ReturnStmt {
            arg: Some(Box::new(IrNode::NumLit("42".to_string()))),
        };
        assert!(matches!(node, IrNode::ReturnStmt { arg: Some(_) }));
    }

    #[test]
    fn test_ir_node_return_stmt_without_arg() {
        let node = IrNode::ReturnStmt { arg: None };
        assert!(matches!(node, IrNode::ReturnStmt { arg: None }));
    }

    #[test]
    fn test_ir_node_throw_stmt() {
        let node = IrNode::ThrowStmt {
            arg: Box::new(IrNode::StrLit("error".to_string())),
        };
        assert!(matches!(node, IrNode::ThrowStmt { .. }));
    }

    #[test]
    fn test_ir_node_bin_expr() {
        let node = IrNode::BinExpr {
            left: Box::new(IrNode::NumLit("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(IrNode::NumLit("2".to_string())),
        };
        assert!(matches!(node, IrNode::BinExpr { op: BinaryOp::Add, .. }));
    }

    #[test]
    fn test_ir_node_unary_expr() {
        let node = IrNode::UnaryExpr {
            op: UnaryOp::Minus,
            arg: Box::new(IrNode::NumLit("5".to_string())),
        };
        assert!(matches!(node, IrNode::UnaryExpr { op: UnaryOp::Minus, .. }));
    }

    #[test]
    fn test_ir_node_call_expr() {
        let node = IrNode::CallExpr {
            callee: Box::new(IrNode::Ident("foo".to_string())),
            type_args: None,
            args: vec![IrNode::NumLit("1".to_string())],
        };
        assert!(matches!(node, IrNode::CallExpr { .. }));
    }

    #[test]
    fn test_ir_node_new_expr() {
        let node = IrNode::NewExpr {
            callee: Box::new(IrNode::Ident("MyClass".to_string())),
            type_args: None,
            args: vec![],
        };
        assert!(matches!(node, IrNode::NewExpr { .. }));
    }

    #[test]
    fn test_ir_node_member_expr() {
        let node = IrNode::MemberExpr {
            obj: Box::new(IrNode::Ident("obj".to_string())),
            prop: Box::new(IrNode::Ident("prop".to_string())),
            computed: false,
        };
        assert!(matches!(node, IrNode::MemberExpr { computed: false, .. }));
    }

    #[test]
    fn test_ir_node_object_lit() {
        let node = IrNode::ObjectLit { props: vec![] };
        assert!(matches!(node, IrNode::ObjectLit { props } if props.is_empty()));
    }

    #[test]
    fn test_ir_node_array_lit() {
        let node = IrNode::ArrayLit {
            elems: vec![IrNode::NumLit("1".to_string())],
        };
        assert!(matches!(node, IrNode::ArrayLit { elems } if elems.len() == 1));
    }

    #[test]
    fn test_ir_node_arrow_expr() {
        let node = IrNode::ArrowExpr {
            async_: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: Box::new(IrNode::NumLit("42".to_string())),
        };
        assert!(matches!(node, IrNode::ArrowExpr { async_: false, .. }));
    }

    #[test]
    fn test_ir_node_cond_expr() {
        let node = IrNode::CondExpr {
            test: Box::new(IrNode::BoolLit(true)),
            cons: Box::new(IrNode::NumLit("1".to_string())),
            alt: Box::new(IrNode::NumLit("2".to_string())),
        };
        assert!(matches!(node, IrNode::CondExpr { .. }));
    }

    #[test]
    fn test_ir_node_keyword_type() {
        let node = IrNode::KeywordType(TsKeyword::String);
        assert!(matches!(node, IrNode::KeywordType(TsKeyword::String)));
    }

    #[test]
    fn test_ir_node_union_type() {
        let node = IrNode::UnionType {
            types: vec![
                IrNode::KeywordType(TsKeyword::String),
                IrNode::KeywordType(TsKeyword::Number),
            ],
        };
        assert!(matches!(node, IrNode::UnionType { types } if types.len() == 2));
    }

    #[test]
    fn test_ir_node_array_type() {
        let node = IrNode::ArrayType {
            elem: Box::new(IrNode::KeywordType(TsKeyword::Number)),
        };
        assert!(matches!(node, IrNode::ArrayType { .. }));
    }

    #[test]
    fn test_ir_node_var_decl() {
        let node = IrNode::VarDecl {
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![],
        };
        assert!(matches!(node, IrNode::VarDecl { kind: VarKind::Const, .. }));
    }

    #[test]
    fn test_ir_node_fn_decl() {
        let node = IrNode::FnDecl {
            exported: false,
            declare: false,
            async_: true,
            generator: false,
            name: Box::new(IrNode::Ident("myFn".to_string())),
            type_params: None,
            params: vec![],
            return_type: None,
            body: Some(Box::new(IrNode::BlockStmt { stmts: vec![] })),
        };
        assert!(matches!(node, IrNode::FnDecl { async_: true, .. }));
    }

    #[test]
    fn test_ir_node_class_decl() {
        let node = IrNode::ClassDecl {
            exported: false,
            declare: false,
            abstract_: false,
            decorators: vec![],
            name: Box::new(IrNode::Ident("MyClass".to_string())),
            type_params: None,
            extends: None,
            implements: vec![],
            body: vec![],
        };
        assert!(matches!(node, IrNode::ClassDecl { abstract_: false, .. }));
    }

    #[test]
    fn test_ir_node_interface_decl() {
        let node = IrNode::InterfaceDecl {
            exported: true,
            declare: false,
            name: Box::new(IrNode::Ident("MyInterface".to_string())),
            type_params: None,
            extends: vec![],
            body: vec![],
        };
        assert!(matches!(node, IrNode::InterfaceDecl { exported: true, .. }));
    }

    #[test]
    fn test_ir_node_type_alias_decl() {
        let node = IrNode::TypeAliasDecl {
            exported: false,
            declare: false,
            name: Box::new(IrNode::Ident("MyType".to_string())),
            type_params: None,
            type_ann: Box::new(IrNode::KeywordType(TsKeyword::String)),
        };
        assert!(matches!(node, IrNode::TypeAliasDecl { .. }));
    }

    #[test]
    fn test_ir_node_clone() {
        let node = IrNode::Ident("test".to_string());
        let cloned = node.clone();
        assert!(matches!(cloned, IrNode::Ident(s) if s == "test"));
    }

    // ==================== VarDeclarator Tests ====================

    #[test]
    fn test_var_declarator_basic() {
        let decl = VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            init: None,
            definite: false,
        };
        assert!(!decl.definite);
        assert!(decl.init.is_none());
        assert!(decl.type_ann.is_none());
    }

    #[test]
    fn test_var_declarator_with_init() {
        let decl = VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            init: Some(Box::new(IrNode::NumLit("42".to_string()))),
            definite: false,
        };
        assert!(decl.init.is_some());
    }

    #[test]
    fn test_var_declarator_with_type() {
        let decl = VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: Some(Box::new(IrNode::KeywordType(TsKeyword::Number))),
            init: None,
            definite: true,
        };
        assert!(decl.type_ann.is_some());
        assert!(decl.definite);
    }

    #[test]
    fn test_var_declarator_clone() {
        let decl = VarDeclarator {
            name: Box::new(IrNode::Ident("x".to_string())),
            type_ann: None,
            init: None,
            definite: false,
        };
        let cloned = decl.clone();
        assert!(!cloned.definite);
    }

    // ==================== MatchArm Tests ====================

    #[test]
    fn test_match_arm_basic() {
        let arm = MatchArm {
            pattern: TokenStream::new(),
            guard: None,
            body: vec![],
        };
        assert!(arm.guard.is_none());
        assert!(arm.body.is_empty());
    }

    #[test]
    fn test_match_arm_with_guard() {
        let arm = MatchArm {
            pattern: TokenStream::new(),
            guard: Some(TokenStream::new()),
            body: vec![IrNode::ReturnStmt { arg: None }],
        };
        assert!(arm.guard.is_some());
        assert_eq!(arm.body.len(), 1);
    }

    #[test]
    fn test_match_arm_clone() {
        let arm = MatchArm {
            pattern: TokenStream::new(),
            guard: None,
            body: vec![],
        };
        let cloned = arm.clone();
        assert!(cloned.guard.is_none());
    }

    // ==================== Ir Tests ====================

    #[test]
    fn test_ir_new() {
        let ir = Ir::new();
        assert!(ir.nodes.is_empty());
    }

    #[test]
    fn test_ir_default() {
        let ir = Ir::default();
        assert!(ir.nodes.is_empty());
    }

    #[test]
    fn test_ir_with_nodes() {
        let nodes = vec![
            IrNode::Ident("foo".to_string()),
            IrNode::NumLit("42".to_string()),
        ];
        let ir = Ir::with_nodes(nodes);
        assert_eq!(ir.nodes.len(), 2);
    }

    #[test]
    fn test_ir_with_empty_nodes() {
        let ir = Ir::with_nodes(vec![]);
        assert!(ir.nodes.is_empty());
    }

    #[test]
    fn test_ir_debug() {
        let ir = Ir::new();
        let debug_str = format!("{:?}", ir);
        assert!(debug_str.contains("Ir"));
        assert!(debug_str.contains("nodes"));
    }

    // ==================== Complex Node Structure Tests ====================

    #[test]
    fn test_nested_member_expr() {
        // obj.foo.bar
        let node = IrNode::MemberExpr {
            obj: Box::new(IrNode::MemberExpr {
                obj: Box::new(IrNode::Ident("obj".to_string())),
                prop: Box::new(IrNode::Ident("foo".to_string())),
                computed: false,
            }),
            prop: Box::new(IrNode::Ident("bar".to_string())),
            computed: false,
        };
        assert!(matches!(node, IrNode::MemberExpr { .. }));
    }

    #[test]
    fn test_call_with_args() {
        // foo(1, "hello", true)
        let node = IrNode::CallExpr {
            callee: Box::new(IrNode::Ident("foo".to_string())),
            type_args: None,
            args: vec![
                IrNode::NumLit("1".to_string()),
                IrNode::StrLit("hello".to_string()),
                IrNode::BoolLit(true),
            ],
        };
        if let IrNode::CallExpr { args, .. } = node {
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected CallExpr");
        }
    }

    #[test]
    fn test_class_with_members() {
        let node = IrNode::ClassDecl {
            exported: true,
            declare: false,
            abstract_: false,
            decorators: vec![],
            name: Box::new(IrNode::Ident("MyClass".to_string())),
            type_params: None,
            extends: Some(Box::new(IrNode::Ident("BaseClass".to_string()))),
            implements: vec![IrNode::Ident("Interface1".to_string())],
            body: vec![
                IrNode::Constructor {
                    accessibility: Some(Accessibility::Public),
                    params: vec![],
                    body: Some(Box::new(IrNode::BlockStmt { stmts: vec![] })),
                },
                IrNode::ClassProp {
                    static_: false,
                    accessibility: Some(Accessibility::Private),
                    readonly: true,
                    declare: false,
                    optional: false,
                    definite: false,
                    name: Box::new(IrNode::Ident("value".to_string())),
                    type_ann: Some(Box::new(IrNode::KeywordType(TsKeyword::Number))),
                    value: None,
                },
            ],
        };
        if let IrNode::ClassDecl { body, .. } = node {
            assert_eq!(body.len(), 2);
        } else {
            panic!("Expected ClassDecl");
        }
    }

    #[test]
    fn test_type_ref_with_params() {
        // Promise<string>
        let node = IrNode::TypeRef {
            name: Box::new(IrNode::Ident("Promise".to_string())),
            type_params: Some(Box::new(IrNode::TypeArgs {
                args: vec![IrNode::KeywordType(TsKeyword::String)],
            })),
        };
        assert!(matches!(node, IrNode::TypeRef { .. }));
    }

    #[test]
    fn test_fn_type() {
        // (x: number) => string
        let node = IrNode::FnType {
            type_params: None,
            params: vec![IrNode::Param {
                decorators: vec![],
                pat: Box::new(IrNode::BindingIdent {
                    name: Box::new(IrNode::Ident("x".to_string())),
                    type_ann: Some(Box::new(IrNode::KeywordType(TsKeyword::Number))),
                    optional: false,
                }),
            }],
            return_type: Box::new(IrNode::KeywordType(TsKeyword::String)),
        };
        assert!(matches!(node, IrNode::FnType { .. }));
    }
}
