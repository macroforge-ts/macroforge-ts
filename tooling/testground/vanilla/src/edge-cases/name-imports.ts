/**
 * Types named in lowercase, held from another module: their functions are
 * imported like any other type's.
 */

import type { point2D } from './name-shapes';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NameImports {
    origin: point2D;
    path: point2D[];
    byName: Map<string, point2D>;
    maybe: point2D | null;
}
