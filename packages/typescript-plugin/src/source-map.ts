/**
 * @fileoverview Source mapping type definitions for the TypeScript plugin.
 *
 * This module re-exports and extends position mapping types from the core
 * macroforge package. These types are used to translate positions between
 * original source code and macro-expanded code.
 *
 * @module @macroforge/typescript-plugin/source-map
 */

import type { SourceMappingResult } from "macroforge";

/**
 * Re-export of the core source mapping result type.
 *
 * Contains the complete mapping data between original and expanded code,
 * including segment information and generated region metadata.
 */
export type SourceMapping = SourceMappingResult;

/**
 * Interface for mapping positions between original and expanded code.
 *
 * The TypeScript plugin uses this interface to translate cursor positions,
 * text spans, and other location data between the original source code
 * (what the user sees) and the expanded code (what TypeScript analyzes).
 *
 * @remarks
 * Position mapping is bidirectional:
 * - Original → Expanded: Used when sending positions TO TypeScript (cursor, selection)
 * - Expanded → Original: Used when receiving positions FROM TypeScript (errors, spans)
 *
 * @example
 * ```typescript
 * // Map a cursor position from original to expanded
 * const expandedPos = mapper.originalToExpanded(cursorPosition);
 *
 * // Map an error span from expanded back to original
 * const originalSpan = mapper.mapSpanToOriginal(error.start, error.length);
 * if (originalSpan) {
 *   // Error is in user code - show it at originalSpan position
 * } else {
 *   // Error is in generated code - show it at decorator position instead
 * }
 * ```
 */
export interface PositionMapper {
  /**
   * Maps a position from original source to expanded code.
   *
   * @param pos - Position in the original source (0-indexed character offset)
   * @returns Corresponding position in the expanded code
   */
  originalToExpanded(pos: number): number;

  /**
   * Maps a position from expanded code back to original source.
   *
   * @param pos - Position in the expanded code (0-indexed character offset)
   * @returns Corresponding position in the original source, or `null` if the
   *          position falls within generated code (no original equivalent)
   */
  expandedToOriginal(pos: number): number | null;

  /**
   * Maps a text span from expanded code back to original source.
   *
   * @param start - Start position in expanded code
   * @param length - Length of the span in expanded code
   * @returns Object with mapped start and length, or `null` if the span
   *          falls within generated code
   */
  mapSpanToOriginal(
    start: number,
    length: number,
  ): { start: number; length: number } | null;

  /**
   * Maps a text span from original source to expanded code.
   *
   * @param start - Start position in original source
   * @param length - Length of the span in original source
   * @returns Object with mapped start and length in expanded code
   */
  mapSpanToExpanded(
    start: number,
    length: number,
  ): { start: number; length: number };

  /**
   * Checks if this mapper has any mapping data.
   *
   * @returns `true` if no macro expansion occurred (original === expanded),
   *          `false` if there are position differences to account for
   */
  isEmpty(): boolean;
}
