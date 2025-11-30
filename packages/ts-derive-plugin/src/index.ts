import type ts from "typescript/lib/tsserverlibrary";
import type {
  ExpandOptions,
  ExpandResult,
  MacroDiagnostic,
} from "@ts-macros/swc-napi";
import type { PositionMapper } from "./source-map";

const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];

function shouldProcess(fileName: string) {
  return !fileName.endsWith(".expanded.ts");
}

interface CachedExpansion {
  version: string;
  codeOutput: string | null;
  typesOutput: string | null;
  diagnostics: MacroDiagnostic[];
  mapper: PositionMapper | null;
}

type NativeBindings = typeof import("@ts-macros/swc-napi");

// Try to load the native module, but don't crash if it fails
let nativeModuleLoaded = false;
let loadedNativeModule: NativeBindings | null = null;

try {
  loadedNativeModule = require("@ts-macros/swc-napi") as NativeBindings;
  nativeModuleLoaded = true;
} catch (e) {
  // Native module failed to load - plugin will be disabled
}

function init(modules: { typescript: typeof ts }) {
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;
    const expansionCache = new Map<string, CachedExpansion>();
    // Map to store generated virtual .d.ts files
    const virtualDtsFiles = new Map<string, ts.IScriptSnapshot>();
    // Cache snapshots to ensure identity stability for TypeScript's incremental compiler
    const snapshotCache = new Map<string, { version: string; snapshot: ts.IScriptSnapshot }>();
    // Guard against reentrancy
    const processingFiles = new Set<string>();

    // Write logs to a file we can actually see
    const fs = require("fs");
    const logFile = "/tmp/ts-macros-plugin.log";

    // Clear log on startup
    try {
      fs.writeFileSync(
        logFile,
        `=== ts-macros plugin loaded at ${new Date().toISOString()} ===\n`,
      );
    } catch {}

    const log = (msg: string) => {
      const line = `[${new Date().toISOString()}] ${msg}\n`;
      try {
        fs.appendFileSync(logFile, line);
      } catch {}
      try {
        info.project.projectService.logger.info(`[ts-macros] ${msg}`);
      } catch {}
      try {
        console.error(`[ts-macros] ${msg}`);
      } catch {}
    };

    // Log plugin initialization with module status
    log("Plugin initialized");

    // Instantiate a native plugin per language service instance
    const nativePlugin =
      nativeModuleLoaded && loadedNativeModule?.NativePlugin
        ? new loadedNativeModule.NativePlugin()
        : null;

    if (!nativePlugin) {
      log("Native plugin unavailable; using original language service");
      return info.languageService;
    }

    function getExpansion(
      fileName: string,
      content: string,
      version: string,
    ): CachedExpansion {
      // 1. Check Cache
      const cached = expansionCache.get(fileName);
      if (cached && cached.version === version) {
        return cached;
      }

      // 2. Define Fallback (No-Op) State
      const noOpExpansion: CachedExpansion = {
        version,
        codeOutput: content,
        typesOutput: null,
        diagnostics: [],
        mapper: null,
      };

      // 3. Fast Exit: Empty Content
      if (!content || content.trim().length === 0) {
        return noOpExpansion;
      }

      try {
        log(`getExpansion START for ${fileName}`);

        if (!nativePlugin?.processFile) {
          return noOpExpansion;
        }

        const result = nativePlugin.processFile(fileName, content, {
          keepDecorators: true,
          version,
        }) as ExpandResult;

        const mapper =
          (nativePlugin.getMapper?.(fileName) as PositionMapper | undefined) ||
          null;

        if (mapper && !mapper.isEmpty() && result.sourceMapping) {
          log(
            `Source mapping: ${result.sourceMapping.segments.length} segments, ${result.sourceMapping.generatedRegions.length} generated regions`,
          );
        }

        const expansion: CachedExpansion = {
          version,
          codeOutput: result.code || null,
          typesOutput: result.types || null,
          diagnostics: result.diagnostics || [],
          mapper,
        };

        // 6. Update State (Cache & Virtual Files)
        expansionCache.set(fileName, expansion);

        const virtualDtsFileName = fileName + ".ts-macros.d.ts";
        if (expansion.typesOutput) {
          virtualDtsFiles.set(
            virtualDtsFileName,
            tsModule.ScriptSnapshot.fromString(expansion.typesOutput),
          );
          log(`Generated virtual .d.ts for ${fileName}`);
        } else {
          virtualDtsFiles.delete(virtualDtsFileName);
        }

        return expansion;
      } catch (e) {
        // 7. Error Recovery
        const errorMessage =
          e instanceof Error ? e.stack || e.message : String(e);
        log(`Plugin expansion failed for ${fileName}: ${errorMessage}`);

        expansionCache.set(fileName, noOpExpansion);
        virtualDtsFiles.delete(fileName + ".ts-macros.d.ts");
        return noOpExpansion;
      }
    }

    // Hook getScriptVersion to provide versions for virtual .d.ts files
    const originalGetScriptVersion =
      info.languageServiceHost.getScriptVersion.bind(info.languageServiceHost);

    info.languageServiceHost.getScriptVersion = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName)) {
          const sourceFileName = fileName.replace(".ts-macros.d.ts", "");
          return originalGetScriptVersion(sourceFileName);
        }
        return originalGetScriptVersion(fileName);
      } catch (e) {
        log(
          `Error in getScriptVersion: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetScriptVersion(fileName);
      }
    };

    // Hook getScriptFileNames to include our virtual .d.ts files
    // This allows TS to "see" these new files as part of the project
    const originalGetScriptFileNames = info.languageServiceHost
      .getScriptFileNames
      ? info.languageServiceHost.getScriptFileNames.bind(
          info.languageServiceHost,
        )
      : () => [];

    info.languageServiceHost.getScriptFileNames = () => {
      try {
        const originalFiles = originalGetScriptFileNames();
        return [...originalFiles, ...Array.from(virtualDtsFiles.keys())];
      } catch (e) {
        log(
          `Error in getScriptFileNames: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetScriptFileNames();
      }
    };

    // Hook fileExists to resolve our virtual .d.ts files
    const originalFileExists = info.languageServiceHost.fileExists
      ? info.languageServiceHost.fileExists.bind(info.languageServiceHost)
      : tsModule.sys.fileExists;

    info.languageServiceHost.fileExists = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName)) {
          return true;
        }
        return originalFileExists(fileName);
      } catch (e) {
        log(
          `Error in fileExists: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalFileExists(fileName);
      }
    };

    // Hook getScriptSnapshot to provide the "expanded" type definition view
    const originalGetScriptSnapshot =
      info.languageServiceHost.getScriptSnapshot.bind(info.languageServiceHost);

    info.languageServiceHost.getScriptSnapshot = (fileName) => {
      try {
        log(`getScriptSnapshot: ${fileName}`);

        // If it's one of our virtual .d.ts files, return its snapshot
        if (virtualDtsFiles.has(fileName)) {
          log(`  -> virtual .d.ts cache hit`);
          return virtualDtsFiles.get(fileName);
        }

        // Guard against reentrancy - if we're already processing this file, return original
        if (processingFiles.has(fileName)) {
          log(`  -> REENTRANCY DETECTED, returning original`);
          return originalGetScriptSnapshot(fileName);
        }

        // Don't process non-TypeScript files or .expanded.ts files
        if (!shouldProcess(fileName)) {
          log(`  -> not processable (excluded file), returning original`);
          return originalGetScriptSnapshot(fileName);
        }

        const snapshot = originalGetScriptSnapshot(fileName);
        if (!snapshot) {
          log(`  -> no snapshot available`);
          return snapshot;
        }

        const text = snapshot.getText(0, snapshot.getLength());

        // Only process files with @Derive decorator
        if (!text.includes("@Derive")) {
          log(`  -> no @Derive, returning original`);
          return snapshot;
        }

        log(`  -> has @Derive, expanding...`);
        // Mark as processing to prevent reentrancy
        processingFiles.add(fileName);
        try {
          const version = info.languageServiceHost.getScriptVersion(fileName);
          log(`  -> version: ${version}`);

          // Check if we have a cached snapshot for this version
          const cached = snapshotCache.get(fileName);
          if (cached && cached.version === version) {
            log(`  -> snapshot cache hit`);
            return cached.snapshot;
          }

          const expansion = getExpansion(fileName, text, version);
          log(`  -> getExpansion returned`);

          if (expansion.codeOutput && expansion.codeOutput !== text) {
            log(
              `  -> creating expanded snapshot (${expansion.codeOutput.length} chars)`,
            );
            const expandedSnapshot = tsModule.ScriptSnapshot.fromString(expansion.codeOutput);
            // Cache the snapshot for stable identity
            snapshotCache.set(fileName, { version, snapshot: expandedSnapshot });
            log(`  -> returning expanded snapshot`);
            return expandedSnapshot;
          }

          // Cache the original snapshot
          snapshotCache.set(fileName, { version, snapshot });
          return snapshot;
        } finally {
          processingFiles.delete(fileName);
        }
      } catch (e) {
        log(
          `ERROR in getScriptSnapshot for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        // Make sure we clean up on error
        processingFiles.delete(fileName);
        return originalGetScriptSnapshot(fileName);
      }
    };

    // Helper to get mapper for a file (triggering expansion if needed)
    function getMapper(fileName: string): PositionMapper | null {
      const nativeMapper = nativePlugin?.getMapper?.(fileName) as
        | PositionMapper
        | undefined;
      if (nativeMapper) {
        return nativeMapper;
      }
      return expansionCache.get(fileName)?.mapper ?? null;
    }

    function toPlainDiagnostic(diag: ts.Diagnostic): {
      start?: number;
      length?: number;
      message?: string;
      code?: number;
      category?: string;
    } {
      const message =
        typeof diag.messageText === "string"
          ? diag.messageText
          : diag.messageText.messageText;
      const category =
        diag.category === tsModule.DiagnosticCategory.Error
          ? "error"
          : diag.category === tsModule.DiagnosticCategory.Warning
            ? "warning"
            : "message";

      return {
        start: diag.start,
        length: diag.length,
        message,
        code: diag.code,
        category,
      };
    }

    function applyMappedDiagnostics(
      original: readonly ts.Diagnostic[],
      mapped: Array<{ start?: number; length?: number }>,
    ): ts.Diagnostic[] {
      return original.map((diag, idx) => {
        const mappedDiag = mapped[idx];
        if (
          !mappedDiag ||
          mappedDiag.start === undefined ||
          mappedDiag.length === undefined
        ) {
          return diag;
        }

        return {
          ...diag,
          start: mappedDiag.start,
          length: mappedDiag.length,
        };
      });
    }

    // Hook getSemanticDiagnostics to provide macro errors and map positions
    const originalGetSemanticDiagnostics =
      info.languageService.getSemanticDiagnostics.bind(info.languageService);

    info.languageService.getSemanticDiagnostics = (fileName) => {
      try {
        log(`getSemanticDiagnostics: ${fileName}`);

        // If it's one of our virtual .d.ts files, don't get diagnostics for it
        if (virtualDtsFiles.has(fileName)) {
          log(`  -> skipping virtual .d.ts`);
          return [];
        }

        if (!shouldProcess(fileName)) {
          log(`  -> not processable, using original`);
          return originalGetSemanticDiagnostics(fileName);
        }

        if (!nativePlugin?.mapDiagnostics) {
          log(`  -> native plugin unavailable, using original diagnostics`);
          return originalGetSemanticDiagnostics(fileName);
        }

        // Ensure mapper is ready
        getMapper(fileName);

        log(`  -> getting original diagnostics...`);
        const expandedDiagnostics = originalGetSemanticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics`);

        log(`  -> mapping diagnostics in native plugin`);
        const mappedDiagnostics = applyMappedDiagnostics(
          expandedDiagnostics,
          nativePlugin.mapDiagnostics(
            fileName,
            expandedDiagnostics.map(toPlainDiagnostic),
          ),
        );

        // Also get macro diagnostics from expansion
        const version = info.languageServiceHost.getScriptVersion(fileName);
        const cached = expansionCache.get(fileName);

        if (
          !cached ||
          cached.version !== version ||
          cached.diagnostics.length === 0
        ) {
          return mappedDiagnostics;
        }

        const macroDiagnostics: ts.Diagnostic[] = cached.diagnostics.map(
          (d) => {
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
          },
        );

        return [...mappedDiagnostics, ...macroDiagnostics];
      } catch (e) {
        log(
          `Error in getSemanticDiagnostics for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        return originalGetSemanticDiagnostics(fileName);
      }
    };

    // Hook getSyntacticDiagnostics to map positions
    const originalGetSyntacticDiagnostics =
      info.languageService.getSyntacticDiagnostics.bind(info.languageService);

    info.languageService.getSyntacticDiagnostics = (fileName) => {
      try {
        log(`getSyntacticDiagnostics: ${fileName}`);

        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          log(`  -> using original`);
          return originalGetSyntacticDiagnostics(fileName);
        }

        if (!nativePlugin?.mapDiagnostics) {
          return originalGetSyntacticDiagnostics(fileName);
        }

        // Ensure mapper ready
        getMapper(fileName);

        const expandedDiagnostics = originalGetSyntacticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics, mapping...`);
        const result = applyMappedDiagnostics(
          expandedDiagnostics,
          nativePlugin.mapDiagnostics(
            fileName,
            expandedDiagnostics.map(toPlainDiagnostic),
          ),
        ) as ts.DiagnosticWithLocation[];
        log(`  -> returning ${result.length} mapped diagnostics`);
        return result;
      } catch (e) {
        log(
          `ERROR in getSyntacticDiagnostics: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        return originalGetSyntacticDiagnostics(fileName);
      }
    };

    // Hook getQuickInfoAtPosition to map input position and output spans
    const originalGetQuickInfoAtPosition =
      info.languageService.getQuickInfoAtPosition.bind(info.languageService);

    info.languageService.getQuickInfoAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetQuickInfoAtPosition(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetQuickInfoAtPosition(fileName, position);
        }
        // Map original position to expanded
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetQuickInfoAtPosition(fileName, expandedPos);

        if (!result) return result;

        // Map result spans back to original
        const mappedTextSpan = mapper.mapSpanToOriginal(
          result.textSpan.start,
          result.textSpan.length,
        );
        if (!mappedTextSpan) return undefined; // In generated code - hide hover

        return {
          ...result,
          textSpan: {
            start: mappedTextSpan.start,
            length: mappedTextSpan.length,
          },
        };
      } catch (e) {
        log(
          `Error in getQuickInfoAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetQuickInfoAtPosition(fileName, position);
      }
    };

    // Hook getCompletionsAtPosition to map input position
    const originalGetCompletionsAtPosition =
      info.languageService.getCompletionsAtPosition.bind(info.languageService);

    info.languageService.getCompletionsAtPosition = (
      fileName,
      position,
      options,
      formattingSettings,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetCompletionsAtPosition(
            fileName,
            position,
            options,
            formattingSettings,
          );
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetCompletionsAtPosition(
            fileName,
            position,
            options,
            formattingSettings,
          );
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetCompletionsAtPosition(
          fileName,
          expandedPos,
          options,
          formattingSettings,
        );

        if (!result) return result;

        // Map optionalReplacementSpan if present
        let mappedOptionalSpan = undefined;
        if (result.optionalReplacementSpan) {
          const mapped = mapper.mapSpanToOriginal(
            result.optionalReplacementSpan.start,
            result.optionalReplacementSpan.length,
          );
          if (mapped) {
            mappedOptionalSpan = { start: mapped.start, length: mapped.length };
          }
        }

        // Map entries replacementSpan
        const mappedEntries = result.entries.map((entry) => {
          if (!entry.replacementSpan) return entry;
          const mapped = mapper.mapSpanToOriginal(
            entry.replacementSpan.start,
            entry.replacementSpan.length,
          );
          if (!mapped) return { ...entry, replacementSpan: undefined }; // Remove invalid span
          return {
            ...entry,
            replacementSpan: { start: mapped.start, length: mapped.length },
          };
        });

        return {
          ...result,
          optionalReplacementSpan: mappedOptionalSpan,
          entries: mappedEntries,
        };
      } catch (e) {
        log(
          `Error in getCompletionsAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetCompletionsAtPosition(
          fileName,
          position,
          options,
          formattingSettings,
        );
      }
    };

    // Hook getDefinitionAtPosition to map input and output positions
    const originalGetDefinitionAtPosition =
      info.languageService.getDefinitionAtPosition.bind(info.languageService);

    info.languageService.getDefinitionAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDefinitionAtPosition(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetDefinitionAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const definitions = originalGetDefinitionAtPosition(
          fileName,
          expandedPos,
        );

        if (!definitions) return definitions;

        // Map each definition's span back to original (only for same file)
        return definitions.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);
      } catch (e) {
        log(
          `Error in getDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDefinitionAtPosition(fileName, position);
      }
    };

    // Hook getDefinitionAndBoundSpan for more complete definition handling
    const originalGetDefinitionAndBoundSpan =
      info.languageService.getDefinitionAndBoundSpan.bind(info.languageService);

    info.languageService.getDefinitionAndBoundSpan = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDefinitionAndBoundSpan(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetDefinitionAndBoundSpan(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetDefinitionAndBoundSpan(fileName, expandedPos);

        if (!result) return result;

        // Map textSpan back to original
        const mappedTextSpan = mapper.mapSpanToOriginal(
          result.textSpan.start,
          result.textSpan.length,
        );
        if (!mappedTextSpan) return undefined; // In generated code

        // Map each definition's span
        const mappedDefinitions = result.definitions?.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);

        return {
          textSpan: {
            start: mappedTextSpan.start,
            length: mappedTextSpan.length,
          },
          definitions: mappedDefinitions,
        };
      } catch (e) {
        log(
          `Error in getDefinitionAndBoundSpan: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDefinitionAndBoundSpan(fileName, position);
      }
    };

    // Hook getTypeDefinitionAtPosition
    const originalGetTypeDefinitionAtPosition =
      info.languageService.getTypeDefinitionAtPosition.bind(
        info.languageService,
      );

    info.languageService.getTypeDefinitionAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetTypeDefinitionAtPosition(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetTypeDefinitionAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const definitions = originalGetTypeDefinitionAtPosition(
          fileName,
          expandedPos,
        );

        if (!definitions) return definitions;

        return definitions.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);
      } catch (e) {
        log(
          `Error in getTypeDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetTypeDefinitionAtPosition(fileName, position);
      }
    };

    // Hook getReferencesAtPosition
    const originalGetReferencesAtPosition =
      info.languageService.getReferencesAtPosition.bind(info.languageService);

    info.languageService.getReferencesAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetReferencesAtPosition(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetReferencesAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const refs = originalGetReferencesAtPosition(fileName, expandedPos);

        if (!refs) return refs;

        return refs.reduce((acc, ref) => {
          if (!shouldProcess(ref.fileName)) {
            acc.push(ref);
            return acc;
          }
          const refMapper = getMapper(ref.fileName);
          if (!refMapper) {
            acc.push(ref);
            return acc;
          }
          const mapped = refMapper.mapSpanToOriginal(
            ref.textSpan.start,
            ref.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...ref,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.ReferenceEntry[]);
      } catch (e) {
        log(
          `Error in getReferencesAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetReferencesAtPosition(fileName, position);
      }
    };

    // Hook findReferences
    const originalFindReferences = info.languageService.findReferences.bind(
      info.languageService,
    );

    info.languageService.findReferences = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalFindReferences(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalFindReferences(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const refSymbols = originalFindReferences(fileName, expandedPos);

        if (!refSymbols) return refSymbols;

        return refSymbols
          .map((refSymbol) => ({
            ...refSymbol,
            references: refSymbol.references.reduce((acc, ref) => {
              if (!shouldProcess(ref.fileName)) {
                acc.push(ref);
                return acc;
              }
              const refMapper = getMapper(ref.fileName);
              if (!refMapper) {
                acc.push(ref);
                return acc;
              }
              const mapped = refMapper.mapSpanToOriginal(
                ref.textSpan.start,
                ref.textSpan.length,
              );
              if (mapped) {
                acc.push({
                  ...ref,
                  textSpan: { start: mapped.start, length: mapped.length },
                });
              }
              return acc;
            }, [] as ts.ReferenceEntry[]),
          }))
          .filter((s) => s.references.length > 0);
      } catch (e) {
        log(
          `Error in findReferences: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalFindReferences(fileName, position);
      }
    };

    // Hook getSignatureHelpItems
    const originalGetSignatureHelpItems =
      info.languageService.getSignatureHelpItems.bind(info.languageService);

    info.languageService.getSignatureHelpItems = (
      fileName,
      position,
      options,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetSignatureHelpItems(fileName, position, options);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetSignatureHelpItems(fileName, position, options);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetSignatureHelpItems(
          fileName,
          expandedPos,
          options,
        );

        if (!result) return result;

        // Map applicableSpan back to original
        const mappedSpan = mapper.mapSpanToOriginal(
          result.applicableSpan.start,
          result.applicableSpan.length,
        );
        if (!mappedSpan) return undefined;

        return {
          ...result,
          applicableSpan: {
            start: mappedSpan.start,
            length: mappedSpan.length,
          },
        };
      } catch (e) {
        log(
          `Error in getSignatureHelpItems: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetSignatureHelpItems(fileName, position, options);
      }
    };

    // Hook getRenameInfo
    const originalGetRenameInfo = (
      info.languageService.getRenameInfo as any
    ).bind(info.languageService);

    type RenameInfoOptions = {
      allowRenameOfImportPath?: boolean;
    };

    const callGetRenameInfo = (
      fileName: string,
      position: number,
      options?: RenameInfoOptions,
    ) => {
      // Prefer object overload if available; otherwise fall back to legacy args
      if ((originalGetRenameInfo as any).length <= 2) {
        return (originalGetRenameInfo as any)(fileName, position, options);
      }
      return (originalGetRenameInfo as any)(
        fileName,
        position,
        options?.allowRenameOfImportPath,
      );
    };

    info.languageService.getRenameInfo = (fileName, position, options) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return callGetRenameInfo(fileName, position, options as any);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return callGetRenameInfo(fileName, position, options as any);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = callGetRenameInfo(fileName, expandedPos, options as any);

        if (!result.canRename || !result.triggerSpan) return result;

        const mappedSpan = mapper.mapSpanToOriginal(
          result.triggerSpan.start,
          result.triggerSpan.length,
        );
        if (!mappedSpan) {
          return {
            canRename: false,
            localizedErrorMessage: "Cannot rename in generated code",
          };
        }

        return {
          ...result,
          triggerSpan: { start: mappedSpan.start, length: mappedSpan.length },
        };
      } catch (e) {
        log(
          `Error in getRenameInfo: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetRenameInfo(fileName, position, options);
      }
    };

    // Hook findRenameLocations (newer overload prefers options object)
    const originalFindRenameLocations =
      info.languageService.findRenameLocations.bind(info.languageService);

    type RenameLocationOptions = {
      findInStrings?: boolean;
      findInComments?: boolean;
      providePrefixAndSuffixTextForRename?: boolean;
    };

    const callFindRenameLocations = (
      fileName: string,
      position: number,
      opts?: RenameLocationOptions,
    ) => {
      // Prefer object overload if available; otherwise fall back to legacy args
      if ((originalFindRenameLocations as any).length <= 3) {
        return (originalFindRenameLocations as any)(fileName, position, opts);
      }
      return (originalFindRenameLocations as any)(
        fileName,
        position,
        !!opts?.findInStrings,
        !!opts?.findInComments,
        !!opts?.providePrefixAndSuffixTextForRename,
      );
    };

    (info.languageService as any).findRenameLocations = (
      fileName: string,
      position: number,
      options?: RenameLocationOptions,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return callFindRenameLocations(fileName, position, options);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return callFindRenameLocations(fileName, position, options);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const locations = callFindRenameLocations(
          fileName,
          expandedPos,
          options,
        );

        if (!locations) return locations;

        return locations.reduce((acc: ts.RenameLocation[], loc: ts.RenameLocation) => {
          if (!shouldProcess(loc.fileName)) {
            acc.push(loc);
            return acc;
          }
          const locMapper = getMapper(loc.fileName);
          if (!locMapper) {
            acc.push(loc);
            return acc;
          }
          const mapped = locMapper.mapSpanToOriginal(
            loc.textSpan.start,
            loc.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...loc,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.RenameLocation[]);
      } catch (e) {
        log(
          `Error in findRenameLocations: ${e instanceof Error ? e.message : String(e)}`,
        );
        return callFindRenameLocations(fileName, position, options);
      }
    };

    // Hook getDocumentHighlights
    const originalGetDocumentHighlights =
      info.languageService.getDocumentHighlights.bind(info.languageService);

    info.languageService.getDocumentHighlights = (
      fileName,
      position,
      filesToSearch,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDocumentHighlights(
            fileName,
            position,
            filesToSearch,
          );
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetDocumentHighlights(
            fileName,
            position,
            filesToSearch,
          );
        }
        const expandedPos = mapper.originalToExpanded(position);
        const highlights = originalGetDocumentHighlights(
          fileName,
          expandedPos,
          filesToSearch,
        );

        if (!highlights) return highlights;

        return highlights
          .map((docHighlight) => ({
            ...docHighlight,
            highlightSpans: docHighlight.highlightSpans.reduce((acc, span) => {
              if (!shouldProcess(docHighlight.fileName)) {
                acc.push(span);
                return acc;
              }
              const spanMapper = getMapper(docHighlight.fileName);
              if (!spanMapper) {
                acc.push(span);
                return acc;
              }
              const mapped = spanMapper.mapSpanToOriginal(
                span.textSpan.start,
                span.textSpan.length,
              );
              if (mapped) {
                acc.push({
                  ...span,
                  textSpan: { start: mapped.start, length: mapped.length },
                });
              }
              return acc;
            }, [] as ts.HighlightSpan[]),
          }))
          .filter((h) => h.highlightSpans.length > 0);
      } catch (e) {
        log(
          `Error in getDocumentHighlights: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDocumentHighlights(fileName, position, filesToSearch);
      }
    };

    // Hook getImplementationAtPosition
    const originalGetImplementationAtPosition =
      info.languageService.getImplementationAtPosition.bind(
        info.languageService,
      );

    info.languageService.getImplementationAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetImplementationAtPosition(fileName, position);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetImplementationAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const implementations = originalGetImplementationAtPosition(
          fileName,
          expandedPos,
        );

        if (!implementations) return implementations;

        return implementations.reduce((acc, impl) => {
          if (!shouldProcess(impl.fileName)) {
            acc.push(impl);
            return acc;
          }
          const implMapper = getMapper(impl.fileName);
          if (!implMapper) {
            acc.push(impl);
            return acc;
          }
          const mapped = implMapper.mapSpanToOriginal(
            impl.textSpan.start,
            impl.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...impl,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.ImplementationLocation[]);
      } catch (e) {
        log(
          `Error in getImplementationAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetImplementationAtPosition(fileName, position);
      }
    };
    // Hook getCodeFixesAtPosition
    const originalGetCodeFixesAtPosition =
      info.languageService.getCodeFixesAtPosition.bind(info.languageService);

    info.languageService.getCodeFixesAtPosition = (
      fileName,
      start,
      end,
      errorCodes,
      formatOptions,
      preferences,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetCodeFixesAtPosition(
            fileName,
            start,
            end,
            errorCodes,
            formatOptions,
            preferences,
          );
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetCodeFixesAtPosition(
            fileName,
            start,
            end,
            errorCodes,
            formatOptions,
            preferences,
          );
        }
        const expandedStart = mapper.originalToExpanded(start);
        const expandedEnd = mapper.originalToExpanded(end);
        return originalGetCodeFixesAtPosition(
          fileName,
          expandedStart,
          expandedEnd,
          errorCodes,
          formatOptions,
          preferences,
        );
      } catch (e) {
        log(
          `Error in getCodeFixesAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetCodeFixesAtPosition(
          fileName,
          start,
          end,
          errorCodes,
          formatOptions,
          preferences,
        );
      }
    };

    // Hook getNavigationTree
    const originalGetNavigationTree =
      info.languageService.getNavigationTree.bind(info.languageService);

    info.languageService.getNavigationTree = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetNavigationTree(fileName);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetNavigationTree(fileName);
        }
        const navMapper = mapper;
        const tree = originalGetNavigationTree(fileName);

        // Recursively map spans in navigation tree
        function mapNavigationItem(item: ts.NavigationTree): ts.NavigationTree {
          const mappedSpans = item.spans.map((span) => {
            const mapped = navMapper.mapSpanToOriginal(span.start, span.length);
            return mapped
              ? { start: mapped.start, length: mapped.length }
              : span;
          });

          const mappedNameSpan = item.nameSpan
            ? (navMapper.mapSpanToOriginal(
                item.nameSpan.start,
                item.nameSpan.length,
              ) ?? item.nameSpan)
            : undefined;

          return {
            ...item,
            spans: mappedSpans,
            nameSpan: mappedNameSpan
              ? { start: mappedNameSpan.start, length: mappedNameSpan.length }
              : undefined,
            childItems: item.childItems?.map(mapNavigationItem),
          };
        }

        return mapNavigationItem(tree);
      } catch (e) {
        log(
          `Error in getNavigationTree: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetNavigationTree(fileName);
      }
    };

    // Hook getOutliningSpans
    const originalGetOutliningSpans =
      info.languageService.getOutliningSpans.bind(info.languageService);

    info.languageService.getOutliningSpans = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetOutliningSpans(fileName);
        }

        const mapper = getMapper(fileName);
        if (!mapper) {
          return originalGetOutliningSpans(fileName);
        }
        const spans = originalGetOutliningSpans(fileName);

        return spans.map((span) => {
          const mappedTextSpan = mapper.mapSpanToOriginal(
            span.textSpan.start,
            span.textSpan.length,
          );
          const mappedHintSpan = mapper.mapSpanToOriginal(
            span.hintSpan.start,
            span.hintSpan.length,
          );

          if (!mappedTextSpan || !mappedHintSpan) return span;

          return {
            ...span,
            textSpan: {
              start: mappedTextSpan.start,
              length: mappedTextSpan.length,
            },
            hintSpan: {
              start: mappedHintSpan.start,
              length: mappedHintSpan.length,
            },
          };
        });
      } catch (e) {
        log(
          `Error in getOutliningSpans: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetOutliningSpans(fileName);
      }
    };

    // Hook provideInlayHints to map positions
    const originalProvideInlayHints =
      info.languageService.provideInlayHints?.bind(info.languageService);

    if (originalProvideInlayHints) {
      info.languageService.provideInlayHints = (
        fileName,
        span,
        preferences,
      ) => {
        try {
          if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
            return originalProvideInlayHints(fileName, span, preferences);
          }

          const mapper = getMapper(fileName);
          if (!mapper) {
            return originalProvideInlayHints(fileName, span, preferences);
          }
          // If no mapping info, avoid remapping to reduce risk
          if (mapper.isEmpty()) {
            return originalProvideInlayHints(fileName, span, preferences);
          }
          // Map the input span to expanded coordinates
          const expandedSpan = mapper.mapSpanToExpanded(
            span.start,
            span.length,
          );
          const result = originalProvideInlayHints(
            fileName,
            expandedSpan,
            preferences,
          );

          if (!result) return result;

          // Map each hint's position back to original coordinates
          return result.flatMap((hint) => {
            const originalPos = mapper.expandedToOriginal(hint.position);
            if (originalPos === null) {
              // Hint is in generated code, skip it
              return [];
            }
            return [
              {
                ...hint,
                position: originalPos,
              },
            ];
          });
        } catch (e) {
          log(
            `Error in provideInlayHints: ${e instanceof Error ? e.message : String(e)}`,
          );
          return originalProvideInlayHints(fileName, span, preferences);
        }
      };
    }

    return info.languageService;
  }

  return { create };
}

type TsMacrosPluginFactory = typeof init & {};

const pluginFactory = init as TsMacrosPluginFactory;

export = pluginFactory;
