# ts\_macro\_derive

The `#[ts_macro_derive]` attribute is a Rust procedural macro that registers your function as a
Macroforge derive macro.

## Basic Syntax

Rust

```
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}
```

Note

The generated code refers to `macroforge_ts` by name, so depend on it under that name, without
renaming it in `Cargo.toml`.

## Attribute Options

### Name (Required)

The first argument is the macro name that users will reference in `@derive()`:

Rust

```
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
```

### Description

Provides documentation for the macro, shown by editors and in the macro manifest:

Rust

```
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
```

### Attributes

Declare which field-level decorators your macro accepts. Each can carry its own description, shown
when a user hovers the decorator:

Rust

```
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(
        debug,                                       // allows @debug(...) on fields
        (redact, "Hides the field's value in toString()"),
    )
)]
pub fn derive_debug(...)
```

Note

Declared attributes become available as `@attributeName({ options })` decorators in TypeScript.

### Kind

`kind` is `"derive"` by default. [`#[ts_macro]`](../../docs/custom-macros/ts-macro) and
[`#[ts_macro_attribute]`](../../docs/custom-macros/ts-macro-attribute) set it to `"call"` and
`"attribute"` for you, so writing it out is rarely needed.

## Function Signature

Rust

```
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>
```

| Parameter             | Description                                                                                                                                      |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mut input: TsStream` | The target's source, with the [macro context](../../docs/custom-macros/context-and-ir) attached                                                  |
| `Result<TsStream, E>` | The generated code, or an error. `E` is usually `MacroforgeError`; any type that converts into a `MacroResult` works, such as `MacroforgeErrors` |

The attribute turns the function into a `Macroforge` implementation named after it (`derive_json`
becomes `DeriveJson`), registers it, and adds the exports a package built with `macroforge build`
needs.

## Parsing Input

Use `parse_ts_macro_input!` to convert the token stream:

Rust

```
use macroforge_ts::ts_syn::{Data, DeriveInput, parse_ts_macro_input};

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.fields();
            // ...
        }
        Data::Interface(interface) => {
            // Handle interfaces
        }
        Data::Enum(_) => {
            // Handle enums (if supported)
        }
        Data::TypeAlias(_) => {
            // Handle type aliases (if supported)
        }
    }
}
```

When the input cannot be parsed, `parse_ts_macro_input!` returns early with a `MacroforgeError`, so
the function's error type must accept one.

## DeriveInput Structure

Rust

```
struct DeriveInput {
    pub ident: Ident,            // The type name and its span
    pub span: SpanIR,            // Span of the type definition
    pub attrs: Vec<Attribute>,   // Decorators (excluding @derive)
    pub data: Data,              // The parsed type data
    pub context: MacroContextIR, // The macro context

    // Helper methods
    fn name(&self) -> &str;                       // The type name
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;
    fn as_type_alias(&self) -> Option<&DataTypeAlias>;
    fn decorator_span(&self) -> SpanIR;           // The whole @derive(...)
    fn macro_name_span(&self) -> Option<SpanIR>;  // This macro's name inside it
    fn error_span(&self) -> SpanIR;               // Where to point an error
    fn target_span(&self) -> SpanIR;              // The declaration
    fn body_span(&self) -> Option<SpanIR>;        // The body; None for enums and type aliases
    fn from_context(ctx: MacroContextIR) -> Result<Self, TsSynError>;
}

struct Attribute {
    fn name(&self) -> &str;   // e.g. "endec"
    fn args(&self) -> &str;   // the arguments as written
    fn span(&self) -> SpanIR;
}

enum Data {
    Class(DataClass),
    Interface(DataInterface),
    Enum(DataEnum),
    TypeAlias(DataTypeAlias),
}

impl DataClass {
    fn fields(&self) -> &[FieldIR];
    fn methods(&self) -> &[MethodSigIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&FieldIR>;
    fn method(&self, name: &str) -> Option<&MethodSigIR>;
    fn body_span(&self) -> SpanIR;      // For inserting code into class body
    fn type_params(&self) -> &[TypeParamIR]; // Generic type parameters
    fn heritage(&self) -> &[String];    // extends/implements clauses
    fn is_abstract(&self) -> bool;
}

impl DataInterface {
    fn fields(&self) -> &[InterfaceFieldIR];
    fn methods(&self) -> &[InterfaceMethodIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
    fn method(&self, name: &str) -> Option<&InterfaceMethodIR>;
    fn body_span(&self) -> SpanIR;
    fn type_params(&self) -> &[TypeParamIR];
    fn heritage(&self) -> &[String];    // extends clauses
}

impl DataEnum {
    fn variants(&self) -> &[EnumVariantIR];
    fn variant_names(&self) -> impl Iterator<Item = &str>;
    fn variant(&self, name: &str) -> Option<&EnumVariantIR>;
}

impl DataTypeAlias {
    fn body(&self) -> &TypeBody;
    fn type_params(&self) -> &[TypeParamIR];
    fn is_union(&self) -> bool;
    fn is_intersection(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_tuple(&self) -> bool;
    fn is_alias(&self) -> bool;
    fn as_union(&self) -> Option<&[TypeMember]>;
    fn as_intersection(&self) -> Option<&[TypeMember]>;
    fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
    fn as_tuple(&self) -> Option<&[String]>;
    fn as_alias(&self) -> Option<&str>;
}
```

Each `Data*` wrapper keeps the full IR in its `inner` field; see
[Context and IR](../../docs/custom-macros/context-and-ir) for every type.

Note

Inside a template, write the type's name with `@{input.name()}`. `input.ident` is a `ts_syn::Ident`,
which records where the name is, and templates do not interpolate it; `ts_ident!(...)` makes the
identifier type that they do.

## Accessing Field Data

### Class Fields (FieldIR)

Rust

```
struct FieldIR {
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
    pub initializer: Option<String>,  // Initializer as written, e.g. "dark"
}
```

### Interface Fields (InterfaceFieldIR)

Rust

```
struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}
```

### Enum Variants (EnumVariantIR)

Rust

```
struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // String(String), Number(f64), Auto or Expr(String)
    pub decorators: Vec<DecoratorIR>,
}
```

### Decorator Structure

Rust

```
struct DecoratorIR {
    pub name: String,      // e.g., "endec"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}
```

Note

To check for decorators, iterate through `field.decorators` and check `decorator.name`. `has_flag`
and `extract_named_string` in `macroforge_ts::builtin::derive::common` read options out of
`args_src`; see [Decorators](../../docs/custom-macros/context-and-ir#decorators).

## Adding Imports

If your macro generates code that requires imports, use the `add_import` method on `TsStream`:

Rust

```
// Add an import to be inserted at the top of the file
let mut output = ts_template!(Within {
    validate(): ValidationResult {
        return validateFields(this);
    }
});

// Adds: import { validateFields } from "my-validation-lib";
//       import type { ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_type_import("ValidationResult", "my-validation-lib");

Ok(output)
```

An import the file already has is not added again. Aliased imports, imports resolved from a type's
module and the rest are in [Output and Imports](../../docs/custom-macros/output#imports).

## Returning Errors

Use `MacroforgeError` to report errors with source locations:

Rust

```
#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(ts_template!(Within { /* ... */ }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }
}
```

Reporting several problems at once, and warnings on success, are covered in
[Errors and Diagnostics](../../docs/custom-macros/diagnostics).

## Complete Example

Rust

```
use macroforge_ts::macros::{ts_macro_derive, ts_template};
use macroforge_ts::ts_syn::{
    Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
};

// Helper function to check if a field has a decorator
fn has_decorator(field: &FieldIR, name: &str) -> bool {
    field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(ts_template!(Within {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}
```

## Next Steps

- [Learn the template syntax](../../docs/custom-macros/ts-quote)
- [Output and Imports](../../docs/custom-macros/output)
- [Context and IR](../../docs/custom-macros/context-and-ir)
- [Type-Aware Macros](../../docs/custom-macros/type-aware)
- [Errors and Diagnostics](../../docs/custom-macros/diagnostics)
- [Testing and Debugging](../../docs/custom-macros/testing-and-debugging)
