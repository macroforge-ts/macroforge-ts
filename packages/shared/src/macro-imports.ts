/**
 * @module macro-imports
 *
 * Utilities for parsing macro import comments in TypeScript code.
 */

/**
 * JSDoc tag heads that macroforge recognises as annotations the engine must
 * see. Each entry is matched against line-starts after stripping JSDoc
 * comment syntax. `@derive(` and `@cfg(` must be parenthesised; the others
 * may appear bare or with args.
 */
const MACRO_TAG_PREFIXES = [
    '@derive(',
    '@cfg(',
    '@deprecated',
    '@mustUse',
    '@nonExhaustive'
] as const;

/**
 * Checks whether source code contains a macroforge JSDoc annotation —
 * `@derive(...)`, `@cfg(...)`, `@deprecated`, `@mustUse`, or `@nonExhaustive`.
 *
 * Only matches when the tag appears at the start of a JSDoc comment line
 * (after stripping comment syntax like `/**`, `*​/`, `*`, and whitespace).
 * This correctly rejects matches embedded in prose text such as
 * `"Deserialize result format from @derive(Deserialize)"`.
 *
 * Use this instead of `code.includes("@derive")` to avoid false positives.
 *
 * @param source - The TypeScript source code to scan
 * @returns `true` if the source contains any macroforge annotation
 *
 * @example
 * ```typescript
 * hasMacroAnnotations('/** @derive(Debug) *​/ class X {}');       // true
 * hasMacroAnnotations('/** @cfg({ feature: "ssr" }) *​/ fn f() {}'); // true
 * hasMacroAnnotations('/** @nonExhaustive *​/ type K = "a";');     // true
 * hasMacroAnnotations('/** result from @derive(Debug) *​/');       // false — embedded in prose
 * hasMacroAnnotations('class X {}');                              // false
 * ```
 */
export function hasMacroAnnotations(source: string): boolean {
    // Cheap bail-out: if none of the tag heads are textually present, no line
    // can start with one.
    const bareHeads = [
        '@derive',
        '@cfg',
        '@deprecated',
        '@mustUse',
        '@nonExhaustive'
    ];
    if (!bareHeads.some((head) => source.includes(head))) {
        return false;
    }
    let inCodeBlock = false;
    for (const line of source.split('\n')) {
        // Strip JSDoc comment syntax: /**, */, leading *, and whitespace
        const trimmed = line
            .trim()
            .replace(/^\/+/, '')
            .replace(/^\*+/, '')
            .replace(/\*+\/$/, '')
            .replace(/\/+$/, '')
            .trim();
        if (trimmed.startsWith('```')) {
            inCodeBlock = !inCodeBlock;
            continue;
        }
        if (inCodeBlock) {
            continue;
        }
        if (MACRO_TAG_PREFIXES.some((prefix) => trimmed.startsWith(prefix))) {
            return true;
        }
    }
    return false;
}

/**
 * Parses macro import comments from TypeScript code.
 *
 * @remarks
 * Extracts macro names mapped to their source module paths from
 * macro import comments like: `import macro { ... } from "package"`.
 *
 * @param text - The TypeScript source code to parse
 * @returns Map of macro names to their module paths
 *
 * @example
 * ```typescript
 * const text = `/** import macro {JSON, FieldController} from "@playground/macro"; *​/`;
 * parseMacroImportComments(text);
 * // => Map { "JSON" => "@playground/macro", "FieldController" => "@playground/macro" }
 * ```
 */
export function parseMacroImportComments(text: string): Map<string, string> {
    const imports = new Map<string, string>();
    const pattern = /\/\*\*\s*import\s+macro\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']/gi;
    let match: RegExpExecArray | null;

    while ((match = pattern.exec(text)) !== null) {
        const names = match[1]
            .split(',')
            .map((n) => n.trim())
            .filter(Boolean);
        const modulePath = match[2];
        for (const name of names) {
            imports.set(name, modulePath);
        }
    }
    return imports;
}
