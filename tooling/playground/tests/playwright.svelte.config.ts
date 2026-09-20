import { defineConfig, devices } from '@playwright/test';

// Env-driven for the same reason as the vanilla config: a hardcoded port plus
// `reuseExistingServer` silently runs the suite against whatever is already
// listening there.
const port = globalThis.process.env.PLAYGROUND_SVELTE_PORT ?? '5173';
const origin = `http://localhost:${port}`;

export default defineConfig({
    testDir: './e2e',
    testMatch: '**/svelte-*.spec.ts',
    fullyParallel: true,
    forbidOnly: !!globalThis.process.env.CI,
    retries: globalThis.process.env.CI ? 2 : 0,
    workers: globalThis.process.env.CI ? 1 : undefined,
    reporter: 'html',
    use: {
        baseURL: origin,
        trace: 'on-first-retry'
    },
    projects: [
        {
            name: 'chromium',
            use: { ...devices['Desktop Chrome'] }
        }
    ],
    webServer: {
        command: 'deno task dev',
        cwd: '../svelte',
        url: origin,
        reuseExistingServer: !globalThis.process.env.CI,
        timeout: 120000
    }
});
