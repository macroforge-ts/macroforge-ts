# Installation

*Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros.*

## Requirements

- Node.js 24.0 or later

- TypeScript 5.9 or later

## Install the Package

Install Macroforge using your preferred package manager:

`npm`
```bash
npm install macroforge
```

`bun`
```bash
bun add macroforge
```

`pnpm`
```bash
pnpm add macroforge
```

> **Note:**
> Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64).

## Basic Usage

The simplest way to use Macroforge is with the built-in derive macros. Add a `@derive` comment decorator to your class:

`user.ts`
```typescript
import { Debug, Clone, Eq } from "macroforge";

/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

// After macro expansion, User has:
// - toString(): string         (from Debug)
// - clone(): User              (from Clone)
// - equals(other: User): boolean  (from Eq)
```

## IDE Integration

For the best development experience, add the TypeScript plugin to your `tsconfig.json`:

`tsconfig.json`
```json
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "@macroforge/typescript-plugin"
      }
    ]
  }
}
```

This enables features like:

- Accurate error positions in your source code

- Autocompletion for generated methods

- Type checking for expanded code

## Build Integration (Vite)

If you're using Vite, add the plugin to your config for automatic macro expansion during build:

`vite.config.ts`
```typescript
import macroforge from "@macroforge/vite-plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    macroforge({
      generateTypes: true,
      typesOutputDir: ".macroforge/types"
    })
  ]
});
```

## Next Steps

Now that you have Macroforge installed, learn how to use it:

- [Create your first macro]({base}/docs/getting-started/first-macro)

- [Understand how macros work]({base}/docs/concepts)

- [Explore built-in macros]({base}/docs/builtin-macros)