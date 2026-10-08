/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}
