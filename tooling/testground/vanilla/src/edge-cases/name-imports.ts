/**
 * Types held from another module under names that do not lead to their
 * functions by themselves: one named in lowercase, and one imported under
 * another name. Their functions are imported like any other type's.
 */

import type { EdgeItem as Item } from './edge-types';
import type { point2D } from './name-shapes';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NameImports {
    origin: point2D;
    path: point2D[];
    byName: Map<string, point2D>;
    maybe: point2D | null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NameRenamed {
    held: Item;
    listed: Item[];
    maybe: Item | null;
    some: Partial<Item>;
}
