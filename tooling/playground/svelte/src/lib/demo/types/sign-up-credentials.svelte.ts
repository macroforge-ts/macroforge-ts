/** import macro {Gigaform} from "@playground/macro"; */

import type { EmailParts } from './email-parts.svelte';
import type { FirstName } from './first-name.svelte';
import type { LastName } from './last-name.svelte';
import type { Password } from './password.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface SignUpCredentials {
    firstName: FirstName;
    lastName: LastName;
    email: EmailParts;
    password: Password;
    rememberMe: boolean;
}
