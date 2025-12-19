import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock SvelteKit modules that are commonly used but unavailable during testing
vi.mock('$app/forms', () => ({
	applyAction: vi.fn(),
	deserialize: vi.fn(),
	enhance: vi.fn()
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
	invalidate: vi.fn(),
	invalidateAll: vi.fn(),
	prefetch: vi.fn(),
	prefetchRoutes: vi.fn(),
	beforeNavigate: vi.fn(),
	afterNavigate: vi.fn()
}));

vi.mock('$app/stores', () => ({
	page: {
		subscribe: vi.fn()
	},
	navigating: {
		subscribe: vi.fn()
	},
	updated: {
		subscribe: vi.fn(),
		check: vi.fn()
	}
}));

vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

vi.mock('$app/paths', () => ({
	base: '',
	assets: ''
}));
