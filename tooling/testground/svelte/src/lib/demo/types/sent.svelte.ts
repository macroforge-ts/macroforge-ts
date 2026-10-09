/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Sent {
    recipient: string | null;
    method: string | null;
}
