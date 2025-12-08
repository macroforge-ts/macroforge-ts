/**
 * Comprehensive test class for Svelte playground.
 * Uses all available macros for Playwright e2e testing.
 */

/** @derive(Debug, Clone, Eq, Serialize, Deserialize) */
export class SvelteAllMacrosTest {
  /** @debug({ rename: "testId" }) */
  id: string;

  title: string;

  content: string;

  /** @debug({ skip: true }) */
  apiKey: string;

  count: number;

  enabled: boolean;

  constructor(
    id: string,
    title: string,
    content: string,
    apiKey: string,
    count: number,
    enabled: boolean
  ) {
    this.id = id;
    this.title = title;
    this.content = content;
    this.apiKey = apiKey;
    this.count = count;
    this.enabled = enabled;
  }
}

// Pre-instantiated test instance for e2e tests
export const svelteTestInstance = new SvelteAllMacrosTest(
  "svelte-001",
  "Svelte Test",
  "Testing all macros in SvelteKit",
  "sk-secret-key",
  42,
  true
);
