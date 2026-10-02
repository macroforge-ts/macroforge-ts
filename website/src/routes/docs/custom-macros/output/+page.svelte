<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Output and Imports - Macroforge Documentation</title>
	<meta
		name="description"
		content="What a custom macro returns, where its code goes, and how it adds imports, patches and warnings."
	/>
</svelte:head>

<h1>Output and Imports</h1>

<p class="lead">
	A macro returns a <code>TsStream</code>: generated TypeScript plus everything that travels with
	it, such as imports, patches and warnings. This page covers each part.
</p>

<h2 id="what-a-macro-returns">What a Macro Returns</h2>

<p>
	A function marked with <code>#[ts_macro_derive]</code>, <code>#[ts_macro_attribute]</code> or
	<code>#[ts_macro]</code> returns <code>Result&lt;TsStream, E&gt;</code>. On <code>Ok</code>, the
	host turns the stream into a <code>MacroResult</code>; on <code>Err</code>, the error becomes the
	result's diagnostics. <code>E</code> is anything that converts into a <code>MacroResult</code>,
	usually <code>MacroforgeError</code>; see
	<a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a>.
</p>

<p>What happens to the stream's source depends on the kind of macro:</p>

<table>
	<thead>
		<tr>
			<th>Kind</th>
			<th>The stream's source</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>Derive</td>
			<td>Inserted next to the target, at the stream's <a href="#insert-positions">insert position</a></td>
		</tr>
		<tr>
			<td>Attribute</td>
			<td>Ignored: an attribute macro changes code through <a href="#patches">patches</a></td>
		</tr>
		<tr>
			<td>Call</td>
			<td>Replaces the <code>$name(...)</code> call expression</td>
		</tr>
	</tbody>
</table>

<p>
	Imports, patches, cross-module suffixes and warnings on the stream apply for every kind.
</p>

<h2 id="insert-positions">Insert Positions</h2>

<p>A derive's code goes in one of five places, named by <code>InsertPos</code>:</p>

<CodeBlock code={`// ─── Top ────────────────────────
import { foo } from "./runtime";

// ─── Above ──────────────────────
/** @derive(Debug) */
class User {
    // ─── Within ─────────────────
    name: string;
}
// ─── Below (the default) ────────
export function userToString(value: User): string { ... }

// ─── Bottom ─────────────────────`} lang="typescript" />

<p>
	<code>ts_template!</code> takes the position as its first word, as in
	<code>ts_template!(Within &#123; ... &#125;)</code>; without one the code goes
	<code>Below</code>. A stream built by hand takes it from
	<code>TsStream::with_insert_pos</code>.
</p>

<CodeBlock code={`use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::{InsertPos, TsStream};

// Members of the class body
let members = ts_template!(Within {
    toString(): string { return "User"; }
});

// A function after the class
let standalone = ts_template! {
    export function describeUser(): string { return "a user"; }
};

// The same, built from a string
let by_hand = TsStream::with_insert_pos(
    "export const userVersion = 1;".to_string(),
    InsertPos::Below,
);`} lang="rust" />

<h3>Combining Streams</h3>

<p>
	<code>merge</code> appends one stream to another and keeps the first stream's position;
	<code>TsStream::merge_all</code> folds a list. Everything else on the streams (imports, patches,
	suffixes, warnings) is combined too. Code for two different positions goes in two streams,
	injected into one template with <code>&#123;$typescript&#125;</code>:
</p>

<CodeBlock code={`Ok(ts_template! {
    {$typescript standalone}
    {$typescript members}
})`} lang="rust" />

<p>
	A template with an explicit position starts its code with a marker comment, such as
	<code>/* @macroforge:body */</code> for <code>Within</code>, and the host splits a combined
	stream back up at the markers. Code before the first marker takes the combined stream's own
	position, so put a stream without an explicit position first, as above.
</p>

<h3>Other Stream Methods</h3>

<table>
	<thead>
		<tr>
			<th>Method</th>
			<th>Use</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td><code>TsStream::from_string(source)</code></td>
			<td>A stream of hand-written code, positioned <code>Below</code></td>
		</tr>
		<tr>
			<td><code>source()</code></td>
			<td>The generated code so far</td>
		</tr>
		<tr>
			<td><code>take_source()</code></td>
			<td>Takes the code out, leaving the rest of the stream to merge elsewhere</td>
		</tr>
		<tr>
			<td><code>context()</code></td>
			<td>The <a href={resolve('/docs/custom-macros/context-and-ir')}>macro context</a> the host passed in</td>
		</tr>
		<tr>
			<td><code>parse_stmt(&amp;allocator)</code></td>
			<td>Parses the first statement of the stream with oxc, to inspect generated code</td>
		</tr>
	</tbody>
</table>

<h2 id="imports">Imports</h2>

<p>
	Generated code that calls a helper needs an import for it. The import methods on
	<code>TsStream</code> register the import, and the host writes every requested import at the top
	of the file when the expansion is done. A request is skipped when the file already imports, or
	another macro already requested, the same local name, so asking twice is harmless.
</p>

<h3>Plain Imports</h3>

<CodeBlock code={`// import { validate } from "my-validation-lib";
output.add_import("validate", "my-validation-lib");

// import type { Result } from "my-validation-lib";
output.add_type_import("Result", "my-validation-lib");

// import { validate as runValidate } from "my-validation-lib";
output.add_import("validate as runValidate", "my-validation-lib");`} lang="rust" />

<h3>Aliased Imports</h3>

<p>
	A helper imported under its own name can clash with a name the user's file already uses. An
	alias avoids that:
</p>

<CodeBlock code={`// import { resultOk as __mf_resultOk } from "@my/runtime";
output.add_aliased_import("resultOk", "@my/runtime");
output.add_aliased_type_import("Options", "@my/runtime"); // __mf_Options

// Any alias you choose
output.add_import_as("resultOk", "myResultOk", "@my/runtime");
output.add_type_import_as("Options", "MyOptions", "@my/runtime");`} lang="rust" />

<p>
	Generated code then refers to the alias, as in <code>__mf_resultOk(value)</code>. A macro with a
	fixed set of runtime imports can declare them once:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::ImportConfig;

const RUNTIME_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("resultOk", "__mf_resultOk", "@my/runtime"),
    ImportConfig::type_only("Options", "__mf_Options", "@my/runtime"),
];

output.add_imports(RUNTIME_IMPORTS);`} lang="rust" />

<h3>Imports Resolved From a Type</h3>

<p>
	A helper generated beside a type, such as <code>userValidate</code> next to <code>User</code>,
	lives in the type's module. These methods find that module through the project's
	<a href={resolve('/docs/custom-macros/type-aware')}>type registry</a> and the file's own imports,
	and do nothing when the type is in the same file or unknown:
</p>

<CodeBlock code={`// The module the current file imports \`User\` from, if any
let module: Option<String> = output.module_specifier_for("User");

// import { userValidate } from "<the module of User>";
let added: bool = output.add_import_for("userValidate", "User");
output.add_type_import_for("UserErrors", "User");

// Several helpers from one module, resolving it once: (name, type-only?)
output.add_helpers_for("User", &[("userValidate", false), ("UserErrors", true)]);`} lang="rust" />

<h3 id="cross-module-suffixes">Cross-Module Suffixes</h3>

<p>
	When a macro generates calls to helpers that other macros generate for other types, it can name
	the helpers' suffix instead of resolving each one. With the suffix <code>GetFields</code>
	registered, a generated call to <code>companyNameGetFields()</code> gets
	<code>import &#123; companyNameGetFields &#125;</code> from wherever the file imports
	<code>CompanyName</code>:
</p>

<CodeBlock code={`// {camelCaseType}GetFields(...) calls are imported from the type's module
output.add_cross_module_suffix("GetFields");

// {PascalCaseType}Errors type references get an \`import type\`
output.add_cross_module_type_suffix("Errors");`} lang="rust" />

<h2 id="patches">Patches</h2>

<p>
	A patch edits the user's source directly: inserts code at a position, or replaces or deletes a
	span. Attribute macros work this way, and any macro can add patches to its stream's
	<code>runtime_patches</code>:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{Patch, SpanIR};

// Replace the target with new code
output.runtime_patches.push(Patch::Replace {
    span: ctx.target_span,
    code: rewritten,
    source_macro: Some("traced".to_string()),
});

// Insert before the closing brace of a class body
output.runtime_patches.push(
    macroforge_ts::ts_syn::insert_into_class(class.body_span(), "static version = 1;")
        .with_source_macro("Versioned"),
);

// Delete a span
output.runtime_patches.push(Patch::Delete { span: decorator_span });`} lang="rust" />

<p>
	<code>source_macro</code> names the macro in source maps and in errors about the patch. Patches
	from one expansion must not overlap, and a patch outside the file is an error.
</p>

<h3 id="spans">Spans</h3>

<p>
	A <code>SpanIR</code> marks a range of the file in <em>positions</em>: byte offsets plus one, so
	the first byte of the file is position 1. Spans from the IR are already in positions, and
	<code>source_range()</code> turns one into the 0-based byte range of the file it came from.
</p>

<p>
	A macro sees the target's own text, not the whole file: <code>ctx.target_source</code> is the
	file's text from <code>ctx.target_span.start</code> to <code>ctx.target_span.end</code>. To read
	the text of a span inside the target, count from the target's start:
</p>

<CodeBlock code={`let offset = ctx.target_span.start;
let field_text = &ctx.target_source
    [(field.span.start - offset) as usize..(field.span.end - offset) as usize];`} lang="rust" />

<Alert type="note">
	<span>
		An insertion point is a zero-width span, with <code>start == end</code>. Code inserted at
		position <code>n</code> goes before the byte at offset <code>n - 1</code>.
	</span>
</Alert>

<h2 id="warnings">Warnings</h2>

<p>
	A macro that succeeds can still report something, such as a hint about a likely mistake, with
	<code>add_diagnostic</code>. The expansion goes ahead and the warning is shown with it:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{Diagnostic, DiagnosticLevel};

output.add_diagnostic(Diagnostic {
    level: DiagnosticLevel::Warning,
    message: "field \`id\` has no type; it is serialized as unknown".to_string(),
    span: Some(field.span),
    notes: vec![],
    help: Some("give \`id\` a type annotation".to_string()),
});`} lang="rust" />

<p>
	To fail the macro instead, return an error; see
	<a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a>.
</p>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/ts-quote')}>Template syntax</a> for writing the generated code</li>
	<li><a href={resolve('/docs/custom-macros/context-and-ir')}>Context and IR</a> for reading the target</li>
	<li><a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a></li>
</ul>
