# vtsls-macroforge

A Zed extension that wraps [VTSLS](https://github.com/yioneko/vtsls) (VS Code's TypeScript language
server) with the `@macroforge/typescript-plugin` pre-configured for compile-time macros.

Developer Installation Required

This extension is not yet in the Zed extension registry. You'll need to install it as a developer
extension.

## Features

- Full TypeScript language features via VTSLS
- Macro expansion at edit time
- Accurate error positions in original source
- Completions for macro-generated methods
- Automatic download of `@vtsls/language-server` and `@macroforge/typescript-plugin`
- Platform-aware native binary installation

## How It Works

The extension:

1. Downloads `@vtsls/language-server` and `@macroforge/typescript-plugin` from npm
2. Launches VTSLS as the language server
3. Configures VTSLS with the macroforge plugin via `globalPlugins`

## Installation

### 1\. Clone the Repository

Bash

```
git clone https://gitlab.com/macroforge-ts/macroforge-ts.git
cd macroforge-ts/crates/extensions/vtsls_macroforge
```

### 2\. Install as Dev Extension in Zed

In Zed, open the command palette and run **zed: install dev extension**, then select the
`vtsls-macroforge` directory.

Alternatively, symlink to your Zed extensions directory:

Bash

```
# macOS
ln -s /path/to/macroforge-ts/crates/extensions/vtsls_macroforge ~/Library/Application\\ Support/Zed/extensions/installed/vtsls-macroforge

# Linux
ln -s /path/to/macroforge-ts/crates/extensions/vtsls_macroforge ~/.config/zed/extensions/installed/vtsls-macroforge
```

### 3\. Configure Zed Settings

Add this to your `.zed/settings.json`:

.zed/settings.json

```
{
  "languages": {
    "TypeScript": {
      "language_servers": ["macroforge-ts", "!vtsls", "!typescript-language-server"]
    }
  }
}
```

## Supported Languages

| Language   | Supported |
| ---------- | --------- |
| TypeScript | Yes       |
| TSX        | Yes       |
| JavaScript | Yes       |

## Troubleshooting

### Extension not loading

Make sure you've restarted Zed after installing the extension. Check the Zed logs for any error
messages.

### Macros not expanding

Ensure your project has the `macroforge` package installed. The extension handles the TypeScript
plugin configuration automatically — you don't need to add it to your `tsconfig.json` when using
this extension.
