/**
 * @module @macroforge/deno-plugin/cli
 *
 * Command-line entry. Three subcommands:
 *
 *   expand   one-shot expansion of every macro-bearing file under --root
 *   watch    same as expand, then re-runs on file changes
 *   clean    remove the mirror output directory
 *
 * Run with `deno run -A jsr:@macroforge/deno-plugin/cli <subcommand>`.
 */

import { parseArgs } from "@std/cli/parse-args";
import * as path from "@std/path";
import {
  type ExpandFileEvent,
  expandProject,
  watchProject,
} from "./project.ts";

interface CliOptions {
  root: string;
  out: string;
  copyPassthrough: boolean;
  buildMode: "dev" | "prod";
}

function printEvent(event: ExpandFileEvent, root: string) {
  const rel = path.relative(root, event.source);
  if (event.skipped) return;
  if (event.expanded) {
    console.log(`  expanded  ${rel}`);
  } else {
    console.log(`  unchanged ${rel}`);
  }
  for (const diag of event.diagnostics) {
    console.warn(`    ${diag.level}: ${diag.message}`);
  }
}

function parseCliOptions(args: ReturnType<typeof parseArgs>): CliOptions {
  const buildMode = (args["build-mode"] as string | undefined) ?? "prod";
  if (buildMode !== "dev" && buildMode !== "prod") {
    console.error(`--build-mode must be 'dev' or 'prod' (got '${buildMode}')`);
    Deno.exit(2);
  }
  return {
    root: path.resolve((args.root as string | undefined) ?? Deno.cwd()),
    out: (args.out as string | undefined) ?? ".macroforge/cache",
    copyPassthrough: Boolean(args["copy-passthrough"]),
    buildMode,
  };
}

async function runExpand(opts: CliOptions) {
  console.log(
    `[@macroforge/deno-plugin] expand: root=${opts.root} out=${opts.out}`,
  );
  const start = performance.now();
  const events = await expandProject({
    root: opts.root,
    outDir: opts.out,
    copyPassthrough: opts.copyPassthrough,
    buildMode: opts.buildMode,
    onFile: (event) => printEvent(event, opts.root),
  });
  const expanded = events.filter((event) => event.expanded).length;
  const skipped = events.filter((event) => event.skipped).length;
  const elapsed = (performance.now() - start).toFixed(0);
  console.log(
    `[@macroforge/deno-plugin] done in ${elapsed}ms — ${expanded} expanded, ${
      events.length - expanded - skipped
    } unchanged, ${skipped} skipped`,
  );
}

async function runWatch(opts: CliOptions) {
  console.log(
    `[@macroforge/deno-plugin] watch: root=${opts.root} out=${opts.out}`,
  );
  const stop = await watchProject({
    root: opts.root,
    outDir: opts.out,
    copyPassthrough: opts.copyPassthrough,
    buildMode: opts.buildMode,
    onFile: (event) => printEvent(event, opts.root),
  });
  Deno.addSignalListener("SIGINT", () => {
    stop();
    Deno.exit(0);
  });
  // Keep the process alive; the watcher loop is detached inside watchProject.
  await new Promise(() => {});
}

async function runClean(opts: CliOptions) {
  const target = path.resolve(opts.root, opts.out);
  try {
    await Deno.remove(target, { recursive: true });
    console.log(`[@macroforge/deno-plugin] removed ${target}`);
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) return;
    throw error;
  }
}

function printHelp() {
  console.log(
    `@macroforge/deno-plugin

Usage:
  deno run -A jsr:@macroforge/deno-plugin/cli <command> [options]

Commands:
  expand    Walk --root and write expanded files to --out
  watch     Run expand once, then re-expand on file changes
  clean     Delete the --out directory

Options:
  --root <dir>           Project root (default: cwd)
  --out <dir>            Mirror output dir relative to root (default: .macroforge/cache)
  --build-mode <mode>    'dev' or 'prod' (default: prod)
  --copy-passthrough     Copy non-macro files to the mirror unchanged
  --help                 Show this help`,
  );
}

async function main() {
  const args = parseArgs(Deno.args, {
    boolean: ["help", "copy-passthrough"],
    string: ["root", "out", "build-mode"],
  });

  if (args.help || args._.length === 0) {
    printHelp();
    Deno.exit(args.help ? 0 : 1);
  }

  const command = String(args._[0]);
  const opts = parseCliOptions(args);

  switch (command) {
    case "expand":
      await runExpand(opts);
      break;
    case "watch":
      await runWatch(opts);
      break;
    case "clean":
      await runClean(opts);
      break;
    default:
      console.error(`Unknown command: ${command}`);
      printHelp();
      Deno.exit(2);
  }
}

if (import.meta.main) {
  await main();
}
