/** import macro {Gigaform} from "@testground/macro"; */

import type { Product } from './product.svelte';
import type { RecordLink } from './record-link.svelte';
import type { Service } from './service.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;
