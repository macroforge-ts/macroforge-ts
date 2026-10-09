/**
 * A generic alias over a foreign type, declared apart from the types that use
 * it: a field typed `Stamped<T>` expands to `DateTime.DateTime | T` in a module
 * that imports nothing from `effect` itself.
 */

import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export type Stamped<T> = /** @default */ DateTime.DateTime | T;

/** @derive(Default, Encode, Decode) */
/** @endec(nonEmpty) */
/** @default("untitled") */
export type Slug = $Newtype<string>;
