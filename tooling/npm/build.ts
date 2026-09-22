/**
 * Builds every package the monorepo publishes to npm with dnt, into
 * `npm/<name>/` at the repository root. That directory is exactly what
 * `npm publish` uploads and what the playground links against.
 *
 * Each package's `package.json` supplies its publish metadata and dependency
 * ranges; dnt supplies the module output, `exports`, `bin` and declarations.
 *
 * Usage: deno run -A tooling/npm/build.ts [name...]   (all packages by default)
 */

import { build, emptyDir, type EntryPoint, type PackageJson } from 'jsr:@deno/dnt@0.43.2';
import { dirname, fromFileUrl, join, relative, toFileUrl } from 'jsr:@std/path@^1';
import { copy } from 'jsr:@std/fs@^1';

const repoRoot = join(dirname(fromFileUrl(import.meta.url)), '..', '..');

interface NpmPackage {
    /** Repo-relative package directory. */
    dir: string;
    entryPoints: EntryPoint[];
    /** `'cjs'` for packages whose consumers `require` them. */
    moduleFormat: 'esm' | 'cjs';
    /** Repo-relative files or directories copied into the output, keyed by destination. */
    assets?: Record<string, string>;
}

/** In dependency order: a package builds after the workspace packages it links. */
const PACKAGES: Record<string, NpmPackage> = {
    core: {
        dir: 'crates/macroforge_ts',
        entryPoints: [
            { name: '.', path: 'pkg/macroforge_ts.js' },
            { name: './buildtime', path: 'js/buildtime/index.ts' },
            { name: './rules', path: 'js/rules/index.ts' },
            { name: './serde', path: 'js/serde/index.ts' },
            { name: './traits', path: 'js/traits/index.ts' }
        ],
        moduleFormat: 'esm',
        // The glue reads the module with `new URL(..., import.meta.url)`,
        // which is not an import, so it has to be placed beside the glue.
        assets: {
            'esm/pkg/macroforge_ts_bg.wasm': 'crates/macroforge_ts/pkg/macroforge_ts_bg.wasm'
        }
    },
    shared: {
        dir: 'packages/shared',
        entryPoints: [{ name: '.', path: 'src/index.ts' }],
        moduleFormat: 'esm'
    },
    'vite-plugin': {
        dir: 'packages/vite-plugin',
        entryPoints: [{ name: '.', path: 'src/index.js' }],
        moduleFormat: 'esm'
    },
    'svelte-preprocessor': {
        dir: 'packages/svelte-preprocessor',
        entryPoints: [{ name: '.', path: 'src/index.ts' }],
        moduleFormat: 'esm'
    },
    'typescript-plugin': {
        dir: 'packages/typescript-plugin',
        entryPoints: [{ name: '.', path: 'src/index.ts' }],
        moduleFormat: 'esm'
    },
    'mcp-server': {
        dir: 'packages/mcp-server',
        entryPoints: [
            { name: '.', path: 'src/index.ts' },
            { kind: 'bin', name: 'macroforge-mcp', path: 'src/index.ts' }
        ],
        moduleFormat: 'esm',
        // The docs loader reads `docs/` two levels above its module.
        assets: { docs: 'packages/mcp-server/docs' }
    },
    'svelte-language-server': {
        dir: 'packages/svelte-language-server',
        entryPoints: [
            { name: '.', path: 'src/index.ts' },
            { kind: 'bin', name: 'svelteserver', path: 'src/server-bin.ts' }
        ],
        moduleFormat: 'cjs'
    }
};

/** The `package.json` fields that describe a published package. */
type PackageManifest =
    & Pick<
        PackageJson,
        | 'name'
        | 'description'
        | 'license'
        | 'author'
        | 'homepage'
        | 'repository'
        | 'bugs'
        | 'keywords'
        | 'engines'
        | 'dependencies'
        | 'peerDependencies'
        | 'devDependencies'
    >
    & { peerDependenciesMeta?: Record<string, { optional?: boolean }> };

interface DenoManifest {
    version: string;
}

/** `@macroforge/shared` -> `shared`: the output directory of a workspace package. */
function shortName(packageName: string): string {
    return packageName.replace(/^@macroforge\//, '');
}

interface WorkspaceManifest {
    workspace: string[];
}

interface MemberManifest {
    name?: string;
}

interface ModuleGraph {
    modules: { specifier: string }[];
}

/** Names of the workspace packages reachable from the entry points' module graph. */
async function importedWorkspacePackages(
    packageDir: string,
    entryPoints: EntryPoint[]
): Promise<Set<string>> {
    const { workspace } = await readJson<WorkspaceManifest>(join(repoRoot, 'deno.json'));
    const members = new Map<string, string>();
    for (const member of workspace) {
        const memberDir = join(repoRoot, member);
        const { name } = await readJson<MemberManifest>(join(memberDir, 'deno.json'));
        if (name) members.set(`${toFileUrl(memberDir).href}/`, name);
    }

    const imported = new Set<string>();
    for (const entry of entryPoints) {
        const path = join(packageDir, typeof entry === 'string' ? entry : entry.path);
        const output = await new Deno.Command('deno', {
            args: ['info', '--json', '--config', join(packageDir, 'deno.json'), path]
        }).output();
        if (!output.success) {
            throw new Error(
                `deno info failed for ${path}:\n${new TextDecoder().decode(output.stderr)}`
            );
        }
        const graph: ModuleGraph = JSON.parse(new TextDecoder().decode(output.stdout));
        for (const { specifier } of graph.modules) {
            for (const [memberUrl, name] of members) {
                if (
                    specifier.startsWith(memberUrl) &&
                    !specifier.startsWith(toFileUrl(packageDir).href)
                ) {
                    imported.add(name);
                }
            }
        }
    }
    return imported;
}

interface ExportConditions {
    types?: string;
    import?: string;
    require?: string;
    default?: string;
}

interface GeneratedManifest {
    main?: string;
    types?: string;
    bin?: Record<string, string>;
    exports?: Record<string, ExportConditions>;
    dependencies?: Record<string, string>;
    scripts?: Record<string, string>;
}

/**
 * Settles what dnt's generated manifest cannot know from the module graph:
 * - every ESM entry gets `types` (first) and `default`, and the root entry a
 *   top-level `main` and `types`, so every TypeScript resolution mode finds
 *   the declarations and `require()` reaches the module via `require(esm)`;
 * - a package the source manifest declares as a peer or a dev dependency is
 *   never also a regular dependency, even when a type-only import made dnt
 *   add it as one;
 * - `bin` paths carry no leading `./`, which `npm publish` rejects and
 *   silently drops the command over.
 */
async function finishManifest(path: string, source: PackageManifest): Promise<void> {
    const generated = await readJson<GeneratedManifest>(path);
    const exportMap = generated.exports ?? {};
    for (const [subpath, conditions] of Object.entries(exportMap)) {
        if (!conditions.import) continue;
        // `types` must come first for TypeScript to honour it.
        const declarations = conditions.import.replace(/\.js$/, '.d.ts');
        exportMap[subpath] = {
            types: declarations,
            import: conditions.import,
            default: conditions.default ?? conditions.import
        };
        if (subpath === '.') {
            generated.main = conditions.import;
            generated.types = declarations;
        }
    }
    for (const peer of Object.keys(source.peerDependencies ?? {})) {
        delete generated.dependencies?.[peer];
    }
    for (const devOnly of Object.keys(source.devDependencies ?? {})) {
        if (!source.dependencies?.[devOnly]) {
            delete generated.dependencies?.[devOnly];
        }
    }
    if (generated.scripts && Object.keys(generated.scripts).length === 0) {
        delete generated.scripts;
    }
    if (generated.bin) {
        generated.bin = Object.fromEntries(
            Object.entries(generated.bin).map((
                [command, target]
            ) => [command, target.replace(/^\.\//, '')])
        );
    }
    await Deno.writeTextFile(path, `${JSON.stringify(generated, null, 2)}\n`);
}

async function readJson<T>(path: string): Promise<T> {
    return JSON.parse(await Deno.readTextFile(path));
}

async function buildPackage(name: string, spec: NpmPackage): Promise<void> {
    const packageDir = join(repoRoot, spec.dir);
    const manifest = await readJson<PackageManifest>(join(packageDir, 'package.json'));
    const { version } = await readJson<DenoManifest>(join(packageDir, 'deno.json'));
    const outDir = join(repoRoot, 'npm', name);

    // Workspace packages are published separately: map the ones the module
    // graph imports to their npm name and exact version, so dnt doesn't
    // inline their sources. One loaded at runtime (`createRequire`) is not in
    // the graph and stays a plain dependency.
    const workspaceDependencies = Object.entries(manifest.dependencies ?? {})
        .filter(([dependency]) => dependency.startsWith('@macroforge/'));
    const imported = await importedWorkspacePackages(packageDir, spec.entryPoints);
    const mappings = Object.fromEntries(
        workspaceDependencies
            .filter(([dependency]) => imported.has(dependency))
            .map(([dependency, range]) => [dependency, { name: dependency, version: range }])
    );

    console.log(`\n=== ${manifest.name}@${version} -> ${relative(repoRoot, outDir)}`);
    await emptyDir(outDir);
    // dnt installs the output's dependencies to type-check it. The workspace
    // ones may not be on the registry yet, so link their built outputs, the
    // way a consumer's install resolves them.
    const links = workspaceDependencies.map(([dependency]) => `../${shortName(dependency)}`);
    await Deno.writeTextFile(join(outDir, 'deno.json'), JSON.stringify({ links }, null, 4));

    await build({
        entryPoints: spec.entryPoints.map((entry) =>
            typeof entry === 'string'
                ? join(packageDir, entry)
                : { ...entry, path: join(packageDir, entry.path) }
        ),
        outDir,
        configFile: join(packageDir, 'deno.json'),
        scriptModule: spec.moduleFormat === 'cjs' ? 'cjs' : false,
        esModule: spec.moduleFormat === 'esm',
        declaration: 'inline',
        // The Deno test suites run against the sources; the output is proven
        // by the playground, which installs it the way a consumer does.
        test: false,
        packageManager: 'deno',
        shims: {},
        polyfills: spec.moduleFormat === 'esm' ? { importMeta: false } : {},
        compilerOptions: { target: 'ES2023', lib: ['ES2023'] },
        mappings,
        package: {
            name: manifest.name,
            version,
            description: manifest.description,
            license: manifest.license,
            author: manifest.author,
            homepage: manifest.homepage,
            repository: manifest.repository,
            bugs: manifest.bugs,
            keywords: manifest.keywords,
            engines: manifest.engines,
            dependencies: manifest.dependencies,
            peerDependencies: manifest.peerDependencies,
            peerDependenciesMeta: manifest.peerDependenciesMeta,
            devDependencies: manifest.devDependencies
        },
        async postBuild() {
            for (const scaffolding of ['deno.json', 'deno.lock', 'node_modules']) {
                await Deno.remove(join(outDir, scaffolding), { recursive: true });
            }
            await Deno.copyFile(join(repoRoot, 'LICENSE'), join(outDir, 'LICENSE'));
            await Deno.copyFile(join(packageDir, 'README.md'), join(outDir, 'README.md'));
            for (const [destination, source] of Object.entries(spec.assets ?? {})) {
                await copy(join(repoRoot, source), join(outDir, destination), { overwrite: true });
            }
            await finishManifest(join(outDir, 'package.json'), manifest);
        }
    });
}

const requested = Deno.args.length > 0 ? Deno.args : Object.keys(PACKAGES);
for (const name of requested) {
    const spec = PACKAGES[name];
    if (!spec) {
        throw new Error(
            `unknown npm package "${name}"; known: ${Object.keys(PACKAGES).join(', ')}`
        );
    }
    await buildPackage(name, spec);
}
