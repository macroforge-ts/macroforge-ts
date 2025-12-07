# macroforge

TypeScript macro expansion engine powered by Rust and SWC.

## Overview

`macroforge` is a native Node.js module that enables compile-time code generation for TypeScript through a Rust-like derive macro system. It parses TypeScript using SWC, expands macros written in Rust, and outputs transformed TypeScript code with full source mapping support.

## Installation

```bash
npm install macroforge
```

The package includes pre-built binaries for:
- macOS (x64, arm64)
- Linux (x64, arm64)
- Windows (x64, arm64)

## Quick Start

### Using Built-in Macros

```typescript
import { Derive, Debug, Clone, Eq } from "macroforge";

/** @derive(Debug, Clone, Eq) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

// After macro expansion, the class gains:
// - toString(): string           (from Debug)
// - clone(): User                (from Clone)
// - equals(other: User): boolean (from Eq)
```

### Programmatic API

```typescript
import { expandSync, NativePlugin } from "macroforge";

// One-shot expansion
const result = expandSync(sourceCode, "file.ts", {
  keepDecorators: false
});

console.log(result.code);        // Transformed TypeScript
console.log(result.diagnostics); // Any warnings/errors
console.log(result.metadata);    // Macro IR metadata (JSON)

// Cached expansion (for language servers)
const plugin = new NativePlugin();
const cached = plugin.processFile("file.ts", sourceCode, {
  version: "1.0.0" // Cache key
});
```

## API Reference

### Core Functions

#### `expandSync(code, filepath, options?)`

Expands macros in TypeScript code synchronously.

```typescript
function expandSync(
  code: string,
  filepath: string,
  options?: ExpandOptions
): ExpandResult;

interface ExpandOptions {
  keepDecorators?: boolean; // Keep @derive decorators in output (default: false)
}

interface ExpandResult {
  code: string;                        // Transformed TypeScript
  types?: string;                      // Generated .d.ts content
  metadata?: string;                   // Macro IR as JSON
  diagnostics: MacroDiagnostic[];      // Warnings and errors
  sourceMapping?: SourceMappingResult; // Position mapping data
}
```

#### `transformSync(code, filepath)`

Lower-level transform that returns additional metadata.

```typescript
function transformSync(code: string, filepath: string): TransformResult;

interface TransformResult {
  code: string;
  map?: string;    // Source map (not yet implemented)
  types?: string;  // Generated declarations
  metadata?: string;
}
```

#### `checkSyntax(code, filepath)`

Validates TypeScript syntax without macro expansion.

```typescript
function checkSyntax(code: string, filepath: string): SyntaxCheckResult;

interface SyntaxCheckResult {
  ok: boolean;
  error?: string;
}
```

#### `parseImportSources(code, filepath)`

Extracts import information from TypeScript code.

```typescript
function parseImportSources(code: string, filepath: string): ImportSourceResult[];

interface ImportSourceResult {
  local: string;  // Local identifier name
  module: string; // Module specifier
}
```

### Classes

#### `NativePlugin`

Stateful plugin with caching for language server integration.

```typescript
class NativePlugin {
  constructor();

  // Process file with version-based caching
  processFile(
    filepath: string,
    code: string,
    options?: ProcessFileOptions
  ): ExpandResult;

  // Get position mapper for a cached file
  getMapper(filepath: string): NativeMapper | null;

  // Map diagnostics from expanded to original positions
  mapDiagnostics(filepath: string, diags: JsDiagnostic[]): JsDiagnostic[];

  // Logging utilities
  log(message: string): void;
  setLogFile(path: string): void;
}
```

#### `NativeMapper` / `PositionMapper`

Maps positions between original and expanded code.

```typescript
class NativeMapper {
  constructor(mapping: SourceMappingResult);

  isEmpty(): boolean;
  originalToExpanded(pos: number): number;
  expandedToOriginal(pos: number): number | null;
  generatedBy(pos: number): string | null;
  mapSpanToOriginal(start: number, length: number): SpanResult | null;
  mapSpanToExpanded(start: number, length: number): SpanResult;
  isInGenerated(pos: number): boolean;
}
```

### Built-in Decorators

| Decorator | Description |
|-----------|-------------|
| `@Derive(...features)` | Class decorator that triggers macro expansion |
| `@Debug` | Generates `toString(): string` method |
| `@Clone` | Generates `clone(): T` method |
| `@Eq` | Generates `equals(other: T): boolean` method |

## Writing Custom Macros

Custom macros are written in Rust and compiled to native Node.js addons. See the [playground/macro](../../playground/macro) directory for examples.

### Minimal Macro Crate

**Cargo.toml:**
```toml
[package]
name = "my-macros"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
macroforge_ts = "0.1.0"
napi = { version = "3.5.2", features = ["napi8", "compat-mode"] }
napi-derive = "3.3.3"

[build-dependencies]
napi-build = "2.3.1"
```

**src/lib.rs:**
```rust
use macroforge_ts::ts_macro_derive::ts_macro_derive;
use macroforge_ts::ts_quote::body;
use macroforge_ts::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            Ok(body! {
                toJSON(): Record<string, unknown> {
                    return {
                        {#for field in class.field_names()}
                            @{field}: this.@{field},
                        {/for}
                    };
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    }
}
```

### Template Syntax

The `ts_template!` and `body!` macros support:

| Syntax | Description |
|--------|-------------|
| `@{expr}` | Interpolate Rust expression as identifier/code |
| `{#for x in iter}...{/for}` | Loop over iterables |
| `{%let name = expr}` | Local variable binding |
| `{#if cond}...{/if}` | Conditional blocks |

### Re-exported Crates

`macroforge_ts` re-exports everything needed for macro development:

```rust
// All available via macroforge_ts::*
pub extern crate ts_syn;         // AST types, parsing
pub extern crate ts_quote;       // Code generation templates
pub extern crate ts_macro_derive; // #[ts_macro_derive] attribute
pub extern crate inventory;       // Macro registration
pub extern crate serde_json;      // Serialization
pub extern crate napi;            // Node.js bindings
pub extern crate napi_derive;     // NAPI proc-macros

// SWC modules
pub use ts_syn::swc_core;
pub use ts_syn::swc_common;
pub use ts_syn::swc_ecma_ast;
```

## Integration

### Vite Plugin

```typescript
// vite.config.ts
import macroforge from "@macroforge/vite-plugin";

export default defineConfig({
  plugins: [
    macroforge({
      typesOutputDir: ".macroforge/types",
      metadataOutputDir: ".macroforge/meta",
      generateTypes: true,
      emitMetadata: true,
    }),
  ],
});
```

### TypeScript Plugin

Add to `tsconfig.json` for IDE support:

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

## Debug API

For debugging macro registration:

```typescript
import {
  __macroforgeGetManifest,
  __macroforgeGetMacroNames,
  __macroforgeIsMacroPackage,
  __macroforgeDebugDescriptors,
  __macroforgeDebugLookup,
} from "macroforge";

// Get all registered macros
const manifest = __macroforgeGetManifest();
console.log(manifest.macros);

// Check if current package exports macros
console.log(__macroforgeIsMacroPackage());

// List macro names
console.log(__macroforgeGetMacroNames());

// Debug lookup
console.log(__macroforgeDebugLookup("@my/macros", "JSON"));
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Node.js / Vite                       │
├─────────────────────────────────────────────────────────┤
│                   NAPI-RS Bindings                      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │   ts_syn    │  │   ts_quote   │  │ts_macro_derive│  │
│  │  (parsing)  │  │ (templating) │  │  (proc-macro) │  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
├─────────────────────────────────────────────────────────┤
│                    SWC Core                             │
│            (TypeScript parsing & codegen)               │
└─────────────────────────────────────────────────────────┘
```

## Performance

- **Thread-safe expansion**: Each expansion runs in an isolated thread with a 32MB stack to handle deep AST recursion
- **Caching**: `NativePlugin` caches expansion results by file version
- **Binary search**: Position mapping uses O(log n) lookups
- **Zero-copy parsing**: SWC's arena allocator minimizes allocations

## License

MIT
