/** import macro {Gigaform} from "@playground/macro"; */

import type { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OverviewDisplay = /** @default */ 'Card' | 'Table';
