<script lang="ts">
    import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
    import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
    <title>ts_macro - Macroforge Documentation</title>
    <meta
        name="description"
        content="Define function-like (call) macros with #[ts_macro]. Invoked in TypeScript as $name(...)."
    />
</svelte:head>

<h1>ts_macro</h1>

<p class="lead">
    <code>#[ts_macro]</code> registers a function-like macro — the
    TypeScript equivalent of Rust's <code>#[proc_macro]</code>. Users
    invoke it as <code>$name(args)</code> in their source; the macro
    receives the argument text as a <code>TsStream</code> and emits a
    replacement <code>TsStream</code> that takes over the call site.
</p>

<h2 id="basic-syntax">Basic Syntax</h2>

<CodeBlock
    code={`use macroforge_ts::macros::ts_macro;
use macroforge_ts::ts_syn::{MacroforgeError, TsStream};

#[ts_macro(stringify, description = "Quote the argument source as a string literal")]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    Ok(TsStream::from_string(format!("\\"{src}\\"")))
}`}
    lang="rust"
/>

<h2 id="call-site">Call Site</h2>

<p>Consumers invoke the macro with a <code>$</code> prefix:</p>

<CodeBlock
    code={`/** import macro { $stringify } from "@my/macro-package" */

export const quoted = $stringify(1 + 2 * 3);
//    ↓ expands to
export const quoted = "1 + 2 * 3";`}
    lang="ts"
/>

<Alert type="info">
    <p>
        The <code>$</code> prefix at call sites distinguishes macro
        invocations from regular function calls. The prefix is purely
        syntactic — the macro name itself (as declared in
        <code>#[ts_macro(name)]</code>) does not contain it.
    </p>
</Alert>

<h2 id="attribute-options">Attribute Options</h2>

<h3>name (required)</h3>

<p>The first argument is the macro name, a bare identifier:</p>

<CodeBlock
    code={`#[ts_macro(sql)]              // users write $sql(...)
#[ts_macro(stringify)]        // users write $stringify(...)`}
    lang="rust"
/>

<h3>description</h3>

<p>Documentation surfaced in IDE tooling:</p>

<CodeBlock
    code={`#[ts_macro(
    sql,
    description = "Compile-time SQL validation"
)]`}
    lang="rust"
/>

<h3>kind</h3>

<p>
    <code>#[ts_macro]</code> defaults to <code>kind = "call"</code>. You
    can change the kind explicitly (for example to share the same
    macro in multiple positions), but <code>"call"</code> is the
    natural default.
</p>

<CodeBlock
    code={`#[ts_macro(name, kind = "call")]   // default
#[ts_macro(name, kind = "attribute")]  // same as #[ts_macro_attribute]
#[ts_macro(name, kind = "derive")]     // same as #[ts_macro_derive]`}
    lang="rust"
/>

<h2 id="input-output">Input and Output</h2>

<p>
    The input <code>TsStream</code> contains the raw source text of
    the arguments between the parentheses. Access it via
    <code>input.source()</code>:
</p>

<CodeBlock
    code={`#[ts_macro(concat_names)]
pub fn concat_names_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source();                    // e.g. "foo, bar"
    let (left, right) = src.split_once(',').ok_or_else(|| {
        MacroforgeError::new_global("expected two args")
    })?;
    Ok(TsStream::from_string(format!(
        "\\"{}_{}\\"",
        left.trim(),
        right.trim()
    )))
}`}
    lang="rust"
/>

<p>
    Call-macro expansion works by replacing the entire
    <code>$name(...)</code> span with the returned stream's source.
    The engine inlines the output verbatim — no auto-wrapping.
</p>

<h2 id="no-op-exports">No-op Exports</h2>

<p>
    Each <code>#[ts_macro]</code> auto-generates a no-op identity
    function in the WASM and NAPI builds so consumers can
    type-check their code before expansion:
</p>

<CodeBlock
    code={`// Generated automatically for #[ts_macro(stringify)]
export function stringify(value: any): any;
// The \`macroforge build\` CLI also appends:
export { stringify as $stringify };`}
    lang="ts"
/>

<p>
    Consumers can import the <code>$</code>-prefixed alias from the
    generated package and both the runtime (as a passthrough) and the
    compile-time expansion will work.
</p>

<h2 id="import-at-call-site">Importing Call Macros in Consumer Code</h2>

<p>
    Callable macros from external packages are advertised to the
    expander via the <code>import macro</code> JSDoc comment,
    including the <code>$</code> prefix:
</p>

<CodeBlock
    code={`/** import macro { $stringify, $concat_names } from "@my/macros" */

const s = $stringify(1 + 2 * 3);
const k = $concat_names(user, name);`}
    lang="ts"
/>

<Alert type="note">
    <p>
        Unlike the declarative <code>macroRules</code> system, proc
        macros ship inside a Rust crate compiled to WebAssembly.
        Publish the crate with <code>macroforge build</code> so the
        <code>$</code> aliases are exported correctly.
    </p>
</Alert>

<h2 id="examples">Complete Examples</h2>

<h3>$stringify — quote source text</h3>

<CodeBlock
    code={`#[ts_macro(stringify)]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    let escaped = src
        .replace('\\\\', "\\\\\\\\")
        .replace('"', "\\\\\\"");
    Ok(TsStream::from_string(format!("\\"{escaped}\\"")))
}`}
    lang="rust"
/>

<h3>$state — Svelte-runes-style reactive signal</h3>

<CodeBlock
    code={`#[ts_macro(state)]
pub fn state_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let args = input.source().trim();
    Ok(TsStream::from_string(format!("createSignal({})", args)))
}

// Call site: let count = $state(0);
// Expanded:  let count = createSignal(0);`}
    lang="rust"
/>
