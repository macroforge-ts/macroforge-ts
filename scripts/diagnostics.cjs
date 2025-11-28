const { exec } = require('node:child_process');
const { promises: fs } = require('node:fs');
const path = require('node:path');

const ROOT_DIR = process.cwd();
const LOGS_DIR = path.join(ROOT_DIR, 'diagnostics_logs');
const IGNORED_PATH_SEGMENTS = new Set(['node_modules']);
const IGNORED_PATH_PREFIXES = [
    path.join('playground', 'svelte', 'src', 'lib', 'form'),
    path.join('playground', 'svelte', 'src', 'lib', 'types'),
    path.join('playground', 'svelte', 'src', 'lib', 'traits'),
].map((prefix) => prefix.split(path.sep).join('/'));

async function runCommand(command, cwd = ROOT_DIR) {
    return new Promise((resolve) => {
        exec(command, { cwd }, (error, stdout, stderr) => {
            resolve({ error, stdout, stderr });
        });
    });
}

async function isGitIgnored(filePath) {
    return new Promise((resolve) => {
        exec(`git check-ignore "${filePath}"`, { cwd: ROOT_DIR }, (error) => {
            if (!error) {
                resolve(true);
            } else if (error && error.code === 1) {
                resolve(false);
            } else {
                resolve(false);
            }
        });
    });
}

function normalizeFilePath(filePath) {
    const absolute = path.isAbsolute(filePath) ? filePath : path.join(ROOT_DIR, filePath);
    const relative = path.relative(ROOT_DIR, absolute);
    return relative.replace(/\\/g, '/');
}

function isIgnoredDiagnosticPath(filePath) {
    const normalized = normalizeFilePath(filePath);
    if (!normalized || normalized.startsWith('..')) {
        return false;
    }
    if (IGNORED_PATH_PREFIXES.some((prefix) => normalized.startsWith(prefix))) {
        return true;
    }
    const segments = normalized.split('/');
    return segments.some((segment) => IGNORED_PATH_SEGMENTS.has(segment));
}

function parseTypeScriptErrors(output) {
    const errors = [];
    const errorRegex = /^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$/gm;
    let match;

    while ((match = errorRegex.exec(output)) !== null) {
        const filePath = match[1];
        if (isIgnoredDiagnosticPath(filePath)) {
            continue;
        }
        errors.push({
            file: filePath,
            line: parseInt(match[2], 10),
            column: parseInt(match[3], 10),
            code: match[4],
            message: match[5],
            raw: match[0],
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
    await fs.rm(LOGS_DIR, { recursive: true, force: true });
    await fs.mkdir(LOGS_DIR, { recursive: true });
    console.log(`Diagnostics logs will be saved in: ${LOGS_DIR}`);

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

    // --- TypeScript Type Check ---
    console.log('\nRunning TypeScript type checks...');
    const tsConfigPaths = await getTsConfigPaths();
    
    for (const tsConfigPath of tsConfigPaths) {
        console.log(`  Checking project: ${path.relative(ROOT_DIR, tsConfigPath)}`);
        const tsResult = await runCommand(`npx tsc --noEmit -p ${tsConfigPath}`);
        if (tsResult.stdout || tsResult.stderr) {
            console.log('TypeScript output detected. Parsing...');
            const tsErrors = parseTypeScriptErrors(tsResult.stdout + tsResult.stderr);
            allErrors.push(...tsErrors);
        } else {
            console.log('TypeScript check completed with no output.');
        }
    }

    // --- Categorize and Log Errors ---
    const categorizedErrors = new Map();
    for (const error of allErrors) {
        const category = error.tool === 'tsc' ? error.code : error.code; // Use rule for biome, TS code for tsc
        if (!categorizedErrors.has(category)) {
            categorizedErrors.set(category, []);
        }
        categorizedErrors.get(category).push(error.raw);
    }

    console.log('\nWriting categorized logs...');
    let totalErrors = 0;
    for (const [category, errors] of categorizedErrors.entries()) {
        const logFileName = path.join(LOGS_DIR, `${category}.log`);
        await fs.writeFile(logFileName, errors.join('\n') + '\n');
        console.log(`  ${category}: ${errors.length} errors -> ${logFileName}`);
        totalErrors += errors.length;
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
