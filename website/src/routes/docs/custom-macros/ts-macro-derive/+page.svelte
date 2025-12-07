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
    // Get the item name (class/interface name)
    fn name(&self) -> &str;

    // Get the span of the @derive decorator
    fn decorator_span(&self) -> Span;

    // The item data (class, interface, or enum)
    data: Data,
}

enum Data {
    Class(ClassData),
    Interface(InterfaceData),
    Enum(EnumData),
}

struct ClassData {
    fn fields(&self) -> &[FieldData];
    fn field_names(&self) -> impl Iterator<Item = &str>;
}`} lang="rust" />

<h2 id="field-data">Accessing Field Data</h2>

<CodeBlock code={`struct FieldData {
    pub name: String,           // Field name
    pub ts_type: String,        // TypeScript type annotation
    pub decorators: Vec<Decorator>, // Field decorators

    // Check if field has a specific decorator
    fn has_decorator(&self, name: &str) -> bool;

    // Get decorator options as JSON
    fn get_decorator_options(&self, name: &str) -> Option<serde_json::Value>;
}`} lang="rust" />

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
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| f.has_decorator("validate"))
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
