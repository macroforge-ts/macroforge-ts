# TypeScript Macros IntelliSense Extension Plan (UPDATED)

## Problem Statement

### The Core Issue
- **Classes decorated with `@Derive("Debug", "JSON")` don't get IntelliSense** for the methods that will be injected at runtime
- TypeScript cannot augment module-scoped classes from external `.d.ts` files
- TypeScript Language Service plugins are unreliable and don't work properly in modern IDEs
- Manual interface declarations in the same file work but defeat the purpose of automation

### Current Limitations
1. **Type Generation Approach**: Generates `.d.ts` files but can't augment module-scoped classes
2. **Language Service Plugin**: Doesn't work reliably in VS Code or Zed
3. **TypeScript Transformer**: Only works at build time, no IntelliSense
4. **Manual Augmentation**: Works but requires manual work in each file

## Solution: Hybrid Approach with Tree-sitter and LSP

### Updated Understanding (After Zed Docs Review)

Based on the Zed extension documentation, we need to revise our approach:

**What Zed Extensions CAN do:**
- Provide Language Servers via LSP
- Define Tree-sitter grammars for syntax highlighting
- Configure language metadata and file associations
- Provide completion items, diagnostics, and other LSP features
- **USE SYNTAX OVERRIDES to mark special scopes** ← KEY INSIGHT!

**What Zed Extensions CANNOT do (based on docs):**
- The docs don't explicitly state limitations
- However, they don't mention:
  - Direct IDE API access (no vscode-style APIs)
  - Virtual document providers
  - Direct TypeScript plugin integration

**What's UNCLEAR from the docs:**
- Can a Language Server provide virtual/modified document content?
- Can syntax overrides affect completion behavior?
- How much control does the LSP have over TypeScript's view of files?

### The Minimal Solution: Tree-sitter + TS Plugin

Instead of creating a whole new language server, we can:

1. **Use Tree-sitter queries** to detect `@Derive` decorated classes
2. **Configure TypeScript Language Server** to use our existing `ts-derive-plugin`
3. **Pass decorator information** to the TS server via configuration

## Phase 1: Minimal TypeScript Extension

### Approach A: TypeScript Plugin Configuration

The Zed extension can:
1. Detect `@Derive` decorators using Tree-sitter
2. Generate/update a `tsconfig.json` with our plugin
3. Let the TypeScript language server use the plugin

```rust
// In the Zed extension
fn language_server_command(&mut self, ...) -> Result<Command> {
    // Configure TS server to use our plugin
    Ok(Command {
        command: "typescript-language-server",
        args: vec![
            "--stdio",
            "--tsserver-path", "node_modules/typescript/lib",
            // Pass plugin configuration
            "--plugin", "@ts-macros/ts-derive-plugin"
        ],
        env: // Set up plugin path
    })
}
```

### Approach B: Tree-sitter Scope + Modified Completions

1. Mark decorated classes with Tree-sitter:
```scheme
; overrides.scm
(
  decorator
  (call_expression
    function: (identifier) @decorator.name
    (#eq? @decorator.name "Derive"))
  (class_declaration
    name: (identifier) @class.name)
) @macro.derive
```

2. Configure TypeScript to recognize these scopes:
```toml
# config.toml
[overrides.macro.derive]
# This scope has special completion rules
completion_query_characters = [".", "t", "j"]  # Trigger on . for methods
```

3. Minimal TS server wrapper that:
   - Reads Tree-sitter scope information
   - Adds completions for marked classes
   - No need for full proxy

### Approach C: Hybrid with Minimal Proxy

Create a **thin proxy** that only intercepts completion requests:

```typescript
// Minimal proxy - only touches completions
class MinimalTSProxy {
  constructor(private decoratedClasses: Map<string, string[]>) {}

  handleRequest(method: string, params: any) {
    if (method === 'textDocument/completion') {
      // Only modify completions for decorated classes
      const completions = this.tsServer.getCompletions(params);
      return this.augmentCompletions(completions, params);
    }
    // Pass everything else through unchanged
    return this.tsServer.handle(method, params);
  }
}
```

## The Focused Solution: Tree-sitter Detection + Targeted Override

Based on your suggestion, here's the minimal approach:

### Step 1: Tree-sitter Query for @Derive Detection

```scheme
; queries/highlights.scm or queries/injections.scm
(decorator
  (call_expression
    function: (identifier) @function.decorator
    (#eq? @function.decorator "Derive")
    arguments: (arguments (string) @macro.feature))
  (class_declaration
    name: (identifier) @class.decorated))
```

### Step 2: Minimal LSP Override

Instead of wrapping the entire TypeScript server, we create a **tiny completion provider** that:
1. Gets notified when Tree-sitter finds `@Derive`
2. Only provides the missing method completions
3. Lets TypeScript handle everything else

```typescript
// Ultra-minimal completion provider
export class DeriveCompletionProvider {
  // Tree-sitter tells us: "User class has @Derive('Debug', 'JSON')"
  private decoratedClasses = new Map<string, string[]>();

  async provideCompletions(doc: TextDocument, position: Position) {
    // Check if we're after a dot on a decorated class instance
    const line = doc.lineAt(position.line);
    const beforeDot = line.text.substring(0, position.character - 1);

    if (beforeDot.endsWith('.')) {
      const varName = extractVariable(beforeDot);
      const className = getClassOfVariable(varName);

      if (this.decoratedClasses.has(className)) {
        const features = this.decoratedClasses.get(className);
        const completions = [];

        if (features.includes('Debug')) {
          completions.push({
            label: 'toString',
            kind: CompletionItemKind.Method,
            detail: '(): string'
          });
        }

        if (features.includes('JSON')) {
          completions.push({
            label: 'toJSON',
            kind: CompletionItemKind.Method,
            detail: '(): object'
          });
        }

        return completions;
      }
    }

    return []; // Let TypeScript handle everything else
  }
}
```

### Step 3: Integration

The Zed extension would:
1. Use Tree-sitter to find decorated classes
2. Register a minimal completion provider for those specific cases
3. TypeScript server handles everything else normally

No full language server needed, just surgical additions where needed!

## Phase 1: Tree-sitter Detection

### Architecture Overview

```
┌─────────────────────────────────────────┐
│     Custom TS-Macros Language Server    │
├─────────────────────────────────────────┤
│  1. Proxies TypeScript Language Server  │
│  2. Parses @Derive decorators           │
│  3. Augments completion responses       │
│  4. Modifies diagnostic messages        │
│  5. Provides hover information          │
└─────────────────────────────────────────┘
                    ↕ LSP
┌─────────────────────────────────────────┐
│          Zed Extension                  │
├─────────────────────────────────────────┤
│  extension.toml:                        │
│    - Registers ts-macros-lsp            │
│    - Maps to .ts/.tsx files             │
│  src/lib.rs:                            │
│    - Downloads/provides LSP binary      │
│    - Returns LSP command                │
└─────────────────────────────────────────┘
```

### Language Server Implementation (TypeScript/Node.js)

```
ts-macros-lsp/
├── package.json              # Node.js dependencies
├── tsconfig.json            # TypeScript configuration
├── src/
│   ├── server.ts            # Main LSP server
│   ├── decorator-parser.ts  # Parse @Derive decorators
│   ├── type-augmenter.ts    # Augment TypeScript responses
│   └── proxy.ts             # Proxy to official TS server
└── dist/                    # Compiled JavaScript
```

### Zed Extension Structure

```
ts-macros-zed/
├── extension.toml           # Extension metadata
├── Cargo.toml              # Rust dependencies
├── src/
│   └── lib.rs              # Extension implementation
└── README.md
```

## Implementation Plan

### Phase 1 Deliverable (2024-12-16)

- ✅ Added `packages/ts-macros-lsp`: a minimal Node-based LSP that parses TypeScript/TSX files, detects `@Derive` decorators, tracks identifiers referencing those classes, and injects completions for `toString()`/`toJSON()` (kept for experimentation).
- ✅ Added `extensions/ts-macros-zed`: a Rust extension that launches `@vtsls/language-server` for TS/TSX and `svelte-language-server` for `.svelte` files (auto-installing both via Zed’s `npm:install` capability) while forcing `@ts-macros/ts-derive-plugin` to load in both environments.
- ✅ Registered a Zed extension that reuses the built-in TypeScript/TSX grammars so the server can attach without compiling extra grammars (Tree-sitter overrides deferred to phase 2).
- ⏭️ Next iteration: proxy completions/diagnostics from the official TypeScript server, add tests, and expand feature coverage (hover docs, error filtering, etc.).

### Step 1: Create the Language Server (Node.js/TypeScript)

#### 1.1 Basic LSP Setup
```typescript
// server.ts
import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  InitializeParams,
  InitializeResult,
  CompletionItem,
  TextDocumentPositionParams
} from 'vscode-languageserver/node';

const connection = createConnection(ProposedFeatures.all);
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

// Initialize TypeScript Language Service
const tsService = new TypeScriptService();

connection.onInitialize((params: InitializeParams): InitializeResult => {
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: ['.']
      }
    }
  };
});
```

#### 1.2 Decorator Detection
```typescript
// decorator-parser.ts
export function findDeriveDecorators(sourceFile: ts.SourceFile): Map<string, string[]> {
  const decoratedClasses = new Map<string, string[]>();

  ts.forEachChild(sourceFile, function visit(node) {
    if (ts.isClassDeclaration(node) && node.decorators) {
      const deriveDecorator = findDerive(node.decorators);
      if (deriveDecorator) {
        const features = extractFeatures(deriveDecorator);
        decoratedClasses.set(node.name.text, features);
      }
    }
  });

  return decoratedClasses;
}
```

#### 1.3 Augment Completions
```typescript
// type-augmenter.ts
connection.onCompletion((params: TextDocumentPositionParams): CompletionItem[] => {
  // Get completions from TypeScript
  const tsCompletions = tsService.getCompletions(params);

  // Check if we're completing on a decorated class
  const classInfo = getClassAtPosition(params);
  if (classInfo && decoratedClasses.has(classInfo.name)) {
    const features = decoratedClasses.get(classInfo.name);

    // Add @Derive methods
    if (features.includes('Debug')) {
      tsCompletions.push({
        label: 'toString',
        kind: CompletionItemKind.Method,
        detail: '(method) toString(): string',
        documentation: 'Generated by @Derive("Debug")'
      });
    }

    if (features.includes('JSON')) {
      tsCompletions.push({
        label: 'toJSON',
        kind: CompletionItemKind.Method,
        detail: '(method) toJSON(): object',
        documentation: 'Generated by @Derive("JSON")'
      });
    }
  }

  return tsCompletions;
});
```

### Step 2: Create the Zed Extension

#### 2.1 Extension Configuration
```toml
# extension.toml
id = "ts-macros"
name = "TypeScript Macros"
version = "0.1.0"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "IntelliSense support for @Derive TypeScript macros"
repository = "https://github.com/your-name/ts-macros-zed"

[language_servers.ts-macros-lsp]
name = "TS-Macros Language Server"
languages = ["TypeScript", "TSX", "JavaScript", "JSX"]
```

#### 2.2 Extension Implementation
```rust
// src/lib.rs
use zed_extension_api as zed;
use std::fs;

struct TsMacrosExtension {
    cached_binary_path: Option<String>
}

impl zed::Extension for TsMacrosExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Download or use local LSP binary
        let binary_path = self.get_or_download_lsp()?;

        Ok(zed::Command {
            command: binary_path,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn get_or_download_lsp(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            return Ok(path.clone());
        }

        // Download LSP from npm or GitHub releases
        let version = "0.1.0";
        let binary_path = download_lsp_binary(version)?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(TsMacrosExtension);
```

### Step 3: Distribution Strategy

#### Option A: NPM Package
1. Publish the Language Server to npm as `ts-macros-lsp`
2. Zed extension downloads it during installation
3. Updates handled through npm versioning

#### Option B: GitHub Releases
1. Build Language Server binaries for each platform
2. Upload to GitHub Releases
3. Zed extension downloads appropriate binary

#### Option C: Bundle with Extension
1. Include compiled Language Server in extension
2. Larger extension size but no download needed
3. Self-contained solution

## Technical Challenges & Solutions

### Challenge 1: TypeScript Service Integration
**Problem**: Need to proxy all TypeScript Language Server features while adding our own
**Solution**:
- Use `typescript/lib/tsserverlibrary` to create a proper TS service host
- Forward all standard requests to TS server
- Intercept and augment only completion/diagnostic responses

### Challenge 2: Performance
**Problem**: Proxying adds latency to every LSP request
**Solution**:
- Cache decorator information per file
- Only re-parse on file changes
- Use incremental parsing where possible
- Consider worker threads for parsing

### Challenge 3: Type Accuracy
**Problem**: Need to match exact TypeScript types and behavior
**Solution**:
- Study TypeScript's completion item format
- Ensure our injected methods match expected signatures
- Test with various TypeScript configurations

## Testing Strategy

### Unit Tests (Language Server)
- Test decorator parsing logic
- Test completion augmentation
- Test diagnostic filtering
- Test hover information

### Integration Tests
- Test with sample TypeScript project
- Verify LSP communication
- Test with Zed editor
- Test with VS Code (LSP is universal)

### End-to-End Tests
- Use with `playground/vanilla` project
- Verify IntelliSense for decorated classes
- Test file watching and updates
- Performance benchmarks

## Success Criteria

✅ **Functional Requirements**
- [ ] IntelliSense shows `toString()` and `toJSON()` for decorated classes
- [ ] No false positive TypeScript errors
- [ ] Works with both module and global scopes
- [ ] Handles multiple files and projects

✅ **Performance Requirements**
- [ ] < 100ms added latency for completions
- [ ] < 50MB memory overhead
- [ ] Incremental updates on file changes

✅ **User Experience**
- [ ] Zero configuration after extension install
- [ ] Seamless integration with existing TypeScript tooling
- [ ] Clear attribution of generated methods

## Alternative Approaches (If LSP Wrapper Fails)

### Plan B: Code Generation with File Watching
- Watch for files with `@Derive`
- Generate companion `.d.ts` files with augmentations
- Use module augmentation where possible
- Accept limitations of module scope

### Plan C: Different Syntax
- Use function calls instead of decorators: `Derive(class User {}, "Debug", "JSON")`
- TypeScript can infer return types from functions
- Less elegant but more type-safe

### Plan D: Build-Time Only
- Accept that IntelliSense won't work
- Provide clear documentation
- Focus on runtime functionality
- Use type assertions as needed

## Implementation Timeline

### Week 1: Language Server Core
- Set up TypeScript Language Server proxy
- Implement basic LSP protocol
- Test with simple completions

### Week 2: Decorator Parsing & Augmentation
- Implement `@Derive` detection
- Add completion augmentation
- Handle diagnostics

### Week 3: Zed Extension
- Create Zed extension wrapper
- Test in Zed editor
- Handle binary distribution

### Week 4: Polish & Testing
- Performance optimization
- Comprehensive testing
- Documentation
- Consider VS Code extension

## Resources & References

### Documentation
- [Language Server Protocol Specification](https://microsoft.github.io/language-server-protocol/)
- [TypeScript Language Service API](https://github.com/microsoft/TypeScript/wiki/Using-the-Language-Service-API)
- [Zed Extension API](https://zed.dev/docs/extensions/developing-extensions)
- [VS Code Language Server Extension Guide](https://code.visualstudio.com/api/language-extensions/language-server-extension-guide)

### Example Implementations
- [Volar (Vue Language Server)](https://github.com/vuejs/language-tools)
- [Svelte Language Server](https://github.com/sveltejs/language-tools)
- [Astro Language Server](https://github.com/withastro/language-tools)
- [Angular Language Server](https://github.com/angular/vscode-ng-language-service)

### Tools
- [LSP Inspector](https://microsoft.github.io/language-server-protocol/inspector/)
- [TypeScript AST Viewer](https://ts-ast-viewer.com/)

## Next Steps

1. **Immediate**: Build proof-of-concept Language Server ✅ (`packages/ts-macros-lsp`)
2. **Short term**: Test augmented completions work ✅ (baseline completions wired into the custom LSP)
3. **Medium term**: Create Zed extension wrapper ✅ (`extensions/ts-macros-zed`)
4. **Long term**: Support VS Code and other LSP-compatible editors ⏳

## Key Insights

After reviewing the Zed documentation, the approach must be:
1. **Language Server Protocol is the only way** - Zed extensions work through LSP, not IDE APIs
2. **We need a proxy server** - Must wrap TypeScript's Language Server (Phase 2: replace standalone analyzer with TS proxy)
3. **Universal solution** - LSP works in any compatible editor (Zed, VS Code, Neovim, etc.)
4. **Distribution is key** - Need reliable way to distribute the Language Server binary

---

**Status**: Phase 1 scaffolding landed (LSP + Zed extension). Continue hardening proxy + tests.
**Priority**: High
**Estimated Effort**: 3-4 weeks
**Risk Level**: Medium-High (Complex but proven approach)
