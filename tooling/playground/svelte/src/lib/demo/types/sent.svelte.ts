/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Sent {
    recipient: string | null;
    method: string | null;
}
