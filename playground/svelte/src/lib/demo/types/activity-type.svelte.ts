/** import macro {Gigaform} from "@playground/macro"; */

import { Edited } from './edited.svelte';
import { Commented } from './commented.svelte';
import { Viewed } from './viewed.svelte';
import { Paid } from './paid.svelte';
import { Created } from './created.svelte';
import { Sent } from './sent.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;
