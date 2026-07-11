/**
 * svelte-package wrapper — spawned by the CLI's `run_svelte_package_wrapper`.
 *
 * Expands `@derive` macros in `.ts`/`.svelte.ts` type modules *while*
 * @sveltejs/package runs, so the published package ships the generated runtime
 * (and correct .d.ts) with no separate expand step or staging tree.
 *
 * Two read paths are covered, both fed by one `globalThis.__macroforgeExpand`:
 *   - JS emit: svelte-package reads source via `import * as fs from 'node:fs'`.
 *     A module resolve hook (svelte-package-fs-hook.mjs) redirects `node:fs`
 *     to a shim whose `readFileSync` expands macro modules.
 *   - .d.ts emit: svelte2tsx reads `.ts` sources through `ts.sys.readFile`,
 *     which we monkeypatch directly (a writable property on the ts.sys object).
 *
 * Arguments (forwarded to svelte-package): --input, --output, --tsconfig, --no-types
 * Environment: MACROFORGE_TYPE_REGISTRY_PATH, MACROFORGE_DECLARATIVE_REGISTRY_PATH
 */
import { register, createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
import path from 'node:path';

const cwdRequire = createRequire(process.cwd() + '/package.json');
const fsReal = cwdRequire('fs');

let ts;
let macros;
try {
    ts = cwdRequire('typescript');
} catch {
    console.error('[macroforge] error: typescript is not installed in this project');
    process.exit(1);
}
try {
    macros = cwdRequire('macroforge');
} catch {
    console.error('[macroforge] error: macroforge is not installed in this project');
    process.exit(1);
}

// External macro callbacks (WASM build cannot spawn Node to resolve macro packages).
if (macros.setupExternalMacros) {
    const req = createRequire(process.cwd() + '/package.json');
    const resolveDecoratorNames = (packagePath) => {
        try {
            const pkg = req(packagePath);
            const names = [];
            if (pkg.__macroforgeGetManifest) {
                names.push(...(pkg.__macroforgeGetManifest().decorators || []).map((d) => d.export));
            }
            for (const key of Object.keys(pkg)) {
                if (key.startsWith('__macroforgeGetManifest_') && typeof pkg[key] === 'function') {
                    names.push(...(pkg[key]().decorators || []).map((d) => d.export));
                }
            }
            if (names.length > 0) return [...new Set(names)];
        } catch {}
        return [];
    };
    const runMacro = (ctxJson) => {
        const ctx = JSON.parse(ctxJson);
        const fnName = `__macroforgeRun${ctx.macro_name}`;
        try {
            const pkg = req(ctx.module_path);
            const fn = pkg?.[fnName] || pkg?.default?.[fnName];
            if (typeof fn === 'function') return fn(ctxJson);
        } catch {}
        throw new Error(`Macro ${fnName} not found in ${ctx.module_path}`);
    };
    macros.setupExternalMacros(resolveDecoratorNames, runMacro);
}

// Discover macroforge.config.* (walking up to the nearest package.json).
const CONFIG_FILES = [
    'macroforge.config.ts',
    'macroforge.config.mts',
    'macroforge.config.js',
    'macroforge.config.mjs',
    'macroforge.config.cjs'
];
let macroConfigPath = null;
let currentDir = process.cwd();
while (true) {
    for (const filename of CONFIG_FILES) {
        const candidate = path.join(currentDir, filename);
        if (fsReal.existsSync(candidate)) {
            macroConfigPath = candidate;
            break;
        }
    }
    if (macroConfigPath) break;
    if (fsReal.existsSync(path.join(currentDir, 'package.json'))) break;
    const parent = path.dirname(currentDir);
    if (parent === currentDir) break;
    currentDir = parent;
}
if (macroConfigPath) {
    try {
        macros.loadConfig(fsReal.readFileSync(macroConfigPath, 'utf8'), macroConfigPath);
    } catch {}
}

function readEnvJson(envVar) {
    const p = process.env[envVar];
    if (!p) return undefined;
    try {
        return fsReal.readFileSync(p, 'utf8');
    } catch {
        return undefined;
    }
}

const plugin = new macros.NativePlugin();
const expandOpts = {};
if (macroConfigPath) expandOpts.configPath = macroConfigPath;
const typeRegistryJson = readEnvJson('MACROFORGE_TYPE_REGISTRY_PATH');
if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
const declarativeRegistryJson = readEnvJson('MACROFORGE_DECLARATIVE_REGISTRY_PATH');
if (declarativeRegistryJson) expandOpts.declarativeRegistryJson = declarativeRegistryJson;

function hasMacroMarkers(sourceText) {
    if (!sourceText) return false;
    return (
        sourceText.includes('@derive') ||
        sourceText.includes('macroforge/rules') ||
        sourceText.includes('import macro')
    );
}

// Shared expander used by both the fs shim (JS emit) and ts.sys.readFile (.d.ts).
globalThis.__macroforgeExpand = function (filePath, content) {
    try {
        if (
            typeof filePath === 'string' &&
            (filePath.endsWith('.ts') || filePath.endsWith('.tsx')) &&
            !filePath.endsWith('.d.ts') &&
            hasMacroMarkers(content)
        ) {
            const result = plugin.processFile(filePath, content, expandOpts);
            return result.code || content;
        }
    } catch {}
    return content;
};

// .d.ts side: svelte2tsx's emitDts reads .ts sources through ts.sys.readFile.
const origTsRead = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
    const content = origTsRead(filePath, encoding);
    if (content == null) return content;
    return globalThis.__macroforgeExpand(filePath, content);
};

// JS side: redirect node:fs to the expanding shim before svelte-package loads.
register('./svelte-package-fs-hook.mjs', import.meta.url);

// Hand off to svelte-package's own CLI (reuses its arg parsing / build pipeline).
const forwarded = process.argv.slice(2);
process.argv = [process.argv[0], 'svelte-package', ...forwarded];

let binPath;
try {
    const pkgJsonPath = cwdRequire.resolve('@sveltejs/package/package.json');
    binPath = path.join(path.dirname(pkgJsonPath), 'svelte-package.js');
} catch {
    console.error('[macroforge] error: @sveltejs/package is not installed in this project');
    console.error('[macroforge] install it with: npm install --save-dev @sveltejs/package');
    process.exit(1);
}

await import(pathToFileURL(binPath).href);
