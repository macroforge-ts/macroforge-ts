import { expect, test } from '@playwright/test';

// End-to-end checks for the four Rust-inspired attribute macros in the
// vanilla playground. Mirrors the structure of `vanilla-buildtime.spec.ts`:
// each assertion inspects a DOM element whose content depends on the
// attribute pre-pass having run during the Vite build.
//
// The fixture lives in `playground/vanilla/src/attributes-test.ts` and the
// `cfg` flags it relies on are set in `playground/vanilla/macroforge.config.ts`
// (`features: ['playground']`, `target: 'web'`).

test.describe('Vanilla Playground attribute macro tests', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
        await page.waitForSelector('[data-testid="attributes-results"]');
    });

    test('@cfg keeps declarations whose feature is in the config', async ({ page }) => {
        const kept = page.locator('[data-testid="attr-kept-feature"]');
        await expect(kept).toHaveText('kept-by-feature');
    });

    test('@cfg strips declarations whose feature is missing', async ({ page }) => {
        // Stripped exports become `undefined`. main.ts renders the literal
        // string "<stripped>" in that case.
        const stripped = page.locator('[data-testid="attr-stripped-feature"]');
        await expect(stripped).toHaveText('(stripped)');
    });

    test('@cfg keeps declarations whose target matches', async ({ page }) => {
        const kept = page.locator('[data-testid="attr-kept-target"]');
        await expect(kept).toHaveText('kept-by-target');
    });

    test('@cfg strips declarations whose target does not match', async ({ page }) => {
        const stripped = page.locator('[data-testid="attr-stripped-target"]');
        await expect(stripped).toHaveText('(stripped)');
    });

    test('@deprecated function still callable after JSDoc rewrite', async ({ page }) => {
        // The annotation is rewritten to a tsc-readable JSDoc tag — the
        // function itself stays callable. Snapshot test enforces the JSDoc
        // shape; here we just verify the runtime side wasn't damaged.
        const dep = page.locator('[data-testid="attr-deprecated-call"]');
        await expect(dep).toHaveText('render-v1');
    });

    test('@nonExhaustive value passes through at runtime', async ({ page }) => {
        // The brand intersection is type-only; runtime sees a plain string.
        const status = page.locator('[data-testid="attr-non-exhaustive"]');
        await expect(status).toHaveText('green');
    });

    test('attribute results attached to globalThis', async ({ page }) => {
        const results = await page.evaluate(
            () =>
                (globalThis as unknown as {
                    attributesResults?: {
                        keptByFeature: string | null;
                        strippedByFeature: string | null;
                        keptByTarget: string | null;
                        strippedByTarget: string | null;
                        deprecatedCall: string;
                        nonExhaustiveValue: string;
                    };
                }).attributesResults
        );

        expect(results).toBeDefined();
        expect(results?.keptByFeature).toBe('kept-by-feature');
        expect(results?.strippedByFeature).toBeNull();
        expect(results?.keptByTarget).toBe('kept-by-target');
        expect(results?.strippedByTarget).toBeNull();
        expect(results?.deprecatedCall).toBe('render-v1');
        expect(results?.nonExhaustiveValue).toBe('green');
    });
});
