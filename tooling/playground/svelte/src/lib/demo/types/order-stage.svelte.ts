/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';
