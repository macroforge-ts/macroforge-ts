/** import macro {Gigaform} from "@playground/macro"; */

import type { Weekday } from './weekday.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Array<Weekday>;
}
