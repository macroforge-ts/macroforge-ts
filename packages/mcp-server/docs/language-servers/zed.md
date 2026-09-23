# Zed Extensions

Macroforge provides two extensions for the [Zed editor](https://zed.dev): one for TypeScript via
VTSLS, and one for Svelte.

Developer Installation Required

These extensions are not yet in the Zed extension registry. You'll need to install them as developer
extensions.

## Available Extensions

| Extension           | Description                                  | Location                              |
| ------------------- | -------------------------------------------- | ------------------------------------- |
| `vtsls-macroforge`  | VTSLS with macroforge support for TypeScript | `crates/extensions/vtsls_macroforge`  |
| `svelte-macroforge` | Svelte language support with macroforge      | `crates/extensions/svelte_macroforge` |

## Installation

### 1\. Clone the Repository

Bash

```
git clone https://gitlab.com/macroforge-ts/macroforge-ts.git
cd macroforge-ts
```

### 2\. Build the Extension

Build the extension you want to use:

Bash

```
# For VTSLS (TypeScript)
cd crates/extensions/vtsls_macroforge

# Or for Svelte
cd crates/extensions/svelte_macroforge
```

### 3\. Install as Dev Extension in Zed

In Zed, open the command palette and run **zed: install dev extension**, then select the extension
directory.

Alternatively, symlink the extension to your Zed extensions directory:

Bash

```
# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls_macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls_macroforge ~/.config/zed/extensions/installed/vtsls-macroforge
```

## vtsls-macroforge

This extension wraps [VTSLS](https://github.com/yioneko/vtsls) (a TypeScript language server) with
macroforge integration. See the
[dedicated vtsls-macroforge page](../../docs/language-servers/vtsls-macroforge) for full
documentation.

## svelte-macroforge

This extension provides Svelte support using the `@macroforge/svelte-language-server`. It includes:

- Svelte component syntax support
- HTML, CSS, and TypeScript features
- Macroforge integration in script blocks

## Troubleshooting

### Extension not loading

Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error
messages.

### Macros not expanding

Ensure your project has the `macroforge` package installed and a valid `tsconfig.json` with the
TypeScript plugin configured.
