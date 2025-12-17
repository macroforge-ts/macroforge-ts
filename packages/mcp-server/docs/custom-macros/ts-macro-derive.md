# ts_macro_derive
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[ts_macro_derive]</code> attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*
 ## Basic Syntax
 ```
use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::&#123;TsStream, MacroforgeError&#125;;

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    // Macro implementation
&#125;
``` ## Attribute Options
 ### Name (Required)
 The first argument is the macro name that users will reference in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>:
 ```
#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)
``` ### Description
 Provides documentation for the macro:
 ```
#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)
``` ### Attributes
 Declare which field-level decorators your macro accepts:
 ```
#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(debug)  // Allows @debug(&#123; ... &#125;) on fields
)]
pub fn derive_debug(...)
``` > **Note:** Declared attributes become available as @attributeName(&#123; options&#125;) decorators in TypeScript. ## Function Signature
 ```
pub fn my_macro(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError>
``` | Parameter | Description |
| --- | --- |
| input: TsStream | Token stream containing the class/interface AST |
| Result&lt;TsStream, MacroforgeError&gt; | Returns generated code or an error with source location |
 ## Parsing Input
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">parse_ts_macro_input!</code> to convert the token stream:
 ```
use macroforge_ts::ts_syn::&#123;Data, DeriveInput, parse_ts_macro_input&#125;;

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    // Access class data
    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            let class_name = input.name();
            let fields = class.fields();
            // ...
        &#125;
        Data::Interface(interface) => &#123;
            // Handle interfaces
        &#125;
        Data::Enum(_) => &#123;
            // Handle enums (if supported)
        &#125;
    &#125;
&#125;
``` ## DeriveInput Structure
 ```
struct DeriveInput &#123;
    pub ident: Ident,           // The type name
    pub span: SpanIR,           // Span of the type definition
    pub attrs: Vec&#x3C;Attribute>,  // Decorators (excluding @derive)
    pub data: Data,             // The parsed type data
    pub context: MacroContextIR, // Macro context with spans

    // Helper methods
    fn name(&#x26;self) -> &#x26;str;              // Get the type name
    fn decorator_span(&#x26;self) -> SpanIR;  // Span of @derive decorator
    fn as_class(&#x26;self) -> Option&#x3C;&#x26;DataClass>;
    fn as_interface(&#x26;self) -> Option&#x3C;&#x26;DataInterface>;
    fn as_enum(&#x26;self) -> Option&#x3C;&#x26;DataEnum>;
&#125;

enum Data &#123;
    Class(DataClass),
    Interface(DataInterface),
    Enum(DataEnum),
    TypeAlias(DataTypeAlias),
&#125;

impl DataClass &#123;
    fn fields(&#x26;self) -> &#x26;[FieldIR];
    fn methods(&#x26;self) -> &#x26;[MethodSigIR];
    fn field_names(&#x26;self) -> impl Iterator&#x3C;Item = &#x26;str>;
    fn field(&#x26;self, name: &#x26;str) -> Option&#x3C;&#x26;FieldIR>;
    fn body_span(&#x26;self) -> SpanIR;      // For inserting code into class body
    fn type_params(&#x26;self) -> &#x26;[String]; // Generic type parameters
    fn heritage(&#x26;self) -> &#x26;[String];    // extends/implements clauses
    fn is_abstract(&#x26;self) -> bool;
&#125;

impl DataInterface &#123;
    fn fields(&#x26;self) -> &#x26;[InterfaceFieldIR];
    fn methods(&#x26;self) -> &#x26;[InterfaceMethodIR];
    fn field_names(&#x26;self) -> impl Iterator&#x3C;Item = &#x26;str>;
    fn field(&#x26;self, name: &#x26;str) -> Option&#x3C;&#x26;InterfaceFieldIR>;
    fn body_span(&#x26;self) -> SpanIR;
    fn type_params(&#x26;self) -> &#x26;[String];
    fn heritage(&#x26;self) -> &#x26;[String];    // extends clauses
&#125;

impl DataEnum &#123;
    fn variants(&#x26;self) -> &#x26;[EnumVariantIR];
    fn variant_names(&#x26;self) -> impl Iterator&#x3C;Item = &#x26;str>;
    fn variant(&#x26;self, name: &#x26;str) -> Option&#x3C;&#x26;EnumVariantIR>;
&#125;

impl DataTypeAlias &#123;
    fn body(&#x26;self) -> &#x26;TypeBody;
    fn type_params(&#x26;self) -> &#x26;[String];
    fn is_union(&#x26;self) -> bool;
    fn is_object(&#x26;self) -> bool;
    fn as_union(&#x26;self) -> Option&#x3C;&#x26;[TypeMember]>;
    fn as_object(&#x26;self) -> Option&#x3C;&#x26;[InterfaceFieldIR]>;
&#125;
``` ## Accessing Field Data
 ### Class Fields (FieldIR)
 ```
struct FieldIR &#123;
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec&#x3C;DecoratorIR>, // Field decorators
&#125;
``` ### Interface Fields (InterfaceFieldIR)
 ```
struct InterfaceFieldIR &#123;
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec&#x3C;DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
&#125;
``` ### Enum Variants (EnumVariantIR)
 ```
struct EnumVariantIR &#123;
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // Auto, String(String), or Number(f64)
    pub decorators: Vec&#x3C;DecoratorIR>,
&#125;
``` ### Decorator Structure
 ```
struct DecoratorIR &#123;
    pub name: String,      // e.g., "serde"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
&#125;
``` > **Note:** To check for decorators, iterate through field.decorators and check decorator.name. For parsing options, you can write helper functions like the built-in macros do. ## Adding Imports
 If your macro generates code that requires imports, use the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">add_import</code> method on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">TsStream</code>:
 ```
// Add an import to be inserted at the top of the file
let mut output = body! &#123;
    validate(): ValidationResult &#123;
        return validateFields(this);
    &#125;
&#125;;

// This will add: import &#123; validateFields, ValidationResult &#125; from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_import("ValidationResult", "my-validation-lib");

Ok(output)
``` > **Note:** Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again. ## Returning Errors
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroforgeError</code> to report errors with source locations:
 ```
#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(_) => &#123;
            // Generate code...
            Ok(body! &#123; /* ... */ &#125;)
        &#125;
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    &#125;
&#125;
``` ## Complete Example
 ```
use macroforge_ts::macros::&#123;ts_macro_derive, body&#125;;
use macroforge_ts::ts_syn::&#123;
    Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
&#125;;

// Helper function to check if a field has a decorator
fn has_decorator(field: &#x26;FieldIR, name: &#x26;str) -> bool &#123;
    field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
&#125;

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            let validations: Vec&#x3C;_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(body! &#123;
                validate(): string[] &#123;
                    const errors: string[] = [];
                    &#123;#for field in validations&#125;
                        if (!this.@&#123;field.name&#125;) &#123;
                            errors.push("@&#123;field.name&#125; is required");
                        &#125;
                    &#123;/for&#125;
                    return errors;
                &#125;
            &#125;)
        &#125;
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Validate) only works on classes",
        )),
    &#125;
&#125;
``` ## Next Steps
 - [Learn the template syntax](../../docs/custom-macros/ts-quote)