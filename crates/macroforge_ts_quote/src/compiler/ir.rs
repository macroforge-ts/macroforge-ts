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
//!
//! Each IR node carries a `span` field indicating its source position for precise
//! error reporting during code generation.

use proc_macro2::TokenStream;

/// Source position for error reporting.
/// Stores byte offset in the template source.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IrSpan {
    /// Start byte offset in template source
    pub start: usize,
    /// End byte offset in template source
    pub end: usize,
}

impl IrSpan {
    /// Create a new span from start and end positions.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Create a span at a single position.
    pub fn at(pos: usize) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    /// Create an empty/unknown span.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Extend this span to include another span.
    pub fn extend(self, other: IrSpan) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Create a span from a start position and text length.
    pub fn from_pos_len(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }
}

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
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Exp, // **

    // Comparison
    EqEq,    // ==
    NotEq,   // !=
    EqEqEq,  // ===
    NotEqEq, // !==
    Lt,      // <
    Le,      // <=
    Gt,      // >
    Ge,      // >=

    // Logical
    And,             // &&
    Or,              // ||
    NullishCoalesce, // ??

    // Bitwise
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
    UShr,   // >>>

    // Other
    In,         // in
    InstanceOf, // instanceof
}

/// Assignment operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,        // =
    AddAssign,     // +=
    SubAssign,     // -=
    MulAssign,     // *=
    DivAssign,     // /=
    ModAssign,     // %=
    ExpAssign,     // **=
    ShlAssign,     // <<=
    ShrAssign,     // >>=
    UShrAssign,    // >>>=
    BitAndAssign,  // &=
    BitOrAssign,   // |=
    BitXorAssign,  // ^=
    AndAssign,     // &&=
    OrAssign,      // ||=
    NullishAssign, // ??=
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,  // -
    Plus,   // +
    Not,    // !
    BitNot, // ~
    TypeOf, // typeof
    Void,   // void
    Delete, // delete
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
/// Each node carries a `span` for precise error reporting.
#[derive(Debug, Clone)]
pub enum IrNode {
    // =========================================================================
    // Declarations
    // =========================================================================
    /// Function declaration: `function name(params): RetType { body }`
    FnDecl {
        span: IrSpan,
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
        span: IrSpan,
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
        span: IrSpan,
        exported: bool,
        declare: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        extends: Vec<IrNode>,
        body: Vec<IrNode>,
    },

    /// Type alias: `type Name<T> = Type`
    TypeAliasDecl {
        span: IrSpan,
        exported: bool,
        declare: bool,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
        type_ann: Box<IrNode>,
    },

    /// Variable declaration: `const/let/var name: Type = init`
    VarDecl {
        span: IrSpan,
        exported: bool,
        declare: bool,
        kind: VarKind,
        decls: Vec<VarDeclarator>,
    },

    /// Enum declaration: `enum Name { ... }`
    EnumDecl {
        span: IrSpan,
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
        span: IrSpan,
        accessibility: Option<Accessibility>,
        params: Vec<IrNode>,
        body: Option<Box<IrNode>>,
    },

    /// Method: `name(params): Type { body }`
    Method {
        span: IrSpan,
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
        span: IrSpan,
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
        span: IrSpan,
        readonly: bool,
        params: Vec<IrNode>,
        type_ann: Box<IrNode>,
    },

    // =========================================================================
    // Interface Members
    // =========================================================================
    /// Property signature: `name?: Type`
    PropSignature {
        span: IrSpan,
        readonly: bool,
        name: Box<IrNode>,
        optional: bool,
        type_ann: Option<Box<IrNode>>,
    },

    /// Method signature: `name(params): Type`
    MethodSignature {
        span: IrSpan,
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
    BlockStmt { span: IrSpan, stmts: Vec<IrNode> },

    /// Expression statement: `expr;`
    ExprStmt { span: IrSpan, expr: Box<IrNode> },

    /// Return statement: `return expr;`
    ReturnStmt {
        span: IrSpan,
        arg: Option<Box<IrNode>>,
    },

    /// Throw statement: `throw expr;`
    ThrowStmt { span: IrSpan, arg: Box<IrNode> },

    /// TypeScript if statement: `if (test) { cons } else { alt }`
    TsIfStmt {
        span: IrSpan,
        test: Box<IrNode>,
        cons: Box<IrNode>,
        alt: Option<Box<IrNode>>,
    },

    /// TypeScript for/while loop (parsed as raw text with placeholders)
    TsLoopStmt { span: IrSpan, parts: Vec<IrNode> },

    /// For-in statement: `for (left in right) body`
    ForInStmt {
        span: IrSpan,
        left: Box<IrNode>,
        right: Box<IrNode>,
        body: Box<IrNode>,
    },

    /// For-of statement: `for (left of right) body`
    ForOfStmt {
        span: IrSpan,
        await_: bool,
        left: Box<IrNode>,
        right: Box<IrNode>,
        body: Box<IrNode>,
    },

    /// Empty statement: `;`
    EmptyStmt { span: IrSpan },

    // =========================================================================
    // Expressions
    // =========================================================================
    /// Identifier: `foo`
    Ident { span: IrSpan, value: String },

    /// Private name: `#foo`
    PrivateName { span: IrSpan, value: String },

    /// String literal: `"hello"`
    StrLit { span: IrSpan, value: String },

    /// Number literal: `42`, `3.14`
    NumLit { span: IrSpan, value: String },

    /// BigInt literal: `42n`
    BigIntLit { span: IrSpan, value: String },

    /// Boolean literal: `true`, `false`
    BoolLit { span: IrSpan, value: bool },

    /// Null literal: `null`
    NullLit { span: IrSpan },

    /// This expression: `this`
    ThisExpr { span: IrSpan },

    /// Super expression: `super`
    SuperExpr { span: IrSpan },

    /// Call expression: `callee(args)`
    CallExpr {
        span: IrSpan,
        callee: Box<IrNode>,
        type_args: Option<Box<IrNode>>,
        args: Vec<IrNode>,
    },

    /// New expression: `new Callee(args)`
    NewExpr {
        span: IrSpan,
        callee: Box<IrNode>,
        type_args: Option<Box<IrNode>>,
        args: Vec<IrNode>,
    },

    /// Member expression: `obj.prop` or `obj[prop]`
    MemberExpr {
        span: IrSpan,
        obj: Box<IrNode>,
        prop: Box<IrNode>,
        computed: bool,
    },

    /// Optional chain expression: `obj?.prop` or `obj?.[prop]` or `fn?.()`
    /// The `expr` contains the full chained expression (MemberExpr or CallExpr)
    OptChainExpr {
        span: IrSpan,
        /// The base object being accessed
        base: Box<IrNode>,
        /// The chained expression (MemberExpr or CallExpr with optional access)
        expr: Box<IrNode>,
    },

    /// Object literal: `{ props }`
    ObjectLit { span: IrSpan, props: Vec<IrNode> },

    /// Array literal: `[elems]`
    ArrayLit { span: IrSpan, elems: Vec<IrNode> },

    /// Arrow function: `(params) => body`
    ArrowExpr {
        span: IrSpan,
        async_: bool,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Option<Box<IrNode>>,
        body: Box<IrNode>, // BlockStmt or Expr
    },

    /// Function expression: `function(params) { body }`
    FnExpr {
        span: IrSpan,
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
        span: IrSpan,
        name: Option<Box<IrNode>>,
        type_params: Option<Box<IrNode>>,
        extends: Option<Box<IrNode>>,
        implements: Vec<IrNode>,
        body: Vec<IrNode>,
    },

    /// Binary expression: `left op right`
    BinExpr {
        span: IrSpan,
        left: Box<IrNode>,
        op: BinaryOp,
        right: Box<IrNode>,
    },

    /// Assignment expression: `left = right`
    AssignExpr {
        span: IrSpan,
        left: Box<IrNode>,
        op: AssignOp,
        right: Box<IrNode>,
    },

    /// Unary expression: `op arg`
    UnaryExpr {
        span: IrSpan,
        op: UnaryOp,
        arg: Box<IrNode>,
    },

    /// Update expression: `++x` or `x++`
    UpdateExpr {
        span: IrSpan,
        op: UpdateOp,
        prefix: bool,
        arg: Box<IrNode>,
    },

    /// Conditional expression: `test ? consequent : alternate`
    CondExpr {
        span: IrSpan,
        test: Box<IrNode>,
        consequent: Box<IrNode>,
        alternate: Box<IrNode>,
    },

    /// Sequence expression: `a, b, c`
    SeqExpr { span: IrSpan, exprs: Vec<IrNode> },

    /// Template literal: `` `hello ${expr}` ``
    TplLit {
        span: IrSpan,
        quasis: Vec<String>,
        exprs: Vec<IrNode>,
    },

    /// Tagged template: `` tag`hello ${expr}` ``
    TaggedTpl {
        span: IrSpan,
        /// The tag function/expression
        tag: Box<IrNode>,
        /// Optional type arguments for the tag
        type_args: Option<Box<IrNode>>,
        /// The template literal (TplLit node)
        tpl: Box<IrNode>,
    },

    /// Parenthesized expression: `(expr)`
    ParenExpr { span: IrSpan, expr: Box<IrNode> },

    /// Await expression: `await expr`
    AwaitExpr { span: IrSpan, arg: Box<IrNode> },

    /// Yield expression: `yield expr` or `yield* expr`
    YieldExpr {
        span: IrSpan,
        delegate: bool,
        arg: Option<Box<IrNode>>,
    },

    /// Type assertion: `expr as Type`
    TsAsExpr {
        span: IrSpan,
        expr: Box<IrNode>,
        type_ann: Box<IrNode>,
    },

    /// Const assertion: `expr as const`
    TsConstAssertion { span: IrSpan, expr: Box<IrNode> },

    /// Satisfies expression: `expr satisfies Type`
    TsSatisfiesExpr {
        span: IrSpan,
        expr: Box<IrNode>,
        type_ann: Box<IrNode>,
    },

    /// Non-null assertion: `expr!`
    TsNonNullExpr { span: IrSpan, expr: Box<IrNode> },

    /// Type instantiation: `expr<Type>`
    TsInstantiation {
        span: IrSpan,
        expr: Box<IrNode>,
        type_args: Box<IrNode>,
    },

    // =========================================================================
    // Object Properties
    // =========================================================================
    /// Key-value property: `key: value`
    KeyValueProp {
        span: IrSpan,
        key: Box<IrNode>,
        value: Box<IrNode>,
    },

    /// Shorthand property: `key` (same as `key: key`)
    ShorthandProp { span: IrSpan, key: Box<IrNode> },

    /// Spread property: `...expr`
    SpreadElement { span: IrSpan, expr: Box<IrNode> },

    /// Computed property name: `[expr]`
    ComputedPropName { span: IrSpan, expr: Box<IrNode> },

    /// Method property: `name() { }`
    MethodProp {
        span: IrSpan,
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
        span: IrSpan,
        name: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
        body: Box<IrNode>,
    },

    /// Setter property: `set name(param) { }`
    SetterProp {
        span: IrSpan,
        name: Box<IrNode>,
        param: Box<IrNode>,
        body: Box<IrNode>,
    },

    // =========================================================================
    // Parameters
    // =========================================================================
    /// Parameter: `name: Type = default`
    Param {
        span: IrSpan,
        decorators: Vec<IrNode>,
        pat: Box<IrNode>,
    },

    /// Binding identifier: `name`
    BindingIdent {
        span: IrSpan,
        name: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Rest pattern: `...rest`
    RestPat {
        span: IrSpan,
        arg: Box<IrNode>,
        type_ann: Option<Box<IrNode>>,
    },

    /// Assignment pattern: `name = default`
    AssignPat {
        span: IrSpan,
        left: Box<IrNode>,
        right: Box<IrNode>,
    },

    /// Array pattern: `[a, b, ...rest]`
    ArrayPat {
        span: IrSpan,
        elems: Vec<Option<IrNode>>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Object pattern: `{ a, b: c }`
    ObjectPat {
        span: IrSpan,
        props: Vec<IrNode>,
        type_ann: Option<Box<IrNode>>,
        optional: bool,
    },

    /// Object pattern property: `a` or `a: b` or `a = default`
    ObjectPatProp {
        span: IrSpan,
        key: Box<IrNode>,
        value: Option<Box<IrNode>>,
    },

    // =========================================================================
    // Type Nodes
    // =========================================================================
    /// Type annotation wrapper: `: Type`
    TypeAnnotation { span: IrSpan, type_ann: Box<IrNode> },

    /// Type reference: `TypeName<Args>`
    TypeRef {
        span: IrSpan,
        name: Box<IrNode>,
        type_params: Option<Box<IrNode>>,
    },

    /// Qualified type name: `NS.Type`
    QualifiedName {
        span: IrSpan,
        left: Box<IrNode>,
        right: Box<IrNode>,
    },

    /// Type parameters: `<T, U extends V>`
    TypeParams { span: IrSpan, params: Vec<IrNode> },

    /// Type parameter: `T extends Constraint = Default`
    TypeParam {
        span: IrSpan,
        name: String,
        constraint: Option<Box<IrNode>>,
        default: Option<Box<IrNode>>,
    },

    /// Type arguments: `<T, U>`
    TypeArgs { span: IrSpan, args: Vec<IrNode> },

    /// Keyword type: `string`, `number`, etc.
    KeywordType { span: IrSpan, keyword: TsKeyword },

    /// Literal type: `"hello"`, `42`, `true`
    LiteralType { span: IrSpan, lit: Box<IrNode> },

    /// Union type: `A | B`
    UnionType { span: IrSpan, types: Vec<IrNode> },

    /// Intersection type: `A & B`
    IntersectionType { span: IrSpan, types: Vec<IrNode> },

    /// Array type: `T[]`
    ArrayType { span: IrSpan, elem: Box<IrNode> },

    /// Tuple type: `[A, B, C]`
    TupleType { span: IrSpan, elems: Vec<IrNode> },

    /// Optional type element: `T?` (in tuple)
    OptionalType { span: IrSpan, type_ann: Box<IrNode> },

    /// Rest type element: `...T` (in tuple)
    RestType { span: IrSpan, type_ann: Box<IrNode> },

    /// Function type: `(params) => RetType`
    FnType {
        span: IrSpan,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Box<IrNode>,
    },

    /// Constructor type: `new (params) => Type`
    ConstructorType {
        span: IrSpan,
        type_params: Option<Box<IrNode>>,
        params: Vec<IrNode>,
        return_type: Box<IrNode>,
    },

    /// Object type / type literal: `{ prop: Type }`
    ObjectType { span: IrSpan, members: Vec<IrNode> },

    /// Parenthesized type: `(Type)`
    ParenType { span: IrSpan, type_ann: Box<IrNode> },

    /// Typeof type: `typeof expr`
    TypeofType { span: IrSpan, expr: Box<IrNode> },

    /// Keyof type: `keyof Type`
    KeyofType { span: IrSpan, type_ann: Box<IrNode> },

    /// Indexed access type: `T[K]`
    IndexedAccessType {
        span: IrSpan,
        obj: Box<IrNode>,
        index: Box<IrNode>,
    },

    /// Conditional type: `T extends U ? X : Y`
    ConditionalType {
        span: IrSpan,
        check: Box<IrNode>,
        extends: Box<IrNode>,
        true_type: Box<IrNode>,
        false_type: Box<IrNode>,
    },

    /// Infer type: `infer T`
    InferType {
        span: IrSpan,
        type_param: Box<IrNode>,
    },

    /// Mapped type: `{ [K in Keys]: Type }`
    MappedType {
        span: IrSpan,
        readonly: Option<bool>, // true = +readonly, false = -readonly, None = no modifier
        type_param: Box<IrNode>,
        name_type: Option<Box<IrNode>>,
        optional: Option<bool>,
        type_ann: Option<Box<IrNode>>,
    },

    /// Import type: `import("module").Type`
    ImportType {
        span: IrSpan,
        arg: Box<IrNode>,
        qualifier: Option<Box<IrNode>>,
        type_args: Option<Box<IrNode>>,
    },

    /// This type: `this`
    ThisType { span: IrSpan },

    // =========================================================================
    // Template Constructs (Rust Integration)
    // =========================================================================
    /// Placeholder: `@{expr}`
    Placeholder {
        span: IrSpan,
        kind: PlaceholderKind,
        expr: TokenStream,
    },

    /// Template if/else-if/else: `{#if cond}...{:else}...{/if}`
    If {
        span: IrSpan,
        condition: TokenStream,
        then_body: Vec<IrNode>,
        else_if_branches: Vec<(TokenStream, Vec<IrNode>)>,
        else_body: Option<Vec<IrNode>>,
    },

    /// Template for loop: `{#for pat in iter}...{/for}`
    For {
        span: IrSpan,
        pattern: TokenStream,
        iterator: TokenStream,
        body: Vec<IrNode>,
    },

    /// Template while loop: `{#while cond}...{/while}`
    While {
        span: IrSpan,
        condition: TokenStream,
        body: Vec<IrNode>,
    },

    /// Template match: `{#match expr}{:case pat}...{/match}`
    Match {
        span: IrSpan,
        expr: TokenStream,
        arms: Vec<MatchArm>,
    },

    /// Let binding: `{$let name = expr}`
    Let {
        span: IrSpan,
        pattern: TokenStream,
        mutable: bool,
        type_hint: Option<TokenStream>,
        value: TokenStream,
    },

    /// Do directive: `{$do code}`
    Do { span: IrSpan, code: TokenStream },

    // =========================================================================
    // Expression-Level Control Flow (produces values, not statements)
    // =========================================================================
    /// If expression - produces a value: `{#if cond} expr {:else} expr {/if}`
    /// All branches required in expression context.
    IfExpr {
        span: IrSpan,
        condition: TokenStream,
        then_expr: Box<IrNode>,
        else_if_branches: Vec<(TokenStream, Box<IrNode>)>,
        else_expr: Box<IrNode>,
    },

    /// For expression - produces an iterator: `{#for pat in iter} expr {/for}`
    ForExpr {
        span: IrSpan,
        pattern: TokenStream,
        iterator: TokenStream,
        body_expr: Box<IrNode>,
    },

    /// While expression - produces an iterator: `{#while cond} expr {/while}`
    WhileExpr {
        span: IrSpan,
        condition: TokenStream,
        body_expr: Box<IrNode>,
    },

    /// Match expression - all arms produce values: `{#match expr}{:case pat} expr {/match}`
    MatchExpr {
        span: IrSpan,
        expr: TokenStream,
        arms: Vec<MatchArmExpr>,
    },

    /// TypeScript injection: `{$typescript stream}`
    TypeScript { span: IrSpan, stream: TokenStream },

    // =========================================================================
    // Comments
    // =========================================================================
    /// Line comment: `{> text <}` -> `//`
    LineComment { span: IrSpan, text: String },

    /// Block comment: `{>> text <<}` -> `/* */`
    BlockComment { span: IrSpan, text: String },

    /// Doc comment: `/** text */`
    DocComment { span: IrSpan, text: String },

    /// A node with an attached doc comment.
    Documented {
        span: IrSpan,
        doc: String,
        inner: Box<IrNode>,
    },

    // =========================================================================
    // Special Constructs
    // =========================================================================
    /// Identifier block: `{|prefix_@{name}_suffix|}`
    IdentBlock { span: IrSpan, parts: Vec<IrNode> },

    /// String interpolation: `"text @{expr} more"`
    StringInterp {
        span: IrSpan,
        quote: char,
        parts: Vec<IrNode>,
    },

    /// Raw text (fallback for unparseable content - should be minimized)
    Raw { span: IrSpan, value: String },

    /// Enum member: `Name = value`
    EnumMember {
        span: IrSpan,
        name: Box<IrNode>,
        init: Option<Box<IrNode>>,
    },

    /// Decorator: `@expr`
    Decorator { span: IrSpan, expr: Box<IrNode> },

    /// Import declaration
    ImportDecl {
        span: IrSpan,
        type_only: bool,
        specifiers: Vec<IrNode>,
        src: String,
    },

    /// Named import: `{ a, b as c }`
    NamedImport {
        span: IrSpan,
        local: Box<IrNode>,
        imported: Option<Box<IrNode>>,
    },

    /// Default import: `foo`
    DefaultImport { span: IrSpan, local: Box<IrNode> },

    /// Namespace import: `* as foo`
    NamespaceImport { span: IrSpan, local: Box<IrNode> },

    /// Named export: `export { a, b as c }`
    NamedExport {
        span: IrSpan,
        specifiers: Vec<IrNode>,
        src: Option<String>,
        type_only: bool,
    },

    /// Export specifier: `a as b`
    ExportSpecifier {
        span: IrSpan,
        local: Box<IrNode>,
        exported: Option<Box<IrNode>>,
    },

    /// Default export: `export default expr`
    ExportDefaultExpr { span: IrSpan, expr: Box<IrNode> },

    /// Re-export all: `export * from "module"`
    ExportAll {
        span: IrSpan,
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
    pub span: IrSpan,
    pub name: Box<IrNode>,
    pub type_ann: Option<Box<IrNode>>,
    pub init: Option<Box<IrNode>>,
    pub definite: bool,
}

/// Match arm for template match construct (statement context).
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub span: IrSpan,
    pub pattern: TokenStream,
    pub guard: Option<TokenStream>,
    pub body: Vec<IrNode>,
}

/// Match arm for expression context - produces a value instead of statements.
#[derive(Debug, Clone)]
pub struct MatchArmExpr {
    pub span: IrSpan,
    pub pattern: TokenStream,
    pub guard: Option<TokenStream>,
    pub body_expr: Box<IrNode>,
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

// =============================================================================
// IrNode Builder Methods
// =============================================================================

/// Trait for types that can be converted to IR nodes.
/// Implemented by Token to allow direct conversion.
pub trait IntoIrNode {
    fn text(&self) -> &str;
    fn ir_span(&self) -> IrSpan;
}

/// Builder methods for creating IR nodes from tokens.
/// These methods extract both span and text from the token automatically.
impl IrNode {
    // -------------------------------------------------------------------------
    // From Token Builders
    // -------------------------------------------------------------------------

    /// Create an identifier node from a token.
    pub fn ident(token: &impl IntoIrNode) -> Self {
        IrNode::Ident {
            span: token.ir_span(),
            value: token.text().to_string(),
        }
    }

    /// Create a number literal node from a token.
    pub fn num_lit(token: &impl IntoIrNode) -> Self {
        IrNode::NumLit {
            span: token.ir_span(),
            value: token.text().to_string(),
        }
    }

    /// Create a boolean literal node from a token (true/false based on token text).
    pub fn bool_lit(token: &impl IntoIrNode) -> Self {
        IrNode::BoolLit {
            span: token.ir_span(),
            value: token.text() == "true",
        }
    }

    /// Create a bigint literal node from a token (strips trailing 'n').
    pub fn bigint_lit(token: &impl IntoIrNode) -> Self {
        let text = token.text();
        let value = text.strip_suffix('n').unwrap_or(text);
        IrNode::BigIntLit {
            span: token.ir_span(),
            value: value.to_string(),
        }
    }

    /// Create a private name node from a token (strips leading #).
    pub fn private_name(token: &impl IntoIrNode) -> Self {
        let text = token.text();
        let value = text.strip_prefix('#').unwrap_or(text);
        IrNode::PrivateName {
            span: token.ir_span(),
            value: value.to_string(),
        }
    }

    /// Create a null literal node from a token.
    pub fn null_lit(token: &impl IntoIrNode) -> Self {
        IrNode::NullLit {
            span: token.ir_span(),
        }
    }

    /// Create a this expression node from a token.
    pub fn this_expr(token: &impl IntoIrNode) -> Self {
        IrNode::ThisExpr {
            span: token.ir_span(),
        }
    }

    /// Create a super expression node from a token.
    pub fn super_expr(token: &impl IntoIrNode) -> Self {
        IrNode::SuperExpr {
            span: token.ir_span(),
        }
    }

    /// Create a raw text node from a token.
    pub fn raw(token: &impl IntoIrNode) -> Self {
        IrNode::Raw {
            span: token.ir_span(),
            value: token.text().to_string(),
        }
    }

    /// Create an empty statement from a token.
    pub fn empty_stmt(token: &impl IntoIrNode) -> Self {
        IrNode::EmptyStmt {
            span: token.ir_span(),
        }
    }

    // -------------------------------------------------------------------------
    // Type Keywords
    // -------------------------------------------------------------------------

    /// Create a keyword type node from a token and keyword enum.
    pub fn keyword_type(token: &impl IntoIrNode, keyword: TsKeyword) -> Self {
        IrNode::KeywordType {
            span: token.ir_span(),
            keyword,
        }
    }

    /// Create a this type node from a token.
    pub fn this_type(token: &impl IntoIrNode) -> Self {
        IrNode::ThisType {
            span: token.ir_span(),
        }
    }

    // -------------------------------------------------------------------------
    // With Custom Value (for when token text differs from desired value)
    // -------------------------------------------------------------------------

    /// Create an identifier with a custom value (uses token for span only).
    pub fn ident_with(token: &impl IntoIrNode, value: impl Into<String>) -> Self {
        IrNode::Ident {
            span: token.ir_span(),
            value: value.into(),
        }
    }

    // -------------------------------------------------------------------------
    // Span Helpers
    // -------------------------------------------------------------------------

    /// Get the span of this node.
    pub fn span(&self) -> IrSpan {
        match self {
            // Declarations
            IrNode::FnDecl { span, .. } => *span,
            IrNode::ClassDecl { span, .. } => *span,
            IrNode::InterfaceDecl { span, .. } => *span,
            IrNode::TypeAliasDecl { span, .. } => *span,
            IrNode::VarDecl { span, .. } => *span,
            IrNode::EnumDecl { span, .. } => *span,

            // Class Members
            IrNode::Constructor { span, .. } => *span,
            IrNode::Method { span, .. } => *span,
            IrNode::ClassProp { span, .. } => *span,
            IrNode::IndexSignature { span, .. } => *span,

            // Interface Members
            IrNode::PropSignature { span, .. } => *span,
            IrNode::MethodSignature { span, .. } => *span,

            // Statements
            IrNode::BlockStmt { span, .. } => *span,
            IrNode::ExprStmt { span, .. } => *span,
            IrNode::ReturnStmt { span, .. } => *span,
            IrNode::ThrowStmt { span, .. } => *span,
            IrNode::TsIfStmt { span, .. } => *span,
            IrNode::TsLoopStmt { span, .. } => *span,
            IrNode::ForInStmt { span, .. } => *span,
            IrNode::ForOfStmt { span, .. } => *span,
            IrNode::EmptyStmt { span } => *span,

            // Expressions
            IrNode::Ident { span, .. } => *span,
            IrNode::PrivateName { span, .. } => *span,
            IrNode::StrLit { span, .. } => *span,
            IrNode::NumLit { span, .. } => *span,
            IrNode::BigIntLit { span, .. } => *span,
            IrNode::BoolLit { span, .. } => *span,
            IrNode::NullLit { span } => *span,
            IrNode::ThisExpr { span } => *span,
            IrNode::SuperExpr { span } => *span,
            IrNode::CallExpr { span, .. } => *span,
            IrNode::NewExpr { span, .. } => *span,
            IrNode::MemberExpr { span, .. } => *span,
            IrNode::OptChainExpr { span, .. } => *span,
            IrNode::ObjectLit { span, .. } => *span,
            IrNode::ArrayLit { span, .. } => *span,
            IrNode::ArrowExpr { span, .. } => *span,
            IrNode::FnExpr { span, .. } => *span,
            IrNode::ClassExpr { span, .. } => *span,
            IrNode::BinExpr { span, .. } => *span,
            IrNode::AssignExpr { span, .. } => *span,
            IrNode::UnaryExpr { span, .. } => *span,
            IrNode::UpdateExpr { span, .. } => *span,
            IrNode::CondExpr { span, .. } => *span,
            IrNode::SeqExpr { span, .. } => *span,
            IrNode::TplLit { span, .. } => *span,
            IrNode::TaggedTpl { span, .. } => *span,
            IrNode::ParenExpr { span, .. } => *span,
            IrNode::AwaitExpr { span, .. } => *span,
            IrNode::YieldExpr { span, .. } => *span,
            IrNode::TsAsExpr { span, .. } => *span,
            IrNode::TsConstAssertion { span, .. } => *span,
            IrNode::TsSatisfiesExpr { span, .. } => *span,
            IrNode::TsNonNullExpr { span, .. } => *span,
            IrNode::TsInstantiation { span, .. } => *span,

            // Object Properties
            IrNode::KeyValueProp { span, .. } => *span,
            IrNode::ShorthandProp { span, .. } => *span,
            IrNode::SpreadElement { span, .. } => *span,
            IrNode::ComputedPropName { span, .. } => *span,
            IrNode::MethodProp { span, .. } => *span,
            IrNode::GetterProp { span, .. } => *span,
            IrNode::SetterProp { span, .. } => *span,

            // Parameters
            IrNode::Param { span, .. } => *span,
            IrNode::BindingIdent { span, .. } => *span,
            IrNode::RestPat { span, .. } => *span,
            IrNode::AssignPat { span, .. } => *span,
            IrNode::ArrayPat { span, .. } => *span,
            IrNode::ObjectPat { span, .. } => *span,
            IrNode::ObjectPatProp { span, .. } => *span,

            // Type Nodes
            IrNode::TypeAnnotation { span, .. } => *span,
            IrNode::TypeRef { span, .. } => *span,
            IrNode::QualifiedName { span, .. } => *span,
            IrNode::TypeParams { span, .. } => *span,
            IrNode::TypeParam { span, .. } => *span,
            IrNode::TypeArgs { span, .. } => *span,
            IrNode::KeywordType { span, .. } => *span,
            IrNode::LiteralType { span, .. } => *span,
            IrNode::UnionType { span, .. } => *span,
            IrNode::IntersectionType { span, .. } => *span,
            IrNode::ArrayType { span, .. } => *span,
            IrNode::TupleType { span, .. } => *span,
            IrNode::OptionalType { span, .. } => *span,
            IrNode::RestType { span, .. } => *span,
            IrNode::FnType { span, .. } => *span,
            IrNode::ConstructorType { span, .. } => *span,
            IrNode::ObjectType { span, .. } => *span,
            IrNode::ParenType { span, .. } => *span,
            IrNode::TypeofType { span, .. } => *span,
            IrNode::KeyofType { span, .. } => *span,
            IrNode::IndexedAccessType { span, .. } => *span,
            IrNode::ConditionalType { span, .. } => *span,
            IrNode::InferType { span, .. } => *span,
            IrNode::MappedType { span, .. } => *span,
            IrNode::ImportType { span, .. } => *span,
            IrNode::ThisType { span } => *span,

            // Template Constructs
            IrNode::Placeholder { span, .. } => *span,
            IrNode::If { span, .. } => *span,
            IrNode::For { span, .. } => *span,
            IrNode::While { span, .. } => *span,
            IrNode::Match { span, .. } => *span,
            IrNode::Let { span, .. } => *span,
            IrNode::Do { span, .. } => *span,
            IrNode::IfExpr { span, .. } => *span,
            IrNode::ForExpr { span, .. } => *span,
            IrNode::WhileExpr { span, .. } => *span,
            IrNode::MatchExpr { span, .. } => *span,
            IrNode::TypeScript { span, .. } => *span,

            // Comments
            IrNode::LineComment { span, .. } => *span,
            IrNode::BlockComment { span, .. } => *span,
            IrNode::DocComment { span, .. } => *span,
            IrNode::Documented { span, .. } => *span,

            // Special Constructs
            IrNode::IdentBlock { span, .. } => *span,
            IrNode::StringInterp { span, .. } => *span,
            IrNode::Raw { span, .. } => *span,
            IrNode::EnumMember { span, .. } => *span,
            IrNode::Decorator { span, .. } => *span,
            IrNode::ImportDecl { span, .. } => *span,
            IrNode::NamedImport { span, .. } => *span,
            IrNode::DefaultImport { span, .. } => *span,
            IrNode::NamespaceImport { span, .. } => *span,
            IrNode::NamedExport { span, .. } => *span,
            IrNode::ExportSpecifier { span, .. } => *span,
            IrNode::ExportDefaultExpr { span, .. } => *span,
            IrNode::ExportAll { span, .. } => *span,
        }
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
        let cloned = kind;
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
        let node = IrNode::Ident {
            span: IrSpan::empty(),
            value: "foo".to_string(),
        };
        assert!(matches!(node, IrNode::Ident { value, .. } if value == "foo"));
    }

    #[test]
    fn test_ir_node_str_lit() {
        let node = IrNode::StrLit {
            span: IrSpan::empty(),
            value: "hello".to_string(),
        };
        assert!(matches!(node, IrNode::StrLit { value, .. } if value == "hello"));
    }

    #[test]
    fn test_ir_node_num_lit() {
        let node = IrNode::NumLit {
            span: IrSpan::empty(),
            value: "42".to_string(),
        };
        assert!(matches!(node, IrNode::NumLit { value, .. } if value == "42"));
    }

    #[test]
    fn test_ir_node_bool_lit() {
        let true_node = IrNode::BoolLit {
            span: IrSpan::empty(),
            value: true,
        };
        let false_node = IrNode::BoolLit {
            span: IrSpan::empty(),
            value: false,
        };
        assert!(matches!(true_node, IrNode::BoolLit { value: true, .. }));
        assert!(matches!(false_node, IrNode::BoolLit { value: false, .. }));
    }

    #[test]
    fn test_ir_node_null_lit() {
        let node = IrNode::NullLit {
            span: IrSpan::empty(),
        };
        assert!(matches!(node, IrNode::NullLit { .. }));
    }

    #[test]
    fn test_ir_node_this_expr() {
        let node = IrNode::ThisExpr {
            span: IrSpan::empty(),
        };
        assert!(matches!(node, IrNode::ThisExpr { .. }));
    }

    #[test]
    fn test_ir_node_super_expr() {
        let node = IrNode::SuperExpr {
            span: IrSpan::empty(),
        };
        assert!(matches!(node, IrNode::SuperExpr { .. }));
    }

    #[test]
    fn test_ir_node_empty_stmt() {
        let node = IrNode::EmptyStmt {
            span: IrSpan::empty(),
        };
        assert!(matches!(node, IrNode::EmptyStmt { .. }));
    }

    #[test]
    fn test_ir_node_block_stmt() {
        let node = IrNode::BlockStmt {
            span: IrSpan::empty(),
            stmts: vec![],
        };
        assert!(matches!(node, IrNode::BlockStmt { stmts, .. } if stmts.is_empty()));
    }

    #[test]
    fn test_ir_node_expr_stmt() {
        let node = IrNode::ExprStmt {
            span: IrSpan::empty(),
            expr: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "x".to_string(),
            }),
        };
        assert!(matches!(node, IrNode::ExprStmt { .. }));
    }

    #[test]
    fn test_ir_node_return_stmt_with_arg() {
        let node = IrNode::ReturnStmt {
            span: IrSpan::empty(),
            arg: Some(Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "42".to_string(),
            })),
        };
        assert!(matches!(node, IrNode::ReturnStmt { arg: Some(_), .. }));
    }

    #[test]
    fn test_ir_node_return_stmt_without_arg() {
        let node = IrNode::ReturnStmt {
            span: IrSpan::empty(),
            arg: None,
        };
        assert!(matches!(node, IrNode::ReturnStmt { arg: None, .. }));
    }

    #[test]
    fn test_ir_node_throw_stmt() {
        let node = IrNode::ThrowStmt {
            span: IrSpan::empty(),
            arg: Box::new(IrNode::StrLit {
                span: IrSpan::empty(),
                value: "error".to_string(),
            }),
        };
        assert!(matches!(node, IrNode::ThrowStmt { .. }));
    }

    #[test]
    fn test_ir_node_bin_expr() {
        let node = IrNode::BinExpr {
            span: IrSpan::empty(),
            left: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "1".to_string(),
            }),
            op: BinaryOp::Add,
            right: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "2".to_string(),
            }),
        };
        assert!(matches!(
            node,
            IrNode::BinExpr {
                op: BinaryOp::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_unary_expr() {
        let node = IrNode::UnaryExpr {
            span: IrSpan::empty(),
            op: UnaryOp::Minus,
            arg: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "5".to_string(),
            }),
        };
        assert!(matches!(
            node,
            IrNode::UnaryExpr {
                op: UnaryOp::Minus,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_call_expr() {
        let node = IrNode::CallExpr {
            span: IrSpan::empty(),
            callee: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "foo".to_string(),
            }),
            type_args: None,
            args: vec![IrNode::NumLit {
                span: IrSpan::empty(),
                value: "1".to_string(),
            }],
        };
        assert!(matches!(node, IrNode::CallExpr { .. }));
    }

    #[test]
    fn test_ir_node_new_expr() {
        let node = IrNode::NewExpr {
            span: IrSpan::empty(),
            callee: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "MyClass".to_string(),
            }),
            type_args: None,
            args: vec![],
        };
        assert!(matches!(node, IrNode::NewExpr { .. }));
    }

    #[test]
    fn test_ir_node_member_expr() {
        let node = IrNode::MemberExpr {
            span: IrSpan::empty(),
            obj: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "obj".to_string(),
            }),
            prop: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "prop".to_string(),
            }),
            computed: false,
        };
        assert!(matches!(
            node,
            IrNode::MemberExpr {
                computed: false,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_object_lit() {
        let node = IrNode::ObjectLit {
            span: IrSpan::empty(),
            props: vec![],
        };
        assert!(matches!(node, IrNode::ObjectLit { props, .. } if props.is_empty()));
    }

    #[test]
    fn test_ir_node_array_lit() {
        let node = IrNode::ArrayLit {
            span: IrSpan::empty(),
            elems: vec![IrNode::NumLit {
                span: IrSpan::empty(),
                value: "1".to_string(),
            }],
        };
        assert!(matches!(node, IrNode::ArrayLit { elems, .. } if elems.len() == 1));
    }

    #[test]
    fn test_ir_node_arrow_expr() {
        let node = IrNode::ArrowExpr {
            span: IrSpan::empty(),
            async_: false,
            type_params: None,
            params: vec![],
            return_type: None,
            body: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "42".to_string(),
            }),
        };
        assert!(matches!(node, IrNode::ArrowExpr { async_: false, .. }));
    }

    #[test]
    fn test_ir_node_cond_expr() {
        let node = IrNode::CondExpr {
            span: IrSpan::empty(),
            test: Box::new(IrNode::BoolLit {
                span: IrSpan::empty(),
                value: true,
            }),
            consequent: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "1".to_string(),
            }),
            alternate: Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "2".to_string(),
            }),
        };
        assert!(matches!(node, IrNode::CondExpr { .. }));
    }

    #[test]
    fn test_ir_node_keyword_type() {
        let node = IrNode::KeywordType {
            span: IrSpan::empty(),
            keyword: TsKeyword::String,
        };
        assert!(matches!(
            node,
            IrNode::KeywordType {
                keyword: TsKeyword::String,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_union_type() {
        let node = IrNode::UnionType {
            span: IrSpan::empty(),
            types: vec![
                IrNode::KeywordType {
                    span: IrSpan::empty(),
                    keyword: TsKeyword::String,
                },
                IrNode::KeywordType {
                    span: IrSpan::empty(),
                    keyword: TsKeyword::Number,
                },
            ],
        };
        assert!(matches!(node, IrNode::UnionType { types, .. } if types.len() == 2));
    }

    #[test]
    fn test_ir_node_array_type() {
        let node = IrNode::ArrayType {
            span: IrSpan::empty(),
            elem: Box::new(IrNode::KeywordType {
                span: IrSpan::empty(),
                keyword: TsKeyword::Number,
            }),
        };
        assert!(matches!(node, IrNode::ArrayType { .. }));
    }

    #[test]
    fn test_ir_node_var_decl() {
        let node = IrNode::VarDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            kind: VarKind::Const,
            decls: vec![],
        };
        assert!(matches!(
            node,
            IrNode::VarDecl {
                kind: VarKind::Const,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_fn_decl() {
        let node = IrNode::FnDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            async_: true,
            generator: false,
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "myFn".to_string(),
            }),
            type_params: None,
            params: vec![],
            return_type: None,
            body: Some(Box::new(IrNode::BlockStmt {
                span: IrSpan::empty(),
                stmts: vec![],
            })),
        };
        assert!(matches!(node, IrNode::FnDecl { async_: true, .. }));
    }

    #[test]
    fn test_ir_node_class_decl() {
        let node = IrNode::ClassDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            abstract_: false,
            decorators: vec![],
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "MyClass".to_string(),
            }),
            type_params: None,
            extends: None,
            implements: vec![],
            body: vec![],
        };
        assert!(matches!(
            node,
            IrNode::ClassDecl {
                abstract_: false,
                ..
            }
        ));
    }

    #[test]
    fn test_ir_node_interface_decl() {
        let node = IrNode::InterfaceDecl {
            span: IrSpan::empty(),
            exported: true,
            declare: false,
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "MyInterface".to_string(),
            }),
            type_params: None,
            extends: vec![],
            body: vec![],
        };
        assert!(matches!(node, IrNode::InterfaceDecl { exported: true, .. }));
    }

    #[test]
    fn test_ir_node_type_alias_decl() {
        let node = IrNode::TypeAliasDecl {
            span: IrSpan::empty(),
            exported: false,
            declare: false,
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "MyType".to_string(),
            }),
            type_params: None,
            type_ann: Box::new(IrNode::KeywordType {
                span: IrSpan::empty(),
                keyword: TsKeyword::String,
            }),
        };
        assert!(matches!(node, IrNode::TypeAliasDecl { .. }));
    }

    #[test]
    fn test_ir_node_clone() {
        let node = IrNode::Ident {
            span: IrSpan::empty(),
            value: "test".to_string(),
        };
        let cloned = node.clone();
        assert!(matches!(cloned, IrNode::Ident { value, .. } if value == "test"));
    }

    // ==================== VarDeclarator Tests ====================

    #[test]
    fn test_var_declarator_basic() {
        let decl = VarDeclarator {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "x".to_string(),
            }),
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
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "x".to_string(),
            }),
            type_ann: None,
            init: Some(Box::new(IrNode::NumLit {
                span: IrSpan::empty(),
                value: "42".to_string(),
            })),
            definite: false,
        };
        assert!(decl.init.is_some());
    }

    #[test]
    fn test_var_declarator_with_type() {
        let decl = VarDeclarator {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "x".to_string(),
            }),
            type_ann: Some(Box::new(IrNode::KeywordType {
                span: IrSpan::empty(),
                keyword: TsKeyword::Number,
            })),
            init: None,
            definite: true,
        };
        assert!(decl.type_ann.is_some());
        assert!(decl.definite);
    }

    #[test]
    fn test_var_declarator_clone() {
        let decl = VarDeclarator {
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "x".to_string(),
            }),
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
            span: IrSpan::empty(),
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
            span: IrSpan::empty(),
            pattern: TokenStream::new(),
            guard: Some(TokenStream::new()),
            body: vec![IrNode::ReturnStmt {
                span: IrSpan::empty(),
                arg: None,
            }],
        };
        assert!(arm.guard.is_some());
        assert_eq!(arm.body.len(), 1);
    }

    #[test]
    fn test_match_arm_clone() {
        let arm = MatchArm {
            span: IrSpan::empty(),
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
            IrNode::Ident {
                span: IrSpan::empty(),
                value: "foo".to_string(),
            },
            IrNode::NumLit {
                span: IrSpan::empty(),
                value: "42".to_string(),
            },
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
            span: IrSpan::empty(),
            obj: Box::new(IrNode::MemberExpr {
                span: IrSpan::empty(),
                obj: Box::new(IrNode::Ident {
                    span: IrSpan::empty(),
                    value: "obj".to_string(),
                }),
                prop: Box::new(IrNode::Ident {
                    span: IrSpan::empty(),
                    value: "foo".to_string(),
                }),
                computed: false,
            }),
            prop: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "bar".to_string(),
            }),
            computed: false,
        };
        assert!(matches!(node, IrNode::MemberExpr { .. }));
    }

    #[test]
    fn test_call_with_args() {
        // foo(1, "hello", true)
        let node = IrNode::CallExpr {
            span: IrSpan::empty(),
            callee: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "foo".to_string(),
            }),
            type_args: None,
            args: vec![
                IrNode::NumLit {
                    span: IrSpan::empty(),
                    value: "1".to_string(),
                },
                IrNode::StrLit {
                    span: IrSpan::empty(),
                    value: "hello".to_string(),
                },
                IrNode::BoolLit {
                    span: IrSpan::empty(),
                    value: true,
                },
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
            span: IrSpan::empty(),
            exported: true,
            declare: false,
            abstract_: false,
            decorators: vec![],
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "MyClass".to_string(),
            }),
            type_params: None,
            extends: Some(Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "BaseClass".to_string(),
            })),
            implements: vec![IrNode::Ident {
                span: IrSpan::empty(),
                value: "Interface1".to_string(),
            }],
            body: vec![
                IrNode::Constructor {
                    span: IrSpan::empty(),
                    accessibility: Some(Accessibility::Public),
                    params: vec![],
                    body: Some(Box::new(IrNode::BlockStmt {
                        span: IrSpan::empty(),
                        stmts: vec![],
                    })),
                },
                IrNode::ClassProp {
                    span: IrSpan::empty(),
                    static_: false,
                    accessibility: Some(Accessibility::Private),
                    readonly: true,
                    declare: false,
                    optional: false,
                    definite: false,
                    name: Box::new(IrNode::Ident {
                        span: IrSpan::empty(),
                        value: "value".to_string(),
                    }),
                    type_ann: Some(Box::new(IrNode::KeywordType {
                        span: IrSpan::empty(),
                        keyword: TsKeyword::Number,
                    })),
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
            span: IrSpan::empty(),
            name: Box::new(IrNode::Ident {
                span: IrSpan::empty(),
                value: "Promise".to_string(),
            }),
            type_params: Some(Box::new(IrNode::TypeArgs {
                span: IrSpan::empty(),
                args: vec![IrNode::KeywordType {
                    span: IrSpan::empty(),
                    keyword: TsKeyword::String,
                }],
            })),
        };
        assert!(matches!(node, IrNode::TypeRef { .. }));
    }

    #[test]
    fn test_fn_type() {
        // (x: number) => string
        let node = IrNode::FnType {
            span: IrSpan::empty(),
            type_params: None,
            params: vec![IrNode::Param {
                span: IrSpan::empty(),
                decorators: vec![],
                pat: Box::new(IrNode::BindingIdent {
                    span: IrSpan::empty(),
                    name: Box::new(IrNode::Ident {
                        span: IrSpan::empty(),
                        value: "x".to_string(),
                    }),
                    type_ann: Some(Box::new(IrNode::KeywordType {
                        span: IrSpan::empty(),
                        keyword: TsKeyword::Number,
                    })),
                    optional: false,
                }),
            }],
            return_type: Box::new(IrNode::KeywordType {
                span: IrSpan::empty(),
                keyword: TsKeyword::String,
            }),
        };
        assert!(matches!(node, IrNode::FnType { .. }));
    }
}
