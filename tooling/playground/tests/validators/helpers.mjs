import { assert as assertTrue, assertEquals, assertStrictEquals } from 'jsr:@std/assert@1';
import * as fs from 'node:fs';
import { createRequire } from 'node:module';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, '..', '..');
const repoRoot = path.resolve(playgroundRoot, '..', '..');
const vanillaRoot = path.join(playgroundRoot, 'vanilla');

// Use require for synchronous loading of native module
const require = createRequire(import.meta.url);
const swcMacrosPath = path.join(repoRoot, 'crates/macroforge_ts/index.js');
const macroforge = require(swcMacrosPath);
const expandSync = macroforge.expandSync;
const loadConfig = macroforge.loadConfig;

// Load and cache the config for the vanilla playground
const configPath = path.join(vanillaRoot, 'macroforge.config.ts');
if (fs.existsSync(configPath)) {
    const configContent = fs.readFileSync(configPath, 'utf8');
    loadConfig(configContent, configPath);
}

// Module cache for compiled files
const moduleCache = new Map();

/**
 * Expand macros in a TypeScript file and compile it
 */
export async function expandAndCompile(filePath) {
    if (moduleCache.has(filePath)) {
        return moduleCache.get(filePath);
    }

    const sourceCode = fs.readFileSync(filePath, 'utf8');
    const result = expandSync(sourceCode, path.basename(filePath), {
        configPath
    });

    if (result.diagnostics && result.diagnostics.length > 0) {
        const errors = result.diagnostics.filter((d) => d.severity === 'error');
        if (errors.length > 0) {
            throw new Error(
                `Macro expansion errors:\n${errors.map((e) => e.message).join('\n')}`
            );
        }
    }

    const tempDir = path.join(__dirname, '.temp');
    if (!fs.existsSync(tempDir)) {
        fs.mkdirSync(tempDir, { recursive: true });
    }

    const tempFile = path.join(tempDir, path.basename(filePath));
    fs.writeFileSync(tempFile, result.code);

    try {
        const mod = await import(tempFile);
        moduleCache.set(filePath, mod);
        return mod;
    } catch (error) {
        console.error('Failed to compile expanded code:', error.message);
        console.error('Expanded code written to:', tempFile);
        throw error;
    }
}

/**
 * Load a validator test module
 */
export async function loadValidatorModule(moduleName) {
    const filePath = path.join(vanillaRoot, 'src/validators', `${moduleName}.ts`);
    return expandAndCompile(filePath);
}

/**
 * Assert that fromStringifiedJSON returns an error result with expected message substring
 */
export function assertValidationError(result, _fieldName, messageSubstring) {
    assertStrictEquals(result.success, false, 'Expected validation to fail');
    const errors = result.errors;
    const hasExpectedError = errors.some((e) => {
        const msg = typeof e === 'string' ? e : e.message;
        return msg.includes(messageSubstring);
    });
    assertTrue(
        hasExpectedError,
        `Expected error containing "${messageSubstring}"`
    );
}

/**
 * Assert that fromStringifiedJSON returns a successful result
 */
export function assertValidationSuccess(result, fieldName) {
    if (!result.success) {
        const errors = result.errors;
        const errorMsgs = errors.map((e) => typeof e === 'string' ? e : `${e.field}: ${e.message}`);
        throw new Error(
            `Expected validation to succeed for "${fieldName}", but got errors: ${
                errorMsgs.join('; ')
            }`
        );
    }
    assertStrictEquals(result.success, true);
}

/**
 * Assert that fromStringifiedJSON returns specific error count
 */
export function assertErrorCount(result, expectedCount) {
    assertStrictEquals(result.success, false, 'Expected validation to fail');
    const errors = result.errors;
    assertStrictEquals(
        errors.length,
        expectedCount,
        `Expected ${expectedCount} errors`
    );
}

// Deno test wrappers
async function runDescribeBody(t, fn) {
    const items = [];
    let beforeFn = null;

    const testFn = (testName, testBody) => {
        items.push({ name: testName, fn: testBody, type: 'test' });
    };

    const nestedDescribeFn = (nestedName, nestedFn) => {
        items.push({ name: nestedName, fn: nestedFn, type: 'describe' });
    };

    const beforeFnWrapper = (fn) => {
        beforeFn = fn;
    };

    const origDescribe = globalThis._describe;
    const origTest = globalThis._test;
    const origBefore = globalThis._before;

    globalThis._describe = nestedDescribeFn;
    globalThis._test = testFn;
    globalThis._before = beforeFnWrapper;

    await fn();

    globalThis._describe = origDescribe;
    globalThis._test = origTest;
    globalThis._before = origBefore;

    if (beforeFn) {
        await beforeFn();
    }

    for (const item of items) {
        if (item.type === 'describe') {
            await t.step(item.name, async (st) => {
                await runDescribeBody(st, item.fn);
            });
        } else {
            await t.step(item.name, item.fn);
        }
    }
}

export function describe(name, fn) {
    if (globalThis._describe) {
        globalThis._describe(name, fn);
        return;
    }

    Deno.test(name, async (t) => {
        await runDescribeBody(t, fn);
    });
}

export function test(name, fn) {
    if (globalThis._test) {
        globalThis._test(name, fn);
    } else {
        Deno.test(name, fn);
    }
}

export function before(fn) {
    if (globalThis._before) {
        globalThis._before(fn);
    }
}

export function after(_fn) {
    // No-op for now
}

export function beforeEach(_fn) {
    // No-op for now
}

export function afterEach(_fn) {
    // No-op for now
}

export const assert = {
    ok: assertTrue,
    strictEqual: assertStrictEquals,
    deepStrictEqual: assertEquals,
    equal: assertStrictEquals
};
