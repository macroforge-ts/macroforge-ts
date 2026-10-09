/** import macro {Gigaform} from "@testground/macro"; */

import type { ServiceDefaults } from './service-defaults.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Service {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Quick Code" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    quickCode: string;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    group: string | null;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    subgroup: string | null;
    /** @comboboxController({ labeool: "Unit", allowCustom: true }) */
    unit: string | null;
    /** @switchController({ label: "Active" }) */
    active: boolean;
    /** @switchController({ label: "Commission" }) */
    commission: boolean;
    /** @switchController({ label: "Favorite" }) */
    favorite: boolean;
    /** @textController({ label: "Average Time" }) */
    averageTime: string | null;
    defaults: ServiceDefaults;
}
