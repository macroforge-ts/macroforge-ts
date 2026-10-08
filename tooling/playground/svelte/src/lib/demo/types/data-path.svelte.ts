/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}
