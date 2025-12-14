# Language Servers
 *Macroforge provides language server integrations for enhanced IDE support beyond the TypeScript plugin.*
  **Work in Progress Language server integrations are currently experimental. They work in the repository but are not yet published as official extensions. You'll need to fork the repo and install them as developer extensions. ## Overview
 While the [TypeScript Plugin](../docs/integration/typescript-plugin) provides macro support in any TypeScript-aware editor, dedicated language servers offer deeper integration for specific frameworks and editors.
 | Integration | Purpose | Status |
| --- | --- | --- |
| [Svelte Language Server](../docs/language-servers/svelte) | Full Svelte support with macroforge | Working (dev install) |
| [Zed Extensions](../docs/language-servers/zed) | VTSLS and Svelte for Zed editor | Working (dev install) |
 ## Current Status
 The language servers are functional and used during development of macroforge itself. However, they require manual installation:
 1. Fork or clone the [macroforge-ts repository](https://github.com/rymskip/macroforge-ts)
 2. Build the extension you need
 3. Install it as a developer extension in your editor
 See the individual pages for detailed installation instructions.
 ## Roadmap
 We're working on official extension releases for:
 - VS Code (via VTSLS)
 - Zed (native extensions)
 - Other editors with LSP support
 ## Detailed Guides
 - [Svelte Language Server](../docs/language-servers/svelte) - Full Svelte IDE support
 - [Zed Extensions](../docs/language-servers/zed) - VTSLS and Svelte for Zed
**