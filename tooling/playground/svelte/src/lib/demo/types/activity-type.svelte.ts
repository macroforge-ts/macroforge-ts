/** import macro {Gigaform} from "@playground/macro"; */

import type { Edited } from './edited.svelte';
import type { Commented } from './commented.svelte';
import type { Viewed } from './viewed.svelte';
import type { Paid } from './paid.svelte';
import type { Created } from './created.svelte';
import type { Sent } from './sent.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;
