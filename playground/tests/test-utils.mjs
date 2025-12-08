/**
 * Shared test utilities for playground tests
 */

import fs from "node:fs";
import path from "node:path";
import { createServer } from "vite";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const playgroundRoot = path.resolve(__dirname, "..");
export const repoRoot = path.resolve(playgroundRoot, "..");
export const vanillaRoot = path.join(playgroundRoot, "vanilla");
export const svelteRoot = path.join(playgroundRoot, "svelte");
export const rootConfigPath = path.join(repoRoot, "macroforge.json");

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
export async function withViteServer(rootDir, optionsOrRunner, maybeRunner) {
  const options = typeof optionsOrRunner === "function" ? {} : optionsOrRunner;
  const runner =
    typeof optionsOrRunner === "function" ? optionsOrRunner : maybeRunner;
  const { useProjectCwd = true } = options ?? {};

  const configFile = path.join(rootDir, "vite.config.ts");
  const previousCwd = process.cwd();
  let server;
  let copiedConfig = false;
  const localConfigPath = path.join(rootDir, "macroforge.json");

  try {
    if (useProjectCwd) {
      process.chdir(rootDir);
    }

    // Copy the workspace-level config so the macro host loads the shared macro packages
    if (!fs.existsSync(localConfigPath) && fs.existsSync(rootConfigPath)) {
      fs.copyFileSync(rootConfigPath, localConfigPath);
      copiedConfig = true;
    }

    const uniquePort = getNextPort();

    server = await createServer({
      root: rootDir,
      configFile,
      logLevel: "error",
      appType: "custom",
      server: {
        middlewareMode: true,
        hmr: {
          // Use unique port for each server to prevent conflicts
          port: uniquePort,
        },
        // Disable WebSocket server entirely for SSR-only tests
        ws: false,
      },
      optimizeDeps: {
        disabled: true,
      },
    });

    return await runner(server);
  } finally {
    if (server) {
      // Give a small grace period for pending requests to settle
      await new Promise((resolve) => setTimeout(resolve, 100));
      await server.close();
    }
    if (copiedConfig && fs.existsSync(localConfigPath)) {
      fs.rmSync(localConfigPath);
    }
    if (useProjectCwd) {
      process.chdir(previousCwd);
    }
  }
}
