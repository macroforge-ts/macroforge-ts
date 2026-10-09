/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}
