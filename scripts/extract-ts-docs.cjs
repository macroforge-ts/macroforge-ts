#!/usr/bin/env node

/**
 * Extract documentation from TypeScript source files.
 *
 * Parses JSDoc comments from TypeScript packages
 * and outputs structured JSON for website/README generation.
 */

const { program } = require('commander');
const fs = require('fs');
const path = require('path');

const packagesRoot = path.join(__dirname, '..', 'packages');
const defaultOutput = path.join(__dirname, '..', 'website', 'static', 'api-data', 'typescript');

program
	.name('extract-ts-docs')
	.description('Extract documentation from TypeScript source files into JSON')
	.argument('[output-dir]', 'Output directory for JSON files', defaultOutput);

// Packages to process (in order)
const PACKAGES = [
	{
		name: 'shared',
		displayName: '@macroforge/shared',
		entryFile: 'src/index.ts'
	},
	{
		name: 'typescript-plugin',
		displayName: '@macroforge/typescript-plugin',
		entryFile: 'src/index.ts'
	},
	{ name: 'vite-plugin', displayName: '@macroforge/vite-plugin', entryFile: 'src/index.ts' },
	{
		name: 'svelte-preprocessor',
		displayName: '@macroforge/svelte-preprocessor',
		entryFile: 'src/index.ts'
	},
	{ name: 'mcp-server', displayName: '@macroforge/mcp-server', entryFile: 'src/index.ts' }
];

/**
 * Parse a package.json file to extract package metadata.
 * @param {string} pkgPath - Path to package.json
 * @returns {{ name: string, version: string, description: string }}
 */
function parsePackageJson(pkgPath) {
	const content = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
	return {
		name: content.name || '',
		version: content.version || '',
		description: content.description || ''
	};
}

/**
 * Extract @fileoverview JSDoc from the beginning of a file.
 * @param {string} source - TypeScript source code
 * @returns {string} - File overview documentation
 */
function extractFileOverview(source) {
	// Match the first JSDoc block that contains @fileoverview or @module
	const fileDocMatch = source.match(/\/\*\*[\s\S]*?(@fileoverview|@module)[\s\S]*?\*\//);
	if (!fileDocMatch) return '';

	const docBlock = fileDocMatch[0];

	// Extract the main content (excluding tags)
	const lines = docBlock
		.split('\n')
		.map((line) => line.replace(/^\s*\*\s?/, '').trim())
		.filter((line) => !line.startsWith('/') && line !== '*');

	// Find the fileoverview content
	const result = [];
	let inMainDoc = false;
	let skipLine = false;

	for (const line of lines) {
		// Skip @fileoverview and @module tags themselves
		if (line.startsWith('@fileoverview') || line.startsWith('@module')) {
			const afterTag = line.replace(/^@(fileoverview|module)\s*/, '');
			if (afterTag) result.push(afterTag);
			inMainDoc = true;
			continue;
		}

		// Stop at other tags
		if (line.startsWith('@') && !line.startsWith('@example')) {
			break;
		}

		if (inMainDoc || result.length > 0) {
			result.push(line);
		}
	}

	return result.join('\n').trim();
}

/**
 * Parse a JSDoc block and extract structured information.
 * @param {string} docBlock - The JSDoc block including /** and * /
 * @returns {Object} - Parsed JSDoc structure
 */
function parseJSDoc(docBlock) {
	const lines = docBlock
		.split('\n')
		.map((line) => line.replace(/^\s*\*\s?/, '').trim())
		.filter((line) => !line.startsWith('/') && line !== '');

	const result = {
		description: [],
		params: [],
		returns: null,
		examples: [],
		remarks: null,
		see: [],
		internal: false,
		deprecated: null
	};

	let currentTag = null;
	let currentContent = [];

	for (const line of lines) {
		// Check for @tags
		const tagMatch = line.match(/^@(\w+)\s*(.*)?$/);
		if (tagMatch) {
			// Save previous tag content
			if (currentTag) {
				saveTagContent(result, currentTag, currentContent);
			}
			currentTag = tagMatch[1];
			currentContent = tagMatch[2] ? [tagMatch[2]] : [];
		} else if (currentTag) {
			currentContent.push(line);
		} else {
			result.description.push(line);
		}
	}

	// Save last tag
	if (currentTag) {
		saveTagContent(result, currentTag, currentContent);
	}

	result.description = result.description.join('\n').trim();
	return result;
}

/**
 * Save tag content to the result object.
 */
function saveTagContent(result, tag, content) {
	const text = content.join('\n').trim();

	switch (tag) {
		case 'param': {
			// Parse @param {type} name - description
			const paramMatch = text.match(/^(?:\{([^}]+)\}\s+)?(\w+)\s*-?\s*([\s\S]*)?$/);
			if (paramMatch) {
				result.params.push({
					name: paramMatch[2],
					type: paramMatch[1] || '',
					description: (paramMatch[3] || '').trim()
				});
			}
			break;
		}
		case 'returns':
		case 'return': {
			// Parse @returns {type} description
			const returnMatch = text.match(/^(?:\{([^}]+)\}\s*)?([\s\S]*)$/);
			if (returnMatch) {
				result.returns = {
					type: returnMatch[1] || '',
					description: (returnMatch[2] || '').trim()
				};
			}
			break;
		}
		case 'example': {
			result.examples.push(text);
			break;
		}
		case 'remarks': {
			result.remarks = text;
			break;
		}
		case 'see': {
			result.see.push(text);
			break;
		}
		case 'internal': {
			result.internal = true;
			break;
		}
		case 'deprecated': {
			result.deprecated = text || true;
			break;
		}
	}
}

/**
 * Extract function/class documentation from TypeScript source.
 * @param {string} source - TypeScript source code
 * @returns {Array<Object>} - Array of documented items
 */
function extractItemDocs(source) {
	const items = [];
	// Match JSDoc blocks followed by exports/functions/classes/types
	const docBlockRegex =
		/\/\*\*[\s\S]*?\*\/\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?(function|class|type|interface|const|let|var)\s+(\w+)/g;

	let match;
	while ((match = docBlockRegex.exec(source)) !== null) {
		const fullMatch = match[0];
		const kind = match[1];
		const name = match[2];

		// Extract the JSDoc block
		const docMatch = fullMatch.match(/\/\*\*[\s\S]*?\*\//);
		if (!docMatch) continue;

		const parsed = parseJSDoc(docMatch[0]);

		// Skip internal items
		if (parsed.internal) continue;

		// Extract signature (simplified)
		const afterDoc = fullMatch.slice(docMatch[0].length);
		let signature = afterDoc.trim();

		// For functions, try to get the full signature
		if (kind === 'function') {
			const funcStart = source.indexOf(match[0]) + docMatch[0].length;
			const funcEnd = source.indexOf('{', funcStart);
			if (funcEnd > funcStart) {
				signature = source.slice(funcStart, funcEnd).trim();
			}
		}

		items.push({
			name,
			kind: kind === 'const' || kind === 'let' || kind === 'var' ? 'variable' : kind,
			signature,
			description: parsed.description,
			params: parsed.params.length > 0 ? parsed.params : undefined,
			returns: parsed.returns,
			examples: parsed.examples.length > 0 ? parsed.examples : undefined,
			remarks: parsed.remarks,
			see: parsed.see.length > 0 ? parsed.see : undefined,
			deprecated: parsed.deprecated
		});
	}

	return items;
}

/**
 * Process a single package and extract its documentation.
 * @param {Object} pkgConfig - Package configuration
 * @returns {Object} - Structured API documentation
 */
function processPackage(pkgConfig) {
	const pkgPath = path.join(packagesRoot, pkgConfig.name);
	const pkgJsonPath = path.join(pkgPath, 'package.json');
	const entryPath = path.join(pkgPath, pkgConfig.entryFile);

	if (!fs.existsSync(entryPath)) {
		console.warn(`Warning: Entry file not found: ${entryPath}`);
		return null;
	}

	const pkgMeta = fs.existsSync(pkgJsonPath) ? parsePackageJson(pkgJsonPath) : {};
	const source = fs.readFileSync(entryPath, 'utf-8');

	const fileOverview = extractFileOverview(source);
	const items = extractItemDocs(source);

	return {
		name: pkgConfig.displayName,
		kind: 'typescript_package',
		version: pkgMeta.version || 'unknown',
		description: pkgMeta.description || '',
		overview: fileOverview,
		items
	};
}

/**
 * Main entry point.
 */
function main(outputDir) {

	// Ensure output directory exists
	fs.mkdirSync(outputDir, { recursive: true });

	console.log('Extracting TypeScript documentation...\n');

	const allDocs = [];

	for (const pkg of PACKAGES) {
		console.log(`Processing ${pkg.name}...`);
		const docs = processPackage(pkg);
		if (docs) {
			allDocs.push(docs);

			// Write individual package file
			const outPath = path.join(outputDir, `${pkg.name}.json`);
			fs.writeFileSync(outPath, JSON.stringify(docs, null, 2));
			console.log(`  -> ${outPath}`);
			console.log(`     ${docs.items.length} documented items`);
			if (docs.overview) {
				console.log(`     Has file overview (${docs.overview.length} chars)`);
			}
		}
	}

	// Write combined index file
	const indexPath = path.join(outputDir, 'index.json');
	fs.writeFileSync(
		indexPath,
		JSON.stringify(
			{
				generated: new Date().toISOString(),
				packages: allDocs.map((d) => ({
					name: d.name,
					version: d.version,
					itemCount: d.items.length,
					hasOverview: !!d.overview
				}))
			},
			null,
			2
		)
	);

	console.log(`\nWrote index: ${indexPath}`);
	console.log(`\nTotal: ${allDocs.reduce((sum, d) => sum + d.items.length, 0)} documented items`);
}

program.action(main);
program.parse();
