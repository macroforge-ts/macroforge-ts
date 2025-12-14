/**
 * Server-side utility for loading generated API documentation.
 *
 * This module provides functions to load API data from the generated JSON files
 * at build time, allowing hand-written pages to pull specific signatures,
 * parameters, and descriptions from the source code doc comments.
 *
 * @example
 * ```typescript
 * // In +page.server.ts
 * import { getRustItem, getTsItem } from '$lib/server/api-docs';
 *
 * export async function load() {
 *   return {
 *     expandSync: await getRustItem('macroforge_ts', 'expand_sync')
 *   };
 * }
 * ```
 */

import fs from 'fs';
import path from 'path';

const API_DATA_DIR = path.join(process.cwd(), 'static', 'api-data');

interface ApiParam {
	name: string;
	type: string;
	description: string;
}

interface ApiItem {
	name: string;
	kind: string;
	signature: string;
	description: string;
	params?: ApiParam[];
	returns?: { type: string; description: string };
	examples?: string[];
	remarks?: string;
	errors?: string;
	panics?: string;
	notes?: string;
}

interface ApiModule {
	name: string;
	kind: 'rust_crate' | 'typescript_package';
	version: string;
	description: string;
	overview: string;
	items: ApiItem[];
}

// Cache for loaded modules
const moduleCache = new Map<string, ApiModule>();

/**
 * Load a Rust crate's API documentation.
 */
export function loadRustCrate(crateName: string): ApiModule | null {
	const cacheKey = `rust:${crateName}`;
	if (moduleCache.has(cacheKey)) {
		return moduleCache.get(cacheKey)!;
	}

	const filePath = path.join(API_DATA_DIR, 'rust', `${crateName}.json`);
	if (!fs.existsSync(filePath)) {
		console.warn(`API docs not found for Rust crate: ${crateName}`);
		return null;
	}

	try {
		const data = JSON.parse(fs.readFileSync(filePath, 'utf-8')) as ApiModule;
		moduleCache.set(cacheKey, data);
		return data;
	} catch (e) {
		console.error(`Failed to load API docs for ${crateName}:`, e);
		return null;
	}
}

/**
 * Load a TypeScript package's API documentation.
 */
export function loadTsPackage(packageName: string): ApiModule | null {
	const cacheKey = `ts:${packageName}`;
	if (moduleCache.has(cacheKey)) {
		return moduleCache.get(cacheKey)!;
	}

	const filePath = path.join(API_DATA_DIR, 'typescript', `${packageName}.json`);
	if (!fs.existsSync(filePath)) {
		console.warn(`API docs not found for TypeScript package: ${packageName}`);
		return null;
	}

	try {
		const data = JSON.parse(fs.readFileSync(filePath, 'utf-8')) as ApiModule;
		moduleCache.set(cacheKey, data);
		return data;
	} catch (e) {
		console.error(`Failed to load API docs for ${packageName}:`, e);
		return null;
	}
}

/**
 * Get a specific item from a Rust crate's documentation.
 *
 * @param crateName - The crate name (e.g., 'macroforge_ts')
 * @param itemName - The item name (e.g., 'expand_sync', 'ExpandResult')
 * @returns The API item or null if not found
 *
 * @example
 * ```typescript
 * const expandSync = getRustItem('macroforge_ts', 'expand_sync');
 * // Use expandSync.signature, expandSync.params, etc.
 * ```
 */
export function getRustItem(crateName: string, itemName: string): ApiItem | null {
	const crate = loadRustCrate(crateName);
	if (!crate) return null;

	return crate.items.find((item) => item.name === itemName) || null;
}

/**
 * Get a specific item from a TypeScript package's documentation.
 *
 * @param packageName - The package name (e.g., 'typescript-plugin')
 * @param itemName - The item name (e.g., 'init', 'shouldProcess')
 * @returns The API item or null if not found
 */
export function getTsItem(packageName: string, itemName: string): ApiItem | null {
	const pkg = loadTsPackage(packageName);
	if (!pkg) return null;

	return pkg.items.find((item) => item.name === itemName) || null;
}

/**
 * Get multiple items from a Rust crate.
 */
export function getRustItems(crateName: string, itemNames: string[]): Record<string, ApiItem | null> {
	const result: Record<string, ApiItem | null> = {};
	for (const name of itemNames) {
		result[name] = getRustItem(crateName, name);
	}
	return result;
}

/**
 * Get multiple items from a TypeScript package.
 */
export function getTsItems(packageName: string, itemNames: string[]): Record<string, ApiItem | null> {
	const result: Record<string, ApiItem | null> = {};
	for (const name of itemNames) {
		result[name] = getTsItem(packageName, name);
	}
	return result;
}

/**
 * Get the crate/package version.
 */
export function getVersion(type: 'rust' | 'ts', name: string): string | null {
	const module = type === 'rust' ? loadRustCrate(name) : loadTsPackage(name);
	return module?.version || null;
}

/**
 * Get all items of a specific kind from a crate/package.
 */
export function getItemsByKind(
	type: 'rust' | 'ts',
	moduleName: string,
	kind: string
): ApiItem[] {
	const module = type === 'rust' ? loadRustCrate(moduleName) : loadTsPackage(moduleName);
	if (!module) return [];
	return module.items.filter((item) => item.kind === kind);
}

// Builtin macro documentation
interface ExampleCodeBlock {
	lang: string;
	code: string;
	rawCode: string;
}

interface BuiltinMacroDoc {
	name: string;
	slug: string;
	description: string;
	generatedOutput: string;
	fieldOptions: string;
	example: string;
	exampleCode: ExampleCodeBlock[];
	title: string;
	raw: string;
}

let builtinMacrosCache: Record<string, BuiltinMacroDoc> | null = null;

/**
 * Load all builtin macro documentation.
 */
export function loadBuiltinMacros(): Record<string, BuiltinMacroDoc> {
	if (builtinMacrosCache) {
		return builtinMacrosCache;
	}

	const filePath = path.join(API_DATA_DIR, 'rust', 'builtin-macros.json');
	if (!fs.existsSync(filePath)) {
		console.warn('Builtin macros documentation not found');
		return {};
	}

	try {
		builtinMacrosCache = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
		return builtinMacrosCache!;
	} catch (e) {
		console.error('Failed to load builtin macros docs:', e);
		return {};
	}
}

/**
 * Get documentation for a specific builtin macro.
 *
 * @param macroName - The macro name (e.g., 'debug', 'clone', 'partial_eq')
 * @returns The macro documentation or null if not found
 *
 * @example
 * ```typescript
 * const debugDocs = getBuiltinMacro('debug');
 * // Use debugDocs.description, debugDocs.fieldOptions, etc.
 * ```
 */
export function getBuiltinMacro(macroName: string): BuiltinMacroDoc | null {
	const macros = loadBuiltinMacros();
	// Support both 'partial_eq' and 'partial-eq' formats
	const normalizedName = macroName.replace(/-/g, '_');
	return macros[normalizedName] || null;
}

/**
 * Get documentation for multiple builtin macros.
 */
export function getBuiltinMacros(macroNames: string[]): Record<string, BuiltinMacroDoc | null> {
	const result: Record<string, BuiltinMacroDoc | null> = {};
	for (const name of macroNames) {
		result[name] = getBuiltinMacro(name);
	}
	return result;
}

// CLI documentation
interface CliCommand {
	description: string;
}

interface CliDoc {
	name: string;
	version: string;
	title: string;
	description: string;
	commands: Record<string, string>;
	outputFileNaming: string;
	exitCodes: string;
	nodeIntegration: string;
	functions: Array<{
		name: string;
		signature: string;
		description: string;
		params?: ApiParam[];
		returns?: { type: string; description: string };
	}>;
	raw: string;
}

let cliDocsCache: CliDoc | null = null;

/**
 * Load CLI documentation.
 */
export function loadCliDocs(): CliDoc | null {
	if (cliDocsCache) {
		return cliDocsCache;
	}

	const filePath = path.join(API_DATA_DIR, 'rust', 'cli.json');
	if (!fs.existsSync(filePath)) {
		console.warn('CLI documentation not found');
		return null;
	}

	try {
		cliDocsCache = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
		return cliDocsCache;
	} catch (e) {
		console.error('Failed to load CLI docs:', e);
		return null;
	}
}

/**
 * Get CLI documentation.
 *
 * @returns The CLI documentation or null if not found
 *
 * @example
 * ```typescript
 * const cliDocs = getCliDocs();
 * // Use cliDocs.commands, cliDocs.exitCodes, etc.
 * ```
 */
export function getCliDocs(): CliDoc | null {
	return loadCliDocs();
}

// Re-export types for use in pages
export type { ApiItem, ApiModule, ApiParam, BuiltinMacroDoc, CliDoc, ExampleCodeBlock };
