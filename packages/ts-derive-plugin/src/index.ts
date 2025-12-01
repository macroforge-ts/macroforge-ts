import type ts from "typescript/lib/tsserverlibrary";
import type { ExpandResult } from "@ts-macros/swc-napi";
import { NativePlugin, PositionMapper } from "@ts-macros/swc-napi";
import path from "path";
import fs from "fs";

const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];

function shouldProcess(fileName: string) {
  const lower = fileName.toLowerCase();
  if (lower.endsWith(".expanded.ts")) return false;
  if (lower.endsWith(".d.ts")) return false;
  if (fileName.includes(`${path.sep}.ts-macros${path.sep}`)) return false;
  return FILE_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

function hasMacroDirectives(text: string) {
  return (
    text.includes("@derive") ||
    /\/\*\*\s*@derive\s*\(/i.test(text) ||
    /\/\*\*\s*import\s+macro\b/i.test(text)
  );
}

type MacroConfig = {
  keepDecorators: boolean;
};

function loadMacroConfig(startDir: string): MacroConfig {
  let current = startDir;
  const fallback: MacroConfig = { keepDecorators: false };

  while (true) {
    const candidate = path.join(current, "ts-macros.json");
    if (fs.existsSync(candidate)) {
      try {
        const raw = fs.readFileSync(candidate, "utf8");
        const parsed = JSON.parse(raw);
        return { keepDecorators: Boolean(parsed.keepDecorators) };
      } catch {
        return fallback;
      }
    }

    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }

  return fallback;
}

function init(modules: { typescript: typeof ts }) {
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;
    // Map to store generated virtual .d.ts files
    const virtualDtsFiles = new Map<string, ts.IScriptSnapshot>();
    // Cache snapshots to ensure identity stability for TypeScript's incremental compiler
    const snapshotCache = new Map<
      string,
      { version: string; snapshot: ts.IScriptSnapshot }
    >();
    // Guard against reentrancy
    const processingFiles = new Set<string>();

    // Instantiate native plugin (handles caching and logging in Rust)
    const nativePlugin = new NativePlugin();

    const getCurrentDirectory = () =>
      info.project.getCurrentDirectory?.() ??
      info.languageServiceHost.getCurrentDirectory?.() ??
      process.cwd();

    const macroConfig = loadMacroConfig(getCurrentDirectory());
    const keepDecorators = macroConfig.keepDecorators;

    // Log helper - delegates to Rust
    const log = (msg: string) => {
      const line = `[${new Date().toISOString()}] ${msg}`;
      nativePlugin.log(line);
      try {
        info.project.projectService.logger.info(`[ts-macros] ${msg}`);
      } catch {}
      try {
        console.error(`[ts-macros] ${msg}`);
      } catch {}
    };

    const ensureVirtualDtsRegistered = (fileName: string) => {
      const projectService = info.project.projectService as any;
      const register = projectService?.getOrCreateScriptInfoNotOpenedByClient;
      if (!register) return;

      try {
        const scriptInfo = register(
          fileName,
          getCurrentDirectory(),
          info.languageServiceHost,
          /*deferredDeleteOk*/ false,
        );
        if (scriptInfo?.attachToProject) {
          scriptInfo.attachToProject(info.project);
        }
      } catch (error) {
        log(
          `Failed to register virtual .d.ts ${fileName}: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    };

    const cleanupVirtualDts = (fileName: string) => {
      const projectService = info.project.projectService as any;
      const getScriptInfo = projectService?.getScriptInfo;
      if (!getScriptInfo) return;

      try {
        const scriptInfo = getScriptInfo.call(projectService, fileName);
        if (!scriptInfo) return;

        scriptInfo.detachFromProject?.(info.project);
        if (
          !scriptInfo.isScriptOpen?.() &&
          scriptInfo.containingProjects?.length === 0
        ) {
          projectService.deleteScriptInfo?.(scriptInfo);
        }
      } catch (error) {
        log(
          `Failed to clean up virtual .d.ts ${fileName}: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    };

    const projectService = info.project.projectService as any;
    if (projectService?.setDocument) {
      projectService.setDocument = (
        key: unknown,
        filePath: string,
        sourceFile: unknown,
      ) => {
        try {
          const scriptInfo =
            projectService.getScriptInfoForPath?.(filePath) ??
            projectService.getOrCreateScriptInfoNotOpenedByClient?.(
              filePath,
              getCurrentDirectory(),
              info.languageServiceHost,
              /*deferredDeleteOk*/ false,
            );

          if (!scriptInfo) {
            log(`Skipping cache write for missing ScriptInfo at ${filePath}`);
            return;
          }

          scriptInfo.attachToProject?.(info.project);
          // Mirror the behavior of the original setDocument but avoid throwing when ScriptInfo is absent.
          scriptInfo.cacheSourceFile = { key, sourceFile } as any;
        } catch (error) {
          log(
            `Error in guarded setDocument for ${filePath}: ${error instanceof Error ? error.message : String(error)}`,
          );
        }
      };
    }

    // Log plugin initialization
    log("Plugin initialized");

    // Process file through macro expansion (caching handled in Rust)
    function processFile(
      fileName: string,
      content: string,
      version: string,
    ): { result: ExpandResult; code: string } {
      // Fast Exit: Empty Content
      if (!content || content.trim().length === 0) {
        return {
          result: {
            code: content,
            types: undefined,
            metadata: undefined,
            diagnostics: [],
            sourceMapping: undefined,
          },
          code: content,
        };
      }

      try {
        log(`Processing ${fileName}`);

        const result = nativePlugin.processFile(fileName, content, {
          keepDecorators,
          version,
        });

        // Update virtual .d.ts files
        const virtualDtsFileName = fileName + ".ts-macros.d.ts";
        if (result.types) {
          virtualDtsFiles.set(
            virtualDtsFileName,
            tsModule.ScriptSnapshot.fromString(result.types),
          );
          ensureVirtualDtsRegistered(virtualDtsFileName);
          log(`Generated virtual .d.ts for ${fileName}`);
        } else {
          virtualDtsFiles.delete(virtualDtsFileName);
          cleanupVirtualDts(virtualDtsFileName);
        }

        return { result, code: result.code };
      } catch (e) {
        const errorMessage =
          e instanceof Error ? e.stack || e.message : String(e);
        log(`Plugin expansion failed for ${fileName}: ${errorMessage}`);

        virtualDtsFiles.delete(fileName + ".ts-macros.d.ts");
        cleanupVirtualDts(fileName + ".ts-macros.d.ts");
        return {
          result: {
            code: content,
            types: undefined,
            metadata: undefined,
            diagnostics: [],
            sourceMapping: undefined,
          },
          code: content,
        };
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
          // Avoid tsserver crashes when a file was reported but no snapshot exists
          log(
            `  -> no snapshot available for ${fileName}, returning empty snapshot`,
          );
          return tsModule.ScriptSnapshot.fromString("");
        }

        const text = snapshot.getText(0, snapshot.getLength());

        // Only process files with macro directives
        if (!hasMacroDirectives(text)) {
          log(`  -> no macro directives, returning original`);
          return snapshot;
        }

        log(`  -> has @derive, expanding...`);
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

          const { code } = processFile(fileName, text, version);
          log(`  -> processFile returned`);

          if (code && code !== text) {
            log(`  -> creating expanded snapshot (${code.length} chars)`);
            const expandedSnapshot = tsModule.ScriptSnapshot.fromString(code);
            // Cache the snapshot for stable identity
            snapshotCache.set(fileName, {
              version,
              snapshot: expandedSnapshot,
            });
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

        log(`  -> getting original diagnostics...`);
        const expandedDiagnostics = originalGetSemanticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics`);

        // Map diagnostics using mapper
        const effectiveMapper = nativePlugin.getMapper(fileName);
        let mappedDiagnostics: ts.Diagnostic[];

        if (effectiveMapper && !effectiveMapper.isEmpty()) {
          log(`  -> mapping diagnostics with mapper`);
          mappedDiagnostics = expandedDiagnostics.map((diag) => {
            if (diag.start === undefined || diag.length === undefined) {
              return diag;
            }
            const mapped = effectiveMapper!.mapSpanToOriginal(
              diag.start,
              diag.length,
            );
            if (!mapped) {
              return diag;
            }
            return {
              ...diag,
              start: mapped.start,
              length: mapped.length,
            };
          });
        } else {
          // Native plugin is guaranteed to exist after early return check
          log(`  -> mapping diagnostics in native plugin`);
          mappedDiagnostics = applyMappedDiagnostics(
            expandedDiagnostics,
            nativePlugin.mapDiagnostics(
              fileName,
              expandedDiagnostics.map(toPlainDiagnostic),
            ),
          );
        }

        // Get macro diagnostics from Rust (hits cache if already expanded)
        const snapshot = originalGetScriptSnapshot(fileName);
        if (!snapshot) {
          return mappedDiagnostics;
        }

        const text = snapshot.getText(0, snapshot.getLength());
        const version = info.languageServiceHost.getScriptVersion(fileName);
        const { result } = processFile(fileName, text, version);

        if (!result.diagnostics || result.diagnostics.length === 0) {
          return mappedDiagnostics;
        }

        const macroDiagnostics: ts.Diagnostic[] = result.diagnostics.map(
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

        // Ensure mapper ready
        nativePlugin.getMapper(fileName);

        const expandedDiagnostics = originalGetSyntacticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics, mapping...`);
        // Native plugin is guaranteed to exist after early return check
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
          const defMapper = nativePlugin.getMapper(def.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
          const defMapper = nativePlugin.getMapper(def.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
          const defMapper = nativePlugin.getMapper(def.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
          const refMapper = nativePlugin.getMapper(ref.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
              const refMapper = nativePlugin.getMapper(ref.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        return locations.reduce(
          (acc: ts.RenameLocation[], loc: ts.RenameLocation) => {
            if (!shouldProcess(loc.fileName)) {
              acc.push(loc);
              return acc;
            }
            const locMapper = nativePlugin.getMapper(loc.fileName);
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
          },
          [] as ts.RenameLocation[],
        );
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

        const mapper = nativePlugin.getMapper(fileName);
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
              const spanMapper = nativePlugin.getMapper(docHighlight.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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
          const implMapper = nativePlugin.getMapper(impl.fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

        const mapper = nativePlugin.getMapper(fileName);
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

          const mapper = nativePlugin.getMapper(fileName);
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

export = init;
