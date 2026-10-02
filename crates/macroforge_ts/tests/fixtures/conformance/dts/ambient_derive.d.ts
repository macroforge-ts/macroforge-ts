/** @derive(Debug, Clone) */
export declare class Point {
    x: number;
    y: number;
}

/** @derive(Debug) */
export interface Named {
    name: string;
}

/**
 * Formats a point.
 * @deprecated Use `Point.toString` instead.
 */
export declare function formatPoint(point: Point): string;
