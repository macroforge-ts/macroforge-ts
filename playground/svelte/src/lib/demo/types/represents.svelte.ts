/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Represents {
    /** @default("") */
    in: string | Employee;
    /** @default("") */
    out: string | Account;
    id: string;
    dateStarted: string;
}
