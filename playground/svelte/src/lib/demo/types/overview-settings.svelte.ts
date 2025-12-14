/** import macro {Gigaform} from "@playground/macro"; */

import { ColumnConfig } from './column-config.svelte';
import { OverviewDisplay } from './overview-display.svelte';
import { RowHeight } from './row-height.svelte';
import { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface OverviewSettings {
    /** @default("Medium") */
    rowHeight: RowHeight;
    /** @default("Table") */
    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: ColumnConfig[];
}
