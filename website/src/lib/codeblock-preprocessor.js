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
 * Detect language for inline code based on content patterns.
 * @param {string} code - The inline code content
 * @returns {string} - Detected language
 */
function detectInlineLanguage(code) {
	// TypeScript patterns - decorators, method signatures, generics
	if (/^@\w+/.test(code)) return 'typescript'; // @derive, @serde
	if (/[():]|=>|<[A-Z]/.test(code)) return 'typescript'; // Function signatures, generics
	if (/\b(string|number|boolean|void|any|null|undefined)\b/.test(code)) return 'typescript';

	// File extensions
	if (/\.(ts|tsx)$/.test(code)) return 'typescript';
	if (/\.(js|jsx)$/.test(code)) return 'javascript';
	if (/\.json$/.test(code)) return 'json';
	if (/\.toml$/.test(code)) return 'toml';
	if (/\.svelte$/.test(code)) return 'svelte';
	if (/\.rs$/.test(code)) return 'rust';
	if (/\.(sh|bash)$/.test(code)) return 'bash';

	// CLI commands
	if (/^(npm|pnpm|yarn|cargo|npx|macroforge)\s/.test(code)) return 'bash';

	// Rust patterns
	if (/^(impl|fn|struct|enum|trait|pub|use|mod|let|mut)\b/.test(code)) return 'rust';
	if (/!$/.test(code) && /^[a-z_]+!$/.test(code)) return 'rust'; // macro!

	// Default to typescript for most code-like content
	return 'typescript';
}

/**
 * Highlight inline code using Shiki with dual theme support.
 * Returns a <code> element without the <pre> wrapper.
 * @param {string} code - The code to highlight
 * @param {string} lang - The language
 * @returns {Promise<string>} - Highlighted HTML wrapped in <code class="shiki-inline">
 */
async function highlightInlineCode(code, lang) {
	const highlighter = await getHighlighter();
	const loadedLangs = highlighter.getLoadedLanguages();
	const effectiveLang = loadedLangs.includes(lang) ? lang : 'text';

	let html = highlighter.codeToHtml(code.trim(), {
		lang: effectiveLang,
		themes: {
			dark: 'github-dark',
			light: 'github-light'
		},
		defaultColor: false
	});

	// Extract just the inner content from <pre...><code>...</code></pre>
	// Shiki outputs: <pre class="shiki" style="..."><code><span>...</span></code></pre>
	const codeMatch = html.match(/<code[^>]*>([\s\S]*?)<\/code>/);
	const innerHtml = codeMatch ? codeMatch[1] : html;

	// Wrap in inline code element with shiki-inline class
	const result = `<code class="shiki-inline">${innerHtml}</code>`;

	return escapeCurlyBracesForSvelte(result);
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
 * Parse an InlineCode component and extract content and lang props.
 * @param {string} componentStr - The full component string
 * @returns {{ content: string | null, lang: string, fullMatch: string, hasHtmlProp: boolean }}
 */
function parseInlineCodeComponent(componentStr) {
	// Check if html prop already exists
	const hasHtmlProp = /\bhtml\s*=/.test(componentStr);

	// Extract lang prop
	const langMatch = componentStr.match(/\blang\s*=\s*(?:"([^"]+)"|{["'`]([^"'`]+)["'`]})/);
	const lang = langMatch ? (langMatch[1] || langMatch[2]) : 'typescript';

	// Extract content between <InlineCode ...> and </InlineCode>
	let content = null;

	// Match content between opening and closing tag
	const contentMatch = componentStr.match(/<InlineCode[^>]*>([^<]*)<\/InlineCode>/);
	if (contentMatch) {
		content = contentMatch[1];
	}

	return { content, lang, fullMatch: componentStr, hasHtmlProp };
}

/**
 * Svelte preprocessor that adds syntax highlighting to CodeBlock and InlineCode components.
 * @returns {import('svelte/compiler').PreprocessorGroup}
 */
export function codeBlockPreprocess() {
	return {
		name: 'codeblock-highlighter',
		/**
		 * @param {{ content: string, filename?: string }} params
		 */
		async markup({ content, filename }) {
			// Skip non-Svelte/mdsvex files and files in node_modules
			const isSvelteFile = filename?.endsWith('.svelte');
			const isMdsvexFile = filename?.endsWith('.svx');
			if (!filename || (!isSvelteFile && !isMdsvexFile) || filename.includes('node_modules')) {
				return;
			}

			// Skip the CodeBlock and InlineCode components themselves
			if (filename.endsWith('CodeBlock.svelte') || filename.endsWith('InlineCode.svelte')) {
				return;
			}

			const replacements = [];

			// Find all CodeBlock usages
			// This regex matches <CodeBlock ... /> or <CodeBlock ...>...</CodeBlock>
			const codeBlockRegex = /<CodeBlock\s+[^>]*?\bcode\s*=[\s\S]*?(?:\/>|<\/CodeBlock>)/g;

			let match;
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
					console.warn(`[codeblock-preprocessor] Failed to highlight CodeBlock in ${filename}:`, err);
				}
			}

			// Find all InlineCode usages with lang prop
			// This regex matches <InlineCode lang="...">content</InlineCode>
			const inlineCodeRegex = /<InlineCode\s+[^>]*?\blang\s*=[^>]*>([^<]*)<\/InlineCode>/g;

			while ((match = inlineCodeRegex.exec(content)) !== null) {
				const componentStr = match[0];
				const { content: codeContent, lang, hasHtmlProp } = parseInlineCodeComponent(componentStr);

				// Skip if already has html prop or content couldn't be extracted
				if (hasHtmlProp || !codeContent) {
					continue;
				}

				try {
					const highlightedHtml = await highlightInlineCode(codeContent, lang);

					// Escape the HTML for use in a Svelte template literal expression
					const escapedHtml = highlightedHtml
						.replace(/\\/g, '\\\\')
						.replace(/`/g, '\\`')
						.replace(/\$\{/g, '\\${');

					// Replace with self-closing tag that has html prop
					// Extract just the opening tag attributes
					const openingMatch = componentStr.match(/<InlineCode([^>]*)>/);
					const attrs = openingMatch ? openingMatch[1] : '';
					const newComponent = `<InlineCode${attrs} html={\`${escapedHtml}\`} />`;

					replacements.push({
						start: match.index,
						end: match.index + componentStr.length,
						replacement: newComponent
					});
				} catch (err) {
					console.warn(`[codeblock-preprocessor] Failed to highlight InlineCode in ${filename}:`, err);
				}
			}

			// Find all plain <code> tags that are NOT inside <pre> tags
			// This handles inline code from markdown backticks and manually written <code> tags
			const plainCodeRegex = /<code>([^<]+)<\/code>/g;

			while ((match = plainCodeRegex.exec(content)) !== null) {
				const fullMatch = match[0];
				const codeContent = match[1];

				// Skip if this <code> is inside a <pre> block
				// Look backwards from match position to find if we're inside <pre>
				const beforeMatch = content.slice(0, match.index);
				const lastPreOpen = beforeMatch.lastIndexOf('<pre');
				const lastPreClose = beforeMatch.lastIndexOf('</pre>');

				// If last <pre> open is after last </pre> close, we're inside a pre block
				if (lastPreOpen > lastPreClose) {
					continue;
				}

				// Skip if already has shiki-inline class
				if (fullMatch.includes('shiki-inline')) {
					continue;
				}

				// Skip empty or whitespace-only content
				if (!codeContent.trim()) {
					continue;
				}

				try {
					const lang = detectInlineLanguage(codeContent);
					const highlightedHtml = await highlightInlineCode(codeContent, lang);

					replacements.push({
						start: match.index,
						end: match.index + fullMatch.length,
						replacement: highlightedHtml
					});
				} catch (err) {
					console.warn(`[codeblock-preprocessor] Failed to highlight inline code in ${filename}:`, err);
				}
			}

			// Apply replacements in reverse order to preserve indices
			if (replacements.length === 0) {
				return;
			}

			// Sort by start position descending
			replacements.sort((a, b) => b.start - a.start);

			let result = content;
			for (const { start, end, replacement } of replacements) {
				result = result.slice(0, start) + replacement + result.slice(end);
			}

			return { code: result };
		}
	};
}

export default codeBlockPreprocess;
