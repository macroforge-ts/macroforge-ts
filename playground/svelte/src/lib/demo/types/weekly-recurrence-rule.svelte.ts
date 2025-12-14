/** import macro {Gigaform} from "@playground/macro"; */

import { Weekday } from './weekday.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Weekday[];
}
