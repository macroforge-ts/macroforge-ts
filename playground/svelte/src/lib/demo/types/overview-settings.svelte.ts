/** import macro {Gigaform} from "@playground/macro"; */

import type { ColumnConfig } from './column-config.svelte';
import type { OverviewDisplay } from './overview-display.svelte';
import type { RowHeight } from './row-height.svelte';
import type { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface OverviewSettings {
    /** @default("Medium") */
    rowHeight: RowHeight;
    /** @default("Table") */
    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: ColumnConfig[];
}
