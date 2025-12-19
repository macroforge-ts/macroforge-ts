/** import macro {Gigaform} from "@playground/macro"; */

import type { Page } from './page.svelte';
import type { Applications } from './applications.svelte';
import type { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface AppPermissions {
    applications: Applications[];
    pages: Page[];
    data: Table[];
}
