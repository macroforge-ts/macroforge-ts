/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
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
