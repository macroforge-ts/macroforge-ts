/**
 * @module macro-imports
 *
 * Utilities for parsing macro import comments in TypeScript code.
 */

/**
 * Checks whether source code contains `@derive(` as a real JSDoc directive.
 *
 * Only matches `@derive(` when it appears at the start of a JSDoc comment line
 * (after stripping comment syntax like `/**`, `*​/`, `*`, and whitespace).
 * This correctly rejects `@derive` embedded in prose text such as
 * `"Deserialize result format from @derive(Deserialize)"`.
 *
 * Use this instead of `code.includes("@derive")` to avoid false positives.
 *
 * @param source - The TypeScript source code to scan
 * @returns `true` if the source contains a real `@derive(` directive
 *
 * @example
 * ```typescript
 * hasMacroAnnotations('/** @derive(Debug) *​/ class X {}');     // true
 * hasMacroAnnotations('/** result from @derive(Debug) *​/');    // false — embedded in prose
 * hasMacroAnnotations('class X {}');                            // false
 * ```
 */
export function hasMacroAnnotations(source: string): boolean {
    if (!source.includes('@derive')) {
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
        // A line must START with @derive( to be a real directive.
        if (trimmed.startsWith('@derive(')) {
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
