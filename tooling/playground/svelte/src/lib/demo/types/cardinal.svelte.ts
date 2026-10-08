/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}
