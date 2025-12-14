#!/usr/bin/env node

/**
 * Check if generated documentation is in sync with source code.
 *
 * This script is designed for CI - it regenerates docs to a temp location
 * and compares with the committed versions. Exits with code 1 if docs are stale.
 *
 * Usage: node scripts/check-docs-freshness.cjs
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const crypto = require('crypto');

const rootDir = path.join(__dirname, '..');

// Files to check
const GENERATED_FILES = [
	'website/static/api-data/rust/index.json',
	'website/static/api-data/rust/macroforge_ts.json',
	'website/static/api-data/rust/macroforge_ts_syn.json',
	'website/static/api-data/rust/macroforge_ts_quote.json',
	'website/static/api-data/rust/macroforge_ts_macros.json',
	'website/static/api-data/typescript/index.json',
	'website/static/api-data/typescript/typescript-plugin.json',
	'website/static/api-data/typescript/vite-plugin.json',
	'website/static/api-data/typescript/svelte-preprocessor.json',
	'website/static/api-data/typescript/mcp-server.json'
];

const README_FILES = [
	'crates/macroforge_ts/README.md',
	'crates/macroforge_ts_syn/README.md',
	'crates/macroforge_ts_quote/README.md',
	'crates/macroforge_ts_macros/README.md',
	'packages/typescript-plugin/README.md',
	'packages/vite-plugin/README.md',
	'packages/svelte-preprocessor/README.md',
	'packages/mcp-server/README.md'
];

/**
 * Calculate MD5 hash of file content (excluding timestamps).
 */
function hashFile(filePath) {
	if (!fs.existsSync(filePath)) {
		return null;
	}

	let content = fs.readFileSync(filePath, 'utf-8');

	// For JSON files, parse and remove the 'generated' timestamp field
	if (filePath.endsWith('.json')) {
		try {
			const data = JSON.parse(content);
			delete data.generated;
			content = JSON.stringify(data, null, 2);
		} catch {
			// Not valid JSON, hash as-is
		}
	}

	return crypto.createHash('md5').update(content).digest('hex');
}

/**
 * Store current hashes of generated files.
 */
function getCurrentHashes(files) {
	const hashes = {};
	for (const file of files) {
		const fullPath = path.join(rootDir, file);
		hashes[file] = hashFile(fullPath);
	}
	return hashes;
}

/**
 * Main check function.
 */
function main() {
	console.log('Checking documentation freshness...\n');

	// Store current hashes
	const beforeApiHashes = getCurrentHashes(GENERATED_FILES);
	const beforeReadmeHashes = getCurrentHashes(README_FILES);

	// Regenerate API docs
	console.log('Regenerating API documentation...');
	try {
		execSync('node scripts/extract-rust-docs.cjs', { cwd: rootDir, stdio: 'pipe' });
		execSync('node scripts/extract-ts-docs.cjs', { cwd: rootDir, stdio: 'pipe' });
	} catch (e) {
		console.error('Failed to regenerate API docs:', e.message);
		process.exit(1);
	}

	// Regenerate READMEs
	console.log('Regenerating READMEs...');
	try {
		execSync('node scripts/generate-readmes.cjs', { cwd: rootDir, stdio: 'pipe' });
	} catch (e) {
		console.error('Failed to regenerate READMEs:', e.message);
		process.exit(1);
	}

	// Compare hashes
	const afterApiHashes = getCurrentHashes(GENERATED_FILES);
	const afterReadmeHashes = getCurrentHashes(README_FILES);

	const staleFiles = [];

	// Check API docs
	for (const file of GENERATED_FILES) {
		if (beforeApiHashes[file] !== afterApiHashes[file]) {
			staleFiles.push(file);
		}
	}

	// Check READMEs
	for (const file of README_FILES) {
		if (beforeReadmeHashes[file] !== afterReadmeHashes[file]) {
			staleFiles.push(file);
		}
	}

	if (staleFiles.length > 0) {
		console.log('\n❌ Documentation is out of sync!\n');
		console.log('The following files need to be regenerated:');
		for (const file of staleFiles) {
			console.log(`  - ${file}`);
		}
		console.log('\nRun `pixi run docs:all` to update documentation.\n');
		process.exit(1);
	}

	console.log('\n✅ All documentation is up to date.\n');
	process.exit(0);
}

main();
