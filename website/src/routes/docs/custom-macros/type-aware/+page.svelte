<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Type-Aware Macros - Macroforge Documentation</title>
	<meta
		name="description"
		content="How a custom macro looks up other types in the project, follows field types to their declarations, and reads the project's config."
	/>
</svelte:head>

<h1>Type-Aware Macros</h1>

<p class="lead">
	Before expanding, Macroforge scans the project and records every class, interface, enum and type
	alias it finds. A macro can look any of them up, so it can generate code that depends on what a
	field's type really is, not just on its name.
</p>

<h2 id="type-registry">The Type Registry</h2>

<p>
	The context's <code>type_registry</code> holds the scan. Each entry is a type's IR, where it is
	declared and the imports of its file:
</p>

<CodeBlock code={`pub struct TypeRegistryEntry {
    pub name: String,                     // "User"
    pub file_path: String,                // where it is declared
    pub is_exported: bool,
    pub definition: TypeDefinitionIR,     // Class, Interface, Enum or TypeAlias IR
    pub file_imports: Vec<FileImportEntry>, // that file's imports
}`} lang="rust" />

<h3>Looking Types Up</h3>

<table>
	<thead>
		<tr>
			<th>Method</th>
			<th>Finds</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>resolve_in_file(name, file, imports)</code></td>
			<td>
				The type <code>name</code> means in <code>file</code>: what it imports under that name, or
				what it declares. Use this one for a name read from a field.
			</td>
		</tr>
		<tr>
			<td><code>resolve(name, imports)</code></td>
			<td>The same, without the same-file check</td>
		</tr>
		<tr>
			<td><code>get(name)</code></td>
			<td>The type by name alone; <code>None</code> when several files declare that name</td>
		</tr>
		<tr>
			<td><code>get_all(name)</code></td>
			<td>Every declaration with that name</td>
		</tr>
		<tr>
			<td><code>get_qualified("src/user.ts::User")</code></td>
			<td>A declaration in a given file</td>
		</tr>
		<tr>
			<td><code>alias_cycle(name)</code></td>
			<td>The loop, if following <code>name</code>'s aliases leads back to it</td>
		</tr>
	</tbody>
</table>

<CodeBlock code={`use macroforge_ts::ts_syn::TypeDefinitionIR;

let ctx = &input.context;
let imports = ctx.import_registry.file_import_entries();
let resolved_fields = ctx.resolved_fields.as_ref();

for field in class.fields() {
    // The field's type with arrays and generics stripped: "User" for "User[]"
    let Some(resolved) = resolved_fields.and_then(|fields| fields.get(&field.name)) else {
        continue;
    };
    let Some(entry) =
        ctx.type_registry.resolve_in_file(&resolved.base_type_name, &ctx.file_name, &imports)
    else {
        continue; // a primitive, a generic parameter, or a type outside the project
    };
    if let TypeDefinitionIR::Enum(enum_ir) = &entry.definition {
        // the field holds one of enum_ir.variants
    }
}`} lang="rust" />

<Alert type="note">
	<span>
		Macroforge records every lookup a macro makes, so when a looked-up type changes, the files
		whose expansion depended on it are expanded again. Look types up through the registry rather
		than reading other files yourself, or a change will not reach your macro's output.
	</span>
</Alert>

<h3>Resolved Field Types</h3>

<p>
	For a derive on a class or interface, <code>resolved_fields</code> has every field's type already
	followed, keyed by field name:
</p>

<CodeBlock code={`pub struct ResolvedTypeRef {
    pub raw_type: String,           // "User[]"
    pub base_type_name: String,     // "User"
    pub registry_key: Option<String>, // the registry entry, when it is a project type
    pub is_collection: bool,        // an array, Set, Map or the like
    pub is_optional: bool,          // | null or | undefined
    pub type_args: Vec<ResolvedTypeRef>, // Map<string, User> → [string, User]
}`} lang="rust" />

<CodeBlock code={`use macroforge_ts::builtin::derive_common::resolved_type_has_derive;

if let Some(resolved) = input.context.resolved_fields.as_ref().and_then(|f| f.get(&field.name)) {
    // Does the field's type, or its element type, also derive Clone?
    if resolved_type_has_derive(&input.context.type_registry, resolved, "Clone") {
        // call the type's generated clone helper instead of copying
    }
}`} lang="rust" />

<h3>Generic Aliases</h3>

<p>
	<code>resolve_generic_aliases</code> expands every generic alias in a type string, so a macro sees
	the shape behind it. With <code>type Link&lt;T&gt; = string | T</code>,
	<code>Link&lt;User&gt;[]</code> becomes <code>(string | User)[]</code>:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::resolve_generic_aliases;

let shape = resolve_generic_aliases(
    &field.ts_type,
    &ctx.type_registry,
    &ctx.file_name,
    &ctx.import_registry.file_import_entries(),
);`} lang="rust" />

<h3>Helpers</h3>

<p><code>macroforge_ts::builtin::derive_common</code> holds the helpers the built-in macros use:</p>

<table>
	<thead>
		<tr>
			<th>Helper</th>
			<th>Use</th>
		</tr>
	</thead>
	<tbody>
		<tr><td><code>type_has_derive(registry, name, "Clone")</code></td><td>Whether a project type derives a macro</td></tr>
		<tr><td><code>resolved_type_has_derive(registry, resolved, "Clone")</code></td><td>The same, for a field's resolved type</td></tr>
		<tr><td><code>collection_element_type(resolved)</code></td><td>The element of <code>User[]</code>, a <code>Set</code>'s value, a <code>Map</code>'s value</td></tr>
		<tr><td><code>standalone_fn_name("User", "Clone")</code></td><td>The name of a generated helper: <code>userClone</code></td></tr>
		<tr><td><code>get_effective_fields(alias, registry)</code></td><td>The fields of an object alias, or of an intersection of them</td></tr>
		<tr><td><code>is_primitive_type</code>, <code>is_numeric_type</code>, <code>is_nullable_type</code>, <code>is_generic_type</code></td><td>Classify a type string</td></tr>
		<tr><td><code>get_type_default</code>, <code>get_type_default_with_registry</code></td><td>A default value expression for a type</td></tr>
	</tbody>
</table>

<h2 id="imports-of-types">Importing a Type's Helpers</h2>

<p>
	Code generated for one type often calls helpers generated beside another, in another module.
	<code>ctx.import_specifier_for("User")</code> gives the module the current file should import from:
	the one it already imports <code>User</code> from, or else the relative path to
	<code>User</code>'s file. The stream methods in
	<a href={resolve('/docs/custom-macros/output#imports')}>Output and Imports</a> build on it.
</p>

<h2 id="config">The Project's Config</h2>

<p>
	<code>ctx.config</code> is the project's <code>macroforge.config</code>, when there is one:
</p>

<CodeBlock code={`pub struct MacroforgeConfig {
    pub keep_decorators: bool,
    pub generate_convenience_const: bool,
    pub foreign_types: Vec<ForeignTypeConfig>,
    pub cfg: CfgFlags,                 // features, target, debug_assertions, custom
    pub deprecated: DeprecatedConfig,
    pub must_use: MustUseConfig,
    pub non_exhaustive: NonExhaustiveConfig,
    pub buildtime: BuildtimeConfig,
    pub config_imports: HashMap<String, ImportInfo>, // the config file's own imports
}`} lang="rust" />

<p>
	<a href={resolve('/docs/endec/foreign-types')}>Foreign types</a> are the usual reason to read it:
	a macro that handles field types itself should treat a configured foreign type, such as
	<code>DateTime.DateTime</code>, the way the project configured it.
</p>

<CodeBlock code={`let foreign = input
    .context
    .config
    .as_ref()
    .and_then(|config| config.foreign_types.iter().find(|ft| ft.name == field.ts_type));

if let Some(foreign) = foreign {
    // foreign.serialize_expr, deserialize_expr, default_expr, has_shape_expr
    // hold the configured expressions, as source text
}`} lang="rust" />

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/context-and-ir')}>Context and IR</a></li>
	<li><a href={resolve('/docs/custom-macros/testing-and-debugging')}>Testing and Debugging</a></li>
</ul>
