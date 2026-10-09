/** import macro {Gigaform} from "@testground/macro"; */

import type { CompanyName } from './company-name.svelte';
import type { PersonName } from './person-name.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
/** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
export type AccountName = /** @default */ CompanyName | PersonName;
