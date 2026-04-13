<script lang="ts">
    import CodeBlock from '$lib/components/ui/CodeBlock.svelte';
    import Alert from '$lib/components/ui/Alert.svelte';
</script>

<svelte:head>
    <title>ts_macro_attribute - Macroforge Documentation</title>
    <meta
        name="description"
        content="Define attribute macros with #[ts_macro_attribute]. Invoked via @name decorator on declarations."
    />
</svelte:head>

<h1>ts_macro_attribute</h1>

<p class="lead">
    <code>#[ts_macro_attribute]</code> registers an attribute macro —
    the analog of Rust's <code>#[proc_macro_attribute]</code>. Users
    invoke it with a JSDoc <code>@name</code> decorator on a
    declaration; the macro rewrites the entire declaration.
</p>

<h2 id="basic-syntax">Basic Syntax</h2>

<CodeBlock
    code={`use macroforge_ts::macros::ts_macro_attribute;
use macroforge_ts::ts_syn::{
    MacroforgeError, Patch, PatchCode, TargetIR, TsStream,
};

#[ts_macro_attribute(traced, description = "Count calls to the decorated function")]
pub fn traced_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let ctx = input
        .context()
        .ok_or_else(|| MacroforgeError::new_global("no macro context"))?;

    let TargetIR::Function(f) = &ctx.target else {
        return Err(MacroforgeError::new_global(
            "@traced can only be applied to functions",
        ));
    };

    let full = input.source();
    let open = full.find('{').unwrap();
    let close = full.rfind('}').unwrap();
    let sig = &full[..open];
    let body = &full[open + 1..close].trim();

    let replacement = format!(
        "{sig}{{\\n    (globalThis as any).__traced ??= {{}};\\n    \\
         (globalThis as any).__traced[{name:?}] = \\
         ((globalThis as any).__traced[{name:?}] || 0) + 1;\\n    {body}\\n}}",
        sig = sig,
        name = f.name,
        body = body,
    );

    let mut out = TsStream::from_string(String::new());
    out.runtime_patches.push(Patch::Replace {
        span: f.span,
        code: PatchCode::Text(replacement),
        source_macro: Some("traced".to_string()),
    });
    Ok(out)
}`}
    lang="rust"
/>

<h2 id="call-site">Call Site</h2>

<p>Consumers decorate a declaration with a JSDoc <code>@name</code>:</p>

<CodeBlock
    code={`/** import macro { traced } from "@my/macro-package" */

/** @traced */
export function add(a: number, b: number): number {
    return a + b;
}`}
    lang="ts"
/>

<h2 id="patch-based-output">Patch-Based Output</h2>

<Alert type="warning">
    <p>
        Attribute macros must emit their rewrite as an explicit
        <code>Patch::Replace</code> over the target span. Unlike
        derive macros, the expander does <strong>not</strong>
        auto-convert a returned <code>TsStream</code>'s tokens into
        patches for attribute macros — your macro is responsible for
        describing the edit.
    </p>
</Alert>

<p>Minimal pattern:</p>

<CodeBlock
    code={`let ctx = input.context().ok_or_else(|| ...)?;
let TargetIR::Function(f) = &ctx.target else { ... };

// Build the new declaration text...
let replacement = format!("{sig}{{ /* new body */ }}", sig = signature);

// Emit a Replace patch covering the function's full span:
let mut out = TsStream::from_string(String::new());
out.runtime_patches.push(Patch::Replace {
    span: f.span,
    code: PatchCode::Text(replacement),
    source_macro: Some("my_macro".to_string()),
});
Ok(out)`}
    lang="rust"
/>

<h2 id="target-kinds">Target Kinds</h2>

<p>
    Attribute macros can be applied to any top-level declaration.
    <code>ctx.target</code> is a <code>TargetIR</code> enum; the
    structured <code>IR</code> types give you access to names, spans,
    parameters, fields, etc.
</p>

<table>
    <thead>
        <tr><th>Decoration</th><th>TargetIR variant</th></tr>
    </thead>
    <tbody>
        <tr><td>Function</td><td><code>TargetIR::Function(FunctionIR)</code></td></tr>
        <tr><td>Class</td><td><code>TargetIR::Class(ClassIR)</code></td></tr>
        <tr><td>Interface</td><td><code>TargetIR::Interface(InterfaceIR)</code></td></tr>
        <tr><td>Enum</td><td><code>TargetIR::Enum(EnumIR)</code></td></tr>
        <tr><td>Type alias</td><td><code>TargetIR::TypeAlias(TypeAliasIR)</code></td></tr>
    </tbody>
</table>

<h2 id="attribute-options">Attribute Options</h2>

<h3>name (required)</h3>

<CodeBlock
    code={`#[ts_macro_attribute(reactive)]   // users write /** @reactive */
#[ts_macro_attribute(traced)]     // users write /** @traced */`}
    lang="rust"
/>

<h3>description</h3>

<CodeBlock
    code={`#[ts_macro_attribute(
    reactive,
    description = "Add reactivity tracking to a function"
)]`}
    lang="rust"
/>

<h2 id="no-op-exports">No-op Exports</h2>

<p>
    Each <code>#[ts_macro_attribute]</code> auto-generates a no-op
    void export in the WASM build so TypeScript can resolve the
    decorator name during type-checking:
</p>

<CodeBlock
    code={`// Generated automatically for #[ts_macro_attribute(traced)]
export function traced(): void;`}
    lang="ts"
/>

<h2 id="example-walkthrough">Example Walkthrough</h2>

<p>
    The <code>@traced</code> macro in
    <code>tooling/playground/macro/src/attrs.rs</code> wraps a function
    so every call increments a counter on
    <code>globalThis.__traced[fnName]</code>:
</p>

<CodeBlock
    code={`/** import macro { traced } from "@playground/macro" */

/** @traced */
export function add(a: number, b: number): number {
    return a + b;
}

// Expanded output:
export function add(a: number, b: number): number {
    (globalThis as any).__traced ??= {};
    (globalThis as any).__traced["add"] =
        ((globalThis as any).__traced["add"] || 0) + 1;
    return a + b;
}`}
    lang="ts"
/>

<p>
    The wrapper preserves the original signature (including
    <code>export</code>, <code>async</code>, parameters, and return
    type) and only injects new statements at the top of the body.
</p>
