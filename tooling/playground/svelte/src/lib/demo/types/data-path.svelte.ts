/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}
