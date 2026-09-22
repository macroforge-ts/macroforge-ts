import type { Page } from '@playwright/test';
import type { SveltePlayground } from '../../svelte/src/lib/playground-globals';

/** Waits for the Svelte playground to publish a slice of its results and returns it. */
export async function readSvelteSlice<Key extends keyof SveltePlayground>(
    page: Page,
    key: Key
): Promise<NonNullable<SveltePlayground[Key]>> {
    await page.waitForFunction(
        (sliceKey) => globalThis.sveltePlayground?.[sliceKey] !== undefined,
        key,
        { timeout: 15_000 }
    );
    const playground = await page.evaluate(() => globalThis.sveltePlayground);
    const slice = playground?.[key];
    if (slice === undefined || slice === null) {
        throw new Error(`sveltePlayground.${key} was not published`);
    }
    return slice;
}
