import { assert as assertTrue, assertEquals, assertStrictEquals } from 'jsr:@std/assert@1';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { cliBinary } from '../test-utils.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, '..', '..');
const vanillaRoot = path.join(playgroundRoot, 'vanilla');
const vanillaCacheDir = path.join(vanillaRoot, '.macroforge', 'cache');

// Warm the cache once for all validator tests
let cacheWarmed = false;

function warmCache() {
    if (cacheWarmed) return;
    const command = new Deno.Command(cliBinary, {
        args: ['cache', vanillaRoot, '--builtin-only'],
        cwd: vanillaRoot,
        stdout: 'piped',
        stderr: 'piped'
    });
    const result = command.outputSync();
    if (!result.success) {
        const decoder = new TextDecoder();
        throw new Error(
            `macroforge cache failed:\n${decoder.decode(result.stderr)}`
        );
    }
    cacheWarmed = true;
}

// Module cache for compiled files
const moduleCache = new Map();

/**
 * Expand macros in a TypeScript file via the cache and compile it
 */
export async function expandAndCompile(filePath) {
    if (moduleCache.has(filePath)) {
        return moduleCache.get(filePath);
    }

    warmCache();

    const relPath = path.relative(vanillaRoot, filePath);
    const cachePath = path.join(vanillaCacheDir, relPath + '.cache');

    let expandedCode;
    if (existsSync(cachePath)) {
        expandedCode = fs.readFileSync(cachePath, 'utf8');
    } else {
        // File has no macros — use the original source
        expandedCode = fs.readFileSync(filePath, 'utf8');
    }

    const tempDir = path.join(__dirname, '.temp');
    if (!existsSync(tempDir)) {
        fs.mkdirSync(tempDir, { recursive: true });
    }

    const tempFile = path.join(tempDir, path.basename(filePath));
    fs.writeFileSync(tempFile, expandedCode);

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
