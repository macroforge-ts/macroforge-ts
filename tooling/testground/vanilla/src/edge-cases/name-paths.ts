/**
 * A type held from another module by a path that is not a relative one, as
 * a `paths` mapping allows: its functions are imported by the same path.
 */

import type { EdgeItem } from 'src/edge-cases/edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NamePaths {
    held: EdgeItem;
    listed: EdgeItem[];
    maybe: EdgeItem | null;
}
