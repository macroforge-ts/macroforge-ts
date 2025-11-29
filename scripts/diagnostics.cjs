const { exec } = require('node:child_process');
const { promises: fs } = require('node:fs');
const path = require('node:path');

const ROOT_DIR = process.cwd();
const LOGS_DIR = path.join(ROOT_DIR, 'diagnostics_logs');

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

    for (const tsConfigPath of tsConfigPaths) {
        console.log(`  Checking project: ${path.relative(ROOT_DIR, tsConfigPath)}`);
        const tsResult = await runCommand(`npx tsc --noEmit -p ${tsConfigPath}`);
        if (tsResult.stdout || tsResult.stderr) {
            console.log('TypeScript output detected. Parsing...');
            const tsErrors = await parseTypeScriptErrors(tsResult.stdout + tsResult.stderr);
            allErrors.push(...tsErrors);
        } else {
            console.log('TypeScript check completed with no output.');
        }
    }

    // --- Categorize and Log Errors ---
    const categorizedErrors = new Map();
    for (const error of allErrors) {
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
        for (const error of allErrors) {
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
