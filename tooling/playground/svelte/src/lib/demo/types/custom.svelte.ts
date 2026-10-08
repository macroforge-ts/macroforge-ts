/** import macro {Gigaform} from "@playground/macro"; */

import type { DirectionHue } from './direction-hue.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Custom {
    mappings: Array<DirectionHue>;
}
