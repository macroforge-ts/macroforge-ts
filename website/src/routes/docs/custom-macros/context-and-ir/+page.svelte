<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Context and IR - Macroforge Documentation</title>
	<meta
		name="description"
		content="The macro context and the IR types a custom macro reads: classes, interfaces, enums, type aliases and functions."
	/>
</svelte:head>

<h1>Context and IR</h1>

<p class="lead">
	Every macro runs with a <code>MacroContextIR</code>: what invoked it, where, and a structured view
	(the IR) of the declaration it is attached to. Derive macros usually read the IR through
	<a href={resolve('/docs/custom-macros/ts-macro-derive')}><code>DeriveInput</code></a>; this page
	describes the context and IR types underneath, which attribute macros use directly.
</p>

<h2 id="macro-context">MacroContextIR</h2>

<p>
	A macro gets its context from its input stream with <code>input.context()</code>, or from
	<code>DeriveInput</code>'s <code>context</code> field.
</p>

<CodeBlock code={`pub struct MacroContextIR {
    pub macro_kind: MacroKind,        // Derive, Attribute or Call
    pub macro_name: String,           // e.g. "Debug"
    pub module_path: String,          // the module the macro was imported from
    pub decorator_span: SpanIR,       // the whole @derive(...) or @attr(...)
    pub macro_name_span: Option<SpanIR>, // just the macro's name in it
    pub target_span: SpanIR,          // the declaration the macro is attached to
    pub file_name: String,            // the file being expanded
    pub target: TargetIR,             // the declaration, as IR
    pub target_source: String,        // the declaration's source text
    pub import_registry: ImportRegistry, // the file's imports
    pub config: Option<MacroforgeConfig>, // the project's macroforge.config
    pub type_registry: TypeRegistry,  // every type in the project
    pub resolved_fields: Option<HashMap<String, ResolvedTypeRef>>,
    pub abi_version: u32,
}`} lang="rust" />

<table>
	<thead>
		<tr>
			<th>Method</th>
			<th>Returns</th>
		</tr>
	</thead>
	<tbody>
		<tr><td><code>as_class()</code>, <code>as_interface()</code>, <code>as_enum()</code>, <code>as_type_alias()</code>, <code>as_function()</code></td><td>The target as that kind of IR, or <code>None</code></td></tr>
		<tr><td><code>error_span()</code></td><td>The macro's name if known, else the whole decorator: the place to point an error</td></tr>
		<tr><td><code>import_specifier_for(type_name)</code></td><td>The module the file should import a type from; see <a href={resolve('/docs/custom-macros/type-aware')}>Type-Aware Macros</a></td></tr>
	</tbody>
</table>

<p>
	<code>type_registry</code>, <code>resolved_fields</code> and <code>config</code> are covered in
	<a href={resolve('/docs/custom-macros/type-aware')}>Type-Aware Macros</a>, the import methods in
	<a href={resolve('/docs/custom-macros/output#imports')}>Output and Imports</a>.
</p>

<Alert type="note">
	<span>
		A call macro (<code>$name(...)</code>) gets a minimal context: its target is
		<code>TargetIR::Other</code>, <code>target_source</code> is the text between the parentheses,
		and the file name, config and registries are empty.
	</span>
</Alert>

<h2 id="target">TargetIR</h2>

<CodeBlock code={`pub enum TargetIR {
    Class(ClassIR),
    Interface(InterfaceIR),
    Enum(EnumIR),
    TypeAlias(TypeAliasIR),
    Function(FunctionIR),
    Other,            // a call macro's arguments, or anything unsupported
}`} lang="rust" />

<p>
	An attribute macro on a class method receives the method as a <code>FunctionIR</code>.
</p>

<h2 id="type-parameters">Type parameters</h2>

<p>
    Every generic declaration lists its type parameters as declared. <code>declaration()</code>
    renders one as a generic function would declare it, and <code>declare_all</code> and
    <code>apply_all</code> render a whole list as <code>&lt;T extends Shape&gt;</code> and
    <code>&lt;T&gt;</code>. Variance annotations (<code>in</code>, <code>out</code>) are not
    kept: TypeScript rejects them on a function.
</p>

<CodeBlock code={`pub struct TypeParamIR {
    pub name: String,
    pub constraint: Option<String>,  // the type after \`extends\`
    pub default: Option<String>,     // the type after \`=\`
    pub is_const: bool,
}`} lang="rust" />

<h2 id="classes">Classes</h2>

<CodeBlock code={`pub struct ClassIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,           // the braces and everything between them
    pub is_abstract: bool,
    pub type_params: Vec<TypeParamIR>, // e.g. T, K extends string
    pub heritage: Vec<String>,       // extends and implements clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<FieldIR>,
    pub methods: Vec<MethodSigIR>,
}

pub struct FieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,             // the annotation as written, e.g. "string[]"
    pub optional: bool,              // declared with ?
    pub readonly: bool,
    pub visibility: Visibility,      // Public, Protected or Private
    pub decorators: Vec<DecoratorIR>,
}

pub struct MethodSigIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub params_src: String,          // the parameter list as written
    pub return_type_src: String,
    pub is_static: bool,
    pub is_async: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
    pub body_span: Option<SpanIR>,   // None for an abstract method
    pub body_src: Option<String>,
}`} lang="rust" />

<h2 id="interfaces">Interfaces</h2>

<CodeBlock code={`pub struct InterfaceIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub heritage: Vec<String>,       // extends clauses
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<InterfaceFieldIR>,
    pub methods: Vec<InterfaceMethodIR>,
}

pub struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
}

pub struct InterfaceMethodIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params: Vec<TypeParamIR>,
    pub params_src: String,
    pub return_type_src: String,
    pub optional: bool,
    pub decorators: Vec<DecoratorIR>,
}`} lang="rust" />

<h2 id="enums">Enums</h2>

<CodeBlock code={`pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
    pub is_const: bool,              // declared const enum
}

pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    pub value: EnumValue,
    pub decorators: Vec<DecoratorIR>,
}

pub enum EnumValue {
    String(String),   // Active = "active"
    Number(f64),      // Low = 1
    Auto,             // no initializer: one more than the previous member
    Expr(String),     // Mask = A | B, kept as source text
}`} lang="rust" />

<p>
	<code>EnumValue</code> has a test and an accessor for each kind:
	<code>is_string()</code>/<code>as_string()</code>, <code>is_number()</code>/<code>as_number()</code>,
	<code>is_expr()</code>/<code>as_expr()</code> and <code>is_auto()</code>.
</p>

<h2 id="type-aliases">Type Aliases</h2>

<CodeBlock code={`pub struct TypeAliasIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub type_params: Vec<TypeParamIR>,
    pub body: TypeBody,
}

pub enum TypeBody {
    Union(Vec<TypeMember>),          // A | B | "c"
    Intersection(Vec<TypeMember>),   // A & B
    Object { fields: Vec<InterfaceFieldIR> }, // { a: string }
    Tuple(Vec<String>),              // [string, number]
    Alias(String),                   // Other<T>
    Other(String),                   // anything else, as source text
}

pub struct TypeMember {
    pub kind: TypeMemberKind,
    pub decorators: Vec<DecoratorIR>, // JSDoc tags on a union member
}

pub enum TypeMemberKind {
    Literal(String),                 // "active", 42
    TypeRef(String),                 // User
    Object { fields: Vec<InterfaceFieldIR> },
    Intersection(Vec<TypeMember>),
}`} lang="rust" />

<p>
	<code>TypeBody</code> has <code>is_union()</code>, <code>is_intersection()</code>,
	<code>is_object()</code>, <code>is_tuple()</code> and <code>is_alias()</code>, each with an
	<code>as_...()</code> accessor. <code>TypeMember</code> has <code>is_literal()</code>,
	<code>is_type_ref()</code>, <code>is_object()</code>, their <code>as_...()</code> accessors,
	<code>as_intersection_members()</code>, <code>type_name()</code> (the name of a literal or type
	reference) and <code>has_decorator(name)</code>.
</p>

<h2 id="functions">Functions</h2>

<p>Attribute macros on functions, and on class methods, receive a <code>FunctionIR</code>:</p>

<CodeBlock code={`pub struct FunctionIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub signature_span: SpanIR,      // from the start to the body's opening brace
    pub is_async: bool,
    pub is_generator: bool,
    pub is_exported: bool,
    pub is_default_export: bool,
    pub type_params: Vec<TypeParamIR>,
    pub params: Vec<FunctionParamIR>,
    pub return_type_src: String,
    pub body_src: String,
    pub decorators: Vec<DecoratorIR>,
}

pub struct FunctionParamIR {
    pub name: String,
    pub span: SpanIR,
    pub type_src: String,
    pub default_src: Option<String>,
    pub is_optional: bool,
    pub is_rest: bool,
    pub decorators: Vec<DecoratorIR>,
}`} lang="rust" />

<h2 id="decorators">Decorators</h2>

<CodeBlock code={`pub struct DecoratorIR {
    pub name: String,      // e.g. "endec"
    pub args_src: String,  // the arguments as written, e.g. "skip, rename: \\"id\\""
    pub span: SpanIR,
}`} lang="rust" />

<p>
	A macro reads its options from <code>args_src</code>. The helpers in
	<code>macroforge_ts::builtin::derive::common</code> parse the common shapes:
</p>

<CodeBlock code={`use macroforge_ts::builtin::derive::common::{extract_named_string, has_flag};

for decorator in &field.decorators {
    if decorator.name.eq_ignore_ascii_case("validate") {
        let required = has_flag(&decorator.args_src, "required");
        // rename: "id", rename = "id" or rename("id"); escapes are decoded
        let rename = extract_named_string(&decorator.args_src, "rename");
    }
}`} lang="rust" />

<h2 id="spans">Spans</h2>

<p>
	Every IR node carries a <code>SpanIR</code>. Spans count positions, which are byte offsets plus
	one; see <a href={resolve('/docs/custom-macros/output#spans')}>Spans</a> for reading the text a
	span covers.
</p>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/type-aware')}>Type-Aware Macros</a></li>
	<li><a href={resolve('/docs/custom-macros/output')}>Output and Imports</a></li>
</ul>
