/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}
