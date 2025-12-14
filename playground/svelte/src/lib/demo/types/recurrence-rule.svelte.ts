/** import macro {Gigaform} from "@playground/macro"; */

import { Interval } from './interval.svelte';
import { RecurrenceEnd } from './recurrence-end.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: string[] | null;
    additionalInstances: string[] | null;
}
