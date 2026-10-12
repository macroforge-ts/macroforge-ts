/** @derive(Encode, Decode) */
interface Item {
    id: string;
    score: number;
}

/** @derive(Encode, Decode) */
interface Draft {
    patch: Partial<Pick<Item, 'id'>>;
}
