import type ts from "typescript/lib/tsserverlibrary";
import { expandSync } from "@ts-macros/swc-napi";

interface PluginConfig {
  // Config options (optional)
}

const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];

function shouldProcess(fileName: string) {
  return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}

// Cache expansion results to avoid re-running on every call
interface CachedExpansion {
  version: string;
  output: string | null;
  diagnostics: MacroDiagnostic[];
}

interface MacroDiagnostic {
  level: string;
  message: string;
  start?: number;
  end?: number;
}

function init(modules: { typescript: typeof ts }) {
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;
    const expansionCache = new Map<string, CachedExpansion>();

    function getExpansion(fileName: string, content: string, version: string): CachedExpansion {
      const cached = expansionCache.get(fileName);
      if (cached && cached.version === version) {
        return cached;
      }

      try {
        // Run the macro expansion
        const result = expandSync(content, fileName);
        
        const expansion: CachedExpansion = {
          version,
          output: result.types || null,
          diagnostics: result.diagnostics,
        };
        
        expansionCache.set(fileName, expansion);
        return expansion;
      } catch (e) {
        console.error('Plugin expansion failed:', e);
        // Fallback on error
        const errorExpansion: CachedExpansion = {
          version,
          output: null,
          diagnostics: [], // Could add a general diagnostic here
        };
        expansionCache.set(fileName, errorExpansion);
        return errorExpansion;
      }
    }

    // Hook getScriptSnapshot to provide the "expanded" type definition view
    const originalGetScriptSnapshot = info.languageServiceHost.getScriptSnapshot.bind(
      info.languageServiceHost
    );

    info.languageServiceHost.getScriptSnapshot = (fileName) => {
      if (!shouldProcess(fileName)) {
        return originalGetScriptSnapshot(fileName);
      }

      const snapshot = originalGetScriptSnapshot(fileName);
      if (!snapshot) {
        return snapshot;
      }

      // We need the file version to cache correctly.
      // getScriptVersion is usually available on the host.
      const version = info.languageServiceHost.getScriptVersion(fileName);
      const text = snapshot.getText(0, snapshot.getLength());

      // If the text doesn't contain macros, skip
      if (!text.includes("@Derive") && !text.includes("@")) {
         return snapshot;
      }

      const expansion = getExpansion(fileName, text, version);

      if (expansion.output) {
        return tsModule.ScriptSnapshot.fromString(expansion.output);
      } else {
        console.error('Plugin expansion returned no output for', fileName);
      }

      return snapshot;
    };

    // Hook getSemanticDiagnostics to provide macro errors
    const originalGetSemanticDiagnostics = info.languageService.getSemanticDiagnostics.bind(
      info.languageService
    );

    info.languageService.getSemanticDiagnostics = (fileName) => {
      const originalDiagnostics = originalGetSemanticDiagnostics(fileName);
      
      if (!shouldProcess(fileName)) {
        return originalDiagnostics;
      }

      const snapshot = originalGetScriptSnapshot(fileName);
      if (!snapshot) return originalDiagnostics;

      const version = info.languageServiceHost.getScriptVersion(fileName);
      const text = snapshot.getText(0, snapshot.getLength());
      const expansion = getExpansion(fileName, text, version);

      const macroDiagnostics: ts.Diagnostic[] = expansion.diagnostics.map((d) => {
        const category =
          d.level === "error"
            ? tsModule.DiagnosticCategory.Error
            : d.level === "warning"
            ? tsModule.DiagnosticCategory.Warning
            : tsModule.DiagnosticCategory.Message;

        return {
          file: info.languageService.getProgram()?.getSourceFile(fileName),
          start: d.start || 0,
          length: (d.end || 0) - (d.start || 0),
          messageText: d.message,
          category,
          code: 9999, // Custom error code
          source: "ts-macros",
        };
      });

      return [...originalDiagnostics, ...macroDiagnostics];
    };

    return info.languageService;
  }

  return { create };
}

export = init;
