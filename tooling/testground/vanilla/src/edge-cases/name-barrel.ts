/**
 * A module that declares nothing of its own: it exports again what two other
 * modules export, one of them whole, the other by name, and one name it
 * imports first and exports after.
 */

import type { NameShown } from './name-exports';

export * from './edge-types';
export { default as NameBarrelModel } from './name-exports';
export type { NameShown as NameBarrelShown } from './name-exports';
export type { NameShown as NameBarrelHanded };
