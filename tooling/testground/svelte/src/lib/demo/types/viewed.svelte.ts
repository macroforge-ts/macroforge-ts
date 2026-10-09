/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}
