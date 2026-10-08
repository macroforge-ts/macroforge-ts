/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}
