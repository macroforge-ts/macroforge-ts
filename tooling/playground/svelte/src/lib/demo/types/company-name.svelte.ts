/** import macro {Gigaform} from "@playground/macro"; */

import type { Company } from './company.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface CompanyName {
    /** @textController({ label: "Company Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    companyName: string;
}
