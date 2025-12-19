/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface DataPath {
    path: string[];
    formatter: string | null;
}
