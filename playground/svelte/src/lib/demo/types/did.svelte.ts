/** import macro {Gigaform} from "@playground/macro"; */

import { Target } from './target.svelte';
import { ActivityType } from './activity-type.svelte';
import { Actor } from './actor.svelte';

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
