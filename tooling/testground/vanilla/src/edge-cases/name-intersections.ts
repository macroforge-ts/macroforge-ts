/**
 * An intersection with a type held from another module under another name:
 * its fields are that type's, read through the import.
 */

import type { EdgeItem as Item } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type NameMixed = Item & { extra: string };
