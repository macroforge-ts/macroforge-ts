<script lang="ts">
	import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
	import Alert from '$lib/components/ui/Alert.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Testing and Debugging - Macroforge Documentation</title>
	<meta
		name="description"
		content="How to see a custom macro's output, unit-test it, log from inside it, and register macros by hand."
	/>
</svelte:head>

<h1>Testing and Debugging</h1>

<p class="lead">
	A macro is ordinary Rust, so it can be unit-tested like any function. For a quick look at what it
	generates, run it on a file.
</p>

<h2 id="expand-a-file">Running a Macro on a File</h2>

<p>
	After <code>macroforge build</code>, expand one file of a project that uses the macro and print
	the result:
</p>

<CodeBlock code={`macroforge expand src/user.ts --print`} lang="bash" />

<p>
	Diagnostics are printed with their file, line and column, followed by any notes and help.
	<code>--out</code> writes the expansion to a file instead, and <code>--types-out</code> writes the
	generated type declarations.
</p>

<h2 id="unit-tests">Unit Tests</h2>

<p>
	A test builds the input a macro would get, calls the macro function and checks the stream it
	returns. To build the input, lower a TypeScript snippet to IR and wrap it in a context:
</p>

<CodeBlock code={`use macroforge_ts::ts_syn::abi::{MacroContextIR, SpanIR};
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::oxc::parser::Parser;
use macroforge_ts::ts_syn::oxc::span::SourceType;
use macroforge_ts::ts_syn::{TsStream, lower_classes};

/// The input a derive gets for the first class in \`source\`.
fn class_input(source: &str) -> TsStream {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    let class = lower_classes(&parsed.program, source, None)
        .expect("the class lowers")
        .into_iter()
        .next()
        .expect("the source has a class");
    let context = MacroContextIR::new_derive_class(
        "Validate".to_string(),
        "my-macros".to_string(),
        SpanIR::new(0, 0),
        class.span,
        "test.ts".to_string(),
        class,
        source.to_string(),
    );
    TsStream::with_context(source, "test.ts", context).expect("the input stream builds")
}

#[test]
fn validate_checks_every_field() {
    let output = derive_validate(class_input("class User { name: string; email: string; }"))
        .expect("the macro succeeds");
    assert!(output.source().contains("this.name"));
    assert!(output.source().contains("this.email"));
}`} lang="rust" />

<p>
	<code>lower_interfaces</code>, <code>lower_enums</code>, <code>lower_type_aliases</code> and
	<code>lower_functions</code> lower the other kinds, and <code>MacroContextIR</code> has a matching
	constructor for each: <code>new_derive_interface</code>, <code>new_derive_enum</code>,
	<code>new_derive_type_alias</code>, and <code>new_attribute_class</code>,
	<code>new_attribute_function</code> and the rest for attribute macros.
</p>

<p>
	The context starts with empty imports and an empty type registry. To test a macro that reads
	them, fill them in: <code>context.import_registry.install_source_imports(...)</code> gives the
	file imports, and <code>context.type_registry.insert(...)</code> adds project types.
</p>

<h2 id="debug-logging">Debug Logging</h2>

<p>
	<code>macroforge_ts::debug</code> writes timestamped lines to the project's
	<code>.macroforge/debug.log</code>, in the nearest directory holding a
	<code>macroforge.config</code>:
</p>

<CodeBlock code={`use macroforge_ts::debug;

debug::log("Validate", "starting");
debug::log_ctx("Validate", &input.context);   // macro, file, target and field count
macroforge_ts::debug_log!("Validate", "{} has {} fields", input.name(), class.fields().len());

let result = output.clone().into_result();
debug::log_result("Validate", &result);       // patch, token and diagnostic counts

debug::clear();                               // empty the log`} lang="rust" />

<p>
	A macro package runs in a WebAssembly sandbox with no file access, so its lines travel back with
	its result. When the CLI runs the macro they are written to <code>debug.log</code> as usual; in a
	JavaScript host such as the Vite plugin they are printed to the console instead.
</p>

<h2 id="by-hand">Implementing a Macro by Hand</h2>

<p>
	The <code>#[ts_macro_*]</code> attributes generate an implementation of the
	<code>Macroforge</code> trait, plus the registration and exports a macro package needs. Writing the
	trait yourself is only needed when you embed Macroforge in your own Rust program:
</p>

<CodeBlock code={`use std::sync::Arc;
use macroforge_ts::host::{Macroforge, MacroRegistry, Result};
use macroforge_ts::ts_syn::{MacroKind, MacroResult, TsStream};

struct Stamp;

impl Macroforge for Stamp {
    fn name(&self) -> &str { "Stamp" }
    fn kind(&self) -> MacroKind { MacroKind::Derive }
    fn run(&self, input: TsStream) -> MacroResult {
        let name = input.context().map_or("Unknown", |ctx| ctx.macro_name.as_str());
        TsStream::from_string(format!("export const stampedBy = \"{name}\";")).into_result()
    }
}

fn register(registry: &MacroRegistry) -> Result<()> {
    registry.register("my-macros", "Stamp", Arc::new(Stamp))
}

macroforge_ts::register_macro_package!("my-macros", register);`} lang="rust" />

<Alert type="warning">
	<span>
		<code>register_macro_package!</code> registers macros with the expander in the same program. A
		package built with <code>macroforge build</code> exposes only the macros declared with the
		<code>#[ts_macro_*]</code> attributes.
	</span>
</Alert>

<h2 id="next-steps">Next Steps</h2>

<ul>
	<li><a href={resolve('/docs/custom-macros/diagnostics')}>Errors and Diagnostics</a></li>
	<li><a href={resolve('/docs/custom-macros/output')}>Output and Imports</a></li>
</ul>
