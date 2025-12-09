<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
	<title>ts_macro_derive - Macroforge Documentation</title>
	<meta name="description" content="Learn how to use the #[ts_macro_derive] attribute to create custom macros." />
</svelte:head>

<h1>ts_macro_derive</h1>

<p class="lead">
	The <code>#[ts_macro_derive]</code> attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.
</p>

<h2 id="basic-syntax">Basic Syntax</h2>

<CodeBlock code={`use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}`} lang="rust" />

<h2 id="attribute-options">Attribute Options</h2>

<h3>Name (Required)</h3>

<p>The first argument is the macro name that users will reference in <code>@derive()</code>:</p>

<CodeBlock code={`#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)`} lang="rust" />

<h3>Description</h3>

<p>Provides documentation for the macro:</p>

<CodeBlock code={`#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)`} lang="rust" />

<h3>Attributes</h3>

<p>Declare which field-level decorators your macro accepts:</p>

<CodeBlock code={`#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(debug)  // Allows @debug({ ... }) on fields
)]
pub fn derive_debug(...)`} lang="rust" />

<Alert type="note">
	<span>Declared attributes become available as <code>@attributeName(&#123; options &#125;)</code> decorators in TypeScript.</span>
</Alert>

<h2 id="function-signature">Function Signature</h2>

<CodeBlock code={`pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>`} lang="rust" />

<table>
	<thead>
		<tr>
			<th>Parameter</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>input: TsStream</code></td>
			<td>Token stream containing the class/interface AST</td>
		</tr>
		<tr>
			<td><code>Result&lt;TsStream, MacroforgeError&gt;</code></td>
			<td>Returns generated code or an error with source location</td>
		</tr>
	</tbody>
</table>

<h2 id="parsing-input">Parsing Input</h2>

<p>Use <code>parse_ts_macro_input!</code> to convert the token stream:</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{DeriveInput, Data, parse_ts_macro_input};

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    // Access class data
    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.fields();
            // ...
        }
        Data::Interface(interface) => {
            // Handle interfaces
        }
        Data::Enum(_) => {
            // Handle enums (if supported)
        }
    }
}`} lang="rust" />

<h2 id="derive-input">DeriveInput Structure</h2>

<CodeBlock code={`struct DeriveInput {
    pub ident: Ident,           // The type name
    pub span: SpanIR,           // Span of the type definition
    pub attrs: Vec<Attribute>,  // Decorators (excluding @derive)
    pub data: Data,             // The parsed type data
    pub context: MacroContextIR, // Macro context with spans

    // Helper methods
    fn name(&self) -> &str;              // Get the type name
    fn decorator_span(&self) -> SpanIR;  // Span of @derive decorator
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;
}

enum Data {
    Class(DataClass),
    Interface(DataInterface),
    Enum(DataEnum),
    TypeAlias(DataTypeAlias),
}

impl DataClass {
    fn fields(&self) -> &[FieldIR];
    fn methods(&self) -> &[MethodSigIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&FieldIR>;
    fn body_span(&self) -> SpanIR;      // For inserting code into class body
    fn type_params(&self) -> &[String]; // Generic type parameters
    fn heritage(&self) -> &[String];    // extends/implements clauses
    fn is_abstract(&self) -> bool;
}

impl DataInterface {
    fn fields(&self) -> &[InterfaceFieldIR];
    fn methods(&self) -> &[InterfaceMethodIR];
    fn field_names(&self) -> impl Iterator<Item = &str>;
    fn field(&self, name: &str) -> Option<&InterfaceFieldIR>;
    fn body_span(&self) -> SpanIR;
    fn type_params(&self) -> &[String];
    fn heritage(&self) -> &[String];    // extends clauses
}

impl DataEnum {
    fn variants(&self) -> &[EnumVariantIR];
    fn variant_names(&self) -> impl Iterator<Item = &str>;
    fn variant(&self, name: &str) -> Option<&EnumVariantIR>;
}

impl DataTypeAlias {
    fn body(&self) -> &TypeBody;
    fn type_params(&self) -> &[String];
    fn is_union(&self) -> bool;
    fn is_object(&self) -> bool;
    fn as_union(&self) -> Option<&[TypeMember]>;
    fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
}`} lang="rust" />

<h2 id="field-data">Accessing Field Data</h2>

<h3>Class Fields (FieldIR)</h3>

<CodeBlock code={`struct FieldIR {
    pub name: String,               // Field name
    pub span: SpanIR,               // Field span
    pub ts_type: String,            // TypeScript type annotation
    pub optional: bool,             // Whether field has ?
    pub readonly: bool,             // Whether field is readonly
    pub visibility: Visibility,     // Public, Protected, Private
    pub decorators: Vec<DecoratorIR>, // Field decorators
}`} lang="rust" />

<h3>Interface Fields (InterfaceFieldIR)</h3>

<CodeBlock code={`struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
    // Note: No visibility field (interfaces are always public)
}`} lang="rust" />

<h3>Enum Variants (EnumVariantIR)</h3>

<CodeBlock code={`struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,  // Auto, String(String), or Number(f64)
    pub decorators: Vec<DecoratorIR>,
}`} lang="rust" />

<h3>Decorator Structure</h3>

<CodeBlock code={`struct DecoratorIR {
    pub name: String,      // e.g., "serde"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}`} lang="rust" />

<Alert type="note">
	<span>To check for decorators, iterate through <code>field.decorators</code> and check <code>decorator.name</code>. For parsing options, you can write helper functions like the built-in macros do.</span>
</Alert>

<h2 id="adding-imports">Adding Imports</h2>

<p>
	If your macro generates code that requires imports, use the <code>add_import</code> method on <code>TsStream</code>:
</p>

<CodeBlock code={`// Add an import to be inserted at the top of the file
let mut output = body! {
    validate(): ValidationResult {
        return validateFields(this);
    }
};

// This will add: import { validateFields, ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_import("ValidationResult", "my-validation-lib");

Ok(output)`} lang="rust" />

<Alert type="note">
	<span>Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again.</span>
</Alert>

<h2 id="returning-errors">Returning Errors</h2>

<p>Use <code>MacroforgeError</code> to report errors with source locations:</p>

<CodeBlock code={`#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(body! { /* ... */ })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }
}`} lang="rust" />

<h2 id="complete-example">Complete Example</h2>

<CodeBlock code={`use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_quote::body;
use macroforge_ts::ts_syn::{
    Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
};

// Helper function to check if a field has a decorator
fn has_decorator(field: &FieldIR, name: &str) -> bool {
    field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(body! {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}`} lang="rust" />

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href="/docs/custom-macros/ts-quote">Learn the ts_quote template syntax</a></li>
</ul>
