<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Errors and Diagnostics - Macroforge Documentation</title>
	<meta
		name="description"
		content="How a custom macro reports errors, collects several problems at once, and emits warnings."
	/>
</svelte:head>

<h1>Errors and Diagnostics</h1>

<p class="lead">
	A macro reports problems as diagnostics: an error fails the macro, while a warning or note is
	shown alongside its output. Each one points at the source it is about.
</p>

<h2 id="diagnostic">Diagnostic</h2>

<CodeBlock code={`pub struct Diagnostic {
    pub level: DiagnosticLevel,   // Error, Warning or Info
    pub message: String,
    pub span: Option<SpanIR>,     // the source it is about
    pub notes: Vec<String>,       // extra lines of context
    pub help: Option<String>,     // a suggested fix
}`} lang="rust" />

<p>
	Point a diagnostic at the narrowest span that explains it: the field with the bad type, not the
	whole class. For a problem with the macro's use as a whole, <code>input.error_span()</code> is the
	macro's name inside <code>@derive(...)</code>, falling back to the whole decorator.
</p>

<h2 id="one-error">Failing With One Error</h2>

<p>Return a <code>MacroforgeError</code>:</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{Data, MacroforgeError};

match &input.data {
    Data::Class(class) => { /* ... */ }
    _ => {
        return Err(MacroforgeError::new(
            input.error_span(),
            "@derive(Validate) can only be used on classes",
        ));
    }
}

// Not about any one place in the source:
return Err(MacroforgeError::new_global("the macro configuration is missing"));`} lang="rust" />

<h2 id="many-errors">Reporting Every Problem at Once</h2>

<p>
	When several fields can be wrong, report them all in one run rather than one per build. Collect
	them with <code>DiagnosticCollector</code>, then fail if any is an error:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::{DiagnosticCollector, MacroforgeErrors};

let mut diagnostics = DiagnosticCollector::new();
for field in class.fields() {
    if field.ts_type.is_empty() {
        diagnostics.error_with_help(
            field.span,
            format!("field \`{}\` has no type", field.name),
            "add a type annotation",
        );
    }
    if field.ts_type == "any" {
        diagnostics.warning(field.span, format!("field \`{}\` is \`any\`", field.name));
    }
}

if diagnostics.has_errors() {
    return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
}`} lang="rust" />

<p>
	<code>MacroforgeErrors</code> converts into a <code>MacroforgeError</code> that keeps every
	diagnostic, warnings included, so the function can keep returning
	<code>Result&lt;TsStream, MacroforgeError&gt;</code>. A function can also return
	<code>Result&lt;TsStream, MacroforgeErrors&gt;</code> directly.
</p>

<table>
	<thead>
		<tr>
			<th><code>DiagnosticCollector</code> method</th>
			<th>Adds</th>
		</tr>
	</thead>
	<tbody>
		<tr><td><code>error(span, message)</code></td><td>An error</td></tr>
		<tr><td><code>error_with_help(span, message, help)</code></td><td>An error with a suggested fix</td></tr>
		<tr><td><code>warning(span, message)</code></td><td>A warning</td></tr>
		<tr><td><code>push(diagnostic)</code></td><td>Any <code>Diagnostic</code></td></tr>
		<tr><td><code>extend(other)</code></td><td>Everything another collector holds</td></tr>
	</tbody>
</table>

<p>
	<code>has_errors()</code>, <code>is_empty()</code> and <code>len()</code> inspect it, and
	<code>into_vec()</code> hands the diagnostics over.
</p>

<h2 id="warnings">Warnings on Success</h2>

<p>
	When the macro succeeds, report warnings and notes on the stream it returns. With the collector
	above, after the error check:
</p>

<CodeBlock code={`let mut output = ts_template!(Within { /* ... */ });
output.add_diagnostics(diagnostics.into_vec());
Ok(output)`} lang="rust" />

<p>
	<code>add_diagnostic</code> adds a single one. Streams injected with
	<code>&#123;$typescript&#125;</code> or combined with <code>merge</code> keep their diagnostics.
</p>

<h2 id="declarative">Declarative Macro Errors</h2>

<p>
	<a href={resolve('/docs/declarative-macros')}>Declarative macros</a> report their own errors.
	Tooling that builds on <code>macroforge_ts_syn</code>'s declarative parser receives a
	<code>DeclarativeError</code>, which carries a span, a message, an optional
	<code>with_help(...)</code> fix and any number of <code>with_note(...)</code> lines.
</p>

<Alert type="tip">
	<span>
		Run <code>macroforge expand</code> on a file to see a macro's diagnostics with their positions,
		without building the whole project.
	</span>
</Alert>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/output')}>Output and Imports</a></li>
	<li><a href={resolve('/docs/custom-macros/testing-and-debugging')}>Testing and Debugging</a></li>
</ul>
