/** import macro {Gigaform} from "@playground/macro"; */

import { Item } from './item.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface BilledItem {
    /** @comboboxController({ label: "Item", allowCustom: true, fetchUrls: ["/api/products", "/api/services"] }) */
    /** @default("") */
    item: Item;
    /** @numberController({ label: "Quantity", min: 0, step: 1 }) */
    quantity: number;
    /** @switchController({ label: "Taxed" }) */
    taxed: boolean;
    /** @switchController({ label: "Upsale" }) */
    upsale: boolean;
}
