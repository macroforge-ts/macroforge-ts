# Context and IR

Every macro runs with a `MacroContextIR`: what invoked it, where, and a structured view (the IR) of
the declaration it is attached to. Derive macros usually read the IR through
[`DeriveInput`](../../docs/custom-macros/ts-macro-derive); this page describes the context and IR
types underneath, which attribute macros use directly.

## MacroContextIR

A macro gets its context from its input stream with `input.context()`, or from `DeriveInput`'s
`context` field.

Rust

```
pub struct MacroContextIR {
    pub macro_kind: MacroKind,        // Derive, Attribute or Call
    pub macro_name: String,           // e.g. "Debug"
    pub module_path: String,          // the module the macro was imported from
    pub decorator_span: SpanIR,       // the whole @derive(...) or @attr(...)
    pub macro_name_span: Option<SpanIR>, // just the macro's name in it
    pub target_span: SpanIR,          // the declaration the macro is attached to
    pub file_name: String,            // the file being expanded
    pub target: TargetIR,             // the declaration, as IR
    pub target_source: String,        // the declaration's source text
    pub import_registry: ImportRegistry, // the file's imports
    pub config: Option<MacroforgeConfig>, // the project's macroforge.config
    pub type_registry: TypeRegistry,  // every type in the project
    pub resolved_fields: Option<HashMap<String, ResolvedTypeRef>>,
    pub abi_version: u32,
}
```

| Method                                                                          | Returns                                                                                                     |
| ------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `as_class()`, `as_interface()`, `as_enum()`, `as_type_alias()`, `as_function()` | The target as that kind of IR, or `None`                                                                    |
| `error_span()`                                                                  | The macro's name if known, else the whole decorator: the place to point an error                            |
| `import_specifier_for(type_name)`                                               | The module the file should import a type from; see [Type-Aware Macros](../../docs/custom-macros/type-aware) |

`type_registry`, `resolved_fields` and `config` are covered in
[Type-Aware Macros](../../docs/custom-macros/type-aware), the import methods in
[Output and Imports](../../docs/custom-macros/output#imports).

Note

A call macro (`$name(...)`) gets a minimal context: its target is `TargetIR::Other`, `target_source`
is the text between the parentheses, and the file name, config and registries are empty.

## TargetIR

Rust

```
pub enum TargetIR {
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
    Function(FunctionIR),
    Other,            // a call macro's arguments, or anything unsupported
}
```

An attribute macro on a class method receives the method as a `FunctionIR`.

## Type parameters

Every generic declaration lists its type parameters as declared. `declaration()` renders one as a
generic function would declare it, and `declare_all` and `apply_all` render a whole list as
`<T extends Shape>` and `<T>`. Variance annotations (`in`, `out`) are not kept: TypeScript rejects
them on a function.

Rust

```
pub struct TypeParamIR {
    pub name: String,
    pub constraint: Option<String>,  // the type after \`extends\`
    pub default: Option<String>,     // the type after \`=\`
    pub is_const: bool,
}
```

## Classes

Rust

```
pub struct ClassIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,           // the braces and everything between them
    pub is_abstract: bool,
    pub type_params: Vec<TypeParamIR>, // e.g. T, K extends string
    pub heritage: Vec<String>,       // extends and implements clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<FieldIR>,
    pub methods: Vec<MethodSigIR>,
}

pub struct FieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,             // the annotation as written, e.g. "string[]"
    pub optional: bool,              // declared with ?
    pub readonly: bool,
    pub visibility: Visibility,      // Public, Protected or Private
    pub decorators: Vec<DecoratorIR>,
}

pub struct MethodSigIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub params_src: String,          // the parameter list as written
    pub return_type_src: String,
    pub is_static: bool,
    pub is_async: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
    pub body_span: Option<SpanIR>,   // None for an abstract method
    pub body_src: Option<String>,
}
```

## Interfaces

Rust

```
pub struct InterfaceIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub heritage: Vec<String>,       // extends clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<InterfaceFieldIR>,
    pub methods: Vec<InterfaceMethodIR>,
}

pub struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
}

pub struct InterfaceMethodIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub params_src: String,
    pub return_type_src: String,
    pub optional: bool,
    pub decorators: Vec<DecoratorIR>,
}
```

## Enums

Rust

```
pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
    pub is_const: bool,              // declared const enum
}

pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,
    pub decorators: Vec<DecoratorIR>,
}

pub enum EnumValue {
    String(String),   // Active = "active"
    Number(f64),      // Low = 1
    Auto,             // no initializer: one more than the previous member
    Expr(String),     // Mask = A | B, kept as source text
}
```

`EnumValue` has a test and an accessor for each kind: `is_string()`/`as_string()`,
`is_number()`/`as_number()`, `is_expr()`/`as_expr()` and `is_auto()`.

## Type Aliases

Rust

```
pub struct TypeAliasIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub type_params: Vec<TypeParamIR>,
    pub body: TypeBody,
}

pub enum TypeBody {
    Union(Vec<TypeMember>),          // A | B | "c"
    Intersection(Vec<TypeMember>),   // A & B
    Object { fields: Vec<InterfaceFieldIR> }, // { a: string }
    Tuple(Vec<String>),              // [string, number]
    Alias(String),                   // Other<T>
    Other(String),                   // anything else, as source text
}

pub struct TypeMember {
    pub kind: TypeMemberKind,
    pub decorators: Vec<DecoratorIR>, // JSDoc tags on a union member
}

pub enum TypeMemberKind {
    Literal(String),                 // "active", 42
    TypeRef(String),                 // User
    Object { fields: Vec<InterfaceFieldIR> },
    Intersection(Vec<TypeMember>),
}
```

`TypeBody` has `is_union()`, `is_intersection()`, `is_object()`, `is_tuple()` and `is_alias()`, each
with an `as_...()` accessor. `TypeMember` has `is_literal()`, `is_type_ref()`, `is_object()`, their
`as_...()` accessors, `as_intersection_members()`, `type_name()` (the name of a literal or type
reference) and `has_decorator(name)`.

## Functions

Attribute macros on functions, and on class methods, receive a `FunctionIR`:

Rust

```
pub struct FunctionIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub signature_span: SpanIR,      // from the start to the body's opening brace
    pub is_async: bool,
    pub is_generator: bool,
    pub is_exported: bool,
    pub is_default_export: bool,
    pub type_params: Vec<TypeParamIR>,
    pub params: Vec<FunctionParamIR>,
    pub return_type_src: String,
    pub body_src: String,
    pub decorators: Vec<DecoratorIR>,
}

pub struct FunctionParamIR {
    pub name: String,
    pub span: SpanIR,
    pub type_src: String,
    pub default_src: Option<String>,
    pub is_optional: bool,
    pub is_rest: bool,
    pub decorators: Vec<DecoratorIR>,
}
```

## Decorators

Rust

```
pub struct DecoratorIR {
    pub name: String,      // e.g. "endec"
    pub args_src: String,  // the arguments as written, e.g. "skip, rename: \\"id\\""
    pub span: SpanIR,
}
```

A macro reads its options from `args_src`. The helpers in `macroforge_ts::builtin::derive::common`
parse the common shapes:

Rust

```
use macroforge_ts::builtin::derive::common::{extract_named_string, has_flag};

for decorator in &field.decorators {
    if decorator.name.eq_ignore_ascii_case("validate") {
        let required = has_flag(&decorator.args_src, "required");
        // rename: "id", rename = "id" or rename("id"); escapes are decoded
        let rename = extract_named_string(&decorator.args_src, "rename");
    }
}
```

## Spans

Every IR node carries a `SpanIR`. Spans count positions, which are byte offsets plus one; see
[Spans](../../docs/custom-macros/output#spans) for reading the text a span covers.

## Next Steps

- [Type-Aware Macros](../../docs/custom-macros/type-aware)
- [Output and Imports](../../docs/custom-macros/output)
