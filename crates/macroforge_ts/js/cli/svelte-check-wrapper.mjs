/**
 * Svelte-check wrapper, spawned by the CLI's `run_svelte_check_wrapper`.
 *
 * Patches `ts.sys.readFile` to expand macros before svelte-check sees the files.
 * Since svelte-check uses TypeScript as a peer dependency and Node.js caches
 * modules, the patched `ts.sys.readFile` is shared with svelte-check's internal
 * TypeScript language service.
 *
 * svelte-check then reports positions in the expanded text, so its output is
 * rewritten line by line: positions in an expanded file move back onto the
 * file as written, in each of its output formats.
 *
 * Arguments:
 *   argv[2..]: forwarded to svelte-check CLI
 *
 * Environment:
 *   MACROFORGE_TYPE_REGISTRY_PATH: path to pre-built type registry JSON
 *   MACROFORGE_DECLARATIVE_REGISTRY_PATH: path to pre-built declarative macro registry JSON
 */
import path from "node:path";
import {
  createExpandOptions,
  expandSourceResult,
  isMacroSourceFile,
  loadProjectModule,
  mapSpanToSource,
} from "./wrapper-common.mjs";

const ts = loadProjectModule("typescript");
const macros = loadProjectModule("@macroforge/core");

const plugin = new macros.NativePlugin();
const expandOptions = createExpandOptions(macros);

/** Expanded files by absolute path: both texts and the mapping between them. */
const expandedFiles = new Map();

const originalReadFile = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
  const content = originalReadFile(filePath, encoding);
  if (content === undefined) return content;
  if (isMacroSourceFile(filePath)) {
    const result = expandSourceResult(plugin, filePath, content, expandOptions);
    if (result.sourceMapping) {
      expandedFiles.set(path.resolve(filePath), {
        original: content,
        expanded: result.code,
        mapping: result.sourceMapping,
      });
    }
    return result.code;
  }
  return content;
};

/** Offset of a 0-based line and character in `text`. */
function offsetOf(text, line, character) {
  let offset = 0;
  for (let current = 0; current < line; current += 1) {
    const newline = text.indexOf("\n", offset);
    if (newline === -1) return text.length;
    offset = newline + 1;
  }
  return Math.min(offset + character, text.length);
}

/** 0-based line and character of an offset in `text`. */
function lineAndCharacterOf(text, offset) {
  const before = text.slice(0, offset);
  const line = before.split("\n").length - 1;
  return { line, character: offset - (before.lastIndexOf("\n") + 1) };
}

/** Moves a 0-based position in `file`'s expansion onto its source. */
function toSource(file, line, character) {
  const offset = offsetOf(file.expanded, line, character);
  const span = mapSpanToSource(macros, file.mapping, offset, 0);
  return lineAndCharacterOf(file.original, span.start);
}

const workspaceIndex = process.argv.indexOf("--workspace");
const workspace = path.resolve(
  workspaceIndex === -1 ? process.cwd() : process.argv[workspaceIndex + 1],
);
const expandedFile = (name) => expandedFiles.get(path.resolve(workspace, name));

const HUMAN = /^(.+):(\d+):(\d+)$/;
// Human output colours the file name, so it is matched without the colour codes.
const COLOUR = `${String.fromCharCode(27)}\\[[0-9;]*m`;
const ANSI = new RegExp(COLOUR, "g");
const TRAILING_POSITION = new RegExp(`:\\d+:\\d+((?:${COLOUR})*)$`);
const MACHINE = /^(\d+ (?:ERROR|WARNING) ")([^"]+)(" )(\d+):(\d+)( .*)$/;
const MACHINE_VERBOSE = /^(\d+ )(\{.*\})$/;

/** One line of svelte-check output with its positions moved onto the source. */
function rewriteLine(line) {
  const human = HUMAN.exec(line.replace(ANSI, ""));
  if (human) {
    const file = expandedFile(human[1]);
    if (!file) return line;
    const moved = toSource(file, Number(human[2]) - 1, Number(human[3]) - 1);
    return line.replace(
      TRAILING_POSITION,
      (_position, colours) => `:${moved.line + 1}:${moved.character + 1}${colours}`,
    );
  }
  const machine = MACHINE.exec(line);
  if (machine) {
    const file = expandedFile(machine[2]);
    if (!file) return line;
    const moved = toSource(file, Number(machine[4]) - 1, Number(machine[5]) - 1);
    return `${machine[1]}${machine[2]}${machine[3]}${moved.line + 1}:${moved.character + 1}${machine[6]}`;
  }
  const verbose = MACHINE_VERBOSE.exec(line);
  if (verbose) {
    const entry = JSON.parse(verbose[2]);
    const file = entry.filename && expandedFile(entry.filename);
    if (!file || !entry.start) return line;
    entry.start = toSource(file, entry.start.line, entry.start.character);
    if (entry.end) entry.end = toSource(file, entry.end.line, entry.end.character);
    return `${verbose[1]}${JSON.stringify(entry)}`;
  }
  return line;
}

/** Routes a stream's writes through `rewriteLine`, a whole line at a time. */
function rewriteStream(stream) {
  const write = stream.write.bind(stream);
  let pending = "";
  stream.write = (chunk, ...rest) => {
    pending += typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8");
    const lines = pending.split("\n");
    pending = lines.pop() ?? "";
    if (lines.length === 0) return true;
    return write(lines.map((line) => `${rewriteLine(line)}\n`).join(""), ...rest);
  };
  process.on("exit", () => {
    if (pending !== "") write(rewriteLine(pending));
  });
}
rewriteStream(process.stdout);
rewriteStream(process.stderr);

process.argv = [process.argv[0], "svelte-check", ...process.argv.slice(2)];
loadProjectModule("svelte-check");
