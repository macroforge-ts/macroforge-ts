#!/usr/bin/env node

/**
 * Extract documentation from Rust source files.
 *
 * Parses //! (module docs) and /// (item docs) comments from Rust crates
 * and outputs structured JSON for website/README generation.
 *
 * Usage: node scripts/extract-rust-docs.cjs [output-dir]
 * Output defaults to: website/static/api-data/rust/
 */

const fs = require('fs');
const path = require('path');

const cratesRoot = path.join(__dirname, '..', 'crates');
const defaultOutput = path.join(__dirname, '..', 'website', 'static', 'api-data', 'rust');

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
 * Transform decorator argument syntax from `name = value` to `name: value`.
 * @param {string} args - Decorator arguments
 * @returns {string} - Transformed arguments
 */
function transformDecoratorArgs(args) {
	// Transform `name = "value"` to `name: "value"`
	// Transform `name = value` to `name: value`
	return args
		.replace(/(\w+)\s*=\s*/g, '$1: ')
		// Handle boolean-like flags (e.g., "skip" -> "skip: true")
		.replace(/^(\w+)$/, '$1: true');
}

/**
 * Transform shorthand @derive syntax to proper JSDoc comment syntax.
 * Converts `@derive(...)` to `/** @derive(...) * /` (without space)
 * Also transforms `@serde(...)`, `@debug(...)`, etc.
 * @param {string} code - TypeScript code with shorthand decorators
 * @returns {string} - Code with proper JSDoc comment decorators
 */
function transformToJSDocSyntax(code) {
	// Match @derive(...) and similar decorators at the start of a line
	// Transform them to JSDoc comment style
	return code
		// Handle @derive(...) on its own line
		.replace(/^(\s*)@derive\(([^)]+)\)$/gm, '$1/** @derive($2) */')
		// Handle @serde(...) on its own line - transform args
		.replace(/^(\s*)@serde\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @serde({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @debug(...) on its own line - transform args
		.replace(/^(\s*)@debug\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @debug({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @clone(...) on its own line - transform args
		.replace(/^(\s*)@clone\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @clone({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @hash(...) on its own line - transform args
		.replace(/^(\s*)@hash\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @hash({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @ord(...) on its own line - transform args
		.replace(/^(\s*)@ord\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @ord({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @partialEq(...) on its own line - transform args
		.replace(/^(\s*)@partialEq\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @partialEq({ ${transformDecoratorArgs(args)} }) */`
		)
		// Handle @default(...) on its own line - transform args
		.replace(/^(\s*)@default\(([^)]+)\)$/gm, (_, indent, args) =>
			`${indent}/** @default({ ${transformDecoratorArgs(args)} }) */`
		);
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
			code: transformToJSDocSyntax(rawCode),
			rawCode: rawCode
		});
	}
	return codeBlocks;
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
function main() {
	const outputDir = process.argv[2] || defaultOutput;

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

	// Process builtin macros
	console.log('\nProcessing builtin macros...');
	const builtinMacros = processBuiltinMacros();
	const builtinPath = path.join(outputDir, 'builtin-macros.json');
	fs.writeFileSync(builtinPath, JSON.stringify(builtinMacros, null, 2));
	console.log(`  -> ${builtinPath}`);
	console.log(`     ${Object.keys(builtinMacros).length} macros documented`);

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
}

main();
