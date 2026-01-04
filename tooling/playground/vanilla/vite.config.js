import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Runtime imports from env variables (no fallbacks - must be set)
const { VITE_PLUGIN_PKG, MACROFORGE_TS_CRATE, PLAYGROUND_MACRO, SHARED_PKG } = process.env;

const { macroforge } = await import(`${VITE_PLUGIN_PKG}/src/index.js`);

export default defineConfig({
  plugins: [macroforge()],
  server: {
    port: 3000,
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        "validator-form": resolve(__dirname, "validator-form.html"),
      },
    },
  },
  ssr: {
    noExternal: ["effect", "@playground/macro"],
  },
  optimizeDeps: {
    exclude: ["@playground/macro"],
  },
  resolve: {
    dedupe: ["effect"],
    alias: [
      // macroforge subpaths (explicit mappings to match package.json exports)
      { find: "macroforge/serde", replacement: `${MACROFORGE_TS_CRATE}/js/serde/index.mjs` },
      { find: "macroforge/traits", replacement: `${MACROFORGE_TS_CRATE}/js/traits/index.mjs` },
      { find: "macroforge/reexports/effect", replacement: `${MACROFORGE_TS_CRATE}/js/reexports/effect.mjs` },
      { find: "macroforge/reexports", replacement: `${MACROFORGE_TS_CRATE}/js/reexports/index.mjs` },
      { find: "macroforge", replacement: MACROFORGE_TS_CRATE },
      { find: "@playground/macro", replacement: PLAYGROUND_MACRO },
      { find: "@macroforge/vite-plugin", replacement: VITE_PLUGIN_PKG },
      { find: "@macroforge/shared", replacement: SHARED_PKG },
    ],
  },
});
