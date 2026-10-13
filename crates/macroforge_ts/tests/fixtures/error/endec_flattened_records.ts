/** @derive(Encode, Decode) */
export interface Rest {
    name: string;
    /** @endec(flatten) */
    counts: Record<string, number>;
    /** @endec(flatten) */
    labels: Record<string, string>;
}
