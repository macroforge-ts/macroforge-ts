import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { macroforge } from "@macroforge/vite-plugin";

export default defineConfig({
  plugins: [macroforge(), sveltekit()],
  ssr: {
    noExternal: ["effect", "@playground/macro"],
  },
  resolve: {
    dedupe: ["effect"],
  },
});
