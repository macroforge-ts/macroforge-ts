# Gemini Workspace Context: `macroforge`

This document provides an overview of the `macroforge` project, its structure, and key commands to get you started with development.

## Project Overview

`macroforge` is a high-performance, Rust-powered macro system for TypeScript. It integrates with Vite and uses SWC for AST manipulation and NAPI-RS to bridge the gap between the Rust backend and the Node.js environment.

The project is a monorepo containing both the Rust macro execution engine and the TypeScript/JavaScript packages that form the user-facing plugins and language server.

### Core Components:

*   **Rust Workspace (`crates/`):**
    *   `ts_macro_host`: The core macro-running engine.
    *   `swc-napi-macros`: Provides the N-API interface to expose the Rust functionality to Node.js.
    *   `ts_macro_abi`: Defines the data structures for communication between the host and macros.
    *   `ts_syn`, `ts_quote`: Utilities for parsing and generating TypeScript code within Rust macros.
    *   `extensions/*`: Home to various macro implementations.

*   **TypeScript Workspace (`packages/`):**
    *   `vite-plugin`: The Vite plugin that integrates the macro expansion into the build process.
    *   `language-server`: A language server implementation for providing IDE features like autocompletion and diagnostics for macros.

*   **Configuration (`macroforge.json`):**
    *   A project-specific configuration file that controls which macro packages are loaded and sets execution limits for the macro engine.

## Building and Running

### Prerequisites

*   Node.js (version >= 24.9.0)
*   Rust (stable)
*   `npm` (or a compatible package manager)

### Build Commands

*   **Full Build:** Builds both the Rust components and the TypeScript packages.
    ```bash
    npm run build
    ```

*   **Rust Only:** Compiles the Rust `swc-napi-macros` crate into a Node-API native module.
    ```bash
    npm run build:rust
    ```

### Testing

*   **Run all tests:** Executes tests for all TypeScript workspaces and the playground examples.
    ```bash
    npm run test
    ```

## Development Conventions

*   **Code Formatting:** The project uses `biome` for code formatting and linting. Configuration can be found in `biome.json`.
*   **Monorepo Management:** `npm` workspaces are used to manage the TypeScript packages.
*   **Rust Crates:** The Rust part of the project is managed as a `cargo` workspace.
*   **Playground:** The `playground/` directory contains example projects for manually testing and experimenting with macros. To run the playground development server, use:
    ```bash
    npm run dev
    ```
