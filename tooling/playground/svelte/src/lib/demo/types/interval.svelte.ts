/** import macro {Gigaform} from "@playground/macro"; */

import type { YearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';
import type { MonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import type { DailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import type { WeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;
