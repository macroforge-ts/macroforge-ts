# Installation
 *Get started with Macroforge in just a few minutes. Install the package and configure your project to start using TypeScript macros.*
 ## Requirements
 - Node.js 24.0 or later
 - TypeScript 5.9 or later
 ## Install the Package
 Install Macroforge using your preferred package manager:
 ```
npm install macroforge
``` ```
bun add macroforge
``` ```
pnpm add macroforge
```  **Info Macroforge includes pre-built native binaries for macOS (x64, arm64), Linux (x64, arm64), and Windows (x64, arm64). ## Basic Usage
 The simplest way to use Macroforge is with the built-in derive macros. Add a `@derive` comment decorator to your class:
 ```
/** @derive(Debug, Clone, PartialEq) */
class User &#123;
  name: string;
  age: number;

  constructor(name: string, age: number) &#123;
    this.name = name;
    this.age = age;
  &#125;
&#125;

// After macro expansion, User has:
// - toString(): string              (from Debug)
// - clone(): User                   (from Clone)
// - equals(other: unknown): boolean (from PartialEq)
``` ## IDE Integration
 For the best development experience, add the TypeScript plugin to your `tsconfig.json`:
 ```
&#123;
  "compilerOptions": &#123;
    "plugins": [
      &#123;
        "name": "@macroforge/typescript-plugin"
      &#125;
    ]
  &#125;
&#125;
``` This enables features like:
 - Accurate error positions in your source code
 - Autocompletion for generated methods
 - Type checking for expanded code
 ## Build Integration (Vite)
 If you're using Vite, add the plugin to your config for automatic macro expansion during build:
 ```
import macroforge from "@macroforge/vite-plugin";
import &#123; defineConfig &#125; from "vite";

export default defineConfig(&#123;
  plugins: [
    macroforge(&#123;
      generateTypes: true,
      typesOutputDir: ".macroforge/types"
    &#125;)
  ]
&#125;);
``` ## Next Steps
 Now that you have Macroforge installed, learn how to use it:
 - [Create your first macro](../docs/getting-started/first-macro)
 - [Understand how macros work](../docs/concepts)
 - [Explore built-in macros](../docs/builtin-macros)
**