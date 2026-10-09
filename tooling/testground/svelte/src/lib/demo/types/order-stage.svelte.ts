/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';
