/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Ordinal {
    north: number;
    northeast: number;
    east: number;
    southeast: number;
    south: number;
    southwest: number;
    west: number;
    northwest: number;
}
