/** import macro {Gigaform} from "@playground/macro"; */

import { YearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';
import { MonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import { DailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import { WeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;
