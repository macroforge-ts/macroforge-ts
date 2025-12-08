# NativePlugin

*A stateful plugin class with version-based caching, designed for integration with language servers and IDEs.*

## Constructor

```typescript
const plugin = new NativePlugin();
```

## Methods

### processFile()

Process a file with version-based caching:

```typescript
processFile(
  filepath: string,
  code: string,
  options?: ProcessFileOptions
): ExpandResult
```

```typescript
interface ProcessFileOptions {
  // Cache key - if unchanged, returns cached result
  version?: string;
}
```

### getMapper()

Get the position mapper for a previously processed file:

```typescript
getMapper(filepath: string): NativeMapper | null
```

### mapDiagnostics()

Map diagnostics from expanded positions to original positions:

```typescript
mapDiagnostics(
  filepath: string,
  diagnostics: JsDiagnostic[]
): JsDiagnostic[]
```

### log() / setLogFile()

Logging utilities for debugging:

```typescript
log(message: string): void
setLogFile(path: string): void
```

## Caching Behavior

The plugin caches expansion results by file path and version:

```typescript
const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });
```

## Example: Language Server Integration

```typescript
import { NativePlugin } from "macroforge";

class MacroforgeLanguageService {
  private plugin = new NativePlugin();

  processDocument(uri: string, content: string, version: number) {
    // Process with version-based caching
    const result = this.plugin.processFile(uri, content, {
      version: String(version)
    });

    // Get mapper for position translation
    const mapper = this.plugin.getMapper(uri);

    return { result, mapper };
  }

  getSemanticDiagnostics(uri: string, diagnostics: Diagnostic[]) {
    // Map positions from expanded to original
    return this.plugin.mapDiagnostics(uri, diagnostics);
  }
}
```

## Thread Safety

The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.