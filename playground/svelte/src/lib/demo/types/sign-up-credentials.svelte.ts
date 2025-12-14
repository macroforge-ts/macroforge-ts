/** import macro {Gigaform} from "@playground/macro"; */

import { FirstName } from './first-name.svelte';
import { Password } from './password.svelte';
import { EmailParts } from './email-parts.svelte';
import { LastName } from './last-name.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface SignUpCredentials {
    firstName: FirstName;
    lastName: LastName;
    email: EmailParts;
    password: Password;
    rememberMe: boolean;
}
