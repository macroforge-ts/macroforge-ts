/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';
