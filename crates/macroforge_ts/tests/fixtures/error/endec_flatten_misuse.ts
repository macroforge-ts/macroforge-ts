/** @derive(Encode, Decode) */
export interface Inner {
    id: string;
}

/** @derive(Encode, Decode) */
export type Shape = { kind: 'a'; x: number } | { kind: 'b'; y: number };

/** @derive(Encode, Decode) */
export interface Flattened<T> {
    /** @endec(flatten) */
    text: string;
    /** @endec(flatten) */
    list: Inner[];
    /** @endec(flatten) */
    map: Map<string, number>;
    /** @endec(flatten) */
    partial: Partial<Inner>;
    /** @endec(flatten) */
    inline: { x: number };
    /** @endec(flatten) */
    held: T;
    /** @endec(flatten) */
    whole: Inner;
    /** @endec(flatten) */
    maybe: Inner | null;
    /** @endec(flatten) */
    rest: Record<string, number>;
}
