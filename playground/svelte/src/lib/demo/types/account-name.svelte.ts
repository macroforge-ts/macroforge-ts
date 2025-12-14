/** import macro {Gigaform} from "@playground/macro"; */

import { PersonName } from './person-name.svelte';
import { CompanyName } from './company-name.svelte';
import { Company } from './company.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
/** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
export type AccountName = /** @default */ CompanyName | PersonName;
