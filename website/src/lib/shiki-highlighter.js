import { createHighlighter as createShikiHighlighter } from 'shiki';

/** @type {ReturnType<typeof createShikiHighlighter> | null} */
let highlighterPromise = null;

/**
 * Parse the meta string from a code fence for flags.
 * e.g., "ts before" -> { lang: 'ts', before: true }
 * @param {string} meta - The meta string after the language
 * @returns {{ before: boolean, after: boolean, macro: boolean, interactive: boolean }}
 */
function parseMeta(meta) {
	const flags = {
		before: false,
		after: false,
		macro: false,
		interactive: false
	};

	if (!meta) return flags;

	const parts = meta.toLowerCase().split(/\s+/);
	flags.before = parts.includes('before');
	flags.after = parts.includes('after');
	flags.macro = parts.includes('macro');
	flags.interactive = parts.includes('interactive');

	return flags;
}

/**
 * Escape HTML special characters for use in attributes.
 * @param {string} str - String to escape
 * @returns {string} - Escaped string
 */
function escapeHtmlAttr(str) {
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&#39;');
}

/**
 * Escape curly braces in HTML for Svelte compatibility.
 * Svelte interprets { and } as expression delimiters.
 * @param {string} html - HTML string to escape
 * @returns {string} - Escaped HTML safe for Svelte
 */
function escapeCurlyBracesForSvelte(html) {
	// Use Svelte's escape syntax: {'{'} and {'}'}
	// But this needs to be done carefully to avoid breaking HTML attributes
	// Instead, we'll replace { and } inside text nodes with HTML entities
	return html
		.replace(/\{/g, '&#123;')
		.replace(/\}/g, '&#125;');
}

/**
 * Get the shared Shiki highlighter instance.
 * @returns {Promise<Awaited<ReturnType<typeof createShikiHighlighter>>>}
 */
export async function getShikiHighlighter() {
	if (!highlighterPromise) {
		highlighterPromise = createShikiHighlighter({
			themes: ['github-dark', 'github-light'],
			langs: ['typescript', 'javascript', 'rust', 'bash', 'shell', 'json', 'toml', 'html', 'css', 'svelte', 'markdown', 'text']
		});
	}
	return highlighterPromise;
}

/**
 * Highlight code using the shared Shiki instance.
 * @param {string} code - Code to highlight
 * @param {string} lang - Language
 * @returns {Promise<string>} - Highlighted HTML with escaped curly braces
 */
export async function highlightCode(code, lang = 'typescript') {
	const highlighter = await getShikiHighlighter();
	const loadedLangs = highlighter.getLoadedLanguages();
	const effectiveLang = loadedLangs.includes(lang) ? lang : 'text';

	let html = highlighter.codeToHtml(code.trim(), {
		lang: effectiveLang,
		themes: {
			dark: 'github-dark',
			light: 'github-light'
		},
		defaultColor: false,
		tabindex: false
	});

	return escapeCurlyBracesForSvelte(html);
}

/**
 * Create a custom mdsvex highlighter using Shiki.
 * @returns {Promise<(code: string, lang: string | undefined, meta: string | undefined) => Promise<string>>}
 */
export async function createHighlighter() {
	const highlighter = await getShikiHighlighter();

	/**
	 * Highlighter function for mdsvex.
	 * @param {string} code - The code to highlight
	 * @param {string | undefined} lang - The language
	 * @param {string | undefined} meta - Meta information (flags)
	 * @returns {Promise<string>} - HTML output
	 */
	return async (code, lang, meta) => {
		const flags = parseMeta(meta || '');
		const language = lang || 'text';

		// Check if we support this language, fall back to text
		const loadedLangs = highlighter.getLoadedLanguages();
		const effectiveLang = loadedLangs.includes(language) ? language : 'text';

		// Generate Shiki HTML with dual themes
		// Using defaultColor: false outputs only CSS variables (no inline colors)
		// This allows pure CSS theme switching without !important overrides
		let html = highlighter.codeToHtml(code.trim(), {
			lang: effectiveLang,
			themes: {
				dark: 'github-dark',
				light: 'github-light'
			},
			defaultColor: false,
			tabindex: false // Disable tabindex to avoid a11y warnings
		});

		// Escape curly braces for Svelte compatibility
		html = escapeCurlyBracesForSvelte(html);

		// For before/after blocks, add data attributes for identification
		// Note: We use escaped HTML attributes, not Svelte expressions
		if (flags.before) {
			return `<div class="macro-code-block" data-macro-type="before" data-lang="${effectiveLang}">${html}</div>`;
		}

		if (flags.after) {
			return `<div class="macro-code-block" data-macro-type="after" data-lang="${effectiveLang}">${html}</div>`;
		}

		if (flags.interactive) {
			// For interactive blocks, we can't easily pass the code through HTML
			// So we use a different approach - wrap in a special class and handle client-side
			return `<div class="interactive-macro-block" data-lang="${effectiveLang}">${html}</div>`;
		}

		// Default: wrap with our CodeBlock styling but use Shiki's highlighted HTML
		return `<div class="code-block-wrapper" data-lang="${effectiveLang}">${html}</div>`;
	};
}
