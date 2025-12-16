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
interface ProcessFileOptions &#123;
  // Cache key - if unchanged, returns cached result
  version?: string;
&#125;
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
const result1 = plugin.processFile("user.ts", code, &#123; version: "1" &#125;);

// Same version - returns cached result instantly
const result2 = plugin.processFile("user.ts", code, &#123; version: "1" &#125;);

// Different version - re-expands
const result3 = plugin.processFile("user.ts", newCode, &#123; version: "2" &#125;);
``` ## Example: Language Server Integration
 ```
import &#123; NativePlugin &#125; from "macroforge";

class MacroforgeLanguageService &#123;
  private plugin = new NativePlugin();

  processDocument(uri: string, content: string, version: number) &#123;
    // Process with version-based caching
    const result = this.plugin.processFile(uri, content, &#123;
      version: String(version)
    &#125;);

    // Get mapper for position translation
    const mapper = this.plugin.getMapper(uri);

    return &#123; result, mapper &#125;;
  &#125;

  getSemanticDiagnostics(uri: string, diagnostics: Diagnostic[]) &#123;
    // Map positions from expanded to original
    return this.plugin.mapDiagnostics(uri, diagnostics);
  &#125;
&#125;
``` ## Thread Safety
 The `NativePlugin` class is thread-safe and can be used from multiple async contexts. Each file is processed in an isolated thread with its own stack space.