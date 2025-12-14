/** import macro {Gigaform} from "@playground/macro"; */

import { Account } from './account.svelte';
import { Employee } from './employee.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Represents {
    /** @default("") */
    in: string | Employee;
    /** @default("") */
    out: string | Account;
    id: string;
    dateStarted: string;
}
