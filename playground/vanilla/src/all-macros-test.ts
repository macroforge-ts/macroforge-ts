/**
 * Comprehensive test class demonstrating all available macros.
 * Used for Playwright e2e tests to verify macro expansion works at runtime.
 */

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export class AllMacrosTestClass {
    /** @debug({ rename: "identifier" }) */
    id: number;

    name: string;

    email: string;

    /** @debug({ skip: true }) */
    secretToken: string;

    isActive: boolean;

    score: number;
}

// Pre-instantiated test instance for e2e tests
export const testInstance = new AllMacrosTestClass({
    id: 42,
    name: 'Test User',
    email: 'test@example.com',
    secretToken: 'secret-token-123',
    isActive: true,
    score: 100
});
