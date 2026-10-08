/** import macro {Gigaform} from "@playground/macro"; */

import type { Email } from './email.svelte';
import type { JobTitle } from './job-title.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Route } from './route.svelte';
import type { Settings } from './settings.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Employee {
    id: string;
    imageUrl: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    phones: Array<PhoneNumber>;
    /** @endec({ validate: ["nonEmpty"] }) */
    role: string;
    /** @default("Technician") */
    title: JobTitle;
    email: Email;
    /** @endec({ validate: ["nonEmpty"] }) */
    address: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    username: string;
    /** @default("") */
    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: Array<string>;
    settings: Settings;
}
