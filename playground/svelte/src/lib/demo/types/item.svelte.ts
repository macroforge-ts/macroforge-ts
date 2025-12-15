/** import macro {Gigaform} from "@playground/macro"; */

import type { Service } from './service.svelte';
import type { Product } from './product.svelte';
import type { RecordLink } from './record-link.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;
