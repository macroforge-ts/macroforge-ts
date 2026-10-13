/**
 * Unions whose members are objects, literals and template literal types, in
 * every tagging mode, and a class three levels deep whose fields are typed
 * by their initializers.
 */

/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Encode, Decode) */
export interface VariantCircle {
    radius: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Encode, Decode) */
export interface VariantRect {
    size: [number, number];
    label?: `r-${string}`;
}

/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Encode, Decode) */
export type VariantInternal = VariantCircle | VariantRect | 'none' | `id-${number}` | 7 | null;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ tag: "t", content: "c" }) */
export type VariantAdjacent = VariantCircle | VariantRect;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ externallyTagged: true }) */
export type VariantExternal = VariantCircle | VariantRect;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ untagged: true }) */
export type VariantUntagged = VariantCircle | VariantRect | string | number;

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class VariantBase {
    id = '';
    created = new Date(0);
    protected weight = 1;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class VariantMid extends VariantBase {
    tags = new Set<string>();
    protected note: string | null = null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class VariantLeaf extends VariantMid {
    shape: VariantInternal = null;
    readonly kind = 'leaf';
    children: VariantLeaf[] = [];
}
