# TypeScript Macro Framework - TODO

> Building "proc-macro crates for TS" on top of SWC with Rust implementations

## 🚀 Project Status

### Phase 1: Core Proc-Macro Host - **75% Complete**
- ✅ Core ABI and macro model defined
- ✅ `ts_macro_host` crate with registry and dispatch
- ✅ Three derive macros implemented (@Derive(Debug/Clone/Eq))
- ✅ Basic SWC integration working
- ✅ All crates compile without errors or warnings
- 🔄 Patch application engine needed
- 🔄 CLI tool needed
- 🔄 Host integration with transformer needed

### Recent Progress (Nov 24, 2024)
- Created `ts_macro_host` crate with complete macro registry and dispatch system
- Implemented `MacroContextIR` and enhanced `MacroResult` with separate runtime/type patches
- Built three working derive macros: Debug, Clone, and Eq
- Fixed all SWC version conflicts and compilation issues across workspace
- Established stable ABI with version checking
- Created configuration structure for macro packages

## Project Overview

This project aims to create a Rust-like procedural macro system for TypeScript, using SWC for transformations and Rust for macro implementations. The goal is to achieve Rust-level DX with `@Derive(Debug)`-style syntax and reusable macro crates.

---

## Core Architecture Decisions

### [x] 1. Define Macro Model & ABI ✅ COMPLETED

Design the three kinds of macros (mirroring Rust's macro types):

- [x] **Derive macros**: `@Derive(Debug, Clone, …)` on classes ~~interfaces/enums~~
- [ ] **Attribute macros**: `@log`, `@sqlTable`, etc.
- [ ] **Call macros** (later): `macro.sql` / tagged templates / `Foo!()` equivalents

#### Derive Dispatch Model (lock early)

- [x] `@Derive(...)` is a **built-in dispatcher** (Rust `#[derive]` analog).

  - Example:

    ```ts
    import { Derive, Debug, Clone } from "@macro/derive";

    @Derive(Debug, Clone)
    class User { ... }
    ```

- [x] Each name inside `@Derive(...)` resolves to a registered **derive macro** by `(module, derive_name)`.

#### Stable ABI Between SWC and Macro Crates

- [x] Implement input format scaffold in `crates/`

- [x] **Canonical MacroContextIR** (what macros receive):

  - [x] Define `MacroContextIR` in ABI crate:

    ```rust
    pub struct MacroContextIR {
      pub abi_version: u32,
      pub macro_kind: MacroKind,     // Derive | Attr | Call
      pub macro_name: String,        // e.g. "Debug"
      pub module_path: String,       // e.g. "@macro/derive"
      pub decorator_span: SpanIR,    // span of @Derive(Debug)
      pub target_span: SpanIR,       // span of class/enum/etc
      pub file_name: String,
      pub target: TargetIR,          // Class(ClassIR) | Enum(EnumIR) | ...
    }
    ```

  - [x] `TargetIR` starts with `ClassIR` in v0; add others without breaking ABI.

- [x] Define output format (**patches + diagnostics only**):

  - Patch ABI is stable and cross-runtime friendly

    - native macros can internally mutate SWC AST and emit patches
    - WASM macros only need spans + generated code.

  - Add a _type-surface slot_ now to avoid ABI breaks later:

    ```rust
    pub struct MacroResult {
      pub runtime_patches: Vec<Patch>,  // emitted into JS/TS output
      pub type_patches: Vec<Patch>,     // emitted into .d.ts (may be empty in v0)
      pub diagnostics: Vec<Diagnostic>,
      pub debug: Option<String>,
    }

    pub enum Patch {
      Insert { at: Span, code: String },
      Replace { span: Span, code: String },
      Delete { span: Span },
    }
    ```

  - Error/diagnostic information with spans

- [ ] Document the ABI contract for 3rd-party macro authors

- [x] ABI version handshake between host and macro crates

### [ ] Security Considerations

- [ ] Macro sandboxing (WASM by default)
- [ ] Resource limits (Phase 1 host enforces):

  - [ ] Max execution time
  - [ ] Max memory usage (WASM runtime)
  - [ ] Max output size
  - [ ] Max per-macro time
  - [ ] Max diagnostics count

- [ ] Code injection prevention:

  - [ ] Validate macro outputs
  - [ ] Sanitize generated identifiers
  - [ ] Audit macro packages

- [ ] Native macro execution is **opt-in + gated** (see loading strategy).

---

### [x] 0.1 ABI + Support Crates ✅ IMPLEMENTED

**Purpose:** Provide Rust proc-macro–like ergonomics:

- **`ts_macro_abi`** = stable IR + spans + patches (public ABI) ✅
- **`ts_syn`** = parse/lower SWC → IR (your "syn") ✅
- **`ts_quote`** = codegen helpers + quasiquote → patches/strings (your "quote") 🔄 (skeleton only)
- **`ts_macro_host`** = macro registry, dispatch, and execution ✅
- **`swc-napi-macros`** = N-API bindings for Node.js integration ✅

---

### [x] 2. Discovery & Registration System ✅ CORE IMPLEMENTED

How the system knows which macro to call:

- [x] Implement module-based discovery:

  - Use import source as key: `import { Derive } from "@my-org/macros"`
  - Key format: `(module: string, name: string)`

- [x] Create config file structure:

  ```jsonc
  // ts-macro.config.json
  {
    "macroPackages": ["@my-org/ts-macros-derive", "@my-org/ts-macros-serde"],
    "allowNativeMacros": false,
    "macroRuntimeOverrides": {
      "@bar/big-schema-macro": "native",
    },
  }
  ```

- [ ] Implement package allowlist/validation

#### Macro Package Manifest (ecosystem safety)

- [x] Each macro package ships a manifest, e.g. `macro.manifest.json`:

  ```jsonc
  {
    "abiVersion": 1,
    "macros": [
      { "kind": "derive", "name": "Debug" },
      { "kind": "derive", "name": "Clone" },
      { "kind": "attr", "name": "serde.Json" },
    ],
    "runtime": ["wasm", "native"],
  }
  ```

- [x] Host registers from manifest before running code
- [ ] Clear error if manifest and exports disagree

---

### [ ] 3. Macro Loading Strategy

Implement macro module loading:

- [ ] **WASM-based modules (default)**

  - Like Rust proc macro modules, each macro is its own project
  - Each macro package ships WASM file
  - Define macro ABI trait:

    ```rust
    pub trait TsMacro {
        fn name(&self) -> &str;
        fn run(&self, ctx: MacroContextIR) -> MacroResult;
    }
    ```

  - Pros: Cross-platform, sandboxed, flexible ecosystem

- [ ] **Native (N-API) modules (opt-in)**

  - Same ABI as WASM (identical semantics)
  - Loaded only when explicitly allowed / present
  - Intended for heavy compute / native deps
  - Gated by `allowNativeMacros` and/or per-package overrides

---

## Phase 1: Core Proc-Macro Host

### [x] Build the Macro Host Crate ✅ COMPLETED

#### Rust Macro Host (`ts_macro_host`)

- [x] Create core crate structure

- [x] Implement SWC integration:

  - [x] Parse TypeScript/JavaScript with SWC
  - [x] AST traversal for decorator discovery
  - [ ] AST transformation after macro expansion
  - [x] Code generation/printing

- [x] Implement macro dispatch:

  - [x] Registry for macro modules from manifests
  - [x] Lookup by `(module, name)` key
  - [x] Context building for macro calls (`MacroContextIR`)
  - [ ] Patch application
  - [x] Result handling and error propagation

- [ ] **Precise insertion spans**

  - [ ] During lowering, compute:

    - [ ] `body_open_span`
    - [ ] `body_close_span`

  - [ ] Store on `ClassIR` so helpers don't guess `end - 1`.

#### N-API Bindings (using `napi-rs`)

- [x] Create N-API wrapper crate (`swc-napi-macros`)

- [ ] Expose essential functions:

  ```rust
  #[napi]
  pub fn init_macros(config: MacroConfig) -> Result<()>

  #[napi]
  pub fn expand_file(source: String, file_name: String) -> Result<String>

  #[napi]
  pub fn run_macro(call: MacroCall) -> Result<MacroResult>

  #[napi]
  pub fn analyze_macros(source: String, file_name: String) -> Result<Vec<Diagnostic>>
  ```

- [ ] Implement caching layer:

  - [ ] Cache keyed by `(file_name, version)` or content hash
  - [ ] Invalidation strategy
  - [ ] Memory management

### [x] Implement Core Derives ✅ COMPLETED

#### `@Derive(Debug)` ✅

- [x] Design output format:

  ```ts
  class User {
    // ... existing fields ...

    static DEBUG_SYM = Symbol.for("ts-macro.debug");

    [User.DEBUG_SYM](): string {
      return `User { id: ${this.id}, age: ${this.age} }`;
    }

    toString(): string {
      return this[User.DEBUG_SYM]();
    }
  }
  ```

- [x] Decide naming conventions:

  - [x] Internal helpers: `Symbol.for("ts-macro.debug")`
  - [ ] Document and stabilize

- [x] Define compatibility rules:

  - [x] Valid targets: classes ~~, enums, interfaces?~~
  - [x] Supported members: all field types
  - [x] Error cases

- [x] Implement the macro:

  - [x] Parse class ~~interface/enum~~
  - [x] Generate debug string method
  - [ ] Handle edge cases (circular refs, complex types)

#### Additional Basic Derives

- [x] `@Derive(Clone)` ✅
- [x] `@Derive(Eq)` ✅
- [ ] Design multi-derive support:

  ```ts
  @Derive(Debug, Eq, Clone)
  class User { ... }
  ```

  - [ ] Define deterministic order
  - [x] Dispatch strategy via built-in `Derive` dispatcher
  - [ ] Field-level flags/attributes routed to active derive

### [x] Error Handling & Diagnostics ✅ CORE COMPLETED

- [x] Span tracking:

  - [x] Capture decorator span (`@Derive(Debug)`)
  - [x] Capture annotated node span (`class User`)
  - [ ] Preserve source locations through transformations

- [ ] Error message format (Rust-like):

  ```
  error: `@Derive(Debug)` on `User` is invalid
    --> src/user.ts:12:1
     |
  12 | @Derive(Debug)
     | ^^^^^^^^^^^^^^ computed properties are not supported yet
  13 | class User {
     | ---------- on this class
     |
     = help: remove the `[key: string]` index signature or derive manually
  ```

  Note: Effect TS has a rich error printing ecosystem that we can use.

- [x] Error categories:

  - [x] Syntax errors (invalid decorator usage)
  - [x] Compatibility errors (unsupported patterns)
  - [x] Generation errors (macro implementation failures)

### [ ] CLI Tool (`ts-derive` / `ts-macro`)

- [ ] Create CLI crate

- [ ] Implement commands:

  - [ ] `expand <file>` - Show transformed code

    - [ ] `--macro Derive` - Filter by macro type
    - [ ] `--symbol User` - Filter by symbol

  - [ ] `check <file>` - Validate macro usage without emitting
  - [ ] `init` - Create config file

- [ ] Dev mode features:

  - [ ] Wrap generated code in comments:

    ```ts
    // @macro-generated begin Derive(Debug) for User
    ...
    // @macro-generated end
    ```

  - [ ] Make conditional on dev flag

---

## Phase 2: DX Polish

### [ ] Field-Level Attributes

Implement per-field attribute support:

```ts
@Derive(Debug)
class User {
  @Debug(skip)
  password: string;

  @Debug({ rename: "user_id" })
  id: string;
}
```

- [ ] Define attribute syntax parser
- [ ] Implement attribute handlers:

  - [ ] `skip` - exclude from derive
  - [ ] `rename` - custom field name in output
  - [ ] Custom per-derive attributes

- [ ] Update derive macros to respect attributes

### [ ] Hygiene & Naming Rules

- [ ] Define internal naming convention:

  ```ts
  const __derive_Debug_User_fmt = ...
  ```

- [ ] Implement collision detection:

  - [ ] Error on user-defined names that conflict
  - [ ] Provide clear error messages

- [ ] Define visibility rules:

  - [ ] Public API vs internal implementation
  - [ ] Document what users can rely on

### [ ] Testing Infrastructure

- [ ] Golden tests for expansions:

  - [ ] Input: source files with macros
  - [ ] Expected: expanded output
  - [ ] Diff on changes

- [ ] Macro unit tests:

  - [ ] Valid usage patterns
  - [ ] Error cases
  - [ ] Edge cases

- [ ] Integration tests:

  - [ ] Full build pipeline
  - [ ] Multiple macro interactions

### [ ] Documentation

- [ ] Macro API documentation:

  - [ ] How to use each derive
  - [ ] Expansion examples for each macro
  - [ ] Field attributes reference

- [ ] Framework documentation:

  - [ ] Getting started guide
  - [ ] Configuration reference
  - [ ] Writing custom macros guide

- [ ] Examples repository:

  - [ ] Basic derive usage
  - [ ] Custom macro implementation
  - [ ] Complex real-world patterns

---

## Phase 3: Type/IDE Integration

### [ ] TypeScript Type System Strategy

Choose and implement approach:

#### TSServer Plugin & Generate `.d.ts`

- [ ] Create tsserver plugin that calls macro host

- [ ] Present macro-expanded view to language service

- [ ] Emit `.d.ts` using **type_patches** from macro results
      (host may also synthesize from IR in v0)

- [ ] Wire into TS compiler:

  - [ ] Include generated `.d.ts` in compilation
  - [ ] Update tsconfig paths

- [ ] Update on file changes

### [ ] TSServer Plugin Implementation

#### Plugin Structure

- [ ] Create plugin package structure:

  ```
  ts-macro-tsserver-plugin/
  ├── src/
  │   ├── index.ts          # Plugin entry point
  │   ├── macro-resolver.ts # Import resolution
  │   └── expansion-cache.ts
  ├── native/               # N-API bindings
  └── package.json
  ```

#### Core Functionality

- [ ] Plugin initialization:

  - [ ] Load config
  - [ ] Initialize N-API host
  - [ ] Register macro packages

- [ ] Decorator resolution:

  - [ ] Hook into TS language service
  - [ ] Resolve decorator imports
  - [ ] Extract `(module, name)` keys
  - [ ] Filter to registered macro packages only

- [ ] Macro expansion integration:

  - [ ] Call N-API host when needed
  - [ ] Cache expansions per file version
  - [ ] Invalidate on file changes
  - [ ] Performance monitoring

#### Language Service Hooks

- [ ] `getSemanticDiagnostics`:

  - [ ] Add macro validation errors
  - [ ] Include expansion errors

- [ ] `getQuickInfoAtPosition`:

  - [ ] Show generated member info
  - [ ] Display expansion details

- [ ] `getCompletionsAtPosition`:

  - [ ] Suggest generated methods
  - [ ] Autocomplete macro names

- [ ] Virtual file support (optional):

  - [ ] Override `getScriptSnapshot`
  - [ ] Serve expanded versions
  - [ ] Maintain file mappings

#### Commands

- [ ] `showMacroExpansion`:

  - [ ] Expand current file
  - [ ] Display in virtual document
  - [ ] Syntax highlighting

- [ ] `analyzeMacros`:

  - [ ] Show all macros in file
  - [ ] List generated members
  - [ ] Show diagnostics

### [ ] Cross-Platform Support

- [ ] N-API binary distribution:

  - [ ] Prebuild for platforms:

    - [ ] Linux x64
    - [ ] Linux ARM64
    - [ ] macOS x64 (Intel)
    - [ ] macOS ARM64 (Apple Silicon)
    - [ ] Windows x64

  - [ ] Use `napi-rs` tooling for builds
  - [ ] Test on each platform

- [ ] Fallback strategy:

  - [ ] Pure-JS degraded mode
  - [ ] Basic diagnostics without expansion
  - [ ] Clear error messages

### [ ] Performance Optimization

- [ ] Caching strategy:

  - [ ] File-level caching keyed by hash
  - [ ] AST caching for unchanged nodes
  - [ ] Result caching with TTL

- [ ] Incremental processing:

  - [ ] Only re-expand changed files
  - [ ] Skip validation on unchanged decorators
  - [ ] Batch processing where possible

- [ ] Monitoring:

  - [ ] Track expansion times
  - [ ] Log slow operations
  - [ ] Configurable timeouts

---

## Phase 4: Ecosystem

### [ ] Stabilize Macro Crate ABI

- [ ] Version the ABI:

  ```rust
  pub const MACRO_ABI_VERSION: &str = "0.1.0";
  ```

- [ ] Compatibility checks between host and macros
- [ ] Deprecation strategy for ABI changes
- [ ] Migration guides for version bumps

### [ ] Official Macro Packages

#### `@macro/derive`

- [x] Core derives (initial implementation):

  - [x] Debug ✅
  - [x] Clone ✅
  - [x] Eq ✅ / [ ] PartialEq
  - [ ] Hash
  - [ ] Default
  - [ ] Copy (shallow copy semantics)

- [ ] Documentation and examples

- [ ] Comprehensive test suite

- [ ] Publish to npm

#### `@macro/serde`

- [ ] Serialization derives:

  - [ ] `@serde.Json()` - JSON serialization
  - [ ] `@serde.Yaml()` - YAML serialization
  - [ ] Custom serializers

- [ ] Field attributes:

  - [ ] `@serde(rename = "...")`
  - [ ] `@serde(skip)`
  - [ ] `@serde(default)`

- [ ] Documentation and examples

- [ ] Test suite

- [ ] Publish to npm

### [ ] Third-Party Macro Support

- [ ] Create macro template repository:

  - [ ] Rust macro crate structure
  - [ ] Build configuration
  - [ ] Testing setup
  - [ ] Publishing guide

- [ ] Documentation for macro authors:

  - [ ] ABI reference
  - [ ] Best practices
  - [ ] Testing strategies
  - [ ] Publishing checklist

- [ ] Macro registry (optional):

  - [ ] Searchable catalog
  - [ ] Quality/security badges
  - [ ] Usage examples

### [ ] Build Tool Integration

#### Vite Plugin

- [ ] Create `vite-plugin-ts-macros`
- [ ] Hook into SWC transformation
- [ ] Load macro config
- [ ] Handle HMR for macro changes

---

## Technical Considerations

### [ ] Macro Execution Model

#### Single Host vs Per-Package

**Decision: Single host + plugin-style macro crates (WASM or SWC plugin ABI)**

Rationale:

- Cleaner architecture
- Shared SWC/AST handling
- Consistent behavior across build and IDE
- Easier to maintain ABI

Implementation:

- [ ] Central macro host in Rust
- [ ] Macro packages export ABI-compatible functions
- [ ] Host handles loading and dispatch

#### Same Macros Everywhere

Ensure macro modules work in:

- [ ] Build (Vite + SWC)
- [ ] TSServer plugin
- [ ] CLI tools
- [ ] Testing infrastructure

Strategy:

- [ ] Agnostic macro implementations
- [ ] Unified `MacroContextIR` interface
- [ ] Same ABI for all entry points

### [ ] Type Awareness Limitations

**Key constraint**: SWC-based macros are mostly syntactic, not fully type-aware.

Implications:

- [ ] Macros operate on AST, not type information
- [ ] Cannot resolve complex type relationships
- [ ] No type inference from context

Workarounds:

- [ ] Explicit annotations where needed
- [ ] Runtime validation for complex cases
- [ ] Clear documentation of limitations
- [ ] Consider optional type-aware mode (future host after TS typecheck)

### [ ] Error Recovery & Partial Expansion

- [ ] Graceful degradation:

  - [ ] If one macro fails, continue with others
  - [ ] Partial output vs complete failure

- [ ] Error boundaries:

  - [ ] Isolate macro failures
  - [ ] Preserve valid code
  - [ ] Clear error attribution

---

## Future Enhancements

### [ ] Advanced Features

- [ ] Procedural attributes on:

  - [ ] Individual methods
  - [ ] Properties
  - [ ] Function parameters

### [ ] Type System Integration (Advanced)

- [ ] Full type-aware macros:

  - [ ] Integrate with TS compiler API
  - [ ] Type-driven code generation
  - [ ] Type-level computations

- [ ] Conditional compilation based on types:

  - [ ] Different code for different type parameters
  - [ ] Type-specialized implementations

---

## Resources & References

### External Links

- Rust proc-macro reference: [https://doc.rust-lang.org/reference/procedural-macros.html](https://doc.rust-lang.org/reference/procedural-macros.html)
- SWC documentation: [https://swc.rs/](https://swc.rs/)
- napi-rs: [https://napi.rs/](https://napi.rs/)
- TypeScript Compiler API: [https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API](https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API)

### Inspiration & Prior Art

- Rust: `derive(Debug)`, `serde`, etc.
- Babel macros: `babel-plugin-macros`
- Sweet.js: Hygienic macros for JavaScript
- TypeScript transformers: Custom transformers API

---

## Notes

- **Philosophy**: Make it feel like Rust proc-macros, not a special build step
- **Priority**: DX over absolute performance (but keep it fast enough)
- **Approach**: Start simple, iterate based on real usage
- **Community**: Enable third-party macros from day one
- **Documentation**: Over-document rather than under-document
- **Testing**: If it's not tested, it's broken

---

## 🎯 Next Steps

### Immediate Priorities
1. **Implement Patch Application Engine** - Apply generated patches to source code
2. **Refactor MacroTransformer** - Use new `ts_macro_host` instead of hardcoded macros
3. **Create CLI Tool** - For testing macro expansion and debugging
4. **Complete ts_quote** - Implement actual quote! functionality for code generation

### To Make It Usable
1. **Integration** - Connect `ts_macro_host` with `swc-napi-macros`
2. **Configuration Loading** - Actually load and use `ts-macro.config.json`
3. **Playground Testing** - Verify the new system works with playground apps
4. **Documentation** - Write getting started guide for macro authors

### Current Blockers
- Patch application not implemented (patches are generated but not applied)
- MacroTransformer still uses hardcoded macros instead of registry
- No way to test macro expansion without full build pipeline

---
