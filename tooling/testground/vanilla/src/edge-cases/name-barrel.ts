/**
 * A module that declares nothing of its own: it exports again what two other
 * modules export, one of them whole and the other by name.
 */

export * from './edge-types';
export { default as NameBarrelModel } from './name-exports';
export type { NameShown as NameBarrelShown } from './name-exports';
