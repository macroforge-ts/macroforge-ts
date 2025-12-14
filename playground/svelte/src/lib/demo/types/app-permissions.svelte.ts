/** import macro {Gigaform} from "@playground/macro"; */

import { Page } from './page.svelte';
import { Applications } from './applications.svelte';
import { Table } from './table.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface AppPermissions {
    applications: Applications[];
    pages: Page[];
    data: Table[];
}
