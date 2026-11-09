/**
 * The Source Map v3 of an expansion, built from the engine's source mapping.
 *
 * @module
 */

/** One range of the expanded code holding a range of the original as written. */
export interface SourceMappingSegment {
    originalStart: number;
    originalEnd: number;
    expandedStart: number;
    expandedEnd: number;
}

/** The engine's source mapping of an expansion: offsets in UTF-16 code units. */
export interface SourceMapping {
    segments: ReadonlyArray<SourceMappingSegment>;
}

/** A Source Map v3 of expanded code against one original source. */
export interface SourceMapV3 {
    version: 3;
    sources: string[];
    sourcesContent: string[];
    names: string[];
    mappings: string;
}

const VLQ_DIGITS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';

function vlq(value: number): string {
    let rest = value < 0 ? (-value << 1) | 1 : value << 1;
    let encoded = '';
    do {
        let digit = rest & 31;
        rest >>>= 5;
        if (rest > 0) digit |= 32;
        encoded += VLQ_DIGITS.charAt(digit);
    } while (rest > 0);
    return encoded;
}

/** The offset each line of `text` starts at. */
function lineStarts(text: string): number[] {
    const starts = [0];
    for (let offset = 0; offset < text.length; offset++) {
        if (text.charCodeAt(offset) === 10) starts.push(offset + 1);
    }
    return starts;
}

/** The 0-based line and column of `offset`, given the line starts. */
function position(offset: number, starts: readonly number[]): [number, number] {
    let low = 0;
    let high = starts.length - 1;
    while (low < high) {
        const middle = (low + high + 1) >>> 1;
        if ((starts[middle] ?? 0) <= offset) low = middle;
        else high = middle - 1;
    }
    return [low, offset - (starts[low] ?? 0)];
}

/**
 * The Source Map v3 of `expanded` against `original`, named `source`, or
 * `undefined` when the mapping has no segment. Every line a segment spans
 * is mapped, as the text it holds is the original's as written: a mapping
 * at the segment's start alone leaves the lines after it mapped nowhere.
 */
export function sourceMapOf(
    mapping: SourceMapping,
    source: string,
    original: string,
    expanded: string
): SourceMapV3 | undefined {
    if (mapping.segments.length === 0) return undefined;
    const originalStarts = lineStarts(original);
    const expandedStarts = lineStarts(expanded);
    const lines: Array<Array<[number, number, number]>> = expandedStarts.map(() => []);
    for (const segment of mapping.segments) {
        const length = Math.min(
            segment.originalEnd - segment.originalStart,
            segment.expandedEnd - segment.expandedStart
        );
        const end = segment.expandedStart + length;
        let offset = segment.expandedStart;
        for (;;) {
            const [line, column] = position(offset, expandedStarts);
            const originalAt = position(
                segment.originalStart + offset - segment.expandedStart,
                originalStarts
            );
            lines[line]?.push([column, originalAt[0], originalAt[1]]);
            const next = expandedStarts[line + 1];
            if (next === undefined || next >= end) break;
            offset = next;
        }
    }
    let previousLine = 0;
    let previousColumn = 0;
    const mappings = lines.map((entries) => {
        entries.sort((left, right) => left[0] - right[0]);
        let previousGenerated = 0;
        return entries
            .map(([column, originalLine, originalColumn]) => {
                const encoded = vlq(column - previousGenerated) +
                    vlq(0) +
                    vlq(originalLine - previousLine) +
                    vlq(originalColumn - previousColumn);
                previousGenerated = column;
                previousLine = originalLine;
                previousColumn = originalColumn;
                return encoded;
            })
            .join(',');
    });
    return {
        version: 3,
        sources: [source],
        sourcesContent: [original],
        names: [],
        mappings: mappings.join(';')
    };
}

/**
 * The 1-based line and column of `offset` in `text`, as an error message
 * names a place.
 */
export function lineAndColumn(text: string, offset: number): { line: number; column: number } {
    const [line, column] = position(offset, lineStarts(text));
    return { line: line + 1, column: column + 1 };
}
