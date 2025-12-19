/** import macro {Gigaform} from "@playground/macro"; */

import type { Employee } from './employee.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Route {
    id: string;
    techs: (string | Employee)[] | null;
    active: boolean;
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    phone: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: string[];
    icon: string | null;
    color: string | null;
}
