/** import macro {Gigaform} from "@playground/macro"; */

import type { Cardinal } from "./cardinal.svelte";
import type { Custom } from "./custom.svelte";
import type { Gradient } from "./gradient.svelte";
import type { Ordinal } from "./ordinal.svelte";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ColorsConfig =
  | Cardinal
  | Ordinal
  | Custom
  | /** @default */ Gradient;
