/** import macro {Gigaform} from "@playground/macro"; */

import { Account } from './account.svelte';
import { Order } from './order.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Ordered {
    id: string;
    /** @default("") */
    in: string | Account;
    /** @default("") */
    out: string | Order;
    date: string;
}
