/**
 * Source mapping between original and macro-expanded code.
 *
 * When macros expand code (e.g., inserting toString() method), byte positions shift.
 * This module provides bidirectional position mapping to translate between
 * original source positions and expanded code positions.
 */
/** A segment mapping a contiguous region from original to expanded positions. */
export interface MappingSegment {
    originalStart: number;
    originalEnd: number;
    expandedStart: number;
    expandedEnd: number;
}
/** A region of generated code that has no corresponding original source. */
export interface GeneratedRegion {
    start: number;
    end: number;
    /** Which macro generated this code (e.g., "Debug::toString", "JSON::toJSON") */
    sourceMacro: string;
}
/** Complete source mapping data from the native module. */
export interface SourceMapping {
    segments: MappingSegment[];
    generatedRegions: GeneratedRegion[];
}
/**
 * Bidirectional position mapper between original and expanded source code.
 *
 * Use this to translate positions when:
 * - Reporting diagnostics: expanded -> original (so errors point to user's source)
 * - Processing hover requests: original -> expanded (to find info in expanded code)
 * - Returning spans from LS methods: expanded -> original (so spans match user's view)
 */
export declare class PositionMapper {
    private segments;
    private generatedRegions;
    constructor(mapping: SourceMapping);
    /**
     * Map a position from original source to expanded source.
     * Used for: hover requests, completion requests, go-to-definition requests
     */
    originalToExpanded(pos: number): number;
    /**
     * Map a position from expanded source to original source.
     * Used for: diagnostic positions, definition spans, reference spans
     *
     * Returns null if the position is in generated code (has no original).
     */
    expandedToOriginal(pos: number): number | null;
    /**
     * Check if an expanded position is within generated code.
     */
    isInGenerated(expandedPos: number): boolean;
    /**
     * Get the macro that generated code at a position, if any.
     */
    generatedBy(expandedPos: number): string | null;
    /**
     * Map a span (start, length) from expanded to original coordinates.
     * Returns null if either endpoint is in generated code.
     */
    mapSpanToOriginal(start: number, length: number): {
        start: number;
        length: number;
    } | null;
    /**
     * Map a span (start, length) from original to expanded coordinates.
     */
    mapSpanToExpanded(start: number, length: number): {
        start: number;
        length: number;
    };
    /**
     * Check if the mapping has any data.
     */
    isEmpty(): boolean;
}
/**
 * Identity mapper that passes positions through unchanged.
 * Used when no source mapping is available.
 */
export declare class IdentityMapper extends PositionMapper {
    constructor();
    originalToExpanded(pos: number): number;
    expandedToOriginal(pos: number): number;
    isInGenerated(_pos: number): boolean;
    generatedBy(_pos: number): string | null;
    mapSpanToOriginal(start: number, length: number): {
        start: number;
        length: number;
    };
    mapSpanToExpanded(start: number, length: number): {
        start: number;
        length: number;
    };
    isEmpty(): boolean;
}
//# sourceMappingURL=source-map.d.ts.map