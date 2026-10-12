import { Set } from './collections';

/** @derive(Debug) */
interface Record {
    id: string;
}

class Map {
    size = 0;
}

type Partial = { part: number };

/** @derive(Encode, Decode) */
interface Index {
    byName: Map;
    seen: Set;
}
