/**
 * Dev server e2e tests for @macroforge/vite-plugin
 *
 * These tests verify that macro-expanded code is served correctly
 * over HTTP when using a real Vite dev server (not middleware mode).
 */

import assert from 'node:assert/strict';
import { describe, test } from 'node:test';
import { vanillaRoot, withDevServer } from './test-utils.mjs';

// =============================================================================
// HTTP Server Lifecycle
// =============================================================================

describe('Dev Server HTTP', () => {
    test(
        'starts and serves index.html',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(baseUrl);
                assert.equal(res.status, 200);

                const html = await res.text();
                assert.ok(
                    html.includes('/src/main.ts'),
                    'Should reference the main entry point'
                );
            });
        }
    );
});

// =============================================================================
// Macro Expansion over HTTP
// =============================================================================

describe('Dev Server Macro Expansion', () => {
    test(
        'serves macro-expanded user.ts over HTTP',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(`${baseUrl}/src/user.ts`);
                assert.equal(res.status, 200);

                const code = await res.text();

                // Debug macro generates static toString method
                assert.ok(
                    code.includes('static toString(value)'),
                    'Should include generated static toString method from Debug macro'
                );

                // JSON macro generates toJSON method
                assert.ok(
                    code.includes('toJSON()'),
                    'Should include generated toJSON method from JSON macro'
                );
            });
        }
    );

    test(
        'serves all-macros-test.ts with multiple macro expansions',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(
                    `${baseUrl}/src/all-macros-test.ts`
                );
                assert.equal(res.status, 200);

                const code = await res.text();

                // Debug macro
                assert.ok(
                    code.includes('static toString(value)'),
                    'Should have Debug macro expansion (static toString)'
                );

                // Clone macro
                assert.ok(
                    code.includes('clone'),
                    'Should have Clone macro expansion'
                );

                // Serialize macro
                assert.ok(
                    code.includes('serialize'),
                    'Should have Serialize macro expansion'
                );
            });
        }
    );

    test(
        'serves non-macro files without errors',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(`${baseUrl}/src/main.ts`);
                assert.equal(res.status, 200);

                const code = await res.text();
                assert.ok(
                    code.includes('User'),
                    'Should contain User import reference'
                );
            });
        }
    );
});

// =============================================================================
// HTTP Response Headers
// =============================================================================

describe('Dev Server Response Headers', () => {
    test(
        'serves transformed .ts modules with JavaScript content-type',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(`${baseUrl}/src/user.ts`);
                assert.equal(res.status, 200);

                const contentType = res.headers.get('content-type');
                assert.ok(
                    contentType?.includes('javascript'),
                    `Expected JavaScript content-type, got: ${contentType}`
                );
            });
        }
    );
});

// =============================================================================
// Concurrency
// =============================================================================

describe('Dev Server Concurrency', () => {
    test(
        'handles concurrent HTTP requests',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const [userRes, mainRes, allMacrosRes] = await Promise.all([
                    fetch(`${baseUrl}/src/user.ts`),
                    fetch(`${baseUrl}/src/main.ts`),
                    fetch(`${baseUrl}/src/all-macros-test.ts`)
                ]);

                assert.equal(userRes.status, 200, 'user.ts should return 200');
                assert.equal(mainRes.status, 200, 'main.ts should return 200');
                assert.equal(
                    allMacrosRes.status,
                    200,
                    'all-macros-test.ts should return 200'
                );

                const userCode = await userRes.text();
                assert.ok(
                    userCode.includes('static toString(value)'),
                    'user.ts should have macro expansion in concurrent request'
                );
            });
        }
    );
});

// =============================================================================
// HTTP vs Internal Transform Consistency
// =============================================================================

describe('Dev Server Transform Consistency', () => {
    test(
        'HTTP response matches internal transformRequest output',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (server, baseUrl) => {
                // Get the transform result via the internal API
                const transformResult = await server.transformRequest('/src/user.ts');

                // Get the same file via HTTP
                const res = await fetch(`${baseUrl}/src/user.ts`);
                const httpCode = await res.text();

                // Both should contain the same macro expansions
                assert.ok(transformResult.code.includes('static toString(value)'));
                assert.ok(httpCode.includes('static toString(value)'));
                assert.ok(transformResult.code.includes('toJSON()'));
                assert.ok(httpCode.includes('toJSON()'));
            });
        }
    );
});

// =============================================================================
// .svelte.ts File Handling
// =============================================================================

describe('Dev Server .svelte.ts Handling', () => {
    test(
        'serves macro-expanded .svelte.ts files',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(
                    `${baseUrl}/src/store.svelte.ts`
                );
                assert.equal(res.status, 200);

                const code = await res.text();

                // Debug macro should have expanded
                assert.ok(
                    code.includes('static toString(value)'),
                    'Should include generated static toString from Debug macro'
                );

                // Clone macro should have expanded
                assert.ok(
                    code.includes('clone'),
                    'Should include Clone macro expansion'
                );
            });
        }
    );

    test(
        'strips @derive JSDoc from .svelte.ts output',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(
                    `${baseUrl}/src/store.svelte.ts`
                );
                const code = await res.text();

                // The vite-plugin strips @derive JSDoc from .svelte.ts files
                // to prevent Svelte preprocessor from re-expanding
                assert.ok(
                    !code.includes('@derive(Debug, Clone)'),
                    '@derive JSDoc should be stripped from .svelte.ts output'
                );
            });
        }
    );

    test(
        'serves .svelte.ts with JavaScript content-type',
        { timeout: 30000 },
        async () => {
            await withDevServer(vanillaRoot, async (_server, baseUrl) => {
                const res = await fetch(
                    `${baseUrl}/src/store.svelte.ts`
                );
                assert.equal(res.status, 200);

                const contentType = res.headers.get('content-type');
                assert.ok(
                    contentType?.includes('javascript'),
                    `Expected JavaScript content-type for .svelte.ts, got: ${contentType}`
                );
            });
        }
    );
});
