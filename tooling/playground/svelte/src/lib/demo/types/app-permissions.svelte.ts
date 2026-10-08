/** import macro {Gigaform} from "@playground/macro"; */

import type { Applications } from './applications.svelte';
import type { Page } from './page.svelte';
import type { Table } from './table.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface AppPermissions {
    applications: Array<Applications>;
    pages: Array<Page>;
    data: Array<Table>;
}
