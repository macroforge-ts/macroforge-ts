import { defineConfig } from "vite";
import macroforge from "@macroforge/vite-plugin";
import { resolve } from "node:path";

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
