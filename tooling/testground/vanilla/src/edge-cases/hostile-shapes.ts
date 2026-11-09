/**
 * Types decoded from input nobody vouches for: keys that reach a prototype,
 * values of the wrong kind, members that throw, references to nothing and
 * nesting deeper than the stack.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface HostileItem {
    id: string;
    score: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface HostileBag {
    name: string;
    item: HostileItem;
    items: HostileItem[];
    byName: Record<string, HostileItem>;
    counts: Record<string, number>;
    lookup: Map<string, number>;
    tags: Set<string>;
    pair: [string, number];
    at: Date;
    big: bigint;
    flag: boolean;
    nested: HostileBag | null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export class HostileClass {
    name: string = '';
    count: number = 0;

    label(): string {
        return `${this.name}:${this.count}`;
    }
}

/** @derive(Encode, Decode) */
/** @endec({ denyUnknownFields: true }) */
export interface HostileStrict {
    name: string;
}
