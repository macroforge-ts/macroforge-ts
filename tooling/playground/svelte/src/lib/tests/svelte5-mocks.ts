/**
 * Svelte 5 / SvelteKit mocks for testing superforms.
 * This file should be imported at the top of test files that test client-side form functionality.
 */
import { vi } from 'vitest';

// Mock $app/environment
vi.mock('$app/environment', () => ({
    browser: false,
    building: false,
    dev: true,
    version: 'test'
}));

// Mock $app/navigation
vi.mock('$app/navigation', () => ({
    beforeNavigate: vi.fn(),
    afterNavigate: vi.fn(),
    goto: vi.fn(),
    invalidate: vi.fn(),
    invalidateAll: vi.fn(),
    preloadCode: vi.fn(),
    preloadData: vi.fn(),
    pushState: vi.fn(),
    replaceState: vi.fn()
}));

// Mock $app/state (Svelte 5)
vi.mock('$app/state', () => {
    const pageState = {
        url: new URL('http://localhost'),
        params: {},
        route: { id: '/' },
        status: 200,
        error: null,
        data: {},
        form: null
    };

    return {
        page: pageState,
        navigating: null,
        updated: { current: false, check: vi.fn() }
    };
});

// Mock $app/stores (Svelte 4 compatibility - some code may still use this)
vi.mock('$app/stores', async () => {
    const { readable, writable } = await import('svelte/store');

    const getStores = () => ({
        navigating: readable(null),
        page: readable({ url: new URL('http://localhost'), params: {} }),
        session: writable(null),
        updated: readable(false)
    });

    const page = {
        subscribe(fn: (value: unknown) => void) {
            return getStores().page.subscribe(fn);
        }
    };

    const navigating = {
        subscribe(fn: (value: unknown) => void) {
            return getStores().navigating.subscribe(fn);
        }
    };

    return {
        getStores,
        navigating,
        page
    };
});

// Mock svelte lifecycle functions
vi.mock('svelte', async (original) => {
    const module = (await original()) as Record<string, unknown>;
    return {
        ...module,
        onDestroy: vi.fn(),
        onMount: vi.fn()
    };
});

// Mock $app/forms
vi.mock('$app/forms', () => ({
    enhance: vi.fn((form, handlers) => {
        return {
            destroy: vi.fn()
        };
    }),
    applyAction: vi.fn(),
    deserialize: vi.fn()
}));
