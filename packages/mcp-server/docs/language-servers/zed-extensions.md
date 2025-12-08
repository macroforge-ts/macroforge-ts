# Zed Extensions

*Macroforge provides two extensions for the [Zed editor](https://zed.dev): one for TypeScript via VTSLS, and one for Svelte.*

<Alert type="warning" title="Developer Installation Required">
These extensions are not yet in the Zed extension registry. You'll need to install them as developer extensions.
</Alert>

## Available Extensions

| `vtsls-macroforge` 
| VTSLS with macroforge support for TypeScript 
| `crates/extensions/vtsls-macroforge` 

| `svelte-macroforge` 
| Svelte language support with macroforge 
| `crates/extensions/svelte-macroforge`

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts
```

### 2. Build the Extension

Build the extension you want to use:

```bash
# For VTSLS (TypeScript)
cd crates/extensions/vtsls-macroforge

# Or for Svelte
cd crates/extensions/svelte-macroforge
```

### 3. Install as Dev Extension in Zed

In Zed, open the command palette and run **zed: install dev extension**, then select the extension directory.

Alternatively, symlink the extension to your Zed extensions directory:

```bash
# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls-macroforge ~/.config/zed/extensions/installed/vtsls-macroforge
```

## vtsls-macroforge

This extension wraps [VTSLS](https://github.com/yioneko/vtsls) (a TypeScript language server) with macroforge integration. It provides:

- Full TypeScript language features

- Macro expansion at edit time

- Accurate error positions in original source

- Completions for macro-generated methods

## svelte-macroforge

This extension provides Svelte support using the `@macroforge/svelte-language-server`. It includes:

- Svelte component syntax support

- HTML, CSS, and TypeScript features

- Macroforge integration in script blocks

## Troubleshooting

### Extension not loading

Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error messages.

### Macros not expanding

Ensure your project has the `macroforge` package installed and a valid `tsconfig.json` with the TypeScript plugin configured.