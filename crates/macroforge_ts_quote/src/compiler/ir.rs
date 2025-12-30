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
