/** import macro {Gigaform} from "@playground/macro"; */

import { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OverviewDisplay = /** @default */ 'Card' | 'Table';
