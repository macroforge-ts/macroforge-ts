import { createHighlighter as createShikiHighlighter } from 'shiki';

/** @type {ReturnType<typeof createShikiHighlighter> | null} */
let highlighterPromise = null;

/**
 * Get or create the shared Shiki highlighter instance.
 * @returns {Promise<import('shiki').Highlighter>}
 */
async function getHighlighter() {
	if (!highlighterPromise) {
		highlighterPromise = createShikiHighlighter({
			themes: ['github-dark', 'github-light'],
			langs: ['typescript', 'javascript', 'rust', 'bash', 'shell', 'json', 'toml', 'html', 'css', 'svelte', 'markdown', 'text']
		});
	}
	return highlighterPromise;
}

/**
 * Escape curly braces in HTML for Svelte compatibility.
 * @param {string} html - HTML string to escape
 * @returns {string} - Escaped HTML safe for Svelte
 */
function escapeCurlyBracesForSvelte(html) {
	return html
		.replace(/\{/g, '&#123;')
		.replace(/\}/g, '&#125;');
}

/**
 * Highlight code using Shiki with dual theme support.
 * @param {string} code - The code to highlight
 * @param {string} lang - The language
 * @returns {Promise<string>} - Highlighted HTML
 */
async function highlightCode(code, lang) {
	const highlighter = await getHighlighter();
	const loadedLangs = highlighter.getLoadedLanguages();
	const effectiveLang = loadedLangs.includes(lang) ? lang : 'text';

	let html = highlighter.codeToHtml(code.trim(), {
		lang: effectiveLang,
		themes: {
			dark: 'github-dark',
			light: 'github-light'
		},
		defaultColor: false,
		tabindex: false // Disable tabindex to avoid a11y warnings
	});

	return escapeCurlyBracesForSvelte(html);
}

/**
 * Extract string content from various prop formats.
 * Handles: code="...", code={`...`}, code={'...'}, code={"..."}
 * @param {string} propValue - The raw prop value from the regex match
 * @returns {{ content: string | null, isExpression: boolean }}
 */
function extractStringContent(propValue) {
	if (!propValue) return { content: null, isExpression: false };

	// Handle code="..." (plain string attribute)
	if (propValue.startsWith('"') && propValue.endsWith('"')) {
		return {
			content: propValue.slice(1, -1).replace(/\\"/g, '"').replace(/\\n/g, '\n'),
			isExpression: false
		};
	}

	// Handle code={`...`} (template literal expression)
	if (propValue.startsWith('{`') && propValue.endsWith('`}')) {
		const inner = propValue.slice(2, -2);
		// Check for interpolations - if present, we can't statically extract
		if (inner.includes('${')) {
			return { content: null, isExpression: true };
		}
		return { content: inner, isExpression: false };
	}

	// Handle code={"..."} or code={'...'} (string expression)
	if ((propValue.startsWith('{"') && propValue.endsWith('"}')) ||
		(propValue.startsWith("{\'") && propValue.endsWith("\'}"))) {
		const quote = propValue[1];
		const inner = propValue.slice(2, -2);
		return {
			content: inner.replace(new RegExp(`\\\\${quote}`, 'g'), quote).replace(/\\n/g, '\n'),
			isExpression: false
		};
	}

	// Handle code={...} (other expressions - cannot extract statically)
	if (propValue.startsWith('{') && propValue.endsWith('}')) {
		return { content: null, isExpression: true };
	}

	return { content: null, isExpression: false };
}

/**
 * Parse a CodeBlock component and extract code and lang props.
 * @param {string} componentStr - The full component string
 * @returns {{ code: string | null, lang: string, fullMatch: string, hasHtmlProp: boolean }}
 */
function parseCodeBlockComponent(componentStr) {
	// Check if html prop already exists
	const hasHtmlProp = /\bhtml\s*=/.test(componentStr);

	// Extract lang prop
	const langMatch = componentStr.match(/\blang\s*=\s*(?:"([^"]+)"|{["'`]([^"'`]+)["'`]})/);
	const lang = langMatch ? (langMatch[1] || langMatch[2]) : 'typescript';

	// Extract code prop - this is more complex due to template literals
	// Match code= followed by either "..." or {...}
	let code = null;

	// Try to match code="..."
	const simpleCodeMatch = componentStr.match(/\bcode\s*=\s*"((?:[^"\\]|\\.)*)"/);
	if (simpleCodeMatch) {
		const extracted = extractStringContent(`"${simpleCodeMatch[1]}"`);
		code = extracted.content;
	}

	// Try to match code={`...`} (template literal without interpolation)
	if (!code) {
		const templateMatch = componentStr.match(/\bcode\s*=\s*\{`([\s\S]*?)`\}/);
		if (templateMatch) {
			const inner = templateMatch[1];
			// Only use if no interpolation
			if (!inner.includes('${')) {
				code = inner;
			}
		}
	}

	// Try to match code={"..."} or code={'...'}
	if (!code) {
		const stringExprMatch = componentStr.match(/\bcode\s*=\s*\{(["'])((?:[^\\]|\\.)*?)\1\}/);
		if (stringExprMatch) {
			code = stringExprMatch[2].replace(/\\n/g, '\n');
		}
	}

	return { code, lang, fullMatch: componentStr, hasHtmlProp };
}

/**
 * Svelte preprocessor that adds syntax highlighting to CodeBlock components.
 * @returns {import('svelte/compiler').PreprocessorGroup}
 */
export function codeBlockPreprocess() {
	return {
		name: 'codeblock-highlighter',
		/**
		 * @param {{ content: string, filename?: string }} params
		 */
		async markup({ content, filename }) {
			// Skip non-Svelte files and files in node_modules
			if (!filename || !filename.endsWith('.svelte') || filename.includes('node_modules')) {
				return;
			}

			// Skip the CodeBlock component itself
			if (filename.endsWith('CodeBlock.svelte')) {
				return;
			}

			// Find all CodeBlock usages
			// This regex matches <CodeBlock ... /> or <CodeBlock ...>...</CodeBlock>
			const codeBlockRegex = /<CodeBlock\s+[^>]*?\bcode\s*=[\s\S]*?(?:\/>|<\/CodeBlock>)/g;

			let match;
			const replacements = [];

			while ((match = codeBlockRegex.exec(content)) !== null) {
				const componentStr = match[0];
				const { code, lang, hasHtmlProp } = parseCodeBlockComponent(componentStr);

				// Skip if already has html prop or code couldn't be extracted
				if (hasHtmlProp || !code) {
					continue;
				}

				try {
					const highlightedHtml = await highlightCode(code, lang);

					// Escape the HTML for use in a Svelte template literal expression
					// We use html={`...`} syntax which requires escaping backticks and ${
					const escapedHtml = highlightedHtml
						.replace(/\\/g, '\\\\')
						.replace(/`/g, '\\`')
						.replace(/\$\{/g, '\\${');

					// Insert html prop before the closing /> or >
					let newComponent;
					if (componentStr.endsWith('/>')) {
						newComponent = componentStr.slice(0, -2) + ` html={\`${escapedHtml}\`} />`;
					} else {
						// Has closing tag
						const closingIndex = componentStr.indexOf('>');
						newComponent = componentStr.slice(0, closingIndex) + ` html={\`${escapedHtml}\`}` + componentStr.slice(closingIndex);
					}

					replacements.push({
						start: match.index,
						end: match.index + componentStr.length,
						replacement: newComponent
					});
				} catch (err) {
					console.warn(`[codeblock-preprocessor] Failed to highlight code in ${filename}:`, err);
				}
			}

			// Apply replacements in reverse order to preserve indices
			if (replacements.length === 0) {
				return;
			}

			let result = content;
			for (let i = replacements.length - 1; i >= 0; i--) {
				const { start, end, replacement } = replacements[i];
				result = result.slice(0, start) + replacement + result.slice(end);
			}

			return { code: result };
		}
	};
}

export default codeBlockPreprocess;
