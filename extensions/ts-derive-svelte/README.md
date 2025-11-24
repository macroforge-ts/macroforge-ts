# TS Derive Svelte Language Server

Builds and runs the forked `svelte-language-server` bundled under `extensions/ts-derive-svelte/language-tools` so `.svelte` files inherit the ts-macros TypeScript plugin fix.

On first launch the extension runs:

1. `npm install --workspaces --prefix extensions/ts-derive-svelte/language-tools`
2. `npm run build --workspace svelte-language-server --prefix extensions/ts-derive-svelte/language-tools`

Afterwards it spawns `node extensions/ts-derive-svelte/language-tools/packages/language-server/bin/server.js --stdio` whenever Zed needs the Svelte language server.
