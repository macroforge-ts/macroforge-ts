<script lang="ts">
	import { MacroUser, showcaseUserJson, showcaseUserSummary } from '$lib/demo/macro-user'

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
</script>

<main class="page">
	<section>
		<h1>Rust-Powered TypeScript Macros in SvelteKit</h1>
		<p>
			This page is rendered by <code>test-macros-svelte</code> and showcases the local
			<code>@ts-macros/swc-napi</code> transformer running inside Vite via <code>vite-plugin-napi</code>.
		</p>
	</section>

	<section>
		<h2>@Derive decorator example</h2>
		<p>
			A <code>MacroUser</code> class is decorated with <code>@Derive('Debug', 'JSON')</code>. The macro
			injects <code>toString()</code> and <code>toJSON()</code>, so we can hydrate typed data without
			manually writing serializers.
		</p>
		<div class="card">
			<h3>toString()</h3>
			<p>{derivedSummary}</p>
			<h3>toJSON()</h3>
			<pre>{derivedJsonPretty}</pre>
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

	code {
		font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
		background: #e2e8f0;
		padding: 0.15rem 0.35rem;
		border-radius: 0.35rem;
	}
</style>
