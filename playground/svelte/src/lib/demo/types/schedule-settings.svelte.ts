/** import macro {Gigaform} from "@playground/macro"; */

import type { RowHeight } from './row-height.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ScheduleSettings {
    daysPerWeek: number;
    /** @default("Medium") */
    rowHeight: RowHeight;
    visibleRoutes: string[];
    detailedCards: boolean;
}
