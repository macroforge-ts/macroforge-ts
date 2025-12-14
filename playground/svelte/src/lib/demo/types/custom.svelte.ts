/** import macro {Gigaform} from "@playground/macro"; */

import { DirectionHue } from './direction-hue.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Custom {
    mappings: DirectionHue[];
}
