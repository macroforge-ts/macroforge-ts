/**
 * Shared test utilities for playground tests
 */

import fs from 'node:fs';
import path from 'node:path';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';
import macroforge from '@macroforge/vite-plugin';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const playgroundRoot = path.resolve(__dirname, '..');
// Derive repo root from file location (tests/ -> playground/ -> tooling/ -> repo root)
export const repoRoot = globalThis.process.env.MACROFORGE_ROOT ||
    path.resolve(__dirname, '..', '..', '..');
export const vanillaRoot = path.join(playgroundRoot, 'vanilla');
export const svelteRoot = path.join(playgroundRoot, 'svelte');
export const rootConfigPath = path.join(repoRoot, 'macroforge.config.ts');

/**
 * `MACROFORGE_CLI` when set, otherwise this checkout's debug build. Never the
 * `macroforge` on PATH, which other projects pin to their own version.
 */
export const cliBinary = (() => {
    const binary = globalThis.process.env.MACROFORGE_CLI ||
        path.join(repoRoot, 'target', 'debug', 'macroforge');
    if (!existsSync(binary)) {
        throw new Error(
            `macroforge CLI not found at ${binary}; build it with \`pixi run build:cli\``
        );
    }
    return binary;
})();

/**
 * Run the macroforge CLI binary and capture output.
 */
export function runCli(args, options = {}) {
    const command = new Deno.Command(cliBinary, {
        args,
        cwd: options.cwd || globalThis.process.cwd(),
        stdout: 'piped',
        stderr: 'piped'
    });
    const result = command.outputSync();
    const decoder = new TextDecoder();
    return {
        stdout: decoder.decode(result.stdout),
        stderr: decoder.decode(result.stderr),
        status: result.code,
        success: result.success
    };
}

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
            dedupe: ['effect']
        }
    };
}

/**
 * Manage cwd, macroforge.config.ts copying, and cleanup around a server lifecycle.
 */
async function withManagedEnv(rootDir, options, serverFactory) {
    const { useProjectCwd = true } = options ?? {};
    const previousCwd = globalThis.process.cwd();
    let server;
    let copiedConfig = false;
    const localConfigPath = path.join(rootDir, 'macroforge.config.ts');

    try {
        if (useProjectCwd) {
            globalThis.process.chdir(rootDir);
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
            globalThis.process.chdir(previousCwd);
        }
    }
}

export function withViteServer(rootDir, optionsOrRunner, maybeRunner) {
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
export function withDevServer(rootDir, optionsOrRunner, maybeRunner) {
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
