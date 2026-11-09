const KEY = 'computed';

/** @derive(Debug, Clone) */
interface Bag {
    id: string;
    [key: string]: unknown;
}

/** @derive(PartialEq, Hash) */
class Tagged {
    name: string = '';
    [KEY]: number = 0;
}

/** @derive(Encode, Decode) */
type Keyed = { id: string; [KEY]: number };

/** @derive(Default) */
type Shape = { kind: 'dot' } | { kind: 'bag'; [key: string]: string };

/** @derive(Encode, Decode) */
interface Holder {
    entries: { id: string; [key: string]: Date };
}
