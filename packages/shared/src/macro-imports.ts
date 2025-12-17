/**
 * @module macro-imports
 *
 * Utilities for parsing macro import comments in TypeScript code.
 */

/**
 * Parses macro import comments from TypeScript code.
 *
 * @remarks
 * Extracts macro names mapped to their source module paths from
 * `/** import macro { ... } from "package" *​/` comments.
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
  const pattern =
    /\/\*\*\s*import\s+macro\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']/gi;
  let match: RegExpExecArray | null;

  while ((match = pattern.exec(text)) !== null) {
    const names = match[1]
      .split(",")
      .map((n) => n.trim())
      .filter(Boolean);
    const modulePath = match[2];
    for (const name of names) {
      imports.set(name, modulePath);
    }
  }
  return imports;
}
