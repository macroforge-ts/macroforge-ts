# TS Derive Svelte Language Server

Builds and runs the forked `svelte-language-server` found in `node_modules/svelte-language-server` (after a repo-wide `npm install`) or under `packages/language-server` in the current workspace so `.svelte` files inherit the ts-macros TypeScript plugin fix.

Before launching Zed, ensure the language server artifacts exist:

1. `npm install`
2. (Optional) If you're iterating on the fork in `packages/language-server`, run `npm install --workspace packages/language-server` and `npm run build --workspace packages/language-server`.

Afterwards it spawns `node node_modules/svelte-language-server/bin/server.js --stdio` (falling back to `packages/language-server/bin/server.js` if you're running the fork directly) whenever Zed needs the Svelte language server.
