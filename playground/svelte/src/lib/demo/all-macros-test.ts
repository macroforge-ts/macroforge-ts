/**
 * Comprehensive test class for Svelte playground.
 * Uses all available macros for Playwright e2e testing.
 */

/** @derive(Debug, Clone, Eq, Serialize, Deserialize) */
export interface SvelteAllMacrosTest {
  /** @debug({ rename: "testId" }) */
  id: string;
  title: string;
  content: string;
  /** @debug({ skip: true }) */
  apiKey: string;
  count: number;
  enabled: boolean;
}

export namespace SvelteAllMacrosTest {
  export function make(
    id: string,
    title: string,
    content: string,
    apiKey: string,
    count: number,
    enabled: boolean,
  ): SvelteAllMacrosTest {
    return { id, title, content, apiKey, count, enabled };
  }
}

// Pre-instantiated test instance for e2e tests
export const svelteTestInstance = SvelteAllMacrosTest.make(
  "svelte-001",
  "Svelte Test",
  "Testing all macros in SvelteKit",
  "sk-secret-key",
  42,
  true,
);
