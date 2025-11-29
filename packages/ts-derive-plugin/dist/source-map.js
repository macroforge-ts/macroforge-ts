"use strict";
/**
 * Source mapping between original and macro-expanded code.
 *
 * When macros expand code (e.g., inserting toString() method), byte positions shift.
 * This module provides bidirectional position mapping to translate between
 * original source positions and expanded code positions.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.IdentityMapper = exports.PositionMapper = void 0;
/**
 * Bidirectional position mapper between original and expanded source code.
 *
 * Use this to translate positions when:
 * - Reporting diagnostics: expanded -> original (so errors point to user's source)
 * - Processing hover requests: original -> expanded (to find info in expanded code)
 * - Returning spans from LS methods: expanded -> original (so spans match user's view)
 */
class PositionMapper {
    segments;
    generatedRegions;
    constructor(mapping) {
        this.segments = mapping.segments;
        this.generatedRegions = mapping.generatedRegions;
    }
    /**
     * Map a position from original source to expanded source.
     * Used for: hover requests, completion requests, go-to-definition requests
     */
    originalToExpanded(pos) {
        for (const seg of this.segments) {
            if (pos >= seg.originalStart && pos < seg.originalEnd) {
                const offset = pos - seg.originalStart;
                return seg.expandedStart + offset;
            }
        }
        // Position after last segment - calculate delta from last known mapping
        const last = this.segments[this.segments.length - 1];
        if (last && pos >= last.originalEnd) {
            const delta = pos - last.originalEnd;
            return last.expandedEnd + delta;
        }
        // No mapping found, return as-is (identity)
        return pos;
    }
    /**
     * Map a position from expanded source to original source.
     * Used for: diagnostic positions, definition spans, reference spans
     *
     * Returns null if the position is in generated code (has no original).
     */
    expandedToOriginal(pos) {
        // Check if in a generated region first
        if (this.isInGenerated(pos)) {
            return null;
        }
        for (const seg of this.segments) {
            if (pos >= seg.expandedStart && pos < seg.expandedEnd) {
                const offset = pos - seg.expandedStart;
                return seg.originalStart + offset;
            }
        }
        // Position after last segment
        const last = this.segments[this.segments.length - 1];
        if (last && pos >= last.expandedEnd) {
            const delta = pos - last.expandedEnd;
            return last.originalEnd + delta;
        }
        return null;
    }
    /**
     * Check if an expanded position is within generated code.
     */
    isInGenerated(expandedPos) {
        return this.generatedRegions.some((r) => expandedPos >= r.start && expandedPos < r.end);
    }
    /**
     * Get the macro that generated code at a position, if any.
     */
    generatedBy(expandedPos) {
        const region = this.generatedRegions.find((r) => expandedPos >= r.start && expandedPos < r.end);
        return region ? region.sourceMacro : null;
    }
    /**
     * Map a span (start, length) from expanded to original coordinates.
     * Returns null if either endpoint is in generated code.
     */
    mapSpanToOriginal(start, length) {
        const end = start + length;
        const originalStart = this.expandedToOriginal(start);
        const originalEnd = this.expandedToOriginal(end);
        if (originalStart === null || originalEnd === null) {
            return null;
        }
        return {
            start: originalStart,
            length: Math.max(0, originalEnd - originalStart),
        };
    }
    /**
     * Map a span (start, length) from original to expanded coordinates.
     */
    mapSpanToExpanded(start, length) {
        const end = start + length;
        const expandedStart = this.originalToExpanded(start);
        const expandedEnd = this.originalToExpanded(end);
        return {
            start: expandedStart,
            length: Math.max(0, expandedEnd - expandedStart),
        };
    }
    /**
     * Check if the mapping has any data.
     */
    isEmpty() {
        return this.segments.length === 0 && this.generatedRegions.length === 0;
    }
}
exports.PositionMapper = PositionMapper;
/**
 * Identity mapper that passes positions through unchanged.
 * Used when no source mapping is available.
 */
class IdentityMapper extends PositionMapper {
    constructor() {
        super({ segments: [], generatedRegions: [] });
    }
    originalToExpanded(pos) {
        return pos;
    }
    expandedToOriginal(pos) {
        return pos;
    }
    isInGenerated(_pos) {
        return false;
    }
    generatedBy(_pos) {
        return null;
    }
    mapSpanToOriginal(start, length) {
        return { start, length };
    }
    mapSpanToExpanded(start, length) {
        return { start, length };
    }
    isEmpty() {
        return true;
    }
}
exports.IdentityMapper = IdentityMapper;
