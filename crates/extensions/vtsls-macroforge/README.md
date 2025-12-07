# Macroforge TypeScript Extension

> **Warning:** This is a work in progress and probably won't work for you. Use at your own risk!

A Zed extension that provides TypeScript language server support with the `@macroforge/typescript-plugin` pre-configured for compile-time macros.

## How it works

This extension:
1. Downloads `@vtsls/language-server` and `@macroforge/typescript-plugin` from npm
2. Launches vtsls as the language server
3. Configures vtsls with the macroforge plugin via `globalPlugins`

## Usage

1. Install the extension in Zed (or use as a dev extension)
2. Configure your `.zed/settings.json` to use `macroforge-ts`:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["macroforge-ts", "!vtsls", "!typescript-language-server"]
    }
  }
}
```

3. Classes with `@derive` annotations will now have augmented types in the editor.
