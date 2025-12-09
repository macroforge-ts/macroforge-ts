# Svelte Language Server

*`@macroforge/svelte-language-server` provides full Svelte IDE support with macroforge integration.*

<Alert type="warning" title="Developer Installation Required">
This package is not yet published as an official extension. You'll need to build and install it manually.
</Alert>

## Features

- **Svelte syntax diagnostics** - Errors and warnings in .svelte files

- **HTML support** - Hover info, autocompletions, Emmet, outline symbols

- **CSS/SCSS/LESS** - Diagnostics, hover, completions, formatting, Emmet, color picking

- **TypeScript/JavaScript** - Full language features with macroforge macro expansion

- **Go-to-definition** - Navigate to macro-generated code

- **Code actions** - Quick fixes and refactorings

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts
```

### 2. Build the Language Server

```bash
# Install dependencies
npm install

# Build the Svelte language server
cd packages/svelte-language-server
npm run build
```

### 3. Configure Your Editor

The language server exposes a `svelteserver` binary that implements the Language Server Protocol (LSP). Configure your editor to use it:

```bash
# The binary is located at:
./packages/svelte-language-server/bin/server.js
```

## Package Info

| Package 
| `@macroforge/svelte-language-server` 

| Version 
| 0.1.7 

| CLI Command 
| `svelteserver` 

| Node Version 
| >= 18.0.0

## How It Works

The Svelte language server extends the standard Svelte language tooling with macroforge integration:

1. Parses `.svelte` files and extracts TypeScript/JavaScript blocks

2. Expands macros using the `@macroforge/typescript-plugin`

3. Maps diagnostics back to original source positions

4. Provides completions for macro-generated methods

## Using with Zed

For Zed editor, see the [Zed Extensions]({base}/docs/language-servers/zed) page for the dedicated `svelte-macroforge` extension.