/** import macro {Gigaform} from "@playground/macro"; */

import type { Employee } from './employee.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Route {
    id: string;
    techs: Array<string | Employee> | null;
    active: boolean;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    phone: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: Array<string>;
    icon: string | null;
    color: string | null;
}
