/**
 * svelte-package wrapper — spawned by the CLI's `run_svelte_package_wrapper`.
 *
 * Macro expansion has already happened by the time this runs. The CLI expands
 * the changed `.ts`/`.svelte.ts` modules in parallel, up front, into a tree
 * under `.macroforge/svelte-package/expanded/`; this script's only job is to
 * make @sveltejs/package read that tree instead of the raw sources, so the
 * published package ships the generated derive runtime and correct `.d.ts`.
 *
 * Two read paths are covered, both routed through `globalThis.__macroforgeExpand`:
 *   - JS emit: svelte-package reads source via `import * as fs from 'node:fs'`.
 *     A module resolve hook (svelte-package-fs-hook.mjs) redirects `node:fs`
 *     to a shim whose `readFileSync` calls us.
 *   - .d.ts emit: svelte2tsx reads `.ts` sources through `ts.sys.readFile`,
 *     which we monkeypatch directly (a writable property on the ts.sys object).
 *
 * The packager still runs against the *real* input directory. Redirecting the
 * bytes rather than the path keeps `$lib` alias resolution and `.d.ts.map`
 * sources — both computed relative to the input dir — pointing at the actual
 * sources, which packaging from a scratch directory would silently break.
 *
 * Arguments (forwarded to svelte-package): --input, --output, --tsconfig, --no-types
 * Environment:
 *   MACROFORGE_PACKAGE_INPUT     absolute input directory
 *   MACROFORGE_PACKAGE_EXPANDED  absolute expanded tree
 *   MACROFORGE_PACKAGE_ENTRIES   JSON array of expanded paths, relative to input
 *
 * Also supports `--macroforge-resolve-config`, which reports the options the
 * packager would run with and exits. `input` and `extensions` come from
 * `svelte.config.js`, so only Node can answer them, and the CLI needs both
 * before it can decide whether a build is necessary at all.
 */
import { register, createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
import path from 'node:path';

const cwdRequire = createRequire(process.cwd() + '/package.json');
const fsReal = cwdRequire('fs');

/** Resolve a file inside the installed @sveltejs/package, whose exports map hides its internals. */
function sveltePackageFile(relative) {
    const pkgJsonPath = cwdRequire.resolve('@sveltejs/package/package.json');
    return path.join(path.dirname(pkgJsonPath), relative);
}

function missingSveltePackage() {
    console.error('[macroforge] error: @sveltejs/package is not installed in this project');
    console.error('[macroforge] install it with: npm install --save-dev @sveltejs/package');
    process.exit(1);
}

const argv = process.argv.slice(2);

// --- Config probe -----------------------------------------------------------
// Answers only what requires evaluating svelte.config.js. Everything else the
// CLI already knows, so keeping this narrow keeps it cheap: the CLI caches the
// answer and re-runs the probe only when its inputs change.
if (argv.includes('--macroforge-resolve-config')) {
    const flag = (name) => {
        const index = argv.indexOf(name);
        return index !== -1 && index + 1 < argv.length ? argv[index + 1] : undefined;
    };

    let configFile;
    try {
        configFile = sveltePackageFile('src/config.js');
    } catch {
        missingSveltePackage();
    }

    const { load_config } = await import(pathToFileURL(configFile).href);
    const config = await load_config();

    // Same precedence as svelte-package's own CLI (src/cli.js).
    const input = flag('--input') ?? config.kit?.files?.lib ?? 'src/lib';

    process.stdout.write(
        JSON.stringify({
            input: path.resolve(process.cwd(), input),
            extensions: config.extensions ?? ['.svelte']
        })
    );
    process.exit(0);
}

// --- Read redirection -------------------------------------------------------
let ts;
try {
    ts = cwdRequire('typescript');
} catch {
    console.error('[macroforge] error: typescript is not installed in this project');
    process.exit(1);
}

const inputDir = process.env.MACROFORGE_PACKAGE_INPUT;
const expandedDir = process.env.MACROFORGE_PACKAGE_EXPANDED;

/** Paths (relative to the input dir) that have an expanded artifact. */
const entries = new Set();
if (process.env.MACROFORGE_PACKAGE_ENTRIES) {
    try {
        const listed = JSON.parse(
            fsReal.readFileSync(process.env.MACROFORGE_PACKAGE_ENTRIES, 'utf8')
        );
        for (const entry of listed) entries.add(entry);
    } catch (e) {
        console.error(`[macroforge] error: could not read the expanded file list: ${e.message}`);
        process.exit(1);
    }
}

/** The path relative to the input directory, or null if it lies outside it. */
function relativeToInput(filePath) {
    if (typeof filePath !== 'string' || !inputDir) return null;
    const rel = path.relative(inputDir, path.resolve(filePath));
    if (!rel || rel.startsWith('..') || path.isAbsolute(rel)) return null;
    return rel.split(path.sep).join('/');
}

/** Entries actually served, so a silent path mismatch can be caught at exit. */
const served = new Set();

/**
 * Serves an expanded module in place of its source.
 *
 * A listed entry that cannot be read is fatal rather than a fall-through to the
 * raw source: passing the unexpanded module through is what publishes a library
 * whose generated runtime is missing, and that failure is invisible until
 * someone imports the package.
 */
globalThis.__macroforgeExpand = function (filePath, content) {
    const rel = relativeToInput(filePath);
    if (rel === null || !entries.has(rel)) return content;

    const expandedPath = path.join(expandedDir, rel);
    let expanded;
    try {
        expanded = fsReal.readFileSync(expandedPath, 'utf8');
    } catch (e) {
        throw new Error(
            `[macroforge] expanded source for ${rel} is missing or unreadable ` +
                `(${expandedPath}): ${e.message}. Re-run with --full-rebuild.`
        );
    }
    served.add(rel);
    return expanded;
};

// Every `.ts` module the CLI expanded is read at least once by the packager, so
// one that was never served means the redirect never matched its path — and the
// package being written right now contains its raw source. Failing here is what
// keeps that from being discovered by whoever installs the published library.
// `.tsx` is excluded: the packager copies those with `copyFileSync`, which never
// reaches a read hook at all.
process.on('exit', () => {
    const required = [...entries].filter((rel) => rel.endsWith('.ts'));
    const missed = required.filter((rel) => !served.has(rel));
    if (missed.length === 0) return;

    console.error(
        `[macroforge] error: ${missed.length} expanded module(s) were never read by ` +
            `the packager, so the output would ship unexpanded source:`
    );
    for (const rel of missed) console.error(`  ${rel}`);
    console.error(`[macroforge] input directory: ${inputDir}`);
    process.exitCode = process.exitCode || 1;
});

// .d.ts side: svelte2tsx's emitDts reads .ts sources through ts.sys.readFile.
const origTsRead = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
    const content = origTsRead(filePath, encoding);
    if (content == null) return content;
    return globalThis.__macroforgeExpand(filePath, content);
};

// JS side: redirect node:fs to the shim before svelte-package loads.
register('./svelte-package-fs-hook.mjs', import.meta.url);

// Hand off to svelte-package's own CLI (reuses its arg parsing / build pipeline).
process.argv = [process.argv[0], 'svelte-package', ...argv];

let binPath;
try {
    binPath = sveltePackageFile('svelte-package.js');
} catch {
    missingSveltePackage();
}

await import(pathToFileURL(binPath).href);
