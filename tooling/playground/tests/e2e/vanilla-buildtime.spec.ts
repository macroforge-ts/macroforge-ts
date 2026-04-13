import { expect, test } from '@playwright/test';

// End-to-end sanity check for the `@buildtime` evaluation pipeline in
// the vanilla playground. Each assertion inspects a DOM element whose
// content was produced by a `@buildtime` declaration that macroforge
// spliced into `buildtime-demo.ts` before the browser saw the module.
// If the Vite plugin didn't run the pre-pass, the runtime stubs from
// `macroforge/buildtime` would throw and the page would never render.

test.describe('Vanilla Playground @buildtime Tests', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
        await page.waitForSelector('[data-testid="buildtime-results"]');
    });

    test('Tier 1 arithmetic spliced as literal', async ({ page }) => {
        const answer = page.locator('[data-testid="bt-answer"]');
        await expect(answer).toHaveText('42');
    });

    test('Tier 1 compile-time sha256 hash is stable', async ({ page }) => {
        // sha256('user-schema-v1') is deterministic — the compile-time
        // result must match what a runtime sha256 would produce.
        const hash = page.locator('[data-testid="bt-hash"]');
        const hashText = (await hash.textContent())?.trim() ?? '';
        // 64 hex chars
        expect(hashText).toMatch(/^[0-9a-f]{64}$/);
    });

    test('Tier 1 fs.readJson pulls data from sibling file', async ({ page }) => {
        const appName = page.locator('[data-testid="bt-app-name"]');
        await expect(appName).toHaveText('macroforge-playground');

        const appVersion = page.locator('[data-testid="bt-app-version"]');
        await expect(appVersion).toHaveText('2.4.1');

        const routes = page.locator('[data-testid="bt-routes"]');
        await expect(routes).toHaveText('home,users,settings');
    });

    test('Tier 1 IIFE produces compile-time greeting table', async ({ page }) => {
        const alice = page.locator('[data-testid="bt-greet-alice"]');
        await expect(alice).toHaveText('hello, alice');

        // Keys must be sorted alphabetically (main.ts sorts them before display).
        const keys = page.locator('[data-testid="bt-greet-keys"]');
        await expect(keys).toHaveText('alice,bob,cam');
    });

    test('Tier 1 nested object is serialized as a proper object literal', async ({ page }) => {
        const thirteen = page.locator('[data-testid="bt-const-thirteen"]');
        await expect(thirteen).toHaveText('13');
    });

    test('Tier 1 template literal composes compile-time values', async ({ page }) => {
        const summary = page.locator('[data-testid="bt-summary"]');
        const text = (await summary.textContent())?.trim() ?? '';
        // `answer=42, hash=XXXXXXXX` (8 hex chars from the sha256 prefix)
        expect(text).toMatch(/^answer=42, hash=[0-9a-f]{8}$/);
    });

    test('runtime `macroforge/buildtime` stub still throws', async ({ page }) => {
        // If the Vite plugin runs, every real @buildtime use was already
        // evaluated at compile time. But importing `buildtime` at runtime
        // and calling a method on it should still throw — that's the
        // contract of the runtime stub.
        const stubThrows = page.locator('[data-testid="bt-stub-throws"]');
        await expect(stubThrows).toHaveText('true');
    });

    test('buildtime results are attached to globalThis for inspection', async ({ page }) => {
        const results = await page.evaluate(
            () =>
                (globalThis as unknown as {
                    buildtimeResults?: {
                        answer: number;
                        schemaHash: string;
                        appName: string;
                        greetingAlice: string;
                        runtimeStubThrows: boolean;
                    };
                }).buildtimeResults
        );

        expect(results).toBeDefined();
        expect(results?.answer).toBe(42);
        expect(results?.schemaHash).toMatch(/^[0-9a-f]{64}$/);
        expect(results?.appName).toBe('macroforge-playground');
        expect(results?.greetingAlice).toBe('hello, alice');
        expect(results?.runtimeStubThrows).toBe(true);
    });
});
