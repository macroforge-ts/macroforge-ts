const { exec } = require('node:child_process');
const { promises: fs } = require('node:fs');
const path = require('node:path');

const ROOT_DIR = process.cwd();
const LOGS_DIR = path.join(ROOT_DIR, 'diagnostics_logs');

// Lazy-loaded TypeScript for Language Service
let ts = null;
function getTypeScript() {
    if (!ts) {
        try {
            ts = require('typescript');
        } catch {
            console.warn('TypeScript not found, plugin diagnostics will be skipped');
        }
    }
    return ts;
}

// Lazy-loaded ts-macros native module
let tsMacrosModule = null;
function getTsMacrosModule() {
    if (tsMacrosModule === undefined) {
        return tsMacrosModule;
    }
    if (tsMacrosModule === null) {
        try {
            tsMacrosModule = require('@ts-macros/swc-napi');
        } catch {
            tsMacrosModule = undefined;
            console.warn('ts-macros native module not found, macro diagnostics will be skipped');
        }
    }
    return tsMacrosModule;
}

// Parse command line arguments
const args = process.argv.slice(2);
const WRITE_LOGS = args.includes('--log');

// Cache for git ignore checks to avoid repeated calls for the same path
const gitIgnoreCache = new Map();

async function runCommand(command, cwd = ROOT_DIR) {
    return new Promise((resolve) => {
        exec(command, { cwd }, (error, stdout, stderr) => {
            resolve({ error, stdout, stderr });
        });
    });
}

async function isGitIgnored(filePath) {
    // Normalize the path for consistent caching
    const normalized = normalizeFilePath(filePath);

    if (gitIgnoreCache.has(normalized)) {
        return gitIgnoreCache.get(normalized);
    }

    const result = await new Promise((resolve) => {
        exec(`git check-ignore "${normalized}"`, { cwd: ROOT_DIR }, (error) => {
            if (!error) {
                resolve(true);
            } else if (error && error.code === 1) {
                resolve(false);
            } else {
                resolve(false);
            }
        });
    });

    gitIgnoreCache.set(normalized, result);
    return result;
}

function normalizeFilePath(filePath) {
    const absolute = path.isAbsolute(filePath) ? filePath : path.join(ROOT_DIR, filePath);
    const relative = path.relative(ROOT_DIR, absolute);
    return relative.replace(/\\/g, '/');
}

async function isIgnoredDiagnosticPath(filePath) {
    const normalized = normalizeFilePath(filePath);

    // Paths outside the project are not ignored (let them through for reporting)
    if (!normalized || normalized.startsWith('..')) {
        return false;
    }

    // Use git check-ignore to determine if the path should be ignored
    return await isGitIgnored(normalized);
}

async function parseTypeScriptErrors(output) {
    const errors = [];
    const errorRegex = /^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$/gm;
    let match;

    // Collect all matches first
    const matches = [];
    while ((match = errorRegex.exec(output)) !== null) {
        matches.push({
            filePath: match[1],
            line: parseInt(match[2], 10),
            column: parseInt(match[3], 10),
            code: match[4],
            message: match[5],
            raw: match[0],
        });
    }

    // Filter out ignored paths (async)
    for (const m of matches) {
        if (await isIgnoredDiagnosticPath(m.filePath)) {
            continue;
        }
        errors.push({
            file: m.filePath,
            line: m.line,
            column: m.column,
            code: m.code,
            message: m.message,
            raw: m.raw,
            tool: 'tsc'
        });
    }

    return errors;
}

function parseBiomeErrors(output) {
    const errors = [];
    // Biome output can be complex. We'll look for lines starting with 'error' or 'warning'
    // and try to extract a rule identifier.
    // Example:  lint/complexity/noForEach (https://biomejs.dev/l/no-for-each)
    const errorRegex = /^(.*?): (error|warning) (.*?): (.*)$/gm; // Basic match for now
    const ruleRegex = /(lint\/[a-zA-Z]+\/[a-zA-Z]+)/;

    let match;
    for (const line of output.split('\n')) {
        const lineMatch = line.match(errorRegex);
        if (lineMatch) {
            const ruleMatch = line.match(ruleRegex);
            const rule = ruleMatch ? ruleMatch[1] : 'biome-general';
            errors.push({
                code: rule,
                message: line, // Store the whole line for now
                raw: line,
                tool: 'biome'
            });
        }
    }
    return errors;
}

async function parseClippyErrors(output) {
    const errors = [];
    const candidates = [];

    // Parse JSON messages from clippy (one JSON object per line)
    for (const line of output.split('\n')) {
        if (!line.trim()) continue;

        try {
            const msg = JSON.parse(line);

            // Only process compiler messages with actual diagnostic info
            if (msg.reason !== 'compiler-message' || !msg.message) continue;

            const diagnostic = msg.message;

            // Skip notes and help messages, only capture warnings and errors
            if (diagnostic.level !== 'warning' && diagnostic.level !== 'error') continue;

            // Get the lint code (e.g., "clippy::needless_return" or "unused_variables")
            const code = diagnostic.code?.code || 'clippy-general';

            // Get file location from the primary span
            const primarySpan = diagnostic.spans?.find(s => s.is_primary) || diagnostic.spans?.[0];
            const file = primarySpan?.file_name || 'unknown';
            const line_num = primarySpan?.line_start || 0;
            const column = primarySpan?.column_start || 0;

            // Skip absolute paths (external crates)
            if (file.startsWith('/')) continue;

            // Format the raw output similar to rustc's default format
            const raw = `${diagnostic.level}[${code}]: ${diagnostic.message}\n  --> ${file}:${line_num}:${column}`;

            // Normalize the code - some already have clippy:: prefix, some don't
            const normalizedCode = code.startsWith('clippy::') ? code : `clippy::${code}`;

            candidates.push({
                file,
                line: line_num,
                column,
                code: normalizedCode,
                message: diagnostic.message,
                raw,
                tool: 'clippy'
            });
        } catch {
            // Not a JSON line, skip it
        }
    }

    // Filter out ignored paths (async) - this handles target/, node_modules, etc. via gitignore
    for (const candidate of candidates) {
        if (await isIgnoredDiagnosticPath(candidate.file)) {
            continue;
        }
        errors.push(candidate);
    }

    return errors;
}

function loadCompilerOptions(ts, tsConfigPath) {
    const projectRoot = path.dirname(tsConfigPath);
    const configFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
    if (configFile.error) {
        return { options: undefined, projectRoot };
    }
    const parsed = ts.parseJsonConfigFileContent(
        configFile.config,
        ts.sys,
        projectRoot
    );

    const options = {
        ...parsed.options,
        declaration: true,
        emitDeclarationOnly: true,
        noEmit: false,
        noEmitOnError: false,
        incremental: false
    };

    delete options.outDir;
    delete options.outFile;
    options.moduleResolution ??= ts.ModuleResolutionKind.Bundler;
    options.module ??= ts.ModuleKind.ESNext;
    options.target ??= ts.ScriptTarget.ESNext;
    options.strict ??= true;
    options.skipLibCheck ??= true;

    return { options, projectRoot };
}

function emitDeclarationsFromCode(ts, code, fileName, options, projectRoot) {
    if (!options) return undefined;

    const normalizedFileName = path.resolve(fileName);
    const compilerHost = ts.createCompilerHost(options, true);

    compilerHost.getSourceFile = (requestedFileName, languageVersion) => {
        if (path.resolve(requestedFileName) === normalizedFileName) {
            return ts.createSourceFile(requestedFileName, code, languageVersion, true);
        }
        const text = ts.sys.readFile(requestedFileName);
        return text !== undefined
            ? ts.createSourceFile(requestedFileName, text, languageVersion, true)
            : undefined;
    };

    compilerHost.readFile = (requestedFileName) => {
        return path.resolve(requestedFileName) === normalizedFileName
            ? code
            : ts.sys.readFile(requestedFileName);
    };

    compilerHost.fileExists = (requestedFileName) => {
        return (
            path.resolve(requestedFileName) === normalizedFileName || ts.sys.fileExists(requestedFileName)
        );
    };

    let output;
    const writeFile = (outputName, text) => {
        if (outputName.endsWith('.d.ts')) {
            output = text;
        }
    };

    const program = ts.createProgram([normalizedFileName], options, compilerHost);
    const emitResult = program.emit(undefined, writeFile, undefined, true);

    if (emitResult.emitSkipped && emitResult.diagnostics.length > 0) {
        const formatted = ts.formatDiagnosticsWithColorAndContext(emitResult.diagnostics, {
            getCurrentDirectory: () => projectRoot,
            getCanonicalFileName: (fileName) => fileName,
            getNewLine: () => ts.sys.newLine
        });
        console.warn(`[diagnostics] Declaration emit failed for ${path.relative(projectRoot, fileName)}\n${formatted}`);
        return undefined;
    }

    return output;
}

async function parseSvelteCheckErrors(output) {
    const errors = [];
    // svelte-check outputs in format: /path/to/file.svelte:line:col - (error|warning) message
    // Or with --output machine: /path/to/file.svelte:line:col:endLine:endCol type code message
    const errorRegex = /^(.+?):(\d+):(\d+)\s+-?\s*(error|warning|Error|Warning)\s+(.+)$/gm;

    let match;
    while ((match = errorRegex.exec(output)) !== null) {
        const filePath = match[1];
        if (await isIgnoredDiagnosticPath(filePath)) {
            continue;
        }

        const line = parseInt(match[2], 10);
        const column = parseInt(match[3], 10);
        const level = match[4].toLowerCase();
        const message = match[5].trim();

        // Extract error code if present (e.g., "ts(2322)" or "svelte(a11y-click-events-have-key-events)")
        const codeMatch = message.match(/^(\w+\([^)]+\))\s*/);
        const code = codeMatch ? codeMatch[1] : (level === 'error' ? 'svelte-error' : 'svelte-warning');

        const raw = `${level}[${code}]: ${message}\n  --> ${filePath}:${line}:${column}`;

        errors.push({
            file: filePath,
            line,
            column,
            code,
            message,
            raw,
            tool: 'svelte-check'
        });
    }

    return errors;
}

async function getSvelteProjectPaths() {
    // Find directories containing svelte.config.js or svelte.config.ts
    const svelteConfigs = [];

    try {
        const files = await fs.readdir(ROOT_DIR, { recursive: true });
        for (const file of files) {
            if ((file.endsWith('svelte.config.js') || file.endsWith('svelte.config.ts'))
                && !file.includes('node_modules')
                && !file.includes('test/')
                && !file.includes('/fixtures/')) {
                const configPath = path.join(ROOT_DIR, file);
                if (!(await isGitIgnored(configPath))) {
                    svelteConfigs.push(path.dirname(configPath));
                }
            }
        }
    } catch {
        // Ignore errors
    }

    return svelteConfigs;
}

/**
 * Get diagnostics from TypeScript Language Service (includes plugin diagnostics)
 * This captures diagnostics that plugins report, which tsc --noEmit doesn't see.
 */
async function getLanguageServiceDiagnostics(tsConfigPath) {
    const typescript = getTypeScript();
    if (!typescript) {
        return [];
    }

    const errors = [];
    const projectDir = path.dirname(tsConfigPath);

    try {
        // Read and parse tsconfig
        const configFile = typescript.readConfigFile(tsConfigPath, typescript.sys.readFile);
        if (configFile.error) {
            console.warn(`  Warning: Could not read ${tsConfigPath}`);
            return [];
        }

        const parsedConfig = typescript.parseJsonConfigFileContent(
            configFile.config,
            typescript.sys,
            projectDir
        );

        // Check if there are any plugins configured
        const plugins = configFile.config?.compilerOptions?.plugins;
        if (!plugins || plugins.length === 0) {
            // No plugins, skip Language Service (tsc already handles this)
            return [];
        }

        console.log(`    Found ${plugins.length} plugin(s): ${plugins.map(p => p.name).join(', ')}`);

        // Create a language service host
        const files = new Map();
        const fileVersions = new Map();

        // Read all project files
        for (const fileName of parsedConfig.fileNames) {
            if (await isIgnoredDiagnosticPath(fileName)) {
                continue;
            }
            try {
                const content = await fs.readFile(fileName, 'utf-8');
                files.set(fileName, content);
                fileVersions.set(fileName, 1);
            } catch {
                // File might not exist or not readable
            }
        }

        const host = {
            getScriptFileNames: () => Array.from(files.keys()),
            getScriptVersion: (fileName) => String(fileVersions.get(fileName) || 0),
            getScriptSnapshot: (fileName) => {
                const content = files.get(fileName);
                if (content !== undefined) {
                    return typescript.ScriptSnapshot.fromString(content);
                }
                // Try to read from disk for lib files
                try {
                    const diskContent = typescript.sys.readFile(fileName);
                    if (diskContent) {
                        return typescript.ScriptSnapshot.fromString(diskContent);
                    }
                } catch {
                    // Ignore
                }
                return undefined;
            },
            getCurrentDirectory: () => projectDir,
            getCompilationSettings: () => parsedConfig.options,
            getDefaultLibFileName: (options) => typescript.getDefaultLibFilePath(options),
            fileExists: typescript.sys.fileExists,
            readFile: typescript.sys.readFile,
            readDirectory: typescript.sys.readDirectory,
            directoryExists: typescript.sys.directoryExists,
            getDirectories: typescript.sys.getDirectories,
        };

        // Create the language service
        const service = typescript.createLanguageService(host, typescript.createDocumentRegistry());

        // Collect diagnostics from all files
        for (const fileName of files.keys()) {
            try {
                // Get semantic diagnostics (this includes plugin diagnostics)
                const semanticDiagnostics = service.getSemanticDiagnostics(fileName);
                const syntacticDiagnostics = service.getSyntacticDiagnostics(fileName);
                const suggestionDiagnostics = service.getSuggestionDiagnostics(fileName);

                const allDiagnostics = [
                    ...semanticDiagnostics,
                    ...syntacticDiagnostics,
                    ...suggestionDiagnostics
                ];

                for (const diagnostic of allDiagnostics) {
                    // Skip diagnostics that tsc would already catch (non-plugin diagnostics)
                    // Plugin diagnostics typically have:
                    // - codes >= 100000
                    // - code 9999 (ts-macros)
                    // - source containing 'plugin' or 'macros'
                    // - source that's not 'ts' (standard TypeScript)
                    const isPluginDiagnostic =
                        diagnostic.code >= 100000 ||
                        diagnostic.code === 9999 ||
                        diagnostic.source?.includes('plugin') ||
                        diagnostic.source?.includes('macros') ||
                        diagnostic.source === 'ts-macros' ||
                        (diagnostic.source && diagnostic.source !== 'ts');

                    if (!isPluginDiagnostic) {
                        continue;
                    }

                    const file = diagnostic.file?.fileName || fileName;
                    let line = 1;
                    let column = 1;

                    if (diagnostic.file && diagnostic.start !== undefined) {
                        const pos = diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start);
                        line = pos.line + 1;
                        column = pos.character + 1;
                    }

                    const message = typescript.flattenDiagnosticMessageText(diagnostic.messageText, '\n');
                    const code = diagnostic.source
                        ? `${diagnostic.source}(${diagnostic.code})`
                        : `ts-plugin(${diagnostic.code})`;

                    const level = diagnostic.category === typescript.DiagnosticCategory.Error
                        ? 'error'
                        : diagnostic.category === typescript.DiagnosticCategory.Warning
                            ? 'warning'
                            : 'suggestion';

                    const raw = `${level}[${code}]: ${message}\n  --> ${file}:${line}:${column}`;

                    errors.push({
                        file,
                        line,
                        column,
                        code,
                        message,
                        raw,
                        tool: 'ts-language-service'
                    });
                }
            } catch (e) {
                // Skip files that cause errors
            }
        }

        // Clean up
        service.dispose();

    } catch (e) {
        console.warn(`  Warning: Language Service error for ${tsConfigPath}: ${e.message}`);
    }

    return errors;
}

/**
 * Get macro diagnostics by running the ts-macros expansion directly.
 * This catches errors like "Macro X not found in module Y".
 */
async function getMacroDiagnostics(projectDir) {
    const macroModule = getTsMacrosModule();
    if (!macroModule || !macroModule.expandSync) {
        return [];
    }

    const errors = [];

    try {
        // Find all TypeScript files in the project
        const files = await fs.readdir(projectDir, { recursive: true });
        const tsFiles = files.filter(f =>
            (f.endsWith('.ts') || f.endsWith('.tsx')) &&
            !f.includes('node_modules') &&
            !f.includes('.d.ts')
        );

        for (const file of tsFiles) {
            const filePath = path.join(projectDir, file);

            // Skip git-ignored files
            if (await isIgnoredDiagnosticPath(filePath)) {
                continue;
            }

            try {
                const content = await fs.readFile(filePath, 'utf-8');

                // Skip files without @Derive (quick check)
                if (!content.includes('@Derive')) {
                    continue;
                }

                // Run macro expansion
                const result = macroModule.expandSync(content, filePath);

                // Collect diagnostics
                if (result.diagnostics && result.diagnostics.length > 0) {
                    for (const diag of result.diagnostics) {
                        const level = diag.level || 'error';
                        const message = diag.message || 'Unknown macro error';
                        const start = diag.start || 0;

                        // Try to get line/column from position
                        let line = 1;
                        let column = 1;
                        if (start > 0) {
                            const beforeStart = content.substring(0, start);
                            const lines = beforeStart.split('\n');
                            line = lines.length;
                            column = (lines[lines.length - 1]?.length || 0) + 1;
                        }

                        const code = 'ts-macros(9999)';
                        const raw = `${level}[${code}]: ${message}\n  --> ${path.relative(ROOT_DIR, filePath)}:${line}:${column}`;

                        errors.push({
                            file: filePath,
                            line,
                            column,
                            code,
                            message,
                            raw,
                            tool: 'ts-macros'
                        });
                    }
                }
            } catch (e) {
                // Skip files that cause expansion errors (syntax errors, etc.)
            }
        }
    } catch (e) {
        console.warn(`  Warning: Error scanning for macro diagnostics: ${e.message}`);
    }

    return errors;
}

async function generateMacroTypes(tsConfigPaths) {
    const macroModule = getTsMacrosModule();
    const ts = getTypeScript();
    if (!macroModule || !macroModule.expandSync || !ts) {
        return;
    }

    for (const tsConfigPath of tsConfigPaths) {
        const projectRoot = path.dirname(tsConfigPath);
        const typesRoot = path.join(projectRoot, '.ts-macros', 'types');
        const { options } = loadCompilerOptions(ts, tsConfigPath);

        try {
            await fs.rm(typesRoot, { recursive: true, force: true });
            await fs.mkdir(typesRoot, { recursive: true });
        } catch {
            // ignore mkdir errors
        }

        let files = [];
        try {
            files = await fs.readdir(projectRoot, { recursive: true });
        } catch {
            continue;
        }

        for (const file of files) {
            const fullPath = path.join(projectRoot, file);
            const relPath = normalizeFilePath(fullPath);

            if (
                relPath.endsWith('.d.ts') ||
                (!relPath.endsWith('.ts') && !relPath.endsWith('.tsx')) ||
                relPath.includes('node_modules') ||
                relPath.includes('.ts-macros/types')
            ) {
                continue;
            }

            if (await isIgnoredDiagnosticPath(fullPath)) {
                continue;
            }

            let content = '';
            try {
                content = await fs.readFile(fullPath, 'utf-8');
            } catch {
                continue;
            }

            if (!content.includes('@Derive')) {
                continue;
            }

            try {
                const result = macroModule.expandSync(content, fullPath);
                const decls = emitDeclarationsFromCode(ts, result.code || content, fullPath, options, projectRoot);
                if (decls) {
                    const parsed = path.parse(path.relative(projectRoot, fullPath));
                    const outDir = parsed.dir ? path.join(typesRoot, parsed.dir) : typesRoot;
                    await fs.mkdir(outDir, { recursive: true });
                    const outPath = path.join(outDir, `${parsed.name}.d.ts`);
                    const absWithExt = path.resolve(fullPath).replace(/\\/g, '/');
                    const absNoExt = absWithExt.replace(/\\.(tsx|ts)$/, '');
                    const relNoExt = path
                        .relative(projectRoot, fullPath)
                        .replace(/\\/g, '/')
                        .replace(/\\.(tsx|ts)$/, '');
                    const relWithDot = `./${relNoExt}`;
                    const localRelative = `./${parsed.name}`;
                    const localRelativeExt = `./${parsed.name}${parsed.ext}`;
                    const moduleNames = new Set([absWithExt, absNoExt, relNoExt, relWithDot, localRelative, localRelativeExt]);

                    // Write plain declarations without module wrappers
                    await fs.writeFile(outPath, decls, 'utf-8');
                }
            } catch {
                // ignore expansion errors here; they will surface in later diagnostics
            }
        }
    }
}

async function getTsConfigPaths() {
    const allTsConfigs = (await fs.readdir(ROOT_DIR, { recursive: true }))
        .filter(file => file.endsWith('tsconfig.json') && !file.includes('node_modules'))
        .map(file => path.join(ROOT_DIR, file));

    const trackedTsConfigs = [];
    for (const configPath of allTsConfigs) {
        if (!(await isGitIgnored(configPath))) {
            trackedTsConfigs.push(configPath);
        }
    }
    
    // Filter out test-related tsconfig files
    const primaryTsConfigs = trackedTsConfigs.filter(configPath => 
        !configPath.includes('test/') && 
        (
            configPath.includes('packages/') || 
            configPath.includes('playground/')
        )
    );

    // Add root tsconfig if it exists and is not a test one
    const rootTsConfig = path.join(ROOT_DIR, 'tsconfig.json');
    if (await fs.stat(rootTsConfig).then(stat => stat.isFile()).catch(() => false) && !rootTsConfig.includes('test/')) {
        // Not all projects have a root tsconfig.json that's meant for building the whole thing.
        // For monorepos, typically it's configured via project references.
        // If it exists, we'll include it for a comprehensive check.
        if (!primaryTsConfigs.includes(rootTsConfig)) {
            // primaryTsConfigs.push(rootTsConfig); // Removed this as it can cause duplicate build attempts or issues in composite projects if root is not meant to be built directly.
        }
    }

    return primaryTsConfigs;
}

async function main() {
    if (WRITE_LOGS) {
        await fs.rm(LOGS_DIR, { recursive: true, force: true });
        await fs.mkdir(LOGS_DIR, { recursive: true });
        console.log(`Diagnostics logs will be saved in: ${LOGS_DIR}`);
    }

    const allErrors = [];

    // --- Biome Check ---
    console.log('\nRunning Biome check...');
    const biomeResult = await runCommand('npx biome check .');
    if (biomeResult.stdout || biomeResult.stderr) {
        console.log('Biome output detected. Parsing...');
        const biomeErrors = parseBiomeErrors(biomeResult.stdout + biomeResult.stderr);
        allErrors.push(...biomeErrors);
    } else {
        console.log('Biome check completed with no output.');
    }

    // --- Clippy Check (Rust) ---
    console.log('\nRunning Clippy check for Rust files...');
    const clippyResult = await runCommand('cargo clippy --workspace --all-targets --message-format=json 2>&1');
    if (clippyResult.stdout || clippyResult.stderr) {
        const clippyErrors = await parseClippyErrors(clippyResult.stdout + clippyResult.stderr);
        if (clippyErrors.length > 0) {
            console.log(`Clippy found ${clippyErrors.length} warnings/errors.`);
            allErrors.push(...clippyErrors);
        } else {
            console.log('Clippy check completed with no warnings.');
        }
    } else {
        console.log('Clippy check completed with no output.');
    }

    // --- TypeScript Type Check ---
    console.log('\nRunning TypeScript type checks...');
    const tsConfigPaths = await getTsConfigPaths();
    // Generate macro-driven type output into hidden folders before running tsc
    await generateMacroTypes(tsConfigPaths);

    for (const tsConfigPath of tsConfigPaths) {
        console.log(`  Checking project: ${path.relative(ROOT_DIR, tsConfigPath)}`);
        const tsResult = await runCommand(`npx tsc --noEmit -p ${tsConfigPath}`);
        if (tsResult.stdout || tsResult.stderr) {
            console.log('    TypeScript output detected. Parsing...');
            const tsErrors = await parseTypeScriptErrors(tsResult.stdout + tsResult.stderr);
            allErrors.push(...tsErrors);
        } else {
            console.log('    TypeScript check completed with no output.');
        }
    }

    // --- TypeScript Plugin Diagnostics (via Language Service) ---
    console.log('\nRunning TypeScript Language Service for plugin diagnostics...');
    for (const tsConfigPath of tsConfigPaths) {
        console.log(`  Checking plugins for: ${path.relative(ROOT_DIR, tsConfigPath)}`);
        const pluginErrors = await getLanguageServiceDiagnostics(tsConfigPath);
        if (pluginErrors.length > 0) {
            console.log(`    Found ${pluginErrors.length} plugin diagnostic(s).`);
            allErrors.push(...pluginErrors);
        }
    }

    // --- Svelte Check ---
    const svelteProjects = await getSvelteProjectPaths();
    if (svelteProjects.length > 0) {
        console.log('\nRunning Svelte checks...');
        for (const svelteDir of svelteProjects) {
            const relativePath = path.relative(ROOT_DIR, svelteDir);
            console.log(`  Checking Svelte project: ${relativePath}`);

            // Check if svelte-check is available
            const svelteCheckResult = await runCommand(
                `npx svelte-check --tsconfig ./tsconfig.json`,
                svelteDir
            );

            if (svelteCheckResult.stdout || svelteCheckResult.stderr) {
                const output = svelteCheckResult.stdout + svelteCheckResult.stderr;
                const svelteErrors = await parseSvelteCheckErrors(output);
                if (svelteErrors.length > 0) {
                    console.log(`    Found ${svelteErrors.length} Svelte diagnostic(s).`);
                    allErrors.push(...svelteErrors);
                } else {
                    console.log('    Svelte check completed with no issues.');
                }
            } else {
                console.log('    Svelte check completed with no output.');
            }
        }
    }

    // --- ts-macros Diagnostics ---
    console.log('\nRunning ts-macros expansion checks...');
    const macroModule = getTsMacrosModule();
    if (macroModule) {
        // Check playground directories and any other directories with @Derive usage
        const dirsToCheck = [
            path.join(ROOT_DIR, 'playground'),
            ...tsConfigPaths.map(p => path.dirname(p))
        ];
        const checkedDirs = new Set();

        for (const dir of dirsToCheck) {
            // Avoid checking the same directory twice
            const normalizedDir = path.resolve(dir);
            if (checkedDirs.has(normalizedDir)) continue;
            checkedDirs.add(normalizedDir);

            // Skip if directory doesn't exist
            try {
                await fs.access(dir);
            } catch {
                continue;
            }

            console.log(`  Checking macros in: ${path.relative(ROOT_DIR, dir)}`);
            const macroErrors = await getMacroDiagnostics(dir);
            if (macroErrors.length > 0) {
                console.log(`    Found ${macroErrors.length} macro diagnostic(s).`);
                allErrors.push(...macroErrors);
            } else {
                console.log('    No macro issues found.');
            }
        }
    } else {
        console.log('  Skipping (ts-macros module not available)');
    }

    // --- Deduplicate Errors ---
    // Some errors may be reported multiple times from different check passes
    const seenErrors = new Set();
    const deduplicatedErrors = allErrors.filter(error => {
        const key = `${error.file}:${error.line}:${error.column}:${error.code}:${error.message}`;
        if (seenErrors.has(key)) {
            return false;
        }
        seenErrors.add(key);
        return true;
    });

    // --- Categorize and Log Errors ---
    const categorizedErrors = new Map();
    for (const error of deduplicatedErrors) {
        const category = error.code;
        if (!categorizedErrors.has(category)) {
            categorizedErrors.set(category, []);
        }
        categorizedErrors.get(category).push(error.raw);
    }

    console.log('\n--- Diagnostics Summary ---');
    let totalErrors = 0;
    for (const [category, errors] of categorizedErrors.entries()) {
        if (WRITE_LOGS) {
            const logFileName = path.join(LOGS_DIR, `${category}.log`);
            await fs.writeFile(logFileName, errors.join('\n') + '\n');
            console.log(`  ${category}: ${errors.length} errors -> ${logFileName}`);
        } else {
            console.log(`  ${category}: ${errors.length} errors`);
        }
        totalErrors += errors.length;
    }

    // Print all errors to console when not writing logs
    if (!WRITE_LOGS && totalErrors > 0) {
        console.log('\n--- All Diagnostics ---\n');
        for (const error of deduplicatedErrors) {
            console.log(error.raw);
            console.log('');
        }
    }

    if (totalErrors > 0) {
        console.error(`\nDiagnostics completed with ${totalErrors} errors/warnings.`);
        process.exit(1);
    } else {
        console.log('\nDiagnostics completed with no errors or warnings.');
        process.exit(0);
    }
}

main().catch(console.error);
