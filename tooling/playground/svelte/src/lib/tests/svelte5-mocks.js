/**
 * Svelte 5 runes mocks for testing outside of Svelte component context.
 * These mocks provide basic implementations of Svelte 5 runes for unit tests.
 */
import { vi } from 'vitest';

// Mock $state - returns a simple reactive-like object
globalThis.$state = function (initial) {
    return initial;
};

// Mock $derived - returns the computed value
globalThis.$derived = function (fn) {
    return typeof fn === 'function' ? fn() : fn;
};

// Mock $effect - no-op in tests
globalThis.$effect = function (fn) {
    // Don't run effects in tests by default
};

// Mock $props - returns default props
globalThis.$props = function () {
    return {};
};

// Mock $bindable - returns the value
globalThis.$bindable = function (initial) {
    return initial;
};

// Mock Svelte lifecycle hooks that require component context
vi.mock('svelte', async (importOriginal) => {
    const actual = await importOriginal();
    return {
        ...actual,
        onDestroy: vi.fn(),
        onMount: vi.fn(),
        beforeUpdate: vi.fn(),
        afterUpdate: vi.fn(),
        tick: vi.fn().mockResolvedValue(undefined),
        getContext: vi.fn(),
        setContext: vi.fn(),
        hasContext: vi.fn().mockReturnValue(false),
        getAllContexts: vi.fn().mockReturnValue(new Map())
    };
});
