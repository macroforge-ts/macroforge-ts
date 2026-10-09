import type { Page } from '@playwright/test';
import type { VanillaTestground } from '../../vanilla/src/testground-globals.ts';

/**
 * A value the e2e harness probed, failing the test when the harness recorded
 * `null` because the derive helper threw. The page console has the error.
 */
export function probed<Value>(value: Value | null, label: string): Value {
    if (value === null) {
        throw new Error(`${label} was not produced; the page console has the helper's error`);
    }
    return value;
}

/** Waits for the vanilla testground to publish a slice of its results and returns it. */
export async function readVanillaSlice<Key extends keyof VanillaTestground>(
    page: Page,
    key: Key
): Promise<NonNullable<VanillaTestground[Key]>> {
    await page.waitForFunction(
        (sliceKey) => globalThis.vanillaTestground?.[sliceKey] !== undefined,
        key,
        { timeout: 15_000 }
    );
    const testground = await page.evaluate(() => globalThis.vanillaTestground);
    const slice = testground?.[key];
    if (slice === undefined || slice === null) {
        throw new Error(`vanillaTestground.${key} was not published`);
    }
    return slice;
}
