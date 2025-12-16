/**
 * @fileoverview TypeScript Language Service Plugin for Macroforge
 *
 * This plugin integrates Macroforge's compile-time macro expansion with TypeScript's
 * Language Service to provide seamless IDE support for macro-decorated classes.
 *
 * ## Architecture Overview
 *
 * The plugin operates by intercepting TypeScript's Language Service methods and
 * transforming source code on-the-fly:
 *
 * 1. **Macro Expansion**: When TypeScript requests a file's content via `getScriptSnapshot`,
 *    this plugin intercepts the call and returns the macro-expanded version instead.
 *
 * 2. **Position Mapping**: Since expanded code has different positions than the original,
 *    the plugin maintains a {@link PositionMapper} for each file to translate positions
 *    between original and expanded coordinates.
 *
 * 3. **Virtual .d.ts Files**: For each macro-containing file, the plugin generates a
 *    companion `.macroforge.d.ts` file containing type declarations for generated methods.
 *
 * ## Supported File Types
 *
 * - `.ts` - TypeScript files
 * - `.tsx` - TypeScript JSX files
 * - `.svelte` - Svelte components (with `<script lang="ts">`)
 *
 * ## Hook Categories
 *
 * The plugin hooks into three categories of Language Service methods:
 *
 * - **Host-level hooks**: Control what TypeScript "sees" (`getScriptSnapshot`, `fileExists`, etc.)
 * - **Diagnostic hooks**: Map error positions back to original source (`getSemanticDiagnostics`)
 * - **Navigation hooks**: Handle go-to-definition, references, completions, etc.
 *
 * @example
 * ```typescript
 * // tsconfig.json
 * {
 *   "compilerOptions": {
 *     "plugins": [{ "name": "@macroforge/typescript-plugin" }]
 *   }
 * }
 * ```
 *
 * @see {@link init} - The main plugin factory function
 * @see {@link PositionMapper} - Position mapping between original and expanded code
 * @module @macroforge/typescript-plugin
 */

import type ts from "typescript/lib/tsserverlibrary";
import type { ExpandResult, MacroManifest, MacroManifestEntry, DecoratorManifestEntry } from "macroforge";
import { NativePlugin, PositionMapper, __macroforgeGetManifest } from "macroforge";
import path from "path";
import fs from "fs";

/**
 * Cached macro manifest for hover information.
 *
 * This cache stores macro and decorator metadata loaded from the native Macroforge
 * plugin. The cache is populated on first access and persists for the lifetime of
 * the language server process.
 *
 * @internal
 */
let macroManifestCache: {
  /** Map of lowercase macro name to its manifest entry */
  macros: Map<string, MacroManifestEntry>;
  /** Map of lowercase decorator export name to its manifest entry */
  decorators: Map<string, DecoratorManifestEntry>;
} | null = null;

/**
 * Retrieves the cached macro manifest, loading it if necessary.
 *
 * The manifest contains metadata about all available macros and decorators,
 * including their names, descriptions, and documentation. This information
 * is used to provide hover tooltips in the IDE.
 *
 * @returns The macro manifest with Maps for quick lookup by name, or `null` if
 *          the manifest could not be loaded (e.g., native plugin not available)
 *
 * @remarks
 * The manifest is cached after first load. Macro names and decorator exports
 * are stored in lowercase for case-insensitive lookups.
 *
 * @example
 * ```typescript
 * const manifest = getMacroManifest();
 * if (manifest) {
 *   const debugMacro = manifest.macros.get('debug');
 *   const serdeDecorator = manifest.decorators.get('serde');
 * }
 * ```
 */
function getMacroManifest() {
  if (macroManifestCache) return macroManifestCache;

  try {
    const manifest = __macroforgeGetManifest();
    macroManifestCache = {
      macros: new Map(manifest.macros.map(m => [m.name.toLowerCase(), m])),
      decorators: new Map(manifest.decorators.map(d => [d.export.toLowerCase(), d])),
    };
    return macroManifestCache;
  } catch {
    return null;
  }
}

/**
 * Parses macro import comments to extract macro name to module path mappings.
 *
 * Macroforge supports importing external macros using a special JSDoc comment syntax:
 * `/** import macro {MacroName, Another} from "@scope/package"; *​/`
 *
 * @param text - The source text to search for import comments
 * @returns A Map of macro name to module path
 *
 * @example
 * ```typescript
 * const text = `/** import macro {Gigaform, CustomMacro} from "@playground/macro"; *​/`;
 * parseMacroImportComments(text);
 * // => Map { "Gigaform" => "@playground/macro", "CustomMacro" => "@playground/macro" }
 * ```
 */
function parseMacroImportComments(text: string): Map<string, string> {
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

/**
 * Cache for external macro package manifests.
 * Maps package path to its manifest (or null if failed to load).
 */
const externalManifestCache = new Map<
  string,
  MacroManifest | null
>();

/**
 * Attempts to load the manifest from an external macro package.
 *
 * External macro packages (like `@playground/macro`) export their own
 * `__macroforgeGetManifest()` function that provides macro metadata
 * including descriptions.
 *
 * @param modulePath - The package path (e.g., "@playground/macro")
 * @returns The macro manifest, or null if loading failed
 */
function getExternalManifest(modulePath: string): MacroManifest | null {
  if (externalManifestCache.has(modulePath)) {
    return externalManifestCache.get(modulePath) ?? null;
  }

  try {
    // Try to require the external package
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const pkg = require(modulePath);
    if (typeof pkg.__macroforgeGetManifest === "function") {
      const manifest: MacroManifest = pkg.__macroforgeGetManifest();
      externalManifestCache.set(modulePath, manifest);
      return manifest;
    }
  } catch {
    // Package not found or doesn't export manifest
  }

  externalManifestCache.set(modulePath, null);
  return null;
}

/**
 * Looks up macro info from an external package manifest.
 *
 * @param macroName - The macro name to look up
 * @param modulePath - The package path
 * @returns The macro manifest entry, or null if not found
 */
function getExternalMacroInfo(
  macroName: string,
  modulePath: string,
): MacroManifestEntry | null {
  const manifest = getExternalManifest(modulePath);
  if (!manifest) return null;

  return (
    manifest.macros.find(
      (m) => m.name.toLowerCase() === macroName.toLowerCase(),
    ) ?? null
  );
}

/**
 * Looks up decorator info from an external package manifest.
 *
 * @param decoratorName - The decorator name to look up
 * @param modulePath - The package path
 * @returns The decorator manifest entry, or null if not found
 */
function getExternalDecoratorInfo(
  decoratorName: string,
  modulePath: string,
): DecoratorManifestEntry | null {
  const manifest = getExternalManifest(modulePath);
  if (!manifest) return null;

  return (
    manifest.decorators.find(
      (d) => d.export.toLowerCase() === decoratorName.toLowerCase(),
    ) ?? null
  );
}

/**
 * Finds a macro name within `@derive(...)` decorators at a given cursor position.
 *
 * This function parses JSDoc comments looking for `@derive` directives and determines
 * if the cursor position falls within a specific macro name in the argument list.
 *
 * @param text - The source text to search
 * @param position - The cursor position as a 0-indexed character offset from the start of the file
 * @returns An object containing the macro name and its character span, or `null` if the
 *          position is not within a macro name
 *
 * @remarks
 * The function uses the regex `/@derive\s*\(\s*([^)]+)\s*\)/gi` to find all `@derive`
 * decorators, then parses the comma-separated macro names within the parentheses.
 *
 * Position calculation accounts for:
 * - Whitespace between `@derive` and the opening parenthesis
 * - Whitespace around macro names in the argument list
 * - Multiple macros separated by commas
 *
 * @example
 * ```typescript
 * // Given text: "/** @derive(Debug, Clone) * /"
 * // Position 14 (on "Debug") returns:
 * findDeriveAtPosition(text, 14);
 * // => { macroName: "Debug", start: 12, end: 17 }
 *
 * // Position 20 (on "Clone") returns:
 * findDeriveAtPosition(text, 20);
 * // => { macroName: "Clone", start: 19, end: 24 }
 *
 * // Position 5 (before @derive) returns:
 * findDeriveAtPosition(text, 5);
 * // => null
 * ```
 */
function findDeriveAtPosition(
  text: string,
  position: number,
): { macroName: string; start: number; end: number } | null {
  const derivePattern = /@derive\s*\(\s*([^)]+)\s*\)/gi;
  let match: RegExpExecArray | null;

  while ((match = derivePattern.exec(text)) !== null) {
    const deriveStart = match.index;
    const deriveEnd = deriveStart + match[0].length;

    if (position >= deriveStart && position <= deriveEnd) {
      const argsStart = text.indexOf("(", deriveStart) + 1;
      const argsEnd = text.indexOf(")", argsStart);
      const argsContent = text.substring(argsStart, argsEnd);

      let currentPos = argsStart;
      const macroNames = argsContent.split(",");

      for (const rawName of macroNames) {
        const trimmedName = rawName.trim();
        const nameStartInArgs = rawName.indexOf(trimmedName);
        const nameStart = currentPos + nameStartInArgs;
        const nameEnd = nameStart + trimmedName.length;

        if (position >= nameStart && position <= nameEnd) {
          return { macroName: trimmedName, start: nameStart, end: nameEnd };
        }

        currentPos += rawName.length + 1;
      }
    }
  }

  return null;
}

/**
 * Finds the `@derive` keyword at a given cursor position.
 * This matches the literal "@derive" text before the opening parenthesis,
 * allowing hover documentation on the directive keyword itself.
 *
 * @param text - The source text to search
 * @param position - The cursor position as a 0-indexed character offset
 * @returns An object with start/end positions, or `null` if not on @derive keyword
 *
 * @example
 * ```typescript
 * // Given text: "/** @derive(Debug) *​/"
 * findDeriveKeywordAtPosition(text, 5);
 * // => { start: 4, end: 11 }  // covers "@derive"
 *
 * // Position on "Debug" (inside parens) returns null
 * findDeriveKeywordAtPosition(text, 12);
 * // => null
 * ```
 *
 * @see {@link findDeriveAtPosition} - For macro names inside @derive()
 */
function findDeriveKeywordAtPosition(
  text: string,
  position: number,
): { start: number; end: number } | null {
  // Match @derive only when followed by ( to distinguish from other uses
  const deriveKeywordPattern = /@derive(?=\s*\()/gi;
  let match: RegExpExecArray | null;

  while ((match = deriveKeywordPattern.exec(text)) !== null) {
    const start = match.index; // Position of @
    const end = start + "@derive".length;

    if (position >= start && position < end) {
      return { start, end };
    }
  }
  return null;
}

/**
 * Finds a field decorator (like `@serde` or `@debug`) at a given cursor position.
 *
 * This function searches for decorator patterns (`@name`) in the source text and
 * determines if the cursor falls within one. It's used to provide hover information
 * for Macroforge field decorators.
 *
 * @param text - The source text to search
 * @param position - The cursor position as a 0-indexed character offset
 * @returns An object containing the decorator name (without `@`) and its span
 *          (including the `@` symbol), or `null` if not found
 *
 * @remarks
 * This function explicitly skips `@derive` decorators that appear within JSDoc comments,
 * as those are handled by {@link findDeriveAtPosition} instead. The detection works by
 * checking if the match is between an unclosed JSDoc start and end markers.
 *
 * The span returned includes the `@` symbol, so for `@serde`:
 * - `start` points to the `@` character
 * - `end` points to the character after the last letter of the name
 *
 * @example
 * ```typescript
 * // Given text: "class User { @serde name: string; }"
 * findDecoratorAtPosition(text, 14);
 * // => { name: "serde", start: 13, end: 19 }
 *
 * // @derive in JSDoc is skipped (handled by findDeriveAtPosition)
 * // Given text: "/** @derive(Debug) * /"
 * findDecoratorAtPosition(text, 5);
 * // => null
 * ```
 *
 * @see {@link findDeriveAtPosition} - For `@derive` decorators in JSDoc comments
 */
function findDecoratorAtPosition(
  text: string,
  position: number,
): { name: string; start: number; end: number } | null {
  const decoratorPattern = /@([a-zA-Z_$][a-zA-Z0-9_$]*)/g;
  let match: RegExpExecArray | null;

  while ((match = decoratorPattern.exec(text)) !== null) {
    const atSign = match.index;
    const nameStart = atSign + 1;
    const nameEnd = nameStart + match[1].length;

    if (position >= atSign && position <= nameEnd) {
      // Skip @derive in JSDoc comments - let findDeriveAtPosition handle those
      if (match[1].toLowerCase() === "derive") {
        const beforeMatch = text.substring(0, atSign);
        const lastCommentStart = beforeMatch.lastIndexOf("/**");
        const lastCommentEnd = beforeMatch.lastIndexOf("*/");
        if (lastCommentStart > lastCommentEnd) {
          continue;
        }
      }

      return { name: match[1], start: atSign, end: nameEnd };
    }
  }

  return null;
}

/**
 * Finds what `@derive` macros apply to code at a given position.
 *
 * This function uses a heuristic: it finds the nearest `@derive(...)` decorator
 * that appears before the given position. This is useful for determining which
 * macros might be responsible for a particular field decorator.
 *
 * @param text - The source text to search
 * @param position - The cursor position as a 0-indexed character offset
 * @returns An array of macro names from the enclosing @derive, or `null` if not found
 *
 * @example
 * ```typescript
 * const text = `/** @derive(Debug, Serialize) *​/
 * class User {
 *   @serde({ skip: true })
 *   password: string;
 * }`;
 *
 * // Position on @serde
 * findEnclosingDeriveContext(text, text.indexOf("@serde"));
 * // => ["Debug", "Serialize"]
 * ```
 */
function findEnclosingDeriveContext(
  text: string,
  position: number,
): string[] | null {
  const beforePosition = text.substring(0, position);
  const derivePattern = /@derive\s*\(\s*([^)]+)\s*\)/gi;
  let lastMatch: RegExpExecArray | null = null;
  let match: RegExpExecArray | null;

  while ((match = derivePattern.exec(beforePosition)) !== null) {
    lastMatch = match;
  }

  if (lastMatch) {
    const macros = lastMatch[1]
      .split(",")
      .map((m) => m.trim())
      .filter(Boolean);
    return macros;
  }
  return null;
}

/**
 * Generates hover information (QuickInfo) for macros and decorators at a cursor position.
 *
 * This function provides IDE hover tooltips for Macroforge-specific syntax:
 * - The `@derive` keyword itself
 * - Macro names within `@derive(...)` JSDoc decorators (both built-in and external)
 * - Field decorators like `@serde`, `@debug`, and custom decorators from external macros
 *
 * @param text - The source text to analyze
 * @param position - The cursor position as a 0-indexed character offset
 * @param tsModule - The TypeScript module reference (for creating QuickInfo structures)
 * @returns A TypeScript QuickInfo object suitable for hover display, or `null` if the
 *          position is not on a recognized macro or decorator
 *
 * @remarks
 * The function checks positions in the following order:
 * 1. Check if cursor is on the `@derive` keyword via {@link findDeriveKeywordAtPosition}
 * 2. Check if cursor is on a macro name within `@derive(...)` via {@link findDeriveAtPosition}
 *    - First checks built-in manifest via {@link getMacroManifest}
 *    - Then checks external macro imports via {@link parseMacroImportComments}
 *    - Falls back to generic hover for unknown macros
 * 3. Check if cursor is on a field decorator via {@link findDecoratorAtPosition}
 *    - First checks built-in manifest (macros and decorators)
 *    - Then checks external package manifests via {@link getExternalDecoratorInfo}
 *    - Falls back to generic hover showing enclosing derive context
 *
 * For external macros (imported via `/** import macro {Name} from "package"; * /`),
 * the function attempts to load the external package's manifest to retrieve
 * descriptions and documentation. See {@link getExternalMacroInfo}.
 *
 * The returned QuickInfo includes:
 * - `kind`: `keyword` for @derive, `functionElement` for macros/decorators
 * - `textSpan`: The highlighted range in the editor
 * - `displayParts`: The formatted display text (e.g., "@derive(Debug)")
 * - `documentation`: The macro/decorator description from the manifest
 *
 * @example
 * ```typescript
 * // Hovering over "@derive" keyword
 * const info = getMacroHoverInfo(text, 4, ts);
 * // Returns QuickInfo with documentation about the derive directive
 *
 * // Hovering over "Debug" in "@derive(Debug, Clone)"
 * const info = getMacroHoverInfo(text, 14, ts);
 * // Returns QuickInfo with:
 * // - displayParts: "@derive(Debug)"
 * // - documentation: "Generates a fmt_debug() method for debugging output"
 *
 * // Hovering over external macro "Gigaform" in "@derive(Gigaform)"
 * const info = getMacroHoverInfo(text, 14, ts);
 * // Returns QuickInfo with description loaded from @playground/macro package
 *
 * // Hovering over "@serde" field decorator
 * const info = getMacroHoverInfo(text, 5, ts);
 * // Returns QuickInfo with:
 * // - displayParts: "@serde"
 * // - documentation: "Serialization/deserialization field options"
 *
 * // Hovering over "@hiddenController" from external Gigaform macro
 * const info = getMacroHoverInfo(text, 5, ts);
 * // Returns QuickInfo with docs loaded from external package manifest
 * ```
 *
 * @see {@link findDeriveKeywordAtPosition} - Locates the @derive keyword
 * @see {@link findDeriveAtPosition} - Locates macro names in @derive decorators
 * @see {@link findDecoratorAtPosition} - Locates field decorators
 * @see {@link findEnclosingDeriveContext} - Finds macros that apply to a position
 * @see {@link getMacroManifest} - Provides built-in macro/decorator metadata
 * @see {@link getExternalMacroInfo} - Provides external macro metadata
 * @see {@link getExternalDecoratorInfo} - Provides external decorator metadata
 */
function getMacroHoverInfo(
  text: string,
  position: number,
  tsModule: typeof ts,
): ts.QuickInfo | null {
  const manifest = getMacroManifest();

  // 1. Check if hovering on @derive keyword itself
  const deriveKeyword = findDeriveKeywordAtPosition(text, position);
  if (deriveKeyword) {
    return {
      kind: tsModule.ScriptElementKind.keyword,
      kindModifiers: "",
      textSpan: {
        start: deriveKeyword.start,
        length: deriveKeyword.end - deriveKeyword.start,
      },
      displayParts: [{ text: "@derive", kind: "keyword" }],
      documentation: [
        {
          text:
            "Derive directive - applies compile-time macros to generate methods and implementations.\n\n" +
            "**Usage:** `/** @derive(MacroName, AnotherMacro) */`\n\n" +
            "**Built-in macros:** Debug, Clone, Default, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize\n\n" +
            "External macros can be imported using:\n" +
            '`/** import macro {Name} from "package"; */`',
          kind: "text",
        },
      ],
    };
  }

  // Parse external macro imports for later use
  const externalMacros = parseMacroImportComments(text);

  // 2. Check for @derive(MacroName) in JSDoc comments
  const deriveMatch = findDeriveAtPosition(text, position);
  if (deriveMatch) {
    // 2a. Check built-in manifest
    const macroInfo = manifest?.macros.get(deriveMatch.macroName.toLowerCase());
    if (macroInfo) {
      return {
        kind: tsModule.ScriptElementKind.functionElement,
        kindModifiers: "",
        textSpan: {
          start: deriveMatch.start,
          length: deriveMatch.end - deriveMatch.start,
        },
        displayParts: [
          { text: "@derive(", kind: "punctuation" },
          { text: macroInfo.name, kind: "functionName" },
          { text: ")", kind: "punctuation" },
        ],
        documentation: macroInfo.description
          ? [{ text: macroInfo.description, kind: "text" }]
          : [],
      };
    }

    // 2b. Check external macro imports
    const modulePath = externalMacros.get(deriveMatch.macroName);
    if (modulePath) {
      // Try to get detailed info from the external package manifest
      const externalMacroInfo = getExternalMacroInfo(
        deriveMatch.macroName,
        modulePath,
      );
      const description = externalMacroInfo?.description
        ? externalMacroInfo.description
        : "This macro is loaded from an external package at compile time.";

      return {
        kind: tsModule.ScriptElementKind.functionElement,
        kindModifiers: "external",
        textSpan: {
          start: deriveMatch.start,
          length: deriveMatch.end - deriveMatch.start,
        },
        displayParts: [
          { text: "@derive(", kind: "punctuation" },
          { text: externalMacroInfo?.name ?? deriveMatch.macroName, kind: "functionName" },
          { text: ")", kind: "punctuation" },
        ],
        documentation: [
          {
            text: `**External macro** from \`${modulePath}\`\n\n${description}`,
            kind: "text",
          },
        ],
      };
    }

    // 2c. Fallback for unknown/unrecognized macros
    return {
      kind: tsModule.ScriptElementKind.functionElement,
      kindModifiers: "",
      textSpan: {
        start: deriveMatch.start,
        length: deriveMatch.end - deriveMatch.start,
      },
      displayParts: [
        { text: "@derive(", kind: "punctuation" },
        { text: deriveMatch.macroName, kind: "functionName" },
        { text: ")", kind: "punctuation" },
      ],
      documentation: [
        {
          text:
            `**Macro:** ${deriveMatch.macroName}\n\n` +
            "This macro is not in the built-in manifest. If it's a custom macro, " +
            "ensure it's imported using:\n\n" +
            `\`/** import macro {${deriveMatch.macroName}} from "your-package"; */\``,
          kind: "text",
        },
      ],
    };
  }

  // 3. Check for @decorator patterns
  const decoratorMatch = findDecoratorAtPosition(text, position);
  if (decoratorMatch) {
    // 3a. Check if it's a built-in macro name
    const macroInfo = manifest?.macros.get(decoratorMatch.name.toLowerCase());
    if (macroInfo) {
      return {
        kind: tsModule.ScriptElementKind.functionElement,
        kindModifiers: "",
        textSpan: {
          start: decoratorMatch.start,
          length: decoratorMatch.end - decoratorMatch.start,
        },
        displayParts: [
          { text: "@", kind: "punctuation" },
          { text: macroInfo.name, kind: "functionName" },
        ],
        documentation: macroInfo.description
          ? [{ text: macroInfo.description, kind: "text" }]
          : [],
      };
    }

    // 3b. Check if it's a built-in decorator
    const decoratorInfo = manifest?.decorators.get(
      decoratorMatch.name.toLowerCase(),
    );
    if (decoratorInfo && decoratorInfo.docs) {
      return {
        kind: tsModule.ScriptElementKind.functionElement,
        kindModifiers: "",
        textSpan: {
          start: decoratorMatch.start,
          length: decoratorMatch.end - decoratorMatch.start,
        },
        displayParts: [
          { text: "@", kind: "punctuation" },
          { text: decoratorInfo.export, kind: "functionName" },
        ],
        documentation: [{ text: decoratorInfo.docs, kind: "text" }],
      };
    }

    // 3c. Check if this decorator is in a macro context (for external/custom decorators)
    const enclosingMacros = findEnclosingDeriveContext(
      text,
      decoratorMatch.start,
    );
    if (enclosingMacros && enclosingMacros.length > 0) {
      // Find which external macro might define this decorator
      const likelySourceMacro = enclosingMacros.find((m) =>
        externalMacros.has(m),
      );

      if (likelySourceMacro) {
        const modulePath = externalMacros.get(likelySourceMacro);
        // Try to get detailed decorator info from the external package
        const externalDecoratorInfo = modulePath
          ? getExternalDecoratorInfo(decoratorMatch.name, modulePath)
          : null;
        const description = externalDecoratorInfo?.docs
          ? externalDecoratorInfo.docs
          : "This decorator configures field-level behavior for the macro.";

        return {
          kind: tsModule.ScriptElementKind.functionElement,
          kindModifiers: "external",
          textSpan: {
            start: decoratorMatch.start,
            length: decoratorMatch.end - decoratorMatch.start,
          },
          displayParts: [
            { text: "@", kind: "punctuation" },
            { text: externalDecoratorInfo?.export ?? decoratorMatch.name, kind: "functionName" },
          ],
          documentation: [
            {
              text:
                `**Field decorator** from \`${likelySourceMacro}\` macro (\`${modulePath}\`)\n\n` +
                description,
              kind: "text",
            },
          ],
        };
      }

      // Fallback: Generic decorator in macro context
      return {
        kind: tsModule.ScriptElementKind.functionElement,
        kindModifiers: "",
        textSpan: {
          start: decoratorMatch.start,
          length: decoratorMatch.end - decoratorMatch.start,
        },
        displayParts: [
          { text: "@", kind: "punctuation" },
          { text: decoratorMatch.name, kind: "functionName" },
        ],
        documentation: [
          {
            text:
              `**Field decorator:** ${decoratorMatch.name}\n\n` +
              `Used with @derive(${enclosingMacros.join(", ")}).\n` +
              "This decorator configures field-level behavior for the applied macros.",
            kind: "text",
          },
        ],
      };
    }
  }

  return null;
}

/**
 * File extensions that the plugin will process for macro expansion.
 * @internal
 */
const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];

/**
 * Determines whether a file should be processed for macro expansion.
 *
 * This is a gatekeeper function that filters out files that should not
 * go through macro expansion, either because they're in excluded directories
 * or have unsupported file types.
 *
 * @param fileName - The absolute path to the file
 * @returns `true` if the file should be processed, `false` otherwise
 *
 * @remarks
 * Files are excluded if they:
 * - Are in `node_modules` (dependencies should not be processed)
 * - Are in the `.macroforge` cache directory
 * - End with `.macroforge.d.ts` (generated type declaration files)
 * - Don't have a supported extension (`.ts`, `.tsx`, `.svelte`)
 *
 * @example
 * ```typescript
 * shouldProcess('/project/src/User.ts');        // => true
 * shouldProcess('/project/src/App.svelte');     // => true
 * shouldProcess('/project/node_modules/...');   // => false
 * shouldProcess('/project/User.macroforge.d.ts'); // => false
 * ```
 */
function shouldProcess(fileName: string) {
  const lower = fileName.toLowerCase();
  if (lower.includes("node_modules")) return false;
  if (fileName.includes(`${path.sep}.macroforge${path.sep}`)) return false;
  // Skip generated .d.ts files
  if (fileName.endsWith(".macroforge.d.ts")) return false;
  return FILE_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

/**
 * Performs a quick check to determine if a file contains any macro-related directives.
 *
 * This is a fast pre-filter to avoid expensive macro expansion on files that
 * don't contain any macros. It uses simple string/regex checks rather than
 * full parsing for performance.
 *
 * @param text - The source text to check
 * @returns `true` if the file likely contains macro directives, `false` otherwise
 *
 * @remarks
 * The function checks for the following patterns:
 * - `@derive` anywhere in the text (catches both JSDoc and decorator usage)
 * - `/** @derive(` pattern (JSDoc macro declaration)
 * - `/** import macro` pattern (inline macro import syntax)
 *
 * This is intentionally permissive - it's better to have false positives
 * (which just result in unnecessary expansion attempts) than false negatives
 * (which would break macro functionality).
 *
 * @example
 * ```typescript
 * hasMacroDirectives('/** @derive(Debug) * /');  // => true
 * hasMacroDirectives('@Debug class User {}');    // => true (contains @derive substring? no, but @Debug yes)
 * hasMacroDirectives('class User {}');           // => false
 * ```
 */
function hasMacroDirectives(text: string) {
  return (
    text.includes("@derive") ||
    /\/\*\*\s*@derive\s*\(/i.test(text) ||
    /\/\*\*\s*import\s+macro\b/i.test(text)
  );
}

/**
 * Configuration options loaded from `macroforge.json`.
 *
 * @remarks
 * This configuration affects how macros are expanded and what artifacts
 * are preserved in the output.
 */
type MacroConfig = {
  /**
   * Whether to preserve decorator syntax in the expanded output.
   *
   * When `true`, decorators like `@serde` are kept in the expanded code
   * (useful for runtime decorator processing). When `false`, they are
   * stripped during expansion.
   *
   * @default false
   */
  keepDecorators: boolean;
};

/**
 * Loads Macroforge configuration by searching for `macroforge.json` up the directory tree.
 *
 * Starting from the given directory, this function walks up the filesystem hierarchy
 * looking for a `macroforge.json` configuration file. The first one found is parsed
 * and its settings are returned.
 *
 * @param startDir - The directory to start searching from (typically the project root)
 * @returns The parsed configuration, or default values if no config file is found
 *
 * @remarks
 * The search stops when:
 * - A `macroforge.json` file is found and successfully parsed
 * - The filesystem root is reached
 * - A parse error occurs (falls back to defaults)
 *
 * This allows monorepo setups where a root `macroforge.json` can configure
 * all packages, while individual packages can override with their own config.
 *
 * @example
 * ```typescript
 * // With /project/macroforge.json containing: { "keepDecorators": true }
 * loadMacroConfig('/project/src/components');
 * // => { keepDecorators: true }
 *
 * // With no macroforge.json found:
 * loadMacroConfig('/some/other/path');
 * // => { keepDecorators: false }
 * ```
 */
function loadMacroConfig(startDir: string): MacroConfig {
  let current = startDir;
  const fallback: MacroConfig = { keepDecorators: false };

  while (true) {
    const candidate = path.join(current, "macroforge.json");
    if (fs.existsSync(candidate)) {
      try {
        const raw = fs.readFileSync(candidate, "utf8");
        const parsed = JSON.parse(raw);
        return { keepDecorators: Boolean(parsed.keepDecorators) };
      } catch {
        return fallback;
      }
    }

    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }

  return fallback;
}

/**
 * Main plugin factory function conforming to the TypeScript Language Service Plugin API.
 *
 * This function is called by TypeScript when the plugin is loaded. It receives the
 * TypeScript module reference and returns an object with a `create` function that
 * TypeScript will call to instantiate the plugin for each project.
 *
 * @param modules - Object containing the TypeScript module reference
 * @param modules.typescript - The TypeScript module (`typescript/lib/tsserverlibrary`)
 * @returns An object with a `create` method that TypeScript calls to instantiate the plugin
 *
 * @remarks
 * The plugin follows the standard TypeScript Language Service Plugin pattern:
 * 1. `init()` is called once when the plugin is loaded
 * 2. `create()` is called for each TypeScript project that uses the plugin
 * 3. The returned LanguageService has hooked methods that intercept TypeScript operations
 *
 * ## Plugin Architecture
 *
 * The plugin maintains several internal data structures:
 * - **virtualDtsFiles**: Stores generated `.macroforge.d.ts` type declaration files
 * - **snapshotCache**: Caches expanded file snapshots for stable identity across TS requests
 * - **processingFiles**: Guards against reentrancy during macro expansion
 * - **nativePlugin**: Rust-backed expansion engine (handles actual macro processing)
 *
 * ## Hooked Methods
 *
 * The plugin hooks into ~22 TypeScript Language Service methods to provide seamless
 * IDE support. These fall into three categories:
 *
 * 1. **Host-level hooks** (what TS "sees"):
 *    - `getScriptSnapshot` - Returns expanded code instead of original
 *    - `getScriptVersion` - Provides versions for virtual .d.ts files
 *    - `getScriptFileNames` - Includes virtual .d.ts in project file list
 *    - `fileExists` - Resolves virtual .d.ts files
 *
 * 2. **Diagnostic hooks** (error reporting):
 *    - `getSemanticDiagnostics` - Maps error positions, adds macro errors
 *    - `getSyntacticDiagnostics` - Maps syntax error positions
 *
 * 3. **Navigation hooks** (IDE features):
 *    - `getQuickInfoAtPosition` - Hover information
 *    - `getCompletionsAtPosition` - IntelliSense completions
 *    - `getDefinitionAtPosition` - Go to definition
 *    - `findReferences` - Find all references
 *    - ... and many more
 *
 * @example
 * ```typescript
 * // This is how TypeScript loads the plugin (internal to TS)
 * const plugin = require('@macroforge/typescript-plugin');
 * const { create } = plugin(modules);
 * const languageService = create(pluginCreateInfo);
 * ```
 *
 * @see {@link shouldProcess} - File filtering logic
 * @see {@link processFile} - Main macro expansion entry point
 */
function init(modules: { typescript: typeof ts }) {
  /**
   * Creates the plugin instance for a TypeScript project.
   *
   * This function is called by TypeScript for each project that has the plugin configured.
   * It sets up all the necessary hooks and state, then returns the modified LanguageService.
   *
   * @param info - Plugin creation info provided by TypeScript, containing:
   *   - `project`: The TypeScript project instance
   *   - `languageService`: The base LanguageService to augment
   *   - `languageServiceHost`: The host providing file system access
   *   - `config`: Plugin configuration from tsconfig.json
   * @returns The augmented LanguageService with macro support
   */
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;

    /**
     * Map storing generated virtual `.macroforge.d.ts` files.
     *
     * For each source file containing macros, we generate a companion `.d.ts` file
     * with type declarations for the generated methods. These virtual files are
     * served to TypeScript as if they existed on disk.
     *
     * @remarks
     * Key: Virtual file path (e.g., `/project/src/User.ts.macroforge.d.ts`)
     * Value: ScriptSnapshot containing the generated type declarations
     */
    const virtualDtsFiles = new Map<string, ts.IScriptSnapshot>();

    /**
     * Cache for processed file snapshots to ensure identity stability.
     *
     * TypeScript's incremental compiler relies on snapshot identity to detect changes.
     * By caching snapshots keyed by version, we ensure the same snapshot object is
     * returned for unchanged files, preventing unnecessary recompilation.
     *
     * @remarks
     * Key: Source file path
     * Value: Object containing the file version and its expanded snapshot
     */
    const snapshotCache = new Map<
      string,
      { version: string; snapshot: ts.IScriptSnapshot }
    >();

    /**
     * Set of files currently being processed for macro expansion.
     *
     * This guards against reentrancy - if TypeScript requests a file while we're
     * already processing it (e.g., due to import resolution during expansion),
     * we return the original content to prevent infinite loops.
     */
    const processingFiles = new Set<string>();

    /**
     * Native Rust-backed plugin instance for macro expansion.
     *
     * The NativePlugin handles the actual macro expansion logic, caching, and
     * source mapping. It's implemented in Rust for performance and is accessed
     * via N-API bindings.
     */
    const nativePlugin = new NativePlugin();

    /**
     * Gets the current working directory for the project.
     *
     * Tries multiple sources in order of preference:
     * 1. Project's getCurrentDirectory method
     * 2. Language service host's getCurrentDirectory method
     * 3. Falls back to process.cwd()
     *
     * @returns The project's root directory path
     */
    const getCurrentDirectory = () =>
      info.project.getCurrentDirectory?.() ??
      info.languageServiceHost.getCurrentDirectory?.() ??
      process.cwd();

    const macroConfig = loadMacroConfig(getCurrentDirectory());
    const keepDecorators = macroConfig.keepDecorators;

    /**
     * Logs a message to multiple destinations for debugging.
     *
     * Messages are sent to:
     * 1. The native Rust plugin (for unified logging)
     * 2. TypeScript's project service logger (visible in tsserver logs)
     * 3. stderr (for development debugging)
     *
     * @param msg - The message to log (will be prefixed with timestamp and [macroforge])
     */
    const log = (msg: string) => {
      const line = `[${new Date().toISOString()}] ${msg}`;
      nativePlugin.log(line);
      try {
        info.project.projectService.logger.info(`[macroforge] ${msg}`);
      } catch {}
      try {
        console.error(`[macroforge] ${msg}`);
      } catch {}
    };

    /**
     * Registers a virtual `.macroforge.d.ts` file with TypeScript's project service.
     *
     * This makes TypeScript aware of our generated type declaration files so they
     * can be resolved during import resolution and type checking.
     *
     * @param fileName - The path to the virtual .d.ts file to register
     *
     * @remarks
     * Uses internal TypeScript APIs (`getOrCreateScriptInfoNotOpenedByClient`)
     * which may change between TypeScript versions. The function gracefully
     * handles missing APIs.
     */
    const ensureVirtualDtsRegistered = (fileName: string) => {
      const projectService = info.project.projectService as any;
      const register = projectService?.getOrCreateScriptInfoNotOpenedByClient;
      if (!register) return;

      try {
        const scriptInfo = register(
          fileName,
          getCurrentDirectory(),
          info.languageServiceHost,
          /*deferredDeleteOk*/ false,
        );
        if (scriptInfo?.attachToProject) {
          scriptInfo.attachToProject(info.project);
        }
      } catch (error) {
        log(
          `Failed to register virtual .d.ts ${fileName}: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    };

    /**
     * Removes a virtual `.macroforge.d.ts` file from TypeScript's project service.
     *
     * Called when a source file no longer generates types (e.g., macros removed)
     * to clean up stale virtual files and prevent memory leaks.
     *
     * @param fileName - The path to the virtual .d.ts file to remove
     *
     * @remarks
     * The cleanup is conservative - it only deletes the ScriptInfo if:
     * 1. The file is not open in an editor
     * 2. The file is not attached to any other projects
     */
    const cleanupVirtualDts = (fileName: string) => {
      const projectService = info.project.projectService as any;
      const getScriptInfo = projectService?.getScriptInfo;
      if (!getScriptInfo) return;

      try {
        const scriptInfo = getScriptInfo.call(projectService, fileName);
        if (!scriptInfo) return;

        scriptInfo.detachFromProject?.(info.project);
        if (
          !scriptInfo.isScriptOpen?.() &&
          scriptInfo.containingProjects?.length === 0
        ) {
          projectService.deleteScriptInfo?.(scriptInfo);
        }
      } catch (error) {
        log(
          `Failed to clean up virtual .d.ts ${fileName}: ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    };

    /**
     * Override projectService.setDocument to handle virtual files safely.
     *
     * This guards against TypeScript crashes when it tries to cache source files
     * for virtual .d.ts files that don't have full ScriptInfo backing.
     */
    const projectService = info.project.projectService as any;
    if (projectService?.setDocument) {
      projectService.setDocument = (
        key: unknown,
        filePath: string,
        sourceFile: unknown,
      ) => {
        try {
          const scriptInfo =
            projectService.getScriptInfoForPath?.(filePath) ??
            projectService.getOrCreateScriptInfoNotOpenedByClient?.(
              filePath,
              getCurrentDirectory(),
              info.languageServiceHost,
              /*deferredDeleteOk*/ false,
            );

          if (!scriptInfo) {
            log(`Skipping cache write for missing ScriptInfo at ${filePath}`);
            return;
          }

          scriptInfo.attachToProject?.(info.project);
          // Mirror the behavior of the original setDocument but avoid throwing when ScriptInfo is absent.
          scriptInfo.cacheSourceFile = { key, sourceFile } as any;
        } catch (error) {
          log(
            `Error in guarded setDocument for ${filePath}: ${error instanceof Error ? error.message : String(error)}`,
          );
        }
      };
    }

    log("Plugin initialized");

    /**
     * Processes a file through macro expansion via the native Rust plugin.
     *
     * This is the main entry point for macro expansion. It delegates to the native
     * Rust plugin for the actual transformation and handles virtual .d.ts file
     * management for generated type declarations.
     *
     * @param fileName - The absolute path to the source file
     * @param content - The source file content to expand
     * @param version - The file version (used for cache invalidation)
     * @returns An object containing:
     *   - `result`: The full ExpandResult from the native plugin (includes diagnostics, source mapping)
     *   - `code`: The expanded code (shorthand for result.code)
     *
     * @remarks
     * The function handles several important concerns:
     *
     * 1. **Empty file fast path**: Returns immediately for empty content
     * 2. **Virtual .d.ts management**: Creates/updates/removes companion type declaration files
     * 3. **Error recovery**: On expansion failure, returns original content and cleans up virtual files
     *
     * Caching is handled by the native Rust plugin based on the version parameter.
     * If the version hasn't changed since the last call, the cached result is returned.
     *
     * @example
     * ```typescript
     * const { result, code } = processFile('/project/src/User.ts', sourceText, '1');
     *
     * // result.code - The expanded TypeScript code
     * // result.types - Generated .d.ts content (if any)
     * // result.diagnostics - Macro expansion errors/warnings
     * // result.sourceMapping - Position mapping data
     * ```
     */
    function processFile(
      fileName: string,
      content: string,
      version: string,
    ): { result: ExpandResult; code: string } {
      // Fast Exit: Empty Content
      if (!content || content.trim().length === 0) {
        return {
          result: {
            code: content,
            types: undefined,
            metadata: undefined,
            diagnostics: [],
            sourceMapping: undefined,
          },
          code: content,
        };
      }

      try {
        log(`Processing ${fileName}`);

        const result = nativePlugin.processFile(fileName, content, {
          keepDecorators,
          version,
        });

        // Update virtual .d.ts files
        const virtualDtsFileName = fileName + ".macroforge.d.ts";
        if (result.types) {
          virtualDtsFiles.set(
            virtualDtsFileName,
            tsModule.ScriptSnapshot.fromString(result.types),
          );
          ensureVirtualDtsRegistered(virtualDtsFileName);
          log(`Generated virtual .d.ts for ${fileName}`);
        } else {
          virtualDtsFiles.delete(virtualDtsFileName);
          cleanupVirtualDts(virtualDtsFileName);
        }

        return { result, code: result.code };
      } catch (e) {
        const errorMessage =
          e instanceof Error ? e.stack || e.message : String(e);
        log(`Plugin expansion failed for ${fileName}: ${errorMessage}`);

        virtualDtsFiles.delete(fileName + ".macroforge.d.ts");
        cleanupVirtualDts(fileName + ".macroforge.d.ts");
        return {
          result: {
            code: content,
            types: undefined,
            metadata: undefined,
            diagnostics: [],
            sourceMapping: undefined,
          },
          code: content,
        };
      }
    }

    // =========================================================================
    // HOST-LEVEL HOOKS
    // These hooks control what TypeScript "sees" - the file content, versions,
    // and existence checks. They're the foundation of the plugin's operation.
    // =========================================================================

    /**
     * Hook: getScriptVersion
     *
     * Provides version strings for virtual `.macroforge.d.ts` files by deriving
     * them from the source file's version. This ensures TypeScript invalidates
     * the virtual file when its source changes.
     */
    const originalGetScriptVersion =
      info.languageServiceHost.getScriptVersion.bind(info.languageServiceHost);

    info.languageServiceHost.getScriptVersion = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName)) {
          // Virtual .d.ts files inherit version from their source file
          const sourceFileName = fileName.replace(".macroforge.d.ts", "");
          return originalGetScriptVersion(sourceFileName);
        }
        return originalGetScriptVersion(fileName);
      } catch (e) {
        log(
          `Error in getScriptVersion: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetScriptVersion(fileName);
      }
    };

    /**
     * Hook: getScriptFileNames
     *
     * Includes virtual `.macroforge.d.ts` files in the project's file list.
     * This allows TypeScript to "see" our generated type declaration files
     * and include them in type checking and import resolution.
     */
    const originalGetScriptFileNames = info.languageServiceHost
      .getScriptFileNames
      ? info.languageServiceHost.getScriptFileNames.bind(
          info.languageServiceHost,
        )
      : () => [];

    info.languageServiceHost.getScriptFileNames = () => {
      try {
        const originalFiles = originalGetScriptFileNames();
        // Append all virtual .d.ts files to the project's file list
        return [...originalFiles, ...Array.from(virtualDtsFiles.keys())];
      } catch (e) {
        log(
          `Error in getScriptFileNames: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetScriptFileNames();
      }
    };

    /**
     * Hook: fileExists
     *
     * Makes virtual `.macroforge.d.ts` files appear to exist on disk.
     * This allows TypeScript's module resolution to find our generated
     * type declaration files.
     */
    const originalFileExists = info.languageServiceHost.fileExists
      ? info.languageServiceHost.fileExists.bind(info.languageServiceHost)
      : tsModule.sys.fileExists;

    info.languageServiceHost.fileExists = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName)) {
          return true; // Virtual file exists in our cache
        }
        return originalFileExists(fileName);
      } catch (e) {
        log(
          `Error in fileExists: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalFileExists(fileName);
      }
    };

    /**
     * Hook: getScriptSnapshot (CRITICAL)
     *
     * This is the most important hook - it intercepts file content requests
     * and returns macro-expanded code instead of the original source.
     *
     * The hook handles several scenarios:
     * 1. Virtual .d.ts files - Returns the generated type declarations
     * 2. Reentrancy - Returns original content if file is already being processed
     * 3. Excluded files - Returns original content for node_modules, etc.
     * 4. Non-macro files - Returns original content if no @derive directives
     * 5. Macro files - Returns expanded content with generated methods
     *
     * @remarks
     * Caching strategy:
     * - Uses `snapshotCache` for identity stability (TS incremental compiler needs this)
     * - Uses `processingFiles` Set to prevent infinite loops during expansion
     * - Version-based cache invalidation ensures fresh expansions on file changes
     */
    const originalGetScriptSnapshot =
      info.languageServiceHost.getScriptSnapshot.bind(info.languageServiceHost);

    info.languageServiceHost.getScriptSnapshot = (fileName) => {
      try {
        log(`getScriptSnapshot: ${fileName}`);

        // Scenario 1: Virtual .d.ts file - return from our cache
        if (virtualDtsFiles.has(fileName)) {
          log(`  -> virtual .d.ts cache hit`);
          return virtualDtsFiles.get(fileName);
        }

        // Scenario 2: Reentrancy guard - prevent infinite loops
        if (processingFiles.has(fileName)) {
          log(`  -> REENTRANCY DETECTED, returning original`);
          return originalGetScriptSnapshot(fileName);
        }

        // Scenario 3: Excluded file (node_modules, .macroforge, wrong extension)
        if (!shouldProcess(fileName)) {
          log(`  -> not processable (excluded file), returning original`);
          return originalGetScriptSnapshot(fileName);
        }

        const snapshot = originalGetScriptSnapshot(fileName);
        if (!snapshot) {
          // Avoid tsserver crashes when a file was reported but no snapshot exists
          log(
            `  -> no snapshot available for ${fileName}, returning empty snapshot`,
          );
          return tsModule.ScriptSnapshot.fromString("");
        }

        const text = snapshot.getText(0, snapshot.getLength());

        // Scenario 4: No macro directives - return original
        if (!hasMacroDirectives(text)) {
          log(`  -> no macro directives, returning original`);
          return snapshot;
        }

        // Scenario 5: Has macros - expand and return
        log(`  -> has @derive, expanding...`);
        processingFiles.add(fileName);
        try {
          const version = info.languageServiceHost.getScriptVersion(fileName);
          log(`  -> version: ${version}`);

          // Check snapshot cache for stable identity
          const cached = snapshotCache.get(fileName);
          if (cached && cached.version === version) {
            log(`  -> snapshot cache hit`);
            return cached.snapshot;
          }

          const { code } = processFile(fileName, text, version);
          log(`  -> processFile returned`);

          if (code && code !== text) {
            log(`  -> creating expanded snapshot (${code.length} chars)`);
            const expandedSnapshot = tsModule.ScriptSnapshot.fromString(code);
            // Cache for stable identity across TS requests
            snapshotCache.set(fileName, {
              version,
              snapshot: expandedSnapshot,
            });
            log(`  -> returning expanded snapshot`);
            return expandedSnapshot;
          }

          // No change after expansion - cache original
          snapshotCache.set(fileName, { version, snapshot });
          return snapshot;
        } finally {
          processingFiles.delete(fileName);
        }
      } catch (e) {
        log(
          `ERROR in getScriptSnapshot for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        processingFiles.delete(fileName);
        return originalGetScriptSnapshot(fileName);
      }
    };

    // =========================================================================
    // DIAGNOSTIC HELPER FUNCTIONS
    // These utilities convert and map diagnostics between expanded and original
    // code positions.
    // =========================================================================

    /**
     * Converts a TypeScript diagnostic to a plain object for the native plugin.
     *
     * The native Rust plugin expects a simplified diagnostic format. This function
     * extracts the essential fields and normalizes the message text (which can be
     * either a string or a DiagnosticMessageChain).
     *
     * @param diag - The TypeScript diagnostic to convert
     * @returns A plain object with diagnostic information
     */
    function toPlainDiagnostic(diag: ts.Diagnostic): {
      start?: number;
      length?: number;
      message?: string;
      code?: number;
      category?: string;
    } {
      const message =
        typeof diag.messageText === "string"
          ? diag.messageText
          : diag.messageText.messageText;
      const category =
        diag.category === tsModule.DiagnosticCategory.Error
          ? "error"
          : diag.category === tsModule.DiagnosticCategory.Warning
            ? "warning"
            : "message";

      return {
        start: diag.start,
        length: diag.length,
        message,
        code: diag.code,
        category,
      };
    }

    /**
     * Applies mapped positions to diagnostics, updating their start/length.
     *
     * Takes the original diagnostics and a parallel array of mapped positions
     * (from the native plugin) and creates new diagnostics with corrected positions
     * pointing to the original source instead of the expanded code.
     *
     * @param original - The original diagnostics from TypeScript
     * @param mapped - Array of mapped positions (parallel to original)
     * @returns New diagnostic array with corrected positions
     */
    function applyMappedDiagnostics(
      original: readonly ts.Diagnostic[],
      mapped: Array<{ start?: number; length?: number }>,
    ): ts.Diagnostic[] {
      return original.map((diag, idx) => {
        const mappedDiag = mapped[idx];
        if (
          !mappedDiag ||
          mappedDiag.start === undefined ||
          mappedDiag.length === undefined
        ) {
          return diag; // No mapping available, keep original
        }

        return {
          ...diag,
          start: mappedDiag.start,
          length: mappedDiag.length,
        };
      });
    }

    // =========================================================================
    // DIAGNOSTIC HOOKS
    // These hooks map error positions from expanded code back to original source
    // and inject macro-specific diagnostics.
    // =========================================================================

    /**
     * Hook: getSemanticDiagnostics (COMPLEX)
     *
     * This is one of the most complex hooks. It handles:
     * 1. Mapping TypeScript error positions from expanded code back to original
     * 2. Converting errors in generated code to point at the responsible @derive macro
     * 3. Injecting Macroforge-specific diagnostics (expansion errors, warnings)
     *
     * The hook uses sophisticated position mapping to ensure errors appear at
     * meaningful locations in the user's source code, even when the actual error
     * occurred in macro-generated code.
     */
    const originalGetSemanticDiagnostics =
      info.languageService.getSemanticDiagnostics.bind(info.languageService);

    info.languageService.getSemanticDiagnostics = (fileName) => {
      try {
        log(`getSemanticDiagnostics: ${fileName}`);

        // If it's one of our virtual .d.ts files, don't get diagnostics for it
        if (virtualDtsFiles.has(fileName)) {
          log(`  -> skipping virtual .d.ts`);
          return [];
        }

        if (!shouldProcess(fileName)) {
          log(`  -> not processable, using original`);
          return originalGetSemanticDiagnostics(fileName);
        }

        log(`  -> getting original diagnostics...`);
        const expandedDiagnostics = originalGetSemanticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics`);

        // Map diagnostics using mapper
        const effectiveMapper = nativePlugin.getMapper(fileName);
        let mappedDiagnostics: ts.Diagnostic[];

        // Collect diagnostics in generated code to report them at decorator positions
        const generatedCodeDiagnostics: ts.Diagnostic[] = [];

        if (effectiveMapper && !effectiveMapper.isEmpty()) {
          log(`  -> mapping diagnostics with mapper`);
          mappedDiagnostics = expandedDiagnostics
            .map((diag) => {
              if (diag.start === undefined || diag.length === undefined) {
                return diag;
              }
              const mapped = effectiveMapper!.mapSpanToOriginal(
                diag.start,
                diag.length,
              );
              if (!mapped) {
                // Diagnostic is in generated code - check if we should convert it
                if (effectiveMapper!.isInGenerated(diag.start)) {
                  // This is an error in macro-generated code
                  // Collect it to report at decorator position
                  const macroName = effectiveMapper!.generatedBy(diag.start);
                  log(
                    `  -> collecting diagnostic in generated code (macro: ${macroName}): "${diag.messageText}"`,
                  );
                  generatedCodeDiagnostics.push(diag);
                  return null;
                }
                return diag;
              }
              return {
                ...diag,
                start: mapped.start,
                length: mapped.length,
              };
            })
            .filter((diag): diag is ts.Diagnostic => diag !== null);
        } else {
          // Native plugin is guaranteed to exist after early return check
          log(`  -> mapping diagnostics in native plugin`);
          mappedDiagnostics = applyMappedDiagnostics(
            expandedDiagnostics,
            nativePlugin.mapDiagnostics(
              fileName,
              expandedDiagnostics.map(toPlainDiagnostic),
            ),
          );
        }

        // Get macro diagnostics from Rust (hits cache if already expanded)
        const snapshot = originalGetScriptSnapshot(fileName);
        if (!snapshot) {
          return mappedDiagnostics;
        }

        const text = snapshot.getText(0, snapshot.getLength());
        const version = info.languageServiceHost.getScriptVersion(fileName);
        const { result } = processFile(fileName, text, version);

        // Convert diagnostics from generated code to macro diagnostics
        // pointing to the specific macro name within the decorator
        const generatedDiagsAsMacro: ts.Diagnostic[] = [];
        if (generatedCodeDiagnostics.length > 0 && result.sourceMapping) {
          // Find all @derive decorators with their macro arguments
          const deriveRegex = /@derive\s*\(([^)]*)\)/g;
          const deriveDecorators: Array<{
            fullStart: number;
            fullLength: number;
            macros: Array<{ name: string; start: number; length: number }>;
          }> = [];

          let match;
          while ((match = deriveRegex.exec(text)) !== null) {
            const fullStart = match.index;
            const fullLength = match[0].length;
            const argsStart = match.index + match[0].indexOf("(") + 1;
            const argsText = match[1];

            // Parse individual macro names from the arguments
            const macros: Array<{
              name: string;
              start: number;
              length: number;
            }> = [];
            const macroNameRegex = /([A-Za-z_][A-Za-z0-9_]*)/g;
            let macroMatch;
            while ((macroMatch = macroNameRegex.exec(argsText)) !== null) {
              macros.push({
                name: macroMatch[1],
                start: argsStart + macroMatch.index,
                length: macroMatch[1].length,
              });
            }

            deriveDecorators.push({ fullStart, fullLength, macros });
          }

          // Helper to find the specific macro position for a given expanded position
          const findMacroPosition = (
            expandedPos: number,
            macroName: string,
          ): { start: number; length: number } => {
            // Find the generated region containing this position
            const region = result.sourceMapping!.generatedRegions.find(
              (r) => expandedPos >= r.start && expandedPos < r.end,
            );

            if (!region) {
              // Fallback to first decorator
              const firstDec = deriveDecorators[0];
              if (firstDec) {
                const macro = firstDec.macros.find((m) => m.name === macroName);
                if (macro) return { start: macro.start, length: macro.length };
                return {
                  start: firstDec.fullStart,
                  length: firstDec.fullLength,
                };
              }
              return { start: 0, length: 7 };
            }

            // Find the segment that ends right before this generated region
            const segments = result.sourceMapping!.segments;
            let insertionPointInOriginal = 0;

            for (const seg of segments) {
              if (seg.expandedEnd <= region.start) {
                insertionPointInOriginal = seg.originalEnd;
              }
            }

            // Find the nearest @derive decorator before this insertion point
            let nearestDecorator = deriveDecorators[0];
            for (const dec of deriveDecorators) {
              if (dec.fullStart < insertionPointInOriginal) {
                nearestDecorator = dec;
              } else {
                break;
              }
            }

            if (nearestDecorator) {
              // Find the specific macro within this decorator
              const macro = nearestDecorator.macros.find(
                (m) => m.name === macroName,
              );
              if (macro) {
                return { start: macro.start, length: macro.length };
              }
              // If macro name is "macro" (generic fallback) and there's exactly one macro,
              // use that macro's position
              if (
                (macroName === "macro" || macroName === "") &&
                nearestDecorator.macros.length === 1
              ) {
                const onlyMacro = nearestDecorator.macros[0];
                return { start: onlyMacro.start, length: onlyMacro.length };
              }
              // Fallback to full decorator if macro not found
              return {
                start: nearestDecorator.fullStart,
                length: nearestDecorator.fullLength,
              };
            }

            return { start: 0, length: 7 };
          };

          for (const diag of generatedCodeDiagnostics) {
            const diagStart = diag.start ?? 0;

            // Try to get the macro name from the mapper or from the generated region
            let macroName = effectiveMapper?.generatedBy(diagStart) ?? null;

            // If mapper didn't return a name, try to get it from the generated region
            if (!macroName) {
              const region = result.sourceMapping!.generatedRegions.find(
                (r) => diagStart >= r.start && diagStart < r.end,
              );
              macroName = region?.sourceMacro ?? "macro";
            }

            // Extract just the macro name if it contains a path (e.g., "derive::Debug" -> "Debug")
            const simpleMacroName = macroName.includes("::")
              ? (macroName.split("::").pop() ?? macroName)
              : macroName;

            log(
              `  -> diagnostic at ${diagStart}, macroName="${macroName}", simpleMacroName="${simpleMacroName}"`,
            );
            log(
              `  -> generatedRegions: ${JSON.stringify(result.sourceMapping!.generatedRegions)}`,
            );
            log(
              `  -> deriveDecorators: ${JSON.stringify(deriveDecorators.map((d) => ({ fullStart: d.fullStart, macros: d.macros })))}`,
            );

            const position = findMacroPosition(diagStart, simpleMacroName);
            log(`  -> resolved position: ${JSON.stringify(position)}`);

            generatedDiagsAsMacro.push({
              file: info.languageService.getProgram()?.getSourceFile(fileName),
              start: position.start,
              length: position.length,
              messageText: `[${simpleMacroName}] ${typeof diag.messageText === "string" ? diag.messageText : diag.messageText.messageText}`,
              category: diag.category,
              code: 9998, // Different code for generated code errors
              source: "macroforge-generated",
            });
          }
          log(
            `  -> converted ${generatedDiagsAsMacro.length} generated code diagnostics`,
          );
        } else if (generatedCodeDiagnostics.length > 0) {
          // Fallback when no source mapping available
          const deriveMatch = text.match(/@derive\s*\(/);
          const decoratorStart = deriveMatch?.index ?? 0;
          const decoratorLength = deriveMatch?.[0].length ?? 7;

          for (const diag of generatedCodeDiagnostics) {
            const macroName =
              effectiveMapper?.generatedBy(diag.start ?? 0) ?? "macro";
            generatedDiagsAsMacro.push({
              file: info.languageService.getProgram()?.getSourceFile(fileName),
              start: decoratorStart,
              length: decoratorLength,
              messageText: `[${macroName}] ${typeof diag.messageText === "string" ? diag.messageText : diag.messageText.messageText}`,
              category: diag.category,
              code: 9998, // Different code for generated code errors
              source: "macroforge-generated",
            });
          }
          log(
            `  -> converted ${generatedDiagsAsMacro.length} generated code diagnostics (fallback)`,
          );
        }

        if (!result.diagnostics || result.diagnostics.length === 0) {
          return [...mappedDiagnostics, ...generatedDiagsAsMacro];
        }

        const macroDiagnostics: ts.Diagnostic[] = result.diagnostics.map(
          (d) => {
            const category =
              d.level === "error"
                ? tsModule.DiagnosticCategory.Error
                : d.level === "warning"
                  ? tsModule.DiagnosticCategory.Warning
                  : tsModule.DiagnosticCategory.Message;

            return {
              file: info.languageService.getProgram()?.getSourceFile(fileName),
              start: d.start || 0,
              length: (d.end || 0) - (d.start || 0),
              messageText: d.message,
              category,
              code: 9999, // Custom error code
              source: "macroforge",
            };
          },
        );

        return [
          ...mappedDiagnostics,
          ...macroDiagnostics,
          ...generatedDiagsAsMacro,
        ];
      } catch (e) {
        log(
          `Error in getSemanticDiagnostics for ${fileName}: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        return originalGetSemanticDiagnostics(fileName);
      }
    };

    /**
     * Hook: getSyntacticDiagnostics
     *
     * Maps syntax error positions from expanded code back to original source.
     * Simpler than semantic diagnostics as it doesn't need to handle generated
     * code errors (syntax errors are in user code, not generated code).
     */
    const originalGetSyntacticDiagnostics =
      info.languageService.getSyntacticDiagnostics.bind(info.languageService);

    info.languageService.getSyntacticDiagnostics = (fileName) => {
      try {
        log(`getSyntacticDiagnostics: ${fileName}`);

        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          log(`  -> using original`);
          return originalGetSyntacticDiagnostics(fileName);
        }

        // Ensure mapper ready
        nativePlugin.getMapper(fileName);

        const expandedDiagnostics = originalGetSyntacticDiagnostics(fileName);
        log(`  -> got ${expandedDiagnostics.length} diagnostics, mapping...`);
        // Native plugin is guaranteed to exist after early return check
        const result = applyMappedDiagnostics(
          expandedDiagnostics,
          nativePlugin.mapDiagnostics(
            fileName,
            expandedDiagnostics.map(toPlainDiagnostic),
          ),
        ) as ts.DiagnosticWithLocation[];
        log(`  -> returning ${result.length} mapped diagnostics`);
        return result;
      } catch (e) {
        log(
          `ERROR in getSyntacticDiagnostics: ${e instanceof Error ? e.stack || e.message : String(e)}`,
        );
        return originalGetSyntacticDiagnostics(fileName);
      }
    };

    // =========================================================================
    // NAVIGATION & IDE FEATURE HOOKS
    // These hooks provide IDE features like hover, completions, go-to-definition,
    // find references, rename, etc. All follow a similar pattern:
    // 1. Map input position from original to expanded coordinates
    // 2. Call the original method on expanded code
    // 3. Map output positions back from expanded to original coordinates
    // =========================================================================

    /**
     * Hook: getQuickInfoAtPosition
     *
     * Provides hover information for symbols. This hook has special handling
     * for Macroforge-specific syntax:
     *
     * 1. First checks for macro hover info (@derive macros, field decorators)
     * 2. If not on a macro, maps position and delegates to TypeScript
     * 3. Maps result spans back to original positions
     *
     * @remarks
     * If the hover would be in generated code, returns undefined to hide it
     * (prevents confusing users with hover info for code they can't see).
     */
    const originalGetQuickInfoAtPosition =
      info.languageService.getQuickInfoAtPosition.bind(info.languageService);

    info.languageService.getQuickInfoAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetQuickInfoAtPosition(fileName, position);
        }

        // Check for macro hover first (JSDoc @derive comments and decorators)
        // Use the *original* snapshot for macro/decorator hover detection.
        // The plugin's host hook returns expanded code where macro directives
        // are stripped by default, which would make macro hover impossible.
        const snapshot = originalGetScriptSnapshot(fileName);
        if (snapshot) {
          const text = snapshot.getText(0, snapshot.getLength());
          const macroHover = getMacroHoverInfo(text, position, tsModule);
          if (macroHover) {
            return macroHover;
          }
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetQuickInfoAtPosition(fileName, position);
        }
        // Map original position to expanded
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetQuickInfoAtPosition(fileName, expandedPos);

        if (!result) return result;

        // Map result spans back to original
        const mappedTextSpan = mapper.mapSpanToOriginal(
          result.textSpan.start,
          result.textSpan.length,
        );
        if (!mappedTextSpan) return undefined; // In generated code - hide hover

        return {
          ...result,
          textSpan: {
            start: mappedTextSpan.start,
            length: mappedTextSpan.length,
          },
        };
      } catch (e) {
        log(
          `Error in getQuickInfoAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetQuickInfoAtPosition(fileName, position);
      }
    };

    /**
     * Hook: getCompletionsAtPosition
     *
     * Provides IntelliSense completions. Maps the cursor position to expanded
     * coordinates to get accurate completions that include generated methods,
     * then maps any replacement spans back to original coordinates.
     */
    const originalGetCompletionsAtPosition =
      info.languageService.getCompletionsAtPosition.bind(info.languageService);

    info.languageService.getCompletionsAtPosition = (
      fileName,
      position,
      options,
      formattingSettings,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetCompletionsAtPosition(
            fileName,
            position,
            options,
            formattingSettings,
          );
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetCompletionsAtPosition(
            fileName,
            position,
            options,
            formattingSettings,
          );
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetCompletionsAtPosition(
          fileName,
          expandedPos,
          options,
          formattingSettings,
        );

        if (!result) return result;

        // Map optionalReplacementSpan if present
        let mappedOptionalSpan = undefined;
        if (result.optionalReplacementSpan) {
          const mapped = mapper.mapSpanToOriginal(
            result.optionalReplacementSpan.start,
            result.optionalReplacementSpan.length,
          );
          if (mapped) {
            mappedOptionalSpan = { start: mapped.start, length: mapped.length };
          }
        }

        // Map entries replacementSpan
        const mappedEntries = result.entries.map((entry) => {
          if (!entry.replacementSpan) return entry;
          const mapped = mapper.mapSpanToOriginal(
            entry.replacementSpan.start,
            entry.replacementSpan.length,
          );
          if (!mapped) return { ...entry, replacementSpan: undefined }; // Remove invalid span
          return {
            ...entry,
            replacementSpan: { start: mapped.start, length: mapped.length },
          };
        });

        return {
          ...result,
          optionalReplacementSpan: mappedOptionalSpan,
          entries: mappedEntries,
        };
      } catch (e) {
        log(
          `Error in getCompletionsAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetCompletionsAtPosition(
          fileName,
          position,
          options,
          formattingSettings,
        );
      }
    };

    /**
     * Hook: getDefinitionAtPosition
     *
     * Provides "Go to Definition" functionality. Maps cursor position to
     * expanded code, gets definitions, then maps definition spans back
     * to original positions.
     *
     * @remarks
     * For definitions in other files (not macro-expanded), positions are
     * passed through unchanged. Only same-file definitions need mapping.
     * Definitions pointing to generated code are filtered out.
     */
    const originalGetDefinitionAtPosition =
      info.languageService.getDefinitionAtPosition.bind(info.languageService);

    info.languageService.getDefinitionAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDefinitionAtPosition(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetDefinitionAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const definitions = originalGetDefinitionAtPosition(
          fileName,
          expandedPos,
        );

        if (!definitions) return definitions;

        // Map each definition's span back to original (only for same file)
        return definitions.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = nativePlugin.getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);
      } catch (e) {
        log(
          `Error in getDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDefinitionAtPosition(fileName, position);
      }
    };

    /**
     * Hook: getDefinitionAndBoundSpan
     *
     * Enhanced version of getDefinitionAtPosition that also returns the
     * text span that was used to find the definition (useful for highlighting).
     * Maps both the bound span and definition spans.
     */
    const originalGetDefinitionAndBoundSpan =
      info.languageService.getDefinitionAndBoundSpan.bind(info.languageService);

    info.languageService.getDefinitionAndBoundSpan = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDefinitionAndBoundSpan(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetDefinitionAndBoundSpan(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetDefinitionAndBoundSpan(fileName, expandedPos);

        if (!result) return result;

        // Map textSpan back to original
        const mappedTextSpan = mapper.mapSpanToOriginal(
          result.textSpan.start,
          result.textSpan.length,
        );
        if (!mappedTextSpan) return undefined; // In generated code

        // Map each definition's span
        const mappedDefinitions = result.definitions?.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = nativePlugin.getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);

        return {
          textSpan: {
            start: mappedTextSpan.start,
            length: mappedTextSpan.length,
          },
          definitions: mappedDefinitions,
        };
      } catch (e) {
        log(
          `Error in getDefinitionAndBoundSpan: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDefinitionAndBoundSpan(fileName, position);
      }
    };

    /**
     * Hook: getTypeDefinitionAtPosition
     *
     * Provides "Go to Type Definition" functionality. Similar to
     * getDefinitionAtPosition but navigates to the type's definition
     * rather than the symbol's definition.
     */
    const originalGetTypeDefinitionAtPosition =
      info.languageService.getTypeDefinitionAtPosition.bind(
        info.languageService,
      );

    info.languageService.getTypeDefinitionAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetTypeDefinitionAtPosition(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetTypeDefinitionAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const definitions = originalGetTypeDefinitionAtPosition(
          fileName,
          expandedPos,
        );

        if (!definitions) return definitions;

        return definitions.reduce((acc, def) => {
          if (def.fileName !== fileName) {
            acc.push(def);
            return acc;
          }
          const defMapper = nativePlugin.getMapper(def.fileName);
          if (!defMapper) {
            acc.push(def);
            return acc;
          }
          const mapped = defMapper.mapSpanToOriginal(
            def.textSpan.start,
            def.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...def,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.DefinitionInfo[]);
      } catch (e) {
        log(
          `Error in getTypeDefinitionAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetTypeDefinitionAtPosition(fileName, position);
      }
    };

    /**
     * Hook: getReferencesAtPosition
     *
     * Provides "Find All References" functionality. Maps the cursor position,
     * finds all references in the expanded code, then maps each reference
     * span back to original positions. References in generated code are filtered.
     */
    const originalGetReferencesAtPosition =
      info.languageService.getReferencesAtPosition.bind(info.languageService);

    info.languageService.getReferencesAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetReferencesAtPosition(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetReferencesAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const refs = originalGetReferencesAtPosition(fileName, expandedPos);

        if (!refs) return refs;

        return refs.reduce((acc, ref) => {
          if (!shouldProcess(ref.fileName)) {
            acc.push(ref);
            return acc;
          }
          const refMapper = nativePlugin.getMapper(ref.fileName);
          if (!refMapper) {
            acc.push(ref);
            return acc;
          }
          const mapped = refMapper.mapSpanToOriginal(
            ref.textSpan.start,
            ref.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...ref,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.ReferenceEntry[]);
      } catch (e) {
        log(
          `Error in getReferencesAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetReferencesAtPosition(fileName, position);
      }
    };

    /**
     * Hook: findReferences
     *
     * Alternative "Find All References" that returns grouped references by symbol.
     * Similar to getReferencesAtPosition but with richer structure.
     */
    const originalFindReferences = info.languageService.findReferences.bind(
      info.languageService,
    );

    info.languageService.findReferences = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalFindReferences(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalFindReferences(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const refSymbols = originalFindReferences(fileName, expandedPos);

        if (!refSymbols) return refSymbols;

        return refSymbols
          .map((refSymbol) => ({
            ...refSymbol,
            references: refSymbol.references.reduce((acc, ref) => {
              if (!shouldProcess(ref.fileName)) {
                acc.push(ref);
                return acc;
              }
              const refMapper = nativePlugin.getMapper(ref.fileName);
              if (!refMapper) {
                acc.push(ref);
                return acc;
              }
              const mapped = refMapper.mapSpanToOriginal(
                ref.textSpan.start,
                ref.textSpan.length,
              );
              if (mapped) {
                acc.push({
                  ...ref,
                  textSpan: { start: mapped.start, length: mapped.length },
                });
              }
              return acc;
            }, [] as ts.ReferenceEntry[]),
          }))
          .filter((s) => s.references.length > 0);
      } catch (e) {
        log(
          `Error in findReferences: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalFindReferences(fileName, position);
      }
    };

    /**
     * Hook: getSignatureHelpItems
     *
     * Provides function signature help (parameter hints shown while typing
     * function arguments). Maps cursor position and the applicable span.
     */
    const originalGetSignatureHelpItems =
      info.languageService.getSignatureHelpItems.bind(info.languageService);

    info.languageService.getSignatureHelpItems = (
      fileName,
      position,
      options,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetSignatureHelpItems(fileName, position, options);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetSignatureHelpItems(fileName, position, options);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = originalGetSignatureHelpItems(
          fileName,
          expandedPos,
          options,
        );

        if (!result) return result;

        // Map applicableSpan back to original
        const mappedSpan = mapper.mapSpanToOriginal(
          result.applicableSpan.start,
          result.applicableSpan.length,
        );
        if (!mappedSpan) return undefined;

        return {
          ...result,
          applicableSpan: {
            start: mappedSpan.start,
            length: mappedSpan.length,
          },
        };
      } catch (e) {
        log(
          `Error in getSignatureHelpItems: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetSignatureHelpItems(fileName, position, options);
      }
    };

    /**
     * Hook: getRenameInfo
     *
     * Provides information about whether a symbol can be renamed and what
     * text span should be highlighted. Returns an error message if the
     * cursor is in generated code (can't rename generated symbols).
     *
     * @remarks
     * Uses a compatibility wrapper (callGetRenameInfo) to handle different
     * TypeScript version signatures for this method.
     */
    const originalGetRenameInfo = (
      info.languageService.getRenameInfo as any
    ).bind(info.languageService);

    /** Options for getRenameInfo - varies by TypeScript version */
    type RenameInfoOptions = {
      allowRenameOfImportPath?: boolean;
    };

    /**
     * Compatibility wrapper for getRenameInfo that handles both old and new
     * TypeScript API signatures.
     */
    const callGetRenameInfo = (
      fileName: string,
      position: number,
      options?: RenameInfoOptions,
    ) => {
      // Prefer object overload if available; otherwise fall back to legacy args
      if ((originalGetRenameInfo as any).length <= 2) {
        return (originalGetRenameInfo as any)(fileName, position, options);
      }
      return (originalGetRenameInfo as any)(
        fileName,
        position,
        options?.allowRenameOfImportPath,
      );
    };

    info.languageService.getRenameInfo = (fileName, position, options) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return callGetRenameInfo(fileName, position, options as any);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return callGetRenameInfo(fileName, position, options as any);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const result = callGetRenameInfo(fileName, expandedPos, options as any);

        if (!result.canRename || !result.triggerSpan) return result;

        const mappedSpan = mapper.mapSpanToOriginal(
          result.triggerSpan.start,
          result.triggerSpan.length,
        );
        if (!mappedSpan) {
          return {
            canRename: false,
            localizedErrorMessage: "Cannot rename in generated code",
          };
        }

        return {
          ...result,
          triggerSpan: { start: mappedSpan.start, length: mappedSpan.length },
        };
      } catch (e) {
        log(
          `Error in getRenameInfo: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetRenameInfo(fileName, position, options);
      }
    };

    /**
     * Hook: findRenameLocations
     *
     * Finds all locations that would be affected by a rename operation.
     * Maps each location's span back to original positions. Locations in
     * generated code are filtered out.
     *
     * @remarks
     * Uses a compatibility wrapper (callFindRenameLocations) to handle
     * different TypeScript version signatures.
     */
    const originalFindRenameLocations =
      info.languageService.findRenameLocations.bind(info.languageService);

    /** Options for findRenameLocations - varies by TypeScript version */
    type RenameLocationOptions = {
      findInStrings?: boolean;
      findInComments?: boolean;
      providePrefixAndSuffixTextForRename?: boolean;
    };

    /**
     * Compatibility wrapper for findRenameLocations that handles both old
     * and new TypeScript API signatures.
     */
    const callFindRenameLocations = (
      fileName: string,
      position: number,
      opts?: RenameLocationOptions,
    ) => {
      // Prefer object overload if available; otherwise fall back to legacy args
      if ((originalFindRenameLocations as any).length <= 3) {
        return (originalFindRenameLocations as any)(fileName, position, opts);
      }
      return (originalFindRenameLocations as any)(
        fileName,
        position,
        !!opts?.findInStrings,
        !!opts?.findInComments,
        !!opts?.providePrefixAndSuffixTextForRename,
      );
    };

    (info.languageService as any).findRenameLocations = (
      fileName: string,
      position: number,
      options?: RenameLocationOptions,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return callFindRenameLocations(fileName, position, options);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return callFindRenameLocations(fileName, position, options);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const locations = callFindRenameLocations(
          fileName,
          expandedPos,
          options,
        );

        if (!locations) return locations;

        return locations.reduce(
          (acc: ts.RenameLocation[], loc: ts.RenameLocation) => {
            if (!shouldProcess(loc.fileName)) {
              acc.push(loc);
              return acc;
            }
            const locMapper = nativePlugin.getMapper(loc.fileName);
            if (!locMapper) {
              acc.push(loc);
              return acc;
            }
            const mapped = locMapper.mapSpanToOriginal(
              loc.textSpan.start,
              loc.textSpan.length,
            );
            if (mapped) {
              acc.push({
                ...loc,
                textSpan: { start: mapped.start, length: mapped.length },
              });
            }
            return acc;
          },
          [] as ts.RenameLocation[],
        );
      } catch (e) {
        log(
          `Error in findRenameLocations: ${e instanceof Error ? e.message : String(e)}`,
        );
        return callFindRenameLocations(fileName, position, options);
      }
    };

    /**
     * Hook: getDocumentHighlights
     *
     * Highlights all occurrences of a symbol in the document (used when you
     * click on a variable and see all usages highlighted). Maps highlight
     * spans back to original positions.
     */
    const originalGetDocumentHighlights =
      info.languageService.getDocumentHighlights.bind(info.languageService);

    info.languageService.getDocumentHighlights = (
      fileName,
      position,
      filesToSearch,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetDocumentHighlights(
            fileName,
            position,
            filesToSearch,
          );
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetDocumentHighlights(
            fileName,
            position,
            filesToSearch,
          );
        }
        const expandedPos = mapper.originalToExpanded(position);
        const highlights = originalGetDocumentHighlights(
          fileName,
          expandedPos,
          filesToSearch,
        );

        if (!highlights) return highlights;

        return highlights
          .map((docHighlight) => ({
            ...docHighlight,
            highlightSpans: docHighlight.highlightSpans.reduce((acc, span) => {
              if (!shouldProcess(docHighlight.fileName)) {
                acc.push(span);
                return acc;
              }
              const spanMapper = nativePlugin.getMapper(docHighlight.fileName);
              if (!spanMapper) {
                acc.push(span);
                return acc;
              }
              const mapped = spanMapper.mapSpanToOriginal(
                span.textSpan.start,
                span.textSpan.length,
              );
              if (mapped) {
                acc.push({
                  ...span,
                  textSpan: { start: mapped.start, length: mapped.length },
                });
              }
              return acc;
            }, [] as ts.HighlightSpan[]),
          }))
          .filter((h) => h.highlightSpans.length > 0);
      } catch (e) {
        log(
          `Error in getDocumentHighlights: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetDocumentHighlights(fileName, position, filesToSearch);
      }
    };

    /**
     * Hook: getImplementationAtPosition
     *
     * Provides "Go to Implementation" functionality. Similar to definition
     * but finds concrete implementations of abstract methods/interfaces.
     */
    const originalGetImplementationAtPosition =
      info.languageService.getImplementationAtPosition.bind(
        info.languageService,
      );

    info.languageService.getImplementationAtPosition = (fileName, position) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetImplementationAtPosition(fileName, position);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetImplementationAtPosition(fileName, position);
        }
        const expandedPos = mapper.originalToExpanded(position);
        const implementations = originalGetImplementationAtPosition(
          fileName,
          expandedPos,
        );

        if (!implementations) return implementations;

        return implementations.reduce((acc, impl) => {
          if (!shouldProcess(impl.fileName)) {
            acc.push(impl);
            return acc;
          }
          const implMapper = nativePlugin.getMapper(impl.fileName);
          if (!implMapper) {
            acc.push(impl);
            return acc;
          }
          const mapped = implMapper.mapSpanToOriginal(
            impl.textSpan.start,
            impl.textSpan.length,
          );
          if (mapped) {
            acc.push({
              ...impl,
              textSpan: { start: mapped.start, length: mapped.length },
            });
          }
          return acc;
        }, [] as ts.ImplementationLocation[]);
      } catch (e) {
        log(
          `Error in getImplementationAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetImplementationAtPosition(fileName, position);
      }
    };
    /**
     * Hook: getCodeFixesAtPosition
     *
     * Provides quick fix suggestions for errors at a position. Maps the
     * input span to expanded coordinates to get fixes that work with
     * generated code context.
     *
     * @remarks
     * Note: The returned fixes may include edits to expanded code, which
     * could be problematic. Consider filtering or mapping fix edits in
     * future versions.
     */
    const originalGetCodeFixesAtPosition =
      info.languageService.getCodeFixesAtPosition.bind(info.languageService);

    info.languageService.getCodeFixesAtPosition = (
      fileName,
      start,
      end,
      errorCodes,
      formatOptions,
      preferences,
    ) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetCodeFixesAtPosition(
            fileName,
            start,
            end,
            errorCodes,
            formatOptions,
            preferences,
          );
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetCodeFixesAtPosition(
            fileName,
            start,
            end,
            errorCodes,
            formatOptions,
            preferences,
          );
        }
        const expandedStart = mapper.originalToExpanded(start);
        const expandedEnd = mapper.originalToExpanded(end);
        return originalGetCodeFixesAtPosition(
          fileName,
          expandedStart,
          expandedEnd,
          errorCodes,
          formatOptions,
          preferences,
        );
      } catch (e) {
        log(
          `Error in getCodeFixesAtPosition: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetCodeFixesAtPosition(
          fileName,
          start,
          end,
          errorCodes,
          formatOptions,
          preferences,
        );
      }
    };

    /**
     * Hook: getNavigationTree
     *
     * Provides the document outline/structure tree (shown in the Outline
     * panel). Recursively maps all spans in the tree back to original positions.
     */
    const originalGetNavigationTree =
      info.languageService.getNavigationTree.bind(info.languageService);

    info.languageService.getNavigationTree = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetNavigationTree(fileName);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetNavigationTree(fileName);
        }
        const navMapper = mapper;
        const tree = originalGetNavigationTree(fileName);

        // Recursively map spans in navigation tree
        function mapNavigationItem(item: ts.NavigationTree): ts.NavigationTree {
          const mappedSpans = item.spans.map((span) => {
            const mapped = navMapper.mapSpanToOriginal(span.start, span.length);
            return mapped
              ? { start: mapped.start, length: mapped.length }
              : span;
          });

          const mappedNameSpan = item.nameSpan
            ? (navMapper.mapSpanToOriginal(
                item.nameSpan.start,
                item.nameSpan.length,
              ) ?? item.nameSpan)
            : undefined;

          return {
            ...item,
            spans: mappedSpans,
            nameSpan: mappedNameSpan
              ? { start: mappedNameSpan.start, length: mappedNameSpan.length }
              : undefined,
            childItems: item.childItems?.map(mapNavigationItem),
          };
        }

        return mapNavigationItem(tree);
      } catch (e) {
        log(
          `Error in getNavigationTree: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetNavigationTree(fileName);
      }
    };

    /**
     * Hook: getOutliningSpans
     *
     * Provides code folding regions. Maps both the text span (what gets
     * folded) and hint span (what's shown when collapsed) back to original.
     */
    const originalGetOutliningSpans =
      info.languageService.getOutliningSpans.bind(info.languageService);

    info.languageService.getOutliningSpans = (fileName) => {
      try {
        if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
          return originalGetOutliningSpans(fileName);
        }

        const mapper = nativePlugin.getMapper(fileName);
        if (!mapper) {
          return originalGetOutliningSpans(fileName);
        }
        const spans = originalGetOutliningSpans(fileName);

        return spans.map((span) => {
          const mappedTextSpan = mapper.mapSpanToOriginal(
            span.textSpan.start,
            span.textSpan.length,
          );
          const mappedHintSpan = mapper.mapSpanToOriginal(
            span.hintSpan.start,
            span.hintSpan.length,
          );

          if (!mappedTextSpan || !mappedHintSpan) return span;

          return {
            ...span,
            textSpan: {
              start: mappedTextSpan.start,
              length: mappedTextSpan.length,
            },
            hintSpan: {
              start: mappedHintSpan.start,
              length: mappedHintSpan.length,
            },
          };
        });
      } catch (e) {
        log(
          `Error in getOutliningSpans: ${e instanceof Error ? e.message : String(e)}`,
        );
        return originalGetOutliningSpans(fileName);
      }
    };

    /**
     * Hook: provideInlayHints
     *
     * Provides inlay hints (inline type annotations shown in the editor).
     * Maps the requested span to expanded coordinates, then maps each hint's
     * position back to original. Hints in generated code are filtered out.
     *
     * @remarks
     * This hook is conditional - provideInlayHints may not exist in older
     * TypeScript versions.
     */
    const originalProvideInlayHints =
      info.languageService.provideInlayHints?.bind(info.languageService);

    if (originalProvideInlayHints) {
      info.languageService.provideInlayHints = (
        fileName,
        span,
        preferences,
      ) => {
        try {
          if (virtualDtsFiles.has(fileName) || !shouldProcess(fileName)) {
            return originalProvideInlayHints(fileName, span, preferences);
          }

          const mapper = nativePlugin.getMapper(fileName);
          if (!mapper) {
            return originalProvideInlayHints(fileName, span, preferences);
          }
          // If no mapping info, avoid remapping to reduce risk
          if (mapper.isEmpty()) {
            return originalProvideInlayHints(fileName, span, preferences);
          }
          // Map the input span to expanded coordinates
          const expandedSpan = mapper.mapSpanToExpanded(
            span.start,
            span.length,
          );
          const result = originalProvideInlayHints(
            fileName,
            expandedSpan,
            preferences,
          );

          if (!result) return result;

          // Map each hint's position back to original coordinates
          return result.flatMap((hint) => {
            const originalPos = mapper.expandedToOriginal(hint.position);
            if (originalPos === null) {
              // Hint is in generated code, skip it
              return [];
            }
            return [
              {
                ...hint,
                position: originalPos,
              },
            ];
          });
        } catch (e) {
          log(
            `Error in provideInlayHints: ${e instanceof Error ? e.message : String(e)}`,
          );
          return originalProvideInlayHints(fileName, span, preferences);
        }
      };
    }

    return info.languageService;
  }

  return { create };
}

export = init;
