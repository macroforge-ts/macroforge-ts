# Macroforge Svelte Extension

> **Warning:** This is a work in progress and probably won't work for you. Use at your own risk!

A Zed extension that runs the macroforge-aware Svelte language server, so `.svelte` files get
macroforge macro expansion in the editor.

On first use it installs two pinned npm packages into its own directory:
`@macroforge/svelte-language-server` and `@macroforge/core` at the same version. It then starts the
language server's `svelteserver` command, taken from that package's `bin` field, with Zed's bundled
Node.js over stdio.

To try a local build, install this directory with Zed's **zed: install dev extension** command.
