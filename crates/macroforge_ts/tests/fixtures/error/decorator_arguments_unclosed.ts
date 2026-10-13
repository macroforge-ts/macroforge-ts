/** A decorator whose arguments never close is an error, not a decorator left out. */
/** @derive(Default, Decode) */
export interface Settings {
    /** @default(5 */
    retries: number;
    /** @default({ a: ) */
    extra: Record<string, number>;
    /**
     * @endec({
     *   rename: "label"
     */
    name: string;
}

/** @derive(Default) */
export class Options {
    /** @default(] */
    level: number;
}

/**
 * @derive(Default)
 * @endec({ tag: "kind"
 */
export type Shape = { side: number };

/** @derive(Debug */
export interface Unread {
    id: string;
}

/** A type no derive is on keeps its comments to itself. */
export interface Plain {
    /** @default (see the notes */
    note: string;
}
