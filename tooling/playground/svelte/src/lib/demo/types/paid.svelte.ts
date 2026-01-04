/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}
