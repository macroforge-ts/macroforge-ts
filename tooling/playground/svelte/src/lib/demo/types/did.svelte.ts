/** import macro {Gigaform} from "@playground/macro"; */

import type { ActivityType } from './activity-type.svelte';
import type { Actor } from './actor.svelte';
import type { Target } from './target.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Did {
    /** @default("") */
    in: string | Actor;
    /** @default("") */
    out: string | Target;
    id: string;
    activityType: ActivityType;
    createdAt: string;
    metadata: string | null;
}
