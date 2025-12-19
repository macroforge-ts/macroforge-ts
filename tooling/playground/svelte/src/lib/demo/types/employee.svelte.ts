/** import macro {Gigaform} from "@playground/macro"; */

import type { PhoneNumber } from './phone-number.svelte';
import type { Settings } from './settings.svelte';
import type { Route } from './route.svelte';
import type { Email } from './email.svelte';
import type { JobTitle } from './job-title.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Employee {
    id: string;
    imageUrl: string | null;
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    phones: PhoneNumber[];
    /** @serde({ validate: ["nonEmpty"] }) */
    role: string;
    /** @default("Technician") */
    title: JobTitle;
    email: Email;
    /** @serde({ validate: ["nonEmpty"] }) */
    address: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    username: string;
    /** @default("") */
    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: string[];
    settings: Settings;
}
