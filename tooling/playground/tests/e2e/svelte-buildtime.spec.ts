import { expect, test } from '@playwright/test';

// End-to-end sanity check for the `@buildtime` evaluation pipeline in
// the svelte playground. Mirrors `vanilla-buildtime.spec.ts` against
// the SvelteKit home route.

test.describe('Svelte Playground @buildtime Tests', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('[data-testid="buildtime-results"]');
    });

    test('Tier 1 arithmetic spliced as literal', async ({ page }) => {
        const answer = page.locator('[data-testid="svelte-bt-answer"]');
        await expect(answer).toHaveText('42');
    });

    test('Tier 1 compile-time sha256 hash is stable', async ({ page }) => {
        const hash = page.locator('[data-testid="svelte-bt-hash"]');
        const hashText = (await hash.textContent())?.trim() ?? '';
        expect(hashText).toMatch(/^[0-9a-f]{64}$/);
    });

    test('Tier 1 fs.readJson pulls data from sibling file', async ({ page }) => {
        const appName = page.locator('[data-testid="svelte-bt-app-name"]');
        await expect(appName).toHaveText('macroforge-svelte');

        const appVersion = page.locator('[data-testid="svelte-bt-app-version"]');
        await expect(appVersion).toHaveText('3.0.0');

        const features = page.locator('[data-testid="svelte-bt-features"]');
        await expect(features).toHaveText('SSR,HMR,runes');
    });

    test('Tier 1 .map() compiles to a literal array', async ({ page }) => {
        // [1, 2, 3, 5, 8, 13] doubled = [2, 4, 6, 10, 16, 26]
        const list = page.locator('[data-testid="svelte-bt-constant-list"]');
        await expect(list).toHaveText('2,4,6,10,16,26');
    });

    test('Tier 1 template literal composes compile-time values', async ({ page }) => {
        const summary = page.locator('[data-testid="svelte-bt-summary"]');
        const text = (await summary.textContent())?.trim() ?? '';
        // `app=3.0.0, short=XXXXXXXX`
        expect(text).toMatch(/^app=3\.0\.0, short=[0-9a-f]{8}$/);
    });

    test('runtime `macroforge/buildtime` stub still throws', async ({ page }) => {
        const stubThrows = page.locator('[data-testid="svelte-bt-stub-throws"]');
        await expect(stubThrows).toHaveText('true');
    });
});
