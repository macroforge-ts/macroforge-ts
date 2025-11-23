# GEMINI.md

This file provides a comprehensive overview of the `ts-macros` project, its structure, and how to build, run, and contribute to it.

## Project Overview

`ts-macros` is a high-performance, Rust-powered macro system for TypeScript. It allows developers to use macros, similar to those in Rust, to perform compile-time code generation and other transformations.

The project is a monorepo with the following components:

-   **`crates/swc-napi-macros`**: The core of the project, this is a Rust crate that implements an SWC (Speedy Web Compiler) plugin. It uses N-API to expose a `transform_sync` function to Node.js, which can be used to apply macro transformations to TypeScript code.
-   **`packages/vite-plugin`**: A Vite plugin that integrates the SWC plugin into the Vite build process. It automatically transforms TypeScript files using the `transform_sync` function from the Rust crate.
-   **`playground`**: A sample Vite project that demonstrates how to use the macros. It includes examples of the available macros and how to configure the Vite plugin.

The main technologies used in this project are:

-   **Rust**: For the core macro transformation logic, providing high performance and safety.
-   **SWC**: As the TypeScript compiler and for AST manipulation.
-   **N-API**: To create a bridge between the Rust code and the Node.js environment.
-   **TypeScript**: For the Vite plugin and the playground.
-   **Vite**: As the build tool and development server.

## Building and Running

The project uses `npm` workspaces to manage the different packages. The following commands are available in the root `package.json`:

-   **`npm run dev`**: Starts the development server for the `playground` project. This is the best way to see the macros in action.
-   **`npm run build`**: Builds both the Rust SWC plugin and the Vite plugin.
-   **`npm run build:rust`**: Compiles the Rust crate and creates the `.node` binary.
-   **`npm run build:plugin`**: Compiles the TypeScript code for the Vite plugin.
-   **`npm test`**: Runs the tests for all the packages.

### Development Workflow

1.  Run `npm install` to install all the dependencies.
2.  Run `npm run build` to build the project for the first time.
3.  Run `npm run dev` to start the playground and experiment with the macros.

When you make changes to the Rust code, you will need to run `npm run build:rust` to recompile it. When you make changes to the Vite plugin, you will need to run `npm run build:plugin`.

## Development Conventions

The project follows the standard conventions for Rust and TypeScript projects.

-   **Rust**: The code is formatted with `cargo fmt` and linted with `cargo clippy`.
-   **TypeScript**: The code is formatted with Prettier (not configured yet, but it's a good practice) and linted with ESLint (also not configured yet).

The project uses a `playground` for manual testing and demonstration. For automated testing, the `npm test` command is available, but no tests have been written yet.
