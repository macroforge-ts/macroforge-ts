import type ts from "typescript/lib/tsserverlibrary";
import { expandSync as expandSyncImpl } from "@ts-macros/swc-napi";
import * as path from 'path';

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
  codeOutput: string | null;
  typesOutput: string | null;
  diagnostics: MacroDiagnostic[];
}

interface MacroDiagnostic {
  level: string;
  message: string;
  start?: number;
  end?: number;
}

type ExpandFn = typeof expandSyncImpl;
let expand: ExpandFn = expandSyncImpl;

function setExpandImpl(fn: ExpandFn) {
  expand = fn;
}

function resetExpandImpl() {
  expand = expandSyncImpl;
}

function init(modules: { typescript: typeof ts }) {
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;
    const expansionCache = new Map<string, CachedExpansion>();
    // Map to store generated virtual .d.ts files
    const virtualDtsFiles = new Map<string, ts.IScriptSnapshot>();
    
    function log(msg: string) {
      info.project.projectService.logger.info(`[ts-macros-plugin] ${msg}`);
    }

    // Log plugin initialization
    log('Plugin initialized');

    function getExpansion(fileName: string, content: string, version: string): CachedExpansion {
      const cached = expansionCache.get(fileName);
      if (cached && cached.version === version) {
        return cached;
      }

      try {
        // Run the macro expansion
        // const result = expand(content, fileName);
        const result = { code: "", types: "", diagnostics: [] };
        
        const expansion: CachedExpansion = {
          version,
          codeOutput: result.code || null,
          typesOutput: result.types || null,
          diagnostics: result.diagnostics,
        };
        
        expansionCache.set(fileName, expansion);

        // If typesOutput is present, create a virtual .d.ts file
        if (expansion.typesOutput) {
          const virtualDtsFileName = fileName + '.ts-macros.d.ts';
          const dtsSnapshot = tsModule.ScriptSnapshot.fromString(expansion.typesOutput);
          virtualDtsFiles.set(virtualDtsFileName, dtsSnapshot);
          log(`Generated virtual .d.ts for ${fileName} at ${virtualDtsFileName}`);
        } else {
             const virtualDtsFileName = fileName + '.ts-macros.d.ts';
             if (virtualDtsFiles.has(virtualDtsFileName)) {
                // If typesOutput is no longer present, remove the virtual .d.ts file
                virtualDtsFiles.delete(virtualDtsFileName);
             }
        }
        
        return expansion;
      } catch (e) {
        log(`Plugin expansion failed: ${e}`);
        // Fallback on error
        const errorExpansion: CachedExpansion = {
          version,
          codeOutput: null,
          typesOutput: null,
          diagnostics: [], 
        };
        expansionCache.set(fileName, errorExpansion);
        // Also clean up any virtual .d.ts file if expansion fails
        virtualDtsFiles.delete(fileName + '.ts-macros.d.ts');
        return errorExpansion;
      }
    }

    // Hook getScriptVersion to provide versions for virtual .d.ts files
    const originalGetScriptVersion = info.languageServiceHost.getScriptVersion.bind(
      info.languageServiceHost
    );

    info.languageServiceHost.getScriptVersion = (fileName) => {
      if (virtualDtsFiles.has(fileName)) {
        // We can use the version of the source file if we can map it back, 
        // or just use the cached version. 
        // For simplicity, since we update the virtual file whenever we expand,
        // we can assume it updates with the source file.
        // Let's assume the fileName is source.ts.ts-macros.d.ts
        const sourceFileName = fileName.replace('.ts-macros.d.ts', '');
        return originalGetScriptVersion(sourceFileName);
      }
      return originalGetScriptVersion(fileName);
    };

    // Hook getScriptFileNames to include our virtual .d.ts files
    // This allows TS to "see" these new files as part of the project
    const originalGetScriptFileNames = info.languageServiceHost.getScriptFileNames ? 
        info.languageServiceHost.getScriptFileNames.bind(info.languageServiceHost) : 
        () => [];

    info.languageServiceHost.getScriptFileNames = () => {
      const originalFiles = originalGetScriptFileNames();
      return [...originalFiles, ...Array.from(virtualDtsFiles.keys())];
    };


    // Hook fileExists to resolve our virtual .d.ts files
    const originalFileExists = info.languageServiceHost.fileExists ?
        info.languageServiceHost.fileExists.bind(info.languageServiceHost) :
        tsModule.sys.fileExists;

    info.languageServiceHost.fileExists = (fileName) => {
      if (virtualDtsFiles.has(fileName)) {
        return true;
      }
      return originalFileExists(fileName);
    };

    // Hook getScriptSnapshot to provide the "expanded" type definition view
    const originalGetScriptSnapshot = info.languageServiceHost.getScriptSnapshot.bind(
      info.languageServiceHost
    );

    info.languageServiceHost.getScriptSnapshot = (fileName) => {
      // If it's one of our virtual .d.ts files, return its snapshot
      if (virtualDtsFiles.has(fileName)) {
        // log(`Serving virtual .d.ts for ${fileName}`);
        return virtualDtsFiles.get(fileName);
      }

      if (!shouldProcess(fileName)) {
        return originalGetScriptSnapshot(fileName);
      }

      const snapshot = originalGetScriptSnapshot(fileName);
      if (!snapshot) {
        return snapshot;
      }

      // We need the file version to cache correctly.
      const version = info.languageServiceHost.getScriptVersion(fileName);
      const text = snapshot.getText(0, snapshot.getLength());

      // If the text doesn't contain macros, skip
      if (!text.includes("@Derive") && !text.includes("@")) {
         return snapshot;
      }
      
      const expansion = getExpansion(fileName, text, version);

      if (expansion.codeOutput) {
        // Inject reference to the generated d.ts file
        const dtsReference = expansion.typesOutput ? `/// <reference path="./${path.basename(fileName)}.ts-macros.d.ts" />\n` : '';
        const finalOutput = dtsReference + expansion.codeOutput;

        const expandedSnapshot = tsModule.ScriptSnapshot.fromString(finalOutput);
        // Debug: verify the expanded code contains the class
        if (expansion.codeOutput.includes('export class MacroUser') && !expansion.codeOutput.includes('export class MacroUser {')) {
          log(`Warning: Expanded code for ${fileName} may be malformed`);
        }
        return expandedSnapshot;
      } 

      return snapshot;
    };

    // Hook getSemanticDiagnostics to provide macro errors
    const originalGetSemanticDiagnostics = info.languageService.getSemanticDiagnostics.bind(
      info.languageService
    );

    info.languageService.getSemanticDiagnostics = (fileName) => {
      // If it's one of our virtual .d.ts files, don't get diagnostics for it
      if (virtualDtsFiles.has(fileName)) {
        return [];
      }

      if (!shouldProcess(fileName)) {
        return originalGetSemanticDiagnostics(fileName);
      }

      // Trigger expansion if needed
      info.languageServiceHost.getScriptSnapshot(fileName);

      // Get diagnostics
      const originalDiagnostics = originalGetSemanticDiagnostics(fileName);

      // Also get macro diagnostics from expansion
      const version = info.languageServiceHost.getScriptVersion(fileName);
      
      // We need to pass the *original* text to getExpansion if we want to re-expand 
      // correctly if it wasn't cached. However, since we called getScriptSnapshot above,
      // it should be cached. 
      // CAUTION: Since getScriptSnapshot might have returned the *expanded* code, 
      // we can't rely on calling it again to get original code if we needed it.
      // But we rely on the cache in getExpansion.
      
      // To be safe, we try to retrieve from cache first without arguments if possible, 
      // or pass empty string which is risky if not cached.
      // Better approach: The cache key is fileName.
      const cached = expansionCache.get(fileName);
      
      if (!cached || cached.version !== version) {
          // This case should be rare if getScriptSnapshot was called, but if it happens,
          // we might miss diagnostics if we don't have original text.
          // But typically LS calls getScriptSnapshot before diagnostics.
          return originalDiagnostics;
      }

      const macroDiagnostics: ts.Diagnostic[] = cached.diagnostics.map((d) => {
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

type TsMacrosPluginFactory = typeof init & {
  __setExpandSync?: (fn: ExpandFn) => void;
  __resetExpandSync?: () => void;
};

const pluginFactory = init as TsMacrosPluginFactory;
pluginFactory.__setExpandSync = setExpandImpl;
pluginFactory.__resetExpandSync = resetExpandImpl;

export = pluginFactory;