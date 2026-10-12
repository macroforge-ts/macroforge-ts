/** @derive(Encode, Decode) */
interface Item {
    id: string;
}

/** @derive(Encode, Decode) */
interface Nested {
    grid: Date[][];
    setsByName: Map<string, Set<Item>>;
    byNumber: Map<number, Item | null>;
    pairs: [string, Item][];
    patches: Array<Pick<Item, 'id'>>;
    inline: { at: Date; item?: Item };
    sizes: ('small' | 'large')[];
}
