"use strict";
const source_map_1 = require("./source-map");
const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];
function shouldProcess(fileName) {
    if (fileName.endsWith(".expanded.ts")) {
        return false;
    }
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}
// Try to load the native module, but don't crash if it fails
let nativeModuleLoaded = false;
let nativeModuleError = null;
let expandSyncImpl = null;
try {
    const nativeModule = require("@ts-macros/swc-napi");
    expandSyncImpl = nativeModule.expandSync;
    nativeModuleLoaded = true;
}
catch (e) {
    nativeModuleError = e instanceof Error ? e.message : String(e);
    // Native module failed to load - plugin will be disabled
}
// Default no-op expand function
const noopExpand = (code, _fileName, _options) => ({
    code: code,
    types: undefined,
    diagnostics: [],
});
let expand = expandSyncImpl || noopExpand;
function setExpandImpl(fn) {
    expand = fn;
}
function resetExpandImpl() {
    expand = expandSyncImpl || noopExpand;
}
/** Cache of loaded macro packages by module specifier */
const macroPackageCache = new Map();
/**
 * Try to load a module as a macro package.
 * Returns null if the module is not a macro package.
 */
function tryLoadMacroPackage(moduleSpecifier, logger) {
    // Check cache first
    if (macroPackageCache.has(moduleSpecifier)) {
        return macroPackageCache.get(moduleSpecifier) || null;
    }
    try {
        const mod = require(moduleSpecifier);
        // Check if it's a macro package
        if (typeof mod.__tsMacrosIsMacroPackage !== 'function' || !mod.__tsMacrosIsMacroPackage()) {
            macroPackageCache.set(moduleSpecifier, null);
            return null;
        }
        // Get manifest
        const manifest = mod.__tsMacrosGetManifest();
        // Build run function map
        const runFunctions = new Map();
        for (const macro of manifest.macros) {
            const runFnName = `__tsMacrosRun${macro.name}`;
            if (typeof mod[runFnName] === 'function') {
                runFunctions.set(macro.name, mod[runFnName]);
                logger?.(`Loaded macro ${macro.name} from ${moduleSpecifier}`);
            }
        }
        const pkg = { module: mod, manifest, runFunctions };
        macroPackageCache.set(moduleSpecifier, pkg);
        logger?.(`Loaded macro package ${moduleSpecifier} with ${runFunctions.size} macros`);
        return pkg;
    }
    catch (e) {
        // Module not found or load error - not a macro package
        macroPackageCache.set(moduleSpecifier, null);
        return null;
    }
}
/**
 * Extract import sources from TypeScript/JavaScript code.
 * Returns a map of identifier name -> module specifier.
 */
function extractImportSources(code) {
    const importMap = new Map();
    // Simple regex-based import extraction
    // Matches: import { Foo, Bar } from "module"
    // And: import { Foo as Baz } from "module"
    const importRegex = /import\s+\{([^}]+)\}\s+from\s+["']([^"']+)["']/g;
    let match;
    while ((match = importRegex.exec(code)) !== null) {
        const imports = match[1];
        const moduleSpecifier = match[2];
        // Parse individual imports
        const identifiers = imports.split(',').map(s => s.trim());
        for (const id of identifiers) {
            // Handle "Foo as Bar" syntax
            const asMatch = id.match(/(\w+)\s+as\s+(\w+)/);
            if (asMatch) {
                importMap.set(asMatch[2], moduleSpecifier); // Use local name
            }
            else {
                const name = id.trim();
                if (name) {
                    importMap.set(name, moduleSpecifier);
                }
            }
        }
    }
    return importMap;
}
/**
 * Find macro packages that need to be loaded based on imports in the code.
 * Excludes the built-in @ts-macros/swc-napi package.
 */
function findMacroPackages(code, logger) {
    const importSources = extractImportSources(code);
    const packages = new Map();
    // Get unique module specifiers (excluding built-in)
    const moduleSpecifiers = new Set(importSources.values());
    for (const specifier of moduleSpecifiers) {
        // Skip built-in package
        if (specifier === '@ts-macros/swc-napi' || specifier === '@macro/derive') {
            continue;
        }
        const pkg = tryLoadMacroPackage(specifier, logger);
        if (pkg) {
            packages.set(specifier, pkg);
        }
    }
    return packages;
}
/**
 * Build macro name to package mapping for external macros.
 * Returns a map of macro name -> { moduleSpecifier, package }
 */
function buildMacroToPackageMap(code, logger) {
    const importSources = extractImportSources(code);
    const macroToPackage = new Map();
    for (const [name, moduleSpecifier] of importSources) {
        // Skip built-in
        if (moduleSpecifier === '@ts-macros/swc-napi' || moduleSpecifier === '@macro/derive') {
            continue;
        }
        const pkg = tryLoadMacroPackage(moduleSpecifier, logger);
        if (pkg && pkg.runFunctions.has(name)) {
            macroToPackage.set(name, { moduleSpecifier, package: pkg });
        }
    }
    return macroToPackage;
}
/**
 * Check if a diagnostic indicates an unresolved macro.
 * Returns the macro name and module if so.
 */
function parseUnresolvedMacroDiagnostic(diag) {
    // Pattern: "Macro {name} not found in module {module}"
    const match = diag.message.match(/Macro\s+(\w+)\s+not found in module\s+(\S+)/);
    if (match) {
        return { name: match[1], module: match[2] };
    }
    return null;
}
/** Convert native module source mapping to TypeScript-friendly format */
function convertNativeMapping(native) {
    if (!native) {
        return { segments: [], generatedRegions: [] };
    }
    return {
        segments: native.segments.map((s) => ({
            originalStart: s.originalStart,
            originalEnd: s.originalEnd,
            expandedStart: s.expandedStart,
            expandedEnd: s.expandedEnd,
        })),
        generatedRegions: native.generatedRegions.map((r) => ({
            start: r.start,
            end: r.end,
            sourceMacro: r.sourceMacro,
        })),
    };
}
function init(modules) {
    function create(info) {
        const tsModule = modules.typescript;
        const expansionCache = new Map();
        // Map to store generated virtual .d.ts files
        const virtualDtsFiles = new Map();
        function log(msg) {
            info.project?.projectService?.logger?.info(`[ts-macros-plugin] ${msg}`);
        }
        // Log plugin initialization with module status
        log('Plugin initialized');
        log(`Native module loaded: ${nativeModuleLoaded}`);
        if (nativeModuleError) {
            log(`Native module error: ${nativeModuleError}`);
        }
        function getExpansion(fileName, content, version) {
            const cached = expansionCache.get(fileName);
            if (cached && cached.version === version) {
                return cached;
            }
            try {
                // Build map of external macros available from imports
                const externalMacros = buildMacroToPackageMap(content, log);
                if (externalMacros.size > 0) {
                    log(`Found ${externalMacros.size} external macros: ${Array.from(externalMacros.keys()).join(', ')}`);
                }
                // Run the macro expansion
                log(`Expanding macros for ${fileName}`);
                const result = expand(content, fileName, { keepDecorators: true });
                log(`Expansion result: code=${result.code?.length ?? 0} chars, types=${result.types?.length ?? 0} chars, diagnostics=${result.diagnostics?.length ?? 0}`);
                // Filter out "macro not found" diagnostics for external macros that ARE available
                // These will be expanded at build time by the Vite plugin
                let filteredDiagnostics = result.diagnostics || [];
                if (externalMacros.size > 0) {
                    const originalCount = filteredDiagnostics.length;
                    filteredDiagnostics = filteredDiagnostics.filter(diag => {
                        const unresolved = parseUnresolvedMacroDiagnostic(diag);
                        if (unresolved && externalMacros.has(unresolved.name)) {
                            log(`Suppressing 'macro not found' for external macro: ${unresolved.name}`);
                            return false; // Filter out - macro is available in external package
                        }
                        return true;
                    });
                    if (filteredDiagnostics.length < originalCount) {
                        log(`Filtered ${originalCount - filteredDiagnostics.length} diagnostics for external macros`);
                    }
                }
                // Create position mapper from source mapping
                const sourceMapping = convertNativeMapping(result.sourceMapping);
                const hasMapping = sourceMapping.segments.length > 0 || sourceMapping.generatedRegions.length > 0;
                const mapper = hasMapping ? new source_map_1.PositionMapper(sourceMapping) : new source_map_1.IdentityMapper();
                if (hasMapping) {
                    log(`Source mapping: ${sourceMapping.segments.length} segments, ${sourceMapping.generatedRegions.length} generated regions`);
                }
                const expansion = {
                    version,
                    codeOutput: result.code || null,
                    typesOutput: result.types || null,
                    diagnostics: filteredDiagnostics,
                    mapper,
                };
                expansionCache.set(fileName, expansion);
                // If typesOutput is present, create a virtual .d.ts file
                if (expansion.typesOutput) {
                    const virtualDtsFileName = fileName + '.ts-macros.d.ts';
                    const dtsSnapshot = tsModule.ScriptSnapshot.fromString(expansion.typesOutput);
                    virtualDtsFiles.set(virtualDtsFileName, dtsSnapshot);
                    log(`Generated virtual .d.ts for ${fileName} at ${virtualDtsFileName}`);
                }
                else {
                    const virtualDtsFileName = fileName + '.ts-macros.d.ts';
                    if (virtualDtsFiles.has(virtualDtsFileName)) {
                        // If typesOutput is no longer present, remove the virtual .d.ts file
                        virtualDtsFiles.delete(virtualDtsFileName);
                    }
                }
                return expansion;
            }
            catch (e) {
                const errorMessage = e instanceof Error ? e.stack || e.message : String(e);
                log(`Plugin expansion failed for ${fileName}: ${errorMessage}`);
                // Fallback on error
                const errorExpansion = {
                    version,
                    codeOutput: null,
                    typesOutput: null,
                    diagnostics: [],
                    mapper: new source_map_1.IdentityMapper(),
                };
                expansionCache.set(fileName, errorExpansion);
                // Also clean up any virtual .d.ts file if expansion fails
                virtualDtsFiles.delete(fileName + '.ts-macros.d.ts');
                return errorExpansion;
            }
        }
        // Hook getScriptVersion to provide versions for virtual .d.ts files
        const originalGetScriptVersion = info.languageServiceHost.getScriptVersion.bind(info.languageServiceHost);
        info.languageServiceHost.getScriptVersion = (fileName) => {
            try {
                if (virtualDtsFiles.has(fileName)) {
                    const sourceFileName = fileName.replace('.ts-macros.d.ts', '');
                    return originalGetScriptVersion(sourceFileName);
                }
                return originalGetScriptVersion(fileName);
            }
            catch (e) {
                log(`Error in getScriptVersion: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetScriptVersion(fileName);
            }
        };
        // Hook getScriptFileNames to include our virtual .d.ts files
        // This allows TS to "see" these new files as part of the project
        const originalGetScriptFileNames = info.languageServiceHost.getScriptFileNames ?
            info.languageServiceHost.getScriptFileNames.bind(info.languageServiceHost) :
            () => [];
        info.languageServiceHost.getScriptFileNames = () => {
            try {
                const originalFiles = originalGetScriptFileNames();
                return [...originalFiles, ...Array.from(virtualDtsFiles.keys())];
            }
            catch (e) {
                log(`Error in getScriptFileNames: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetScriptFileNames();
            }
        };
        // Hook fileExists to resolve our virtual .d.ts files
        const originalFileExists = info.languageServiceHost.fileExists ?
            info.languageServiceHost.fileExists.bind(info.languageServiceHost) :
            tsModule.sys.fileExists;
        info.languageServiceHost.fileExists = (fileName) => {
            try {
                if (virtualDtsFiles.has(fileName)) {
                    return true;
                }
                return originalFileExists(fileName);
            }
            catch (e) {
                log(`Error in fileExists: ${e instanceof Error ? e.message : String(e)}`);
                return originalFileExists(fileName);
            }
        };
        // Hook getScriptSnapshot to provide the "expanded" type definition view
        const originalGetScriptSnapshot = info.languageServiceHost.getScriptSnapshot.bind(info.languageServiceHost);
        info.languageServiceHost.getScriptSnapshot = (fileName) => {
            try {
                // If it's one of our virtual .d.ts files, return its snapshot
                if (virtualDtsFiles.has(fileName)) {
                    return virtualDtsFiles.get(fileName);
                }
                const snapshot = originalGetScriptSnapshot(fileName);
                if (!snapshot) {
                    return snapshot;
                }
                // Don't process non-TypeScript files
                if (!shouldProcess(fileName)) {
                    return snapshot;
                }
                const text = snapshot.getText(0, snapshot.getLength());
                // Only process files with @Derive decorator
                if (!text.includes("@Derive")) {
                    return snapshot;
                }
                // Run expansion
                const version = info.languageServiceHost.getScriptVersion(fileName);
                const expansion = getExpansion(fileName, text, version);
                // Return expanded code if available
                // Use codeOutput which contains the actual implementation
                if (expansion.codeOutput && expansion.codeOutput !== text) {
                    log(`Returning expanded code for ${fileName} (${expansion.codeOutput.length} chars)`);
                    const expandedSnapshot = tsModule.ScriptSnapshot.fromString(expansion.codeOutput);
                    return expandedSnapshot;
                }
                return snapshot;
            }
            catch (e) {
                log(`Error in getScriptSnapshot for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`);
                return originalGetScriptSnapshot(fileName);
            }
        };
        // Helper to get mapper for a file (triggering expansion if needed)
        function getMapper(fileName) {
            const version = info.languageServiceHost.getScriptVersion(fileName);
            const cached = expansionCache.get(fileName);
            if (cached && cached.version === version) {
                return cached.mapper;
            }
            // Trigger expansion to get mapper
            info.languageServiceHost.getScriptSnapshot(fileName);
            const newCached = expansionCache.get(fileName);
            return newCached?.mapper ?? new source_map_1.IdentityMapper();
        }
        // Helper to map a diagnostic's position from expanded to original
        function mapDiagnosticPosition(diag, mapper) {
            if (diag.start === undefined || diag.length === undefined) {
                return diag;
            }
            const mapped = mapper.mapSpanToOriginal(diag.start, diag.length);
            if (!mapped) {
                // Diagnostic is in generated code - keep expanded position with a note
                return {
                    ...diag,
                    messageText: `[in generated code] ${typeof diag.messageText === 'string' ? diag.messageText : diag.messageText.messageText}`,
                };
            }
            return {
                ...diag,
                start: mapped.start,
                length: mapped.length,
            };
        }
        // Hook getSemanticDiagnostics to provide macro errors and map positions
        const originalGetSemanticDiagnostics = info.languageService.getSemanticDiagnostics.bind(info.languageService);
        info.languageService.getSemanticDiagnostics = (fileName) => {
            try {
                // If it's one of our virtual .d.ts files, don't get diagnostics for it
                if (virtualDtsFiles.has(fileName)) {
                    return [];
                }
                if (!shouldProcess(fileName)) {
                    return originalGetSemanticDiagnostics(fileName);
                }
                // Get mapper (triggers expansion if needed)
                const mapper = getMapper(fileName);
                // Get diagnostics (these are in expanded positions)
                const expandedDiagnostics = originalGetSemanticDiagnostics(fileName);
                // Map all diagnostic positions back to original
                const mappedDiagnostics = expandedDiagnostics.map((d) => mapDiagnosticPosition(d, mapper));
                // Also get macro diagnostics from expansion
                const version = info.languageServiceHost.getScriptVersion(fileName);
                const cached = expansionCache.get(fileName);
                if (!cached || cached.version !== version || cached.diagnostics.length === 0) {
                    return mappedDiagnostics;
                }
                const macroDiagnostics = cached.diagnostics.map((d) => {
                    const category = d.level === "error"
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
                return [...mappedDiagnostics, ...macroDiagnostics];
            }
            catch (e) {
                log(`Error in getSemanticDiagnostics for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`);
                return originalGetSemanticDiagnostics(fileName);
            }
        };
        // Hook getSyntacticDiagnostics to map positions
        const originalGetSyntacticDiagnostics = info.languageService.getSyntacticDiagnostics.bind(info.languageService);
        info.languageService.getSyntacticDiagnostics = (fileName) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetSyntacticDiagnostics(fileName);
                }
                const mapper = getMapper(fileName);
                const expandedDiagnostics = originalGetSyntacticDiagnostics(fileName);
                return expandedDiagnostics.map((d) => mapDiagnosticPosition(d, mapper));
            }
            catch (e) {
                log(`Error in getSyntacticDiagnostics: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetSyntacticDiagnostics(fileName);
            }
        };
        // Hook getQuickInfoAtPosition to map input position and output spans
        const originalGetQuickInfoAtPosition = info.languageService.getQuickInfoAtPosition.bind(info.languageService);
        info.languageService.getQuickInfoAtPosition = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetQuickInfoAtPosition(fileName, position);
                }
                const mapper = getMapper(fileName);
                // Map original position to expanded
                const expandedPos = mapper.originalToExpanded(position);
                const result = originalGetQuickInfoAtPosition(fileName, expandedPos);
                if (!result)
                    return result;
                // Map result spans back to original
                const mappedTextSpan = mapper.mapSpanToOriginal(result.textSpan.start, result.textSpan.length);
                if (!mappedTextSpan)
                    return undefined; // In generated code - hide hover
                return {
                    ...result,
                    textSpan: { start: mappedTextSpan.start, length: mappedTextSpan.length },
                };
            }
            catch (e) {
                log(`Error in getQuickInfoAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetQuickInfoAtPosition(fileName, position);
            }
        };
        // Hook getCompletionsAtPosition to map input position
        const originalGetCompletionsAtPosition = info.languageService.getCompletionsAtPosition.bind(info.languageService);
        info.languageService.getCompletionsAtPosition = (fileName, position, options, formattingSettings) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetCompletionsAtPosition(fileName, position, options, formattingSettings);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const result = originalGetCompletionsAtPosition(fileName, expandedPos, options, formattingSettings);
                if (!result)
                    return result;
                // Map optionalReplacementSpan if present
                let mappedOptionalSpan = undefined;
                if (result.optionalReplacementSpan) {
                    const mapped = mapper.mapSpanToOriginal(result.optionalReplacementSpan.start, result.optionalReplacementSpan.length);
                    if (mapped) {
                        mappedOptionalSpan = { start: mapped.start, length: mapped.length };
                    }
                }
                // Map entries replacementSpan
                const mappedEntries = result.entries.map(entry => {
                    if (!entry.replacementSpan)
                        return entry;
                    const mapped = mapper.mapSpanToOriginal(entry.replacementSpan.start, entry.replacementSpan.length);
                    if (!mapped)
                        return { ...entry, replacementSpan: undefined }; // Remove invalid span
                    return { ...entry, replacementSpan: { start: mapped.start, length: mapped.length } };
                });
                return {
                    ...result,
                    optionalReplacementSpan: mappedOptionalSpan,
                    entries: mappedEntries
                };
            }
            catch (e) {
                log(`Error in getCompletionsAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetCompletionsAtPosition(fileName, position, options, formattingSettings);
            }
        };
        // Hook getDefinitionAtPosition to map input and output positions
        const originalGetDefinitionAtPosition = info.languageService.getDefinitionAtPosition.bind(info.languageService);
        info.languageService.getDefinitionAtPosition = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetDefinitionAtPosition(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const definitions = originalGetDefinitionAtPosition(fileName, expandedPos);
                if (!definitions)
                    return definitions;
                // Map each definition's span back to original (only for same file)
                return definitions.reduce((acc, def) => {
                    if (def.fileName !== fileName) {
                        acc.push(def);
                        return acc;
                    }
                    const defMapper = getMapper(def.fileName);
                    const mapped = defMapper.mapSpanToOriginal(def.textSpan.start, def.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...def,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
            }
            catch (e) {
                log(`Error in getDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetDefinitionAtPosition(fileName, position);
            }
        };
        // Hook getDefinitionAndBoundSpan for more complete definition handling
        const originalGetDefinitionAndBoundSpan = info.languageService.getDefinitionAndBoundSpan.bind(info.languageService);
        info.languageService.getDefinitionAndBoundSpan = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetDefinitionAndBoundSpan(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const result = originalGetDefinitionAndBoundSpan(fileName, expandedPos);
                if (!result)
                    return result;
                // Map textSpan back to original
                const mappedTextSpan = mapper.mapSpanToOriginal(result.textSpan.start, result.textSpan.length);
                if (!mappedTextSpan)
                    return undefined; // In generated code
                // Map each definition's span
                const mappedDefinitions = result.definitions?.reduce((acc, def) => {
                    if (def.fileName !== fileName) {
                        acc.push(def);
                        return acc;
                    }
                    const defMapper = getMapper(def.fileName);
                    const mapped = defMapper.mapSpanToOriginal(def.textSpan.start, def.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...def,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
                return {
                    textSpan: { start: mappedTextSpan.start, length: mappedTextSpan.length },
                    definitions: mappedDefinitions,
                };
            }
            catch (e) {
                log(`Error in getDefinitionAndBoundSpan: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetDefinitionAndBoundSpan(fileName, position);
            }
        };
        // Hook getTypeDefinitionAtPosition
        const originalGetTypeDefinitionAtPosition = info.languageService.getTypeDefinitionAtPosition.bind(info.languageService);
        info.languageService.getTypeDefinitionAtPosition = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetTypeDefinitionAtPosition(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const definitions = originalGetTypeDefinitionAtPosition(fileName, expandedPos);
                if (!definitions)
                    return definitions;
                return definitions.reduce((acc, def) => {
                    if (def.fileName !== fileName) {
                        acc.push(def);
                        return acc;
                    }
                    const defMapper = getMapper(def.fileName);
                    const mapped = defMapper.mapSpanToOriginal(def.textSpan.start, def.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...def,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
            }
            catch (e) {
                log(`Error in getTypeDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetTypeDefinitionAtPosition(fileName, position);
            }
        };
        // Hook getReferencesAtPosition
        const originalGetReferencesAtPosition = info.languageService.getReferencesAtPosition.bind(info.languageService);
        info.languageService.getReferencesAtPosition = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetReferencesAtPosition(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const refs = originalGetReferencesAtPosition(fileName, expandedPos);
                if (!refs)
                    return refs;
                return refs.reduce((acc, ref) => {
                    if (!shouldProcess(ref.fileName)) {
                        acc.push(ref);
                        return acc;
                    }
                    const refMapper = getMapper(ref.fileName);
                    const mapped = refMapper.mapSpanToOriginal(ref.textSpan.start, ref.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...ref,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
            }
            catch (e) {
                log(`Error in getReferencesAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetReferencesAtPosition(fileName, position);
            }
        };
        // Hook findReferences
        const originalFindReferences = info.languageService.findReferences.bind(info.languageService);
        info.languageService.findReferences = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalFindReferences(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const refSymbols = originalFindReferences(fileName, expandedPos);
                if (!refSymbols)
                    return refSymbols;
                return refSymbols.map((refSymbol) => ({
                    ...refSymbol,
                    references: refSymbol.references.reduce((acc, ref) => {
                        if (!shouldProcess(ref.fileName)) {
                            acc.push(ref);
                            return acc;
                        }
                        const refMapper = getMapper(ref.fileName);
                        const mapped = refMapper.mapSpanToOriginal(ref.textSpan.start, ref.textSpan.length);
                        if (mapped) {
                            acc.push({
                                ...ref,
                                textSpan: { start: mapped.start, length: mapped.length },
                            });
                        }
                        return acc;
                    }, []),
                })).filter(s => s.references.length > 0);
            }
            catch (e) {
                log(`Error in findReferences: ${e instanceof Error ? e.message : String(e)}`);
                return originalFindReferences(fileName, position);
            }
        };
        // Hook getSignatureHelpItems
        const originalGetSignatureHelpItems = info.languageService.getSignatureHelpItems.bind(info.languageService);
        info.languageService.getSignatureHelpItems = (fileName, position, options) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetSignatureHelpItems(fileName, position, options);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const result = originalGetSignatureHelpItems(fileName, expandedPos, options);
                if (!result)
                    return result;
                // Map applicableSpan back to original
                const mappedSpan = mapper.mapSpanToOriginal(result.applicableSpan.start, result.applicableSpan.length);
                if (!mappedSpan)
                    return undefined;
                return {
                    ...result,
                    applicableSpan: { start: mappedSpan.start, length: mappedSpan.length },
                };
            }
            catch (e) {
                log(`Error in getSignatureHelpItems: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetSignatureHelpItems(fileName, position, options);
            }
        };
        // Hook getRenameInfo
        const originalGetRenameInfo = info.languageService.getRenameInfo.bind(info.languageService);
        info.languageService.getRenameInfo = (fileName, position, options) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetRenameInfo(fileName, position, options);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const result = originalGetRenameInfo(fileName, expandedPos, options);
                if (!result.canRename || !result.triggerSpan)
                    return result;
                const mappedSpan = mapper.mapSpanToOriginal(result.triggerSpan.start, result.triggerSpan.length);
                if (!mappedSpan) {
                    return { canRename: false, localizedErrorMessage: "Cannot rename in generated code" };
                }
                return {
                    ...result,
                    triggerSpan: { start: mappedSpan.start, length: mappedSpan.length },
                };
            }
            catch (e) {
                log(`Error in getRenameInfo: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetRenameInfo(fileName, position, options);
            }
        };
        // Hook findRenameLocations
        const originalFindRenameLocations = info.languageService.findRenameLocations.bind(info.languageService);
        // Cast to any to handle multiple overloads
        info.languageService.findRenameLocations = (fileName, position, findInStrings, findInComments, providePrefixAndSuffixTextForRename) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalFindRenameLocations(fileName, position, findInStrings, findInComments, providePrefixAndSuffixTextForRename);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const locations = originalFindRenameLocations(fileName, expandedPos, findInStrings, findInComments, providePrefixAndSuffixTextForRename);
                if (!locations)
                    return locations;
                return locations.reduce((acc, loc) => {
                    if (!shouldProcess(loc.fileName)) {
                        acc.push(loc);
                        return acc;
                    }
                    const locMapper = getMapper(loc.fileName);
                    const mapped = locMapper.mapSpanToOriginal(loc.textSpan.start, loc.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...loc,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
            }
            catch (e) {
                log(`Error in findRenameLocations: ${e instanceof Error ? e.message : String(e)}`);
                return originalFindRenameLocations(fileName, position, findInStrings, findInComments, providePrefixAndSuffixTextForRename);
            }
        };
        // Hook getDocumentHighlights
        const originalGetDocumentHighlights = info.languageService.getDocumentHighlights.bind(info.languageService);
        info.languageService.getDocumentHighlights = (fileName, position, filesToSearch) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetDocumentHighlights(fileName, position, filesToSearch);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const highlights = originalGetDocumentHighlights(fileName, expandedPos, filesToSearch);
                if (!highlights)
                    return highlights;
                return highlights.map((docHighlight) => ({
                    ...docHighlight,
                    highlightSpans: docHighlight.highlightSpans.reduce((acc, span) => {
                        if (!shouldProcess(docHighlight.fileName)) {
                            acc.push(span);
                            return acc;
                        }
                        const spanMapper = getMapper(docHighlight.fileName);
                        const mapped = spanMapper.mapSpanToOriginal(span.textSpan.start, span.textSpan.length);
                        if (mapped) {
                            acc.push({
                                ...span,
                                textSpan: { start: mapped.start, length: mapped.length },
                            });
                        }
                        return acc;
                    }, []),
                })).filter(h => h.highlightSpans.length > 0);
            }
            catch (e) {
                log(`Error in getDocumentHighlights: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetDocumentHighlights(fileName, position, filesToSearch);
            }
        };
        // Hook getImplementationAtPosition
        const originalGetImplementationAtPosition = info.languageService.getImplementationAtPosition.bind(info.languageService);
        info.languageService.getImplementationAtPosition = (fileName, position) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetImplementationAtPosition(fileName, position);
                }
                const mapper = getMapper(fileName);
                const expandedPos = mapper.originalToExpanded(position);
                const implementations = originalGetImplementationAtPosition(fileName, expandedPos);
                if (!implementations)
                    return implementations;
                return implementations.reduce((acc, impl) => {
                    if (!shouldProcess(impl.fileName)) {
                        acc.push(impl);
                        return acc;
                    }
                    const implMapper = getMapper(impl.fileName);
                    const mapped = implMapper.mapSpanToOriginal(impl.textSpan.start, impl.textSpan.length);
                    if (mapped) {
                        acc.push({
                            ...impl,
                            textSpan: { start: mapped.start, length: mapped.length },
                        });
                    }
                    return acc;
                }, []);
            }
            catch (e) {
                log(`Error in getImplementationAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetImplementationAtPosition(fileName, position);
            }
        };
        // Hook getCodeFixesAtPosition
        const originalGetCodeFixesAtPosition = info.languageService.getCodeFixesAtPosition.bind(info.languageService);
        info.languageService.getCodeFixesAtPosition = (fileName, start, end, errorCodes, formatOptions, preferences) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetCodeFixesAtPosition(fileName, start, end, errorCodes, formatOptions, preferences);
                }
                const mapper = getMapper(fileName);
                const expandedStart = mapper.originalToExpanded(start);
                const expandedEnd = mapper.originalToExpanded(end);
                return originalGetCodeFixesAtPosition(fileName, expandedStart, expandedEnd, errorCodes, formatOptions, preferences);
            }
            catch (e) {
                log(`Error in getCodeFixesAtPosition: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetCodeFixesAtPosition(fileName, start, end, errorCodes, formatOptions, preferences);
            }
        };
        // Hook getNavigationTree
        const originalGetNavigationTree = info.languageService.getNavigationTree.bind(info.languageService);
        info.languageService.getNavigationTree = (fileName) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetNavigationTree(fileName);
                }
                const mapper = getMapper(fileName);
                const tree = originalGetNavigationTree(fileName);
                // Recursively map spans in navigation tree
                function mapNavigationItem(item) {
                    const mappedSpans = item.spans.map((span) => {
                        const mapped = mapper.mapSpanToOriginal(span.start, span.length);
                        return mapped ? { start: mapped.start, length: mapped.length } : span;
                    });
                    const mappedNameSpan = item.nameSpan
                        ? mapper.mapSpanToOriginal(item.nameSpan.start, item.nameSpan.length) ?? item.nameSpan
                        : undefined;
                    return {
                        ...item,
                        spans: mappedSpans,
                        nameSpan: mappedNameSpan ? { start: mappedNameSpan.start, length: mappedNameSpan.length } : undefined,
                        childItems: item.childItems?.map(mapNavigationItem),
                    };
                }
                return mapNavigationItem(tree);
            }
            catch (e) {
                log(`Error in getNavigationTree: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetNavigationTree(fileName);
            }
        };
        // Hook getOutliningSpans
        const originalGetOutliningSpans = info.languageService.getOutliningSpans.bind(info.languageService);
        info.languageService.getOutliningSpans = (fileName) => {
            try {
                if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
                    return originalGetOutliningSpans(fileName);
                }
                const mapper = getMapper(fileName);
                const spans = originalGetOutliningSpans(fileName);
                return spans.map((span) => {
                    const mappedTextSpan = mapper.mapSpanToOriginal(span.textSpan.start, span.textSpan.length);
                    const mappedHintSpan = mapper.mapSpanToOriginal(span.hintSpan.start, span.hintSpan.length);
                    if (!mappedTextSpan || !mappedHintSpan)
                        return span;
                    return {
                        ...span,
                        textSpan: { start: mappedTextSpan.start, length: mappedTextSpan.length },
                        hintSpan: { start: mappedHintSpan.start, length: mappedHintSpan.length },
                    };
                });
            }
            catch (e) {
                log(`Error in getOutliningSpans: ${e instanceof Error ? e.message : String(e)}`);
                return originalGetOutliningSpans(fileName);
            }
        };
        return info.languageService;
    }
    return { create };
}
const pluginFactory = init;
pluginFactory.__setExpandSync = setExpandImpl;
pluginFactory.__resetExpandSync = resetExpandImpl;
module.exports = pluginFactory;
