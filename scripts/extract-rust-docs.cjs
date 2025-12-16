#!/usr/bin/env node

/**
 * Extract documentation from Rust source files.
 *
 * Parses //! (module docs) and /// (item docs) comments from Rust crates
 * and outputs structured JSON for website/README generation.
 */

const { program } = require('commander');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Try to load macroforge for macro expansion
// Note: macroforge is a dependency of the website package
let expandSync = null;
try {
	// Try loading from website's node_modules
	const websiteNodeModules = path.join(__dirname, '..', 'website', 'node_modules', 'macroforge');
	const macroforge = require(websiteNodeModules);
	expandSync = macroforge.expandSync;
} catch (e) {
	try {
		// Fallback to global/local require
		const macroforge = require('macroforge');
		expandSync = macroforge.expandSync;
	} catch (e2) {
		console.warn('Warning: macroforge not available, macro expansion disabled');
	}
}

const cratesRoot = path.join(__dirname, '..', 'crates');
const defaultOutput = path.join(__dirname, '..', 'website', 'static', 'api-data', 'rust');
const repoRoot = path.join(__dirname, '..');

program
	.name('extract-rust-docs')
	.description('Extract documentation from Rust source files into JSON')
	.argument('[output-dir]', 'Output directory for JSON files', defaultOutput);

// Crates to process (in order)
const CRATES = [
	{ name: 'macroforge_ts', displayName: 'macroforge_ts', entryFile: 'src/lib.rs' },
	{ name: 'macroforge_ts_syn', displayName: 'macroforge_ts_syn', entryFile: 'src/lib.rs' },
	{ name: 'macroforge_ts_quote', displayName: 'macroforge_ts_quote', entryFile: 'src/lib.rs' },
	{ name: 'macroforge_ts_macros', displayName: 'macroforge_ts_macros', entryFile: 'src/lib.rs' }
];

// CLI binary to extract documentation from
const CLI_SOURCE = { name: 'cli', file: 'src/bin/cli.rs', displayName: 'CLI' };

// Builtin macros to extract documentation from
const BUILTIN_MACROS = [
	{ name: 'debug', file: 'src/builtin/derive_debug.rs', displayName: 'Debug' },
	{ name: 'clone', file: 'src/builtin/derive_clone.rs', displayName: 'Clone' },
	{ name: 'default', file: 'src/builtin/derive_default.rs', displayName: 'Default' },
	{ name: 'hash', file: 'src/builtin/derive_hash.rs', displayName: 'Hash' },
	{ name: 'ord', file: 'src/builtin/derive_ord.rs', displayName: 'Ord' },
	{ name: 'partial_eq', file: 'src/builtin/derive_partial_eq.rs', displayName: 'PartialEq' },
	{ name: 'partial_ord', file: 'src/builtin/derive_partial_ord.rs', displayName: 'PartialOrd' },
	{ name: 'serialize', file: 'src/builtin/serde/derive_serialize.rs', displayName: 'Serialize' },
	{ name: 'deserialize', file: 'src/builtin/serde/derive_deserialize.rs', displayName: 'Deserialize' }
];

/**
 * Parse a Cargo.toml file to extract package metadata.
 * @param {string} cargoPath - Path to Cargo.toml
 * @returns {{ name: string, version: string, description: string }}
 */
function parseCargoToml(cargoPath) {
	const content = fs.readFileSync(cargoPath, 'utf-8');
	const result = { name: '', version: '', description: '' };

	// Check if this uses workspace inheritance
	const versionMatch = content.match(/version\.workspace\s*=\s*true/);
	const hasWorkspaceVersion = !!versionMatch;

	// Extract package name
	const nameMatch = content.match(/\[package\][\s\S]*?name\s*=\s*"([^"]+)"/);
	if (nameMatch) result.name = nameMatch[1];

	// Extract version (may be inherited from workspace)
	if (hasWorkspaceVersion) {
		// Read workspace Cargo.toml
		const workspacePath = path.join(cratesRoot, 'Cargo.toml');
		if (fs.existsSync(workspacePath)) {
			const workspaceContent = fs.readFileSync(workspacePath, 'utf-8');
			const wsVersionMatch = workspaceContent.match(
				/\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/
			);
			if (wsVersionMatch) result.version = wsVersionMatch[1];
		}
	} else {
		const localVersionMatch = content.match(/\[package\][\s\S]*?version\s*=\s*"([^"]+)"/);
		if (localVersionMatch) result.version = localVersionMatch[1];
	}

	// Extract description
	const descMatch = content.match(/description\s*=\s*"([^"]+)"/);
	if (descMatch) result.description = descMatch[1];

	return result;
}

/**
 * Extract //! module-level documentation from Rust source.
 * @param {string} source - Rust source code
 * @returns {string} - Module documentation as markdown
 */
function extractModuleDocs(source) {
	const lines = source.split('\n');
	const docLines = [];
	let inModuleDoc = true;

	for (const line of lines) {
		const trimmed = line.trim();

		// Module docs start with //!
		if (trimmed.startsWith('//!')) {
			// Extract content after //! (preserving leading space if present)
			const content = trimmed.slice(3);
			docLines.push(content.startsWith(' ') ? content.slice(1) : content);
		} else if (trimmed === '' && docLines.length > 0 && inModuleDoc) {
			// Allow blank lines within module docs
			docLines.push('');
		} else if (!trimmed.startsWith('//') && trimmed !== '') {
			// Non-comment, non-empty line ends module docs
			inModuleDoc = false;
			break;
		}
	}

	// Trim trailing empty lines
	while (docLines.length > 0 && docLines[docLines.length - 1] === '') {
		docLines.pop();
	}

	return docLines.join('\n');
}

/**
 * Extract item-level documentation and signature.
 * @param {string} source - Rust source code
 * @returns {Array<{name: string, kind: string, signature: string, description: string, sections: Object}>}
 */
function extractItemDocs(source) {
	const items = [];
	const lines = source.split('\n');

	let currentDoc = [];
	let i = 0;

	while (i < lines.length) {
		const line = lines[i];
		const trimmed = line.trim();

		// Skip module-level docs
		if (trimmed.startsWith('//!')) {
			i++;
			continue;
		}

		// Collect /// doc comments
		if (trimmed.startsWith('///')) {
			const content = trimmed.slice(3);
			currentDoc.push(content.startsWith(' ') ? content.slice(1) : content);
			i++;
			continue;
		}

		// If we have accumulated docs, look for the item definition
		if (currentDoc.length > 0) {
			// Skip attributes like #[napi], #[derive], etc.
			if (trimmed.startsWith('#[')) {
				i++;
				continue;
			}

			// Skip visibility modifiers and find the item
			const item = parseItemSignature(lines, i);
			if (item) {
				const { description, sections } = parseDocSections(currentDoc);
				items.push({
					name: item.name,
					kind: item.kind,
					signature: item.signature,
					description,
					sections
				});
			}

			currentDoc = [];
		}

		i++;
	}

	return items;
}

/**
 * Parse an item signature from source lines.
 * @param {string[]} lines - All source lines
 * @param {number} startIdx - Starting line index
 * @returns {{ name: string, kind: string, signature: string } | null}
 */
function parseItemSignature(lines, startIdx) {
	let signatureLines = [];
	let i = startIdx;

	// Collect lines until we find a complete signature (ends with { or ;)
	while (i < lines.length) {
		const line = lines[i];
		const trimmed = line.trim();

		// Skip attributes
		if (trimmed.startsWith('#[')) {
			i++;
			continue;
		}

		signatureLines.push(trimmed);

		if (trimmed.endsWith('{') || trimmed.endsWith(';') || trimmed.endsWith('}')) {
			break;
		}
		i++;
	}

	const fullSignature = signatureLines.join(' ').replace(/\s+/g, ' ').trim();

	// Parse different item types
	// pub fn name(...)
	const fnMatch = fullSignature.match(/^pub\s+(?:async\s+)?fn\s+(\w+)/);
	if (fnMatch) {
		// Extract up to the opening brace
		const sig = fullSignature.replace(/\s*\{.*$/, '');
		return { name: fnMatch[1], kind: 'function', signature: sig };
	}

	// pub struct Name
	const structMatch = fullSignature.match(/^pub\s+struct\s+(\w+)/);
	if (structMatch) {
		// Include generic parameters if present
		const sig = fullSignature.match(/^pub\s+struct\s+\w+(?:<[^>]+>)?/)?.[0] || structMatch[0];
		return { name: structMatch[1], kind: 'struct', signature: sig };
	}

	// pub enum Name
	const enumMatch = fullSignature.match(/^pub\s+enum\s+(\w+)/);
	if (enumMatch) {
		return { name: enumMatch[1], kind: 'enum', signature: enumMatch[0] };
	}

	// pub type Name
	const typeMatch = fullSignature.match(/^pub\s+type\s+(\w+)/);
	if (typeMatch) {
		return { name: typeMatch[1], kind: 'type', signature: fullSignature.replace(/\s*;$/, '') };
	}

	// pub trait Name
	const traitMatch = fullSignature.match(/^pub\s+trait\s+(\w+)/);
	if (traitMatch) {
		return { name: traitMatch[1], kind: 'trait', signature: traitMatch[0] };
	}

	// pub mod name
	const modMatch = fullSignature.match(/^pub\s+mod\s+(\w+)/);
	if (modMatch) {
		return { name: modMatch[1], kind: 'module', signature: modMatch[0] };
	}

	// impl blocks - for NAPI-exported methods
	const implMatch = fullSignature.match(/^impl\s+(\w+)/);
	if (implMatch) {
		return { name: implMatch[1], kind: 'impl', signature: implMatch[0] };
	}

	return null;
}

/**
 * Parse doc comment sections like # Arguments, # Returns, etc.
 * @param {string[]} docLines - Lines of documentation
 * @returns {{ description: string, sections: Object }}
 */
function parseDocSections(docLines) {
	const sections = {};
	let currentSection = null;
	let currentContent = [];
	let description = [];

	for (const line of docLines) {
		// Check for section headers like # Arguments, # Returns, # Example
		const sectionMatch = line.match(/^#\s+(.+)$/);
		if (sectionMatch) {
			// Save previous section
			if (currentSection) {
				sections[currentSection] = currentContent.join('\n').trim();
			}
			currentSection = sectionMatch[1].toLowerCase().replace(/\s+/g, '_');
			currentContent = [];
		} else if (currentSection) {
			currentContent.push(line);
		} else {
			description.push(line);
		}
	}

	// Save last section
	if (currentSection) {
		sections[currentSection] = currentContent.join('\n').trim();
	}

	return {
		description: description.join('\n').trim(),
		sections
	};
}

/**
 * Process a single crate and extract its documentation.
 * @param {Object} crateConfig - Crate configuration
 * @returns {Object} - Structured API documentation
 */
function processCrate(crateConfig) {
	const cratePath = path.join(cratesRoot, crateConfig.name);
	const cargoPath = path.join(cratePath, 'Cargo.toml');
	const entryPath = path.join(cratePath, crateConfig.entryFile);

	if (!fs.existsSync(entryPath)) {
		console.warn(`Warning: Entry file not found: ${entryPath}`);
		return null;
	}

	const cargoMeta = fs.existsSync(cargoPath) ? parseCargoToml(cargoPath) : {};
	const source = fs.readFileSync(entryPath, 'utf-8');

	const moduleDocs = extractModuleDocs(source);
	const items = extractItemDocs(source);

	// Filter to only public, documented items
	const publicItems = items.filter((item) => item.description || Object.keys(item.sections).length > 0);

	return {
		name: crateConfig.displayName,
		kind: 'rust_crate',
		version: cargoMeta.version || 'unknown',
		description: cargoMeta.description || '',
		overview: moduleDocs,
		items: publicItems.map((item) => ({
			name: item.name,
			kind: item.kind,
			signature: item.signature,
			description: item.description,
			params: parseParams(item.sections.arguments),
			returns: parseReturns(item.sections.returns),
			examples: parseExamples(item.sections.example || item.sections.examples),
			errors: item.sections.errors,
			panics: item.sections.panics,
			notes: item.sections.note || item.sections.notes
		}))
	};
}

/**
 * Parse # Arguments section into structured params.
 * @param {string} content - Arguments section content
 * @returns {Array<{name: string, type: string, description: string}>}
 */
function parseParams(content) {
	if (!content) return undefined;

	const params = [];
	// Match patterns like: * `name` - Description
	const paramRegex = /\*\s*`(\w+)`\s*-\s*(.+)/g;
	let match;
	while ((match = paramRegex.exec(content))) {
		params.push({
			name: match[1],
			type: '', // Type info would need to come from signature parsing
			description: match[2].trim()
		});
	}
	return params.length > 0 ? params : undefined;
}

/**
 * Parse # Returns section.
 * @param {string} content - Returns section content
 * @returns {{ type: string, description: string }}
 */
function parseReturns(content) {
	if (!content) return undefined;
	return {
		type: '', // Would need signature parsing
		description: content.trim()
	};
}

/**
 * Parse # Example section.
 * @param {string} content - Example section content
 * @returns {string[]}
 */
function parseExamples(content) {
	if (!content) return undefined;

	// Extract code blocks
	const examples = [];
	const codeBlockRegex = /```(?:\w+)?\n([\s\S]*?)```/g;
	let match;
	while ((match = codeBlockRegex.exec(content))) {
		examples.push(match[1].trim());
	}
	return examples.length > 0 ? examples : undefined;
}


/**
 * Extract code blocks from markdown content.
 * @param {string} content - Markdown content with code blocks
 * @returns {Array<{lang: string, code: string, rawCode: string}>} - Extracted code blocks
 */
function extractCodeBlocks(content) {
	const codeBlocks = [];
	const regex = /```(\w+)?\n([\s\S]*?)```/g;
	let match;
	while ((match = regex.exec(content))) {
		const rawCode = match[2].trim();
		codeBlocks.push({
			lang: match[1] || 'typescript',
			code: rawCode,
			rawCode: rawCode
		});
	}
	return codeBlocks;
}

// Track formatting errors globally
const formatErrors = [];

/**
 * Format TypeScript code using Biome CLI.
 * @param {string} code - Code to format
 * @param {string} [context] - Optional context for error messages
 * @returns {string} - Formatted code
 */
function formatCode(code, context = 'unknown') {
	try {
		const result = execSync('npx @biomejs/biome format --stdin-file-path=example.ts', {
			input: code,
			encoding: 'utf-8',
			maxBuffer: 10 * 1024 * 1024,
			cwd: path.join(__dirname, '..', 'website'),
			stdio: ['pipe', 'pipe', 'pipe']
		});
		return result.trim();
	} catch (e) {
		const stderr = e.stderr?.toString() || '';
		if (stderr.includes('parsing errors')) {
			formatErrors.push({ context, code: code.slice(0, 200) + '...' });
			console.error(`\n❌ Parse error in ${context}:`);
			console.error(stderr.split('\n').slice(0, 5).join('\n'));
		}
		return code.trim();
	}
}

/**
 * Check if there were any formatting errors and exit if so.
 */
function checkFormatErrors() {
	if (formatErrors.length > 0) {
		console.error(`\n\n${'='.repeat(60)}`);
		console.error(`❌ FAILED: ${formatErrors.length} file(s) have parsing errors`);
		console.error('='.repeat(60));
		for (const err of formatErrors) {
			console.error(`  - ${err.context}`);
		}
		console.error('\nFix the parsing errors in the doc comments before publishing.');
		process.exit(1);
	}
}

/**
 * Check if code contains @derive decorator (the main trigger for macro expansion).
 * Field-level decorators like @hash, @serde, etc. should NOT trigger expansion by themselves.
 * @param {string} code - Code to check
 * @returns {boolean} - True if code contains @derive decorator
 */
function containsMacroDecorators(code) {
	// Only @derive should trigger macro expansion
	// Field-level decorators (@hash, @serde, @debug, etc.) are just configuration
	// and shouldn't cause a "Generated output:" section to be added
	return (
		/@derive\s*\(/.test(code) ||
		/\/\*\*[\s\S]*?@derive/.test(code)
	);
}

/**
 * Expand macro code blocks in markdown content.
 * Transforms single code blocks with macros into before/after pairs.
 * @param {string} markdown - Markdown content
 * @returns {string} - Markdown with expanded macro code blocks
 */
function expandMacroCodeBlocks(markdown) {
	if (!expandSync) {
		// macroforge not available, return unchanged
		return markdown;
	}

	const codeBlockRegex = /```(\w+)?\n([\s\S]*?)```/g;

	return markdown.replace(codeBlockRegex, (match, lang, code) => {
		const trimmedCode = code.trim();

		// Check if this code block contains macro decorators
		if (!containsMacroDecorators(trimmedCode)) {
			// Not a macro example, return unchanged
			return match;
		}

		// Check if language is TypeScript/JavaScript
		const language = (lang || 'typescript').toLowerCase();
		if (!['typescript', 'ts', 'javascript', 'js'].includes(language)) {
			return match;
		}

		try {
			// Expand the macro
			const result = expandSync(trimmedCode, 'example.ts');
			let expandedCode = result.code;

			// Remove the macroforge import line if present
			expandedCode = expandedCode.replace(
				/^import\s+\{[^}]+\}\s+from\s+['"]macroforge['"];\s*\n?/m,
				''
			);

			// Format both codes
			const formattedBefore = formatCode(trimmedCode, 'macro code block (before)');
			const formattedAfter = formatCode(expandedCode, 'macro code block (after)');

			// Return before/after pair with flags
			return `\`\`\`${lang || 'typescript'} before\n${formattedBefore}\n\`\`\`\n\n\`\`\`${lang || 'typescript'} after\n${formattedAfter}\n\`\`\``;
		} catch (e) {
			console.warn(`  Warning: Failed to expand macro in code block: ${e.message}`);
			// Return original code with interactive flag for client-side expansion
			return `\`\`\`${lang || 'typescript'} interactive\n${trimmedCode}\n\`\`\``;
		}
	});
}

/**
 * Sync expanded macro output back to Rust doc comments.
 * For each code example containing @derive, expands the macro and
 * adds the generated output as a separate code block in the doc comment.
 */
function syncExpandedOutputToRustDocs() {
	if (!expandSync) {
		console.warn('  Warning: macroforge not available, skipping doc sync');
		return;
	}

	console.log('\nSyncing expanded output to Rust doc comments...');

	for (const macro of BUILTIN_MACROS) {
		const filePath = path.join(cratesRoot, 'macroforge_ts', macro.file);

		if (!fs.existsSync(filePath)) {
			console.warn(`  Warning: File not found: ${filePath}`);
			continue;
		}

		const source = fs.readFileSync(filePath, 'utf-8');
		const lines = source.split('\n');
		const newLines = [];
		let i = 0;
		let modified = false;

		while (i < lines.length) {
			const line = lines[i];
			const trimmed = line.trim();

			// Check if we're at the start of a code block in doc comments
			if (trimmed.startsWith('//!') && trimmed.includes('```typescript')) {
				// Collect the entire code block
				const codeBlockLines = [line];
				i++;

				while (i < lines.length) {
					const blockLine = lines[i];
					codeBlockLines.push(blockLine);

					if (blockLine.trim().startsWith('//!') && blockLine.trim().includes('```')) {
						// End of code block
						break;
					}
					i++;
				}

				// Extract the code from the block
				const codeContent = codeBlockLines
					.slice(1, -1) // Remove opening and closing ```
					.map((l) => {
						const content = l.trim().slice(3); // Remove //!
						return content.startsWith(' ') ? content.slice(1) : content;
					})
					.join('\n');

				// Check if this is a macro example (contains @derive)
				if (containsMacroDecorators(codeContent)) {
					// Skip any existing "Generated output:" section that follows
					// We'll regenerate it fresh
					let j = i + 1;
					while (j < lines.length) {
						const nextLine = lines[j].trim();
						// Empty doc comment lines before Generated output
						if (nextLine === '//!') {
							j++;
							continue;
						}
						// Found start of Generated output section - skip until we exit the code block
						if (nextLine.startsWith('//!') && nextLine.includes('Generated output:')) {
							j++; // Skip "Generated output:" line
							// Skip any blank lines
							while (j < lines.length && lines[j].trim() === '//!') {
								j++;
							}
							// Skip the code block if present
							if (j < lines.length && lines[j].includes('```')) {
								j++; // Skip opening ```
								while (j < lines.length && !lines[j].includes('```')) {
									j++; // Skip code lines
								}
								if (j < lines.length) {
									j++; // Skip closing ```
								}
							}
							// Update i to skip these lines in the main loop
							i = j - 1; // -1 because the loop will increment
							modified = true;
							break;
						}
						// Hit something else (next section, different content), stop looking
						break;
					}

					// Add the original code block
					newLines.push(...codeBlockLines);

					try {
						// Expand the macro
						const result = expandSync(codeContent, 'example.ts');
						let expandedCode = result.code;

						// Remove macroforge imports
						expandedCode = expandedCode.replace(
							/^import\s+\{[^}]+\}\s+from\s+['"]macroforge(?:\/utils)?['"];\s*\n?/gm,
							''
						);

						// Format the expanded code
						const formattedExpanded = formatCode(expandedCode, `${macro.name} generated output`);

						// Add the "Generated output:" section
						newLines.push('//!');
						newLines.push('//! Generated output:');
						newLines.push('//!');
						newLines.push('//! ```typescript');
						for (const codeLine of formattedExpanded.split('\n')) {
							newLines.push(`//! ${codeLine}`);
						}
						newLines.push('//! ```');

						modified = true;
						console.log(`  -> Regenerated output for ${macro.name}`);
					} catch (e) {
						console.warn(`  Warning: Failed to expand ${macro.name}: ${e.message}`);
					}
				} else {
					// Not a macro example, just add as-is
					newLines.push(...codeBlockLines);
				}
			} else {
				newLines.push(line);
			}

			i++;
		}

		if (modified) {
			fs.writeFileSync(filePath, newLines.join('\n'));
		}
	}
}

/**
 * Parse module docs into structured sections for builtin macros.
 * @param {string} moduleDocs - Raw module documentation
 * @returns {Object} - Parsed sections
 */
function parseModuleDocSections(moduleDocs) {
	const lines = moduleDocs.split('\n');
	const sections = {
		title: '',
		description: '',
		generatedOutput: '',
		fieldOptions: '',
		example: '',
		exampleCode: [], // Extracted code blocks
		raw: moduleDocs
	};

	let currentSection = 'description';
	let currentContent = [];

	for (const line of lines) {
		// Check for ## headers
		const h2Match = line.match(/^##\s+(.+)$/);
		const h1Match = line.match(/^#\s+(.+)$/);

		if (h1Match) {
			sections.title = h1Match[1];
			continue;
		}

		if (h2Match) {
			// Save previous section
			if (currentContent.length > 0) {
				sections[currentSection] = currentContent.join('\n').trim();
			}

			// Determine new section
			const header = h2Match[1].toLowerCase();
			if (header.includes('generated') || header.includes('output')) {
				currentSection = 'generatedOutput';
			} else if (header.includes('field') || header.includes('option') || header.includes('decorator')) {
				currentSection = 'fieldOptions';
			} else if (header.includes('example')) {
				currentSection = 'example';
			} else {
				currentSection = header.replace(/\s+/g, '_').replace(/[^a-z_]/g, '');
			}
			currentContent = [];
			continue;
		}

		currentContent.push(line);
	}

	// Save last section
	if (currentContent.length > 0) {
		sections[currentSection] = currentContent.join('\n').trim();
	}

	// Extract code blocks from the example section
	if (sections.example) {
		sections.exampleCode = extractCodeBlocks(sections.example);
	}

	return sections;
}

/**
 * Process builtin macro files and extract their documentation.
 * @returns {Object} - Map of macro name to documentation
 */
function processBuiltinMacros() {
	const macros = {};

	for (const macro of BUILTIN_MACROS) {
		const filePath = path.join(cratesRoot, 'macroforge_ts', macro.file);

		if (!fs.existsSync(filePath)) {
			console.warn(`  Warning: Builtin macro file not found: ${filePath}`);
			continue;
		}

		const source = fs.readFileSync(filePath, 'utf-8');
		const moduleDocs = extractModuleDocs(source);
		const sections = parseModuleDocSections(moduleDocs);

		macros[macro.name] = {
			name: macro.displayName,
			slug: macro.name.replace(/_/g, '-'),
			description: sections.description,
			generatedOutput: sections.generatedOutput,
			fieldOptions: sections.fieldOptions,
			example: sections.example,
			exampleCode: sections.exampleCode,
			title: sections.title,
			raw: moduleDocs
		};
	}

	return macros;
}

function stripLeadingH1(md) {
	return md.replace(/^#\s+.*?\n+/, '');
}

function escapeComparisonsInHtmlTableCells(md) {
	// Some Rust docs contain HTML tables with comparison operators like `false < true`.
	// In HTML text nodes, `<` must be escaped, otherwise Svelte will try to parse it as a tag.
	return md.replace(/(<t[dh][^>]*>)([\s\S]*?)(<\/t[dh]>)/gi, (_m, open, inner, close) => {
		const escaped = inner
			.replace(/\s<\s/g, ' &lt; ')
			.replace(/\s>\s/g, ' &gt; ')
			.replace(/\s<=\s/g, ' &lt;= ')
			.replace(/\s>=\s/g, ' &gt;= ');
		return `${open}${escaped}${close}`;
	});
}

function escapeComparisonsInMarkdownTables(md) {
	const lines = md.split('\n');
	let inCodeBlock = false;

	const escaped = lines.map((line) => {
		const trimmed = line.trim();
		if (trimmed.startsWith('```')) {
			inCodeBlock = !inCodeBlock;
			return line;
		}

		if (inCodeBlock) {
			return line;
		}

		if (!trimmed.startsWith('|')) {
			return line;
		}

		return line
			.replace(/\s<\s/g, ' &lt; ')
			.replace(/\s>\s/g, ' &gt; ')
			.replace(/\s<=\s/g, ' &lt;= ')
			.replace(/\s>=\s/g, ' &gt;= ');
	});

	return escaped.join('\n');
}

function normalizeBuiltinMacroMarkdown(macro) {
	const raw = escapeComparisonsInMarkdownTables(escapeComparisonsInHtmlTableCells(macro.raw));

	// The Rust module docs already include a title H1. Replace it with a stable one.
	const withTitle = raw.replace(/^#\s+.*$/m, `# ${macro.name}`);

	// Ensure the page starts with a single H1, followed by content.
	const body = stripLeadingH1(withTitle).trim();
	return `# ${macro.name}\n\n${body}\n`;
}

function writeBuiltinMacroMarkdownPages(builtinMacros) {
	const websiteDocsRoot = path.join(repoRoot, 'website', 'src', 'routes', 'docs', 'builtin-macros');
	const mcpDocsRoot = path.join(repoRoot, 'packages', 'mcp-server', 'docs', 'builtin-macros');

	for (const key of Object.keys(builtinMacros)) {
		const macro = builtinMacros[key];
		const slug = macro.slug;
		const title = macro.name;
		const description =
			(macro.description && macro.description.trim()) || `${title} derive macro documentation.`;

		let mdBody = normalizeBuiltinMacroMarkdown(macro);

		// Expand macro code blocks for the website version
		const expandedMdBody = expandMacroCodeBlocks(mdBody);

		// Website mdsvex route (with expanded macros)
		const websiteDir = path.join(websiteDocsRoot, slug);
		fs.mkdirSync(websiteDir, { recursive: true });
		const websitePagePath = path.join(websiteDir, '+page.svx');
		const websitePage = `<!--\n  Generated by node scripts/extract-rust-docs.cjs.\n  Do not edit manually — edit the Rust module docs instead.\n-->\n\n<svelte:head>\n\t<title>${title} Macro - Macroforge Documentation</title>\n\t<meta name=\"description\" content=\"${description.replace(/\"/g, '&quot;')}\" />\n</svelte:head>\n\n${expandedMdBody}`;
		fs.writeFileSync(websitePagePath, websitePage);

		// MCP markdown doc (plain markdown without expansion, no Svelte/HTML)
		fs.mkdirSync(mcpDocsRoot, { recursive: true });
		const mcpPath = path.join(mcpDocsRoot, `${slug}.md`);
		fs.writeFileSync(mcpPath, mdBody);
	}
}

/**
 * Parse CLI module docs into structured sections.
 * @param {string} moduleDocs - Raw module documentation
 * @returns {Object} - Parsed CLI documentation
 */
function parseCliDocSections(moduleDocs) {
	const lines = moduleDocs.split('\n');
	const sections = {
		title: '',
		description: '',
		commands: {},
		outputFileNaming: '',
		exitCodes: '',
		nodeIntegration: '',
		raw: moduleDocs
	};

	let currentSection = 'description';
	let currentCommand = null;
	let currentContent = [];
	let inCodeBlock = false;

	for (const line of lines) {
		// Track code block state to avoid matching headers inside code blocks
		if (line.trim().startsWith('```')) {
			inCodeBlock = !inCodeBlock;
			currentContent.push(line);
			continue;
		}

		// Skip header matching inside code blocks
		if (inCodeBlock) {
			currentContent.push(line);
			continue;
		}

		// Check for ## headers (main sections)
		const h2Match = line.match(/^##\s+(.+)$/);
		const h3Match = line.match(/^###\s+(.+)$/);
		const h1Match = line.match(/^#\s+(.+)$/);

		if (h1Match && !sections.title) {
			sections.title = h1Match[1];
			continue;
		}

		if (h2Match) {
			// Save previous section/command
			if (currentCommand) {
				sections.commands[currentCommand] = currentContent.join('\n').trim();
			} else if (currentContent.length > 0) {
				sections[currentSection] = currentContent.join('\n').trim();
			}

			currentCommand = null;
			currentContent = [];

			// Determine new section
			const header = h2Match[1].toLowerCase();
			if (header.includes('command')) {
				currentSection = 'commands';
			} else if (header.includes('output') && header.includes('naming')) {
				currentSection = 'outputFileNaming';
			} else if (header.includes('exit')) {
				currentSection = 'exitCodes';
			} else if (header.includes('node')) {
				currentSection = 'nodeIntegration';
			} else {
				currentSection = header.replace(/\s+/g, '_').replace(/[^a-z_]/g, '');
			}
			continue;
		}

		if (h3Match && currentSection === 'commands') {
			// Save previous command
			if (currentCommand) {
				sections.commands[currentCommand] = currentContent.join('\n').trim();
			}

			// Extract command name (e.g., "`macroforge expand`" -> "expand")
			const cmdMatch = h3Match[1].match(/`macroforge\s+(\w+)`/);
			currentCommand = cmdMatch ? cmdMatch[1] : h3Match[1].toLowerCase().replace(/[^a-z]/g, '');
			currentContent = [];
			continue;
		}

		currentContent.push(line);
	}

	// Save last section/command
	if (currentCommand) {
		sections.commands[currentCommand] = currentContent.join('\n').trim();
	} else if (currentContent.length > 0) {
		sections[currentSection] = currentContent.join('\n').trim();
	}

	return sections;
}

/**
 * Process the CLI binary documentation.
 * @returns {Object} - CLI documentation
 */
function processCliDocs() {
	const filePath = path.join(cratesRoot, 'macroforge_ts', CLI_SOURCE.file);

	if (!fs.existsSync(filePath)) {
		console.warn(`  Warning: CLI source file not found: ${filePath}`);
		return null;
	}

	const source = fs.readFileSync(filePath, 'utf-8');
	const moduleDocs = extractModuleDocs(source);
	const sections = parseCliDocSections(moduleDocs);
	const items = extractItemDocs(source);

	// Get version from main crate
	const cargoPath = path.join(cratesRoot, 'macroforge_ts', 'Cargo.toml');
	const cargoMeta = fs.existsSync(cargoPath) ? parseCargoToml(cargoPath) : {};

	return {
		name: CLI_SOURCE.displayName,
		version: cargoMeta.version || 'unknown',
		title: sections.title,
		description: sections.description,
		commands: sections.commands,
		outputFileNaming: sections.outputFileNaming,
		exitCodes: sections.exitCodes,
		nodeIntegration: sections.nodeIntegration,
		functions: items.filter((item) => item.kind === 'function').map((item) => ({
			name: item.name,
			signature: item.signature,
			description: item.description,
			params: parseParams(item.sections.arguments),
			returns: parseReturns(item.sections.returns)
		})),
		raw: moduleDocs
	};
}

/**
 * Main entry point.
 */
function main(outputDir) {

	// Ensure output directory exists
	fs.mkdirSync(outputDir, { recursive: true });

	console.log('Extracting Rust documentation...\n');

	const allDocs = [];

	for (const crate of CRATES) {
		console.log(`Processing ${crate.name}...`);
		const docs = processCrate(crate);
		if (docs) {
			allDocs.push(docs);

			// Write individual crate file
			const outPath = path.join(outputDir, `${crate.name}.json`);
			fs.writeFileSync(outPath, JSON.stringify(docs, null, 2));
			console.log(`  -> ${outPath}`);
			console.log(`     ${docs.items.length} documented items`);
		}
	}

	// Sync expanded output to Rust doc comments first
	syncExpandedOutputToRustDocs();

	// Process builtin macros
	console.log('\nProcessing builtin macros...');
	const builtinMacros = processBuiltinMacros();
	const builtinPath = path.join(outputDir, 'builtin-macros.json');
	fs.writeFileSync(builtinPath, JSON.stringify(builtinMacros, null, 2));
	console.log(`  -> ${builtinPath}`);
	console.log(`     ${Object.keys(builtinMacros).length} macros documented`);

	// Also write mdsvex pages + MCP docs from the same source markdown.
	writeBuiltinMacroMarkdownPages(builtinMacros);

	// Process CLI documentation
	console.log('\nProcessing CLI documentation...');
	const cliDocs = processCliDocs();
	if (cliDocs) {
		const cliPath = path.join(outputDir, 'cli.json');
		fs.writeFileSync(cliPath, JSON.stringify(cliDocs, null, 2));
		console.log(`  -> ${cliPath}`);
		console.log(`     ${Object.keys(cliDocs.commands).length} commands documented`);
	}

	// Write combined index file
	const indexPath = path.join(outputDir, 'index.json');
	fs.writeFileSync(
		indexPath,
		JSON.stringify(
			{
				generated: new Date().toISOString(),
				crates: allDocs.map((d) => ({
					name: d.name,
					version: d.version,
					itemCount: d.items.length
				})),
				builtinMacros: Object.keys(builtinMacros),
				cli: cliDocs ? { version: cliDocs.version, commandCount: Object.keys(cliDocs.commands).length } : null
			},
			null,
			2
		)
	);

	console.log(`\nWrote index: ${indexPath}`);
	console.log(`\nTotal: ${allDocs.reduce((sum, d) => sum + d.items.length, 0)} documented items`);
	console.log(`Total: ${Object.keys(builtinMacros).length} builtin macros`);
	if (cliDocs) {
		console.log(`Total: ${Object.keys(cliDocs.commands).length} CLI commands`);
	}

	// Check for any formatting errors and fail if found
	checkFormatErrors();
}

program.action(main);
program.parse();
