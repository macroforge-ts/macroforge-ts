/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';
import type { User } from './user.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
