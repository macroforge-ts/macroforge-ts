# NativePlugin
  *The main plugin class for macro expansion with caching support. `NativePlugin` is designed to be instantiated once and reused across multiple file processing operations. It maintains a cache of expansion results keyed by filepath and version, enabling efficient incremental processing.*
 ## Constructor
 ```
const plugin = new NativePlugin();
``` ## Methods
 ### processFile()
 Process a file with version-based caching:
 ```
processFile(
  filepath: string,
  code: string,
  options?: ProcessFileOptions
): ExpandResult
``` ```
interface ProcessFileOptions {
  // Cache key - if unchanged, returns cached result
  version?: string;
}
``` ### getMapper()
 Get the position mapper for a previously processed file:
 ```
getMapper(filepath: string): NativeMapper | null
``` ### mapDiagnostics()
 Map diagnostics from expanded positions to original positions:
 ```
mapDiagnostics(
  filepath: string,
  diagnostics: JsDiagnostic[]
): JsDiagnostic[]
``` ### log() / setLogFile()
 Logging utilities for debugging:
 ```
log(message: string): void
setLogFile(path: string): void
``` ## Caching Behavior
 The plugin caches expansion results by file path and version:
 ```
const plugin = new NativePlugin();

// First call - performs expansion
const result1 = plugin.processFile("user.ts", code, { version: "1" });

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, { version: "1" });

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, { version: "2" });
``` ## Example: Language Server Integration
 ```
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
``` ## Thread Safety
 The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.