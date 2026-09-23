# NativePlugin

macroforge v0.3.1

A stateful expander for editor integrations: it caches each file's expansion by version and maps
positions and diagnostics back to the source.

## Constructor

TypeScript

```
const plugin = new NativePlugin();
```

## Methods

### processFile()

Process a file with version-based caching:

TypeScript

```
processFile(
  filepath: string,
  code: string,
  options?: ProcessFileOptions
): ExpandResult
```

TypeScript

```
interface ProcessFileOptions {
  // Cache key - if unchanged, returns cached result
  version?: string;

  // Keep @derive decorators in output (default: false)
  keepDecorators?: boolean;

  // External decorator module packages to load
  externalDecoratorModules?: Array<string>;

  // Path to a previously loaded config file
  configPath?: string;

  // JSON string of project-wide type registry
  typeRegistryJson?: string;

  // JSON string of the project-wide declarative macro registry
  declarativeRegistryJson?: string;

  // "dev" | "prod" (default: "prod")
  buildMode?: string;
}
```

### getMapper()

Get the position mapper for a previously processed file:

TypeScript

```
getMapper(filepath: string): PositionMapper | undefined
```

### mapDiagnostics()

Map diagnostics from expanded positions to original positions:

TypeScript

```
mapDiagnostics(
  filepath: string,
  diagnostics: JsDiagnostic[]
): JsDiagnostic[]
```

### log() / setLogFile()

Logging utilities for debugging:

TypeScript

```
log(message: string): void
setLogFile(path: string): void
```

## Caching Behavior

The plugin caches expansion results by file path and version:

TypeScript

```
const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });
```

## Example: Language Server Integration

TypeScript

```
import { NativePlugin } from "@macroforge/core";

class MacroforgeLanguageService {
  private plugin = new NativePlugin();

  processDocument(uri: string, content: string, version: number) {
    // Process with version-based caching
    const result = this.plugin.processFile(uri, content, {
      version: String(version)
    });

    // Get mapper for position translation
    const mapper = this.plugin.getMapper(uri);

    return { result, mapper };
  }

  getSemanticDiagnostics(uri: string, diagnostics: Diagnostic[]) {
    // Map positions from expanded to original
    return this.plugin.mapDiagnostics(uri, diagnostics);
  }
}
```

## Thread Safety

The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is
processed in an isolated thread with its own stack space.
