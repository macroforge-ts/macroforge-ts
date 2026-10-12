/** @derive(Decode) */
interface Item {
    id: string;
}

/** @derive(Decode) */
type OneOrMany = Item | Item[];
