/** import macro {Gigaform} from "@playground/macro"; */

import { Service } from './service.svelte';
import { Product } from './product.svelte';
import { RecordLink } from './record-link.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;
