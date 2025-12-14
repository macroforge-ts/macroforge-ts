/** import macro {Gigaform} from "@playground/macro"; */

import { Gradient } from './gradient.svelte';
import { Custom } from './custom.svelte';
import { Ordinal } from './ordinal.svelte';
import { Cardinal } from './cardinal.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;
