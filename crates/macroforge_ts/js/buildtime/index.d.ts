/**
 * # Macroforge Buildtime Module
 *
 * Compile-time JavaScript evaluation — the Zig-comptime primitive for
 * TypeScript. Annotate a top-level `const` or `function` declaration with
 * `/** @buildtime *\/` and the macroforge build pass evaluates it in a
 * sandboxed JS context, serializes the result, and splices a plain TS
 * literal back into the module.
 *
 * ```ts
 * /** @buildtime *\/
 * const BUILT_AT = buildtime.time.iso();
 *
 * /** @buildtime *\/
 * const SCHEMA = buildtime.fs.readJson("./schema.json");
 *
 * /** @buildtime *\/
 * function generateValidators() {
 *   const schema = buildtime.fs.readJson("./schema.json");
 *   return schema.fields.map(f =>
 *     `export function is_${f.name}(v: unknown): v is ${f.type} {
 *        return typeof v === "${f.jsType}";
 *      }`
 *   ).join("\n");
 * }
 * ```
 *
 * ## Runtime behavior
 *
 * Every export from this module is a sentinel. Calling any of them at
 * runtime throws — they should have been resolved by the macroforge
 * build pass and no longer exist in the output. If you see the runtime
 * error, the build pass is not running on this file.
 *
 * @module macroforge/buildtime
 */
export interface BuildtimeFs {
    /** Read a file as UTF-8 text. Path is resolved relative to the file the
     *  @buildtime declaration lives in. Throws if the path is not in the
     *  `buildtime.capabilities.filesystem.read` allowlist. */
    readText(path: string): string;
    /** Read a file and parse its content as JSON. Same capability rules
     *  as `readText`. */
    readJson(path: string): unknown;
    /** Return whether the path exists on disk. Counts as a read for
     *  dependency tracking — if the file appears later, the cache
     *  invalidates. */
    exists(path: string): boolean;
    /** Return the names of the entries in a directory, sorted. Counts as
     *  a read. */
    listDir(path: string): string[];
}
export interface BuildtimeCrypto {
    /** SHA-256 of the input, lowercase hex. Pure — always allowed. */
    sha256(input: string): string;
    /** SHA-512 of the input, lowercase hex. Pure — always allowed. */
    sha512(input: string): string;
}
export interface BuildtimeTime {
    /** Current wall-clock time as an ISO 8601 string. Makes builds
     *  non-deterministic — prefer recording a fixed timestamp if
     *  determinism matters. */
    now(): string;
    /** Current wall-clock time in unix seconds. */
    unix(): number;
    /** Alias for {@link BuildtimeTime.now}. */
    iso(): string;
}
export interface BuildtimeFlags {
    /** True if the named flag was passed to the build (e.g. from a
     *  `--define` argument or from `config.buildtime.flags`). */
    has(flag: string): boolean;
    /** Value of the named flag, or undefined if not set. */
    get(flag: string): string | undefined;
}
export interface BuildtimeLocation {
    /** Absolute path of the source file the @buildtime declaration
     *  lives in. */
    readonly file: string;
    /** 1-based line number of the `/** @buildtime *\/` annotation. */
    readonly line: number;
    /** 1-based column number of the annotation. */
    readonly column: number;
}
export interface Buildtime {
    readonly fs: BuildtimeFs;
    readonly crypto: BuildtimeCrypto;
    readonly time: BuildtimeTime;
    /** Environment variables the build was allowed to read. Populated
     *  from `buildtime.capabilities.env` in macroforge.config.js. Reads
     *  of variables not in the allowlist return `undefined`. */
    readonly env: Record<string, string | undefined>;
    readonly flags: BuildtimeFlags;
    readonly location: BuildtimeLocation;
}
/**
 * The compile-time API. Inside a `@buildtime` declaration, calls
 * against this object are routed to native implementations. At
 * runtime, every access throws — see module docs for why.
 */
export declare const buildtime: Buildtime;
/**
 * Re-export with the name `$buildtime` for users who prefer the macroforge
 * convention of prefixing compile-time identifiers with `$`. Points at the
 * same object.
 */
export declare const $buildtime: Buildtime;
