# Roadmap

*Planned features and improvements for Macroforge. This roadmap reflects our current priorities but may change based on community feedback.*

## IDE Extensions

Bring Macroforge support directly into your favorite editors with native extensions.

| VS Code / Cursor 
| Planned 
| Native extension with macro expansion preview, error highlighting, and completions 

| Zed 
| Available 
| Full support via vtsls-macroforge and svelte-macroforge extensions 

| Neovim 
| Considering 
| LSP integration for Neovim users 

| JetBrains (WebStorm) 
| Considering 
| Plugin for WebStorm and other JetBrains IDEs

## Framework Support

Expanding Macroforge to work seamlessly with popular frontend frameworks.

| Svelte / SvelteKit 
| Available 
| Full support via svelte-language-server and Vite plugin 

| React 
| Planned 
| React-specific macros and integration with React tooling 

| Vue 
| Planned 
| Vue SFC support and Vue-specific derive macros 

| Angular 
| Planned 
| Angular decorator integration and CLI support 

| Solid 
| Planned 
| SolidJS integration

## Pure TypeScript Macro Creation

While Rust provides the best performance and type safety, we recognize that not everyone wants to write Rust. We're exploring options for writing macros in pure TypeScript.

| TypeScript Macro API 
| Planned 
| Define macros using TypeScript with a simple API 

| Template Strings 
| Planned 
| Generate code using tagged template literals 

| AST Helpers 
| Planned 
| TypeScript utilities for working with the AST

## Built-in Macros

Expanding the library of built-in derive macros.

| Debug, Clone, PartialEq, Ord, Hash 
| Available 
| Core derive macros 

| Serialize, Deserialize 
| Available 
| JSON serialization with validation 

| Builder 
| Planned 
| Generate builder pattern for classes 

| Immutable 
| Considering 
| Generate immutable update methods (with, set)

## Distribution & Packaging

Making it easier to publish and share custom macros.

| Native Node Binaries 
| Available 
| Platform-specific binaries for maximum performance 

| WASM Binary Generation 
| Planned 
| Cross-platform WebAssembly binaries for easier macro distribution 

| Macro Registry 
| Considering 
| Central registry for discovering and sharing community macros

## Tooling & DX

Improvements to the developer experience.

| CLI Expansion 
| Available 
| Expand macros from the command line 

| Macro Playground 
| Planned 
| Web-based playground to test macros 

| create-macroforge 
| Planned 
| Scaffolding tool for new macro projects 

| Macro Debugging 
| Considering 
| Step-through debugging for macro expansion

## Contributing

Interested in helping? We welcome contributions of all kinds:

- Feature requests and feedback on [GitHub Issues](https://github.com/rymskip/macroforge-ts/issues)

- Pull requests for new macros or improvements

- Documentation improvements

- Framework integrations