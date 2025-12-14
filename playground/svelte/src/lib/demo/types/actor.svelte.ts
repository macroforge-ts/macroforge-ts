/** import macro {Gigaform} from "@playground/macro"; */

import { User } from './user.svelte';
import { Account } from './account.svelte';
import { Employee } from './employee.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
