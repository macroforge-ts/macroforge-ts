/**
 * Shared test utilities for playground tests
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';
import macroforge from '@macroforge/vite-plugin';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const playgroundRoot = path.resolve(__dirname, '..');
// Use MACROFORGE_ROOT env var for repo root
if (!process.env.MACROFORGE_ROOT) {
    throw new Error('MACROFORGE_ROOT environment variable must be set');
}
export const repoRoot = process.env.MACROFORGE_ROOT;
export const vanillaRoot = path.join(playgroundRoot, 'vanilla');
export const svelteRoot = path.join(playgroundRoot, 'svelte');
export const rootConfigPath = path.join(repoRoot, 'macroforge.json');

// Port counter for unique WebSocket ports per server instance
let portCounter = 24700;

/**
 * Get a unique port for each Vite server instance.
 * This prevents "Port is already in use" errors when tests run concurrently.
 */
function getNextPort() {
    return portCounter++;
}

/**
 * Helper to create and manage a Vite server for testing.
 * Handles setup, teardown, and config file copying.
 * Uses unique ports to prevent conflicts between concurrent test servers.
 */
/**
 * Build the shared Vite config used by both withViteServer and withDevServer.
 */
function buildMacroforgeViteConfig() {
    return {
        plugins: [macroforge()],
        ssr: {
            noExternal: ['effect', '@playground/macro']
        },
        resolve: {
            dedupe: ['effect'],
            alias: {
                'macroforge/serde': path.resolve(
                    repoRoot,
                    'crates/macroforge_ts/js/serde/index.mjs'
                ),
                'macroforge/traits': path.resolve(
                    repoRoot,
                    'crates/macroforge_ts/js/traits/index.mjs'
                ),
                'macroforge/reexports': path.resolve(
                    repoRoot,
                    'crates/macroforge_ts/js/reexports/index.mjs'
                ),
                'macroforge/reexports/effect': path.resolve(
                    repoRoot,
                    'crates/macroforge_ts/js/reexports/effect.mjs'
                ),
                'macroforge': path.resolve(repoRoot, 'crates/macroforge_ts')
            }
        }
    };
}

/**
 * Manage cwd, macroforge.json copying, and cleanup around a server lifecycle.
 */
async function withManagedEnv(rootDir, options, serverFactory) {
    const { useProjectCwd = true } = options ?? {};
    const previousCwd = process.cwd();
    let server;
    let copiedConfig = false;
    const localConfigPath = path.join(rootDir, 'macroforge.json');

    try {
        if (useProjectCwd) {
            process.chdir(rootDir);
        }

        // Copy the workspace-level config so the macro host loads the shared macro packages
        if (!fs.existsSync(localConfigPath) && fs.existsSync(rootConfigPath)) {
            fs.copyFileSync(rootConfigPath, localConfigPath);
            copiedConfig = true;
        }

        const result = await serverFactory();
        server = result.server;
        return await result.run();
    } finally {
        if (server) {
            await new Promise((resolve) => setTimeout(resolve, 200));
            try {
                await server.close();
            } catch {
                // Ignore errors during server close
            }
            await new Promise((resolve) => setTimeout(resolve, 100));
        }
        if (copiedConfig && fs.existsSync(localConfigPath)) {
            fs.rmSync(localConfigPath);
        }
        if (useProjectCwd) {
            process.chdir(previousCwd);
        }
    }
}

export async function withViteServer(rootDir, optionsOrRunner, maybeRunner) {
    const options = typeof optionsOrRunner === 'function' ? {} : optionsOrRunner;
    const runner = typeof optionsOrRunner === 'function' ? optionsOrRunner : maybeRunner;

    return withManagedEnv(rootDir, options, async () => {
        const _uniquePort = getNextPort();
        const userConfig = buildMacroforgeViteConfig();

        const server = await createServer({
            root: rootDir,
            configFile: false,
            ...userConfig,
            logLevel: 'error',
            appType: 'custom',
            server: {
                ...userConfig.server,
                middlewareMode: true,
                hmr: false,
                ws: false
            },
            optimizeDeps: {
                noDiscovery: true,
                include: []
            }
        });

        return { server, run: () => runner(server) };
    });
}

/**
 * Helper to create a real HTTP Vite dev server for testing.
 * Unlike withViteServer(), this starts a listening HTTP server
 * that can be hit with fetch() requests.
 *
 * The runner receives (server, baseUrl) where baseUrl is e.g. "http://localhost:24703".
 */
export async function withDevServer(rootDir, optionsOrRunner, maybeRunner) {
    const options = typeof optionsOrRunner === 'function' ? {} : optionsOrRunner;
    const runner = typeof optionsOrRunner === 'function' ? optionsOrRunner : maybeRunner;

    return withManagedEnv(rootDir, options, async () => {
        const uniquePort = getNextPort();
        const userConfig = buildMacroforgeViteConfig();

        const server = await createServer({
            root: rootDir,
            configFile: false,
            ...userConfig,
            logLevel: 'error',
            appType: 'spa',
            server: {
                port: uniquePort,
                strictPort: true,
                host: 'localhost',
                hmr: false
            },
            optimizeDeps: {
                noDiscovery: true,
                include: []
            }
        });

        await server.listen(uniquePort);
        const baseUrl = `http://localhost:${uniquePort}`;

        return { server, run: () => runner(server, baseUrl) };
    });
}
