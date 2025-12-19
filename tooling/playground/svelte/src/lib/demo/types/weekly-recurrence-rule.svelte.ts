/** import macro {Gigaform} from "@playground/macro"; */

import type { Weekday } from './weekday.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Weekday[];
}
