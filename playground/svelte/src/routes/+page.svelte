<script lang="ts">
	import { MacroUser, showcaseUserJson, showcaseUserSummary } from '$lib/demo/macro-user'
	import { SvelteAllMacrosTest, svelteTestInstance } from '$lib/demo/all-macros-test'

	const derivedUser = new MacroUser(
		'usr_1001',
		'Cam Solar',
		'Runtime Tinkerer',
		'Derive',
		'2025-01-07',
		'sk-live-token'
	)
	const derivedSummary = derivedUser.toString()
	const derivedUserJson = derivedUser.toJSON()
	const derivedJsonPretty = JSON.stringify(derivedUserJson, null, 2)
	const showcaseJsonPretty = JSON.stringify(showcaseUserJson, null, 2)

	// Test results state
	let testsComplete = false
	let testResults: {
		debug?: string
		clone?: object
		equals?: boolean
		hashCode?: number
		serialize?: object
		deserialize?: object
	} = {}

	function runAllMacroTests() {
		// Test Debug macro -> toString()
		testResults.debug = svelteTestInstance.toString()

		// Test Clone macro -> clone()
		if (typeof (svelteTestInstance as any).clone === 'function') {
			testResults.clone = (svelteTestInstance as any).clone()
		}

		// Test Eq macro -> equals()
		if (typeof (svelteTestInstance as any).equals === 'function') {
			testResults.equals = (svelteTestInstance as any).equals(svelteTestInstance)
		}

		// Test Eq macro -> hashCode()
		if (typeof (svelteTestInstance as any).hashCode === 'function') {
			testResults.hashCode = (svelteTestInstance as any).hashCode()
		}

		// Test Serialize macro -> toJSON()
		testResults.serialize = svelteTestInstance.toJSON()

		// Test Deserialize macro -> fromJSON()
		if (typeof (SvelteAllMacrosTest as any).fromJSON === 'function') {
			const testData = {
				id: 'deser-001',
				title: 'Deserialized',
				content: 'From JSON',
				apiKey: 'key',
				count: 99,
				enabled: false
			}
			testResults.deserialize = (SvelteAllMacrosTest as any).fromJSON(testData)
		}

		// Trigger reactivity
		testResults = testResults
		testsComplete = true
	}
</script>

<main class="page">
	<section>
		<h1>Rust-Powered TypeScript Macros in SvelteKit</h1>
		<p>
			This page is rendered by <code>test-macros-svelte</code> and showcases the local
			<code>macroforge</code> transformer running inside Vite via <code>vite-plugin-napi</code>.
		</p>
	</section>

	<section>
		<h2>Macro Test Panel</h2>
		<div class="test-controls">
			<button onclick={runAllMacroTests} data-testid="test-all-macros">
				Run All Macro Tests
			</button>
		</div>

		<div class="test-results" data-testid="test-results" data-tests-complete={testsComplete}>
			<h3>Test Results</h3>

			<div data-testid="result-debug">
				<strong>Debug (toString):</strong>
				{#if testResults.debug}
					<code>{testResults.debug}</code>
				{:else}
					<em>Click button to run tests</em>
				{/if}
			</div>

			<div data-testid="result-clone">
				<strong>Clone:</strong>
				{#if testResults.clone}
					<pre>{JSON.stringify(testResults.clone, null, 2)}</pre>
				{:else if testsComplete}
					<em>Not available</em>
				{/if}
			</div>

			<div data-testid="result-equals">
				<strong>Equals (self):</strong>
				{#if testResults.equals !== undefined}
					<code>{testResults.equals}</code>
				{:else if testsComplete}
					<em>Not available</em>
				{/if}
			</div>

			<div data-testid="result-hashcode">
				<strong>HashCode:</strong>
				{#if testResults.hashCode !== undefined}
					<code>{testResults.hashCode}</code>
				{:else if testsComplete}
					<em>Not available</em>
				{/if}
			</div>

			<div data-testid="result-serialize">
				<strong>Serialize (toJSON):</strong>
				{#if testResults.serialize}
					<pre>{JSON.stringify(testResults.serialize, null, 2)}</pre>
				{:else if testsComplete}
					<em>Not available</em>
				{/if}
			</div>

			<div data-testid="result-deserialize">
				<strong>Deserialize (fromJSON):</strong>
				{#if testResults.deserialize}
					<pre>{JSON.stringify(testResults.deserialize, null, 2)}</pre>
				{:else if testsComplete}
					<em>Not available</em>
				{/if}
			</div>
		</div>
	</section>

	<section>
		<h2>@derive decorator example</h2>
		<p>
			A <code>MacroUser</code> class is decorated with <code>/** @derive(Debug, JSON) */</code>. The macro
			injects <code>toString()</code> and <code>toJSON()</code>, so we can hydrate typed data without
			manually writing serializers.
		</p>
		<div class="card">
			<h3>toString()</h3>
			<p data-testid="macro-user-summary">{derivedSummary}</p>
			<h3>toJSON()</h3>
			<pre data-testid="macro-user-json">{derivedJsonPretty}</pre>
			<p class="note">
				Field-level <code>@Debug({'{'}{'}'})</code> decorators rename <code>id</code> to <code>userId</code> and skip
				<code>apiToken</code> entirely when printing the summary, while <code>toJSON()</code> still emits the full
				object.
			</p>
		</div>
		<div class="card">
			<h3>Showcase user (module-level)</h3>
			<p>{showcaseUserSummary}</p>
			<pre>{showcaseJsonPretty}</pre>
		</div>
	</section>

	<section>
		<h2>Try it yourself</h2>
		<ol>
			<li>Run <code>npm run dev -w svelte</code>.</li>
			<li>Edit <code>src/lib/demo/macro-snippet.md</code> or <code>macro-user.ts</code> and watch HMR.</li>
			<li>Extend the <code>@playground/macro</code> package and re-export them from Rust.</li>
		</ol>
	</section>
</main>

<style>
	.page {
		margin: 0 auto;
		max-width: 720px;
		padding: 2rem 1.5rem 4rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	pre {
		background: #0f172a;
		color: #e2e8f0;
		padding: 1rem;
		border-radius: 0.5rem;
		overflow-x: auto;
	}

	.card {
		background: #f8fafc;
		border-radius: 0.75rem;
		padding: 1rem 1.5rem;
		box-shadow: 0 10px 35px rgba(15, 23, 42, 0.12);
	}

	.note {
		margin-top: 1rem;
		font-size: 0.95rem;
		color: #475569;
	}

	.test-controls {
		margin: 1rem 0;
	}

	.test-controls button {
		background: #4f46e5;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-size: 1rem;
		cursor: pointer;
	}

	.test-controls button:hover {
		background: #4338ca;
	}

	.test-results {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		padding: 1rem;
		margin: 1rem 0;
	}

	.test-results > div {
		margin: 0.5rem 0;
		padding: 0.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.test-results > div:last-child {
		border-bottom: none;
	}

	code {
		font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
		background: #e2e8f0;
		padding: 0.15rem 0.35rem;
		border-radius: 0.35rem;
	}
</style>
