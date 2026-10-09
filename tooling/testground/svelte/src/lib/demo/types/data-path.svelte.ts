/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}
