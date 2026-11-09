/**
 * A type holding one an ECMAScript module declares: its derives call the
 * functions that module's derives generate, imported from it as `.mjs`.
 */

import type { KindedPoint } from './module-kinds.mjs';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface KindedHolder {
    point: KindedPoint;
    label: string;
}
