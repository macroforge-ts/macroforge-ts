<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
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

<CodeBlock code={`use macroforge_ts::macros::ts_macro_derive;
use macroforge_ts::ts_syn::{TsStream, MacroforgeError};

#[ts_macro_derive(MacroName)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Macro implementation
}`} lang="rust" />

<Alert type="note">
	<span>
		The generated code refers to <code>macroforge_ts</code> by name, so depend on it under that
		name, without renaming it in <code>Cargo.toml</code>.
	</span>
</Alert>

<h2 id="attribute-options">Attribute Options</h2>

<h3>Name (Required)</h3>

<p>The first argument is the macro name that users will reference in <code>@derive()</code>:</p>

<CodeBlock code={`#[ts_macro_derive(JSON)]  // Users write: @derive(JSON)
pub fn derive_json(...)`} lang="rust" />

<h3>Description</h3>

<p>Provides documentation for the macro, shown by editors and in the macro manifest:</p>

<CodeBlock code={`#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(...)`} lang="rust" />

<h3>Attributes</h3>

<p>
	Declare which field-level decorators your macro accepts. Each can carry its own description,
	shown when a user hovers the decorator:
</p>

<CodeBlock code={`#[ts_macro_derive(
    Debug,
    description = "Generates toString()",
    attributes(
        debug,                                       // allows @debug(...) on fields
        (redact, "Hides the field's value in toString()"),
    )
)]
pub fn derive_debug(...)`} lang="rust" />

<Alert type="note">
	<span>Declared attributes become available as <code>@attributeName(&#123; options &#125;)</code> decorators in TypeScript.</span>
</Alert>

<h3>Kind</h3>

<p>
	<code>kind</code> is <code>"derive"</code> by default. <a href={resolve('/docs/custom-macros/ts-macro')}><code>#[ts_macro]</code></a>
	and <a href={resolve('/docs/custom-macros/ts-macro-attribute')}><code>#[ts_macro_attribute]</code></a>
	set it to <code>"call"</code> and <code>"attribute"</code> for you, so writing it out is rarely
	needed.
</p>

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
			<td><code>mut input: TsStream</code></td>
			<td>The target's source, with the <a href={resolve('/docs/custom-macros/context-and-ir')}>macro context</a> attached</td>
		</tr>
		<tr>
			<td><code>Result&lt;TsStream, E&gt;</code></td>
			<td>
				The generated code, or an error. <code>E</code> is usually <code>MacroforgeError</code>;
				any type that converts into a <code>MacroResult</code> works, such as
				<code>MacroforgeErrors</code>
			</td>
		</tr>
	</tbody>
</table>

<p>
	The attribute turns the function into a <code>Macroforge</code> implementation named after it
	(<code>derive_json</code> becomes <code>DeriveJson</code>), registers it, and adds the exports a
	package built with <code>macroforge build</code> needs.
</p>

<h2 id="parsing-input">Parsing Input</h2>

<p>Use <code>parse_ts_macro_input!</code> to convert the token stream:</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{Data, DeriveInput, parse_ts_macro_input};

#[ts_macro_derive(MyMacro)]
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

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
        Data::TypeAlias(_) => {
            // Handle type aliases (if supported)
        }
    }
}`} lang="rust" />

<p>
	When the input cannot be parsed, <code>parse_ts_macro_input!</code> returns early with a
	<code>MacroforgeError</code>, so the function's error type must accept one.
</p>

<h2 id="derive-input">DeriveInput Structure</h2>

<CodeBlock code={`struct DeriveInput {
    pub ident: Ident,            // The type name and its span
    pub span: SpanIR,            // Span of the type definition
    pub attrs: Vec<Attribute>,   // Decorators (excluding @derive)
    pub data: Data,              // The parsed type data
    pub context: MacroContextIR, // The macro context

    // Helper methods
    fn name(&self) -> &str;                       // The type name
    fn as_class(&self) -> Option<&DataClass>;
    fn as_interface(&self) -> Option<&DataInterface>;
    fn as_enum(&self) -> Option<&DataEnum>;
    fn as_type_alias(&self) -> Option<&DataTypeAlias>;
    fn decorator_span(&self) -> SpanIR;           // The whole @derive(...)
    fn macro_name_span(&self) -> Option<SpanIR>;  // This macro's name inside it
    fn error_span(&self) -> SpanIR;               // Where to point an error
    fn target_span(&self) -> SpanIR;              // The declaration
    fn body_span(&self) -> Option<SpanIR>;        // The body; None for enums and type aliases
    fn from_context(ctx: MacroContextIR) -> Result<Self, TsSynError>;
}

struct Attribute {
    fn name(&self) -> &str;   // e.g. "endec"
    fn args(&self) -> &str;   // the arguments as written
    fn span(&self) -> SpanIR;
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
    fn method(&self, name: &str) -> Option<&MethodSigIR>;
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
    fn method(&self, name: &str) -> Option<&InterfaceMethodIR>;
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
    fn is_intersection(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_tuple(&self) -> bool;
    fn is_alias(&self) -> bool;
    fn as_union(&self) -> Option<&[TypeMember]>;
    fn as_intersection(&self) -> Option<&[TypeMember]>;
    fn as_object(&self) -> Option<&[InterfaceFieldIR]>;
    fn as_tuple(&self) -> Option<&[String]>;
    fn as_alias(&self) -> Option<&str>;
}`} lang="rust" />

<p>
	Each <code>Data*</code> wrapper keeps the full IR in its <code>inner</code> field; see
	<a href={resolve('/docs/custom-macros/context-and-ir')}>Context and IR</a> for every type.
</p>

<Alert type="note">
	<span>
		Inside a template, write the type's name with <code>@&#123;input.name()&#125;</code>.
		<code>input.ident</code> is a <code>ts_syn::Ident</code>, which records where the name is, and
		templates do not interpolate it; <code>ts_ident!(...)</code> makes the identifier type that
		they do.
	</span>
</Alert>

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
    pub value: EnumValue,  // String(String), Number(f64), Auto or Expr(String)
    pub decorators: Vec<DecoratorIR>,
}`} lang="rust" />

<h3>Decorator Structure</h3>

<CodeBlock code={`struct DecoratorIR {
    pub name: String,      // e.g., "endec"
    pub args_src: String,  // Raw args text, e.g., "skip, rename: 'id'"
    pub span: SpanIR,
}`} lang="rust" />

<Alert type="note">
	<span>
		To check for decorators, iterate through <code>field.decorators</code> and check
		<code>decorator.name</code>. <code>has_flag</code> and <code>extract_named_string</code> in
		<code>macroforge_ts::builtin::derive_common</code> read options out of
		<code>args_src</code>; see <a href={resolve('/docs/custom-macros/context-and-ir#decorators')}>Decorators</a>.
	</span>
</Alert>

<h2 id="adding-imports">Adding Imports</h2>

<p>
	If your macro generates code that requires imports, use the <code>add_import</code> method on <code>TsStream</code>:
</p>

<CodeBlock code={`// Add an import to be inserted at the top of the file
let mut output = ts_template!(Within {
    validate(): ValidationResult {
        return validateFields(this);
    }
});

// Adds: import { validateFields } from "my-validation-lib";
//       import type { ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_type_import("ValidationResult", "my-validation-lib");

Ok(output)`} lang="rust" />

<p>
	An import the file already has is not added again. Aliased imports, imports resolved from a
	type's module and the rest are in <a href={resolve('/docs/custom-macros/output#imports')}>Output and Imports</a>.
</p>

<h2 id="returning-errors">Returning Errors</h2>

<p>Use <code>MacroforgeError</code> to report errors with source locations:</p>

<CodeBlock code={`#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(ts_template!(Within { /* ... */ }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }
}`} lang="rust" />

<p>
	Reporting several problems at once, and warnings on success, are covered in
	<a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a>.
</p>

<h2 id="complete-example">Complete Example</h2>

<CodeBlock code={`use macroforge_ts::macros::{ts_macro_derive, ts_template};
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

            Ok(ts_template!(Within {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}`} lang="rust" />

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/ts-quote')}>Learn the template syntax</a></li>
	<li><a href={resolve('/docs/custom-macros/output')}>Output and Imports</a></li>
	<li><a href={resolve('/docs/custom-macros/context-and-ir')}>Context and IR</a></li>
	<li><a href={resolve('/docs/custom-macros/type-aware')}>Type-Aware Macros</a></li>
	<li><a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a></li>
	<li><a href={resolve('/docs/custom-macros/testing-and-debugging')}>Testing and Debugging</a></li>
</ul>
