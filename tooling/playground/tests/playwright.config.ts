import { defineConfig, devices } from '@playwright/test';

// Env-driven, matching the vanilla playground's own `server.port`, so a run can
// move off a port another project already holds. Both have to read the same
// variable: pointing the suite at an address this config did not start is how a
// green e2e run ends up having tested someone else's application.
const port = globalThis.process.env.PLAYGROUND_VANILLA_PORT ?? '3000';
const origin = `http://localhost:${port}`;

export default defineConfig({
    testDir: './e2e',
    testMatch: '**/vanilla-*.spec.ts',
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
    // The built app, not the dev server: a preview serves static files, so no
    // test pays for an on-demand macro expansion or races Vite's warm-up.
    // `mf test playground` builds the app; preview fails loudly without one.
    webServer: {
        command: 'deno task preview',
        cwd: '../vanilla',
        url: origin,
        reuseExistingServer: !globalThis.process.env.CI,
        timeout: 120000
    }
});
