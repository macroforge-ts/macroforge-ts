/** import macro {Gigaform} from "@playground/macro"; */

import type { DirectionHue } from './direction-hue.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Custom {
    mappings: DirectionHue[];
}
