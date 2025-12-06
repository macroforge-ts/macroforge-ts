import type { SourceMappingResult } from "@macroforge/swc-napi";

export type SourceMapping = SourceMappingResult;

export interface PositionMapper {
  originalToExpanded(pos: number): number;
  expandedToOriginal(pos: number): number | null;
  mapSpanToOriginal(
    start: number,
    length: number,
  ): { start: number; length: number } | null;
  mapSpanToExpanded(
    start: number,
    length: number,
  ): { start: number; length: number };
  isEmpty(): boolean;
}
