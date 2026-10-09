import type { Page } from '@playwright/test';
import type { SvelteTestground } from '../../svelte/src/lib/testground-globals';

/** Waits for the Svelte testground to publish a slice of its results and returns it. */
export async function readSvelteSlice<Key extends keyof SvelteTestground>(
    page: Page,
    key: Key
): Promise<NonNullable<SvelteTestground[Key]>> {
    await page.waitForFunction(
        (sliceKey) => globalThis.svelteTestground?.[sliceKey] !== undefined,
        key,
        { timeout: 15_000 }
    );
    const testground = await page.evaluate(() => globalThis.svelteTestground);
    const slice = testground?.[key];
    if (slice === undefined || slice === null) {
        throw new Error(`svelteTestground.${key} was not published`);
    }
    return slice;
}
