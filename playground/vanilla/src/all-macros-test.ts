/**
 * Comprehensive test class demonstrating all available macros.
 * Used for Playwright e2e tests to verify macro expansion works at runtime.
 */

/** @derive(Debug, Clone, Eq, Serialize, Deserialize) */
export class AllMacrosTestClass {
  /** @debug({ rename: "identifier" }) */
  id: number;

  name: string;

  email: string;

  /** @debug({ skip: true }) */
  secretToken: string;

  isActive: boolean;

  score: number;

  constructor(
    id: number,
    name: string,
    email: string,
    secretToken: string,
    isActive: boolean,
    score: number
  ) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.secretToken = secretToken;
    this.isActive = isActive;
    this.score = score;
  }
}

// Pre-instantiated test instance for e2e tests
export const testInstance = new AllMacrosTestClass(
  42,
  "Test User",
  "test@example.com",
  "secret-token-123",
  true,
  100
);
