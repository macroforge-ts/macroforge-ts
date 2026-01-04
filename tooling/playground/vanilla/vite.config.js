import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

const __dirname = dirname(fileURLToPath(import.meta.url));

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
  resolve: {
    dedupe: ["effect"],
  },
});
