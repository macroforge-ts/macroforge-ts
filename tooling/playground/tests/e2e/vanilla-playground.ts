import type { Page } from '@playwright/test';
import type { VanillaPlayground } from '../../vanilla/src/playground-globals.ts';

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

/** Waits for the vanilla playground to publish a slice of its results and returns it. */
export async function readVanillaSlice<Key extends keyof VanillaPlayground>(
    page: Page,
    key: Key
): Promise<NonNullable<VanillaPlayground[Key]>> {
    await page.waitForFunction(
        (sliceKey) => globalThis.vanillaPlayground?.[sliceKey] !== undefined,
        key,
        { timeout: 15_000 }
    );
    const playground = await page.evaluate(() => globalThis.vanillaPlayground);
    const slice = playground?.[key];
    if (slice === undefined || slice === null) {
        throw new Error(`vanillaPlayground.${key} was not published`);
    }
    return slice;
}
