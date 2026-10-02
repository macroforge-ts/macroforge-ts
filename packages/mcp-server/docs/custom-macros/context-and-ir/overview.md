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
