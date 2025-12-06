# TS Derive TypeScript Language Server

This extension launches the local `@vtsls/language-server` binary from `node_modules` and injects the `@macroforge/tsserver-plugin-macroforge` so TypeScript/TSX files expose macro APIs.

Usage:

1. `npm install` at the repo root so `node_modules/@vtsls/language-server/bin/vtsls.js` exists.
2. `npm run build -w @macroforge/tsserver-plugin-macroforge` whenever the plugin source changes.
3. Point TypeScript/TSX languages at the `macroforge` server in Zed settings.
