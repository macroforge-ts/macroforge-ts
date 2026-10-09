/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}
