Development Plan: Rust NAPI Macros for Vite

This document outlines the step-by-step execution plan to build a high-performance, Rust-powered macro system for TypeScript using Vite, NAPI-RS, and SWC.

## Current Status (2024-11-23)

### Completed
- ✅ Monorepo structure setup with npm workspaces
- ✅ Rust crate scaffolded with NAPI-RS bindings
- ✅ Vite playground created with TypeScript configuration
- ✅ Core Rust transformation logic implemented (lib.rs and macros.rs)
- ✅ IncludeStr macro for file inclusion at compile-time
- ✅ @Derive decorator for auto-generating toString() and toJSON() methods
- ✅ Vite plugin created to integrate Rust transformer
- ✅ TypeScript type definitions for macros

### Current Blocker
⚠️ **Build Issue**: The Rust binary compilation is failing due to a serde compatibility issue in swc_common v0.40.2. The error is: `serde::__private` not found. This appears to be a known issue with certain versions of SWC and serde.

### Next Steps
1. **Resolve the build issue** by either:
   - Downgrading to an older, stable version of swc_core (e.g., v0.90.x)
   - Using a patched version of swc_common
   - Waiting for an upstream fix
   - Creating a minimal reproduction and filing an issue with SWC

2. **Once the build works**:
   - Run the smoke test
   - Test the macros in the playground
   - Implement proper source maps
   - Add more comprehensive error handling

3. **Future enhancements**:
   - Add more macro types (e.g., compile-time constants, auto-serialization)
   - Implement hot module replacement (HMR) support
   - Set up CI/CD for multi-platform builds

1. Architecture Overview

The system operates as a "Hybrid Compiler":

Vite (Node.js): Acts as the file watcher and dev server. It intercepts .ts files.

Bridge (NAPI): A zero-cost FFI (Foreign Function Interface) that passes strings from Node to Rust.

Kernel (Rust/SWC): A single binary that parses TypeScript, modifies the AST (Abstract Syntax Tree), and emits JavaScript.

Data Flow:
Browser Request -> Vite -> Plugin -> Rust Binary (Transform) -> JS String -> Browser

2. Phased Execution

Phase 1: Environment Scaffolding

Goal: Establish the build chain for both Rust and TypeScript.

[x] Initialize Monorepo

Create root with package.json.

Install vite, typescript, @napi-rs/cli.

[x] Scaffold Rust Crate

Run napi new crates/swc-napi-macros.

Configure Cargo.toml with swc_core (features: ecma_parser, ecma_codegen, ecma_visit).

[x] Scaffold Test Playground

Create playground/ (Vite + TS template).

Link the local Rust binary to this project for testing.

Phase 2: The Rust Kernel (MVP)

Goal: Create a binary that can parse TS and print it back without crashing.

[x] Implement transform_sync in src/lib.rs.

Setup SourceMap (Lrc).

Configure Parser: Use Lexer with TsConfig explicitly enabled for decorators and tsx.

Setup Emitter (Code Generation).

[x] NAPI Exposure

Use #[napi] attribute to expose the function to Node.js.

Compile with npm run build:rust.

[ ] Smoke Test - BLOCKED

Write a Node script: require('./index.node').transform_sync('const x: number = 1;').

Expect output: const x = 1; (Types stripped).

⚠️ **Current Blocker**: SWC v0.40.2 has a serde compatibility issue (`serde::__private` not found). Need to resolve version conflicts or patch the issue.

Phase 3: The Macro Logic

Goal: Implement the actual AST transformations.

[x] File System Macro (IncludeStr)

Implement VisitMut trait.

Detect CallExpr with name IncludeStr.

Resolve paths relative to the current file (pass filepath from JS to Rust).

Use std::fs::read_to_string and replace the node with a StringLiteral.

[x] Decorator Macro (@Derive)

Detect ClassDeclaration decorators.

Parse arguments (Debug, JSON).

AST Generation:

Create toString() method AST manually.

Create toJSON() method AST manually.

Inject methods into class.body.

Remove the decorator to prevent runtime errors.

Phase 4: The TypeScript Bridge

Goal: Ensure excellent Developer Experience (DX).

[x] Vite Plugin (vite-plugin-napi.ts)

Use createRequire to load the .node binary.

Implement transform() hook.

Add error handling (try/catch around Rust calls).

[x] Type Definitions (macros.d.ts)

Declare functions: IncludeStr, Derive.

Define interfaces Debug and JSON.

[ ] Documentation & Patterns

Document the "Declaration Merging" pattern (interface User extends Debug {}) to ensure users get autocomplete for methods injected by Rust. This is critical for LSP support.

Phase 5: Distribution (CI/CD)

Goal: Publish cross-platform binaries.

[ ] GitHub Actions

Use napi-rs standard workflow.

Matrix build: windows-latest, ubuntu-latest, macos-latest.

[ ] NPM Structure

Main package: vite-rust-macros.

Optional dependencies: vite-rust-macros-win32-x64, vite-rust-macros-darwin-arm64, etc.

The loader script detects OS and requires the correct binary.

3. Project Structure

/
├── Cargo.toml                # Rust Workspace
├── package.json              # Node Monorepo
├── crates/
│   └── swc-napi-macros/      # Rust Source
│       ├── Cargo.toml
│       └── src/lib.rs        # The Compiler Driver
├── packages/
│   └── vite-plugin/          # JS Adapter
│       ├── index.ts
│       └── macros.d.ts
└── playground/               # Manual Testing
    └── src/main.ts


4. Verification Checklist

[ ] Startup Speed: Vite should start immediately (Rust compilation is AOT).

[ ] Hot Reload: Changing a macro argument (e.g., IncludeStr path) should trigger HMR.

[ ] Type Safety: VS Code should not show red squiggles for macros.

[ ] Platform Support: Verify .node loading on Linux/Mac/Windows.
