/**
 * @module @macroforge/deno-plugin/project
 *
 * Project-wide expansion: walk a directory tree, expand every `.ts`/`.tsx`
 * file containing macro annotations, and write the output to a mirror
 * directory. This is the workflow that lets you run a Deno app without
 * Vite — point Deno at the mirror via an `imports` entry in `deno.json`.
 */

import * as path from "@std/path";
import { ensureDir, walk } from "@std/fs";
import { hasMacroAnnotations } from "@macroforge/shared";
import { expand, type ExpandOptions } from "./index.ts";

/**
 * Cheap pre-filter to skip files the engine would no-op on. `hasMacroAnnotations`
 * covers JSDoc annotations (`@derive`, `@cfg`, `@deprecated`, `@mustUse`,
 * `@nonExhaustive`); `import macro` covers external macro packages; `$letter(`
 * is a loose match for call macros (`$state`, `$derived`, etc). The check has
 * false positives — anything matching gets handed to the engine, which is
 * the correct fallback.
 */
function mayContainMacros(source: string): boolean {
  if (hasMacroAnnotations(source)) return true;
  if (source.includes("import macro")) return true;
  return /\$[A-Za-z_][\w$]*\s*\(/.test(source);
}

export interface ExpandProjectOptions extends ExpandOptions {
  /** Project root that's walked. Defaults to `Deno.cwd()`. */
  root?: string;
  /** Output directory (relative to `root`). Defaults to `.macroforge/cache`. */
  outDir?: string;
  /** Glob-like extensions to include. Defaults to `['.ts', '.tsx']`. */
  extensions?: string[];
  /** Subpath substrings to skip. Defaults to common build/cache dirs. */
  exclude?: string[];
  /** When true, copy non-macro files through to the mirror unchanged so the mirror is self-contained. */
  copyPassthrough?: boolean;
  /** Called for each processed file; useful for CLI progress output. */
  onFile?: (event: ExpandFileEvent) => void;
}

export interface ExpandFileEvent {
  /** Absolute source path. */
  source: string;
  /** Absolute destination path in the mirror. */
  dest: string;
  /** True when expansion produced different output than the input. */
  expanded: boolean;
  /** True when the file was skipped because no macro annotations were found. */
  skipped: boolean;
  /** Diagnostics emitted by the engine (forwarded for logging). */
  diagnostics: Array<{ level: string; message: string }>;
}

const DEFAULT_EXCLUDES = [
  "/node_modules/",
  "/.macroforge/",
  "/target/",
  "/dist/",
  "/build/",
  "/.git/",
];

const DEFAULT_EXTENSIONS = [".ts", ".tsx"];

/**
 * Expand every macro-bearing file under `root` and write the output to
 * `<root>/<outDir>/`. Returns the per-file events so callers can summarize.
 */
export async function expandProject(
  options: ExpandProjectOptions = {},
): Promise<ExpandFileEvent[]> {
  const root = path.resolve(options.root ?? Deno.cwd());
  const outDir = path.resolve(root, options.outDir ?? ".macroforge/cache");
  const extensions = options.extensions ?? DEFAULT_EXTENSIONS;
  const exclude = [...DEFAULT_EXCLUDES, ...(options.exclude ?? [])];
  const copyPassthrough = options.copyPassthrough ?? false;
  const onFile = options.onFile;

  const events: ExpandFileEvent[] = [];

  for await (
    const entry of walk(root, { includeDirs: false, exts: extensions })
  ) {
    const source = entry.path;
    // The mirror is itself under `root`; never recurse into it.
    if (source.startsWith(outDir + path.SEPARATOR) || source === outDir) {
      continue;
    }
    if (exclude.some((segment) => source.includes(segment))) continue;

    const rel = path.relative(root, source);
    const dest = path.join(outDir, rel);

    const code = await Deno.readTextFile(source);

    // Cheap pre-filter: if the file has no macro annotations, skip the
    // engine entirely. `hasMacroAnnotations` matches `@derive`, `@attr`
    // attribute macros, and `import macro` comments.
    if (!mayContainMacros(code)) {
      if (copyPassthrough) {
        await ensureDir(path.dirname(dest));
        await Deno.writeTextFile(dest, code);
      }
      const event: ExpandFileEvent = {
        source,
        dest,
        expanded: false,
        skipped: true,
        diagnostics: [],
      };
      events.push(event);
      onFile?.(event);
      continue;
    }

    const result = expand(code, source, {
      ...options,
      projectRoot: root,
    });

    await ensureDir(path.dirname(dest));
    await Deno.writeTextFile(dest, result.code);

    const event: ExpandFileEvent = {
      source,
      dest,
      expanded: result.hasMacros,
      skipped: false,
      diagnostics: result.diagnostics,
    };
    events.push(event);
    onFile?.(event);
  }

  return events;
}

/**
 * Watch the project and re-expand any changed file. Returns a function to
 * stop watching. The initial `expandProject` call is awaited before the
 * watcher starts so the mirror is always populated before file events fire.
 */
export async function watchProject(
  options: ExpandProjectOptions = {},
): Promise<() => void> {
  const root = path.resolve(options.root ?? Deno.cwd());
  const outDir = path.resolve(root, options.outDir ?? ".macroforge/cache");
  const extensions = options.extensions ?? DEFAULT_EXTENSIONS;
  const exclude = [...DEFAULT_EXCLUDES, ...(options.exclude ?? [])];

  await expandProject(options);

  const watcher = Deno.watchFs(root, { recursive: true });

  (async () => {
    for await (const event of watcher) {
      if (event.kind !== "modify" && event.kind !== "create") continue;
      for (const changed of event.paths) {
        if (changed.startsWith(outDir + path.SEPARATOR) || changed === outDir) {
          continue;
        }
        if (!extensions.some((ext) => changed.endsWith(ext))) continue;
        if (exclude.some((segment) => changed.includes(segment))) continue;

        try {
          const code = await Deno.readTextFile(changed);
          const rel = path.relative(root, changed);
          const dest = path.join(outDir, rel);

          if (!mayContainMacros(code)) {
            if (options.copyPassthrough) {
              await ensureDir(path.dirname(dest));
              await Deno.writeTextFile(dest, code);
            }
            continue;
          }

          const result = expand(code, changed, {
            ...options,
            projectRoot: root,
          });
          await ensureDir(path.dirname(dest));
          await Deno.writeTextFile(dest, result.code);

          options.onFile?.({
            source: changed,
            dest,
            expanded: result.hasMacros,
            skipped: false,
            diagnostics: result.diagnostics,
          });
        } catch (error) {
          console.error(
            `[@macroforge/deno-plugin] watch: failed to expand ${changed}:`,
            error,
          );
        }
      }
    }
  })();

  return () => watcher.close();
}
