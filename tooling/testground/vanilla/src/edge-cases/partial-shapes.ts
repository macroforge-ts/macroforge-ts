/**
 * `Partial`, `Pick` and `Omit` of types from another module, wherever a
 * field can hold one: on its own, in a collection, nullable, over renamed
 * fields, validators, defaults, flattened fields and cycles.
 */

import type {
    EdgeCard,
    EdgeItem,
    EdgeLabeled,
    EdgeNode,
    EdgeProfile,
    EdgeStrict
} from './edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface EdgePartials {
    patch: Partial<EdgeItem>;
    patches: Partial<EdgeItem>[];
    patchList: Array<Pick<EdgeItem, 'id'>>;
    readonlyPatches: ReadonlyArray<Omit<EdgeItem, 'score'>>;
    patchSet: Set<Partial<EdgeItem>>;
    byName: Map<string, Pick<EdgeItem, 'id'>>;
    record: Record<string, Omit<EdgeItem, 'score'>>;
    maybe: Partial<EdgeItem> | null;
    later: Pick<EdgeItem, 'id'> | undefined;
    optional?: Omit<EdgeItem, 'id'>;
    nullablePatches: (Partial<EdgeItem> | null)[];
    card: Partial<EdgeCard>;
    cards: Pick<EdgeCard, 'title'>[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface EdgeRenamedPartials {
    profile: Partial<EdgeProfile>;
    contact: Pick<EdgeProfile, 'email' | 'displayName'>;
    anonymous: Omit<EdgeProfile, 'email' | 'nickname'>;
    strict: Pick<EdgeStrict, 'id'>;
    labeled: Partial<EdgeLabeled>;
    labelOnly: Pick<EdgeLabeled, 'label'>;
    head: Pick<EdgeNode, 'name'>;
    link: Omit<EdgeNode, 'name'>;
}
