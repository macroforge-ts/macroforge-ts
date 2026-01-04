/** import macro {Gigaform} from "@playground/macro"; */

import type { Product } from "./product.svelte";
import type { RecordLink } from "./record-link.svelte";
import type { Service } from "./service.svelte";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;
