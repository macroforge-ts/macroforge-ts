/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}
