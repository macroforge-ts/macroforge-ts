/** import macro {Gigaform} from "@testground/macro"; */

import type { Item } from './item.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
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
