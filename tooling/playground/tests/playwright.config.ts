import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
    testDir: './e2e',
    testMatch: '**/vanilla-*.spec.ts',
    fullyParallel: true,
    forbidOnly: !!globalThis.process.env.CI,
    retries: globalThis.process.env.CI ? 2 : 0,
    workers: globalThis.process.env.CI ? 1 : undefined,
    reporter: 'html',
    use: {
        baseURL: 'http://localhost:3000',
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
        cwd: '../vanilla',
        url: 'http://localhost:3000',
        reuseExistingServer: !globalThis.process.env.CI,
        timeout: 120000
    }
});
