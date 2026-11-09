/**
 * The types `operator-shapes.ts` reads through `keyof` and an indexed access,
 * without importing the type a field of them names.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface OperatorOwner {
    name: string;
    since: Date;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface OperatorItem {
    owner: OperatorOwner;
    owners: OperatorOwner[];
    label: string;
    weight?: number;
}
