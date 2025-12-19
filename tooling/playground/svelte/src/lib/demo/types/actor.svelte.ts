/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
